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
        s.v[1229] = if ((((-s.v[700])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_exp_ad(693, A::neg(s.ad_value(700)));
        }

        s.v[1230] = if ((-s.v[700]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) && (!(s.v[1229] != 0.0))) && (s.v[1230] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) && (!(s.v[1229] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(A::neg(s.ad_value(700)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_add_ad(694, A::scale(A::sub(s.ad_value(706), s.ad_value(700)), 2.0), A::mul(s.ad_value(709), A::sub_from_scalar(1.0, s.ad_value(693))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_sub_ad(695, A::mul(A::sub(s.ad_value(706), s.ad_value(700)), A::sub(s.ad_value(706), s.ad_value(700))), A::mul(s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_div_ad(701, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_add(716, 700, 701);
        }

        if ((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) {
            s.store_neg(716, 716);
        }

        s.store_div_ad_lhs(708, A::sqrt(A::mul(A::mul(A::scale(s.ad_value(20), (2.0 * 1.602176565e-19)), s.ad_value(229)), s.ad_value(224))), 241);

        s.store_square(709, 708);

        s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);

        s.store_scale(711, 710, 1e-5);

        s.store_div_from_scalar(712, 1.0, 710);

        s.store_div_from_scalar_ad(713, 1.0, A::offset(A::scale(s.ad_value(708), 0.7324648775608221), 1.25));

        s.v[1231] = if (((p.p3 > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p.p4 > 0.0) && (s.v[90] > 0.0))) { 1.0 } else { 0.0 };

        s.v[1232] = if (((s.v[705]) as f64).abs() <= s.v[711]) { 1.0 } else { 0.0 };

        if ((s.v[1231] != 0.0) && (s.v[1232] != 0.0)) {
            s.store_mul_ad_lhs(715, A::neg(s.ad_value(705)), 712);
        }

        s.v[1233] = if (s.v[705] < (-s.v[711])) { 1.0 } else { 0.0 };

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_neg(683, 705);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_mul_ad_lhs(684, A::scale(s.ad_value(683), 1.25), 712);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_scale_ad(685, A::sub(A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(684), (-6.0)), A::offset(s.ad_value(684), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_add_ad(686, A::mul(A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685))), A::mul(s.ad_value(709), A::offset(s.ad_value(685), 1.0)));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_sub_ad_lhs(687, A::scale(A::sub(s.ad_value(683), s.ad_value(685)), 2.0), 709);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_add(689, 686, 687);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_add_ad(690, A::square(s.ad_value(689)), A::mul(s.ad_value(688), A::sub(A::mul(A::scale(s.ad_value(687), 0.5), s.ad_value(687)), s.ad_value(686))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_add_ad_rhs(691, 690, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688)), s.ad_value(688)), s.ad_value(687)), A::sub(A::scale(A::square(s.ad_value(687)), 0.3333333333333), s.ad_value(686))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_add_ad_rhs(692, 685, A::div(A::mul(A::mul(s.ad_value(686), s.ad_value(689)), s.ad_value(688)), s.ad_value(691)));
        }

        s.v[1234] = if (((s.v[692]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) && (s.v[1234] != 0.0)) {
            s.store_exp(693, 692);
        }

        s.v[1235] = if (s.v[692] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) && (!(s.v[1234] != 0.0))) && (s.v[1235] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(692)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) && (!(s.v[1234] != 0.0))) && (!(s.v[1235] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(s.ad_value(692), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_sub(691, 683, 692);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_add_ad(694, A::scale(s.ad_value(691), 2.0), A::mul(s.ad_value(709), A::offset(s.ad_value(693), (-1.0))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_add_ad(695, A::square(s.ad_value(691)), A::mul(s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_div_ad(697, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_neg_ad(715, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_mul_ad_lhs(698, A::offset(A::mul(A::scale(s.ad_value(710), 1.25), s.ad_value(713)), (-1.0)), 713);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_mul_ad(699, A::mul(s.ad_value(705), s.ad_value(712)), A::offset(A::mul(s.ad_value(698), s.ad_value(705)), 1.0));
        }

        s.v[1236] = if ((((-s.v[699])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) && (s.v[1236] != 0.0)) {
            s.store_exp_ad(691, A::neg(s.ad_value(699)));
        }

        s.v[1237] = if ((-s.v[699]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) && (!(s.v[1236] != 0.0))) && (s.v[1237] != 0.0)) {
            s.store_div_from_scalar_ad(691, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) && (!(s.v[1236] != 0.0))) && (!(s.v[1237] != 0.0))) {
            s.store_scale_ad(691, A::offset(A::mul(A::offset(A::neg(s.ad_value(699)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_sub_from_scalar(697, 1.0, 691);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_sub_ad(700, A::add(s.ad_value(705), A::scale(s.ad_value(709), 0.5)), A::mul(s.ad_value(708), A::sqrt(A::sub(A::add(s.ad_value(705), A::scale(s.ad_value(709), 0.25)), s.ad_value(697)))));
        }

        s.v[1238] = if ((((-s.v[700])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_exp_ad(693, A::neg(s.ad_value(700)));
        }

        s.v[1239] = if ((-s.v[700]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) && (!(s.v[1238] != 0.0))) && (s.v[1239] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) && (!(s.v[1238] != 0.0))) && (!(s.v[1239] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(A::neg(s.ad_value(700)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_add_ad(694, A::scale(A::sub(s.ad_value(705), s.ad_value(700)), 2.0), A::mul(s.ad_value(709), A::sub_from_scalar(1.0, s.ad_value(693))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_sub_ad(695, A::mul(A::sub(s.ad_value(705), s.ad_value(700)), A::sub(s.ad_value(705), s.ad_value(700))), A::mul(s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_div_ad(701, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_add(715, 700, 701);
        }

        if ((s.v[1231] != 0.0) && (!(s.v[1232] != 0.0))) {
            s.store_neg(715, 715);
        }

        s.v[1240] = if (s.v[160] > 0.0) { 1.0 } else { 0.0 };

        s.v[1241] = if (((s.v[707]) as f64).abs() <= s.v[711]) { 1.0 } else { 0.0 };

        if ((s.v[1240] != 0.0) && (s.v[1241] != 0.0)) {
            s.store_mul_ad_lhs(717, A::neg(s.ad_value(707)), 712);
        }

        s.v[1242] = if (s.v[707] < (-s.v[711])) { 1.0 } else { 0.0 };

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_neg(683, 707);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_mul_ad_lhs(684, A::scale(s.ad_value(683), 1.25), 712);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_scale_ad(685, A::sub(A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(684), (-6.0)), A::offset(s.ad_value(684), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_add_ad(686, A::mul(A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685))), A::mul(s.ad_value(709), A::offset(s.ad_value(685), 1.0)));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_sub_ad_lhs(687, A::scale(A::sub(s.ad_value(683), s.ad_value(685)), 2.0), 709);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_add(689, 686, 687);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_add_ad(690, A::square(s.ad_value(689)), A::mul(s.ad_value(688), A::sub(A::mul(A::scale(s.ad_value(687), 0.5), s.ad_value(687)), s.ad_value(686))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_add_ad_rhs(691, 690, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688)), s.ad_value(688)), s.ad_value(687)), A::sub(A::scale(A::square(s.ad_value(687)), 0.3333333333333), s.ad_value(686))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_add_ad_rhs(692, 685, A::div(A::mul(A::mul(s.ad_value(686), s.ad_value(689)), s.ad_value(688)), s.ad_value(691)));
        }

        s.v[1243] = if (((s.v[692]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) && (s.v[1243] != 0.0)) {
            s.store_exp(693, 692);
        }

        s.v[1244] = if (s.v[692] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(692)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(s.ad_value(692), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_sub(691, 683, 692);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_add_ad(694, A::scale(s.ad_value(691), 2.0), A::mul(s.ad_value(709), A::offset(s.ad_value(693), (-1.0))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_add_ad(695, A::square(s.ad_value(691)), A::mul(s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_div_ad(697, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_neg_ad(717, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_mul_ad_lhs(698, A::offset(A::mul(A::scale(s.ad_value(710), 1.25), s.ad_value(713)), (-1.0)), 713);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_mul_ad(699, A::mul(s.ad_value(707), s.ad_value(712)), A::offset(A::mul(s.ad_value(698), s.ad_value(707)), 1.0));
        }

        s.v[1245] = if ((((-s.v[699])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) && (s.v[1245] != 0.0)) {
            s.store_exp_ad(691, A::neg(s.ad_value(699)));
        }

        s.v[1246] = if ((-s.v[699]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) && (!(s.v[1245] != 0.0))) && (s.v[1246] != 0.0)) {
            s.store_div_from_scalar_ad(691, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) && (!(s.v[1245] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_scale_ad(691, A::offset(A::mul(A::offset(A::neg(s.ad_value(699)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_sub_from_scalar(697, 1.0, 691);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_sub_ad(700, A::add(s.ad_value(707), A::scale(s.ad_value(709), 0.5)), A::mul(s.ad_value(708), A::sqrt(A::sub(A::add(s.ad_value(707), A::scale(s.ad_value(709), 0.25)), s.ad_value(697)))));
        }

        s.v[1247] = if ((((-s.v[700])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) && (s.v[1247] != 0.0)) {
            s.store_exp_ad(693, A::neg(s.ad_value(700)));
        }

        s.v[1248] = if ((-s.v[700]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) && (!(s.v[1247] != 0.0))) && (s.v[1248] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(A::neg(s.ad_value(700)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_add_ad(694, A::scale(A::sub(s.ad_value(707), s.ad_value(700)), 2.0), A::mul(s.ad_value(709), A::sub_from_scalar(1.0, s.ad_value(693))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_sub_ad(695, A::mul(A::sub(s.ad_value(707), s.ad_value(700)), A::sub(s.ad_value(707), s.ad_value(700))), A::mul(s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_div_ad(701, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_add(717, 700, 701);
        }

        if ((s.v[1240] != 0.0) && (!(s.v[1241] != 0.0))) {
            s.store_neg(717, 717);
        }

        s.store_mul_ad(718, A::neg(s.ad_value(223)), A::add(s.ad_value(704), s.ad_value(714)));

        s.store_mul_ad(719, A::neg(s.ad_value(223)), A::add(s.ad_value(705), s.ad_value(715)));

        s.store_mul_ad(349, A::neg(s.ad_value(223)), A::add(s.ad_value(706), s.ad_value(716)));

        s.store_mul_ad(350, A::neg(s.ad_value(223)), A::add(s.ad_value(707), s.ad_value(717)));

        s.v[733] = 0.0;

        s.v[734] = 0.0;

        s.v[351] = 0.0;

        s.v[352] = 0.0;

        s.v[353] = 0.0;

        s.v[753] = 0.0;

        s.v[754] = 0.0;

        s.v[1249] = if (p.p3 > 0.0) { 1.0 } else { 0.0 };

        s.v[1250] = if ((s.v[69] > 0.0) || (s.v[71] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add(720, 718, 285);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_scale_ad(721, A::sub(s.ad_value(720), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(720)), A::neg(s.ad_value(720))), 0.01))), 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_mul_ad_lhs(722, A::sqrt(A::offset(A::square(s.ad_value(718)), 0.0001)), 276);
        }

        s.v[1251] = if ((((0.5 * s.v[704])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_exp_ad(0, A::scale(s.ad_value(704), 0.5));
        }

        s.v[1252] = if ((0.5 * s.v[704]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::scale(s.ad_value(704), 0.5)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::scale(s.ad_value(704), 0.5)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::scale(s.ad_value(704), 0.5)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(A::scale(s.ad_value(704), 0.5), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::scale(s.ad_value(704), 0.5), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::scale(s.ad_value(704), 0.5), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_div_from_scalar_ad(2, 1.0, A::offset(s.ad_value(0), 1.0));
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_sub_from_scalar(3, 1.0, 2);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add_ad(723, A::mul(s.ad_value(83), s.ad_value(2)), A::mul(s.ad_value(80), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add_ad(724, A::mul(s.ad_value(84), s.ad_value(2)), A::mul(s.ad_value(82), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add_ad(725, A::mul(s.ad_value(282), s.ad_value(2)), A::mul(s.ad_value(281), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add_ad(726, A::mul(s.ad_value(71), s.ad_value(2)), A::mul(s.ad_value(69), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_scaled_mul(727, 73, 3, 1e-6);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_mul_ad_rhs(2, 279, A::div(A::scale(s.ad_value(81), (-1.0)), s.ad_value(722)));
        }

        s.v[1253] = if (s.v[724] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_scale_ad(722, A::sub(A::add(s.ad_value(722), s.ad_value(725)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(722), s.ad_value(725)), A::sub(s.ad_value(722), s.ad_value(725))), 1e-6))), 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add_ad(728, A::offset(s.ad_value(714), 3.0), A::mul(s.ad_value(721), s.ad_value(224)));
        }

        s.v[1254] = if (((s.v[728]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1254] != 0.0)) {
            s.store_exp(729, 728);
        }

        s.v[1255] = if (s.v[728] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1254] != 0.0))) && (s.v[1255] != 0.0)) {
            s.store_div_from_scalar_ad(729, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(728)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1254] != 0.0))) && (!(s.v[1255] != 0.0))) {
            s.store_scale_ad(729, A::offset(A::mul(A::offset(s.ad_value(728), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add_ad_lhs(728, A::add(A::offset(s.ad_value(714), 3.0), A::mul(s.ad_value(721), s.ad_value(224))), 704);
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
        s.v[1256] = if (((s.v[728]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1256] != 0.0)) {
            s.store_exp(730, 728);
        }

        s.v[1257] = if (s.v[728] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1256] != 0.0))) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar_ad(730, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(728)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1256] != 0.0))) && (!(s.v[1257] != 0.0))) {
            s.store_scale_ad(730, A::offset(A::mul(A::offset(s.ad_value(728), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_mul_ad_rhs(0, 279, A::offset(A::mul(s.ad_value(722), A::add(s.ad_value(723), A::mul(s.ad_value(724), s.ad_value(722)))), (-1.5)));
        }

        s.v[1258] = if (s.v[0] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1258] != 0.0)) {
            s.store_offset_ad(731, A::mul(s.ad_value(0), A::offset(A::mul(A::scale(s.ad_value(0), 0.5), A::offset(A::scale(s.ad_value(0), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1259] = if (s.v[0] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1258] != 0.0))) && (s.v[1259] != 0.0)) {
            s.store_exp(731, 0);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1258] != 0.0))) && (!(s.v[1259] != 0.0))) {
            s.store_div_from_scalar_ad(731, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        s.v[1260] = if (s.v[2] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1260] != 0.0)) {
            s.store_offset_ad(732, A::mul(s.ad_value(2), A::offset(A::mul(A::scale(s.ad_value(2), 0.5), A::offset(A::scale(s.ad_value(2), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1261] = if (s.v[2] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1260] != 0.0))) && (s.v[1261] != 0.0)) {
            s.store_exp(732, 2);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_div_from_scalar_ad(732, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_div_ad(0, A::offset(s.ad_value(729), 1.0), A::offset(s.ad_value(730), 1.0));
        }

        s.v[1262] = if (s.v[0] < 1e-80) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1262] != 0.0)) {
            s.store_scalar(0, 1e-80);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_mul_ad_rhs(2, 85, A::sub(s.ad_value(332), s.ad_value(86)));
        }

        s.v[1263] = if (((s.v[2]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1263] != 0.0)) {
            s.store_exp(3, 2);
        }

        s.v[1264] = if (s.v[2] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1263] != 0.0))) && (s.v[1264] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1263] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(2), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_add_ad_lhs(4, A::mul(s.ad_value(85), s.ad_value(703)), 2);
        }

        s.v[1265] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (s.v[1265] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1266] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1265] != 0.0))) && (s.v[1266] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) && (!(s.v[1265] != 0.0))) && (!(s.v[1266] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1250] != 0.0)) {
            s.store_sub_ad(733, A::div(A::mul(A::mul(A::mul(s.ad_value(726), s.ad_value(731)), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)), A::div(A::mul(A::mul(s.ad_value(727), s.ad_value(732)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)));
        }

        s.v[1267] = if ((s.v[70] > 0.0) || (s.v[72] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add(720, 719, 285);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_scale_ad(721, A::sub(s.ad_value(720), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(720)), A::neg(s.ad_value(720))), 0.01))), 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_mul_ad_lhs(722, A::sqrt(A::offset(A::square(s.ad_value(719)), 0.0001)), 276);
        }

        s.v[1268] = if ((((0.5 * s.v[705])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1268] != 0.0)) {
            s.store_exp_ad(0, A::scale(s.ad_value(705), 0.5));
        }

        s.v[1269] = if ((0.5 * s.v[705]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1268] != 0.0))) && (s.v[1269] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::scale(s.ad_value(705), 0.5)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::scale(s.ad_value(705), 0.5)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::scale(s.ad_value(705), 0.5)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1268] != 0.0))) && (!(s.v[1269] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(A::scale(s.ad_value(705), 0.5), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::scale(s.ad_value(705), 0.5), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::scale(s.ad_value(705), 0.5), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_div_from_scalar_ad(2, 1.0, A::offset(s.ad_value(0), 1.0));
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_sub_from_scalar(3, 1.0, 2);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add_ad(723, A::mul(s.ad_value(83), s.ad_value(2)), A::mul(s.ad_value(80), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add_ad(724, A::mul(s.ad_value(84), s.ad_value(2)), A::mul(s.ad_value(82), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add_ad(725, A::mul(s.ad_value(282), s.ad_value(2)), A::mul(s.ad_value(281), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add_ad(726, A::mul(s.ad_value(72), s.ad_value(2)), A::mul(s.ad_value(70), s.ad_value(3)));
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_scaled_mul(727, 74, 3, 1e-6);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_mul_ad_rhs(2, 279, A::div(A::scale(s.ad_value(81), (-1.0)), s.ad_value(722)));
        }

        s.v[1270] = if (s.v[724] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1270] != 0.0)) {
            s.store_scale_ad(722, A::sub(A::add(s.ad_value(722), s.ad_value(725)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(722), s.ad_value(725)), A::sub(s.ad_value(722), s.ad_value(725))), 1e-6))), 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add_ad(728, A::offset(s.ad_value(715), 3.0), A::mul(s.ad_value(721), s.ad_value(224)));
        }

        s.v[1271] = if (((s.v[728]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_exp(729, 728);
        }

        s.v[1272] = if (s.v[728] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1272] != 0.0)) {
            s.store_div_from_scalar_ad(729, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(728)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1272] != 0.0))) {
            s.store_scale_ad(729, A::offset(A::mul(A::offset(s.ad_value(728), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add_ad_lhs(728, A::add(A::offset(s.ad_value(715), 3.0), A::mul(s.ad_value(721), s.ad_value(224))), 705);
        }

        s.v[1273] = if (((s.v[728]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_exp(730, 728);
        }

        s.v[1274] = if (s.v[728] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1274] != 0.0)) {
            s.store_div_from_scalar_ad(730, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(728)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1274] != 0.0))) {
            s.store_scale_ad(730, A::offset(A::mul(A::offset(s.ad_value(728), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_mul_ad_rhs(0, 279, A::offset(A::mul(s.ad_value(722), A::add(s.ad_value(723), A::mul(s.ad_value(724), s.ad_value(722)))), (-1.5)));
        }

        s.v[1275] = if (s.v[0] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1275] != 0.0)) {
            s.store_offset_ad(731, A::mul(s.ad_value(0), A::offset(A::mul(A::scale(s.ad_value(0), 0.5), A::offset(A::scale(s.ad_value(0), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1276] = if (s.v[0] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1275] != 0.0))) && (s.v[1276] != 0.0)) {
            s.store_exp(731, 0);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1275] != 0.0))) && (!(s.v[1276] != 0.0))) {
            s.store_div_from_scalar_ad(731, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        s.v[1277] = if (s.v[2] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1277] != 0.0)) {
            s.store_offset_ad(732, A::mul(s.ad_value(2), A::offset(A::mul(A::scale(s.ad_value(2), 0.5), A::offset(A::scale(s.ad_value(2), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1278] = if (s.v[2] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1277] != 0.0))) && (s.v[1278] != 0.0)) {
            s.store_exp(732, 2);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1277] != 0.0))) && (!(s.v[1278] != 0.0))) {
            s.store_div_from_scalar_ad(732, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_div_ad(0, A::offset(s.ad_value(729), 1.0), A::offset(s.ad_value(730), 1.0));
        }

        s.v[1279] = if (s.v[0] < 1e-80) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1279] != 0.0)) {
            s.store_scalar(0, 1e-80);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_mul_ad_rhs(2, 85, A::sub(s.ad_value(330), s.ad_value(86)));
        }

        s.v[1280] = if (((s.v[2]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1280] != 0.0)) {
            s.store_exp(3, 2);
        }

        s.v[1281] = if (s.v[2] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1280] != 0.0))) && (s.v[1281] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(2), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_add_ad_lhs(4, A::mul(s.ad_value(85), s.ad_value(702)), 2);
        }

        s.v[1282] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (s.v[1282] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1283] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1282] != 0.0))) && (s.v[1283] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1267] != 0.0)) {
            s.store_sub_ad(734, A::div(A::mul(A::mul(A::mul(s.ad_value(726), s.ad_value(731)), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)), A::div(A::mul(A::mul(s.ad_value(727), s.ad_value(732)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)));
        }

        s.v[1284] = if (s.v[68] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul_ad_lhs(735, A::neg(s.ad_value(436)), 386);
        }

        s.v[1285] = if (((((2.0 * s.v[735]) - s.v[411])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (s.v[1285] != 0.0)) {
            s.store_exp_ad(0, A::sub(A::scale(s.ad_value(735), 2.0), s.ad_value(411)));
        }

        s.v[1286] = if (((2.0 * s.v[735]) - s.v[411]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1285] != 0.0))) && (s.v[1286] != 0.0)) {
            let assign26710_ad_e28192: A = A::div_from_scalar(1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::sub(A::scale(s.ad_value(735), 2.0), s.ad_value(411))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::sub(A::scale(s.ad_value(735), 2.0), s.ad_value(411))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::sub(A::scale(s.ad_value(735), 2.0), s.ad_value(411))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
            s.store_ad(0, &assign26710_ad_e28192);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1285] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(A::sub(A::scale(s.ad_value(735), 2.0), s.ad_value(411)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::scale(s.ad_value(735), 2.0), s.ad_value(411)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::scale(s.ad_value(735), 2.0), s.ad_value(411)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul_ad_rhs(736, 226, A::sub(A::offset(s.ad_value(735), 0.6931471805599), A::ln(A::offset(s.ad_value(0), 1.0))));
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_scaled_add(737, 392, 412, 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul(738, 226, 737);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_add(720, 738, 284);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_scale_ad(721, A::sub(s.ad_value(720), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(720)), A::neg(s.ad_value(720))), 0.01))), 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul_ad_lhs(722, A::sqrt(A::offset(A::square(s.ad_value(738)), 0.0001)), 276);
        }

        s.v[1287] = if (s.v[79] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (s.v[1287] != 0.0)) {
            s.store_scale_ad(722, A::sub(A::add(s.ad_value(722), s.ad_value(280)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(722), s.ad_value(280)), A::sub(s.ad_value(722), s.ad_value(280))), 1e-6))), 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_add(740, 400, 234);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_sub(739, 740, 737);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul_ad_lhs(728, A::add(s.ad_value(739), A::mul(A::sub(A::sub(s.ad_value(721), s.ad_value(283)), s.ad_value(736)), s.ad_value(227))), 286);
        }

        s.v[1288] = if (((s.v[728]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (s.v[1288] != 0.0)) {
            s.store_exp(729, 728);
        }

        s.v[1289] = if (s.v[728] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1288] != 0.0))) && (s.v[1289] != 0.0)) {
            s.store_div_from_scalar_ad(729, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(728)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1288] != 0.0))) && (!(s.v[1289] != 0.0))) {
            s.store_scale_ad(729, A::offset(A::mul(A::offset(s.ad_value(728), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul_ad_lhs(728, A::mul(A::neg(A::sub(s.ad_value(335), s.ad_value(736))), s.ad_value(227)), 286);
        }

        s.v[1290] = if (((s.v[728]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (s.v[1290] != 0.0)) {
            s.store_exp(0, 728);
        }

        s.v[1291] = if (s.v[728] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(728)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1290] != 0.0))) && (!(s.v[1291] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(s.ad_value(728), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(728), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul(730, 729, 0);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul_ad_rhs(0, 278, A::offset(A::mul(s.ad_value(722), A::add(s.ad_value(78), A::mul(s.ad_value(79), s.ad_value(722)))), (-1.5)));
        }

        s.v[1292] = if (s.v[0] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (s.v[1292] != 0.0)) {
            s.store_offset_ad(731, A::mul(s.ad_value(0), A::offset(A::mul(A::scale(s.ad_value(0), 0.5), A::offset(A::scale(s.ad_value(0), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1293] = if (((s.v[0]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1292] != 0.0))) && (s.v[1293] != 0.0)) {
            s.store_exp(731, 0);
        }

        s.v[1294] = if (s.v[0] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1292] != 0.0))) && (!(s.v[1293] != 0.0))) && (s.v[1294] != 0.0)) {
            s.store_div_from_scalar_ad(731, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1292] != 0.0))) && (!(s.v[1293] != 0.0))) && (!(s.v[1294] != 0.0))) {
            s.store_scale_ad(731, A::offset(A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul_ad(741, A::mul(s.ad_value(68), s.ad_value(731)), A::ln(A::div(A::offset(s.ad_value(729), 1.0), A::offset(s.ad_value(730), 1.0))));
        }

        s.v[1295] = if ((s.v[740] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0))) { 1.0 } else { 0.0 };

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (s.v[1295] != 0.0)) {
            s.store_scalar(742, 1.0);
        }

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (s.v[1295] != 0.0)) {
            s.store_scalar(743, 0.5);
        }

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) {
            s.store_add_ad_rhs(0, 78, A::mul(A::scale(s.ad_value(79), 2.0), s.ad_value(722)));
        }

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) {
            s.store_mul_ad_lhs(744, A::div(s.ad_value(87), A::mul(s.ad_value(0), s.ad_value(278))), 227);
        }

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) {
            s.store_div(745, 735, 744);
        }

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) {
            s.store_mul_ad_lhs(746, A::mul(s.ad_value(744), s.ad_value(434)), 401);
        }

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) {
            s.store_scale_ad(748, A::mul(s.ad_value(746), A::sub_from_scalar(1.0, s.ad_value(746))), 0.5);
        }

        if (((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) {
            s.store_sub_from_scalar_ad(747, 0.5, A::scale(s.ad_value(748), 3.0));
        }

        s.v[1296] = if (s.v[745] < 0.001) { 1.0 } else { 0.0 };

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (s.v[1296] != 0.0)) {
            s.store_square(749, 745);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (s.v[1296] != 0.0)) {
            s.store_offset_ad(742, A::mul(s.ad_value(749), A::add(A::offset(A::scale(s.ad_value(746), 0.3333333333333), 0.1666666666667), A::mul(A::scale(s.ad_value(749), 0.1666666666667), A::offset(A::scale(s.ad_value(746), 0.2), 0.05)))), 1.0);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (s.v[1296] != 0.0)) {
            s.store_sub_ad(743, A::scale(s.ad_value(742), 0.5), A::mul(A::scale(s.ad_value(745), 0.1666666666667), A::offset(A::mul(s.ad_value(749), A::add(A::scale(A::offset(s.ad_value(748), 0.25), 0.4), A::mul(A::scale(s.ad_value(749), 0.0285714285714), A::offset(s.ad_value(748), 0.125)))), 1.0)));
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_div_from_scalar(750, 1.0, 745);
        }

        s.v[1297] = if (((s.v[745]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) && (s.v[1297] != 0.0)) {
            s.store_exp(751, 745);
        }

        s.v[1298] = if (s.v[745] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) && (!(s.v[1297] != 0.0))) && (s.v[1298] != 0.0)) {
            s.store_div_from_scalar_ad(751, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(745)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(745)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(745)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) && (!(s.v[1297] != 0.0))) && (!(s.v[1298] != 0.0))) {
            s.store_scale_ad(751, A::offset(A::mul(A::offset(s.ad_value(745), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(745), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(745), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_div_from_scalar(752, 1.0, 751);
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
        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_sub(0, 751, 752);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_add(3, 751, 752);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_scale_ad(742, A::add(A::mul(A::mul(A::sub_from_scalar(1.0, s.ad_value(746)), s.ad_value(0)), s.ad_value(750)), A::mul(s.ad_value(746), s.ad_value(3))), 0.5);
        }

        if ((((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_scale_ad(743, A::sub(A::sub(s.ad_value(742), A::mul(s.ad_value(0), A::sub(s.ad_value(748), A::mul(A::mul(s.ad_value(747), s.ad_value(750)), s.ad_value(750))))), A::mul(A::mul(s.ad_value(747), s.ad_value(3)), s.ad_value(750))), 0.5);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul(351, 741, 742);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_mul(754, 741, 743);
        }

        if ((s.v[1249] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_sub(753, 351, 754);
        }

        s.v[1299] = if (s.v[334] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1249] != 0.0) && (s.v[1299] != 0.0)) {
            s.store_add(352, 754, 733);
        }

        if ((s.v[1249] != 0.0) && (s.v[1299] != 0.0)) {
            s.store_add(353, 753, 734);
        }

        if ((s.v[1249] != 0.0) && (!(s.v[1299] != 0.0))) {
            s.store_add(352, 753, 733);
        }

        if ((s.v[1249] != 0.0) && (!(s.v[1299] != 0.0))) {
            s.store_add(353, 754, 734);
        }

        s.v[355] = 0.0;

        s.v[1300] = if (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[718] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1300] != 0.0) {
            s.store_sqrt_ad(755, A::offset(A::add(A::square(s.ad_value(718)), A::mul(A::mul(A::square(s.ad_value(95)), s.ad_value(331)), s.ad_value(331))), 1e-6));
        }

        if (s.v[1300] != 0.0) {
            s.store_div_ad_lhs(0, A::neg(s.ad_value(91)), 755);
        }

        s.v[1301] = if (((s.v[0]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1300] != 0.0) && (s.v[1301] != 0.0)) {
            s.store_exp(3, 0);
        }

        s.v[1302] = if (s.v[0] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1300] != 0.0) && (!(s.v[1301] != 0.0))) && (s.v[1302] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1300] != 0.0) && (!(s.v[1301] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1300] != 0.0) {
            s.store_mul(4, 97, 703);
        }

        s.v[1303] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1300] != 0.0) && (s.v[1303] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1304] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1300] != 0.0) && (!(s.v[1303] != 0.0))) && (s.v[1304] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1300] != 0.0) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1300] != 0.0) {
            s.store_mul_ad(355, A::scale(A::mul(A::mul(A::mul(A::mul(A::neg(s.ad_value(89)), s.ad_value(703)), s.ad_value(718)), s.ad_value(755)), s.ad_value(3)), 0.5), A::offset(s.ad_value(5), 1.0));
        }

        s.v[354] = 0.0;

        s.v[1305] = if (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[719] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1305] != 0.0) {
            s.store_sqrt_ad(756, A::offset(A::add(A::square(s.ad_value(719)), A::mul(A::mul(A::square(s.ad_value(96)), s.ad_value(333)), s.ad_value(333))), 1e-6));
        }

        if (s.v[1305] != 0.0) {
            s.store_div_ad_lhs(0, A::neg(s.ad_value(92)), 756);
        }

        s.v[1306] = if (((s.v[0]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1305] != 0.0) && (s.v[1306] != 0.0)) {
            s.store_exp(3, 0);
        }

        s.v[1307] = if (s.v[0] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1305] != 0.0) && (!(s.v[1306] != 0.0))) && (s.v[1307] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1305] != 0.0) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1305] != 0.0) {
            s.store_mul(4, 98, 702);
        }

        s.v[1308] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1305] != 0.0) && (s.v[1308] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1309] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1305] != 0.0) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1305] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1305] != 0.0) {
            s.store_mul_ad(354, A::scale(A::mul(A::mul(A::mul(A::mul(A::neg(s.ad_value(90)), s.ad_value(702)), s.ad_value(719)), s.ad_value(756)), s.ad_value(3)), 0.5), A::offset(s.ad_value(5), 1.0));
        }

        s.v[356] = 0.0;

        s.v[1310] = if (p.p12 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1310] != 0.0) {
            s.store_mul(758, 336, 289);
        }

        if (s.v[1310] != 0.0) {
            s.store_mul_ad_lhs(759, A::offset(A::sqrt(A::offset(A::square(s.ad_value(336)), 0.01)), (-0.1)), 289);
        }

        if (s.v[1310] != 0.0) {
            s.store_scaled_sub(760, 758, 759, 0.5);
        }

        if (s.v[1310] != 0.0) {
            s.store_sub_ad_lhs(761, A::sub(A::mul(A::sub(s.ad_value(335), s.ad_value(100)), s.ad_value(289)), s.ad_value(760)), 234);
        }

        if (s.v[1310] != 0.0) {
            s.store_sub_ad_lhs(762, A::sub(A::mul(A::sub(A::neg(s.ad_value(337)), s.ad_value(101)), s.ad_value(289)), s.ad_value(760)), 234);
        }

        if (s.v[1310] != 0.0) {
            s.store_div_from_scalar_ad(763, 1.0, A::offset(s.ad_value(105), 1.0));
        }

        if (s.v[1310] != 0.0) {
            s.store_div_from_scalar_ad(764, 1.0, A::offset(s.ad_value(106), 1.0));
        }

        if (s.v[1310] != 0.0) {
            s.store_mul(765, 109, 289);
        }

        if (s.v[1310] != 0.0) {
            s.store_mul_ad(0, A::scale(s.ad_value(765), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(759), s.ad_value(765)), 1.0)), (-1.0)));
        }

        if (s.v[1310] != 0.0) {
            s.store_mul(766, 107, 0);
        }

        if (s.v[1310] != 0.0) {
            s.store_mul(767, 108, 0);
        }

        if (s.v[1310] != 0.0) {
            s.store_add_ad_lhs(768, A::mul(A::add(s.ad_value(761), s.ad_value(766)), s.ad_value(763)), 760);
        }

        if (s.v[1310] != 0.0) {
            s.store_add_ad_lhs(769, A::mul(A::add(s.ad_value(762), s.ad_value(767)), s.ad_value(764)), 760);
        }

        if (s.v[1310] != 0.0) {
            let assign27840_ad_e29793: A = A::sub(A::add(A::add(s.ad_value(769), A::mul(s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)))), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(769), A::mul(s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)))), s.ad_value(225)), A::sub(A::add(s.ad_value(769), A::mul(s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)))), s.ad_value(225))), 0.01)));
            s.store_scale_ad(770, assign27840_ad_e29793, 0.5);
        }

        if (s.v[1310] != 0.0) {
            let assign27850_ad_e29830: A = A::sub(A::add(A::add(s.ad_value(768), A::mul(s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)))), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(768), A::mul(s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)))), s.ad_value(225)), A::sub(A::add(s.ad_value(768), A::mul(s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)))), s.ad_value(225))), 0.01)));
            s.store_scale_ad(771, assign27850_ad_e29830, 0.5);
        }

        if (s.v[1310] != 0.0) {
            s.store_div(772, 246, 763);
        }

        if (s.v[1310] != 0.0) {
            s.store_div(773, 247, 764);
        }

        if (s.v[1310] != 0.0) {
            s.store_div_from_scalar(774, 1.0, 772);
        }

        if (s.v[1310] != 0.0) {
            s.store_div_from_scalar(775, 1.0, 773);
        }

        if (s.v[1310] != 0.0) {
            s.store_div_from_scalar_ad(776, 1.0, A::add(A::offset(s.ad_value(774), 1.0), s.ad_value(775)));
        }

        if (s.v[1310] != 0.0) {
            s.store_div_ad_rhs(777, 290, A::square(s.ad_value(390)));
        }

        if (s.v[1310] != 0.0) {
            s.store_mul_ad_rhs(778, 776, A::sub(s.ad_value(770), s.ad_value(771)));
        }

        s.v[1311] = if ((((s.v[771] - s.v[770])) as f64).abs() <= 1e-12) { 1.0 } else { 0.0 };

        if ((s.v[1310] != 0.0) && (s.v[1311] != 0.0)) {
            s.store_sub_ad(2, A::sub_from_scalar(1.0, A::mul(s.ad_value(776), s.ad_value(774))), A::mul(s.ad_value(776), s.ad_value(775)));
        }

        if ((s.v[1310] != 0.0) && (s.v[1311] != 0.0)) {
            s.store_mul_ad_lhs(3, A::sub(A::sub(A::add(s.ad_value(775), A::mul(A::mul(A::scale(s.ad_value(774), 0.5), s.ad_value(776)), s.ad_value(774))), A::mul(A::mul(A::scale(s.ad_value(775), 0.5), s.ad_value(776)), s.ad_value(775))), A::div_from_scalar(0.5, s.ad_value(776))), 778);
        }

        if ((s.v[1310] != 0.0) && (s.v[1311] != 0.0)) {
            s.store_div_ad_lhs(4, A::mul(A::scale(A::sub(s.ad_value(2), s.ad_value(3)), 0.5), s.ad_value(777)), 776);
        }

        if ((s.v[1310] != 0.0) && (!(s.v[1311] != 0.0))) {
            s.store_exp_ad(2, A::mul(A::neg(s.ad_value(774)), s.ad_value(778)));
        }

        if ((s.v[1310] != 0.0) && (!(s.v[1311] != 0.0))) {
            s.store_exp_ad(3, A::mul(A::sub(s.ad_value(775), A::div_from_scalar(1.0, s.ad_value(776))), s.ad_value(778)));
        }

        if ((s.v[1310] != 0.0) && (!(s.v[1311] != 0.0))) {
            s.store_div_ad(4, A::mul(s.ad_value(777), A::sub(s.ad_value(2), s.ad_value(3))), A::scale(s.ad_value(778), 2.0));
        }

        if (s.v[1310] != 0.0) {
            s.copy_ad(779, 4);
        }

        s.v[1312] = if (s.v[770] < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1310] != 0.0) && (s.v[1312] != 0.0)) {
            s.store_ln_ad(784, A::offset(A::mul(s.ad_value(779), A::exp(s.ad_value(770))), 1.0));
        }

        if ((s.v[1310] != 0.0) && (s.v[1312] != 0.0)) {
            s.store_mul_ad_rhs(0, 784, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0))));
        }

        s.v[1313] = if (s.v[770] < 0.0) { 1.0 } else { 0.0 };

        s.v[1314] = if (s.v[770] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1310] != 0.0) && (!(s.v[1312] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
            s.store_exp(784, 770);
        }

        if ((((s.v[1310] != 0.0) && (!(s.v[1312] != 0.0))) && (s.v[1313] != 0.0)) && (!(s.v[1314] != 0.0))) {
            s.store_div_from_scalar_ad(784, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(770)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(770)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(770)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1310] != 0.0) && (!(s.v[1312] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_mul(0, 779, 784);
        }

        if (((s.v[1310] != 0.0) && (!(s.v[1312] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_add_ad_lhs(784, A::ln(s.ad_value(779)), 770);
        }

        if (((s.v[1310] != 0.0) && (!(s.v[1312] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_mul_ad_rhs(0, 784, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0))));
        }

        if (s.v[1310] != 0.0) {
            s.copy_ad(780, 0);
        }

        s.v[1315] = if ((s.v[770] - s.v[411]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1310] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_ln_ad(784, A::offset(A::mul(s.ad_value(779), A::exp(A::sub(s.ad_value(770), s.ad_value(411)))), 1.0));
        }

        if ((s.v[1310] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_mul_ad_rhs(0, 784, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0))));
        }

        s.v[1316] = if ((s.v[770] - s.v[411]) < 0.0) { 1.0 } else { 0.0 };

        s.v[1317] = if ((s.v[770] - s.v[411]) > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1310] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (s.v[1317] != 0.0)) {
            s.store_exp_ad(784, A::sub(s.ad_value(770), s.ad_value(411)));
        }

        if ((((s.v[1310] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (!(s.v[1317] != 0.0))) {
            s.store_div_from_scalar_ad(784, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::sub(s.ad_value(770), s.ad_value(411))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::sub(s.ad_value(770), s.ad_value(411))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::sub(s.ad_value(770), s.ad_value(411))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1310] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_mul(0, 779, 784);
        }

        if (((s.v[1310] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_add_ad(784, A::ln(s.ad_value(779)), A::sub(s.ad_value(770), s.ad_value(411)));
        }

        if (((s.v[1310] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_mul_ad_rhs(0, 784, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0))));
        }

        if (s.v[1310] != 0.0) {
            s.copy_ad(781, 0);
        }

        if (s.v[1310] != 0.0) {
            s.store_mul_ad(782, A::offset(A::scale(A::add(s.ad_value(780), s.ad_value(781)), 0.5), 1.0), A::sub(s.ad_value(780), s.ad_value(781)));
        }

        if (s.v[1310] != 0.0) {
            s.store_mul_ad_lhs(783, A::square(s.ad_value(288)), 110);
        }

        if (s.v[1310] != 0.0) {
            s.store_div_ad_lhs(356, A::mul(A::mul(s.ad_value(783), s.ad_value(241)), s.ad_value(782)), 422);
        }

        s.v[357] = 0.0;

        s.v[358] = 0.0;

        s.v[1318] = if (p.p8 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1318] != 0.0) {
            s.store_div_ad_lhs(757, A::sub(s.ad_value(339), A::mul(s.ad_value(115), s.ad_value(411))), 227);
        }

        s.v[1319] = if (s.v[757] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1318] != 0.0) && (s.v[1319] != 0.0)) {
            s.store_div_ad(3, A::scale(s.ad_value(113), (-1.0)), A::offset(s.ad_value(757), 1e-30));
        }

        s.v[1320] = if (((s.v[3]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1318] != 0.0) && (s.v[1319] != 0.0)) && (s.v[1320] != 0.0)) {
            s.store_exp(0, 3);
        }

        s.v[1321] = if (s.v[3] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1318] != 0.0) && (s.v[1319] != 0.0)) && (!(s.v[1320] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(3)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1318] != 0.0) && (s.v[1319] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(s.ad_value(3), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(3), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(3), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1318] != 0.0) && (s.v[1319] != 0.0)) {
            s.store_mul_ad_lhs(357, A::mul(s.ad_value(112), s.ad_value(757)), 0);
        }

        if ((s.v[1318] != 0.0) && (s.v[1319] != 0.0)) {
            s.store_mul_ad_rhs(358, 357, A::add(s.ad_value(348), s.ad_value(356)));
        }

        s.v[1322] = if (s.v[6] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1322] != 0.0) {
            s.store_mul_ad_lhs(0, A::abs(A::mul(A::add(s.ad_value(348), s.ad_value(356)), s.ad_value(336))), 168);
        }

        s.v[1323] = if (s.v[0] > (100000000.0 * p.p16)) { 1.0 } else { 0.0 };

        if ((s.v[1322] != 0.0) && (s.v[1323] != 0.0)) {
            s.store_div_from_scalar(359, (-(p.p16 + (0.25 / p.p16))), 168);
        }

        if ((s.v[1322] != 0.0) && (!(s.v[1323] != 0.0))) {
            s.store_div_ad_lhs(359, A::neg(A::offset(A::scale(A::sub(A::offset(s.ad_value(0), p.p16), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(0), (-p.p16)), A::offset(s.ad_value(0), (-p.p16))), 1.0))), 0.5), (0.25 / p.p16))), 168);
        }

        if (s.v[1322] != 0.0) {
            s.store_div(360, 219, 168);
        }

        if (!(s.v[1322] != 0.0)) {
            s.store_scalar(359, 0.0);
        }

        if (!(s.v[1322] != 0.0)) {
            s.store_ad(360, &A::scale(A::voltage(ctx, &nodes, Some(4), None), 0.001));
        }

        s.store_mul(317, 306, 316);

        s.store_mul(321, 306, 320);

        s.store_mul(325, 306, 324);

        s.store_mul(328, 306, 327);

        s.v[1608] = if (p.p11 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1608] != 0.0) {
            s.copy_ad(1418, 130);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1419, 131);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1420, 135);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1421, 136);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1422, 140);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1423, 141);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1424, 274);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1425, 216);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(1426, 158);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1427, A::sub(A::mul(A::sub(s.ad_value(335), s.ad_value(1418)), s.ad_value(227)), s.ad_value(341)), 234);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1428, A::mul(A::sub(A::neg(s.ad_value(337)), s.ad_value(1419)), s.ad_value(227)), 341);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1429, 1428, 234);
        }

        s.v[1609] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_scale(0, 16, p.p14);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_ad(1430, A::offset(s.ad_value(246), 1.0), A::offset(s.ad_value(247), 1.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_ln(1431, 1430);
        }

        s.v[1610] = if (s.v[1431] > 1e-8) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (s.v[1610] != 0.0)) {
            s.store_div_ad(1432, A::mul(A::scale(s.ad_value(1431), 2.0), A::offset(s.ad_value(1430), 1.0)), A::offset(s.ad_value(1430), (-1.0)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1610] != 0.0))) {
            s.store_scaled_offset(1432, 1431, 2.0, 2.0);
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
        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_ad_rhs(1433, 253, A::square(s.ad_value(245)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_from_scalar(1434, 1.0, 246);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_from_scalar(1435, 1.0, 247);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_from_scalar_ad(1462, 1.0, A::add(A::offset(s.ad_value(1434), 1.0), s.ad_value(1435)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_mul_ad_rhs(1463, 1462, A::sub(s.ad_value(1427), s.ad_value(1429)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_sub_ad_rhs(1436, 1427, A::mul(s.ad_value(1463), s.ad_value(1434)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_add_ad_rhs(1437, 1429, A::mul(s.ad_value(1463), s.ad_value(1435)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_from_scalar_ad(1342, 1.0, A::offset(s.ad_value(246), 1.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_from_scalar_ad(1343, 1.0, A::offset(s.ad_value(247), 1.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_offset_ad(1345, A::ln(A::div(A::mul(A::add(s.ad_value(246), A::mul(s.ad_value(247), s.ad_value(1343))), s.ad_value(1432)), s.ad_value(1433))), 1.5);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_offset_ad(1346, A::ln(A::div(A::mul(A::add(s.ad_value(247), A::mul(s.ad_value(246), s.ad_value(1342))), s.ad_value(1432)), s.ad_value(1433))), 1.5);
        }

        s.v[1611] = if (((s.v[1345] - s.v[1436]) / 1.5) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (s.v[1611] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1345), s.ad_value(1436)), 0.6666666666666666)), 1.0));
        }

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1611] != 0.0))) {
            s.store_scaled_sub(1344, 1345, 1436, 0.6666666666666666);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_sub_ad_rhs(1349, 1345, A::scale(s.ad_value(1344), 1.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_mul_ad_lhs(1348, A::add(A::mul(s.ad_value(247), s.ad_value(1429)), s.ad_value(1349)), 1343);
        }

        s.v[1612] = if (((s.v[1346] - s.v[1348]) / 1.5) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (s.v[1612] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1346), s.ad_value(1348)), 0.6666666666666666)), 1.0));
        }

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1612] != 0.0))) {
            s.store_scaled_sub(1344, 1346, 1348, 0.6666666666666666);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_sub_ad_rhs(1, 1346, A::scale(s.ad_value(1344), 1.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_mul(2, 0, 1);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_mul(3, 0, 1429);
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_sub(1394, 2, 3);
        }

        s.v[1613] = if ((((-s.v[266])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (s.v[1613] != 0.0)) {
            s.store_exp_ad(1395, A::neg(s.ad_value(266)));
        }

        s.v[1614] = if ((-s.v[266]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1613] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_div_from_scalar_ad(1395, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1613] != 0.0))) && (!(s.v[1614] != 0.0))) {
            s.store_scale_ad(1395, A::offset(A::mul(A::offset(A::neg(s.ad_value(266)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(266)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(266)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.v[1615] = if (((s.v[1394]) as f64).abs() <= s.v[265]) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (s.v[1615] != 0.0)) {
            s.store_scale_ad(1392, A::square(s.ad_value(264)), (0.1666666666667 * 0.707106781186545));
        }

        if (((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (s.v[1615] != 0.0)) {
            s.store_mul_ad(4, A::mul(s.ad_value(1394), s.ad_value(264)), A::offset(A::mul(A::mul(A::mul(s.ad_value(1394), A::sub_from_scalar(1.0, s.ad_value(1395))), s.ad_value(260)), s.ad_value(1392)), 1.0));
        }

        s.v[1616] = if (s.v[1394] < (-s.v[265])) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_neg(1396, 1394);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_scaled_mul(1397, 1396, 264, 1.25);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_scale_ad(1398, A::sub(A::offset(s.ad_value(1397), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1397), (-6.0)), A::offset(s.ad_value(1397), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub(1391, 1396, 1398);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_add_ad(1399, A::square(s.ad_value(1391)), A::mul(s.ad_value(261), A::offset(s.ad_value(1398), 1.0)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub_ad_lhs(1401, A::scale(s.ad_value(1391), 2.0), 261);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub_ad_lhs(1402, A::ln(A::mul(s.ad_value(1399), s.ad_value(262))), 1398);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_add(1389, 1399, 1401);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_add_ad(1390, A::square(s.ad_value(1389)), A::mul(s.ad_value(1402), A::sub(A::mul(A::scale(s.ad_value(1401), 0.5), s.ad_value(1401)), s.ad_value(1399))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_add_ad_rhs(1403, 1398, A::div(A::mul(A::mul(s.ad_value(1399), s.ad_value(1389)), s.ad_value(1402)), A::add(s.ad_value(1390), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402)), s.ad_value(1402)), s.ad_value(1401)), A::sub(A::scale(A::square(s.ad_value(1401)), 0.3333333333333), s.ad_value(1399))))));
        }

        s.v[1617] = if (s.v[1403] < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) {
            s.store_exp(1404, 1403);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) {
            s.store_scale_ad(1404, A::offset(A::mul(A::offset(s.ad_value(1403), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(1403), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(1403), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_div_from_scalar(1405, 1.0, 1404);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_div_from_scalar_ad(1391, 1.0, A::offset(A::square(s.ad_value(1403)), 2.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_mul_ad_lhs(1406, A::square(s.ad_value(1403)), 1391);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_scale_ad(1407, A::mul(A::mul(s.ad_value(1403), s.ad_value(1391)), s.ad_value(1391)), 4.0);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_mul_ad_lhs(1408, A::mul(A::sub(A::scale(s.ad_value(1391), 8.0), A::scale(s.ad_value(1406), 12.0)), s.ad_value(1391)), 1391);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub(1391, 1396, 1403);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_mul(1392, 1395, 1405);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_add_ad(1409, A::scale(s.ad_value(1391), 2.0), A::mul(s.ad_value(261), A::add(A::sub(A::offset(s.ad_value(1404), (-1.0)), s.ad_value(1392)), A::mul(s.ad_value(1395), A::sub_from_scalar(1.0, s.ad_value(1407))))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub_ad(1410, A::square(s.ad_value(1391)), A::mul(s.ad_value(261), A::add(A::add(A::offset(A::sub(s.ad_value(1404), s.ad_value(1403)), (-1.0)), s.ad_value(1392)), A::mul(s.ad_value(1395), A::sub(A::offset(s.ad_value(1403), (-1.0)), s.ad_value(1406))))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub_from_scalar_ad(1391, 2.0, A::mul(s.ad_value(261), A::sub(A::add(s.ad_value(1404), s.ad_value(1392)), A::mul(s.ad_value(1395), s.ad_value(1408)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub_ad(1391, A::square(s.ad_value(1409)), A::scale(A::mul(s.ad_value(1410), s.ad_value(1391)), 2.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_sub_ad(4, A::neg(s.ad_value(1403)), A::scale(A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_div_from_scalar_ad(1411, 1.0, A::offset(A::scale(s.ad_value(260), 0.732464877560822), 1.25));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_mul_ad_lhs(1412, A::offset(A::mul(A::scale(s.ad_value(263), 1.25), s.ad_value(1411)), (-1.0)), 1411);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_mul_ad(1413, A::mul(s.ad_value(1394), s.ad_value(264)), A::offset(A::mul(s.ad_value(1412), s.ad_value(1394)), 1.0));
        }

        s.v[1618] = if ((-s.v[1413]) > (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (s.v[1618] != 0.0)) {
            s.store_exp_ad(1391, A::neg(s.ad_value(1413)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (!(s.v[1618] != 0.0))) {
            s.store_div_from_scalar_ad(1391, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(1413))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(1413))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(1413))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub_from_scalar(1414, 1.0, 1391);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub_ad(1415, A::add(s.ad_value(1394), A::scale(s.ad_value(261), 0.5)), A::mul(s.ad_value(260), A::sqrt(A::sub(A::add(s.ad_value(1394), A::scale(s.ad_value(261), 0.25)), s.ad_value(1414)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_offset(1416, 266, 3.0);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub_ad(1398, A::scale(A::sub(A::add(s.ad_value(1415), s.ad_value(1416)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1415), s.ad_value(1416)), A::sub(s.ad_value(1415), s.ad_value(1416))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(1416), A::sqrt(A::offset(A::square(s.ad_value(1416)), 5.0))), 0.5));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub(1391, 1394, 1398);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_exp_ad(1392, A::neg(s.ad_value(1398)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_div_from_scalar_ad(1393, 1.0, A::offset(A::square(s.ad_value(1398)), 2.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_mul_ad_lhs(1406, A::square(s.ad_value(1398)), 1393);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_scale_ad(1407, A::mul(A::mul(s.ad_value(1398), s.ad_value(1393)), s.ad_value(1393)), 4.0);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_mul_ad_lhs(1408, A::mul(A::sub(A::scale(s.ad_value(1393), 8.0), A::scale(s.ad_value(1406), 12.0)), s.ad_value(1393)), 1393);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_max_from_scalar_ad(1399, 1e-40, A::sub(A::square(s.ad_value(1391)), A::mul(s.ad_value(261), A::sub(A::offset(A::add(s.ad_value(1392), s.ad_value(1398)), (-1.0)), A::mul(s.ad_value(1395), A::add(A::offset(s.ad_value(1398), 1.0), s.ad_value(1406)))))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub_from_scalar_ad(1400, 1.0, A::scale(A::mul(s.ad_value(261), A::sub(s.ad_value(1392), A::mul(s.ad_value(1395), s.ad_value(1408)))), 0.5));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_add_ad(1401, A::scale(s.ad_value(1391), 2.0), A::mul(s.ad_value(261), A::sub(A::sub_from_scalar(1.0, s.ad_value(1392)), A::mul(s.ad_value(1395), A::offset(s.ad_value(1407), 1.0)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_add_ad(1402, A::sub(s.ad_value(266), s.ad_value(1398)), A::ln(A::div(s.ad_value(1399), s.ad_value(261))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_add(1389, 1399, 1401);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_add_ad(1390, A::square(s.ad_value(1389)), A::mul(s.ad_value(1402), A::sub(A::mul(A::scale(s.ad_value(1401), 0.5), s.ad_value(1401)), A::mul(s.ad_value(1399), s.ad_value(1400)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            let assign29500_ad_e32035: A = A::add(s.ad_value(1398), A::div(A::mul(A::mul(s.ad_value(1399), s.ad_value(1389)), s.ad_value(1402)), A::add(s.ad_value(1390), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402)), s.ad_value(1402)), s.ad_value(1401)), A::sub(A::scale(A::square(s.ad_value(1401)), 0.3333333333333), A::mul(s.ad_value(1399), s.ad_value(1400)))))));
            s.store_ad(1417, &assign29500_ad_e32035);
        }

        s.v[1619] = if (s.v[1417] < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (s.v[1619] != 0.0)) {
            s.store_exp(1404, 1417);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (s.v[1619] != 0.0)) {
            s.store_div_from_scalar(1405, 1.0, 1404);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (s.v[1619] != 0.0)) {
            s.store_mul(1404, 1395, 1404);
        }

        s.v[1620] = if (s.v[1417] > (s.v[266] - 80.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (!(s.v[1619] != 0.0))) && (s.v[1620] != 0.0)) {
            s.store_exp_ad(1404, A::sub(s.ad_value(1417), s.ad_value(266)));
        }

        if ((((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (!(s.v[1619] != 0.0))) && (s.v[1620] != 0.0)) {
            s.store_div(1405, 1395, 1404);
        }

        if ((((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (!(s.v[1619] != 0.0))) && (!(s.v[1620] != 0.0))) {
            s.store_div_from_scalar_ad(1404, 1.80485e-35, A::offset(A::mul(A::offset(A::sub(s.ad_value(266), s.ad_value(1417)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(266), s.ad_value(1417)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(266), s.ad_value(1417)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) && (!(s.v[1619] != 0.0))) && (!(s.v[1620] != 0.0))) {
            s.store_div_from_scalar_ad(1405, 1.80485e-35, A::offset(A::mul(A::offset(s.ad_value(1417), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(1417), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(1417), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_div_from_scalar_ad(1391, 1.0, A::offset(A::square(s.ad_value(1417)), 2.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_mul_ad_lhs(1406, A::square(s.ad_value(1417)), 1391);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_scale_ad(1407, A::mul(A::mul(s.ad_value(1417), s.ad_value(1391)), s.ad_value(1391)), 4.0);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_mul_ad_lhs(1408, A::mul(A::sub(A::scale(s.ad_value(1391), 8.0), A::scale(s.ad_value(1406), 12.0)), s.ad_value(1391)), 1391);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub(1391, 1394, 1417);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_add_ad(1409, A::scale(s.ad_value(1391), 2.0), A::mul(s.ad_value(261), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(1405)), s.ad_value(1404)), A::mul(s.ad_value(1395), A::offset(s.ad_value(1407), 1.0)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub_ad(1410, A::square(s.ad_value(1391)), A::mul(s.ad_value(261), A::sub(A::add(A::offset(A::add(s.ad_value(1405), s.ad_value(1417)), (-1.0)), s.ad_value(1404)), A::mul(s.ad_value(1395), A::add(A::offset(s.ad_value(1417), 1.0), s.ad_value(1406))))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub_from_scalar_ad(1391, 2.0, A::mul(s.ad_value(261), A::sub(A::add(s.ad_value(1405), s.ad_value(1404)), A::mul(s.ad_value(1395), s.ad_value(1408)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_sub_ad(1391, A::square(s.ad_value(1409)), A::scale(A::mul(s.ad_value(1410), s.ad_value(1391)), 2.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_add_ad_rhs(4, 1417, A::scale(A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_mul_ad_rhs(1438, 0, A::add(s.ad_value(4), s.ad_value(3)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1609] != 0.0))) {
            s.copy_ad(1438, 1429);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(0, 248, A::sub(s.ad_value(1427), s.ad_value(1438)));
        }

        s.v[1621] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_scale_ad(1439, A::add(A::add(s.ad_value(0), s.ad_value(257)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_scale_ad(1440, A::add(A::sub(s.ad_value(257), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(257)), A::sub(A::neg(s.ad_value(0)), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_mul_ad_rhs(2, 258, A::exp(A::scale(A::ln(s.ad_value(1439)), (-0.3333333333333))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_mul_ad_rhs(3, 258, A::exp(A::scale(A::ln(s.ad_value(1440)), (-0.3333333333333))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_div(1447, 245, 4);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_offset_ad(1441, A::mul(s.ad_value(246), s.ad_value(2)), 1.0);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_offset_ad(1442, A::mul(s.ad_value(247), s.ad_value(3)), 1.0);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_div_ad_lhs(1443, A::mul(s.ad_value(246), s.ad_value(4)), 1441);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_div_ad_lhs(1444, A::mul(s.ad_value(247), s.ad_value(4)), 1442);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_div_from_scalar_ad(1445, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(1443)), 1.0), A::div_from_scalar(1.0, s.ad_value(1444))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_offset_ad(1441, A::mul(s.ad_value(1443), s.ad_value(2)), 1.0);
        }

        if ((s.v[1608] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_offset_ad(1442, A::mul(s.ad_value(1444), s.ad_value(3)), 1.0);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1621] != 0.0))) {
            s.copy_ad(1447, 245);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1621] != 0.0))) {
            s.copy_ad(1443, 246);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1621] != 0.0))) {
            s.copy_ad(1444, 247);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1621] != 0.0))) {
            s.copy_ad(1445, 248);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1621] != 0.0))) {
            s.store_scalar(1441, 1.0);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1621] != 0.0))) {
            s.store_scalar(1442, 1.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(1446, 1445, A::sub(s.ad_value(1427), s.ad_value(1438)));
        }

        s.v[1622] = if (s.v[1446] > 0.0) { 1.0 } else { 0.0 };

        s.v[1623] = if ((-s.v[1446]) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1622] != 0.0)) && (s.v[1623] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(A::neg(s.ad_value(1446))), 1.0));
        }

        if (((s.v[1608] != 0.0) && (s.v[1622] != 0.0)) && (!(s.v[1623] != 0.0))) {
            s.store_neg(0, 1446);
        }

        if ((s.v[1608] != 0.0) && (s.v[1622] != 0.0)) {
            s.store_offset_ad(1448, A::add(A::sub(s.ad_value(1427), A::div(s.ad_value(1446), s.ad_value(1443))), s.ad_value(0)), (-0.6931471805599));
        }

        s.v[1624] = if (s.v[1446] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1622] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(s.ad_value(1446)), 1.0));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1622] != 0.0))) && (!(s.v[1624] != 0.0))) {
            s.copy_ad(0, 1446);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1622] != 0.0))) {
            s.store_offset_ad(1448, A::add(A::add(s.ad_value(1438), A::div(s.ad_value(1446), s.ad_value(1444))), s.ad_value(0)), (-0.6931471805599));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(1449, A::sub(A::add(s.ad_value(1448), s.ad_value(254)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1448), s.ad_value(254)), A::sub(s.ad_value(1448), s.ad_value(254))), 4.0))), 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_ad(1450, A::sqrt(A::offset(A::div(A::scale(A::sub(s.ad_value(254), s.ad_value(1449)), 2.0), s.ad_value(255)), 1.0)), (-1.0));
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
        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1451, 1449, A::mul(s.ad_value(255), s.ad_value(1450)));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(0, A::add(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1428)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1428)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1428)), 1.0), (-0.5))), 0.01))), 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1452, 1.0, A::offset(A::mul(s.ad_value(1420), s.ad_value(0)), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1453, 1.0, A::offset(A::mul(s.ad_value(1421), s.ad_value(0)), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(0, A::mul(A::mul(A::scale(s.ad_value(329), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(340), s.ad_value(329)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1450)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1428)), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1454, 1422, 0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1455, 1423, 0);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1456, A::add(A::mul(A::add(A::sub(s.ad_value(1427), s.ad_value(1451)), s.ad_value(1454)), s.ad_value(1452)), s.ad_value(1451)), 341);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1457, A::add(A::mul(A::add(A::sub(s.ad_value(1438), s.ad_value(1451)), s.ad_value(1455)), s.ad_value(1453)), s.ad_value(1451)), 341);
        }

        if (s.v[1608] != 0.0) {
            let assign30140_ad_e32954: A = A::sub(A::add(A::add(s.ad_value(1457), A::mul(s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)))), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(1457), A::mul(s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)))), s.ad_value(225)), A::sub(A::add(s.ad_value(1457), A::mul(s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)))), s.ad_value(225))), 0.01)));
            s.store_scale_ad(1458, assign30140_ad_e32954, 0.5);
        }

        if (s.v[1608] != 0.0) {
            let assign30150_ad_e32991: A = A::sub(A::add(A::add(s.ad_value(1456), A::mul(s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)))), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(1456), A::mul(s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)))), s.ad_value(225)), A::sub(A::add(s.ad_value(1456), A::mul(s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)))), s.ad_value(225))), 0.01)));
            s.store_scale_ad(1459, assign30150_ad_e32991, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_div(1460, 1443, 1452);
        }

        if (s.v[1608] != 0.0) {
            s.store_div(1461, 1444, 1453);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar(1434, 1.0, 1460);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar(1435, 1.0, 1461);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1462, 1.0, A::add(A::offset(s.ad_value(1434), 1.0), s.ad_value(1435)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad_rhs(1433, 253, A::square(s.ad_value(1447)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1430, A::offset(s.ad_value(1460), 1.0), A::offset(s.ad_value(1461), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_ln(1431, 1430);
        }

        s.v[1625] = if (s.v[1431] > 1e-8) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1625] != 0.0)) {
            s.store_div_ad(1432, A::mul(A::scale(s.ad_value(1431), 2.0), A::offset(s.ad_value(1430), 1.0)), A::offset(s.ad_value(1430), (-1.0)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1625] != 0.0))) {
            s.store_scaled_offset(1432, 1431, 2.0, 2.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(1463, 1462, A::sub(s.ad_value(1458), s.ad_value(1459)));
        }

        if (s.v[1608] != 0.0) {
            s.store_square(1464, 1463);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1436, 1458, A::mul(s.ad_value(1463), s.ad_value(1434)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1437, 1459, A::mul(s.ad_value(1463), s.ad_value(1435)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1342, 1.0, A::offset(s.ad_value(1460), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1343, 1.0, A::offset(s.ad_value(1461), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_ad(1345, A::ln(A::div(A::mul(A::add(s.ad_value(1460), A::mul(s.ad_value(1461), s.ad_value(1343))), s.ad_value(1432)), s.ad_value(1433))), 3.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_ad(1346, A::ln(A::div(A::mul(A::add(s.ad_value(1461), A::mul(s.ad_value(1460), s.ad_value(1342))), s.ad_value(1432)), s.ad_value(1433))), 3.0);
        }

        s.v[1626] = if (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1626] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1345), s.ad_value(1436)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1626] != 0.0))) {
            s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1349, 1345, A::scale(s.ad_value(1344), 3.0));
        }

        s.v[1627] = if (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1346), s.ad_value(1437)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1627] != 0.0))) {
            s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1350, 1346, A::scale(s.ad_value(1344), 3.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1347, A::add(A::mul(s.ad_value(1460), s.ad_value(1458)), s.ad_value(1350)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1348, A::add(A::mul(s.ad_value(1461), s.ad_value(1459)), s.ad_value(1349)), 1343);
        }

        s.v[1628] = if (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1628] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1345), s.ad_value(1347)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1628] != 0.0))) {
            s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1349, 1345, A::scale(s.ad_value(1344), 3.0));
        }

        s.v[1629] = if (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1629] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1346), s.ad_value(1348)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1629] != 0.0))) {
            s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1350, 1346, A::scale(s.ad_value(1344), 3.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1465, 1458, 1349);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1469, 1459, 1350);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1356, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1359, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1465);
        }

        s.v[1630] = if ((s.v[1458] - s.v[1465]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1630] != 0.0)) {
            s.store_exp_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1630] != 0.0))) {
            s.store_scale_ad(1342, A::offset(A::mul(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1352, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1631] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((s.v[1608] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1632] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (s.v[1632] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1631] != 0.0))) && (!(s.v[1632] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1633] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1633] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1633] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((s.v[1608] != 0.0) && (s.v[1633] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1634] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1633] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1633] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1633] != 0.0))) && (s.v[1634] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1633] != 0.0))) && (!(s.v[1634] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1633] != 0.0))) && (!(s.v[1634] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1635] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1635] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((s.v[1608] != 0.0) && (s.v[1635] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((s.v[1608] != 0.0) && (s.v[1635] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1635] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1635] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1635] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1635] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1635] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1636] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1636] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1636] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1636] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1636] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1636] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1636] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1465);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1636] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1636] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1465)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1376, 1461, 1373);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1465, 1465, 1380);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1465);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1381, 1461, 1469);
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1374, 1351, 1381);
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1383, A::offset(A::scale(s.ad_value(1374), 8.5797362674), 39.478417604), A::mul(s.ad_value(1351), s.ad_value(1381)));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(1384, A::add(A::scale(s.ad_value(1374), 2.0), A::mul(s.ad_value(1351), s.ad_value(1381))), 39.478417604);
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
        if (s.v[1608] != 0.0) {
            s.store_sqrt_ad(1385, A::sub(A::square(s.ad_value(1383)), A::mul(A::scale(s.ad_value(1382), 4.0), s.ad_value(1384))));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1353, A::sub(s.ad_value(1385), s.ad_value(1383)), A::scale(s.ad_value(1382), 2.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

        s.v[1637] = if (s.v[1386] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1637] != 0.0)) {
            s.store_mul_ad_rhs(1377, 1386, A::add(A::sub(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), s.ad_value(1458)), s.ad_value(1465)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1637] != 0.0)) {
            s.store_add_ad_lhs(1378, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1386);
        }

        if ((s.v[1608] != 0.0) && (s.v[1637] != 0.0)) {
            s.store_sub_ad_lhs(1387, A::sub(s.ad_value(1458), s.ad_value(1465)), 1345);
        }

        s.v[1638] = if ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
            s.store_sub_ad_rhs(1465, 1465, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1465);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1381, 1461, 1469);
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1374, 1351, 1381);
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1383, A::offset(A::scale(s.ad_value(1374), 8.5797362674), 39.478417604), A::mul(s.ad_value(1351), s.ad_value(1381)));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(1384, A::add(A::scale(s.ad_value(1374), 2.0), A::mul(s.ad_value(1351), s.ad_value(1381))), 39.478417604);
        }

        if (s.v[1608] != 0.0) {
            s.store_sqrt_ad(1385, A::sub(A::square(s.ad_value(1383)), A::mul(A::scale(s.ad_value(1382), 4.0), s.ad_value(1384))));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1353, A::sub(s.ad_value(1385), s.ad_value(1383)), A::scale(s.ad_value(1382), 2.0));
        }

        s.v[1639] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1639] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1639] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1639] != 0.0)) {
            s.store_div_ad_lhs(1358, A::scale(A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 0.25), 1353);
        }

        s.v[1640] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1639] != 0.0))) && (s.v[1640] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1639] != 0.0))) && (s.v[1640] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1639] != 0.0))) && (s.v[1640] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1639] != 0.0))) && (s.v[1640] != 0.0)) {
            s.store_div_ad_lhs(1358, A::scale(A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1639] != 0.0))) && (!(s.v[1640] != 0.0))) {
            s.store_offset_ad(1357, A::mul(A::scale(s.ad_value(1353), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0238095238095))))), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1639] != 0.0))) && (!(s.v[1640] != 0.0))) {
            s.store_scale_ad(1358, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1353, 1353, A::div(A::add(A::add(A::mul(s.ad_value(1374), s.ad_value(1357)), A::mul(s.ad_value(1351), s.ad_value(1381))), s.ad_value(1353)), A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0)));
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

        s.v[1641] = if (s.v[1386] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1641] != 0.0)) {
            s.store_mul_ad_rhs(1377, 1386, A::add(A::sub(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), s.ad_value(1458)), s.ad_value(1465)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1641] != 0.0)) {
            s.store_add_ad_lhs(1378, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1386);
        }

        if ((s.v[1608] != 0.0) && (s.v[1641] != 0.0)) {
            s.store_sub_ad_lhs(1387, A::sub(s.ad_value(1458), s.ad_value(1465)), 1345);
        }

        s.v[1642] = if ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1641] != 0.0)) && (s.v[1642] != 0.0)) {
            s.store_sub_ad_rhs(1465, 1465, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1465);
        }

        s.v[1643] = if ((s.v[1458] - s.v[1465]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1643] != 0.0)) {
            s.store_exp_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1643] != 0.0))) {
            s.store_scale_ad(1342, A::offset(A::mul(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1352, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1644] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((s.v[1608] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1645] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (s.v[1645] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1644] != 0.0))) && (!(s.v[1645] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1646] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1646] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1646] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((s.v[1608] != 0.0) && (s.v[1646] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1647] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1646] != 0.0))) && (s.v[1647] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1646] != 0.0))) && (s.v[1647] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1646] != 0.0))) && (s.v[1647] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1646] != 0.0))) && (!(s.v[1647] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1646] != 0.0))) && (!(s.v[1647] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1648] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1648] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((s.v[1608] != 0.0) && (s.v[1648] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((s.v[1608] != 0.0) && (s.v[1648] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1648] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1648] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1648] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1648] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1648] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1649] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1649] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1649] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1649] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1649] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1649] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1649] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1465);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1649] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1649] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1465)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1376, 1461, 1373);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1465, 1465, 1380);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1465);
        }

        s.v[1650] = if ((s.v[1458] - s.v[1465]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1650] != 0.0)) {
            s.store_exp_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1650] != 0.0))) {
            s.store_scale_ad(1342, A::offset(A::mul(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1352, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1651] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((s.v[1608] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1652] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
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
        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (s.v[1652] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1651] != 0.0))) && (!(s.v[1652] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1653] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1653] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1653] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((s.v[1608] != 0.0) && (s.v[1653] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1654] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1653] != 0.0))) && (s.v[1654] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1653] != 0.0))) && (s.v[1654] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1653] != 0.0))) && (s.v[1654] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1653] != 0.0))) && (!(s.v[1654] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1653] != 0.0))) && (!(s.v[1654] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1655] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1655] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((s.v[1608] != 0.0) && (s.v[1655] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((s.v[1608] != 0.0) && (s.v[1655] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1655] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1655] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1655] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1655] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1655] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1656] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1656] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1656] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1656] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1656] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1656] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1656] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1465);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1656] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1656] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1465)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1376, 1461, 1373);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1465, 1465, 1380);
        }

        s.v[1657] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        s.v[1658] = if (((s.v[1380]) as f64).abs() > 0.01) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_mul(1351, 1460, 1465);
        }

        s.v[1659] = if ((s.v[1458] - s.v[1465]) < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1659] != 0.0)) {
            s.store_exp_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1659] != 0.0))) {
            s.store_scale_ad(1342, A::offset(A::mul(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_mul(1352, 1433, 1342);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1660] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1661] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (s.v[1661] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1660] != 0.0))) && (!(s.v[1661] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1662] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1662] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1662] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1662] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1663] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1662] != 0.0))) && (s.v[1663] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1662] != 0.0))) && (s.v[1663] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1662] != 0.0))) && (s.v[1663] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1662] != 0.0))) && (!(s.v[1663] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1662] != 0.0))) && (!(s.v[1663] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1664] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1664] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1664] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1664] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1664] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1664] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1664] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1664] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1664] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1665] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1665] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1665] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1665] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (s.v[1665] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1665] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1665] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1465);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1665] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) && (!(s.v[1665] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1465)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_mul(1376, 1461, 1373);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (((s.v[1608] != 0.0) && (s.v[1657] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_add(1465, 1465, 1380);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1467, 1460, 1465);
        }

        s.v[1666] = if ((s.v[1458] - s.v[1465]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1666] != 0.0)) {
            s.store_exp_ad(1342, A::sub(s.ad_value(1458), s.ad_value(1465)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1666] != 0.0))) {
            s.store_scale_ad(1342, A::offset(A::mul(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1471, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1470, A::square(s.ad_value(1467)), 1471);
        }

        s.v[1667] = if (s.v[1471] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1667] != 0.0)) {
            s.store_scalar(1466, 1e-80);
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
        if ((s.v[1608] != 0.0) && (s.v[1667] != 0.0)) {
            s.store_sub(1468, 1466, 1467);
        }

        if ((s.v[1608] != 0.0) && (s.v[1667] != 0.0)) {
            s.store_div(1469, 1468, 1461);
        }

        s.v[1668] = if (s.v[1470] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1668] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1470)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1668] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        s.v[1669] = if (s.v[1470] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1668] != 0.0))) && (s.v[1669] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1470)));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1668] != 0.0))) && (s.v[1669] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1668] != 0.0))) && (s.v[1669] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1668] != 0.0))) && (!(s.v[1669] != 0.0))) {
            s.store_offset_ad(1357, A::mul(A::scale(s.ad_value(1470), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1470), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1470), 0.0238095238095))))), 2.0);
        }

        s.v[1670] = if (((1.01 * s.v[1467]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) {
            s.store_add(1342, 1467, 1357);
        }

        s.v[1671] = if ((s.v[1471] * s.v[1467]) < (((0.9 * s.v[1467]) * s.v[1467]) * s.v[1342])) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (s.v[1671] != 0.0)) {
            s.store_offset_ad(1466, A::div(s.ad_value(1471), s.ad_value(1342)), 1e-80);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (s.v[1671] != 0.0)) {
            s.store_sub(1468, 1466, 1467);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (s.v[1671] != 0.0)) {
            s.store_div(1469, 1468, 1461);
        }

        s.v[1672] = if (s.v[1470] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) && (s.v[1672] != 0.0)) {
            s.store_sub_ad_lhs(1343, A::ln(A::div(A::scale(s.ad_value(1470), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))))), 1356);
        }

        s.v[1673] = if (s.v[1470] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) && (!(s.v[1672] != 0.0))) && (s.v[1673] != 0.0)) {
            s.store_sin_ad(1344, A::scale(s.ad_value(1356), 0.5));
        }

        if ((((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) && (!(s.v[1672] != 0.0))) && (s.v[1673] != 0.0)) {
            s.store_ln_ad(1343, A::div(A::neg(s.ad_value(1470)), A::square(s.ad_value(1344))));
        }

        if ((((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) && (!(s.v[1672] != 0.0))) && (!(s.v[1673] != 0.0))) {
            s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1470), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1470), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1470), 0.0396825396825397)))))));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) {
            s.store_sub_ad_lhs(1469, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1465)), A::scale(A::ln(s.ad_value(1342)), 2.0)), 1343);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) {
            s.store_mul(1468, 1461, 1469);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) {
            s.store_add(1466, 1467, 1468);
        }

        s.v[1674] = if (s.v[1470] > 0.005) { 1.0 } else { 0.0 };

        s.v[1675] = if (((s.v[1465] - s.v[1458]) - s.v[1356]) < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) && (s.v[1674] != 0.0)) && (s.v[1675] != 0.0)) {
            s.store_exp_ad(1344, A::sub(A::sub(s.ad_value(1465), s.ad_value(1458)), s.ad_value(1356)));
        }

        if (((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) && (s.v[1674] != 0.0)) && (!(s.v[1675] != 0.0))) {
            let assign34370_ad_e38451: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1465), s.ad_value(1458)), s.ad_value(1356)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1465), s.ad_value(1458)), s.ad_value(1356)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1465), s.ad_value(1458)), s.ad_value(1356)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1344, assign34370_ad_e38451, 5.54062e34);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) && (s.v[1674] != 0.0)) {
            s.store_div(1343, 1344, 1433);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) && (s.v[1674] != 0.0)) {
            s.store_div_ad(1342, A::mul(A::scale(s.ad_value(1470), 4.0), s.ad_value(1343)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        s.v[1676] = if (s.v[1470] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) && (!(s.v[1674] != 0.0))) && (s.v[1676] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) && (!(s.v[1674] != 0.0))) && (s.v[1676] != 0.0)) {
            s.store_div_ad_lhs(1342, A::div(A::neg(s.ad_value(1470)), A::square(s.ad_value(1343))), 1471);
        }

        if (((((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) && (!(s.v[1674] != 0.0))) && (!(s.v[1676] != 0.0))) {
            s.store_div_ad_lhs(1342, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1470), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1470), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1470), 0.0396825396825397)))))), 1471);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) {
            s.store_offset_ad(1466, A::div(A::sub(s.ad_value(1467), s.ad_value(1357)), A::sub_from_scalar(1.0, s.ad_value(1342))), 1e-80);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) {
            s.store_sub(1468, 1466, 1467);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1667] != 0.0))) && (!(s.v[1670] != 0.0))) {
            s.store_div(1469, 1468, 1461);
        }

        s.v[1677] = if ((s.v[1459] - s.v[1469]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1677] != 0.0)) {
            s.store_exp_ad(1342, A::sub(s.ad_value(1459), s.ad_value(1469)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1677] != 0.0))) {
            s.store_scale_ad(1342, A::offset(A::mul(A::offset(A::sub(s.ad_value(1459), s.ad_value(1469)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1459), s.ad_value(1469)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1459), s.ad_value(1469)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1472, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1475, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1476, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1473, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1474, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1477, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1478, 0.0);
        }

        s.v[1678] = if (s.v[1466] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) {
            s.store_mul(1473, 1471, 1434);
        }

        if ((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) {
            s.store_mul(1474, 1472, 1435);
        }

        if ((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) {
            s.store_add_ad_rhs(1475, 1473, A::scale(s.ad_value(1467), 2.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) {
            s.store_add_ad_rhs(1476, 1474, A::scale(s.ad_value(1468), 2.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) {
            s.store_add_ad_lhs(1477, A::add(A::scale(s.ad_value(1466), 2.0), s.ad_value(1473)), 1474);
        }

        s.v[1679] = if (((s.v[1470]) as f64).abs() > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) && (s.v[1679] != 0.0)) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(1475), s.ad_value(1476)), A::mul(A::scale(A::offset(s.ad_value(1465), 2.0), 2.0), s.ad_value(1476))), A::mul(A::scale(A::offset(s.ad_value(1469), 2.0), 2.0), s.ad_value(1475)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) && (s.v[1679] != 0.0)) {
            s.store_div_ad(1478, A::mul(A::scale(s.ad_value(1470), (-4.0)), s.ad_value(1477)), A::mul(s.ad_value(1466), s.ad_value(2)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) && (!(s.v[1679] != 0.0))) {
            s.store_scale_ad(2, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1470), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1470), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1470), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) && (!(s.v[1679] != 0.0))) {
            s.store_add_ad(3, A::add(A::mul(s.ad_value(1475), s.ad_value(1471)), A::mul(s.ad_value(1476), s.ad_value(1472))), A::mul(A::mul(A::mul(s.ad_value(1475), s.ad_value(1476)), s.ad_value(1466)), A::offset(A::mul(s.ad_value(1466), s.ad_value(2)), 1.0)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1678] != 0.0)) && (!(s.v[1679] != 0.0))) {
            s.store_div_ad(1478, A::mul(A::mul(s.ad_value(1471), s.ad_value(1472)), s.ad_value(1477)), A::mul(s.ad_value(1466), s.ad_value(3)));
        }

        if (s.v[1608] != 0.0) {
            s.store_ln(1479, 1466);
        }

        s.v[1680] = if ((s.v[1467] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::scale(s.ad_value(1467), 0.5)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_scale(2, 1467, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scale(1480, 2, 2.0);
        }

        s.v[1681] = if ((s.v[1468] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1681] != 0.0)) {
            s.store_ln_ad(3, A::offset(A::exp(A::scale(s.ad_value(1468), 0.5)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1681] != 0.0))) {
            s.store_scale(3, 1468, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scale(1481, 3, 2.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1482, 1481, 1468);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1483, 1480, 1467);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1484, A::mul(s.ad_value(270), s.ad_value(1480)), A::mul(s.ad_value(271), s.ad_value(1482)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1485, A::mul(s.ad_value(270), s.ad_value(1481)), A::mul(s.ad_value(271), s.ad_value(1483)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad_rhs(0, 1466, A::add(s.ad_value(1480), s.ad_value(1481)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1486, 1480, 0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1487, 1481, 0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(1488, A::mul(s.ad_value(1480), s.ad_value(191)), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(1489, A::mul(s.ad_value(1481), s.ad_value(192)), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(2, 50, A::add(s.ad_value(1482), A::mul(s.ad_value(51), s.ad_value(1483))));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(3, A::add(A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(4, A::add(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_div(1490, 3, 4);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(1491, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(1482)), 1.0), A::mul(s.ad_value(42), s.ad_value(1483)))), A::exp(A::mul(A::neg(s.ad_value(44)), A::ln(A::add(A::offset(A::mul(s.ad_value(1486), s.ad_value(268)), 1.0), A::mul(s.ad_value(1487), s.ad_value(269)))))));
        }

        s.v[1682] = if (s.v[56] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_scalar(4, 1.0);
        }

        s.v[1683] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1682] != 0.0))) && (s.v[1683] != 0.0)) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12)))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1682] != 0.0))) && (s.v[1683] != 0.0)) {
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1682] != 0.0))) && (!(s.v[1683] != 0.0))) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12)))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1682] != 0.0))) && (!(s.v[1683] != 0.0))) {
            s.store_div_from_scalar_ad(4, 1.0, A::offset(s.ad_value(2), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(1492, A::scale(A::mul(s.ad_value(272), s.ad_value(1447)), 0.5), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428))), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428)))), 0.01))));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(1493, 1492, A::add(A::mul(s.ad_value(1466), s.ad_value(4)), s.ad_value(54)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1494, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1484)), 1e-6)))), 1.0), s.ad_value(1491)), A::mul(s.ad_value(38), s.ad_value(1493)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1495, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1485)), 1e-6)))), 1.0), s.ad_value(1491)), A::mul(s.ad_value(39), s.ad_value(1493)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1496, A::mul(s.ad_value(1490), A::add(s.ad_value(1488), s.ad_value(1489))), A::add(A::div(s.ad_value(1488), s.ad_value(1494)), A::div(s.ad_value(1489), s.ad_value(1495))));
        }

        s.v[1684] = if (((s.v[1463]) as f64).abs() > 0.007) { 1.0 } else { 0.0 };

        s.v[1685] = if (s.v[1463] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_exp_ad(0, A::neg(s.ad_value(1463)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_div_ad_rhs(1497, 1463, A::sub_from_scalar(1.0, s.ad_value(0)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_mul(1498, 0, 1497);
        }

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_add_ad_lhs(1499, A::offset(A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1497)))), (-0.6931471805599)), 1436);
        }

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_exp(0, 1463);
        }

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_div_ad_rhs(1498, 1463, A::offset(s.ad_value(0), (-1.0)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_mul(1497, 0, 1498);
        }

        if (((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_add_ad_lhs(1499, A::offset(A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1498)))), (-0.6931471805599)), 1437);
        }

        if ((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) {
            s.store_div_ad(1500, A::neg(s.ad_value(1463)), A::mul(s.ad_value(1462), A::sub(A::sub_from_scalar(1.0, s.ad_value(1497)), A::mul(s.ad_value(1463), s.ad_value(1435)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) {
            s.store_div_ad_rhs(1501, 1463, A::mul(s.ad_value(1462), A::add(A::sub_from_scalar(1.0, s.ad_value(1498)), A::mul(s.ad_value(1463), s.ad_value(1434)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1684] != 0.0)) {
            s.store_div_ad_rhs(1502, 1463, A::sub(A::div(A::offset(A::mul(s.ad_value(1498), s.ad_value(1435)), 0.5), s.ad_value(1501)), A::div(A::offset(A::mul(s.ad_value(1497), s.ad_value(1434)), 0.5), s.ad_value(1500))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_scale(0, 1464, (0.5 * 0.1666666666667));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_scale(2, 1463, 0.5);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_add_ad_lhs(1497, A::offset(s.ad_value(2), 1.0), 0);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_add_ad_lhs(1498, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_scale(3, 2, 0.1666666666667);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_div_from_scalar_ad(1500, 1.0, A::mul(s.ad_value(1462), A::add(A::offset(s.ad_value(1435), 0.5), s.ad_value(3))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_div_from_scalar_ad(1501, 1.0, A::mul(s.ad_value(1462), A::sub(A::offset(s.ad_value(1434), 0.5), s.ad_value(3))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            s.store_add_ad(1499, A::offset(A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), A::sub_from_scalar(1.0, A::scale(s.ad_value(0), 0.5))))), (-0.6931471805599)), A::scale(A::add(s.ad_value(1436), s.ad_value(1437)), 0.5));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1684] != 0.0))) {
            let assign35250_ad_e39592: A = A::add(A::add(A::add(A::sub_from_scalar(4.0, A::scale(s.ad_value(1462), 3.0)), A::div(A::scale(s.ad_value(1462), 12.0), A::mul(s.ad_value(1460), s.ad_value(1461)))), A::mul(A::mul(s.ad_value(1462), A::sub(s.ad_value(1434), s.ad_value(1435))), s.ad_value(1463))), A::mul(A::scale(A::sub_from_scalar(0.2, A::scale(s.ad_value(1462), 0.25)), 0.3333333333333), s.ad_value(1464)));
            s.store_div_from_scalar_ad(1502, (-12.0), assign35250_ad_e39592);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar(1503, 1.0, 1502);
        }

        s.v[1686] = if (s.v[1466] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_div_ad(1504, A::scale(s.ad_value(1480), 100.0), A::offset(s.ad_value(1480), 100.0));
        }

        s.v[1687] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1687] != 0.0)) {
            s.store_div_from_scalar_ad(1505, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(61), s.ad_value(1504))));
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (!(s.v[1687] != 0.0))) {
            s.store_offset_ad(1505, A::mul(s.ad_value(61), s.ad_value(1504)), 1.0);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_div_ad(1506, A::scale(s.ad_value(1481), 100.0), A::offset(s.ad_value(1481), 100.0));
        }

        s.v[1688] = if (s.v[62] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1688] != 0.0)) {
            s.store_div_from_scalar_ad(1507, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(62), s.ad_value(1506))));
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (!(s.v[1688] != 0.0))) {
            s.store_offset_ad(1507, A::mul(s.ad_value(62), s.ad_value(1506)), 1.0);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_sub_ad(1508, A::div(A::mul(s.ad_value(1478), s.ad_value(1477)), A::mul(s.ad_value(1475), s.ad_value(1476))), A::div(A::add(A::div(s.ad_value(1471), s.ad_value(1475)), A::div(s.ad_value(1472), s.ad_value(1476))), s.ad_value(1466)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_div_ad(1509, A::mul(s.ad_value(1508), s.ad_value(1466)), A::offset(s.ad_value(1508), 1.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_sub(2, 1502, 1509);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_div_ad_lhs(1510, A::add(s.ad_value(1466), A::mul(s.ad_value(1502), s.ad_value(1499))), 2);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_scale_ad(1510, A::add(s.ad_value(1510), A::sqrt(A::offset(A::square(s.ad_value(1510)), 1e-6))), 0.5);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_mul_ad(1511, A::scale(A::div(s.ad_value(1424), s.ad_value(1496)), 0.5), A::add(s.ad_value(1505), s.ad_value(1507)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_sub_from_scalar_ad(1512, 1.0, A::div(s.ad_value(1466), s.ad_value(1509)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_offset(1513, 1499, 1.0);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_mul_ad_lhs(1514, A::sub(A::offset(A::mul(A::sub(A::scale(s.ad_value(1509), 2.0), s.ad_value(1466)), s.ad_value(1503)), (-2.0)), s.ad_value(1499)), 1510);
        }

        s.v[1689] = if (s.v[1511] > 1e-14) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_div_from_scalar_ad(1515, 2.0, A::square(s.ad_value(1511)));
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
        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_mul(1516, 1515, 1512);
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_add(1517, 1515, 1514);
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_mul(1518, 1515, 1513);
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_sqrt_ad(1519, A::offset(A::add(A::square(s.ad_value(1516)), A::mul(A::mul(A::scale(s.ad_value(1515), 0.148148148148), s.ad_value(1515)), s.ad_value(1515))), 1e-20));
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_sqrt_ad(1520, A::offset(A::add(A::square(s.ad_value(1518)), A::mul(A::mul(A::scale(s.ad_value(1517), 0.148148148148), s.ad_value(1517)), s.ad_value(1517))), 1e-20));
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_sub_ad(1521, A::exp(A::scale(A::ln(A::scale(A::add(s.ad_value(1519), s.ad_value(1516)), 0.5)), 0.3333333333333)), A::exp(A::scale(A::ln(A::scale(A::sub(s.ad_value(1519), s.ad_value(1516)), 0.5)), 0.3333333333333)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (s.v[1689] != 0.0)) {
            s.store_sub_ad(1522, A::exp(A::scale(A::ln(A::scale(A::add(s.ad_value(1520), s.ad_value(1518)), 0.5)), 0.3333333333333)), A::exp(A::scale(A::ln(A::scale(A::sub(s.ad_value(1520), s.ad_value(1518)), 0.5)), 0.3333333333333)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (!(s.v[1689] != 0.0))) {
            s.copy_ad(1521, 1512);
        }

        if (((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) && (!(s.v[1689] != 0.0))) {
            s.copy_ad(1522, 1513);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_square(4, 2);
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_scale_ad(1523, A::add(A::add(s.ad_value(1521), s.ad_value(1522)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(1521), s.ad_value(1522)), A::sub(s.ad_value(1521), s.ad_value(1522))), A::scale(s.ad_value(4), 10.0)))), (0.94 * 0.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_add_ad_rhs(1524, 1466, A::mul(s.ad_value(1509), s.ad_value(1523)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_mul_ad_rhs(1525, 1502, A::sub(s.ad_value(1523), s.ad_value(1499)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_scale_ad(1526, A::add(A::add(s.ad_value(1524), s.ad_value(1525)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(1524), s.ad_value(1525)), A::sub(s.ad_value(1524), s.ad_value(1525))), A::scale(s.ad_value(4), 36.0)))), 0.5);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.copy_ad(1509, 1502);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scaled_offset(1523, 1499, 1.0, 0.94);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_add_ad(1526, A::scale(s.ad_value(1466), 0.5), A::mul(s.ad_value(1502), A::sub(s.ad_value(1523), A::scale(s.ad_value(1499), 0.5))));
        }

        s.v[1690] = if ((s.v[1526] - 0.5) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1690] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::offset(s.ad_value(1526), (-0.5))), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1690] != 0.0))) {
            s.store_offset(2, 1526, (-0.5));
        }

        if (s.v[1608] != 0.0) {
            s.store_offset(3, 2, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(4, 1523, A::ln(A::div(s.ad_value(1466), s.ad_value(3))));
        }

        s.v[1691] = if ((s.v[4] - 6.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1691] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::offset(s.ad_value(4), (-6.0))), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1691] != 0.0))) {
            s.store_offset(2, 4, (-6.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_offset(4, 2, 6.0);
        }

        s.v[1692] = if ((s.v[225] - s.v[4]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1692] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::sub(s.ad_value(225), s.ad_value(4))), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1692] != 0.0))) {
            s.store_sub(2, 225, 4);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1527, 225, 2);
        }

        if (s.v[1608] != 0.0) {
            s.store_div(2, 339, 1527);
        }

        if (s.v[1608] != 0.0) {
            s.store_square(3, 2);
        }

        if (s.v[1608] != 0.0) {
            s.store_square(4, 3);
        }

        if (s.v[1608] != 0.0) {
            s.store_square(5, 4);
        }

        if (s.v[1608] != 0.0) {
            s.store_exp_ad(0, A::scale(A::ln(A::offset(A::mul(s.ad_value(1425), s.ad_value(4)), 1.0)), 2.666666666667));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(1528, 339, A::exp(A::scale(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625))));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1342, 1.0, A::offset(s.ad_value(1460), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1343, 1.0, A::offset(s.ad_value(1461), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_ad(1345, A::add(A::ln(A::div(A::mul(A::add(s.ad_value(1460), A::mul(s.ad_value(1461), s.ad_value(1343))), s.ad_value(1432)), s.ad_value(1433))), s.ad_value(1528)), 3.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_ad(1346, A::add(A::ln(A::div(A::mul(A::add(s.ad_value(1461), A::mul(s.ad_value(1460), s.ad_value(1342))), s.ad_value(1432)), s.ad_value(1433))), s.ad_value(1528)), 3.0);
        }

        s.v[1693] = if (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1693] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1345), s.ad_value(1436)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1693] != 0.0))) {
            s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1349, 1345, A::scale(s.ad_value(1344), 3.0));
        }

        s.v[1694] = if (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1346), s.ad_value(1437)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1694] != 0.0))) {
            s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1350, 1346, A::scale(s.ad_value(1344), 3.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1347, A::add(A::mul(s.ad_value(1460), s.ad_value(1458)), s.ad_value(1350)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1348, A::add(A::mul(s.ad_value(1461), s.ad_value(1459)), s.ad_value(1349)), 1343);
        }

        s.v[1695] = if (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1695] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1345), s.ad_value(1347)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1695] != 0.0))) {
            s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1349, 1345, A::scale(s.ad_value(1344), 3.0));
        }

        s.v[1696] = if (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1696] != 0.0)) {
            s.store_ln_ad(1344, A::offset(A::exp(A::scale(A::sub(s.ad_value(1346), s.ad_value(1348)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1696] != 0.0))) {
            s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1350, 1346, A::scale(s.ad_value(1344), 3.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1529, 1458, 1349);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1530, 1459, 1350);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1356, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1359, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1529);
        }

        s.v[1697] = if (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1697] != 0.0)) {
            s.store_exp_ad(1342, A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1697] != 0.0))) {
            let assign36120_ad_e40543: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1342, assign36120_ad_e40543, 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1352, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1698] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((s.v[1608] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1699] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (s.v[1699] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1698] != 0.0))) && (!(s.v[1699] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1700] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1700] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1700] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((s.v[1608] != 0.0) && (s.v[1700] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1701] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1700] != 0.0))) && (s.v[1701] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1700] != 0.0))) && (s.v[1701] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1700] != 0.0))) && (s.v[1701] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1700] != 0.0))) && (!(s.v[1701] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1700] != 0.0))) && (!(s.v[1701] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1702] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1702] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((s.v[1608] != 0.0) && (s.v[1702] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((s.v[1608] != 0.0) && (s.v[1702] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1702] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1702] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1702] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1702] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1702] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1703] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1703] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1703] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1703] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1703] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1703] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1703] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1529);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1703] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1703] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1529)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1376, 1461, 1373);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

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
        if (s.v[1608] != 0.0) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1529, 1529, 1380);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1529);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1381, 1461, 1530);
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1374, 1351, 1381);
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1383, A::offset(A::scale(s.ad_value(1374), 8.5797362674), 39.478417604), A::mul(s.ad_value(1351), s.ad_value(1381)));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(1384, A::add(A::scale(s.ad_value(1374), 2.0), A::mul(s.ad_value(1351), s.ad_value(1381))), 39.478417604);
        }

        if (s.v[1608] != 0.0) {
            s.store_sqrt_ad(1385, A::sub(A::square(s.ad_value(1383)), A::mul(A::scale(s.ad_value(1382), 4.0), s.ad_value(1384))));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1353, A::sub(s.ad_value(1385), s.ad_value(1383)), A::scale(s.ad_value(1382), 2.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

        s.v[1704] = if (s.v[1386] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1704] != 0.0)) {
            s.store_mul_ad_rhs(1377, 1386, A::add(A::sub(A::add(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), s.ad_value(1528)), s.ad_value(1458)), s.ad_value(1529)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1704] != 0.0)) {
            s.store_add_ad_lhs(1378, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1386);
        }

        if ((s.v[1608] != 0.0) && (s.v[1704] != 0.0)) {
            s.store_sub_ad_lhs(1387, A::sub(s.ad_value(1458), s.ad_value(1529)), 1345);
        }

        s.v[1705] = if ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1704] != 0.0)) && (s.v[1705] != 0.0)) {
            s.store_sub_ad_rhs(1529, 1529, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1529);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1381, 1461, 1530);
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1374, 1351, 1381);
        }

        if (s.v[1608] != 0.0) {
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1383, A::offset(A::scale(s.ad_value(1374), 8.5797362674), 39.478417604), A::mul(s.ad_value(1351), s.ad_value(1381)));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(1384, A::add(A::scale(s.ad_value(1374), 2.0), A::mul(s.ad_value(1351), s.ad_value(1381))), 39.478417604);
        }

        if (s.v[1608] != 0.0) {
            s.store_sqrt_ad(1385, A::sub(A::square(s.ad_value(1383)), A::mul(A::scale(s.ad_value(1382), 4.0), s.ad_value(1384))));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1353, A::sub(s.ad_value(1385), s.ad_value(1383)), A::scale(s.ad_value(1382), 2.0));
        }

        s.v[1706] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1706] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1706] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1706] != 0.0)) {
            s.store_div_ad_lhs(1358, A::scale(A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 0.25), 1353);
        }

        s.v[1707] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1706] != 0.0))) && (s.v[1707] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1706] != 0.0))) && (s.v[1707] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1706] != 0.0))) && (s.v[1707] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1706] != 0.0))) && (s.v[1707] != 0.0)) {
            s.store_div_ad_lhs(1358, A::scale(A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1706] != 0.0))) && (!(s.v[1707] != 0.0))) {
            s.store_offset_ad(1357, A::mul(A::scale(s.ad_value(1353), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0238095238095))))), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1706] != 0.0))) && (!(s.v[1707] != 0.0))) {
            s.store_scale_ad(1358, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_rhs(1353, 1353, A::div(A::add(A::add(A::mul(s.ad_value(1374), s.ad_value(1357)), A::mul(s.ad_value(1351), s.ad_value(1381))), s.ad_value(1353)), A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0)));
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

        s.v[1708] = if (s.v[1386] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1708] != 0.0)) {
            s.store_mul_ad_rhs(1377, 1386, A::add(A::sub(A::add(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), s.ad_value(1528)), s.ad_value(1458)), s.ad_value(1529)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1708] != 0.0)) {
            s.store_add_ad_lhs(1378, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1386);
        }

        if ((s.v[1608] != 0.0) && (s.v[1708] != 0.0)) {
            s.store_sub_ad_lhs(1387, A::sub(s.ad_value(1458), s.ad_value(1529)), 1345);
        }

        s.v[1709] = if ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1708] != 0.0)) && (s.v[1709] != 0.0)) {
            s.store_sub_ad_rhs(1529, 1529, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1529);
        }

        s.v[1710] = if (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1710] != 0.0)) {
            s.store_exp_ad(1342, A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1710] != 0.0))) {
            let assign37290_ad_e41976: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1342, assign37290_ad_e41976, 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1352, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1711] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((s.v[1608] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1712] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (s.v[1712] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1711] != 0.0))) && (!(s.v[1712] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1713] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((s.v[1608] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1714] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1713] != 0.0))) && (s.v[1714] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1713] != 0.0))) && (s.v[1714] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1713] != 0.0))) && (s.v[1714] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1713] != 0.0))) && (!(s.v[1714] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1713] != 0.0))) && (!(s.v[1714] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1715] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((s.v[1608] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((s.v[1608] != 0.0) && (s.v[1715] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1715] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1715] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1715] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1715] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1715] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1716] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1716] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1716] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1716] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1716] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1716] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1716] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1529);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1716] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1716] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1529)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1376, 1461, 1373);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1529, 1529, 1380);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1351, 1460, 1529);
        }

        s.v[1717] = if (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1717] != 0.0)) {
            s.store_exp_ad(1342, A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1717] != 0.0))) {
            let assign38040_ad_e42925: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1342, assign38040_ad_e42925, 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1352, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1718] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
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
        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((s.v[1608] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1719] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (s.v[1719] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1718] != 0.0))) && (!(s.v[1719] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1720] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1720] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1720] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((s.v[1608] != 0.0) && (s.v[1720] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1721] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1720] != 0.0))) && (s.v[1721] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1720] != 0.0))) && (s.v[1721] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1720] != 0.0))) && (s.v[1721] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1720] != 0.0))) && (!(s.v[1721] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1720] != 0.0))) && (!(s.v[1721] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1722] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1722] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((s.v[1608] != 0.0) && (s.v[1722] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((s.v[1608] != 0.0) && (s.v[1722] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1722] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1722] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1722] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1722] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1722] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1723] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1723] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1723] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((s.v[1608] != 0.0) && (s.v[1723] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((s.v[1608] != 0.0) && (s.v[1723] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1723] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1723] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1529);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1723] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1723] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1529)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1376, 1461, 1373);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1529, 1529, 1380);
        }

        s.v[1724] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        s.v[1725] = if (((s.v[1380]) as f64).abs() > 0.01) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_mul(1351, 1460, 1529);
        }

        s.v[1726] = if (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1726] != 0.0)) {
            s.store_exp_ad(1342, A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1726] != 0.0))) {
            let assign38810_ad_e43893: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1342, assign38810_ad_e43893, 5.54062e34);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_mul(1352, 1433, 1342);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_add_ad_lhs(1354, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1351)), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1355, A::mul(A::scale(s.ad_value(1460), 2.0), s.ad_value(1460)), 1352);
        }

        s.v[1727] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        s.v[1728] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1353)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_div_ad_lhs(1342, A::scale(s.ad_value(1354), 0.25), 1353);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_mul_ad_lhs(1358, A::add(s.ad_value(1353), A::mul(s.ad_value(1357), A::sub_from_scalar(2.0, s.ad_value(1357)))), 1342);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_add_ad(1360, A::mul(A::sub(s.ad_value(1354), A::mul(A::scale(s.ad_value(1358), 2.0), A::offset(s.ad_value(1357), 1.0))), s.ad_value(1342)), A::div(A::mul(s.ad_value(1358), s.ad_value(1355)), s.ad_value(1354)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_mul_ad_lhs(1363, A::div(s.ad_value(1354), s.ad_value(1353)), 1343);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (s.v[1728] != 0.0)) {
            s.store_div_ad_lhs(1364, A::sub(A::mul(s.ad_value(1355), s.ad_value(1343)), A::mul(s.ad_value(1354), A::add(s.ad_value(1363), A::scale(s.ad_value(1358), 0.5)))), 1353);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_scale_ad(1344, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.025)))))), 0.1666666666667);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_offset_ad(1357, A::mul(s.ad_value(1353), s.ad_value(1344)), 2.0);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_scale_ad(1342, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_mul(1358, 1354, 1342);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_scale_ad(1343, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_sub_ad(1360, A::mul(s.ad_value(1355), s.ad_value(1342)), A::mul(A::square(s.ad_value(1354)), s.ad_value(1343)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::scale(s.ad_value(1354), (-0.5)), 1344);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1727] != 0.0))) && (!(s.v[1728] != 0.0))) {
            s.store_add_ad(1364, A::mul(A::scale(s.ad_value(1355), (-0.5)), s.ad_value(1344)), A::mul(A::mul(A::scale(s.ad_value(1354), (0.25 * 0.0055555555556)), s.ad_value(1354)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1353), 0.075))))));
        }

        s.v[1729] = if (s.v[1353] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1729] != 0.0)) {
            s.store_div_ad(1343, A::scale(s.ad_value(1353), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1729] != 0.0)) {
            s.store_mul(1361, 1343, 1359);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1729] != 0.0)) {
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.v[1730] = if (s.v[1353] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1729] != 0.0))) && (s.v[1730] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1729] != 0.0))) && (s.v[1730] != 0.0)) {
            s.store_div_ad(1361, A::neg(s.ad_value(1353)), A::square(s.ad_value(1343)));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1729] != 0.0))) && (s.v[1730] != 0.0)) {
            s.store_ln(1362, 1361);
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1729] != 0.0))) && (!(s.v[1730] != 0.0))) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul(A::scale(s.ad_value(1353), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1353), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1353), 0.0396825396825397))))));
        }

        if (((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1729] != 0.0))) && (!(s.v[1730] != 0.0))) {
            s.store_ln(1362, 1361);
        }

        s.v[1731] = if (((1.01 * s.v[1351]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1731] != 0.0)) {
            s.store_add(1365, 1351, 1357);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1731] != 0.0)) {
            s.store_add(1366, 1460, 1358);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1731] != 0.0)) {
            s.copy_ad(1367, 1360);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1731] != 0.0))) {
            s.store_div_from_scalar_ad(1343, 1.0, A::sub(s.ad_value(1351), s.ad_value(1357)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1731] != 0.0))) {
            s.store_sub(1344, 1358, 1460);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1731] != 0.0))) {
            s.store_mul_ad_lhs(1365, A::sub(s.ad_value(1352), s.ad_value(1361)), 1343);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1731] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::sub(A::sub(A::mul(s.ad_value(1344), s.ad_value(1365)), s.ad_value(1352)), A::mul(s.ad_value(1363), s.ad_value(1361))), 1343);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1731] != 0.0))) {
            s.store_mul_ad_lhs(1367, A::sub(A::add(A::add(A::mul(s.ad_value(1360), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1344), 2.0), s.ad_value(1366))), s.ad_value(1352)), A::mul(A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361))), 1343);
        }

        s.v[1732] = if (s.v[1365] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1732] != 0.0)) {
            s.store_ln(1368, 1365);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1732] != 0.0)) {
            s.store_div_from_scalar(1342, 1.0, 1365);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1732] != 0.0)) {
            s.store_mul(1369, 1366, 1342);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (s.v[1732] != 0.0)) {
            s.store_sub_ad(1370, A::mul(s.ad_value(1367), s.ad_value(1342)), A::square(s.ad_value(1369)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1732] != 0.0))) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln(A::neg(s.ad_value(1351))));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1732] != 0.0))) {
            s.store_div_from_scalar(1342, 1.0, 1529);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1732] != 0.0))) {
            s.store_add(1369, 1460, 1342);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) && (!(s.v[1732] != 0.0))) {
            s.store_mul_ad_lhs(1370, A::neg(s.ad_value(1342)), 1342);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1371, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1529)), A::scale(s.ad_value(1368), 2.0)), 1362);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1372, A::offset(A::scale(s.ad_value(1369), 2.0), 1.0), 1363);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1373, A::scale(s.ad_value(1370), 2.0), 1364);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_add_ad_rhs(1374, 1351, A::mul(s.ad_value(1461), s.ad_value(1371)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_add_ad_rhs(1375, 1460, A::mul(s.ad_value(1461), s.ad_value(1372)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_mul(1376, 1461, 1373);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1377, A::mul(s.ad_value(1374), s.ad_value(1365)), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_add_ad_lhs(1378, A::add(A::mul(s.ad_value(1375), s.ad_value(1365)), A::mul(s.ad_value(1374), s.ad_value(1366))), 1352);
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1379, A::add(A::add(A::mul(s.ad_value(1376), s.ad_value(1365)), A::mul(A::scale(s.ad_value(1375), 2.0), s.ad_value(1366))), A::mul(s.ad_value(1374), s.ad_value(1367))), 1352);
        }

    }

    pub(super) fn stamp_transient_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad(1388, A::square(s.ad_value(1378)), A::mul(A::scale(s.ad_value(1377), 0.5), s.ad_value(1379)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_div_ad(1380, A::mul(A::mul(A::neg(s.ad_value(1377)), s.ad_value(1378)), s.ad_value(1388)), A::offset(A::square(s.ad_value(1388)), 1e-200));
        }

        if (((s.v[1608] != 0.0) && (s.v[1724] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_add(1529, 1529, 1380);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1532, 1460, 1529);
        }

        s.v[1733] = if (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1733] != 0.0)) {
            s.store_exp_ad(1342, A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1733] != 0.0))) {
            let assign39560_ad_e45102: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1458), s.ad_value(1529)), s.ad_value(1528)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1342, assign39560_ad_e45102, 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1535, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1534, A::square(s.ad_value(1532)), 1535);
        }

        s.v[1734] = if (s.v[1535] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1734] != 0.0)) {
            s.store_scalar(1531, 1e-80);
        }

        if ((s.v[1608] != 0.0) && (s.v[1734] != 0.0)) {
            s.store_sub(1533, 1531, 1532);
        }

        if ((s.v[1608] != 0.0) && (s.v[1734] != 0.0)) {
            s.store_div(1530, 1533, 1461);
        }

        s.v[1735] = if (s.v[1534] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1735] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1534)));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1735] != 0.0)) {
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        s.v[1736] = if (s.v[1534] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1735] != 0.0))) && (s.v[1736] != 0.0)) {
            s.store_sqrt_ad(1356, A::abs(s.ad_value(1534)));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1735] != 0.0))) && (s.v[1736] != 0.0)) {
            s.store_exp_ad(1359, A::neg(s.ad_value(1356)));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1735] != 0.0))) && (s.v[1736] != 0.0)) {
            s.store_div_ad(1357, A::mul(s.ad_value(1356), A::offset(s.ad_value(1359), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1359)));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1735] != 0.0))) && (!(s.v[1736] != 0.0))) {
            s.store_offset_ad(1357, A::mul(A::scale(s.ad_value(1534), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1534), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1534), 0.0238095238095))))), 2.0);
        }

        s.v[1737] = if (((1.01 * s.v[1532]) + s.v[1357]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) {
            s.store_add(1342, 1532, 1357);
        }

        s.v[1738] = if ((s.v[1535] * s.v[1532]) < (((0.9 * s.v[1532]) * s.v[1532]) * s.v[1342])) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (s.v[1738] != 0.0)) {
            s.store_offset_ad(1531, A::div(s.ad_value(1535), s.ad_value(1342)), 1e-80);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (s.v[1738] != 0.0)) {
            s.store_sub(1533, 1531, 1532);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (s.v[1738] != 0.0)) {
            s.store_div(1530, 1533, 1461);
        }

        s.v[1739] = if (s.v[1534] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) && (s.v[1739] != 0.0)) {
            s.store_sub_ad_lhs(1343, A::ln(A::div(A::scale(s.ad_value(1534), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))))), 1356);
        }

        s.v[1740] = if (s.v[1534] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) && (!(s.v[1739] != 0.0))) && (s.v[1740] != 0.0)) {
            s.store_sin_ad(1344, A::scale(s.ad_value(1356), 0.5));
        }

        if ((((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) && (!(s.v[1739] != 0.0))) && (s.v[1740] != 0.0)) {
            s.store_ln_ad(1343, A::div(A::neg(s.ad_value(1534)), A::square(s.ad_value(1344))));
        }

        if ((((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) && (!(s.v[1739] != 0.0))) && (!(s.v[1740] != 0.0))) {
            s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1534), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1534), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1534), 0.0396825396825397)))))));
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) {
            s.store_sub_ad_lhs(1530, A::add(A::add(A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1529)), A::scale(A::ln(s.ad_value(1342)), 2.0)), 1343);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) {
            s.store_mul(1533, 1461, 1530);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) {
            s.store_add(1531, 1532, 1533);
        }

        s.v[1741] = if (s.v[1534] > 0.005) { 1.0 } else { 0.0 };

        s.v[1742] = if ((((s.v[1529] + s.v[1528]) - s.v[1458]) - s.v[1356]) < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) && (s.v[1741] != 0.0)) && (s.v[1742] != 0.0)) {
            s.store_exp_ad(1344, A::sub(A::sub(A::add(s.ad_value(1529), s.ad_value(1528)), s.ad_value(1458)), s.ad_value(1356)));
        }

        if (((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) && (s.v[1741] != 0.0)) && (!(s.v[1742] != 0.0))) {
            let assign39890_ad_e45570: A = A::mul(A::offset(A::sub(A::sub(A::add(s.ad_value(1529), s.ad_value(1528)), s.ad_value(1458)), s.ad_value(1356)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(A::add(s.ad_value(1529), s.ad_value(1528)), s.ad_value(1458)), s.ad_value(1356)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(A::add(s.ad_value(1529), s.ad_value(1528)), s.ad_value(1458)), s.ad_value(1356)), (-80.0)), 0.3333333333333), 1.0)), 1.0));
            s.store_scale_ad(1344, A::offset(assign39890_ad_e45570, 1.0), 5.54062e34);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) && (s.v[1741] != 0.0)) {
            s.store_div(1343, 1344, 1433);
        }

        if ((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) && (s.v[1741] != 0.0)) {
            s.store_div_ad(1342, A::mul(A::scale(s.ad_value(1534), 4.0), s.ad_value(1343)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1359), A::sub_from_scalar(2.0, s.ad_value(1359)))));
        }

        s.v[1743] = if (s.v[1534] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) && (!(s.v[1741] != 0.0))) && (s.v[1743] != 0.0)) {
            s.store_sin_ad(1343, A::scale(s.ad_value(1356), 0.5));
        }

        if (((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) && (!(s.v[1741] != 0.0))) && (s.v[1743] != 0.0)) {
            s.store_div_ad_lhs(1342, A::div(A::neg(s.ad_value(1534)), A::square(s.ad_value(1343))), 1535);
        }

        if (((((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) && (!(s.v[1741] != 0.0))) && (!(s.v[1743] != 0.0))) {
            s.store_div_ad_lhs(1342, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1534), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1534), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1534), 0.0396825396825397)))))), 1535);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) {
            s.store_offset_ad(1531, A::div(A::sub(s.ad_value(1532), s.ad_value(1357)), A::sub_from_scalar(1.0, s.ad_value(1342))), 1e-80);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) {
            s.store_sub(1533, 1531, 1532);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1734] != 0.0))) && (!(s.v[1737] != 0.0))) {
            s.store_div(1530, 1533, 1461);
        }

        s.v[1744] = if (((s.v[1459] - s.v[1530]) - s.v[1528]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1744] != 0.0)) {
            s.store_exp_ad(1342, A::sub(A::sub(s.ad_value(1459), s.ad_value(1530)), s.ad_value(1528)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1744] != 0.0))) {
            let assign40010_ad_e45788: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1459), s.ad_value(1530)), s.ad_value(1528)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1459), s.ad_value(1530)), s.ad_value(1528)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1459), s.ad_value(1530)), s.ad_value(1528)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1342, assign40010_ad_e45788, 5.54062e34);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1536, 1433, 1342);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1539, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1540, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1537, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1538, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1541, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1542, 0.0);
        }

        s.v[1745] = if (s.v[1466] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) {
            s.store_mul(1537, 1535, 1434);
        }

        if ((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) {
            s.store_mul(1538, 1536, 1435);
        }

        if ((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) {
            s.store_add_ad_rhs(1539, 1537, A::scale(s.ad_value(1532), 2.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) {
            s.store_add_ad_rhs(1540, 1538, A::scale(s.ad_value(1533), 2.0));
        }

        if ((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) {
            s.store_add_ad_lhs(1541, A::add(A::scale(s.ad_value(1531), 2.0), s.ad_value(1537)), 1538);
        }

        s.v[1746] = if (((s.v[1534]) as f64).abs() > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) && (s.v[1746] != 0.0)) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(1539), s.ad_value(1540)), A::mul(A::scale(A::offset(s.ad_value(1529), 2.0), 2.0), s.ad_value(1540))), A::mul(A::scale(A::offset(s.ad_value(1530), 2.0), 2.0), s.ad_value(1539)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) && (s.v[1746] != 0.0)) {
            s.store_div_ad(1542, A::mul(A::scale(s.ad_value(1534), (-4.0)), s.ad_value(1541)), A::mul(s.ad_value(1531), s.ad_value(2)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) && (!(s.v[1746] != 0.0))) {
            s.store_scale_ad(2, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1534), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1534), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1534), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) && (!(s.v[1746] != 0.0))) {
            s.store_add_ad(3, A::add(A::mul(s.ad_value(1539), s.ad_value(1535)), A::mul(s.ad_value(1540), s.ad_value(1536))), A::mul(A::mul(A::mul(s.ad_value(1539), s.ad_value(1540)), s.ad_value(1531)), A::offset(A::mul(s.ad_value(1531), s.ad_value(2)), 1.0)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1745] != 0.0)) && (!(s.v[1746] != 0.0))) {
            s.store_div_ad(1542, A::mul(A::mul(s.ad_value(1535), s.ad_value(1536)), s.ad_value(1541)), A::mul(s.ad_value(1531), s.ad_value(3)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad_rhs(1543, 1528, A::ln(s.ad_value(1531)));
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_add(1544, 1466, 1531, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1545, 1543, 1479);
        }

        if (s.v[1608] != 0.0) {
            s.store_scalar(1548, 1.0);
        }

        s.v[1747] = if (p.p9 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1747] != 0.0)) {
            s.store_div_ad_lhs(1546, A::scale(A::add(s.ad_value(1467), s.ad_value(1532)), 0.5), 1460);
        }

        if ((s.v[1608] != 0.0) && (s.v[1747] != 0.0)) {
            s.store_scale_ad(1546, A::add(A::offset(s.ad_value(1546), 1e-5), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1546), (-1e-5)), A::offset(s.ad_value(1546), (-1e-5))), 1.0))), 0.5);
        }

        if ((s.v[1608] != 0.0) && (s.v[1747] != 0.0)) {
            s.store_sub_ad(1, A::sqrt(A::add(A::div(s.ad_value(1546), s.ad_value(227)), A::mul(A::scale(s.ad_value(250), 0.25), s.ad_value(250)))), A::scale(s.ad_value(250), 0.5));
        }

        if ((s.v[1608] != 0.0) && (s.v[1747] != 0.0)) {
            s.store_mul_ad_lhs(1547, A::powf(s.ad_value(1), 2.0), 227);
        }

        if ((s.v[1608] != 0.0) && (s.v[1747] != 0.0)) {
            s.store_sub_from_scalar_ad(1548, 1.0, A::div(s.ad_value(1547), s.ad_value(1546)));
        }

        s.v[1748] = if ((s.v[1532] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1748] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::scale(s.ad_value(1532), 0.5)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1748] != 0.0))) {
            s.store_scale(2, 1532, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scale(1549, 2, 2.0);
        }

        s.v[1749] = if ((s.v[1533] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1749] != 0.0)) {
            s.store_ln_ad(3, A::offset(A::exp(A::scale(s.ad_value(1533), 0.5)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1749] != 0.0))) {
            s.store_scale(3, 1533, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scale(1550, 3, 2.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1551, 1550, 1533);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub(1552, 1549, 1532);
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1553, A::mul(s.ad_value(270), s.ad_value(1549)), A::mul(s.ad_value(271), s.ad_value(1551)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1554, A::mul(s.ad_value(270), s.ad_value(1550)), A::mul(s.ad_value(271), s.ad_value(1552)));
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_add(1555, 1480, 1549, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_add(1556, 1481, 1550, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(0, 1.0, A::add(s.ad_value(1555), s.ad_value(1556)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1557, A::mul(s.ad_value(1544), s.ad_value(1555)), 0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1558, A::mul(s.ad_value(1544), s.ad_value(1556)), 0);
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_add(1559, 1482, 1551, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_add(1560, 1483, 1552, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_add(1561, 1484, 1553, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_add(1562, 1485, 1554, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1563, A::mul(A::mul(s.ad_value(1555), s.ad_value(191)), A::exp(A::mul(s.ad_value(40), s.ad_value(295)))), 1548);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(1564, A::mul(s.ad_value(1556), s.ad_value(192)), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
        }

        if (s.v[1608] != 0.0) {
            s.store_add(1565, 1563, 1564);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(2, 50, A::add(s.ad_value(1559), A::mul(s.ad_value(51), s.ad_value(1560))));
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(3, A::add(A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scale_ad(4, A::add(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_div(1566, 3, 4);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(1567, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(1559)), 1.0), A::mul(s.ad_value(42), s.ad_value(1560)))), A::exp(A::mul(A::neg(s.ad_value(44)), A::ln(A::add(A::offset(A::mul(s.ad_value(1557), s.ad_value(268)), 1.0), A::mul(s.ad_value(1558), s.ad_value(269)))))));
        }

        s.v[1750] = if (s.v[56] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1750] != 0.0)) {
            s.store_scalar(4, 1.0);
        }

        s.v[1751] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (!(s.v[1750] != 0.0))) && (s.v[1751] != 0.0)) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12)))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1750] != 0.0))) && (s.v[1751] != 0.0)) {
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1750] != 0.0))) && (!(s.v[1751] != 0.0))) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12)))));
        }

        if (((s.v[1608] != 0.0) && (!(s.v[1750] != 0.0))) && (!(s.v[1751] != 0.0))) {
            s.store_div_from_scalar_ad(4, 1.0, A::offset(s.ad_value(2), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_rhs(1568, 1492, A::add(A::mul(s.ad_value(1544), s.ad_value(4)), s.ad_value(54)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1569, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1561)), 1e-6)))), 1.0), s.ad_value(1567)), A::mul(s.ad_value(38), s.ad_value(1568)));
        }

        if (s.v[1608] != 0.0) {
            s.store_add_ad(1570, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1562)), 1e-6)))), 1.0), s.ad_value(1567)), A::mul(s.ad_value(39), s.ad_value(1568)));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1571, A::mul(s.ad_value(1566), s.ad_value(1565)), A::add(A::div(s.ad_value(1563), s.ad_value(1569)), A::div(s.ad_value(1564), s.ad_value(1570))));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1572, 1.0, A::offset(s.ad_value(1544), 4.0));
        }

        s.v[1752] = if (s.v[65] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1752] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.0, A::offset(A::mul(s.ad_value(65), s.ad_value(1558)), 1.0));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1752] != 0.0))) {
            s.store_sub_from_scalar_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1558)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1573, A::mul(s.ad_value(1544), s.ad_value(1572)), 0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad_lhs(1574, A::ln(A::offset(A::div(A::sub(s.ad_value(339), s.ad_value(1528)), A::add(A::mul(s.ad_value(66), s.ad_value(227)), A::mul(A::mul(s.ad_value(67), s.ad_value(1544)), s.ad_value(1544)))), 1.0)), 1573);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1575, 1426, 1574);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_from_scalar_ad(1576, 1.0, A::offset(A::mul(s.ad_value(1575), A::offset(s.ad_value(1575), 1.0)), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1504, A::scale(s.ad_value(1555), 100.0), A::offset(s.ad_value(1555), 100.0));
        }

        s.v[1753] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1753] != 0.0)) {
            s.store_div_from_scalar_ad(1505, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(61), s.ad_value(1504))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1753] != 0.0))) {
            s.store_offset_ad(1505, A::mul(s.ad_value(61), s.ad_value(1504)), 1.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad(1506, A::scale(s.ad_value(1556), 100.0), A::offset(s.ad_value(1556), 100.0));
        }

        s.v[1754] = if (s.v[62] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1754] != 0.0)) {
            s.store_div_from_scalar_ad(1507, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(62), s.ad_value(1506))));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1754] != 0.0))) {
            s.store_offset_ad(1507, A::mul(s.ad_value(62), s.ad_value(1506)), 1.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul_ad(1577, A::scale(A::mul(s.ad_value(1424), s.ad_value(1545)), 0.5), A::add(s.ad_value(1505), s.ad_value(1507)));
        }

    }

    pub(super) fn stamp_transient_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1608] != 0.0) {
            s.store_div_ad_rhs(1578, 1577, A::mul(s.ad_value(1571), s.ad_value(1576)));
        }

        if (s.v[1608] != 0.0) {
            s.store_square(1579, 1578);
        }

        if (s.v[1608] != 0.0) {
            s.store_sqrt_ad(1580, A::offset(s.ad_value(1579), 1.0));
        }

        if (s.v[1608] != 0.0) {
            s.store_div_ad_lhs(1581, A::offset(A::scale(s.ad_value(1579), 1.5), 1.0), 1580);
        }

        s.v[1755] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1608] != 0.0) && (s.v[1755] != 0.0)) {
            s.store_mul_ad(2, A::scale(s.ad_value(258), 0.6), A::exp(A::scale(A::ln(A::offset(A::square(s.ad_value(1555)), 60.0)), (-0.1666666666667))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1755] != 0.0)) {
            s.store_mul_ad(3, A::scale(s.ad_value(258), 0.6), A::exp(A::scale(A::ln(A::offset(A::square(s.ad_value(1556)), 60.0)), (-0.1666666666667))));
        }

        if ((s.v[1608] != 0.0) && (s.v[1755] != 0.0)) {
            s.store_div_ad_lhs(1582, A::offset(A::mul(s.ad_value(1460), s.ad_value(2)), 1.0), 1441);
        }

        if ((s.v[1608] != 0.0) && (s.v[1755] != 0.0)) {
            s.store_div_ad_lhs(1583, A::offset(A::mul(s.ad_value(1461), s.ad_value(3)), 1.0), 1442);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1755] != 0.0))) {
            s.store_scalar(1582, 1.0);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1755] != 0.0))) {
            s.store_scalar(1583, 1.0);
        }

        s.v[1756] = if (s.v[1466] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1757] = if (s.v[1531] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1758] = if (((s.v[1540]) as f64).abs() < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_div_ad(0, A::add(A::offset(s.ad_value(1529), 2.0), A::scale(s.ad_value(1539), 0.5)), A::mul(A::offset(s.ad_value(1530), 2.0), s.ad_value(1539)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_mul(2, 0, 1540);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_square(3, 2);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_sub_ad_rhs(5, 4, A::mul(s.ad_value(2), s.ad_value(3)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_div_ad(2, A::sub(s.ad_value(1533), A::mul(A::mul(A::scale(s.ad_value(1534), 2.0), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1539)))), s.ad_value(5))), A::offset(s.ad_value(1530), 2.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_div_ad_lhs(1584, A::sub(A::div(A::sub(A::mul(s.ad_value(1542), s.ad_value(1531)), s.ad_value(1535)), s.ad_value(1539)), s.ad_value(2)), 1531);
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (s.v[1758] != 0.0)) {
            s.store_div_ad(1585, A::mul(s.ad_value(1584), s.ad_value(1531)), A::offset(s.ad_value(1584), 1.0));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (!(s.v[1758] != 0.0))) {
            s.store_sub_ad(1584, A::div(A::mul(s.ad_value(1542), s.ad_value(1541)), A::mul(s.ad_value(1539), s.ad_value(1540))), A::div(A::add(A::div(s.ad_value(1535), s.ad_value(1539)), A::div(s.ad_value(1536), s.ad_value(1540))), s.ad_value(1531)));
        }

        if ((((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) && (!(s.v[1758] != 0.0))) {
            s.store_div_ad(1585, A::mul(s.ad_value(1584), s.ad_value(1531)), A::offset(s.ad_value(1584), 1.0));
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (!(s.v[1757] != 0.0))) {
            s.copy_ad(1585, 1502);
        }

        if ((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) {
            s.store_sub(2, 1585, 1509);
        }

        if ((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) {
            s.store_offset_ad(3, A::mul(A::scale(s.ad_value(2), 36.0), s.ad_value(2)), 1.0);
        }

        s.v[1759] = if (((s.v[2]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1759] != 0.0)) {
            s.store_sub(4, 1531, 1466);
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1759] != 0.0)) {
            s.store_sub_ad_rhs(1586, 4, A::mul(s.ad_value(1585), s.ad_value(1545)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1759] != 0.0)) {
            s.store_sub_ad_rhs(1587, 4, A::mul(s.ad_value(1509), s.ad_value(1545)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1759] != 0.0)) {
            s.store_sqrt_ad(1588, A::add(A::square(s.ad_value(1586)), s.ad_value(3)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1759] != 0.0)) {
            s.store_sqrt_ad(1589, A::add(A::square(s.ad_value(1587)), s.ad_value(3)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1759] != 0.0)) {
            s.store_mul_ad(1590, A::div_from_scalar(0.25, s.ad_value(2)), A::add(A::sub(A::mul(s.ad_value(1589), s.ad_value(1586)), A::mul(s.ad_value(1588), s.ad_value(1587))), A::mul(s.ad_value(3), A::ln(A::div(A::add(s.ad_value(1587), s.ad_value(1589)), A::add(s.ad_value(1586), s.ad_value(1588)))))));
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (!(s.v[1759] != 0.0))) {
            s.store_mul(4, 1545, 2);
        }

        if (((s.v[1608] != 0.0) && (s.v[1756] != 0.0)) && (!(s.v[1759] != 0.0))) {
            s.store_div_ad(1590, A::mul(A::mul(A::scale(s.ad_value(1545), ((-0.25) * 0.1666666666667)), s.ad_value(4)), s.ad_value(4)), A::sqrt(s.ad_value(3)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.copy_ad(1585, 1502);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_scalar(1590, 0.0);
        }

        if (s.v[1608] != 0.0) {
            s.store_sub_ad_lhs(1591, A::add(A::add(A::mul(s.ad_value(1544), s.ad_value(1545)), s.ad_value(1590)), s.ad_value(1466)), 1531);
        }

        s.v[1760] = if (s.v[1466] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1761] = if (s.v[1591] > 1e-30) { 1.0 } else { 0.0 };

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) {
            s.store_div_ad_rhs(1592, 1475, A::sub(A::div(s.ad_value(1471), s.ad_value(1466)), s.ad_value(1478)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) {
            s.store_div_ad_rhs(1593, 1539, A::sub(A::div(s.ad_value(1535), s.ad_value(1531)), s.ad_value(1542)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) {
            s.store_div_ad_lhs(1594, A::sub(s.ad_value(1592), s.ad_value(1593)), 1591);
        }

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) {
            s.store_div_ad_rhs(1595, 1476, A::sub(A::div(s.ad_value(1472), s.ad_value(1466)), s.ad_value(1478)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) {
            s.store_div_ad_rhs(1596, 1540, A::sub(A::div(s.ad_value(1536), s.ad_value(1531)), s.ad_value(1542)));
        }

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) {
            s.store_div_ad_lhs(1597, A::sub(s.ad_value(1595), s.ad_value(1596)), 1591);
        }

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_scalar(1594, 0.0);
        }

        if (((s.v[1608] != 0.0) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_scalar(1597, 0.0);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_mul_ad(1598, A::scale(s.ad_value(1497), (-2.0)), A::add(A::div(s.ad_value(1434), s.ad_value(1500)), s.ad_value(1503)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_mul_ad(1599, A::scale(s.ad_value(1498), (-2.0)), A::add(A::div(s.ad_value(1435), s.ad_value(1501)), s.ad_value(1503)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_mul_ad_lhs(0, A::sub(s.ad_value(1599), s.ad_value(1598)), 1503);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_mul(2, 1598, 1434);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_mul(3, 1599, 1435);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_add(4, 2, 3);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_offset_ad(5, A::scale(A::add(A::mul(s.ad_value(1497), s.ad_value(1434)), A::mul(s.ad_value(1498), s.ad_value(1435))), 2.0), 3.0);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_div_ad_lhs(1600, A::sub(A::add(s.ad_value(3), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(1500))), 5);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_div_ad_lhs(1601, A::sub(A::sub(s.ad_value(2), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(1501))), 5);
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_mul_ad(1594, A::neg(s.ad_value(1500)), A::add(A::mul(s.ad_value(1600), s.ad_value(1500)), s.ad_value(1503)));
        }

        if ((s.v[1608] != 0.0) && (!(s.v[1760] != 0.0))) {
            s.store_mul_ad(1597, A::neg(s.ad_value(1501)), A::add(A::mul(s.ad_value(1601), s.ad_value(1501)), s.ad_value(1503)));
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1602, 1594, 1581);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1603, 1597, 1581);
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_sub(1604, 1532, 1467, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_scaled_sub(1605, 1533, 1468, 0.5);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1606, 1604, 1602);
        }

        if (s.v[1608] != 0.0) {
            s.store_mul(1607, 1605, 1603);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(440, 1428);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(441, 1432);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(442, 1433);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(443, 1434);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(444, 1435);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(445, 1462);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(446, 1463);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(447, 1447);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(448, 1446);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(449, 1450);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(450, 1451);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(451, 1452);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(452, 1453);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(453, 1454);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(454, 1457);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(455, 1459);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(456, 1460);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(457, 1461);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(458, 1467);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(459, 1468);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(460, 1479);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(461, 1532);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(462, 1533);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(463, 1543);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(464, 1544);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(465, 1548);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(466, 1557);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(467, 1558);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(468, 1579);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(469, 1582);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(470, 1583);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(471, 1604);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(472, 1605);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(473, 1606);
        }

        if (s.v[1608] != 0.0) {
            s.copy_ad(474, 1607);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(440, 383);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(441, 384);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(442, 385);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(443, 386);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(444, 387);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(445, 388);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(446, 389);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(447, 390);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(448, 391);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(449, 393);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(450, 394);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(451, 395);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(452, 396);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(453, 397);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(454, 398);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(455, 399);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(456, 401);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(457, 402);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(458, 404);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(459, 405);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(460, 406);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(461, 408);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(462, 409);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(463, 414);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(464, 415);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(465, 416);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(466, 419);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(467, 420);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(468, 428);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(469, 430);
        }

    }

    pub(super) fn stamp_transient_block_29(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(470, 431);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(471, 436);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(472, 437);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(473, 438);
        }

        if (!(s.v[1608] != 0.0)) {
            s.copy_ad(474, 439);
        }

        s.store_div_ad(0, A::mul(s.ad_value(120), A::sub(s.ad_value(448), s.ad_value(446))), A::offset(A::scale(s.ad_value(464), 0.25), 1.0));

        s.store_add_ad_lhs(1324, A::scale(A::add(s.ad_value(458), s.ad_value(461)), 0.5), 0);

        s.store_sub_ad_lhs(1325, A::scale(A::add(s.ad_value(459), s.ad_value(462)), 0.5), 0);

        s.v[1762] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1762] != 0.0) {
            s.store_sub_ad_lhs(1326, A::add(s.ad_value(1324), A::div(s.ad_value(466), s.ad_value(469))), 466);
        }

        if (s.v[1762] != 0.0) {
            s.store_sub_ad_lhs(1327, A::add(s.ad_value(1325), A::div(s.ad_value(467), s.ad_value(470))), 467);
        }

        if (!(s.v[1762] != 0.0)) {
            s.copy_ad(1326, 1324);
        }

        if (!(s.v[1762] != 0.0)) {
            s.copy_ad(1327, 1325);
        }

        s.store_scaled_mul(2, 471, 473, 0.3333333333333);

        s.store_mul_ad(3, A::scale(s.ad_value(471), 0.1666666666667), A::offset(A::mul(s.ad_value(473), A::sub_from_scalar(1.0, A::scale(s.ad_value(473), 0.2))), 1.0));

        s.store_add_ad_lhs(1328, A::mul(A::scale(s.ad_value(1326), 0.5), s.ad_value(465)), 3);

        s.store_add_ad_lhs(1326, A::mul(s.ad_value(1326), s.ad_value(465)), 2);

        s.store_scaled_mul(2, 472, 474, 0.3333333333333);

        s.store_mul_ad(3, A::scale(s.ad_value(472), 0.1666666666667), A::offset(A::mul(s.ad_value(474), A::sub_from_scalar(1.0, A::scale(s.ad_value(474), 0.2))), 1.0));

        s.store_add_ad_lhs(1329, A::scale(s.ad_value(1327), 0.5), 3);

        s.store_add(1327, 1327, 2);

        s.store_mul(0, 447, 287);

        s.store_mul(361, 0, 1326);

        s.store_mul(362, 0, 1327);

        s.store_mul_ad(363, A::neg(s.ad_value(0)), A::add(s.ad_value(1328), s.ad_value(1329)));

        s.v[1763] = if (s.v[119] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1763] != 0.0) {
            s.store_offset(0, 254, (2.0 * 0.6931471805599));
        }

        if (s.v[1763] != 0.0) {
            s.store_add(1330, 460, 0);
        }

        if (s.v[1763] != 0.0) {
            s.store_add(1331, 463, 0);
        }

        if (s.v[1763] != 0.0) {
            s.store_scale_ad(1332, A::sub(A::add(s.ad_value(1330), s.ad_value(254)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1330), s.ad_value(254)), A::sub(s.ad_value(1330), s.ad_value(254))), 9.0))), 0.5);
        }

        if (s.v[1763] != 0.0) {
            s.store_scale_ad(1333, A::sub(A::add(s.ad_value(1331), A::add(s.ad_value(254), s.ad_value(339))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1331), A::add(s.ad_value(254), s.ad_value(339))), A::sub(s.ad_value(1331), A::add(s.ad_value(254), s.ad_value(339)))), 9.0))), 0.5);
        }

        if (s.v[1763] != 0.0) {
            s.store_mul_ad_rhs(1334, 294, A::sqrt(A::mul(s.ad_value(445), A::offset(s.ad_value(444), 0.5))));
        }

        if (s.v[1763] != 0.0) {
            s.store_mul_ad_rhs(1335, 294, A::sqrt(A::mul(A::mul(A::mul(s.ad_value(445), s.ad_value(456)), s.ad_value(444)), A::offset(s.ad_value(443), 0.5))));
        }

        if (s.v[1763] != 0.0) {
            s.store_mul_ad_lhs(1336, A::square(s.ad_value(1334)), 291);
        }

        if (s.v[1763] != 0.0) {
            s.store_mul_ad_lhs(1337, A::square(s.ad_value(1335)), 291);
        }

        if (s.v[1763] != 0.0) {
            s.store_sub(2, 292, 1332);
        }

        if (s.v[1763] != 0.0) {
            s.store_sub_ad_lhs(3, A::add(s.ad_value(292), s.ad_value(339)), 1333);
        }

        if (s.v[1763] != 0.0) {
            s.store_scale(0, 1336, 2.0);
        }

        if (s.v[1763] != 0.0) {
            s.store_add_ad_rhs(1338, 1332, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1336)), 1.0)), (-1.0))));
        }

        if (s.v[1763] != 0.0) {
            s.store_add_ad_rhs(1339, 1333, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1336)), 1.0)), (-1.0))));
        }

        if (s.v[1763] != 0.0) {
            s.store_scale(0, 1337, 2.0);
        }

        if (s.v[1763] != 0.0) {
            s.store_add_ad_rhs(1340, 1332, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1337)), 1.0)), (-1.0))));
        }

        if (s.v[1763] != 0.0) {
            s.store_add_ad_rhs(1341, 1333, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1337)), 1.0)), (-1.0))));
        }

        if (s.v[1763] != 0.0) {
            s.store_mul(0, 293, 447);
        }

        if (s.v[1763] != 0.0) {
            s.store_mul_ad_lhs(2, A::mul(A::mul(A::neg(s.ad_value(0)), s.ad_value(1334)), s.ad_value(456)), 451);
        }

        if (s.v[1763] != 0.0) {
            s.store_mul_ad_lhs(3, A::mul(A::mul(A::neg(s.ad_value(0)), s.ad_value(1335)), s.ad_value(457)), 452);
        }

        if (s.v[1763] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1338), s.ad_value(1330)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1338), s.ad_value(1330)), A::sub(s.ad_value(1338), s.ad_value(1330))), 1.0))), 0.5);
        }

        if (s.v[1763] != 0.0) {
            s.store_div_ad(379, A::mul(A::mul(s.ad_value(2), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1338), s.ad_value(1332)));
        }

        if (s.v[1763] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1339), s.ad_value(1331)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1339), s.ad_value(1331)), A::sub(s.ad_value(1339), s.ad_value(1331))), 1.0))), 0.5);
        }

        if (s.v[1763] != 0.0) {
            s.store_div_ad(380, A::mul(A::mul(s.ad_value(2), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1339), s.ad_value(1333)));
        }

        if (s.v[1763] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1340), s.ad_value(1330)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1340), s.ad_value(1330)), A::sub(s.ad_value(1340), s.ad_value(1330))), 1.0))), 0.5);
        }

        if (s.v[1763] != 0.0) {
            s.store_div_ad(381, A::mul(A::mul(s.ad_value(3), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1340), s.ad_value(1332)));
        }

        if (s.v[1763] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1341), s.ad_value(1331)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1341), s.ad_value(1331)), A::sub(s.ad_value(1341), s.ad_value(1331))), 1.0))), 0.5);
        }

        if (s.v[1763] != 0.0) {
            s.store_div_ad(382, A::mul(A::mul(s.ad_value(3), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1341), s.ad_value(1333)));
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(379, 0.0);
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(380, 0.0);
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(381, 0.0);
        }

        if (!(s.v[1763] != 0.0)) {
            s.store_scalar(382, 0.0);
        }

        s.store_mul(370, 164, 330);

        s.store_mul(371, 165, 332);

        let assign42790_ad_e48345: A = A::add(A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(449)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440))))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(449)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440))))), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(449)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440)))))), 0.2)));
        s.store_scale_ad(0, assign42790_ad_e48345, 0.5);

        s.store_mul_ad_lhs(372, A::mul(s.ad_value(159), s.ad_value(349)), 0);

        s.store_mul_ad_lhs(373, A::mul(s.ad_value(160), s.ad_value(350)), 0);

        s.store_mul(374, 117, 338);

        s.store_mul(375, 166, 336);

        s.store_mul_ad_lhs(377, A::neg(A::add(A::mul(s.ad_value(240), s.ad_value(9)), A::mul(s.ad_value(167), s.ad_value(11)))), 331);

        s.store_mul_ad_lhs(376, A::neg(A::add(A::mul(s.ad_value(240), s.ad_value(10)), A::mul(s.ad_value(167), s.ad_value(12)))), 333);

        s.v[1764] = if (s.v[6] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1764] != 0.0) {
            s.store_mul(378, 170, 219);
        }

        if (!(s.v[1764] != 0.0)) {
            s.store_scalar(378, 0.0);
        }

        s.store_mul_ad(365, A::scale(s.ad_value(13), p.p31), A::add(A::add(s.ad_value(348), s.ad_value(356)), s.ad_value(358)));

        s.store_mul_ad_lhs(366, A::scale(s.ad_value(13), p.p31), 352);

        s.store_mul_ad_lhs(367, A::scale(s.ad_value(13), p.p31), 353);

        s.store_mul_ad_lhs(368, A::scale(s.ad_value(13), p.p31), 354);

        s.store_mul_ad_lhs(369, A::scale(s.ad_value(13), p.p31), 355);

        s.store_mul(1765, 13, 359);

        s.store_mul(1766, 13, 360);

        s.v[1767] = if (s.v[334] < 0.0) { 1.0 } else { 0.0 };

        s.v[1768] = if (s.v[311] > 0.0) { 1.0 } else { 0.0 };

        s.v[1769] = if (s.v[318] > 0.0) { 1.0 } else { 0.0 };

        s.v[1770] = if (s.v[322] > 0.0) { 1.0 } else { 0.0 };

        s.v[1771] = if (s.v[326] > 0.0) { 1.0 } else { 0.0 };

        s.copy_ad(1774, 361);

        s.copy_ad(1775, 362);

        s.copy_ad(1776, 363);

        s.store_neg_ad(364, A::add(A::add(s.ad_value(361), s.ad_value(362)), s.ad_value(363)));

        s.v[1777] = if (s.v[334] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1777] != 0.0) {
            s.copy_ad(1776, 364);
        }

        s.store_mul_ad_lhs(361, A::scale(s.ad_value(13), p.p32), 361);

        s.store_mul_ad_lhs(362, A::scale(s.ad_value(13), p.p32), 362);

        s.store_mul_ad_lhs(363, A::scale(s.ad_value(13), p.p32), 363);

        s.store_neg_ad(364, A::add(A::add(s.ad_value(361), s.ad_value(362)), s.ad_value(363)));

        s.store_mul_ad_lhs(379, A::scale(s.ad_value(13), p.p32), 379);

        s.store_mul_ad_lhs(380, A::scale(s.ad_value(13), p.p32), 380);

        s.store_mul_ad_lhs(381, A::scale(s.ad_value(13), p.p32), 381);

        s.store_mul_ad_lhs(382, A::scale(s.ad_value(13), p.p32), 382);

        s.store_mul_ad_lhs(370, A::scale(s.ad_value(13), p.p32), 370);

        s.store_mul_ad_lhs(371, A::scale(s.ad_value(13), p.p32), 371);

        s.store_mul_ad_lhs(372, A::scale(s.ad_value(13), p.p32), 372);

        s.store_mul_ad_lhs(373, A::scale(s.ad_value(13), p.p32), 373);

        s.store_mul_ad_lhs(374, A::scale(s.ad_value(13), p.p32), 374);

        s.store_mul_ad_lhs(377, A::scale(s.ad_value(13), p.p32), 377);

        s.store_mul_ad_lhs(376, A::scale(s.ad_value(13), p.p32), 376);

        s.store_mul_ad_lhs(375, A::scale(s.ad_value(13), p.p32), 375);

        s.store_mul(378, 13, 378);

        s.v[1778] = if (s.v[334] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1778] != 0.0) {
            s.copy_ad(1772, 363);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(363, 364);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(364, 1772);
        }

        if (s.v[1778] != 0.0) {
            s.store_neg(375, 375);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(1772, 380);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(380, 379);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(379, 1772);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(1772, 382);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(382, 381);
        }

        if (s.v[1778] != 0.0) {
            s.copy_ad(381, 1772);
        }

        s.v[1779] = if (s.v[13] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1779] != 0.0) {
            s.store_mul_ad(1773, A::div(A::scale(s.ad_value(342), 1e-9), A::mul(s.ad_value(345), s.ad_value(116))), A::add(A::div(A::mul(s.ad_value(179), A::add(s.ad_value(1774), s.ad_value(1775))), A::mul(s.ad_value(116), s.ad_value(239))), A::mul(s.ad_value(180), s.ad_value(226))));
        }

        if (!(s.v[1779] != 0.0)) {
            s.store_scalar(1773, 0.0);
        }

        s.store_mul_ad_lhs(1780, A::scale(s.ad_value(390), 6.241509343260179e18), 226);

        s.store_scaled_add(1781, 407, 432, (-0.5));

        s.store_add(1782, 415, 1781);

        s.store_div(0, 415, 1782);

        s.store_scale_ad(1787, A::add(s.ad_value(0), A::sqrt(A::offset(A::mul(s.ad_value(0), s.ad_value(0)), 1e-20))), 0.5);

        s.store_mul_ad_lhs(1788, A::scale(s.ad_value(436), (-0.1666666666667)), 435);

        s.store_square(1789, 1788);

        s.store_offset(1790, 429, (-1.0));

        s.store_max_with_scalar_ad(1791, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1790), 12.0), s.ad_value(1789))), 1e-20);

        s.store_div_from_scalar_ad(1792, 1.0, A::square(s.ad_value(1791)));

        s.store_div_ad_lhs(1793, A::div(A::mul(A::mul(A::mul(A::mul(s.ad_value(342), s.ad_value(390)), s.ad_value(226)), s.ad_value(1782)), s.ad_value(344)), s.ad_value(345)), 346);

        s.store_scale(1794, 1789, 12.0);

        s.store_sub_ad(2, A::add(s.ad_value(1787), s.ad_value(1794)), A::mul(A::mul(A::scale(A::offset(s.ad_value(1787), 1.0), 2.0), s.ad_value(1794)), s.ad_value(1790)));

        s.store_ad(3, &A::max_with_scalar(s.ad_value(2), 1e-40));

        s.store_mul_ad_lhs(1795, A::mul(s.ad_value(1793), s.ad_value(1792)), 3);

        s.v[1812] = if (s.v[172] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1812] != 0.0) {
            s.store_div(1796, 427, 422);
        }

        if (s.v[1812] != 0.0) {
            s.store_div_ad(1797, A::mul(A::mul(A::mul(s.ad_value(309), s.ad_value(348)), s.ad_value(411)), s.ad_value(223)), A::mul(A::mul(A::offset(A::square(s.ad_value(1796)), 1.0), s.ad_value(1791)), s.ad_value(1791)));
        }

        if (s.v[1812] != 0.0) {
            s.store_add_ad_rhs(1795, 1795, A::div(s.ad_value(1797), s.ad_value(308)));
        }

        s.store_mul_ad_lhs(1798, A::mul(A::scale(s.ad_value(13), p.p31), s.ad_value(307)), 1795);

        s.store_div_ad_lhs(1799, A::mul(A::mul(s.ad_value(456), s.ad_value(447)), s.ad_value(116)), 469);

        s.store_mul_ad_lhs(1800, A::offset(s.ad_value(468), 1.0), 1799);

        s.store_mul_ad_rhs(1802, 1800, A::sub_from_scalar(0.5, A::mul(A::scale(s.ad_value(334), 0.25), s.ad_value(1788))));

        s.store_sub(1801, 1800, 1802);

        s.v[1805] = 0.0;

        s.v[1806] = 0.0;

        s.v[1813] = if (p.p6 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1813] != 0.0) {
            s.store_sub_ad(2, A::sub(A::scale(s.ad_value(1787), 0.08333333333333333), A::mul(s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 0.2), s.ad_value(1794)))), A::mul(A::mul(A::scale(s.ad_value(1789), 1.6), A::sub(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794))), s.ad_value(1790)));
        }

        if (s.v[1813] != 0.0) {
            s.store_ad(3, &A::max_with_scalar(s.ad_value(2), 1e-40));
        }

        if (s.v[1813] != 0.0) {
            s.store_div_ad_lhs(1803, A::mul(A::mul(s.ad_value(1793), s.ad_value(1791)), s.ad_value(1791)), 3);
        }

        if (s.v[1813] != 0.0) {
            s.store_mul_ad_lhs(1804, A::mul(A::scale(s.ad_value(13), p.p31), s.ad_value(307)), 1803);
        }

        s.v[1814] = if (s.v[1795] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1813] != 0.0) && (s.v[1814] != 0.0)) {
            s.store_mul_ad(1805, A::mul(s.ad_value(1792), s.ad_value(1788)), A::sub(A::sub_from_scalar(1.0, s.ad_value(1794)), A::mul(A::sub(A::add(s.ad_value(1787), A::scale(s.ad_value(1789), 19.2)), A::mul(s.ad_value(1787), s.ad_value(1794))), s.ad_value(1790))));
        }

        if ((s.v[1813] != 0.0) && (s.v[1814] != 0.0)) {
            s.store_div_ad_lhs(1806, A::mul(A::square(s.ad_value(1805)), s.ad_value(1803)), 1795);
        }

        if ((s.v[1813] != 0.0) && (s.v[1814] != 0.0)) {
            let assign43720_ad_e48945: A = A::sub(A::offset(A::scale(A::add(s.ad_value(1806), A::sqrt(A::offset(A::mul(s.ad_value(1806), s.ad_value(1806)), 1e-40))), 0.5), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::add(s.ad_value(1806), A::sqrt(A::offset(A::mul(s.ad_value(1806), s.ad_value(1806)), 1e-40))), 0.5), (-1.0)), A::offset(A::scale(A::add(s.ad_value(1806), A::sqrt(A::offset(A::mul(s.ad_value(1806), s.ad_value(1806)), 1e-40))), 0.5), (-1.0))), 1e-40)));
            s.store_scale_ad(1806, assign43720_ad_e48945, 0.5);
        }

        if (!(s.v[1813] != 0.0)) {
            s.store_scalar(1803, 1.0);
        }

        if (!(s.v[1813] != 0.0)) {
            s.store_scalar(1804, 0.0);
        }

        s.store_mul_ad_rhs(1807, 1798, A::sub_from_scalar(1.0, s.ad_value(1806)));

        s.copy_ad(1783, 1780);

        s.store_mul_ad_rhs(1784, 1780, A::offset(s.ad_value(415), 1.0));

        s.store_mul_ad_rhs(1785, 1780, A::sub(s.ad_value(403), s.ad_value(413)));

        s.store_mul_ad(2, A::add(A::sub(s.ad_value(173), A::mul(s.ad_value(174), s.ad_value(1783))), A::mul(A::mul(s.ad_value(175), s.ad_value(1783)), s.ad_value(1783))), A::ln(A::div(A::add(s.ad_value(1784), A::scale(s.ad_value(1785), 0.5)), A::sub(s.ad_value(1784), A::scale(s.ad_value(1785), 0.5)))));

        s.store_add_ad_rhs(3, 2, A::mul(A::add(s.ad_value(174), A::mul(s.ad_value(175), A::sub(s.ad_value(1784), A::scale(s.ad_value(1783), 2.0)))), s.ad_value(1785)));

        s.store_offset_ad(0, A::div(A::add(A::mul(s.ad_value(176), s.ad_value(417)), A::mul(s.ad_value(177), s.ad_value(418))), A::offset(s.ad_value(415), 1.0)), 1.0);

        s.store_scale_ad(4, A::add(A::offset(s.ad_value(0), 0.01), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(0), (-0.01)), A::offset(s.ad_value(0), (-0.01))), 0.0001))), 0.5);

        s.store_mul_ad_lhs(0, A::div(A::mul(A::div(A::mul(A::scale(s.ad_value(347), 1.602176565e-19), s.ad_value(348)), s.ad_value(345)), s.ad_value(3)), s.ad_value(1783)), 4);

        s.store_mul_ad(1786, A::scale(s.ad_value(13), p.p33), A::max_with_scalar(s.ad_value(0), 0.0));

        s.store_mul_ad(1808, A::scale(s.ad_value(13), ((2.0 * 1.602176565e-19) * p.p31)), A::abs(s.ad_value(352)));

        s.store_mul_ad(1809, A::scale(s.ad_value(13), ((2.0 * 1.602176565e-19) * p.p31)), A::abs(s.ad_value(353)));

        s.store_scale_ad(1811, A::mul(A::offset(s.ad_value(357), 1.0), A::abs(s.ad_value(358))), (2.0 * 1.602176565e-19));

        s.store_add_ad(1810, A::mul(A::scale(s.ad_value(13), ((2.0 * 1.602176565e-19) * p.p31)), A::abs(A::sub(s.ad_value(354), s.ad_value(355)))), A::mul(A::scale(s.ad_value(13), p.p31), s.ad_value(1811)));

        s.store_div_from_scalar_ad(1823, 1.0, A::scale(s.ad_value(8), 8.617332384961e-5));

        s.store_sub_from_scalar_ad(1824, 1.17, A::div(A::mul(A::scale(s.ad_value(8), 0.000473), s.ad_value(8)), A::offset(s.ad_value(8), 636.0)));

        s.store_sub_from_scalar_ad(1825, 0.744, A::div(A::mul(A::scale(s.ad_value(8), 0.0004774), s.ad_value(8)), A::offset(s.ad_value(8), 235.0)));

        s.store_mul_ad_lhs(1826, A::add(A::sub(s.ad_value(1825), s.ad_value(1824)), A::scale(s.ad_value(228), (-0.4))), 15);

        s.store_add(1827, 1824, 1826);

        s.store_mul_ad_lhs(1828, A::scale(s.ad_value(1827), 0.5), 1823);

        s.store_sub_ad(1829, A::scale(s.ad_value(15), 0.05), A::scale(s.ad_value(1826), 0.5));

        s.store_sqrt_ad(0, A::scale(s.ad_value(8), 0.0033333333333));

        s.store_mul_ad_lhs(2, A::mul(A::scale(s.ad_value(0), 4.05e25), s.ad_value(0)), 0);

        s.store_mul(1830, 2, 238);

        s.store_div_ad_rhs(1831, 1823, A::offset(A::div(A::scale(s.ad_value(17), s.v[7]), s.ad_value(8)), 1.0));

        s.store_mul_ad_lhs(1833, A::mul(A::scale(s.ad_value(1830), (2.0 * 1.602176565e-19)), s.ad_value(229)), 1831);

        s.store_add_ad_lhs(1834, A::offset(A::ln(A::div(A::square(s.ad_value(245)), s.ad_value(1833))), (-0.6931471805599)), 1828);

        s.store_mul_ad_lhs(1835, A::div(A::mul(A::scale(s.ad_value(29), (0.5 * 1.602176565e-19)), s.ad_value(14)), A::add(s.ad_value(241), s.ad_value(242))), 1831);

        s.store_mul(1838, 35, 1831);

        s.v[1839] = 0.0;

        s.v[1832] = 0.0;

    }

    pub(super) fn stamp_transient_block_30(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1884] = if (p.p9 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1884] != 0.0) {
            s.store_mul_ad(1832, A::div_from_scalar(1.0, s.ad_value(1823)), A::ln(A::div(s.ad_value(24), s.ad_value(251))));
        }

        s.v[1885] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        s.v[1886] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1885] != 0.0) && (s.v[1886] != 0.0)) {
            s.store_scale_ad(1839, A::exp(A::scale(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333))), ((0.4 * p.p13) * 1.27520989));
        }

        if ((s.v[1885] != 0.0) && (!(s.v[1886] != 0.0))) {
            s.store_scale_ad(1839, A::exp(A::scale(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333))), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_mul(1842, 336, 1831);

        s.store_mul_ad_lhs(1843, A::offset(A::sqrt(A::offset(A::square(s.ad_value(336)), 0.01)), (-0.1)), 1831);

        s.store_scaled_sub(1844, 1842, 1843, 0.5);

        s.store_div_ad(1815, A::div(s.ad_value(402), s.ad_value(401)), A::offset(s.ad_value(402), 1.0));

        s.store_div_ad(1816, A::div(s.ad_value(401), s.ad_value(402)), A::offset(s.ad_value(401), 1.0));

        s.store_offset_ad(1817, A::ln(A::div(A::mul(A::mul(s.ad_value(401), A::offset(s.ad_value(1815), 1.0)), s.ad_value(384)), s.ad_value(385))), 2.0);

        s.store_offset_ad(1818, A::ln(A::div(A::mul(A::mul(s.ad_value(402), A::offset(s.ad_value(1816), 1.0)), s.ad_value(384)), s.ad_value(385))), 2.0);

        s.store_sub_ad(1819, A::mul(A::offset(s.ad_value(1815), 1.0), s.ad_value(1817)), A::mul(s.ad_value(399), s.ad_value(1815)));

        s.store_sub_ad(1820, A::mul(A::offset(A::div_from_scalar(1.0, s.ad_value(1816)), 1.0), s.ad_value(1818)), A::div(s.ad_value(399), s.ad_value(1816)));

        s.store_add_ad_lhs(1821, A::div(A::sub(A::scale(A::sub(A::add(s.ad_value(1819), s.ad_value(1820)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1819), s.ad_value(1820)), A::sub(s.ad_value(1819), s.ad_value(1820))), 38.0))), 0.5), s.ad_value(398)), s.ad_value(25)), 398);

        s.store_add_ad_lhs(1822, A::mul(s.ad_value(226), A::add(A::sub(A::div(A::sub(s.ad_value(1821), s.ad_value(394)), s.ad_value(395)), s.ad_value(397)), s.ad_value(394))), 21);

        s.store_mul_ad_rhs(0, 34, A::offset(s.ad_value(8), (-s.v[7])));

        s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(23), p.p14), A::offset(s.ad_value(8), (-s.v[7]))), 256);

        s.store_sub_ad_lhs(1840, A::offset(A::add(A::scale(A::add(A::add(s.ad_value(183), s.ad_value(1829)), s.ad_value(243)), p.p14), s.ad_value(0)), p.p34), 1832);

        s.store_add_ad_lhs(1841, A::scale(A::add(A::add(s.ad_value(184), s.ad_value(1829)), s.ad_value(244)), p.p14), 0);

        s.store_sub_ad_lhs(1845, A::mul(A::sub(s.ad_value(1822), s.ad_value(1840)), s.ad_value(1831)), 1844);

        s.store_sub_ad_lhs(1846, A::mul(A::sub(A::neg(s.ad_value(337)), s.ad_value(1841)), s.ad_value(1831)), 1844);

        s.v[1887] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1887] != 0.0) {
            s.store_div_ad_lhs(0, A::mul(A::scale(s.ad_value(16), p.p14), A::sub(s.ad_value(1845), s.ad_value(1846))), 260);
        }

        s.v[1888] = if (s.v[0] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1887] != 0.0) && (s.v[1888] != 0.0)) {
            s.store_scale_ad(2, A::ln(A::sub_from_scalar(1.0, s.ad_value(0))), (-2.0));
        }

        if ((s.v[1887] != 0.0) && (!(s.v[1888] != 0.0))) {
            s.store_div_ad(2, A::square(s.ad_value(0)), A::offset(A::div(A::scale(s.ad_value(0), 2.0), s.ad_value(260)), 1.0));
        }

        if (s.v[1887] != 0.0) {
            s.store_add_ad_rhs(1847, 1846, A::mul(A::scale(s.ad_value(16), p.p14), s.ad_value(2)));
        }

        if (!(s.v[1887] != 0.0)) {
            s.copy_ad(1847, 1846);
        }

        s.store_mul_ad_rhs(0, 248, A::sub(s.ad_value(1845), s.ad_value(1847)));

        s.v[1889] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1889] != 0.0) {
            s.store_scale_ad(1848, A::add(A::add(s.ad_value(0), s.ad_value(257)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if (s.v[1889] != 0.0) {
            s.store_scale_ad(1849, A::add(A::sub(s.ad_value(257), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(257)), A::sub(A::neg(s.ad_value(0)), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if (s.v[1889] != 0.0) {
            s.store_mul_ad_rhs(2, 1839, A::exp(A::scale(A::ln(s.ad_value(1848)), (-0.3333333333333))));
        }

        if (s.v[1889] != 0.0) {
            s.store_mul_ad_rhs(3, 1839, A::exp(A::scale(A::ln(s.ad_value(1849)), (-0.3333333333333))));
        }

        if (s.v[1889] != 0.0) {
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if (s.v[1889] != 0.0) {
            s.store_div_ad(1851, A::mul(s.ad_value(246), s.ad_value(4)), A::offset(A::mul(s.ad_value(246), s.ad_value(2)), 1.0));
        }

        if (s.v[1889] != 0.0) {
            s.store_div_ad(1852, A::mul(s.ad_value(247), s.ad_value(4)), A::offset(A::mul(s.ad_value(247), s.ad_value(3)), 1.0));
        }

        if (s.v[1889] != 0.0) {
            s.store_div_from_scalar_ad(1853, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852))));
        }

        if (!(s.v[1889] != 0.0)) {
            s.copy_ad(1851, 246);
        }

        if (!(s.v[1889] != 0.0)) {
            s.copy_ad(1852, 247);
        }

        if (!(s.v[1889] != 0.0)) {
            s.copy_ad(1853, 248);
        }

        s.store_mul_ad_rhs(1854, 1853, A::sub(s.ad_value(1845), s.ad_value(1847)));

        s.v[1890] = if (s.v[1854] > 0.0) { 1.0 } else { 0.0 };

        s.v[1891] = if ((-s.v[1854]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1890] != 0.0) && (s.v[1891] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(A::neg(s.ad_value(1854))), 1.0));
        }

        if ((s.v[1890] != 0.0) && (!(s.v[1891] != 0.0))) {
            s.store_neg(0, 1854);
        }

        if (s.v[1890] != 0.0) {
            s.store_offset_ad(1855, A::add(A::sub(s.ad_value(1845), A::div(s.ad_value(1854), s.ad_value(1851))), s.ad_value(0)), (-0.6931471805599));
        }

        s.v[1892] = if (s.v[1854] < 80.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1890] != 0.0)) && (s.v[1892] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(s.ad_value(1854)), 1.0));
        }

        if ((!(s.v[1890] != 0.0)) && (!(s.v[1892] != 0.0))) {
            s.copy_ad(0, 1854);
        }

        if (!(s.v[1890] != 0.0)) {
            s.store_offset_ad(1855, A::add(A::add(s.ad_value(1847), A::div(s.ad_value(1854), s.ad_value(1852))), s.ad_value(0)), (-0.6931471805599));
        }

        s.store_scale_ad(1856, A::sub(A::add(s.ad_value(1855), s.ad_value(1834)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1855), s.ad_value(1834)), A::sub(s.ad_value(1855), s.ad_value(1834))), 4.0))), 0.5);

        s.store_offset_ad(1857, A::sqrt(A::offset(A::div(A::scale(A::sub(s.ad_value(1834), s.ad_value(1856)), 2.0), s.ad_value(1835)), 1.0)), (-1.0));

        s.store_scale_ad(0, A::add(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), 1.0), (-0.5))), 0.01))), 0.5);

        s.store_mul_ad(0, A::mul(A::mul(A::scale(s.ad_value(1838), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1846)), 1.0));

        s.v[1894] = if (p.p11 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1894] != 0.0) {
            s.store_div_ad(1815, A::div(s.ad_value(457), s.ad_value(456)), A::offset(s.ad_value(457), 1.0));
        }

        if (s.v[1894] != 0.0) {
            s.store_div_ad(1816, A::div(s.ad_value(456), s.ad_value(457)), A::offset(s.ad_value(456), 1.0));
        }

        if (s.v[1894] != 0.0) {
            s.store_offset_ad(1817, A::ln(A::div(A::mul(A::mul(s.ad_value(456), A::offset(s.ad_value(1815), 1.0)), s.ad_value(441)), s.ad_value(442))), 2.0);
        }

        if (s.v[1894] != 0.0) {
            s.store_offset_ad(1818, A::ln(A::div(A::mul(A::mul(s.ad_value(457), A::offset(s.ad_value(1816), 1.0)), s.ad_value(441)), s.ad_value(442))), 2.0);
        }

        if (s.v[1894] != 0.0) {
            s.store_sub_ad(1819, A::mul(A::offset(s.ad_value(1815), 1.0), s.ad_value(1817)), A::mul(s.ad_value(455), s.ad_value(1815)));
        }

        if (s.v[1894] != 0.0) {
            s.store_sub_ad(1820, A::mul(A::offset(A::div_from_scalar(1.0, s.ad_value(1816)), 1.0), s.ad_value(1818)), A::div(s.ad_value(455), s.ad_value(1816)));
        }

        if (s.v[1894] != 0.0) {
            s.store_add_ad_lhs(1821, A::div(A::sub(A::scale(A::sub(A::add(s.ad_value(1819), s.ad_value(1820)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1819), s.ad_value(1820)), A::sub(s.ad_value(1819), s.ad_value(1820))), 38.0))), 0.5), s.ad_value(454)), s.ad_value(25)), 454);
        }

        if (s.v[1894] != 0.0) {
            s.store_add_ad_lhs(1822, A::mul(s.ad_value(226), A::add(A::sub(A::div(A::sub(s.ad_value(1821), s.ad_value(450)), s.ad_value(451)), s.ad_value(453)), s.ad_value(450))), 130);
        }

        if (s.v[1894] != 0.0) {
            s.store_mul_ad_rhs(0, 34, A::offset(s.ad_value(8), (-s.v[7])));
        }

        if (s.v[1894] != 0.0) {
            s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(23), p.p14), A::offset(s.ad_value(8), (-s.v[7]))), 256);
        }

        if (s.v[1894] != 0.0) {
            s.store_sub_ad_lhs(1840, A::offset(A::add(A::scale(A::add(A::add(s.ad_value(185), s.ad_value(1829)), s.ad_value(243)), p.p14), s.ad_value(0)), p.p34), 1832);
        }

        if (s.v[1894] != 0.0) {
            s.store_add_ad_lhs(1841, A::scale(A::add(A::add(s.ad_value(186), s.ad_value(1829)), s.ad_value(244)), p.p14), 0);
        }

        if (s.v[1894] != 0.0) {
            s.store_sub_ad_lhs(1845, A::mul(A::sub(s.ad_value(1822), s.ad_value(1840)), s.ad_value(1831)), 1844);
        }

        if (s.v[1894] != 0.0) {
            s.store_sub_ad_lhs(1846, A::mul(A::sub(A::neg(s.ad_value(337)), s.ad_value(1841)), s.ad_value(1831)), 1844);
        }

        s.v[1895] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1894] != 0.0) && (s.v[1895] != 0.0)) {
            s.store_div_ad_lhs(0, A::mul(A::scale(s.ad_value(16), p.p14), A::sub(s.ad_value(1845), s.ad_value(1846))), 260);
        }

        s.v[1896] = if (s.v[0] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1894] != 0.0) && (s.v[1895] != 0.0)) && (s.v[1896] != 0.0)) {
            s.store_scale_ad(2, A::ln(A::sub_from_scalar(1.0, s.ad_value(0))), (-2.0));
        }

        if (((s.v[1894] != 0.0) && (s.v[1895] != 0.0)) && (!(s.v[1896] != 0.0))) {
            s.store_div_ad(2, A::square(s.ad_value(0)), A::offset(A::div(A::scale(s.ad_value(0), 2.0), s.ad_value(260)), 1.0));
        }

        if ((s.v[1894] != 0.0) && (s.v[1895] != 0.0)) {
            s.store_add_ad_rhs(1847, 1846, A::mul(A::scale(s.ad_value(16), p.p14), s.ad_value(2)));
        }

        if ((s.v[1894] != 0.0) && (!(s.v[1895] != 0.0))) {
            s.copy_ad(1847, 1846);
        }

        if (s.v[1894] != 0.0) {
            s.store_mul_ad_rhs(0, 248, A::sub(s.ad_value(1845), s.ad_value(1847)));
        }

        s.v[1897] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_scale_ad(1848, A::add(A::add(s.ad_value(0), s.ad_value(257)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_scale_ad(1849, A::add(A::sub(s.ad_value(257), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(257)), A::sub(A::neg(s.ad_value(0)), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_mul_ad_rhs(2, 1839, A::exp(A::scale(A::ln(s.ad_value(1848)), (-0.3333333333333))));
        }

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_mul_ad_rhs(3, 1839, A::exp(A::scale(A::ln(s.ad_value(1849)), (-0.3333333333333))));
        }

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_div_ad(1851, A::mul(s.ad_value(246), s.ad_value(4)), A::offset(A::mul(s.ad_value(246), s.ad_value(2)), 1.0));
        }

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_div_ad(1852, A::mul(s.ad_value(247), s.ad_value(4)), A::offset(A::mul(s.ad_value(247), s.ad_value(3)), 1.0));
        }

        if ((s.v[1894] != 0.0) && (s.v[1897] != 0.0)) {
            s.store_div_from_scalar_ad(1853, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852))));
        }

        if ((s.v[1894] != 0.0) && (!(s.v[1897] != 0.0))) {
            s.copy_ad(1851, 246);
        }

        if ((s.v[1894] != 0.0) && (!(s.v[1897] != 0.0))) {
            s.copy_ad(1852, 247);
        }

        if ((s.v[1894] != 0.0) && (!(s.v[1897] != 0.0))) {
            s.copy_ad(1853, 248);
        }

        if (s.v[1894] != 0.0) {
            s.store_mul_ad_rhs(1854, 1853, A::sub(s.ad_value(1845), s.ad_value(1847)));
        }

        s.v[1898] = if (s.v[1854] > 0.0) { 1.0 } else { 0.0 };

        s.v[1899] = if ((-s.v[1854]) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1894] != 0.0) && (s.v[1898] != 0.0)) && (s.v[1899] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(A::neg(s.ad_value(1854))), 1.0));
        }

        if (((s.v[1894] != 0.0) && (s.v[1898] != 0.0)) && (!(s.v[1899] != 0.0))) {
            s.store_neg(0, 1854);
        }

        if ((s.v[1894] != 0.0) && (s.v[1898] != 0.0)) {
            s.store_offset_ad(1855, A::add(A::sub(s.ad_value(1845), A::div(s.ad_value(1854), s.ad_value(1851))), s.ad_value(0)), (-0.6931471805599));
        }

        s.v[1900] = if (s.v[1854] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1894] != 0.0) && (!(s.v[1898] != 0.0))) && (s.v[1900] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(s.ad_value(1854)), 1.0));
        }

        if (((s.v[1894] != 0.0) && (!(s.v[1898] != 0.0))) && (!(s.v[1900] != 0.0))) {
            s.copy_ad(0, 1854);
        }

        if ((s.v[1894] != 0.0) && (!(s.v[1898] != 0.0))) {
            s.store_offset_ad(1855, A::add(A::add(s.ad_value(1847), A::div(s.ad_value(1854), s.ad_value(1852))), s.ad_value(0)), (-0.6931471805599));
        }

        if (s.v[1894] != 0.0) {
            s.store_scale_ad(1856, A::sub(A::add(s.ad_value(1855), s.ad_value(1834)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1855), s.ad_value(1834)), A::sub(s.ad_value(1855), s.ad_value(1834))), 4.0))), 0.5);
        }

        if (s.v[1894] != 0.0) {
            s.store_offset_ad(1857, A::sqrt(A::offset(A::div(A::scale(A::sub(s.ad_value(1834), s.ad_value(1856)), 2.0), s.ad_value(1835)), 1.0)), (-1.0));
        }

        if (s.v[1894] != 0.0) {
            s.store_scale_ad(0, A::add(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), 1.0), (-0.5))), 0.01))), 0.5);
        }

        if (s.v[1894] != 0.0) {
            s.store_mul_ad(0, A::mul(A::mul(A::scale(s.ad_value(1838), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1846)), 1.0));
        }

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
        s.v[7] = (273.15 + p.p15);

        s.v[0] = ((ctx.temperature() + p.p36)).min(1000.0);

        s.v[529] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));
        }

        if (s.v[529] != 0.0) {
            s.store_scale_ad(225, A::add(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0)), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01))), 0.5);
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scalar(225, 600.0);
        }

        s.v[530] = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0))) { 1.0 } else { 0.0 };

        if (s.v[530] != 0.0) {
            s.store_scalar(6, p.p5);
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scalar(6, 0.0);
        }

        s.v[219] = 0.0;

        s.copy_ad(217, 8);

        s.store_square(218, 217);

        s.store_offset(220, 217, (-s.v[7]));

        s.store_scale(221, 217, 1.0 / (s.v[7]));

        s.store_div_from_scalar(222, s.v[7], 217);

        s.store_scale(223, 217, 8.617332384961e-5);

        s.store_div_from_scalar(224, 1.0, 223);

        s.v[611] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[611] != 0.0) {
            s.store_scalar(10, p.p23);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(9, p.p22);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(12, p.p25);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(11, p.p24);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(13, p.p30);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(533, p.p41);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(14, p.p42);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(15, p.p43);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(534, p.p44);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(535, 1.0);
        }

        s.v[612] = if (p.p45 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[611] != 0.0) && (s.v[612] != 0.0)) {
            s.store_scalar(535, (-1.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(536, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(16, 1.0);
        }

        s.v[613] = if (p.p46 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[611] != 0.0) && (s.v[613] != 0.0)) {
            s.store_scalar(16, (-1.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(537, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(17, p.p47);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(18, p.p48);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(19, (p.p49 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(20, (p.p50 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(183, p.p51);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(184, p.p52);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(23, p.p53);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(24, (p.p54 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(25, p.p55);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(26, p.p56);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(27, p.p57);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(28, A::mul(A::scale(s.ad_value(27), p.p58), s.ad_value(534)), 533);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(29, (p.p59 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(30, p.p60);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(538, p.p61);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(187, p.p62);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(188, A::mul(A::scale(s.ad_value(187), p.p63), s.ad_value(534)), 533);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(34, p.p64);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(35, p.p65);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(36, p.p66);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(37, p.p67);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(191, p.p68);
        }

        if (s.v[611] != 0.0) {
            s.store_scale(192, 191, p.p69);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(40, p.p70);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(195, p.p71);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(41, p.p72);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(42, p.p73);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(43, p.p74);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(196, p.p75);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(45, p.p76);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(539, p.p77);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(540, p.p78);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(193, p.p79);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(48, p.p80);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(194, p.p81);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(49, p.p82);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(197, p.p83);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(51, p.p84);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(52, p.p85);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(541, p.p86);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(198, p.p87);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(54, p.p88);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(55, p.p89);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(56, p.p90);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(57, p.p91);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(58, p.p92);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(199, p.p93);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(60, p.p94);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(61, p.p95);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(62, p.p96);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(542, p.p97);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(63, p.p98);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(64, p.p99);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(65, p.p100);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(66, p.p101);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(67, p.p102);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(75, p.p103);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(201, p.p104);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(202, p.p105);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(203, p.p106);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(204, p.p107);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(205, p.p108);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(76, p.p109);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(77, p.p123);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(78, p.p110);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(79, p.p111);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(80, p.p112);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(81, p.p122);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(82, p.p113);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(83, p.p114);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(84, p.p115);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(85, p.p116);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(86, p.p117);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(87, p.p118);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(88, p.p119);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(89, p.p124);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(90, p.p125);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(208, p.p126);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(209, p.p127);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(93, p.p128);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(94, p.p129);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(95, p.p130);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(96, p.p131);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(97, p.p132);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(98, p.p133);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(210, p.p148);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(114, p.p149);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(115, p.p150);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(99, p.p134);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(211, p.p135);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(212, p.p136);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(102, p.p137);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(103, p.p138);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(104, p.p139);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(105, p.p140);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(106, A::mul(A::scale(s.ad_value(105), p.p141), s.ad_value(534)), 533);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(107, p.p142);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(108, A::mul(A::scale(s.ad_value(107), p.p143), s.ad_value(534)), 533);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(109, p.p144);
        }

    }
}
