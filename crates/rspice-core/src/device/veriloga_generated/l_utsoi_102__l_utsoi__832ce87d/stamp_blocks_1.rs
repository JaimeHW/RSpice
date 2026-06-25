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
        if (((s.v[1218] != 0.0) && (!(s.v[1219] != 0.0))) && (!(s.v[1220] != 0.0))) {
            s.store_add(712, 696, 697);
        }

        if ((s.v[1218] != 0.0) && (!(s.v[1219] != 0.0))) {
            s.store_neg(712, 712);
        }

        s.store_div_ad_lhs(704, A::sqrt(A::mul(A::mul(A::scale(s.ad_value(20), (2.0 * 1.602176565e-19)), s.ad_value(225)), s.ad_value(220))), 237);

        s.store_square(705, 704);

        s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);

        s.store_scale(707, 706, 1e-5);

        s.store_div_from_scalar(708, 1.0, 706);

        s.store_div_from_scalar_ad(709, 1.0, A::offset(A::scale(s.ad_value(704), 0.7324648775608221), 1.25));

        s.v[1227] = if (((p.p3 > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p.p4 > 0.0) && (s.v[90] > 0.0))) { 1.0 } else { 0.0 };

        s.v[1228] = if (((s.v[701]) as f64).abs() <= s.v[707]) { 1.0 } else { 0.0 };

        if ((s.v[1227] != 0.0) && (s.v[1228] != 0.0)) {
            s.store_mul_ad_lhs(711, A::neg(s.ad_value(701)), 708);
        }

        s.v[1229] = if (s.v[701] < (-s.v[707])) { 1.0 } else { 0.0 };

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_neg(679, 701);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_mul_ad_lhs(680, A::scale(s.ad_value(679), 1.25), 708);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_scale_ad(681, A::sub(A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(680), (-6.0)), A::offset(s.ad_value(680), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_add_ad(682, A::mul(A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681))), A::mul(s.ad_value(705), A::offset(s.ad_value(681), 1.0)));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_sub_ad_lhs(683, A::scale(A::sub(s.ad_value(679), s.ad_value(681)), 2.0), 705);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_add(685, 682, 683);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_add_ad(686, A::square(s.ad_value(685)), A::mul(s.ad_value(684), A::sub(A::mul(A::scale(s.ad_value(683), 0.5), s.ad_value(683)), s.ad_value(682))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_add_ad_rhs(687, 686, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684)), s.ad_value(684)), s.ad_value(683)), A::sub(A::scale(A::square(s.ad_value(683)), 0.3333333333333), s.ad_value(682))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_add_ad_rhs(688, 681, A::div(A::mul(A::mul(s.ad_value(682), s.ad_value(685)), s.ad_value(684)), s.ad_value(687)));
        }

        s.v[1230] = if (((s.v[688]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) && (s.v[1230] != 0.0)) {
            s.store_exp(689, 688);
        }

        s.v[1231] = if (s.v[688] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) && (!(s.v[1230] != 0.0))) && (s.v[1231] != 0.0)) {
            s.store_div_from_scalar_ad(689, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(688)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_scale_ad(689, A::offset(A::mul(A::offset(s.ad_value(688), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(688), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(688), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_sub(687, 679, 688);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_add_ad(690, A::scale(s.ad_value(687), 2.0), A::mul(s.ad_value(705), A::offset(s.ad_value(689), (-1.0))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_add_ad(691, A::square(s.ad_value(687)), A::mul(s.ad_value(705), A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_sub_from_scalar_ad(692, 1.0, A::mul(A::scale(s.ad_value(705), 0.5), s.ad_value(689)));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::scale(A::mul(s.ad_value(692), s.ad_value(691)), 4.0));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_div_ad(693, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_neg_ad(711, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_mul_ad_lhs(694, A::offset(A::mul(A::scale(s.ad_value(706), 1.25), s.ad_value(709)), (-1.0)), 709);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_mul_ad(695, A::mul(s.ad_value(701), s.ad_value(708)), A::offset(A::mul(s.ad_value(694), s.ad_value(701)), 1.0));
        }

        s.v[1232] = if ((((-s.v[695])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) && (s.v[1232] != 0.0)) {
            s.store_exp_ad(687, A::neg(s.ad_value(695)));
        }

        s.v[1233] = if ((-s.v[695]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) && (!(s.v[1232] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_div_from_scalar_ad(687, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) && (!(s.v[1232] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_scale_ad(687, A::offset(A::mul(A::offset(A::neg(s.ad_value(695)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_sub_from_scalar(693, 1.0, 687);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_sub_ad(696, A::add(s.ad_value(701), A::scale(s.ad_value(705), 0.5)), A::mul(s.ad_value(704), A::sqrt(A::sub(A::add(s.ad_value(701), A::scale(s.ad_value(705), 0.25)), s.ad_value(693)))));
        }

        s.v[1234] = if ((((-s.v[696])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) && (s.v[1234] != 0.0)) {
            s.store_exp_ad(689, A::neg(s.ad_value(696)));
        }

        s.v[1235] = if ((-s.v[696]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) && (!(s.v[1234] != 0.0))) && (s.v[1235] != 0.0)) {
            s.store_div_from_scalar_ad(689, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1235] != 0.0))) {
            s.store_scale_ad(689, A::offset(A::mul(A::offset(A::neg(s.ad_value(696)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_add_ad(690, A::scale(A::sub(s.ad_value(701), s.ad_value(696)), 2.0), A::mul(s.ad_value(705), A::sub_from_scalar(1.0, s.ad_value(689))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_sub_ad(691, A::mul(A::sub(s.ad_value(701), s.ad_value(696)), A::sub(s.ad_value(701), s.ad_value(696))), A::mul(s.ad_value(705), A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_sub_from_scalar_ad(692, 1.0, A::mul(A::scale(s.ad_value(705), 0.5), s.ad_value(689)));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::scale(A::mul(s.ad_value(692), s.ad_value(691)), 4.0));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_div_ad(697, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
        }

        if (((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_add(711, 696, 697);
        }

        if ((s.v[1227] != 0.0) && (!(s.v[1228] != 0.0))) {
            s.store_neg(711, 711);
        }

        s.v[1236] = if (s.v[160] > 0.0) { 1.0 } else { 0.0 };

        s.v[1237] = if (((s.v[703]) as f64).abs() <= s.v[707]) { 1.0 } else { 0.0 };

        if ((s.v[1236] != 0.0) && (s.v[1237] != 0.0)) {
            s.store_mul_ad_lhs(713, A::neg(s.ad_value(703)), 708);
        }

        s.v[1238] = if (s.v[703] < (-s.v[707])) { 1.0 } else { 0.0 };

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_neg(679, 703);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_mul_ad_lhs(680, A::scale(s.ad_value(679), 1.25), 708);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_scale_ad(681, A::sub(A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(680), (-6.0)), A::offset(s.ad_value(680), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_add_ad(682, A::mul(A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681))), A::mul(s.ad_value(705), A::offset(s.ad_value(681), 1.0)));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_sub_ad_lhs(683, A::scale(A::sub(s.ad_value(679), s.ad_value(681)), 2.0), 705);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_add(685, 682, 683);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_add_ad(686, A::square(s.ad_value(685)), A::mul(s.ad_value(684), A::sub(A::mul(A::scale(s.ad_value(683), 0.5), s.ad_value(683)), s.ad_value(682))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_add_ad_rhs(687, 686, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684)), s.ad_value(684)), s.ad_value(683)), A::sub(A::scale(A::square(s.ad_value(683)), 0.3333333333333), s.ad_value(682))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_add_ad_rhs(688, 681, A::div(A::mul(A::mul(s.ad_value(682), s.ad_value(685)), s.ad_value(684)), s.ad_value(687)));
        }

        s.v[1239] = if (((s.v[688]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) && (s.v[1239] != 0.0)) {
            s.store_exp(689, 688);
        }

        s.v[1240] = if (s.v[688] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) && (!(s.v[1239] != 0.0))) && (s.v[1240] != 0.0)) {
            s.store_div_from_scalar_ad(689, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(688)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) && (!(s.v[1239] != 0.0))) && (!(s.v[1240] != 0.0))) {
            s.store_scale_ad(689, A::offset(A::mul(A::offset(s.ad_value(688), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(688), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(688), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_sub(687, 679, 688);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_add_ad(690, A::scale(s.ad_value(687), 2.0), A::mul(s.ad_value(705), A::offset(s.ad_value(689), (-1.0))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_add_ad(691, A::square(s.ad_value(687)), A::mul(s.ad_value(705), A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_sub_from_scalar_ad(692, 1.0, A::mul(A::scale(s.ad_value(705), 0.5), s.ad_value(689)));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::scale(A::mul(s.ad_value(692), s.ad_value(691)), 4.0));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_div_ad(693, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_neg_ad(713, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_mul_ad_lhs(694, A::offset(A::mul(A::scale(s.ad_value(706), 1.25), s.ad_value(709)), (-1.0)), 709);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_mul_ad(695, A::mul(s.ad_value(703), s.ad_value(708)), A::offset(A::mul(s.ad_value(694), s.ad_value(703)), 1.0));
        }

        s.v[1241] = if ((((-s.v[695])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) && (s.v[1241] != 0.0)) {
            s.store_exp_ad(687, A::neg(s.ad_value(695)));
        }

        s.v[1242] = if ((-s.v[695]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) && (!(s.v[1241] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_div_from_scalar_ad(687, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) && (!(s.v[1241] != 0.0))) && (!(s.v[1242] != 0.0))) {
            s.store_scale_ad(687, A::offset(A::mul(A::offset(A::neg(s.ad_value(695)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_sub_from_scalar(693, 1.0, 687);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_sub_ad(696, A::add(s.ad_value(703), A::scale(s.ad_value(705), 0.5)), A::mul(s.ad_value(704), A::sqrt(A::sub(A::add(s.ad_value(703), A::scale(s.ad_value(705), 0.25)), s.ad_value(693)))));
        }

        s.v[1243] = if ((((-s.v[696])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) && (s.v[1243] != 0.0)) {
            s.store_exp_ad(689, A::neg(s.ad_value(696)));
        }

        s.v[1244] = if ((-s.v[696]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) && (!(s.v[1243] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_div_from_scalar_ad(689, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scale_ad(689, A::offset(A::mul(A::offset(A::neg(s.ad_value(696)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_add_ad(690, A::scale(A::sub(s.ad_value(703), s.ad_value(696)), 2.0), A::mul(s.ad_value(705), A::sub_from_scalar(1.0, s.ad_value(689))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_sub_ad(691, A::mul(A::sub(s.ad_value(703), s.ad_value(696)), A::sub(s.ad_value(703), s.ad_value(696))), A::mul(s.ad_value(705), A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_sub_from_scalar_ad(692, 1.0, A::mul(A::scale(s.ad_value(705), 0.5), s.ad_value(689)));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::scale(A::mul(s.ad_value(692), s.ad_value(691)), 4.0));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_div_ad(697, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
        }

        if (((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_add(713, 696, 697);
        }

        if ((s.v[1236] != 0.0) && (!(s.v[1237] != 0.0))) {
            s.store_neg(713, 713);
        }

        s.store_mul_ad(714, A::neg(s.ad_value(219)), A::add(s.ad_value(700), s.ad_value(710)));

        s.store_mul_ad(715, A::neg(s.ad_value(219)), A::add(s.ad_value(701), s.ad_value(711)));

        s.store_mul_ad(345, A::neg(s.ad_value(219)), A::add(s.ad_value(702), s.ad_value(712)));

        s.store_mul_ad(346, A::neg(s.ad_value(219)), A::add(s.ad_value(703), s.ad_value(713)));

        s.v[729] = 0.0;

        s.v[730] = 0.0;

        s.v[347] = 0.0;

        s.v[348] = 0.0;

        s.v[349] = 0.0;

        s.v[749] = 0.0;

        s.v[750] = 0.0;

        s.v[1245] = if (p.p3 > 0.0) { 1.0 } else { 0.0 };

        s.v[1246] = if ((s.v[69] > 0.0) || (s.v[71] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add(716, 714, 281);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_scale_ad(717, A::sub(s.ad_value(716), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(716)), A::neg(s.ad_value(716))), 0.01))), 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_mul_ad_lhs(718, A::sqrt(A::offset(A::square(s.ad_value(714)), 0.0001)), 272);
        }

        s.v[1247] = if ((((0.5 * s.v[700])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1247] != 0.0)) {
            s.store_exp_ad(0, A::scale(s.ad_value(700), 0.5));
        }

        s.v[1248] = if ((0.5 * s.v[700]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1247] != 0.0))) && (s.v[1248] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(A::scale(s.ad_value(700), 0.5), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::scale(s.ad_value(700), 0.5), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::scale(s.ad_value(700), 0.5), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_div_from_scalar_ad(2, 1.0, A::offset(s.ad_value(0), 1.0));
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_sub_from_scalar(3, 1.0, 2);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add_ad(719, A::mul(s.ad_value(83), s.ad_value(2)), A::mul(s.ad_value(80), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add_ad(720, A::mul(s.ad_value(84), s.ad_value(2)), A::mul(s.ad_value(82), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add_ad(721, A::mul(s.ad_value(278), s.ad_value(2)), A::mul(s.ad_value(277), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add_ad(722, A::mul(s.ad_value(71), s.ad_value(2)), A::mul(s.ad_value(69), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_scaled_mul(723, 73, 3, 1e-6);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_mul_ad_rhs(2, 275, A::div(A::scale(s.ad_value(81), (-1.0)), s.ad_value(718)));
        }

        s.v[1249] = if (s.v[720] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1249] != 0.0)) {
            s.store_scale_ad(718, A::sub(A::add(s.ad_value(718), s.ad_value(721)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(721)), A::sub(s.ad_value(718), s.ad_value(721))), 1e-6))), 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add_ad(724, A::offset(s.ad_value(710), 3.0), A::mul(s.ad_value(717), s.ad_value(220)));
        }

        s.v[1250] = if (((s.v[724]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1250] != 0.0)) {
            s.store_exp(725, 724);
        }

        s.v[1251] = if (s.v[724] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1250] != 0.0))) && (s.v[1251] != 0.0)) {
            s.store_div_from_scalar_ad(725, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1250] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_scale_ad(725, A::offset(A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add_ad_lhs(724, A::add(A::offset(s.ad_value(710), 3.0), A::mul(s.ad_value(717), s.ad_value(220))), 700);
        }

        s.v[1252] = if (((s.v[724]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1252] != 0.0)) {
            s.store_exp(726, 724);
        }

        s.v[1253] = if (s.v[724] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1252] != 0.0))) && (s.v[1253] != 0.0)) {
            s.store_div_from_scalar_ad(726, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1252] != 0.0))) && (!(s.v[1253] != 0.0))) {
            s.store_scale_ad(726, A::offset(A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_mul_ad_rhs(0, 275, A::offset(A::mul(s.ad_value(718), A::add(s.ad_value(719), A::mul(s.ad_value(720), s.ad_value(718)))), (-1.5)));
        }

        s.v[1254] = if (s.v[0] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1254] != 0.0)) {
            s.store_offset_ad(727, A::mul(s.ad_value(0), A::offset(A::mul(A::scale(s.ad_value(0), 0.5), A::offset(A::scale(s.ad_value(0), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1255] = if (s.v[0] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1254] != 0.0))) && (s.v[1255] != 0.0)) {
            s.store_exp(727, 0);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1254] != 0.0))) && (!(s.v[1255] != 0.0))) {
            s.store_div_from_scalar_ad(727, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
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
        s.v[1256] = if (s.v[2] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1256] != 0.0)) {
            s.store_offset_ad(728, A::mul(s.ad_value(2), A::offset(A::mul(A::scale(s.ad_value(2), 0.5), A::offset(A::scale(s.ad_value(2), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1257] = if (s.v[2] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1256] != 0.0))) && (s.v[1257] != 0.0)) {
            s.store_exp(728, 2);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1256] != 0.0))) && (!(s.v[1257] != 0.0))) {
            s.store_div_from_scalar_ad(728, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_div_ad(0, A::offset(s.ad_value(725), 1.0), A::offset(s.ad_value(726), 1.0));
        }

        s.v[1258] = if (s.v[0] < 1e-80) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1258] != 0.0)) {
            s.store_scalar(0, 1e-80);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_mul_ad_rhs(2, 85, A::sub(s.ad_value(328), s.ad_value(86)));
        }

        s.v[1259] = if (((s.v[2]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_exp(3, 2);
        }

        s.v[1260] = if (s.v[2] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(2), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_add_ad_lhs(4, A::mul(s.ad_value(85), s.ad_value(699)), 2);
        }

        s.v[1261] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1262] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1246] != 0.0)) {
            s.store_sub_ad(729, A::div(A::mul(A::mul(A::mul(s.ad_value(722), s.ad_value(727)), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)), A::div(A::mul(A::mul(s.ad_value(723), s.ad_value(728)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)));
        }

        s.v[1263] = if ((s.v[70] > 0.0) || (s.v[72] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add(716, 715, 281);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_scale_ad(717, A::sub(s.ad_value(716), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(716)), A::neg(s.ad_value(716))), 0.01))), 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_mul_ad_lhs(718, A::sqrt(A::offset(A::square(s.ad_value(715)), 0.0001)), 272);
        }

        s.v[1264] = if ((((0.5 * s.v[701])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1264] != 0.0)) {
            s.store_exp_ad(0, A::scale(s.ad_value(701), 0.5));
        }

        s.v[1265] = if ((0.5 * s.v[701]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1264] != 0.0))) && (s.v[1265] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(A::scale(s.ad_value(701), 0.5), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::scale(s.ad_value(701), 0.5), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::scale(s.ad_value(701), 0.5), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_div_from_scalar_ad(2, 1.0, A::offset(s.ad_value(0), 1.0));
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_sub_from_scalar(3, 1.0, 2);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add_ad(719, A::mul(s.ad_value(83), s.ad_value(2)), A::mul(s.ad_value(80), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add_ad(720, A::mul(s.ad_value(84), s.ad_value(2)), A::mul(s.ad_value(82), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add_ad(721, A::mul(s.ad_value(278), s.ad_value(2)), A::mul(s.ad_value(277), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add_ad(722, A::mul(s.ad_value(72), s.ad_value(2)), A::mul(s.ad_value(70), s.ad_value(3)));
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_scaled_mul(723, 74, 3, 1e-6);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_mul_ad_rhs(2, 275, A::div(A::scale(s.ad_value(81), (-1.0)), s.ad_value(718)));
        }

        s.v[1266] = if (s.v[720] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1266] != 0.0)) {
            s.store_scale_ad(718, A::sub(A::add(s.ad_value(718), s.ad_value(721)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(721)), A::sub(s.ad_value(718), s.ad_value(721))), 1e-6))), 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add_ad(724, A::offset(s.ad_value(711), 3.0), A::mul(s.ad_value(717), s.ad_value(220)));
        }

        s.v[1267] = if (((s.v[724]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1267] != 0.0)) {
            s.store_exp(725, 724);
        }

        s.v[1268] = if (s.v[724] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1267] != 0.0))) && (s.v[1268] != 0.0)) {
            s.store_div_from_scalar_ad(725, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1267] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_scale_ad(725, A::offset(A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add_ad_lhs(724, A::add(A::offset(s.ad_value(711), 3.0), A::mul(s.ad_value(717), s.ad_value(220))), 701);
        }

        s.v[1269] = if (((s.v[724]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1269] != 0.0)) {
            s.store_exp(726, 724);
        }

        s.v[1270] = if (s.v[724] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1269] != 0.0))) && (s.v[1270] != 0.0)) {
            s.store_div_from_scalar_ad(726, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1269] != 0.0))) && (!(s.v[1270] != 0.0))) {
            s.store_scale_ad(726, A::offset(A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_mul_ad_rhs(0, 275, A::offset(A::mul(s.ad_value(718), A::add(s.ad_value(719), A::mul(s.ad_value(720), s.ad_value(718)))), (-1.5)));
        }

        s.v[1271] = if (s.v[0] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1271] != 0.0)) {
            s.store_offset_ad(727, A::mul(s.ad_value(0), A::offset(A::mul(A::scale(s.ad_value(0), 0.5), A::offset(A::scale(s.ad_value(0), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1272] = if (s.v[0] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1271] != 0.0))) && (s.v[1272] != 0.0)) {
            s.store_exp(727, 0);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1271] != 0.0))) && (!(s.v[1272] != 0.0))) {
            s.store_div_from_scalar_ad(727, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        s.v[1273] = if (s.v[2] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1273] != 0.0)) {
            s.store_offset_ad(728, A::mul(s.ad_value(2), A::offset(A::mul(A::scale(s.ad_value(2), 0.5), A::offset(A::scale(s.ad_value(2), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1274] = if (s.v[2] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1273] != 0.0))) && (s.v[1274] != 0.0)) {
            s.store_exp(728, 2);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1273] != 0.0))) && (!(s.v[1274] != 0.0))) {
            s.store_div_from_scalar_ad(728, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_div_ad(0, A::offset(s.ad_value(725), 1.0), A::offset(s.ad_value(726), 1.0));
        }

        s.v[1275] = if (s.v[0] < 1e-80) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1275] != 0.0)) {
            s.store_scalar(0, 1e-80);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_mul_ad_rhs(2, 85, A::sub(s.ad_value(326), s.ad_value(86)));
        }

        s.v[1276] = if (((s.v[2]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1276] != 0.0)) {
            s.store_exp(3, 2);
        }

        s.v[1277] = if (s.v[2] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1276] != 0.0))) && (s.v[1277] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(2), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_add_ad_lhs(4, A::mul(s.ad_value(85), s.ad_value(698)), 2);
        }

        s.v[1278] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (s.v[1278] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1279] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1278] != 0.0))) && (s.v[1279] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1263] != 0.0)) {
            s.store_sub_ad(730, A::div(A::mul(A::mul(A::mul(s.ad_value(722), s.ad_value(727)), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)), A::div(A::mul(A::mul(s.ad_value(723), s.ad_value(728)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(5), 1.0)));
        }

        s.v[1280] = if (s.v[68] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul_ad_lhs(731, A::neg(s.ad_value(432)), 382);
        }

        s.v[1281] = if (((((2.0 * s.v[731]) - s.v[407])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_exp_ad(0, A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)));
        }

        s.v[1282] = if (((2.0 * s.v[731]) - s.v[407]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1281] != 0.0))) && (s.v[1282] != 0.0)) {
            let assign26610_ad_e28062: A = A::div_from_scalar(1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
            s.store_ad(0, &assign26610_ad_e28062);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1281] != 0.0))) && (!(s.v[1282] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul_ad_rhs(732, 222, A::sub(A::offset(s.ad_value(731), 0.6931471805599), A::ln(A::offset(s.ad_value(0), 1.0))));
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_scaled_add(733, 388, 408, 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul(734, 222, 733);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_add(716, 734, 280);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_scale_ad(717, A::sub(s.ad_value(716), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(716)), A::neg(s.ad_value(716))), 0.01))), 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul_ad_lhs(718, A::sqrt(A::offset(A::square(s.ad_value(734)), 0.0001)), 272);
        }

        s.v[1283] = if (s.v[79] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (s.v[1283] != 0.0)) {
            s.store_scale_ad(718, A::sub(A::add(s.ad_value(718), s.ad_value(276)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(276)), A::sub(s.ad_value(718), s.ad_value(276))), 1e-6))), 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_add(736, 396, 230);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_sub(735, 736, 733);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul_ad_lhs(724, A::add(s.ad_value(735), A::mul(A::sub(A::sub(s.ad_value(717), s.ad_value(279)), s.ad_value(732)), s.ad_value(223))), 282);
        }

        s.v[1284] = if (((s.v[724]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (s.v[1284] != 0.0)) {
            s.store_exp(725, 724);
        }

        s.v[1285] = if (s.v[724] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1284] != 0.0))) && (s.v[1285] != 0.0)) {
            s.store_div_from_scalar_ad(725, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1284] != 0.0))) && (!(s.v[1285] != 0.0))) {
            s.store_scale_ad(725, A::offset(A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul_ad_lhs(724, A::mul(A::neg(A::sub(s.ad_value(331), s.ad_value(732))), s.ad_value(223)), 282);
        }

        s.v[1286] = if (((s.v[724]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (s.v[1286] != 0.0)) {
            s.store_exp(0, 724);
        }

        s.v[1287] = if (s.v[724] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1286] != 0.0))) && (s.v[1287] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul(726, 725, 0);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul_ad_rhs(0, 274, A::offset(A::mul(s.ad_value(718), A::add(s.ad_value(78), A::mul(s.ad_value(79), s.ad_value(718)))), (-1.5)));
        }

        s.v[1288] = if (s.v[0] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (s.v[1288] != 0.0)) {
            s.store_offset_ad(727, A::mul(s.ad_value(0), A::offset(A::mul(A::scale(s.ad_value(0), 0.5), A::offset(A::scale(s.ad_value(0), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[1289] = if (((s.v[0]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1288] != 0.0))) && (s.v[1289] != 0.0)) {
            s.store_exp(727, 0);
        }

        s.v[1290] = if (s.v[0] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1288] != 0.0))) && (!(s.v[1289] != 0.0))) && (s.v[1290] != 0.0)) {
            s.store_div_from_scalar_ad(727, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1288] != 0.0))) && (!(s.v[1289] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_scale_ad(727, A::offset(A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul_ad(737, A::mul(s.ad_value(68), s.ad_value(727)), A::ln(A::div(A::offset(s.ad_value(725), 1.0), A::offset(s.ad_value(726), 1.0))));
        }

        s.v[1291] = if ((s.v[736] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0))) { 1.0 } else { 0.0 };

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (s.v[1291] != 0.0)) {
            s.store_scalar(738, 1.0);
        }

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (s.v[1291] != 0.0)) {
            s.store_scalar(739, 0.5);
        }

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) {
            s.store_add_ad_rhs(0, 78, A::mul(A::scale(s.ad_value(79), 2.0), s.ad_value(718)));
        }

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) {
            s.store_mul_ad_lhs(740, A::div(s.ad_value(87), A::mul(s.ad_value(0), s.ad_value(274))), 223);
        }

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) {
            s.store_div(741, 731, 740);
        }

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) {
            s.store_mul_ad_lhs(742, A::mul(s.ad_value(740), s.ad_value(430)), 397);
        }

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) {
            s.store_scale_ad(744, A::mul(s.ad_value(742), A::sub_from_scalar(1.0, s.ad_value(742))), 0.5);
        }

        if (((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) {
            s.store_sub_from_scalar_ad(743, 0.5, A::scale(s.ad_value(744), 3.0));
        }

        s.v[1292] = if (s.v[741] < 0.001) { 1.0 } else { 0.0 };

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (s.v[1292] != 0.0)) {
            s.store_square(745, 741);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (s.v[1292] != 0.0)) {
            s.store_offset_ad(738, A::mul(s.ad_value(745), A::add(A::offset(A::scale(s.ad_value(742), 0.3333333333333), 0.1666666666667), A::mul(A::scale(s.ad_value(745), 0.1666666666667), A::offset(A::scale(s.ad_value(742), 0.2), 0.05)))), 1.0);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (s.v[1292] != 0.0)) {
            s.store_sub_ad(739, A::scale(s.ad_value(738), 0.5), A::mul(A::scale(s.ad_value(741), 0.1666666666667), A::offset(A::mul(s.ad_value(745), A::add(A::scale(A::offset(s.ad_value(744), 0.25), 0.4), A::mul(A::scale(s.ad_value(745), 0.0285714285714), A::offset(s.ad_value(744), 0.125)))), 1.0)));
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_div_from_scalar(746, 1.0, 741);
        }

        s.v[1293] = if (((s.v[741]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) && (s.v[1293] != 0.0)) {
            s.store_exp(747, 741);
        }

        s.v[1294] = if (s.v[741] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) && (!(s.v[1293] != 0.0))) && (s.v[1294] != 0.0)) {
            s.store_div_from_scalar_ad(747, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(741)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(741)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(741)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) && (!(s.v[1293] != 0.0))) && (!(s.v[1294] != 0.0))) {
            s.store_scale_ad(747, A::offset(A::mul(A::offset(s.ad_value(741), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(741), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(741), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_div_from_scalar(748, 1.0, 747);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_sub(0, 747, 748);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_add(3, 747, 748);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_scale_ad(738, A::add(A::mul(A::mul(A::sub_from_scalar(1.0, s.ad_value(742)), s.ad_value(0)), s.ad_value(746)), A::mul(s.ad_value(742), s.ad_value(3))), 0.5);
        }

        if ((((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) && (!(s.v[1291] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_scale_ad(739, A::sub(A::sub(s.ad_value(738), A::mul(s.ad_value(0), A::sub(s.ad_value(744), A::mul(A::mul(s.ad_value(743), s.ad_value(746)), s.ad_value(746))))), A::mul(A::mul(s.ad_value(743), s.ad_value(3)), s.ad_value(746))), 0.5);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul(347, 737, 738);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_mul(750, 737, 739);
        }

        if ((s.v[1245] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_sub(749, 347, 750);
        }

        s.v[1295] = if (s.v[330] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1245] != 0.0) && (s.v[1295] != 0.0)) {
            s.store_add(348, 750, 729);
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
        if ((s.v[1245] != 0.0) && (s.v[1295] != 0.0)) {
            s.store_add(349, 749, 730);
        }

        if ((s.v[1245] != 0.0) && (!(s.v[1295] != 0.0))) {
            s.store_add(348, 749, 729);
        }

        if ((s.v[1245] != 0.0) && (!(s.v[1295] != 0.0))) {
            s.store_add(349, 750, 730);
        }

        s.v[351] = 0.0;

        s.v[1296] = if (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[714] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1296] != 0.0) {
            s.store_sqrt_ad(751, A::offset(A::add(A::square(s.ad_value(714)), A::mul(A::mul(A::square(s.ad_value(95)), s.ad_value(327)), s.ad_value(327))), 1e-6));
        }

        if (s.v[1296] != 0.0) {
            s.store_div_ad_lhs(0, A::neg(s.ad_value(91)), 751);
        }

        s.v[1297] = if (((s.v[0]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1296] != 0.0) && (s.v[1297] != 0.0)) {
            s.store_exp(3, 0);
        }

        s.v[1298] = if (s.v[0] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1296] != 0.0) && (!(s.v[1297] != 0.0))) && (s.v[1298] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1296] != 0.0) && (!(s.v[1297] != 0.0))) && (!(s.v[1298] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1296] != 0.0) {
            s.store_mul(4, 97, 699);
        }

        s.v[1299] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1296] != 0.0) && (s.v[1299] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1300] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1296] != 0.0) && (!(s.v[1299] != 0.0))) && (s.v[1300] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1296] != 0.0) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1296] != 0.0) {
            s.store_mul_ad(351, A::scale(A::mul(A::mul(A::mul(A::mul(A::neg(s.ad_value(89)), s.ad_value(699)), s.ad_value(714)), s.ad_value(751)), s.ad_value(3)), 0.5), A::offset(s.ad_value(5), 1.0));
        }

        s.v[350] = 0.0;

        s.v[1301] = if (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[715] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1301] != 0.0) {
            s.store_sqrt_ad(752, A::offset(A::add(A::square(s.ad_value(715)), A::mul(A::mul(A::square(s.ad_value(96)), s.ad_value(329)), s.ad_value(329))), 1e-6));
        }

        if (s.v[1301] != 0.0) {
            s.store_div_ad_lhs(0, A::neg(s.ad_value(92)), 752);
        }

        s.v[1302] = if (((s.v[0]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1301] != 0.0) && (s.v[1302] != 0.0)) {
            s.store_exp(3, 0);
        }

        s.v[1303] = if (s.v[0] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1301] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1303] != 0.0)) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1301] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_scale_ad(3, A::offset(A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1301] != 0.0) {
            s.store_mul(4, 98, 698);
        }

        s.v[1304] = if (((s.v[4]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1301] != 0.0) && (s.v[1304] != 0.0)) {
            s.store_exp(5, 4);
        }

        s.v[1305] = if (s.v[4] < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1301] != 0.0) && (!(s.v[1304] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1301] != 0.0) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_scale_ad(5, A::offset(A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1301] != 0.0) {
            s.store_mul_ad(350, A::scale(A::mul(A::mul(A::mul(A::mul(A::neg(s.ad_value(90)), s.ad_value(698)), s.ad_value(715)), s.ad_value(752)), s.ad_value(3)), 0.5), A::offset(s.ad_value(5), 1.0));
        }

        s.v[352] = 0.0;

        s.v[1306] = if (p.p12 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1306] != 0.0) {
            s.store_mul(754, 332, 285);
        }

        if (s.v[1306] != 0.0) {
            s.store_mul_ad_lhs(755, A::offset(A::sqrt(A::offset(A::square(s.ad_value(332)), 0.01)), (-0.1)), 285);
        }

        if (s.v[1306] != 0.0) {
            s.store_scaled_sub(756, 754, 755, 0.5);
        }

        if (s.v[1306] != 0.0) {
            s.store_sub_ad_lhs(757, A::sub(A::mul(A::sub(s.ad_value(331), s.ad_value(100)), s.ad_value(285)), s.ad_value(756)), 230);
        }

        if (s.v[1306] != 0.0) {
            s.store_sub_ad_lhs(758, A::sub(A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(101)), s.ad_value(285)), s.ad_value(756)), 230);
        }

        if (s.v[1306] != 0.0) {
            s.store_div_from_scalar_ad(759, 1.0, A::offset(s.ad_value(105), 1.0));
        }

        if (s.v[1306] != 0.0) {
            s.store_div_from_scalar_ad(760, 1.0, A::offset(s.ad_value(106), 1.0));
        }

        if (s.v[1306] != 0.0) {
            s.store_mul(761, 109, 285);
        }

        if (s.v[1306] != 0.0) {
            s.store_mul_ad(0, A::scale(s.ad_value(761), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(755), s.ad_value(761)), 1.0)), (-1.0)));
        }

        if (s.v[1306] != 0.0) {
            s.store_mul(762, 107, 0);
        }

        if (s.v[1306] != 0.0) {
            s.store_mul(763, 108, 0);
        }

        if (s.v[1306] != 0.0) {
            s.store_add_ad_lhs(764, A::mul(A::add(s.ad_value(757), s.ad_value(762)), s.ad_value(759)), 756);
        }

        if (s.v[1306] != 0.0) {
            s.store_add_ad_lhs(765, A::mul(A::add(s.ad_value(758), s.ad_value(763)), s.ad_value(760)), 756);
        }

        if (s.v[1306] != 0.0) {
            let assign27740_ad_e29663: A = A::sub(A::add(A::add(s.ad_value(765), A::mul(s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(765), A::mul(s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)))), s.ad_value(221)), A::sub(A::add(s.ad_value(765), A::mul(s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(766, assign27740_ad_e29663, 0.5);
        }

        if (s.v[1306] != 0.0) {
            let assign27750_ad_e29700: A = A::sub(A::add(A::add(s.ad_value(764), A::mul(s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(764), A::mul(s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)))), s.ad_value(221)), A::sub(A::add(s.ad_value(764), A::mul(s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(767, assign27750_ad_e29700, 0.5);
        }

        if (s.v[1306] != 0.0) {
            s.store_div(768, 242, 759);
        }

        if (s.v[1306] != 0.0) {
            s.store_div(769, 243, 760);
        }

        if (s.v[1306] != 0.0) {
            s.store_div_from_scalar(770, 1.0, 768);
        }

        if (s.v[1306] != 0.0) {
            s.store_div_from_scalar(771, 1.0, 769);
        }

        if (s.v[1306] != 0.0) {
            s.store_div_from_scalar_ad(772, 1.0, A::add(A::offset(s.ad_value(770), 1.0), s.ad_value(771)));
        }

        if (s.v[1306] != 0.0) {
            s.store_div_ad_rhs(773, 286, A::square(s.ad_value(386)));
        }

        if (s.v[1306] != 0.0) {
            s.store_mul_ad_rhs(774, 772, A::sub(s.ad_value(766), s.ad_value(767)));
        }

        s.v[1307] = if ((((s.v[767] - s.v[766])) as f64).abs() <= 1e-12) { 1.0 } else { 0.0 };

        if ((s.v[1306] != 0.0) && (s.v[1307] != 0.0)) {
            s.store_sub_ad(2, A::sub_from_scalar(1.0, A::mul(s.ad_value(772), s.ad_value(770))), A::mul(s.ad_value(772), s.ad_value(771)));
        }

        if ((s.v[1306] != 0.0) && (s.v[1307] != 0.0)) {
            s.store_mul_ad_lhs(3, A::sub(A::sub(A::add(s.ad_value(771), A::mul(A::mul(A::scale(s.ad_value(770), 0.5), s.ad_value(772)), s.ad_value(770))), A::mul(A::mul(A::scale(s.ad_value(771), 0.5), s.ad_value(772)), s.ad_value(771))), A::div_from_scalar(0.5, s.ad_value(772))), 774);
        }

        if ((s.v[1306] != 0.0) && (s.v[1307] != 0.0)) {
            s.store_div_ad_lhs(4, A::mul(A::scale(A::sub(s.ad_value(2), s.ad_value(3)), 0.5), s.ad_value(773)), 772);
        }

        if ((s.v[1306] != 0.0) && (!(s.v[1307] != 0.0))) {
            s.store_exp_ad(2, A::mul(A::neg(s.ad_value(770)), s.ad_value(774)));
        }

        if ((s.v[1306] != 0.0) && (!(s.v[1307] != 0.0))) {
            s.store_exp_ad(3, A::mul(A::sub(s.ad_value(771), A::div_from_scalar(1.0, s.ad_value(772))), s.ad_value(774)));
        }

        if ((s.v[1306] != 0.0) && (!(s.v[1307] != 0.0))) {
            s.store_div_ad(4, A::mul(s.ad_value(773), A::sub(s.ad_value(2), s.ad_value(3))), A::scale(s.ad_value(774), 2.0));
        }

        if (s.v[1306] != 0.0) {
            s.copy_ad(775, 4);
        }

        s.v[1308] = if (s.v[766] < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1306] != 0.0) && (s.v[1308] != 0.0)) {
            s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(s.ad_value(766))), 1.0));
        }

        if ((s.v[1306] != 0.0) && (s.v[1308] != 0.0)) {
            s.store_mul_ad_rhs(0, 780, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0))));
        }

        s.v[1309] = if (s.v[766] < 0.0) { 1.0 } else { 0.0 };

        s.v[1310] = if (s.v[766] > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1306] != 0.0) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) && (s.v[1310] != 0.0)) {
            s.store_exp(780, 766);
        }

        if ((((s.v[1306] != 0.0) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) && (!(s.v[1310] != 0.0))) {
            s.store_div_from_scalar_ad(780, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(766)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(766)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(766)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1306] != 0.0) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_mul(0, 775, 780);
        }

        if (((s.v[1306] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_add_ad_lhs(780, A::ln(s.ad_value(775)), 766);
        }

        if (((s.v[1306] != 0.0) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_mul_ad_rhs(0, 780, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0))));
        }

        if (s.v[1306] != 0.0) {
            s.copy_ad(776, 0);
        }

        s.v[1311] = if ((s.v[766] - s.v[407]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1306] != 0.0) && (s.v[1311] != 0.0)) {
            s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(A::sub(s.ad_value(766), s.ad_value(407)))), 1.0));
        }

        if ((s.v[1306] != 0.0) && (s.v[1311] != 0.0)) {
            s.store_mul_ad_rhs(0, 780, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0))));
        }

        s.v[1312] = if ((s.v[766] - s.v[407]) < 0.0) { 1.0 } else { 0.0 };

        s.v[1313] = if ((s.v[766] - s.v[407]) > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1306] != 0.0) && (!(s.v[1311] != 0.0))) && (s.v[1312] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_exp_ad(780, A::sub(s.ad_value(766), s.ad_value(407)));
        }

        if ((((s.v[1306] != 0.0) && (!(s.v[1311] != 0.0))) && (s.v[1312] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_div_from_scalar_ad(780, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1306] != 0.0) && (!(s.v[1311] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_mul(0, 775, 780);
        }

        if (((s.v[1306] != 0.0) && (!(s.v[1311] != 0.0))) && (!(s.v[1312] != 0.0))) {
            s.store_add_ad(780, A::ln(s.ad_value(775)), A::sub(s.ad_value(766), s.ad_value(407)));
        }

        if (((s.v[1306] != 0.0) && (!(s.v[1311] != 0.0))) && (!(s.v[1312] != 0.0))) {
            s.store_mul_ad_rhs(0, 780, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0))));
        }

        if (s.v[1306] != 0.0) {
            s.copy_ad(777, 0);
        }

        if (s.v[1306] != 0.0) {
            s.store_mul_ad(778, A::offset(A::scale(A::add(s.ad_value(776), s.ad_value(777)), 0.5), 1.0), A::sub(s.ad_value(776), s.ad_value(777)));
        }

        if (s.v[1306] != 0.0) {
            s.store_mul_ad_lhs(779, A::square(s.ad_value(284)), 110);
        }

        if (s.v[1306] != 0.0) {
            s.store_div_ad_lhs(352, A::mul(A::mul(s.ad_value(779), s.ad_value(237)), s.ad_value(778)), 418);
        }

        s.v[353] = 0.0;

        s.v[354] = 0.0;

        s.v[1314] = if (p.p8 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1314] != 0.0) {
            s.store_div_ad_lhs(753, A::sub(s.ad_value(335), A::mul(s.ad_value(115), s.ad_value(407))), 223);
        }

        s.v[1315] = if (s.v[753] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1314] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_div_ad(3, A::scale(s.ad_value(113), (-1.0)), A::offset(s.ad_value(753), 1e-30));
        }

        s.v[1316] = if (((s.v[3]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1314] != 0.0) && (s.v[1315] != 0.0)) && (s.v[1316] != 0.0)) {
            s.store_exp(0, 3);
        }

        s.v[1317] = if (s.v[3] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1314] != 0.0) && (s.v[1315] != 0.0)) && (!(s.v[1316] != 0.0))) && (s.v[1317] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(3)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1314] != 0.0) && (s.v[1315] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_scale_ad(0, A::offset(A::mul(A::offset(s.ad_value(3), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(3), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(3), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1314] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_mul_ad_lhs(353, A::mul(s.ad_value(112), s.ad_value(753)), 0);
        }

        if ((s.v[1314] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_mul_ad_rhs(354, 353, A::add(s.ad_value(344), s.ad_value(352)));
        }

        s.v[1318] = if (s.v[6] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1318] != 0.0) {
            s.store_mul_ad_lhs(0, A::abs(A::mul(A::add(s.ad_value(344), s.ad_value(352)), s.ad_value(332))), 168);
        }

        s.v[1319] = if (s.v[0] > (100000000.0 * p.p16)) { 1.0 } else { 0.0 };

        if ((s.v[1318] != 0.0) && (s.v[1319] != 0.0)) {
            s.store_div_from_scalar(355, (-(p.p16 + (0.25 / p.p16))), 168);
        }

        if ((s.v[1318] != 0.0) && (!(s.v[1319] != 0.0))) {
            s.store_div_ad_lhs(355, A::neg(A::offset(A::scale(A::sub(A::offset(s.ad_value(0), p.p16), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(0), (-p.p16)), A::offset(s.ad_value(0), (-p.p16))), 1.0))), 0.5), (0.25 / p.p16))), 168);
        }

        if (s.v[1318] != 0.0) {
            s.store_div(356, 215, 168);
        }

        if (!(s.v[1318] != 0.0)) {
            s.store_scalar(355, 0.0);
        }

        if (!(s.v[1318] != 0.0)) {
            s.store_ad(356, &A::scale(A::voltage(ctx, &nodes, Some(4), None), 0.001));
        }

        s.store_mul(313, 302, 312);

        s.store_mul(317, 302, 316);

        s.store_mul(321, 302, 320);

        s.store_mul(324, 302, 323);

        s.v[1604] = if (p.p11 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1604] != 0.0) {
            s.copy_ad(1414, 130);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1415, 131);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1416, 135);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1417, 136);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1418, 140);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1419, 141);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1420, 270);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1421, 212);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(1422, 158);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1423, A::sub(A::mul(A::sub(s.ad_value(331), s.ad_value(1414)), s.ad_value(223)), s.ad_value(337)), 230);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1424, A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(1415)), s.ad_value(223)), 337);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1425, 1424, 230);
        }

        s.v[1605] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_scale(0, 16, p.p14);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_div_ad(1426, A::offset(s.ad_value(242), 1.0), A::offset(s.ad_value(243), 1.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_ln(1427, 1426);
        }

        s.v[1606] = if (s.v[1427] > 1e-8) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (s.v[1606] != 0.0)) {
            s.store_div_ad(1428, A::mul(A::scale(s.ad_value(1427), 2.0), A::offset(s.ad_value(1426), 1.0)), A::offset(s.ad_value(1426), (-1.0)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1606] != 0.0))) {
            s.store_scaled_offset(1428, 1427, 2.0, 2.0);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_div_ad_rhs(1429, 249, A::square(s.ad_value(241)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_div_from_scalar(1430, 1.0, 242);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_div_from_scalar(1431, 1.0, 243);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_div_from_scalar_ad(1458, 1.0, A::add(A::offset(s.ad_value(1430), 1.0), s.ad_value(1431)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_mul_ad_rhs(1459, 1458, A::sub(s.ad_value(1423), s.ad_value(1425)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_sub_ad_rhs(1432, 1423, A::mul(s.ad_value(1459), s.ad_value(1430)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_add_ad_rhs(1433, 1425, A::mul(s.ad_value(1459), s.ad_value(1431)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_div_from_scalar_ad(1338, 1.0, A::offset(s.ad_value(242), 1.0));
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
        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_div_from_scalar_ad(1339, 1.0, A::offset(s.ad_value(243), 1.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_offset_ad(1341, A::ln(A::div(A::mul(A::add(s.ad_value(242), A::mul(s.ad_value(243), s.ad_value(1339))), s.ad_value(1428)), s.ad_value(1429))), 1.5);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_offset_ad(1342, A::ln(A::div(A::mul(A::add(s.ad_value(243), A::mul(s.ad_value(242), s.ad_value(1338))), s.ad_value(1428)), s.ad_value(1429))), 1.5);
        }

        s.v[1607] = if (((s.v[1341] - s.v[1432]) / 1.5) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1432)), 0.6666666666666666)), 1.0));
        }

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scaled_sub(1340, 1341, 1432, 0.6666666666666666);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 1.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_mul_ad_lhs(1344, A::add(A::mul(s.ad_value(243), s.ad_value(1425)), s.ad_value(1345)), 1339);
        }

        s.v[1608] = if (((s.v[1342] - s.v[1344]) / 1.5) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (s.v[1608] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1344)), 0.6666666666666666)), 1.0));
        }

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1608] != 0.0))) {
            s.store_scaled_sub(1340, 1342, 1344, 0.6666666666666666);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_sub_ad_rhs(1, 1342, A::scale(s.ad_value(1340), 1.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_mul(2, 0, 1);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_mul(3, 0, 1425);
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_sub(1390, 2, 3);
        }

        s.v[1609] = if ((((-s.v[262])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (s.v[1609] != 0.0)) {
            s.store_exp_ad(1391, A::neg(s.ad_value(262)));
        }

        s.v[1610] = if ((-s.v[262]) < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1609] != 0.0))) && (s.v[1610] != 0.0)) {
            s.store_div_from_scalar_ad(1391, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1609] != 0.0))) && (!(s.v[1610] != 0.0))) {
            s.store_scale_ad(1391, A::offset(A::mul(A::offset(A::neg(s.ad_value(262)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(262)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(262)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.v[1611] = if (((s.v[1390]) as f64).abs() <= s.v[261]) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (s.v[1611] != 0.0)) {
            s.store_scale_ad(1388, A::square(s.ad_value(260)), (0.1666666666667 * 0.707106781186545));
        }

        if (((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (s.v[1611] != 0.0)) {
            s.store_mul_ad(4, A::mul(s.ad_value(1390), s.ad_value(260)), A::offset(A::mul(A::mul(A::mul(s.ad_value(1390), A::sub_from_scalar(1.0, s.ad_value(1391))), s.ad_value(256)), s.ad_value(1388)), 1.0));
        }

        s.v[1612] = if (s.v[1390] < (-s.v[261])) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_neg(1392, 1390);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_scaled_mul(1393, 1392, 260, 1.25);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_scale_ad(1394, A::sub(A::offset(s.ad_value(1393), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1393), (-6.0)), A::offset(s.ad_value(1393), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub(1387, 1392, 1394);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_add_ad(1395, A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::offset(s.ad_value(1394), 1.0)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_ad_lhs(1397, A::scale(s.ad_value(1387), 2.0), 257);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_ad_lhs(1398, A::ln(A::mul(s.ad_value(1395), s.ad_value(258))), 1394);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_add(1385, 1395, 1397);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_add_ad(1386, A::square(s.ad_value(1385)), A::mul(s.ad_value(1398), A::sub(A::mul(A::scale(s.ad_value(1397), 0.5), s.ad_value(1397)), s.ad_value(1395))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_add_ad_rhs(1399, 1394, A::div(A::mul(A::mul(s.ad_value(1395), s.ad_value(1385)), s.ad_value(1398)), A::add(s.ad_value(1386), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(1385), s.ad_value(1386)), s.ad_value(1398)), s.ad_value(1398)), s.ad_value(1397)), A::sub(A::scale(A::square(s.ad_value(1397)), 0.3333333333333), s.ad_value(1395))))));
        }

        s.v[1613] = if (s.v[1399] < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) && (s.v[1613] != 0.0)) {
            s.store_exp(1400, 1399);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) && (!(s.v[1613] != 0.0))) {
            s.store_scale_ad(1400, A::offset(A::mul(A::offset(s.ad_value(1399), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(1399), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(1399), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_div_from_scalar(1401, 1.0, 1400);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_div_from_scalar_ad(1387, 1.0, A::offset(A::square(s.ad_value(1399)), 2.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_mul_ad_lhs(1402, A::square(s.ad_value(1399)), 1387);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_scale_ad(1403, A::mul(A::mul(s.ad_value(1399), s.ad_value(1387)), s.ad_value(1387)), 4.0);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_mul_ad_lhs(1404, A::mul(A::sub(A::scale(s.ad_value(1387), 8.0), A::scale(s.ad_value(1402), 12.0)), s.ad_value(1387)), 1387);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub(1387, 1392, 1399);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_mul(1388, 1391, 1401);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_add_ad(1405, A::scale(s.ad_value(1387), 2.0), A::mul(s.ad_value(257), A::add(A::sub(A::offset(s.ad_value(1400), (-1.0)), s.ad_value(1388)), A::mul(s.ad_value(1391), A::sub_from_scalar(1.0, s.ad_value(1403))))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_ad(1406, A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::add(A::add(A::offset(A::sub(s.ad_value(1400), s.ad_value(1399)), (-1.0)), s.ad_value(1388)), A::mul(s.ad_value(1391), A::sub(A::offset(s.ad_value(1399), (-1.0)), s.ad_value(1402))))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_from_scalar_ad(1387, 2.0, A::mul(s.ad_value(257), A::sub(A::add(s.ad_value(1400), s.ad_value(1388)), A::mul(s.ad_value(1391), s.ad_value(1404)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_ad(1387, A::square(s.ad_value(1405)), A::scale(A::mul(s.ad_value(1406), s.ad_value(1387)), 2.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (s.v[1612] != 0.0)) {
            s.store_sub_ad(4, A::neg(s.ad_value(1399)), A::scale(A::div(s.ad_value(1406), A::add(s.ad_value(1405), A::sqrt(s.ad_value(1387)))), 2.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_div_from_scalar_ad(1407, 1.0, A::offset(A::scale(s.ad_value(256), 0.732464877560822), 1.25));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_mul_ad_lhs(1408, A::offset(A::mul(A::scale(s.ad_value(259), 1.25), s.ad_value(1407)), (-1.0)), 1407);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_mul_ad(1409, A::mul(s.ad_value(1390), s.ad_value(260)), A::offset(A::mul(s.ad_value(1408), s.ad_value(1390)), 1.0));
        }

        s.v[1614] = if ((-s.v[1409]) > (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (s.v[1614] != 0.0)) {
            s.store_exp_ad(1387, A::neg(s.ad_value(1409)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (!(s.v[1614] != 0.0))) {
            s.store_div_from_scalar_ad(1387, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(1409))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(1409))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(1409))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub_from_scalar(1410, 1.0, 1387);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub_ad(1411, A::add(s.ad_value(1390), A::scale(s.ad_value(257), 0.5)), A::mul(s.ad_value(256), A::sqrt(A::sub(A::add(s.ad_value(1390), A::scale(s.ad_value(257), 0.25)), s.ad_value(1410)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_offset(1412, 262, 3.0);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub_ad(1394, A::scale(A::sub(A::add(s.ad_value(1411), s.ad_value(1412)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1411), s.ad_value(1412)), A::sub(s.ad_value(1411), s.ad_value(1412))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(1412), A::sqrt(A::offset(A::square(s.ad_value(1412)), 5.0))), 0.5));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub(1387, 1390, 1394);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_exp_ad(1388, A::neg(s.ad_value(1394)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_div_from_scalar_ad(1389, 1.0, A::offset(A::square(s.ad_value(1394)), 2.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_mul_ad_lhs(1402, A::square(s.ad_value(1394)), 1389);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_scale_ad(1403, A::mul(A::mul(s.ad_value(1394), s.ad_value(1389)), s.ad_value(1389)), 4.0);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_mul_ad_lhs(1404, A::mul(A::sub(A::scale(s.ad_value(1389), 8.0), A::scale(s.ad_value(1402), 12.0)), s.ad_value(1389)), 1389);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_max_from_scalar_ad(1395, 1e-40, A::sub(A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::sub(A::offset(A::add(s.ad_value(1388), s.ad_value(1394)), (-1.0)), A::mul(s.ad_value(1391), A::add(A::offset(s.ad_value(1394), 1.0), s.ad_value(1402)))))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub_from_scalar_ad(1396, 1.0, A::scale(A::mul(s.ad_value(257), A::sub(s.ad_value(1388), A::mul(s.ad_value(1391), s.ad_value(1404)))), 0.5));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_add_ad(1397, A::scale(s.ad_value(1387), 2.0), A::mul(s.ad_value(257), A::sub(A::sub_from_scalar(1.0, s.ad_value(1388)), A::mul(s.ad_value(1391), A::offset(s.ad_value(1403), 1.0)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_add_ad(1398, A::sub(s.ad_value(262), s.ad_value(1394)), A::ln(A::div(s.ad_value(1395), s.ad_value(257))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_add(1385, 1395, 1397);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_add_ad(1386, A::square(s.ad_value(1385)), A::mul(s.ad_value(1398), A::sub(A::mul(A::scale(s.ad_value(1397), 0.5), s.ad_value(1397)), A::mul(s.ad_value(1395), s.ad_value(1396)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            let assign29400_ad_e31905: A = A::add(s.ad_value(1394), A::div(A::mul(A::mul(s.ad_value(1395), s.ad_value(1385)), s.ad_value(1398)), A::add(s.ad_value(1386), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(1385), s.ad_value(1386)), s.ad_value(1398)), s.ad_value(1398)), s.ad_value(1397)), A::sub(A::scale(A::square(s.ad_value(1397)), 0.3333333333333), A::mul(s.ad_value(1395), s.ad_value(1396)))))));
            s.store_ad(1413, &assign29400_ad_e31905);
        }

        s.v[1615] = if (s.v[1413] < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_exp(1400, 1413);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_div_from_scalar(1401, 1.0, 1400);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_mul(1400, 1391, 1400);
        }

        s.v[1616] = if (s.v[1413] > (s.v[262] - 80.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_exp_ad(1400, A::sub(s.ad_value(1413), s.ad_value(262)));
        }

        if ((((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (!(s.v[1615] != 0.0))) && (s.v[1616] != 0.0)) {
            s.store_div(1401, 1391, 1400);
        }

        if ((((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_div_from_scalar_ad(1400, 1.80485e-35, A::offset(A::mul(A::offset(A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) && (!(s.v[1615] != 0.0))) && (!(s.v[1616] != 0.0))) {
            s.store_div_from_scalar_ad(1401, 1.80485e-35, A::offset(A::mul(A::offset(s.ad_value(1413), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(1413), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(1413), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_div_from_scalar_ad(1387, 1.0, A::offset(A::square(s.ad_value(1413)), 2.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_mul_ad_lhs(1402, A::square(s.ad_value(1413)), 1387);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_scale_ad(1403, A::mul(A::mul(s.ad_value(1413), s.ad_value(1387)), s.ad_value(1387)), 4.0);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_mul_ad_lhs(1404, A::mul(A::sub(A::scale(s.ad_value(1387), 8.0), A::scale(s.ad_value(1402), 12.0)), s.ad_value(1387)), 1387);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub(1387, 1390, 1413);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_add_ad(1405, A::scale(s.ad_value(1387), 2.0), A::mul(s.ad_value(257), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(1401)), s.ad_value(1400)), A::mul(s.ad_value(1391), A::offset(s.ad_value(1403), 1.0)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub_ad(1406, A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::sub(A::add(A::offset(A::add(s.ad_value(1401), s.ad_value(1413)), (-1.0)), s.ad_value(1400)), A::mul(s.ad_value(1391), A::add(A::offset(s.ad_value(1413), 1.0), s.ad_value(1402))))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub_from_scalar_ad(1387, 2.0, A::mul(s.ad_value(257), A::sub(A::add(s.ad_value(1401), s.ad_value(1400)), A::mul(s.ad_value(1391), s.ad_value(1404)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_sub_ad(1387, A::square(s.ad_value(1405)), A::scale(A::mul(s.ad_value(1406), s.ad_value(1387)), 2.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) && (!(s.v[1611] != 0.0))) && (!(s.v[1612] != 0.0))) {
            s.store_add_ad_rhs(4, 1413, A::scale(A::div(s.ad_value(1406), A::add(s.ad_value(1405), A::sqrt(s.ad_value(1387)))), 2.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1605] != 0.0)) {
            s.store_mul_ad_rhs(1434, 0, A::add(s.ad_value(4), s.ad_value(3)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1605] != 0.0))) {
            s.copy_ad(1434, 1425);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(0, 244, A::sub(s.ad_value(1423), s.ad_value(1434)));
        }

        s.v[1617] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_scale_ad(1435, A::add(A::add(s.ad_value(0), s.ad_value(253)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))))), 0.5);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_scale_ad(1436, A::add(A::sub(s.ad_value(253), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(253)), A::sub(A::neg(s.ad_value(0)), s.ad_value(253))), A::square(s.ad_value(253))))), 0.5);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_mul_ad_rhs(2, 254, A::exp(A::scale(A::ln(s.ad_value(1435)), (-0.3333333333333))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_mul_ad_rhs(3, 254, A::exp(A::scale(A::ln(s.ad_value(1436)), (-0.3333333333333))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_div(1443, 241, 4);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_offset_ad(1437, A::mul(s.ad_value(242), s.ad_value(2)), 1.0);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_offset_ad(1438, A::mul(s.ad_value(243), s.ad_value(3)), 1.0);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_div_ad_lhs(1439, A::mul(s.ad_value(242), s.ad_value(4)), 1437);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_div_ad_lhs(1440, A::mul(s.ad_value(243), s.ad_value(4)), 1438);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_div_from_scalar_ad(1441, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(1439)), 1.0), A::div_from_scalar(1.0, s.ad_value(1440))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_offset_ad(1437, A::mul(s.ad_value(1439), s.ad_value(2)), 1.0);
        }

        if ((s.v[1604] != 0.0) && (s.v[1617] != 0.0)) {
            s.store_offset_ad(1438, A::mul(s.ad_value(1440), s.ad_value(3)), 1.0);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1617] != 0.0))) {
            s.copy_ad(1443, 241);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1617] != 0.0))) {
            s.copy_ad(1439, 242);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1617] != 0.0))) {
            s.copy_ad(1440, 243);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1617] != 0.0))) {
            s.copy_ad(1441, 244);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1617] != 0.0))) {
            s.store_scalar(1437, 1.0);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1617] != 0.0))) {
            s.store_scalar(1438, 1.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(1442, 1441, A::sub(s.ad_value(1423), s.ad_value(1434)));
        }

        s.v[1618] = if (s.v[1442] > 0.0) { 1.0 } else { 0.0 };

        s.v[1619] = if ((-s.v[1442]) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1618] != 0.0)) && (s.v[1619] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(A::neg(s.ad_value(1442))), 1.0));
        }

        if (((s.v[1604] != 0.0) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) {
            s.store_neg(0, 1442);
        }

        if ((s.v[1604] != 0.0) && (s.v[1618] != 0.0)) {
            s.store_offset_ad(1444, A::add(A::sub(s.ad_value(1423), A::div(s.ad_value(1442), s.ad_value(1439))), s.ad_value(0)), (-0.6931471805599));
        }

        s.v[1620] = if (s.v[1442] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1618] != 0.0))) && (s.v[1620] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(s.ad_value(1442)), 1.0));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1618] != 0.0))) && (!(s.v[1620] != 0.0))) {
            s.copy_ad(0, 1442);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1618] != 0.0))) {
            s.store_offset_ad(1444, A::add(A::add(s.ad_value(1434), A::div(s.ad_value(1442), s.ad_value(1440))), s.ad_value(0)), (-0.6931471805599));
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(1445, A::sub(A::add(s.ad_value(1444), s.ad_value(250)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1444), s.ad_value(250)), A::sub(s.ad_value(1444), s.ad_value(250))), 4.0))), 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_ad(1446, A::sqrt(A::offset(A::div(A::scale(A::sub(s.ad_value(250), s.ad_value(1445)), 2.0), s.ad_value(251)), 1.0)), (-1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1447, 1445, A::mul(s.ad_value(251), s.ad_value(1446)));
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(0, A::add(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), 1.0), (-0.5))), 0.01))), 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1448, 1.0, A::offset(A::mul(s.ad_value(1416), s.ad_value(0)), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1449, 1.0, A::offset(A::mul(s.ad_value(1417), s.ad_value(0)), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(0, A::mul(A::mul(A::scale(s.ad_value(325), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(336), s.ad_value(325)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1446)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1424)), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1450, 1418, 0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1451, 1419, 0);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1452, A::add(A::mul(A::add(A::sub(s.ad_value(1423), s.ad_value(1447)), s.ad_value(1450)), s.ad_value(1448)), s.ad_value(1447)), 337);
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
        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1453, A::add(A::mul(A::add(A::sub(s.ad_value(1434), s.ad_value(1447)), s.ad_value(1451)), s.ad_value(1449)), s.ad_value(1447)), 337);
        }

        if (s.v[1604] != 0.0) {
            let assign30040_ad_e32824: A = A::sub(A::add(A::add(s.ad_value(1453), A::mul(s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(1453), A::mul(s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)))), s.ad_value(221)), A::sub(A::add(s.ad_value(1453), A::mul(s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(1454, assign30040_ad_e32824, 0.5);
        }

        if (s.v[1604] != 0.0) {
            let assign30050_ad_e32861: A = A::sub(A::add(A::add(s.ad_value(1452), A::mul(s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(1452), A::mul(s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)))), s.ad_value(221)), A::sub(A::add(s.ad_value(1452), A::mul(s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(1455, assign30050_ad_e32861, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_div(1456, 1439, 1448);
        }

        if (s.v[1604] != 0.0) {
            s.store_div(1457, 1440, 1449);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar(1430, 1.0, 1456);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar(1431, 1.0, 1457);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1458, 1.0, A::add(A::offset(s.ad_value(1430), 1.0), s.ad_value(1431)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad_rhs(1429, 249, A::square(s.ad_value(1443)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1426, A::offset(s.ad_value(1456), 1.0), A::offset(s.ad_value(1457), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_ln(1427, 1426);
        }

        s.v[1621] = if (s.v[1427] > 1e-8) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1621] != 0.0)) {
            s.store_div_ad(1428, A::mul(A::scale(s.ad_value(1427), 2.0), A::offset(s.ad_value(1426), 1.0)), A::offset(s.ad_value(1426), (-1.0)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1621] != 0.0))) {
            s.store_scaled_offset(1428, 1427, 2.0, 2.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(1459, 1458, A::sub(s.ad_value(1454), s.ad_value(1455)));
        }

        if (s.v[1604] != 0.0) {
            s.store_square(1460, 1459);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1432, 1454, A::mul(s.ad_value(1459), s.ad_value(1430)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1433, 1455, A::mul(s.ad_value(1459), s.ad_value(1431)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1338, 1.0, A::offset(s.ad_value(1456), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1339, 1.0, A::offset(s.ad_value(1457), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_ad(1341, A::ln(A::div(A::mul(A::add(s.ad_value(1456), A::mul(s.ad_value(1457), s.ad_value(1339))), s.ad_value(1428)), s.ad_value(1429))), 3.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_ad(1342, A::ln(A::div(A::mul(A::add(s.ad_value(1457), A::mul(s.ad_value(1456), s.ad_value(1338))), s.ad_value(1428)), s.ad_value(1429))), 3.0);
        }

        s.v[1622] = if (((s.v[1341] - s.v[1432]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1622] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1432)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1622] != 0.0))) {
            s.store_scaled_sub(1340, 1341, 1432, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.v[1623] = if (((s.v[1342] - s.v[1433]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1623] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1433)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1623] != 0.0))) {
            s.store_scaled_sub(1340, 1342, 1433, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1343, A::add(A::mul(s.ad_value(1456), s.ad_value(1454)), s.ad_value(1346)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1344, A::add(A::mul(s.ad_value(1457), s.ad_value(1455)), s.ad_value(1345)), 1339);
        }

        s.v[1624] = if (((s.v[1341] - s.v[1343]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1624] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1343)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1624] != 0.0))) {
            s.store_scaled_sub(1340, 1341, 1343, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.v[1625] = if (((s.v[1342] - s.v[1344]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1625] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1344)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1625] != 0.0))) {
            s.store_scaled_sub(1340, 1342, 1344, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1461, 1454, 1345);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1465, 1455, 1346);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1352, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1355, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1461);
        }

        s.v[1626] = if ((s.v[1454] - s.v[1461]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1626] != 0.0)) {
            s.store_exp_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1626] != 0.0))) {
            s.store_scale_ad(1338, A::offset(A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1348, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1627] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((s.v[1604] != 0.0) && (s.v[1627] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1628] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1627] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1629] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1629] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1629] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((s.v[1604] != 0.0) && (s.v[1629] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1630] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1629] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1629] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1629] != 0.0))) && (s.v[1630] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1629] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1629] != 0.0))) && (!(s.v[1630] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1631] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((s.v[1604] != 0.0) && (s.v[1631] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((s.v[1604] != 0.0) && (s.v[1631] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1631] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1631] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1631] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1631] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1631] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1632] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1632] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1632] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1632] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1632] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1632] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1632] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1461);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1632] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1632] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1372, 1457, 1369);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1461, 1461, 1376);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1461);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1377, 1457, 1465);
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1370, 1347, 1377);
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
        }

        if (s.v[1604] != 0.0) {
            s.store_sqrt_ad(1381, A::sub(A::square(s.ad_value(1379)), A::mul(A::scale(s.ad_value(1378), 4.0), s.ad_value(1380))));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.v[1633] = if (s.v[1382] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1633] != 0.0)) {
            s.store_mul_ad_rhs(1373, 1382, A::add(A::sub(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1454)), s.ad_value(1461)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1633] != 0.0)) {
            s.store_add_ad_lhs(1374, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1382);
        }

        if ((s.v[1604] != 0.0) && (s.v[1633] != 0.0)) {
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1461)), 1341);
        }

        s.v[1634] = if ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1633] != 0.0)) && (s.v[1634] != 0.0)) {
            s.store_sub_ad_rhs(1461, 1461, A::div(s.ad_value(1373), s.ad_value(1374)));
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
        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1461);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1377, 1457, 1465);
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1370, 1347, 1377);
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
        }

        if (s.v[1604] != 0.0) {
            s.store_sqrt_ad(1381, A::sub(A::square(s.ad_value(1379)), A::mul(A::scale(s.ad_value(1378), 4.0), s.ad_value(1380))));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
        }

        s.v[1635] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1635] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1635] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1635] != 0.0)) {
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        s.v[1636] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1635] != 0.0))) && (s.v[1636] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1635] != 0.0))) && (s.v[1636] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1635] != 0.0))) && (s.v[1636] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1635] != 0.0))) && (s.v[1636] != 0.0)) {
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1635] != 0.0))) && (!(s.v[1636] != 0.0))) {
            s.store_offset_ad(1353, A::mul(A::scale(s.ad_value(1349), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0238095238095))))), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1635] != 0.0))) && (!(s.v[1636] != 0.0))) {
            s.store_scale_ad(1354, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1349, 1349, A::div(A::add(A::add(A::mul(s.ad_value(1370), s.ad_value(1353)), A::mul(s.ad_value(1347), s.ad_value(1377))), s.ad_value(1349)), A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0)));
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.v[1637] = if (s.v[1382] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1637] != 0.0)) {
            s.store_mul_ad_rhs(1373, 1382, A::add(A::sub(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1454)), s.ad_value(1461)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1637] != 0.0)) {
            s.store_add_ad_lhs(1374, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1382);
        }

        if ((s.v[1604] != 0.0) && (s.v[1637] != 0.0)) {
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1461)), 1341);
        }

        s.v[1638] = if ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1637] != 0.0)) && (s.v[1638] != 0.0)) {
            s.store_sub_ad_rhs(1461, 1461, A::div(s.ad_value(1373), s.ad_value(1374)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1461);
        }

        s.v[1639] = if ((s.v[1454] - s.v[1461]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1639] != 0.0)) {
            s.store_exp_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1639] != 0.0))) {
            s.store_scale_ad(1338, A::offset(A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1348, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1640] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((s.v[1604] != 0.0) && (s.v[1640] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1641] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1642] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1642] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1642] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((s.v[1604] != 0.0) && (s.v[1642] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1643] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1642] != 0.0))) && (s.v[1643] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1642] != 0.0))) && (s.v[1643] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1642] != 0.0))) && (s.v[1643] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1642] != 0.0))) && (!(s.v[1643] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1642] != 0.0))) && (!(s.v[1643] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1644] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((s.v[1604] != 0.0) && (s.v[1644] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((s.v[1604] != 0.0) && (s.v[1644] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1644] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1644] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1644] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1644] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1644] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1645] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1645] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1645] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1645] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1645] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1645] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1645] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1461);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1645] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1645] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1372, 1457, 1369);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1461, 1461, 1376);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1461);
        }

        s.v[1646] = if ((s.v[1454] - s.v[1461]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1646] != 0.0)) {
            s.store_exp_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1646] != 0.0))) {
            s.store_scale_ad(1338, A::offset(A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1348, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1647] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((s.v[1604] != 0.0) && (s.v[1647] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1648] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
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
        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1647] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1649] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1649] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1649] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((s.v[1604] != 0.0) && (s.v[1649] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1650] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1649] != 0.0))) && (s.v[1650] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1649] != 0.0))) && (s.v[1650] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1649] != 0.0))) && (s.v[1650] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1649] != 0.0))) && (!(s.v[1650] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1649] != 0.0))) && (!(s.v[1650] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1651] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((s.v[1604] != 0.0) && (s.v[1651] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((s.v[1604] != 0.0) && (s.v[1651] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1651] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1651] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1651] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1651] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1651] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1652] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1652] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1652] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1652] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1652] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1652] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1652] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1461);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1652] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1652] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1372, 1457, 1369);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1461, 1461, 1376);
        }

        s.v[1653] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        s.v[1654] = if (((s.v[1376]) as f64).abs() > 0.01) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_mul(1347, 1456, 1461);
        }

        s.v[1655] = if ((s.v[1454] - s.v[1461]) < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1655] != 0.0)) {
            s.store_exp_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1655] != 0.0))) {
            s.store_scale_ad(1338, A::offset(A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_mul(1348, 1429, 1338);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1656] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1656] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1657] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (s.v[1657] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1656] != 0.0))) && (!(s.v[1657] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1658] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1658] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1659] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1658] != 0.0))) && (s.v[1659] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1658] != 0.0))) && (s.v[1659] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1658] != 0.0))) && (s.v[1659] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1658] != 0.0))) && (!(s.v[1659] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1658] != 0.0))) && (!(s.v[1659] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1660] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1660] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1660] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1660] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1660] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1660] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1660] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1660] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1661] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1661] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1661] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1661] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (s.v[1661] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1661] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1661] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1461);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1661] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) && (!(s.v[1661] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_mul(1372, 1457, 1369);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (((s.v[1604] != 0.0) && (s.v[1653] != 0.0)) && (s.v[1654] != 0.0)) {
            s.store_add(1461, 1461, 1376);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1463, 1456, 1461);
        }

        s.v[1662] = if ((s.v[1454] - s.v[1461]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1662] != 0.0)) {
            s.store_exp_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1662] != 0.0))) {
            s.store_scale_ad(1338, A::offset(A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1467, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1466, A::square(s.ad_value(1463)), 1467);
        }

        s.v[1663] = if (s.v[1467] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1663] != 0.0)) {
            s.store_scalar(1462, 1e-80);
        }

        if ((s.v[1604] != 0.0) && (s.v[1663] != 0.0)) {
            s.store_sub(1464, 1462, 1463);
        }

        if ((s.v[1604] != 0.0) && (s.v[1663] != 0.0)) {
            s.store_div(1465, 1464, 1457);
        }

        s.v[1664] = if (s.v[1466] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1664] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1466)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1664] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        s.v[1665] = if (s.v[1466] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1664] != 0.0))) && (s.v[1665] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1466)));
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1664] != 0.0))) && (s.v[1665] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1664] != 0.0))) && (s.v[1665] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
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
        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1664] != 0.0))) && (!(s.v[1665] != 0.0))) {
            s.store_offset_ad(1353, A::mul(A::scale(s.ad_value(1466), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1466), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0238095238095))))), 2.0);
        }

        s.v[1666] = if (((1.01 * s.v[1463]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) {
            s.store_add(1338, 1463, 1353);
        }

        s.v[1667] = if ((s.v[1467] * s.v[1463]) < (((0.9 * s.v[1463]) * s.v[1463]) * s.v[1338])) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (s.v[1667] != 0.0)) {
            s.store_offset_ad(1462, A::div(s.ad_value(1467), s.ad_value(1338)), 1e-80);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (s.v[1667] != 0.0)) {
            s.store_sub(1464, 1462, 1463);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (s.v[1667] != 0.0)) {
            s.store_div(1465, 1464, 1457);
        }

        s.v[1668] = if (s.v[1466] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) && (s.v[1668] != 0.0)) {
            s.store_sub_ad_lhs(1339, A::ln(A::div(A::scale(s.ad_value(1466), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))))), 1352);
        }

        s.v[1669] = if (s.v[1466] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) && (!(s.v[1668] != 0.0))) && (s.v[1669] != 0.0)) {
            s.store_sin_ad(1340, A::scale(s.ad_value(1352), 0.5));
        }

        if ((((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) && (!(s.v[1668] != 0.0))) && (s.v[1669] != 0.0)) {
            s.store_ln_ad(1339, A::div(A::neg(s.ad_value(1466)), A::square(s.ad_value(1340))));
        }

        if ((((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) && (!(s.v[1668] != 0.0))) && (!(s.v[1669] != 0.0))) {
            s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1466), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1466), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0396825396825397)))))));
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) {
            s.store_sub_ad_lhs(1465, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(A::ln(s.ad_value(1338)), 2.0)), 1339);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) {
            s.store_mul(1464, 1457, 1465);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) {
            s.store_add(1462, 1463, 1464);
        }

        s.v[1670] = if (s.v[1466] > 0.005) { 1.0 } else { 0.0 };

        s.v[1671] = if (((s.v[1461] - s.v[1454]) - s.v[1352]) < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) && (s.v[1670] != 0.0)) && (s.v[1671] != 0.0)) {
            s.store_exp_ad(1340, A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)));
        }

        if (((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) {
            let assign34270_ad_e38321: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1340, assign34270_ad_e38321, 5.54062e34);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) && (s.v[1670] != 0.0)) {
            s.store_div(1339, 1340, 1429);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) && (s.v[1670] != 0.0)) {
            s.store_div_ad(1338, A::mul(A::scale(s.ad_value(1466), 4.0), s.ad_value(1339)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        s.v[1672] = if (s.v[1466] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) && (!(s.v[1670] != 0.0))) && (s.v[1672] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) && (!(s.v[1670] != 0.0))) && (s.v[1672] != 0.0)) {
            s.store_div_ad_lhs(1338, A::div(A::neg(s.ad_value(1466)), A::square(s.ad_value(1339))), 1467);
        }

        if (((((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) && (!(s.v[1670] != 0.0))) && (!(s.v[1672] != 0.0))) {
            s.store_div_ad_lhs(1338, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1466), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1466), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0396825396825397)))))), 1467);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) {
            s.store_offset_ad(1462, A::div(A::sub(s.ad_value(1463), s.ad_value(1353)), A::sub_from_scalar(1.0, s.ad_value(1338))), 1e-80);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) {
            s.store_sub(1464, 1462, 1463);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) {
            s.store_div(1465, 1464, 1457);
        }

        s.v[1673] = if ((s.v[1455] - s.v[1465]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1673] != 0.0)) {
            s.store_exp_ad(1338, A::sub(s.ad_value(1455), s.ad_value(1465)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1673] != 0.0))) {
            s.store_scale_ad(1338, A::offset(A::mul(A::offset(A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1468, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1471, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1472, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1469, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1470, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1473, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1474, 0.0);
        }

        s.v[1674] = if (s.v[1462] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) {
            s.store_mul(1469, 1467, 1430);
        }

        if ((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) {
            s.store_mul(1470, 1468, 1431);
        }

        if ((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) {
            s.store_add_ad_rhs(1471, 1469, A::scale(s.ad_value(1463), 2.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) {
            s.store_add_ad_rhs(1472, 1470, A::scale(s.ad_value(1464), 2.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) {
            s.store_add_ad_lhs(1473, A::add(A::scale(s.ad_value(1462), 2.0), s.ad_value(1469)), 1470);
        }

        s.v[1675] = if (((s.v[1466]) as f64).abs() > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) && (s.v[1675] != 0.0)) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(1471), s.ad_value(1472)), A::mul(A::scale(A::offset(s.ad_value(1461), 2.0), 2.0), s.ad_value(1472))), A::mul(A::scale(A::offset(s.ad_value(1465), 2.0), 2.0), s.ad_value(1471)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) && (s.v[1675] != 0.0)) {
            s.store_div_ad(1474, A::mul(A::scale(s.ad_value(1466), (-4.0)), s.ad_value(1473)), A::mul(s.ad_value(1462), s.ad_value(2)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) && (!(s.v[1675] != 0.0))) {
            s.store_scale_ad(2, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1466), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1466), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) && (!(s.v[1675] != 0.0))) {
            s.store_add_ad(3, A::add(A::mul(s.ad_value(1471), s.ad_value(1467)), A::mul(s.ad_value(1472), s.ad_value(1468))), A::mul(A::mul(A::mul(s.ad_value(1471), s.ad_value(1472)), s.ad_value(1462)), A::offset(A::mul(s.ad_value(1462), s.ad_value(2)), 1.0)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1674] != 0.0)) && (!(s.v[1675] != 0.0))) {
            s.store_div_ad(1474, A::mul(A::mul(s.ad_value(1467), s.ad_value(1468)), s.ad_value(1473)), A::mul(s.ad_value(1462), s.ad_value(3)));
        }

        if (s.v[1604] != 0.0) {
            s.store_ln(1475, 1462);
        }

        s.v[1676] = if ((s.v[1463] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1676] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::scale(s.ad_value(1463), 0.5)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1676] != 0.0))) {
            s.store_scale(2, 1463, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scale(1476, 2, 2.0);
        }

        s.v[1677] = if ((s.v[1464] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1677] != 0.0)) {
            s.store_ln_ad(3, A::offset(A::exp(A::scale(s.ad_value(1464), 0.5)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1677] != 0.0))) {
            s.store_scale(3, 1464, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scale(1477, 3, 2.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1478, 1477, 1464);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1479, 1476, 1463);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1480, A::mul(s.ad_value(266), s.ad_value(1476)), A::mul(s.ad_value(267), s.ad_value(1478)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1481, A::mul(s.ad_value(266), s.ad_value(1477)), A::mul(s.ad_value(267), s.ad_value(1479)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad_rhs(0, 1462, A::add(s.ad_value(1476), s.ad_value(1477)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1482, 1476, 0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1483, 1477, 0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(1484, A::mul(s.ad_value(1476), s.ad_value(187)), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(1485, A::mul(s.ad_value(1477), s.ad_value(188)), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(2, 50, A::add(s.ad_value(1478), A::mul(s.ad_value(51), s.ad_value(1479))));
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(3, A::add(A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(4, A::add(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_div(1486, 3, 4);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(1487, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(1478)), 1.0), A::mul(s.ad_value(42), s.ad_value(1479)))), A::exp(A::mul(A::neg(s.ad_value(44)), A::ln(A::add(A::offset(A::mul(s.ad_value(1482), s.ad_value(264)), 1.0), A::mul(s.ad_value(1483), s.ad_value(265)))))));
        }

        s.v[1678] = if (s.v[56] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1678] != 0.0)) {
            s.store_scalar(4, 1.0);
        }

        s.v[1679] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1678] != 0.0))) && (s.v[1679] != 0.0)) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1462), 1e-12)))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1678] != 0.0))) && (s.v[1679] != 0.0)) {
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1678] != 0.0))) && (!(s.v[1679] != 0.0))) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1462), 1e-12)))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1678] != 0.0))) && (!(s.v[1679] != 0.0))) {
            s.store_div_from_scalar_ad(4, 1.0, A::offset(s.ad_value(2), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(1488, A::scale(A::mul(s.ad_value(268), s.ad_value(1443)), 0.5), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424))), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424)))), 0.01))));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(1489, 1488, A::add(A::mul(s.ad_value(1462), s.ad_value(4)), s.ad_value(54)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1490, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1480)), 1e-6)))), 1.0), s.ad_value(1487)), A::mul(s.ad_value(38), s.ad_value(1489)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1491, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1481)), 1e-6)))), 1.0), s.ad_value(1487)), A::mul(s.ad_value(39), s.ad_value(1489)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1492, A::mul(s.ad_value(1486), A::add(s.ad_value(1484), s.ad_value(1485))), A::add(A::div(s.ad_value(1484), s.ad_value(1490)), A::div(s.ad_value(1485), s.ad_value(1491))));
        }

        s.v[1680] = if (((s.v[1459]) as f64).abs() > 0.007) { 1.0 } else { 0.0 };

        s.v[1681] = if (s.v[1459] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1681] != 0.0)) {
            s.store_exp_ad(0, A::neg(s.ad_value(1459)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1681] != 0.0)) {
            s.store_div_ad_rhs(1493, 1459, A::sub_from_scalar(1.0, s.ad_value(0)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1681] != 0.0)) {
            s.store_mul(1494, 0, 1493);
        }

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1681] != 0.0)) {
            s.store_add_ad_lhs(1495, A::offset(A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1493)))), (-0.6931471805599)), 1432);
        }

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (!(s.v[1681] != 0.0))) {
            s.store_exp(0, 1459);
        }

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (!(s.v[1681] != 0.0))) {
            s.store_div_ad_rhs(1494, 1459, A::offset(s.ad_value(0), (-1.0)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (!(s.v[1681] != 0.0))) {
            s.store_mul(1493, 0, 1494);
        }

        if (((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) && (!(s.v[1681] != 0.0))) {
            s.store_add_ad_lhs(1495, A::offset(A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1494)))), (-0.6931471805599)), 1433);
        }

        if ((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_div_ad(1496, A::neg(s.ad_value(1459)), A::mul(s.ad_value(1458), A::sub(A::sub_from_scalar(1.0, s.ad_value(1493)), A::mul(s.ad_value(1459), s.ad_value(1431)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_div_ad_rhs(1497, 1459, A::mul(s.ad_value(1458), A::add(A::sub_from_scalar(1.0, s.ad_value(1494)), A::mul(s.ad_value(1459), s.ad_value(1430)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_div_ad_rhs(1498, 1459, A::sub(A::div(A::offset(A::mul(s.ad_value(1494), s.ad_value(1431)), 0.5), s.ad_value(1497)), A::div(A::offset(A::mul(s.ad_value(1493), s.ad_value(1430)), 0.5), s.ad_value(1496))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_scale(0, 1460, (0.5 * 0.1666666666667));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_scale(2, 1459, 0.5);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_add_ad_lhs(1493, A::offset(s.ad_value(2), 1.0), 0);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_add_ad_lhs(1494, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_scale(3, 2, 0.1666666666667);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_div_from_scalar_ad(1496, 1.0, A::mul(s.ad_value(1458), A::add(A::offset(s.ad_value(1431), 0.5), s.ad_value(3))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_div_from_scalar_ad(1497, 1.0, A::mul(s.ad_value(1458), A::sub(A::offset(s.ad_value(1430), 0.5), s.ad_value(3))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            s.store_add_ad(1495, A::offset(A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), A::sub_from_scalar(1.0, A::scale(s.ad_value(0), 0.5))))), (-0.6931471805599)), A::scale(A::add(s.ad_value(1432), s.ad_value(1433)), 0.5));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1680] != 0.0))) {
            let assign35150_ad_e39462: A = A::add(A::add(A::add(A::sub_from_scalar(4.0, A::scale(s.ad_value(1458), 3.0)), A::div(A::scale(s.ad_value(1458), 12.0), A::mul(s.ad_value(1456), s.ad_value(1457)))), A::mul(A::mul(s.ad_value(1458), A::sub(s.ad_value(1430), s.ad_value(1431))), s.ad_value(1459))), A::mul(A::scale(A::sub_from_scalar(0.2, A::scale(s.ad_value(1458), 0.25)), 0.3333333333333), s.ad_value(1460)));
            s.store_div_from_scalar_ad(1498, (-12.0), assign35150_ad_e39462);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar(1499, 1.0, 1498);
        }

        s.v[1682] = if (s.v[1462] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_div_ad(1500, A::scale(s.ad_value(1476), 100.0), A::offset(s.ad_value(1476), 100.0));
        }

        s.v[1683] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1683] != 0.0)) {
            s.store_div_from_scalar_ad(1501, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(61), s.ad_value(1500))));
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (!(s.v[1683] != 0.0))) {
            s.store_offset_ad(1501, A::mul(s.ad_value(61), s.ad_value(1500)), 1.0);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_div_ad(1502, A::scale(s.ad_value(1477), 100.0), A::offset(s.ad_value(1477), 100.0));
        }

        s.v[1684] = if (s.v[62] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1684] != 0.0)) {
            s.store_div_from_scalar_ad(1503, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(62), s.ad_value(1502))));
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (!(s.v[1684] != 0.0))) {
            s.store_offset_ad(1503, A::mul(s.ad_value(62), s.ad_value(1502)), 1.0);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_sub_ad(1504, A::div(A::mul(s.ad_value(1474), s.ad_value(1473)), A::mul(s.ad_value(1471), s.ad_value(1472))), A::div(A::add(A::div(s.ad_value(1467), s.ad_value(1471)), A::div(s.ad_value(1468), s.ad_value(1472))), s.ad_value(1462)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_div_ad(1505, A::mul(s.ad_value(1504), s.ad_value(1462)), A::offset(s.ad_value(1504), 1.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_sub(2, 1498, 1505);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_div_ad_lhs(1506, A::add(s.ad_value(1462), A::mul(s.ad_value(1498), s.ad_value(1495))), 2);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_scale_ad(1506, A::add(s.ad_value(1506), A::sqrt(A::offset(A::square(s.ad_value(1506)), 1e-6))), 0.5);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_mul_ad(1507, A::scale(A::div(s.ad_value(1420), s.ad_value(1492)), 0.5), A::add(s.ad_value(1501), s.ad_value(1503)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_sub_from_scalar_ad(1508, 1.0, A::div(s.ad_value(1462), s.ad_value(1505)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_offset(1509, 1495, 1.0);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_mul_ad_lhs(1510, A::sub(A::offset(A::mul(A::sub(A::scale(s.ad_value(1505), 2.0), s.ad_value(1462)), s.ad_value(1499)), (-2.0)), s.ad_value(1495)), 1506);
        }

        s.v[1685] = if (s.v[1507] > 1e-14) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_div_from_scalar_ad(1511, 2.0, A::square(s.ad_value(1507)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_mul(1512, 1511, 1508);
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_add(1513, 1511, 1510);
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_mul(1514, 1511, 1509);
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_sqrt_ad(1515, A::offset(A::add(A::square(s.ad_value(1512)), A::mul(A::mul(A::scale(s.ad_value(1511), 0.148148148148), s.ad_value(1511)), s.ad_value(1511))), 1e-20));
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_sqrt_ad(1516, A::offset(A::add(A::square(s.ad_value(1514)), A::mul(A::mul(A::scale(s.ad_value(1513), 0.148148148148), s.ad_value(1513)), s.ad_value(1513))), 1e-20));
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_sub_ad(1517, A::exp(A::scale(A::ln(A::scale(A::add(s.ad_value(1515), s.ad_value(1512)), 0.5)), 0.3333333333333)), A::exp(A::scale(A::ln(A::scale(A::sub(s.ad_value(1515), s.ad_value(1512)), 0.5)), 0.3333333333333)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_sub_ad(1518, A::exp(A::scale(A::ln(A::scale(A::add(s.ad_value(1516), s.ad_value(1514)), 0.5)), 0.3333333333333)), A::exp(A::scale(A::ln(A::scale(A::sub(s.ad_value(1516), s.ad_value(1514)), 0.5)), 0.3333333333333)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.copy_ad(1517, 1508);
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
        if (((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.copy_ad(1518, 1509);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_square(4, 2);
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_scale_ad(1519, A::add(A::add(s.ad_value(1517), s.ad_value(1518)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(1517), s.ad_value(1518)), A::sub(s.ad_value(1517), s.ad_value(1518))), A::scale(s.ad_value(4), 10.0)))), (0.94 * 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_add_ad_rhs(1520, 1462, A::mul(s.ad_value(1505), s.ad_value(1519)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_mul_ad_rhs(1521, 1498, A::sub(s.ad_value(1519), s.ad_value(1495)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1682] != 0.0)) {
            s.store_scale_ad(1522, A::add(A::add(s.ad_value(1520), s.ad_value(1521)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(1520), s.ad_value(1521)), A::sub(s.ad_value(1520), s.ad_value(1521))), A::scale(s.ad_value(4), 36.0)))), 0.5);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1682] != 0.0))) {
            s.copy_ad(1505, 1498);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1682] != 0.0))) {
            s.store_scaled_offset(1519, 1495, 1.0, 0.94);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1682] != 0.0))) {
            s.store_add_ad(1522, A::scale(s.ad_value(1462), 0.5), A::mul(s.ad_value(1498), A::sub(s.ad_value(1519), A::scale(s.ad_value(1495), 0.5))));
        }

        s.v[1686] = if ((s.v[1522] - 0.5) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1686] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::offset(s.ad_value(1522), (-0.5))), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_offset(2, 1522, (-0.5));
        }

        if (s.v[1604] != 0.0) {
            s.store_offset(3, 2, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(4, 1519, A::ln(A::div(s.ad_value(1462), s.ad_value(3))));
        }

        s.v[1687] = if ((s.v[4] - 6.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1687] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::offset(s.ad_value(4), (-6.0))), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1687] != 0.0))) {
            s.store_offset(2, 4, (-6.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_offset(4, 2, 6.0);
        }

        s.v[1688] = if ((s.v[221] - s.v[4]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1688] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::sub(s.ad_value(221), s.ad_value(4))), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1688] != 0.0))) {
            s.store_sub(2, 221, 4);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1523, 221, 2);
        }

        if (s.v[1604] != 0.0) {
            s.store_div(2, 335, 1523);
        }

        if (s.v[1604] != 0.0) {
            s.store_square(3, 2);
        }

        if (s.v[1604] != 0.0) {
            s.store_square(4, 3);
        }

        if (s.v[1604] != 0.0) {
            s.store_square(5, 4);
        }

        if (s.v[1604] != 0.0) {
            s.store_exp_ad(0, A::scale(A::ln(A::offset(A::mul(s.ad_value(1421), s.ad_value(4)), 1.0)), 2.666666666667));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(1524, 335, A::exp(A::scale(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625))));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1338, 1.0, A::offset(s.ad_value(1456), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1339, 1.0, A::offset(s.ad_value(1457), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_ad(1341, A::add(A::ln(A::div(A::mul(A::add(s.ad_value(1456), A::mul(s.ad_value(1457), s.ad_value(1339))), s.ad_value(1428)), s.ad_value(1429))), s.ad_value(1524)), 3.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_ad(1342, A::add(A::ln(A::div(A::mul(A::add(s.ad_value(1457), A::mul(s.ad_value(1456), s.ad_value(1338))), s.ad_value(1428)), s.ad_value(1429))), s.ad_value(1524)), 3.0);
        }

        s.v[1689] = if (((s.v[1341] - s.v[1432]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1689] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1432)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1689] != 0.0))) {
            s.store_scaled_sub(1340, 1341, 1432, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.v[1690] = if (((s.v[1342] - s.v[1433]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1690] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1433)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1690] != 0.0))) {
            s.store_scaled_sub(1340, 1342, 1433, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1343, A::add(A::mul(s.ad_value(1456), s.ad_value(1454)), s.ad_value(1346)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1344, A::add(A::mul(s.ad_value(1457), s.ad_value(1455)), s.ad_value(1345)), 1339);
        }

        s.v[1691] = if (((s.v[1341] - s.v[1343]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1691] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1343)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1691] != 0.0))) {
            s.store_scaled_sub(1340, 1341, 1343, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.v[1692] = if (((s.v[1342] - s.v[1344]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1692] != 0.0)) {
            s.store_ln_ad(1340, A::offset(A::exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1344)), 0.3333333333333)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1692] != 0.0))) {
            s.store_scaled_sub(1340, 1342, 1344, 0.3333333333333);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1525, 1454, 1345);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1526, 1455, 1346);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1352, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1355, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1525);
        }

        s.v[1693] = if (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1693] != 0.0)) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1693] != 0.0))) {
            let assign36020_ad_e40413: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign36020_ad_e40413, 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1348, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1694] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((s.v[1604] != 0.0) && (s.v[1694] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1695] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (s.v[1695] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1694] != 0.0))) && (!(s.v[1695] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1696] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1696] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1696] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((s.v[1604] != 0.0) && (s.v[1696] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1697] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1696] != 0.0))) && (s.v[1697] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1696] != 0.0))) && (s.v[1697] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1696] != 0.0))) && (s.v[1697] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1696] != 0.0))) && (!(s.v[1697] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1696] != 0.0))) && (!(s.v[1697] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1698] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((s.v[1604] != 0.0) && (s.v[1698] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((s.v[1604] != 0.0) && (s.v[1698] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1698] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1698] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1698] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1698] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1698] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1699] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1699] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1699] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1699] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1699] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1699] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1699] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1525);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1699] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1699] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1372, 1457, 1369);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1525, 1525, 1376);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1525);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1377, 1457, 1526);
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1370, 1347, 1377);
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
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
        if (s.v[1604] != 0.0) {
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
        }

        if (s.v[1604] != 0.0) {
            s.store_sqrt_ad(1381, A::sub(A::square(s.ad_value(1379)), A::mul(A::scale(s.ad_value(1378), 4.0), s.ad_value(1380))));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.v[1700] = if (s.v[1382] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1700] != 0.0)) {
            s.store_mul_ad_rhs(1373, 1382, A::add(A::sub(A::add(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1525)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1700] != 0.0)) {
            s.store_add_ad_lhs(1374, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1382);
        }

        if ((s.v[1604] != 0.0) && (s.v[1700] != 0.0)) {
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1525)), 1341);
        }

        s.v[1701] = if ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1700] != 0.0)) && (s.v[1701] != 0.0)) {
            s.store_sub_ad_rhs(1525, 1525, A::div(s.ad_value(1373), s.ad_value(1374)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1525);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1377, 1457, 1526);
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1370, 1347, 1377);
        }

        if (s.v[1604] != 0.0) {
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
        }

        if (s.v[1604] != 0.0) {
            s.store_sqrt_ad(1381, A::sub(A::square(s.ad_value(1379)), A::mul(A::scale(s.ad_value(1378), 4.0), s.ad_value(1380))));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
        }

        s.v[1702] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1702] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1702] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1702] != 0.0)) {
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        s.v[1703] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1702] != 0.0))) && (s.v[1703] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1702] != 0.0))) && (s.v[1703] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1702] != 0.0))) && (s.v[1703] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1702] != 0.0))) && (s.v[1703] != 0.0)) {
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1702] != 0.0))) && (!(s.v[1703] != 0.0))) {
            s.store_offset_ad(1353, A::mul(A::scale(s.ad_value(1349), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0238095238095))))), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1702] != 0.0))) && (!(s.v[1703] != 0.0))) {
            s.store_scale_ad(1354, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_rhs(1349, 1349, A::div(A::add(A::add(A::mul(s.ad_value(1370), s.ad_value(1353)), A::mul(s.ad_value(1347), s.ad_value(1377))), s.ad_value(1349)), A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0)));
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.v[1704] = if (s.v[1382] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1704] != 0.0)) {
            s.store_mul_ad_rhs(1373, 1382, A::add(A::sub(A::add(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1525)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1704] != 0.0)) {
            s.store_add_ad_lhs(1374, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1382);
        }

        if ((s.v[1604] != 0.0) && (s.v[1704] != 0.0)) {
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1525)), 1341);
        }

        s.v[1705] = if ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1704] != 0.0)) && (s.v[1705] != 0.0)) {
            s.store_sub_ad_rhs(1525, 1525, A::div(s.ad_value(1373), s.ad_value(1374)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1525);
        }

        s.v[1706] = if (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1706] != 0.0)) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1706] != 0.0))) {
            let assign37190_ad_e41846: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign37190_ad_e41846, 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1348, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1707] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((s.v[1604] != 0.0) && (s.v[1707] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1708] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (s.v[1708] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1707] != 0.0))) && (!(s.v[1708] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1709] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1709] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1709] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((s.v[1604] != 0.0) && (s.v[1709] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1710] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1709] != 0.0))) && (s.v[1710] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1709] != 0.0))) && (s.v[1710] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1709] != 0.0))) && (s.v[1710] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1709] != 0.0))) && (!(s.v[1710] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1709] != 0.0))) && (!(s.v[1710] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1711] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((s.v[1604] != 0.0) && (s.v[1711] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((s.v[1604] != 0.0) && (s.v[1711] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1711] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1711] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1711] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1711] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1711] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1712] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1712] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1712] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1525);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1712] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1712] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1372, 1457, 1369);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1525, 1525, 1376);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1347, 1456, 1525);
        }

        s.v[1713] = if (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1713] != 0.0)) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1713] != 0.0))) {
            let assign37940_ad_e42795: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign37940_ad_e42795, 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1348, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1714] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((s.v[1604] != 0.0) && (s.v[1714] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1715] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
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
        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (s.v[1715] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1714] != 0.0))) && (!(s.v[1715] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1716] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1716] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1716] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((s.v[1604] != 0.0) && (s.v[1716] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1717] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1716] != 0.0))) && (s.v[1717] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1716] != 0.0))) && (s.v[1717] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1716] != 0.0))) && (s.v[1717] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1716] != 0.0))) && (!(s.v[1717] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1716] != 0.0))) && (!(s.v[1717] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1718] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((s.v[1604] != 0.0) && (s.v[1718] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((s.v[1604] != 0.0) && (s.v[1718] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1718] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1718] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1718] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1718] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1718] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1719] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1719] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1719] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((s.v[1604] != 0.0) && (s.v[1719] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((s.v[1604] != 0.0) && (s.v[1719] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1719] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1719] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1525);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1719] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1719] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1372, 1457, 1369);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1525, 1525, 1376);
        }

        s.v[1720] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        s.v[1721] = if (((s.v[1376]) as f64).abs() > 0.01) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_mul(1347, 1456, 1525);
        }

        s.v[1722] = if (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1722] != 0.0)) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1722] != 0.0))) {
            let assign38710_ad_e43763: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign38710_ad_e43763, 5.54062e34);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_mul(1348, 1429, 1338);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_add_ad_lhs(1350, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1347)), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad_lhs(1351, A::mul(A::scale(s.ad_value(1456), 2.0), s.ad_value(1456)), 1348);
        }

        s.v[1723] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1723] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.v[1724] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1349)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_div_ad_lhs(1338, A::scale(s.ad_value(1350), 0.25), 1349);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 1338);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul(A::scale(s.ad_value(1354), 2.0), A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_mul_ad_lhs(1359, A::div(s.ad_value(1350), s.ad_value(1349)), 1339);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (s.v[1724] != 0.0)) {
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_scale_ad(1340, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025)))))), 0.1666666666667);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_offset_ad(1353, A::mul(s.ad_value(1349), s.ad_value(1340)), 2.0);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_scale_ad(1338, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_mul(1354, 1350, 1338);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_scale_ad(1339, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421)))))), 0.0055555555556);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_mul_ad_lhs(1359, A::scale(s.ad_value(1350), (-0.5)), 1340);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1723] != 0.0))) && (!(s.v[1724] != 0.0))) {
            s.store_add_ad(1360, A::mul(A::scale(s.ad_value(1351), (-0.5)), s.ad_value(1340)), A::mul(A::mul(A::scale(s.ad_value(1350), (0.25 * 0.0055555555556)), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.v[1725] = if (s.v[1349] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_mul(1357, 1339, 1355);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1725] != 0.0)) {
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.v[1726] = if (s.v[1349] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1725] != 0.0))) && (s.v[1726] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1725] != 0.0))) && (s.v[1726] != 0.0)) {
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1725] != 0.0))) && (s.v[1726] != 0.0)) {
            s.store_ln(1358, 1357);
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1725] != 0.0))) && (!(s.v[1726] != 0.0))) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul(A::scale(s.ad_value(1349), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1349), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
        }

        if (((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1725] != 0.0))) && (!(s.v[1726] != 0.0))) {
            s.store_ln(1358, 1357);
        }

        s.v[1727] = if (((1.01 * s.v[1347]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_add(1361, 1347, 1353);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1727] != 0.0)) {
            s.store_add(1362, 1456, 1354);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1727] != 0.0)) {
            s.copy_ad(1363, 1356);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1727] != 0.0))) {
            s.store_div_from_scalar_ad(1339, 1.0, A::sub(s.ad_value(1347), s.ad_value(1353)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1727] != 0.0))) {
            s.store_sub(1340, 1354, 1456);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1727] != 0.0))) {
            s.store_mul_ad_lhs(1361, A::sub(s.ad_value(1348), s.ad_value(1357)), 1339);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1727] != 0.0))) {
            s.store_mul_ad_lhs(1362, A::sub(A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357))), 1339);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1727] != 0.0))) {
            s.store_mul_ad_lhs(1363, A::sub(A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1340), 2.0), s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357))), 1339);
        }

        s.v[1728] = if (s.v[1361] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1728] != 0.0)) {
            s.store_ln(1364, 1361);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1728] != 0.0)) {
            s.store_div_from_scalar(1338, 1.0, 1361);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1728] != 0.0)) {
            s.store_mul(1365, 1362, 1338);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (s.v[1728] != 0.0)) {
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1728] != 0.0))) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln(A::neg(s.ad_value(1347))));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1728] != 0.0))) {
            s.store_div_from_scalar(1338, 1.0, 1525);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1728] != 0.0))) {
            s.store_add(1365, 1456, 1338);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) && (!(s.v[1728] != 0.0))) {
            s.store_mul_ad_lhs(1366, A::neg(s.ad_value(1338)), 1338);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_mul(1372, 1457, 1369);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul(A::scale(s.ad_value(1371), 2.0), s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul(A::scale(s.ad_value(1373), 0.5), s.ad_value(1375)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_div_ad(1376, A::mul(A::mul(A::neg(s.ad_value(1373)), s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
        }

        if (((s.v[1604] != 0.0) && (s.v[1720] != 0.0)) && (s.v[1721] != 0.0)) {
            s.store_add(1525, 1525, 1376);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1528, 1456, 1525);
        }

        s.v[1729] = if (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1729] != 0.0)) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1729] != 0.0))) {
            let assign39460_ad_e44972: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign39460_ad_e44972, 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1531, 1429, 1338);
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
        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1530, A::square(s.ad_value(1528)), 1531);
        }

        s.v[1730] = if (s.v[1531] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1730] != 0.0)) {
            s.store_scalar(1527, 1e-80);
        }

        if ((s.v[1604] != 0.0) && (s.v[1730] != 0.0)) {
            s.store_sub(1529, 1527, 1528);
        }

        if ((s.v[1604] != 0.0) && (s.v[1730] != 0.0)) {
            s.store_div(1526, 1529, 1457);
        }

        s.v[1731] = if (s.v[1530] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1731] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1530)));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1731] != 0.0)) {
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        s.v[1732] = if (s.v[1530] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1731] != 0.0))) && (s.v[1732] != 0.0)) {
            s.store_sqrt_ad(1352, A::abs(s.ad_value(1530)));
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1731] != 0.0))) && (s.v[1732] != 0.0)) {
            s.store_exp_ad(1355, A::neg(s.ad_value(1352)));
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1731] != 0.0))) && (s.v[1732] != 0.0)) {
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1731] != 0.0))) && (!(s.v[1732] != 0.0))) {
            s.store_offset_ad(1353, A::mul(A::scale(s.ad_value(1530), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1530), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0238095238095))))), 2.0);
        }

        s.v[1733] = if (((1.01 * s.v[1528]) + s.v[1353]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) {
            s.store_add(1338, 1528, 1353);
        }

        s.v[1734] = if ((s.v[1531] * s.v[1528]) < (((0.9 * s.v[1528]) * s.v[1528]) * s.v[1338])) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (s.v[1734] != 0.0)) {
            s.store_offset_ad(1527, A::div(s.ad_value(1531), s.ad_value(1338)), 1e-80);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (s.v[1734] != 0.0)) {
            s.store_sub(1529, 1527, 1528);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (s.v[1734] != 0.0)) {
            s.store_div(1526, 1529, 1457);
        }

        s.v[1735] = if (s.v[1530] > 0.005) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (!(s.v[1734] != 0.0))) && (s.v[1735] != 0.0)) {
            s.store_sub_ad_lhs(1339, A::ln(A::div(A::scale(s.ad_value(1530), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))))), 1352);
        }

        s.v[1736] = if (s.v[1530] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (!(s.v[1734] != 0.0))) && (!(s.v[1735] != 0.0))) && (s.v[1736] != 0.0)) {
            s.store_sin_ad(1340, A::scale(s.ad_value(1352), 0.5));
        }

        if ((((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (!(s.v[1734] != 0.0))) && (!(s.v[1735] != 0.0))) && (s.v[1736] != 0.0)) {
            s.store_ln_ad(1339, A::div(A::neg(s.ad_value(1530)), A::square(s.ad_value(1340))));
        }

        if ((((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (!(s.v[1734] != 0.0))) && (!(s.v[1735] != 0.0))) && (!(s.v[1736] != 0.0))) {
            s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1530), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1530), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0396825396825397)))))));
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (!(s.v[1734] != 0.0))) {
            s.store_sub_ad_lhs(1526, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(A::ln(s.ad_value(1338)), 2.0)), 1339);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (!(s.v[1734] != 0.0))) {
            s.store_mul(1529, 1457, 1526);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (s.v[1733] != 0.0)) && (!(s.v[1734] != 0.0))) {
            s.store_add(1527, 1528, 1529);
        }

        s.v[1737] = if (s.v[1530] > 0.005) { 1.0 } else { 0.0 };

        s.v[1738] = if ((((s.v[1525] + s.v[1524]) - s.v[1454]) - s.v[1352]) < 80.0) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) && (s.v[1737] != 0.0)) && (s.v[1738] != 0.0)) {
            s.store_exp_ad(1340, A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)));
        }

        if (((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) && (s.v[1737] != 0.0)) && (!(s.v[1738] != 0.0))) {
            let assign39790_ad_e45440: A = A::mul(A::offset(A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.3333333333333), 1.0)), 1.0));
            s.store_scale_ad(1340, A::offset(assign39790_ad_e45440, 1.0), 5.54062e34);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) && (s.v[1737] != 0.0)) {
            s.store_div(1339, 1340, 1429);
        }

        if ((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) && (s.v[1737] != 0.0)) {
            s.store_div_ad(1338, A::mul(A::scale(s.ad_value(1530), 4.0), s.ad_value(1339)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        s.v[1739] = if (s.v[1530] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) && (!(s.v[1737] != 0.0))) && (s.v[1739] != 0.0)) {
            s.store_sin_ad(1339, A::scale(s.ad_value(1352), 0.5));
        }

        if (((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) && (!(s.v[1737] != 0.0))) && (s.v[1739] != 0.0)) {
            s.store_div_ad_lhs(1338, A::div(A::neg(s.ad_value(1530)), A::square(s.ad_value(1339))), 1531);
        }

        if (((((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) && (!(s.v[1737] != 0.0))) && (!(s.v[1739] != 0.0))) {
            s.store_div_ad_lhs(1338, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(1530), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1530), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0396825396825397)))))), 1531);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) {
            s.store_offset_ad(1527, A::div(A::sub(s.ad_value(1528), s.ad_value(1353)), A::sub_from_scalar(1.0, s.ad_value(1338))), 1e-80);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) {
            s.store_sub(1529, 1527, 1528);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1730] != 0.0))) && (!(s.v[1733] != 0.0))) {
            s.store_div(1526, 1529, 1457);
        }

        s.v[1740] = if (((s.v[1455] - s.v[1526]) - s.v[1524]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1740] != 0.0)) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1740] != 0.0))) {
            let assign39910_ad_e45658: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign39910_ad_e45658, 5.54062e34);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1532, 1429, 1338);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1535, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1536, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1533, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1534, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1537, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1538, 0.0);
        }

        s.v[1741] = if (s.v[1462] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_mul(1533, 1531, 1430);
        }

        if ((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_mul(1534, 1532, 1431);
        }

        if ((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_add_ad_rhs(1535, 1533, A::scale(s.ad_value(1528), 2.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_add_ad_rhs(1536, 1534, A::scale(s.ad_value(1529), 2.0));
        }

        if ((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_add_ad_lhs(1537, A::add(A::scale(s.ad_value(1527), 2.0), s.ad_value(1533)), 1534);
        }

        s.v[1742] = if (((s.v[1530]) as f64).abs() > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1742] != 0.0)) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(1535), s.ad_value(1536)), A::mul(A::scale(A::offset(s.ad_value(1525), 2.0), 2.0), s.ad_value(1536))), A::mul(A::scale(A::offset(s.ad_value(1526), 2.0), 2.0), s.ad_value(1535)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1742] != 0.0)) {
            s.store_div_ad(1538, A::mul(A::scale(s.ad_value(1530), (-4.0)), s.ad_value(1537)), A::mul(s.ad_value(1527), s.ad_value(2)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1742] != 0.0))) {
            s.store_scale_ad(2, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1530), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1530), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0333333333333)))))), 0.1666666666667);
        }

        if (((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1742] != 0.0))) {
            s.store_add_ad(3, A::add(A::mul(s.ad_value(1535), s.ad_value(1531)), A::mul(s.ad_value(1536), s.ad_value(1532))), A::mul(A::mul(A::mul(s.ad_value(1535), s.ad_value(1536)), s.ad_value(1527)), A::offset(A::mul(s.ad_value(1527), s.ad_value(2)), 1.0)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1742] != 0.0))) {
            s.store_div_ad(1538, A::mul(A::mul(s.ad_value(1531), s.ad_value(1532)), s.ad_value(1537)), A::mul(s.ad_value(1527), s.ad_value(3)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad_rhs(1539, 1524, A::ln(s.ad_value(1527)));
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_add(1540, 1462, 1527, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1541, 1539, 1475);
        }

        if (s.v[1604] != 0.0) {
            s.store_scalar(1544, 1.0);
        }

        s.v[1743] = if (p.p9 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1743] != 0.0)) {
            s.store_div_ad_lhs(1542, A::scale(A::add(s.ad_value(1463), s.ad_value(1528)), 0.5), 1456);
        }

        if ((s.v[1604] != 0.0) && (s.v[1743] != 0.0)) {
            s.store_scale_ad(1542, A::add(A::offset(s.ad_value(1542), 1e-5), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1542), (-1e-5)), A::offset(s.ad_value(1542), (-1e-5))), 1.0))), 0.5);
        }

        if ((s.v[1604] != 0.0) && (s.v[1743] != 0.0)) {
            s.store_sub_ad(1, A::sqrt(A::add(A::div(s.ad_value(1542), s.ad_value(223)), A::mul(A::scale(s.ad_value(246), 0.25), s.ad_value(246)))), A::scale(s.ad_value(246), 0.5));
        }

        if ((s.v[1604] != 0.0) && (s.v[1743] != 0.0)) {
            s.store_mul_ad_lhs(1543, A::powf(s.ad_value(1), 2.0), 223);
        }

        if ((s.v[1604] != 0.0) && (s.v[1743] != 0.0)) {
            s.store_sub_from_scalar_ad(1544, 1.0, A::div(s.ad_value(1543), s.ad_value(1542)));
        }

        s.v[1744] = if ((s.v[1528] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1744] != 0.0)) {
            s.store_ln_ad(2, A::offset(A::exp(A::scale(s.ad_value(1528), 0.5)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1744] != 0.0))) {
            s.store_scale(2, 1528, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scale(1545, 2, 2.0);
        }

        s.v[1745] = if ((s.v[1529] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1745] != 0.0)) {
            s.store_ln_ad(3, A::offset(A::exp(A::scale(s.ad_value(1529), 0.5)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1745] != 0.0))) {
            s.store_scale(3, 1529, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scale(1546, 3, 2.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1547, 1546, 1529);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub(1548, 1545, 1528);
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1549, A::mul(s.ad_value(266), s.ad_value(1545)), A::mul(s.ad_value(267), s.ad_value(1547)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1550, A::mul(s.ad_value(266), s.ad_value(1546)), A::mul(s.ad_value(267), s.ad_value(1548)));
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_add(1551, 1476, 1545, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_add(1552, 1477, 1546, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(0, 1.0, A::add(s.ad_value(1551), s.ad_value(1552)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1553, A::mul(s.ad_value(1540), s.ad_value(1551)), 0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1554, A::mul(s.ad_value(1540), s.ad_value(1552)), 0);
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_add(1555, 1478, 1547, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_add(1556, 1479, 1548, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_add(1557, 1480, 1549, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_add(1558, 1481, 1550, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1559, A::mul(A::mul(s.ad_value(1551), s.ad_value(187)), A::exp(A::mul(s.ad_value(40), s.ad_value(291)))), 1544);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(1560, A::mul(s.ad_value(1552), s.ad_value(188)), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
        }

        if (s.v[1604] != 0.0) {
            s.store_add(1561, 1559, 1560);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(2, 50, A::add(s.ad_value(1555), A::mul(s.ad_value(51), s.ad_value(1556))));
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(3, A::add(A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scale_ad(4, A::add(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01))), 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_div(1562, 3, 4);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(1563, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(1555)), 1.0), A::mul(s.ad_value(42), s.ad_value(1556)))), A::exp(A::mul(A::neg(s.ad_value(44)), A::ln(A::add(A::offset(A::mul(s.ad_value(1553), s.ad_value(264)), 1.0), A::mul(s.ad_value(1554), s.ad_value(265)))))));
        }

        s.v[1746] = if (s.v[56] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1746] != 0.0)) {
            s.store_scalar(4, 1.0);
        }

        s.v[1747] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (!(s.v[1746] != 0.0))) && (s.v[1747] != 0.0)) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1540), 1e-12)))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1746] != 0.0))) && (s.v[1747] != 0.0)) {
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1746] != 0.0))) && (!(s.v[1747] != 0.0))) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1540), 1e-12)))));
        }

        if (((s.v[1604] != 0.0) && (!(s.v[1746] != 0.0))) && (!(s.v[1747] != 0.0))) {
            s.store_div_from_scalar_ad(4, 1.0, A::offset(s.ad_value(2), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_rhs(1564, 1488, A::add(A::mul(s.ad_value(1540), s.ad_value(4)), s.ad_value(54)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1565, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1557)), 1e-6)))), 1.0), s.ad_value(1563)), A::mul(s.ad_value(38), s.ad_value(1564)));
        }

        if (s.v[1604] != 0.0) {
            s.store_add_ad(1566, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1558)), 1e-6)))), 1.0), s.ad_value(1563)), A::mul(s.ad_value(39), s.ad_value(1564)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1567, A::mul(s.ad_value(1562), s.ad_value(1561)), A::add(A::div(s.ad_value(1559), s.ad_value(1565)), A::div(s.ad_value(1560), s.ad_value(1566))));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1568, 1.0, A::offset(s.ad_value(1540), 4.0));
        }

        s.v[1748] = if (s.v[65] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1748] != 0.0)) {
            s.store_div_from_scalar_ad(0, 1.0, A::offset(A::mul(s.ad_value(65), s.ad_value(1554)), 1.0));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1748] != 0.0))) {
            s.store_sub_from_scalar_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1554)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1569, A::mul(s.ad_value(1540), s.ad_value(1568)), 0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(1570, A::ln(A::offset(A::div(A::sub(s.ad_value(335), s.ad_value(1524)), A::add(A::mul(s.ad_value(66), s.ad_value(223)), A::mul(A::mul(s.ad_value(67), s.ad_value(1540)), s.ad_value(1540)))), 1.0)), 1569);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1571, 1422, 1570);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_from_scalar_ad(1572, 1.0, A::offset(A::mul(s.ad_value(1571), A::offset(s.ad_value(1571), 1.0)), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1500, A::scale(s.ad_value(1551), 100.0), A::offset(s.ad_value(1551), 100.0));
        }

        s.v[1749] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1749] != 0.0)) {
            s.store_div_from_scalar_ad(1501, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(61), s.ad_value(1500))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1749] != 0.0))) {
            s.store_offset_ad(1501, A::mul(s.ad_value(61), s.ad_value(1500)), 1.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad(1502, A::scale(s.ad_value(1552), 100.0), A::offset(s.ad_value(1552), 100.0));
        }

        s.v[1750] = if (s.v[62] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1750] != 0.0)) {
            s.store_div_from_scalar_ad(1503, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(62), s.ad_value(1502))));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1750] != 0.0))) {
            s.store_offset_ad(1503, A::mul(s.ad_value(62), s.ad_value(1502)), 1.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad(1573, A::scale(A::mul(s.ad_value(1420), s.ad_value(1541)), 0.5), A::add(s.ad_value(1501), s.ad_value(1503)));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad_rhs(1574, 1573, A::mul(s.ad_value(1567), s.ad_value(1572)));
        }

        if (s.v[1604] != 0.0) {
            s.store_square(1575, 1574);
        }

        if (s.v[1604] != 0.0) {
            s.store_sqrt_ad(1576, A::offset(s.ad_value(1575), 1.0));
        }

        if (s.v[1604] != 0.0) {
            s.store_div_ad_lhs(1577, A::offset(A::scale(s.ad_value(1575), 1.5), 1.0), 1576);
        }

        s.v[1751] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1604] != 0.0) && (s.v[1751] != 0.0)) {
            s.store_mul_ad(2, A::scale(s.ad_value(254), 0.6), A::exp(A::scale(A::ln(A::offset(A::square(s.ad_value(1551)), 60.0)), (-0.1666666666667))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1751] != 0.0)) {
            s.store_mul_ad(3, A::scale(s.ad_value(254), 0.6), A::exp(A::scale(A::ln(A::offset(A::square(s.ad_value(1552)), 60.0)), (-0.1666666666667))));
        }

        if ((s.v[1604] != 0.0) && (s.v[1751] != 0.0)) {
            s.store_div_ad_lhs(1578, A::offset(A::mul(s.ad_value(1456), s.ad_value(2)), 1.0), 1437);
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
        if ((s.v[1604] != 0.0) && (s.v[1751] != 0.0)) {
            s.store_div_ad_lhs(1579, A::offset(A::mul(s.ad_value(1457), s.ad_value(3)), 1.0), 1438);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1751] != 0.0))) {
            s.store_scalar(1578, 1.0);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1751] != 0.0))) {
            s.store_scalar(1579, 1.0);
        }

        s.v[1752] = if (s.v[1462] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1753] = if (s.v[1527] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1754] = if (((s.v[1536]) as f64).abs() < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_div_ad(0, A::add(A::offset(s.ad_value(1525), 2.0), A::scale(s.ad_value(1535), 0.5)), A::mul(A::offset(s.ad_value(1526), 2.0), s.ad_value(1535)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_mul(2, 0, 1536);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_square(3, 2);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_sub_ad_rhs(5, 4, A::mul(s.ad_value(2), s.ad_value(3)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_div_ad(2, A::sub(s.ad_value(1529), A::mul(A::mul(A::scale(s.ad_value(1530), 2.0), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1535)))), s.ad_value(5))), A::offset(s.ad_value(1526), 2.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_div_ad_lhs(1580, A::sub(A::div(A::sub(A::mul(s.ad_value(1538), s.ad_value(1527)), s.ad_value(1531)), s.ad_value(1535)), s.ad_value(2)), 1527);
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_div_ad(1581, A::mul(s.ad_value(1580), s.ad_value(1527)), A::offset(s.ad_value(1580), 1.0));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_sub_ad(1580, A::div(A::mul(s.ad_value(1538), s.ad_value(1537)), A::mul(s.ad_value(1535), s.ad_value(1536))), A::div(A::add(A::div(s.ad_value(1531), s.ad_value(1535)), A::div(s.ad_value(1532), s.ad_value(1536))), s.ad_value(1527)));
        }

        if ((((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1753] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_div_ad(1581, A::mul(s.ad_value(1580), s.ad_value(1527)), A::offset(s.ad_value(1580), 1.0));
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.copy_ad(1581, 1498);
        }

        if ((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) {
            s.store_sub(2, 1581, 1505);
        }

        if ((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) {
            s.store_offset_ad(3, A::mul(A::scale(s.ad_value(2), 36.0), s.ad_value(2)), 1.0);
        }

        s.v[1755] = if (((s.v[2]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_sub(4, 1527, 1462);
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_sub_ad_rhs(1582, 4, A::mul(s.ad_value(1581), s.ad_value(1541)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_sub_ad_rhs(1583, 4, A::mul(s.ad_value(1505), s.ad_value(1541)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_sqrt_ad(1584, A::add(A::square(s.ad_value(1582)), s.ad_value(3)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_sqrt_ad(1585, A::add(A::square(s.ad_value(1583)), s.ad_value(3)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_mul_ad(1586, A::div_from_scalar(0.25, s.ad_value(2)), A::add(A::sub(A::mul(s.ad_value(1585), s.ad_value(1582)), A::mul(s.ad_value(1584), s.ad_value(1583))), A::mul(s.ad_value(3), A::ln(A::div(A::add(s.ad_value(1583), s.ad_value(1585)), A::add(s.ad_value(1582), s.ad_value(1584)))))));
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul(4, 1541, 2);
        }

        if (((s.v[1604] != 0.0) && (s.v[1752] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_ad(1586, A::mul(A::mul(A::scale(s.ad_value(1541), ((-0.25) * 0.1666666666667)), s.ad_value(4)), s.ad_value(4)), A::sqrt(s.ad_value(3)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1752] != 0.0))) {
            s.copy_ad(1581, 1498);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1752] != 0.0))) {
            s.store_scalar(1586, 0.0);
        }

        if (s.v[1604] != 0.0) {
            s.store_sub_ad_lhs(1587, A::add(A::add(A::mul(s.ad_value(1540), s.ad_value(1541)), s.ad_value(1586)), s.ad_value(1462)), 1527);
        }

        s.v[1756] = if (s.v[1462] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1757] = if (s.v[1587] > 1e-30) { 1.0 } else { 0.0 };

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) {
            s.store_div_ad_rhs(1588, 1471, A::sub(A::div(s.ad_value(1467), s.ad_value(1462)), s.ad_value(1474)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) {
            s.store_div_ad_rhs(1589, 1535, A::sub(A::div(s.ad_value(1531), s.ad_value(1527)), s.ad_value(1538)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) {
            s.store_div_ad_lhs(1590, A::sub(s.ad_value(1588), s.ad_value(1589)), 1587);
        }

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) {
            s.store_div_ad_rhs(1591, 1472, A::sub(A::div(s.ad_value(1468), s.ad_value(1462)), s.ad_value(1474)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) {
            s.store_div_ad_rhs(1592, 1536, A::sub(A::div(s.ad_value(1532), s.ad_value(1527)), s.ad_value(1538)));
        }

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (s.v[1757] != 0.0)) {
            s.store_div_ad_lhs(1593, A::sub(s.ad_value(1591), s.ad_value(1592)), 1587);
        }

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (!(s.v[1757] != 0.0))) {
            s.store_scalar(1590, 0.0);
        }

        if (((s.v[1604] != 0.0) && (s.v[1756] != 0.0)) && (!(s.v[1757] != 0.0))) {
            s.store_scalar(1593, 0.0);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_mul_ad(1594, A::scale(s.ad_value(1493), (-2.0)), A::add(A::div(s.ad_value(1430), s.ad_value(1496)), s.ad_value(1499)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_mul_ad(1595, A::scale(s.ad_value(1494), (-2.0)), A::add(A::div(s.ad_value(1431), s.ad_value(1497)), s.ad_value(1499)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_mul_ad_lhs(0, A::sub(s.ad_value(1595), s.ad_value(1594)), 1499);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_mul(2, 1594, 1430);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_mul(3, 1595, 1431);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_add(4, 2, 3);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_offset_ad(5, A::scale(A::add(A::mul(s.ad_value(1493), s.ad_value(1430)), A::mul(s.ad_value(1494), s.ad_value(1431))), 2.0), 3.0);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_div_ad_lhs(1596, A::sub(A::add(s.ad_value(3), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(1496))), 5);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_div_ad_lhs(1597, A::sub(A::sub(s.ad_value(2), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(1497))), 5);
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_mul_ad(1590, A::neg(s.ad_value(1496)), A::add(A::mul(s.ad_value(1596), s.ad_value(1496)), s.ad_value(1499)));
        }

        if ((s.v[1604] != 0.0) && (!(s.v[1756] != 0.0))) {
            s.store_mul_ad(1593, A::neg(s.ad_value(1497)), A::add(A::mul(s.ad_value(1597), s.ad_value(1497)), s.ad_value(1499)));
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1598, 1590, 1577);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1599, 1593, 1577);
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_sub(1600, 1528, 1463, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_scaled_sub(1601, 1529, 1464, 0.5);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1602, 1600, 1598);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul(1603, 1601, 1599);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(436, 1424);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(437, 1428);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(438, 1429);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(439, 1430);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(440, 1431);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(441, 1458);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(442, 1459);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(443, 1443);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(444, 1442);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(445, 1446);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(446, 1447);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(447, 1448);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(448, 1449);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(449, 1450);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(450, 1453);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(451, 1455);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(452, 1456);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(453, 1457);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(454, 1463);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(455, 1464);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(456, 1475);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(457, 1528);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(458, 1529);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(459, 1539);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(460, 1540);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(461, 1544);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(462, 1553);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(463, 1554);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(464, 1575);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(465, 1578);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(466, 1579);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(467, 1600);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(468, 1601);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(469, 1602);
        }

        if (s.v[1604] != 0.0) {
            s.copy_ad(470, 1603);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(436, 379);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(437, 380);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(438, 381);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(439, 382);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(440, 383);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(441, 384);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(442, 385);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(443, 386);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(444, 387);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(445, 389);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(446, 390);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(447, 391);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(448, 392);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(449, 393);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(450, 394);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(451, 395);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(452, 397);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(453, 398);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(454, 400);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(455, 401);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(456, 402);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(457, 404);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(458, 405);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(459, 410);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(460, 411);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(461, 412);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(462, 415);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(463, 416);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(464, 424);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(465, 426);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(466, 427);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(467, 432);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(468, 433);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(469, 434);
        }

        if (!(s.v[1604] != 0.0)) {
            s.copy_ad(470, 435);
        }

        s.store_div_ad(0, A::mul(s.ad_value(120), A::sub(s.ad_value(444), s.ad_value(442))), A::offset(A::scale(s.ad_value(460), 0.25), 1.0));

        s.store_add_ad_lhs(1320, A::scale(A::add(s.ad_value(454), s.ad_value(457)), 0.5), 0);

        s.store_sub_ad_lhs(1321, A::scale(A::add(s.ad_value(455), s.ad_value(458)), 0.5), 0);

        s.v[1758] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1758] != 0.0) {
            s.store_sub_ad_lhs(1322, A::add(s.ad_value(1320), A::div(s.ad_value(462), s.ad_value(465))), 462);
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
        if (s.v[1758] != 0.0) {
            s.store_sub_ad_lhs(1323, A::add(s.ad_value(1321), A::div(s.ad_value(463), s.ad_value(466))), 463);
        }

        if (!(s.v[1758] != 0.0)) {
            s.copy_ad(1322, 1320);
        }

        if (!(s.v[1758] != 0.0)) {
            s.copy_ad(1323, 1321);
        }

        s.store_scaled_mul(2, 467, 469, 0.3333333333333);

        s.store_mul_ad(3, A::scale(s.ad_value(467), 0.1666666666667), A::offset(A::mul(s.ad_value(469), A::sub_from_scalar(1.0, A::scale(s.ad_value(469), 0.2))), 1.0));

        s.store_add_ad_lhs(1324, A::mul(A::scale(s.ad_value(1322), 0.5), s.ad_value(461)), 3);

        s.store_add_ad_lhs(1322, A::mul(s.ad_value(1322), s.ad_value(461)), 2);

        s.store_scaled_mul(2, 468, 470, 0.3333333333333);

        s.store_mul_ad(3, A::scale(s.ad_value(468), 0.1666666666667), A::offset(A::mul(s.ad_value(470), A::sub_from_scalar(1.0, A::scale(s.ad_value(470), 0.2))), 1.0));

        s.store_add_ad_lhs(1325, A::scale(s.ad_value(1323), 0.5), 3);

        s.store_add(1323, 1323, 2);

        s.store_mul(0, 443, 283);

        s.store_mul(357, 0, 1322);

        s.store_mul(358, 0, 1323);

        s.store_mul_ad(359, A::neg(s.ad_value(0)), A::add(s.ad_value(1324), s.ad_value(1325)));

        s.v[1759] = if (s.v[119] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1759] != 0.0) {
            s.store_offset(0, 250, (2.0 * 0.6931471805599));
        }

        if (s.v[1759] != 0.0) {
            s.store_add(1326, 456, 0);
        }

        if (s.v[1759] != 0.0) {
            s.store_add(1327, 459, 0);
        }

        if (s.v[1759] != 0.0) {
            s.store_scale_ad(1328, A::sub(A::add(s.ad_value(1326), s.ad_value(250)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1326), s.ad_value(250)), A::sub(s.ad_value(1326), s.ad_value(250))), 9.0))), 0.5);
        }

        if (s.v[1759] != 0.0) {
            s.store_scale_ad(1329, A::sub(A::add(s.ad_value(1327), A::add(s.ad_value(250), s.ad_value(335))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1327), A::add(s.ad_value(250), s.ad_value(335))), A::sub(s.ad_value(1327), A::add(s.ad_value(250), s.ad_value(335)))), 9.0))), 0.5);
        }

        if (s.v[1759] != 0.0) {
            s.store_mul_ad_rhs(1330, 290, A::sqrt(A::mul(s.ad_value(441), A::offset(s.ad_value(440), 0.5))));
        }

        if (s.v[1759] != 0.0) {
            s.store_mul_ad_rhs(1331, 290, A::sqrt(A::mul(A::mul(A::mul(s.ad_value(441), s.ad_value(452)), s.ad_value(440)), A::offset(s.ad_value(439), 0.5))));
        }

        if (s.v[1759] != 0.0) {
            s.store_mul_ad_lhs(1332, A::square(s.ad_value(1330)), 287);
        }

        if (s.v[1759] != 0.0) {
            s.store_mul_ad_lhs(1333, A::square(s.ad_value(1331)), 287);
        }

        if (s.v[1759] != 0.0) {
            s.store_sub(2, 288, 1328);
        }

        if (s.v[1759] != 0.0) {
            s.store_sub_ad_lhs(3, A::add(s.ad_value(288), s.ad_value(335)), 1329);
        }

        if (s.v[1759] != 0.0) {
            s.store_scale(0, 1332, 2.0);
        }

        if (s.v[1759] != 0.0) {
            s.store_add_ad_rhs(1334, 1328, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1332)), 1.0)), (-1.0))));
        }

        if (s.v[1759] != 0.0) {
            s.store_add_ad_rhs(1335, 1329, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1332)), 1.0)), (-1.0))));
        }

        if (s.v[1759] != 0.0) {
            s.store_scale(0, 1333, 2.0);
        }

        if (s.v[1759] != 0.0) {
            s.store_add_ad_rhs(1336, 1328, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1333)), 1.0)), (-1.0))));
        }

        if (s.v[1759] != 0.0) {
            s.store_add_ad_rhs(1337, 1329, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1333)), 1.0)), (-1.0))));
        }

        if (s.v[1759] != 0.0) {
            s.store_mul(0, 289, 443);
        }

        if (s.v[1759] != 0.0) {
            s.store_mul_ad_lhs(2, A::mul(A::mul(A::neg(s.ad_value(0)), s.ad_value(1330)), s.ad_value(452)), 447);
        }

        if (s.v[1759] != 0.0) {
            s.store_mul_ad_lhs(3, A::mul(A::mul(A::neg(s.ad_value(0)), s.ad_value(1331)), s.ad_value(453)), 448);
        }

        if (s.v[1759] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1334), s.ad_value(1326)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1334), s.ad_value(1326)), A::sub(s.ad_value(1334), s.ad_value(1326))), 1.0))), 0.5);
        }

        if (s.v[1759] != 0.0) {
            s.store_div_ad(375, A::mul(A::mul(s.ad_value(2), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1334), s.ad_value(1328)));
        }

        if (s.v[1759] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1335), s.ad_value(1327)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1335), s.ad_value(1327)), A::sub(s.ad_value(1335), s.ad_value(1327))), 1.0))), 0.5);
        }

        if (s.v[1759] != 0.0) {
            s.store_div_ad(376, A::mul(A::mul(s.ad_value(2), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1335), s.ad_value(1329)));
        }

        if (s.v[1759] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1336), s.ad_value(1326)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1336), s.ad_value(1326)), A::sub(s.ad_value(1336), s.ad_value(1326))), 1.0))), 0.5);
        }

        if (s.v[1759] != 0.0) {
            s.store_div_ad(377, A::mul(A::mul(s.ad_value(3), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1336), s.ad_value(1328)));
        }

        if (s.v[1759] != 0.0) {
            s.store_scale_ad(0, A::add(A::sub(s.ad_value(1337), s.ad_value(1327)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1337), s.ad_value(1327)), A::sub(s.ad_value(1337), s.ad_value(1327))), 1.0))), 0.5);
        }

        if (s.v[1759] != 0.0) {
            s.store_div_ad(378, A::mul(A::mul(s.ad_value(3), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1337), s.ad_value(1329)));
        }

        if (!(s.v[1759] != 0.0)) {
            s.store_scalar(375, 0.0);
        }

        if (!(s.v[1759] != 0.0)) {
            s.store_scalar(376, 0.0);
        }

        if (!(s.v[1759] != 0.0)) {
            s.store_scalar(377, 0.0);
        }

        if (!(s.v[1759] != 0.0)) {
            s.store_scalar(378, 0.0);
        }

        s.store_mul(366, 164, 326);

        s.store_mul(367, 165, 328);

        let assign42690_ad_e48215: A = A::add(A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(445)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436))))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(445)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436))))), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(445)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436)))))), 0.2)));
        s.store_scale_ad(0, assign42690_ad_e48215, 0.5);

        s.store_mul_ad_lhs(368, A::mul(s.ad_value(159), s.ad_value(345)), 0);

        s.store_mul_ad_lhs(369, A::mul(s.ad_value(160), s.ad_value(346)), 0);

        s.store_mul(370, 117, 334);

        s.store_mul(371, 166, 332);

        s.store_mul_ad_lhs(373, A::neg(A::add(A::mul(s.ad_value(236), s.ad_value(9)), A::mul(s.ad_value(167), s.ad_value(11)))), 327);

        s.store_mul_ad_lhs(372, A::neg(A::add(A::mul(s.ad_value(236), s.ad_value(10)), A::mul(s.ad_value(167), s.ad_value(12)))), 329);

        s.v[1760] = if (s.v[6] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1760] != 0.0) {
            s.store_mul(374, 170, 215);
        }

        if (!(s.v[1760] != 0.0)) {
            s.store_scalar(374, 0.0);
        }

        s.store_mul_ad(361, A::scale(s.ad_value(13), p.p31), A::add(A::add(s.ad_value(344), s.ad_value(352)), s.ad_value(354)));

        s.store_mul_ad_lhs(362, A::scale(s.ad_value(13), p.p31), 348);

        s.store_mul_ad_lhs(363, A::scale(s.ad_value(13), p.p31), 349);

        s.store_mul_ad_lhs(364, A::scale(s.ad_value(13), p.p31), 350);

        s.store_mul_ad_lhs(365, A::scale(s.ad_value(13), p.p31), 351);

        s.store_mul(1761, 13, 355);

        s.store_mul(1762, 13, 356);

        s.v[1763] = if (s.v[330] < 0.0) { 1.0 } else { 0.0 };

        s.v[1764] = if (s.v[307] > 0.0) { 1.0 } else { 0.0 };

        s.v[1765] = if (s.v[314] > 0.0) { 1.0 } else { 0.0 };

        s.v[1766] = if (s.v[318] > 0.0) { 1.0 } else { 0.0 };

        s.v[1767] = if (s.v[322] > 0.0) { 1.0 } else { 0.0 };

        s.store_mul_ad_lhs(357, A::scale(s.ad_value(13), p.p32), 357);

        s.store_mul_ad_lhs(358, A::scale(s.ad_value(13), p.p32), 358);

        s.store_mul_ad_lhs(359, A::scale(s.ad_value(13), p.p32), 359);

        s.store_neg_ad(360, A::add(A::add(s.ad_value(357), s.ad_value(358)), s.ad_value(359)));

        s.store_mul_ad_lhs(375, A::scale(s.ad_value(13), p.p32), 375);

        s.store_mul_ad_lhs(376, A::scale(s.ad_value(13), p.p32), 376);

        s.store_mul_ad_lhs(377, A::scale(s.ad_value(13), p.p32), 377);

        s.store_mul_ad_lhs(378, A::scale(s.ad_value(13), p.p32), 378);

        s.store_mul_ad_lhs(366, A::scale(s.ad_value(13), p.p32), 366);

        s.store_mul_ad_lhs(367, A::scale(s.ad_value(13), p.p32), 367);

        s.store_mul_ad_lhs(368, A::scale(s.ad_value(13), p.p32), 368);

        s.store_mul_ad_lhs(369, A::scale(s.ad_value(13), p.p32), 369);

        s.store_mul_ad_lhs(370, A::scale(s.ad_value(13), p.p32), 370);

        s.store_mul_ad_lhs(373, A::scale(s.ad_value(13), p.p32), 373);

        s.store_mul_ad_lhs(372, A::scale(s.ad_value(13), p.p32), 372);

        s.store_mul_ad_lhs(371, A::scale(s.ad_value(13), p.p32), 371);

        s.store_mul(374, 13, 374);

        s.v[1769] = if (s.v[330] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1769] != 0.0) {
            s.copy_ad(1768, 359);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(359, 360);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(360, 1768);
        }

        if (s.v[1769] != 0.0) {
            s.store_neg(371, 371);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(1768, 376);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(376, 375);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(375, 1768);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(1768, 378);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(378, 377);
        }

        if (s.v[1769] != 0.0) {
            s.copy_ad(377, 1768);
        }

        s.store_mul_ad_lhs(1770, A::scale(s.ad_value(386), 6.241509343260179e18), 222);

        s.store_scaled_add(1771, 403, 428, (-0.5));

        s.store_add(1772, 411, 1771);

        s.store_div(0, 411, 1772);

        s.store_scale_ad(1777, A::add(s.ad_value(0), A::sqrt(A::offset(A::mul(s.ad_value(0), s.ad_value(0)), 1e-20))), 0.5);

        s.store_mul_ad_lhs(1778, A::scale(s.ad_value(432), (-0.1666666666667)), 431);

        s.store_square(1779, 1778);

        s.store_offset(1780, 425, (-1.0));

        s.store_max_with_scalar_ad(1781, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1780), 12.0), s.ad_value(1779))), 1e-20);

        s.store_div_from_scalar_ad(1782, 1.0, A::square(s.ad_value(1781)));

        s.store_div_ad_lhs(1783, A::div(A::mul(A::mul(A::mul(A::mul(s.ad_value(338), s.ad_value(386)), s.ad_value(222)), s.ad_value(1772)), s.ad_value(340)), s.ad_value(341)), 342);

        s.store_scale(1784, 1779, 12.0);

        s.store_sub_ad(2, A::add(s.ad_value(1777), s.ad_value(1784)), A::mul(A::mul(A::scale(A::offset(s.ad_value(1777), 1.0), 2.0), s.ad_value(1784)), s.ad_value(1780)));

        s.store_ad(3, &A::max_with_scalar(s.ad_value(2), 1e-40));

        s.store_mul_ad_lhs(1785, A::mul(s.ad_value(1783), s.ad_value(1782)), 3);

        s.v[1802] = if (s.v[172] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1802] != 0.0) {
            s.store_div(1786, 423, 418);
        }

        if (s.v[1802] != 0.0) {
            s.store_div_ad(1787, A::mul(A::mul(A::mul(s.ad_value(305), s.ad_value(344)), s.ad_value(407)), s.ad_value(219)), A::mul(A::mul(A::offset(A::square(s.ad_value(1786)), 1.0), s.ad_value(1781)), s.ad_value(1781)));
        }

        if (s.v[1802] != 0.0) {
            s.store_add_ad_rhs(1785, 1785, A::div(s.ad_value(1787), s.ad_value(304)));
        }

        s.store_mul_ad_lhs(1788, A::mul(A::scale(s.ad_value(13), p.p31), s.ad_value(303)), 1785);

        s.store_div_ad_lhs(1789, A::mul(A::mul(s.ad_value(452), s.ad_value(443)), s.ad_value(116)), 465);

        s.store_mul_ad_lhs(1790, A::offset(s.ad_value(464), 1.0), 1789);

        s.store_mul_ad_rhs(1792, 1790, A::sub_from_scalar(0.5, A::mul(A::scale(s.ad_value(330), 0.25), s.ad_value(1778))));

        s.store_sub(1791, 1790, 1792);

        s.v[1795] = 0.0;

        s.v[1796] = 0.0;

        s.v[1803] = if (p.p6 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1803] != 0.0) {
            s.store_sub_ad(2, A::sub(A::scale(s.ad_value(1777), 0.08333333333333333), A::mul(s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 0.2), s.ad_value(1784)))), A::mul(A::mul(A::scale(s.ad_value(1779), 1.6), A::sub(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784))), s.ad_value(1780)));
        }

        if (s.v[1803] != 0.0) {
            s.store_ad(3, &A::max_with_scalar(s.ad_value(2), 1e-40));
        }

        if (s.v[1803] != 0.0) {
            s.store_div_ad_lhs(1793, A::mul(A::mul(s.ad_value(1783), s.ad_value(1781)), s.ad_value(1781)), 3);
        }

        if (s.v[1803] != 0.0) {
            s.store_mul_ad_lhs(1794, A::mul(A::scale(s.ad_value(13), p.p31), s.ad_value(303)), 1793);
        }

        s.v[1804] = if (s.v[1785] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1803] != 0.0) && (s.v[1804] != 0.0)) {
            s.store_mul_ad(1795, A::mul(s.ad_value(1782), s.ad_value(1778)), A::sub(A::sub_from_scalar(1.0, s.ad_value(1784)), A::mul(A::sub(A::add(s.ad_value(1777), A::scale(s.ad_value(1779), 19.2)), A::mul(s.ad_value(1777), s.ad_value(1784))), s.ad_value(1780))));
        }

        if ((s.v[1803] != 0.0) && (s.v[1804] != 0.0)) {
            s.store_div_ad_lhs(1796, A::mul(A::square(s.ad_value(1795)), s.ad_value(1793)), 1785);
        }

        if ((s.v[1803] != 0.0) && (s.v[1804] != 0.0)) {
            let assign43530_ad_e48767: A = A::sub(A::offset(A::scale(A::add(s.ad_value(1796), A::sqrt(A::offset(A::mul(s.ad_value(1796), s.ad_value(1796)), 1e-40))), 0.5), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::add(s.ad_value(1796), A::sqrt(A::offset(A::mul(s.ad_value(1796), s.ad_value(1796)), 1e-40))), 0.5), (-1.0)), A::offset(A::scale(A::add(s.ad_value(1796), A::sqrt(A::offset(A::mul(s.ad_value(1796), s.ad_value(1796)), 1e-40))), 0.5), (-1.0))), 1e-40)));
            s.store_scale_ad(1796, assign43530_ad_e48767, 0.5);
        }

        if (!(s.v[1803] != 0.0)) {
            s.store_scalar(1793, 1.0);
        }

        if (!(s.v[1803] != 0.0)) {
            s.store_scalar(1794, 0.0);
        }

        s.store_mul_ad_rhs(1797, 1788, A::sub_from_scalar(1.0, s.ad_value(1796)));

        s.copy_ad(1773, 1770);

        s.store_mul_ad_rhs(1774, 1770, A::offset(s.ad_value(411), 1.0));

        s.store_mul_ad_rhs(1775, 1770, A::sub(s.ad_value(399), s.ad_value(409)));

        s.store_mul_ad(2, A::add(A::sub(s.ad_value(173), A::mul(s.ad_value(174), s.ad_value(1773))), A::mul(A::mul(s.ad_value(175), s.ad_value(1773)), s.ad_value(1773))), A::ln(A::div(A::add(s.ad_value(1774), A::scale(s.ad_value(1775), 0.5)), A::sub(s.ad_value(1774), A::scale(s.ad_value(1775), 0.5)))));

        s.store_add_ad_rhs(3, 2, A::mul(A::add(s.ad_value(174), A::mul(s.ad_value(175), A::sub(s.ad_value(1774), A::scale(s.ad_value(1773), 2.0)))), s.ad_value(1775)));

        s.store_offset_ad(0, A::div(A::add(A::mul(s.ad_value(176), s.ad_value(413)), A::mul(s.ad_value(177), s.ad_value(414))), A::offset(s.ad_value(411), 1.0)), 1.0);

        s.store_scale_ad(4, A::add(A::offset(s.ad_value(0), 0.01), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(0), (-0.01)), A::offset(s.ad_value(0), (-0.01))), 0.0001))), 0.5);

        s.store_mul_ad_lhs(0, A::div(A::mul(A::div(A::mul(A::scale(s.ad_value(343), 1.602176565e-19), s.ad_value(344)), s.ad_value(341)), s.ad_value(3)), s.ad_value(1773)), 4);

        s.store_mul_ad(1776, A::scale(s.ad_value(13), p.p33), A::max_with_scalar(s.ad_value(0), 0.0));

        s.store_mul_ad(1798, A::scale(s.ad_value(13), ((2.0 * 1.602176565e-19) * p.p31)), A::abs(s.ad_value(348)));

        s.store_mul_ad(1799, A::scale(s.ad_value(13), ((2.0 * 1.602176565e-19) * p.p31)), A::abs(s.ad_value(349)));

        s.store_scale_ad(1801, A::mul(A::offset(s.ad_value(353), 1.0), A::abs(s.ad_value(354))), (2.0 * 1.602176565e-19));

        s.store_add_ad(1800, A::mul(A::scale(s.ad_value(13), ((2.0 * 1.602176565e-19) * p.p31)), A::abs(A::sub(s.ad_value(350), s.ad_value(351)))), A::mul(A::scale(s.ad_value(13), p.p31), s.ad_value(1801)));

        s.store_div_from_scalar_ad(1813, 1.0, A::scale(s.ad_value(8), 8.617332384961e-5));

        s.store_sub_from_scalar_ad(1814, 1.17, A::div(A::mul(A::scale(s.ad_value(8), 0.000473), s.ad_value(8)), A::offset(s.ad_value(8), 636.0)));

        s.store_sub_from_scalar_ad(1815, 0.744, A::div(A::mul(A::scale(s.ad_value(8), 0.0004774), s.ad_value(8)), A::offset(s.ad_value(8), 235.0)));

        s.store_mul_ad_lhs(1816, A::add(A::sub(s.ad_value(1815), s.ad_value(1814)), A::scale(s.ad_value(224), (-0.4))), 15);

        s.store_add(1817, 1814, 1816);

        s.store_mul_ad_lhs(1818, A::scale(s.ad_value(1817), 0.5), 1813);

        s.store_sub_ad(1819, A::scale(s.ad_value(15), 0.05), A::scale(s.ad_value(1816), 0.5));

        s.store_sqrt_ad(0, A::scale(s.ad_value(8), 0.0033333333333));

        s.store_mul_ad_lhs(2, A::mul(A::scale(s.ad_value(0), 4.05e25), s.ad_value(0)), 0);

        s.store_mul(1820, 2, 234);

        s.store_div_ad_rhs(1821, 1813, A::offset(A::div(A::scale(s.ad_value(17), s.v[7]), s.ad_value(8)), 1.0));

        s.store_mul_ad_lhs(1823, A::mul(A::scale(s.ad_value(1820), (2.0 * 1.602176565e-19)), s.ad_value(225)), 1821);

        s.store_add_ad_lhs(1824, A::offset(A::ln(A::div(A::square(s.ad_value(241)), s.ad_value(1823))), (-0.6931471805599)), 1818);

        s.store_mul_ad_lhs(1825, A::div(A::mul(A::scale(s.ad_value(29), (0.5 * 1.602176565e-19)), s.ad_value(14)), A::add(s.ad_value(237), s.ad_value(238))), 1821);

        s.store_mul(1828, 35, 1821);

        s.v[1829] = 0.0;

        s.v[1822] = 0.0;

        s.v[1874] = if (p.p9 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1874] != 0.0) {
            s.store_mul_ad(1822, A::div_from_scalar(1.0, s.ad_value(1813)), A::ln(A::div(s.ad_value(24), s.ad_value(247))));
        }

        s.v[1875] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        s.v[1876] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1875] != 0.0) && (s.v[1876] != 0.0)) {
            s.store_scale_ad(1829, A::exp(A::scale(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333))), ((0.4 * p.p13) * 1.27520989));
        }

        if ((s.v[1875] != 0.0) && (!(s.v[1876] != 0.0))) {
            s.store_scale_ad(1829, A::exp(A::scale(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333))), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_mul(1832, 332, 1821);

        s.store_mul_ad_lhs(1833, A::offset(A::sqrt(A::offset(A::square(s.ad_value(332)), 0.01)), (-0.1)), 1821);

        s.store_scaled_sub(1834, 1832, 1833, 0.5);

        s.store_div_ad(1805, A::div(s.ad_value(398), s.ad_value(397)), A::offset(s.ad_value(398), 1.0));

        s.store_div_ad(1806, A::div(s.ad_value(397), s.ad_value(398)), A::offset(s.ad_value(397), 1.0));

        s.store_offset_ad(1807, A::ln(A::div(A::mul(A::mul(s.ad_value(397), A::offset(s.ad_value(1805), 1.0)), s.ad_value(380)), s.ad_value(381))), 2.0);

        s.store_offset_ad(1808, A::ln(A::div(A::mul(A::mul(s.ad_value(398), A::offset(s.ad_value(1806), 1.0)), s.ad_value(380)), s.ad_value(381))), 2.0);

        s.store_sub_ad(1809, A::mul(A::offset(s.ad_value(1805), 1.0), s.ad_value(1807)), A::mul(s.ad_value(395), s.ad_value(1805)));

        s.store_sub_ad(1810, A::mul(A::offset(A::div_from_scalar(1.0, s.ad_value(1806)), 1.0), s.ad_value(1808)), A::div(s.ad_value(395), s.ad_value(1806)));

        s.store_add_ad_lhs(1811, A::div(A::sub(A::scale(A::sub(A::add(s.ad_value(1809), s.ad_value(1810)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1809), s.ad_value(1810)), A::sub(s.ad_value(1809), s.ad_value(1810))), 38.0))), 0.5), s.ad_value(394)), s.ad_value(25)), 394);

        s.store_add_ad_lhs(1812, A::mul(s.ad_value(222), A::add(A::sub(A::div(A::sub(s.ad_value(1811), s.ad_value(390)), s.ad_value(391)), s.ad_value(393)), s.ad_value(390))), 21);

        s.store_mul_ad_rhs(0, 34, A::offset(s.ad_value(8), (-s.v[7])));

        s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(23), p.p14), A::offset(s.ad_value(8), (-s.v[7]))), 252);

        s.store_sub_ad_lhs(1830, A::offset(A::add(A::scale(A::add(A::add(s.ad_value(179), s.ad_value(1819)), s.ad_value(239)), p.p14), s.ad_value(0)), p.p34), 1822);

        s.store_add_ad_lhs(1831, A::scale(A::add(A::add(s.ad_value(180), s.ad_value(1819)), s.ad_value(240)), p.p14), 0);

        s.store_sub_ad_lhs(1835, A::mul(A::sub(s.ad_value(1812), s.ad_value(1830)), s.ad_value(1821)), 1834);

        s.store_sub_ad_lhs(1836, A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(1831)), s.ad_value(1821)), 1834);

        s.v[1877] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

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
        if (s.v[1877] != 0.0) {
            s.store_div_ad_lhs(0, A::mul(A::scale(s.ad_value(16), p.p14), A::sub(s.ad_value(1835), s.ad_value(1836))), 256);
        }

        s.v[1878] = if (s.v[0] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1877] != 0.0) && (s.v[1878] != 0.0)) {
            s.store_scale_ad(2, A::ln(A::sub_from_scalar(1.0, s.ad_value(0))), (-2.0));
        }

        if ((s.v[1877] != 0.0) && (!(s.v[1878] != 0.0))) {
            s.store_div_ad(2, A::square(s.ad_value(0)), A::offset(A::div(A::scale(s.ad_value(0), 2.0), s.ad_value(256)), 1.0));
        }

        if (s.v[1877] != 0.0) {
            s.store_add_ad_rhs(1837, 1836, A::mul(A::scale(s.ad_value(16), p.p14), s.ad_value(2)));
        }

        if (!(s.v[1877] != 0.0)) {
            s.copy_ad(1837, 1836);
        }

        s.store_mul_ad_rhs(0, 244, A::sub(s.ad_value(1835), s.ad_value(1837)));

        s.v[1879] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1879] != 0.0) {
            s.store_scale_ad(1838, A::add(A::add(s.ad_value(0), s.ad_value(253)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))))), 0.5);
        }

        if (s.v[1879] != 0.0) {
            s.store_scale_ad(1839, A::add(A::sub(s.ad_value(253), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(253)), A::sub(A::neg(s.ad_value(0)), s.ad_value(253))), A::square(s.ad_value(253))))), 0.5);
        }

        if (s.v[1879] != 0.0) {
            s.store_mul_ad_rhs(2, 1829, A::exp(A::scale(A::ln(s.ad_value(1838)), (-0.3333333333333))));
        }

        if (s.v[1879] != 0.0) {
            s.store_mul_ad_rhs(3, 1829, A::exp(A::scale(A::ln(s.ad_value(1839)), (-0.3333333333333))));
        }

        if (s.v[1879] != 0.0) {
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if (s.v[1879] != 0.0) {
            s.store_div_ad(1841, A::mul(s.ad_value(242), s.ad_value(4)), A::offset(A::mul(s.ad_value(242), s.ad_value(2)), 1.0));
        }

        if (s.v[1879] != 0.0) {
            s.store_div_ad(1842, A::mul(s.ad_value(243), s.ad_value(4)), A::offset(A::mul(s.ad_value(243), s.ad_value(3)), 1.0));
        }

        if (s.v[1879] != 0.0) {
            s.store_div_from_scalar_ad(1843, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842))));
        }

        if (!(s.v[1879] != 0.0)) {
            s.copy_ad(1841, 242);
        }

        if (!(s.v[1879] != 0.0)) {
            s.copy_ad(1842, 243);
        }

        if (!(s.v[1879] != 0.0)) {
            s.copy_ad(1843, 244);
        }

        s.store_mul_ad_rhs(1844, 1843, A::sub(s.ad_value(1835), s.ad_value(1837)));

        s.v[1880] = if (s.v[1844] > 0.0) { 1.0 } else { 0.0 };

        s.v[1881] = if ((-s.v[1844]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1880] != 0.0) && (s.v[1881] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(A::neg(s.ad_value(1844))), 1.0));
        }

        if ((s.v[1880] != 0.0) && (!(s.v[1881] != 0.0))) {
            s.store_neg(0, 1844);
        }

        if (s.v[1880] != 0.0) {
            s.store_offset_ad(1845, A::add(A::sub(s.ad_value(1835), A::div(s.ad_value(1844), s.ad_value(1841))), s.ad_value(0)), (-0.6931471805599));
        }

        s.v[1882] = if (s.v[1844] < 80.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1880] != 0.0)) && (s.v[1882] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(s.ad_value(1844)), 1.0));
        }

        if ((!(s.v[1880] != 0.0)) && (!(s.v[1882] != 0.0))) {
            s.copy_ad(0, 1844);
        }

        if (!(s.v[1880] != 0.0)) {
            s.store_offset_ad(1845, A::add(A::add(s.ad_value(1837), A::div(s.ad_value(1844), s.ad_value(1842))), s.ad_value(0)), (-0.6931471805599));
        }

        s.store_scale_ad(1846, A::sub(A::add(s.ad_value(1845), s.ad_value(1824)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1845), s.ad_value(1824)), A::sub(s.ad_value(1845), s.ad_value(1824))), 4.0))), 0.5);

        s.store_offset_ad(1847, A::sqrt(A::offset(A::div(A::scale(A::sub(s.ad_value(1824), s.ad_value(1846)), 2.0), s.ad_value(1825)), 1.0)), (-1.0));

        s.store_scale_ad(0, A::add(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5))), 0.01))), 0.5);

        s.store_mul_ad(0, A::mul(A::mul(A::scale(s.ad_value(1828), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1836)), 1.0));

        s.v[1884] = if (p.p11 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1884] != 0.0) {
            s.store_div_ad(1805, A::div(s.ad_value(453), s.ad_value(452)), A::offset(s.ad_value(453), 1.0));
        }

        if (s.v[1884] != 0.0) {
            s.store_div_ad(1806, A::div(s.ad_value(452), s.ad_value(453)), A::offset(s.ad_value(452), 1.0));
        }

        if (s.v[1884] != 0.0) {
            s.store_offset_ad(1807, A::ln(A::div(A::mul(A::mul(s.ad_value(452), A::offset(s.ad_value(1805), 1.0)), s.ad_value(437)), s.ad_value(438))), 2.0);
        }

        if (s.v[1884] != 0.0) {
            s.store_offset_ad(1808, A::ln(A::div(A::mul(A::mul(s.ad_value(453), A::offset(s.ad_value(1806), 1.0)), s.ad_value(437)), s.ad_value(438))), 2.0);
        }

        if (s.v[1884] != 0.0) {
            s.store_sub_ad(1809, A::mul(A::offset(s.ad_value(1805), 1.0), s.ad_value(1807)), A::mul(s.ad_value(451), s.ad_value(1805)));
        }

        if (s.v[1884] != 0.0) {
            s.store_sub_ad(1810, A::mul(A::offset(A::div_from_scalar(1.0, s.ad_value(1806)), 1.0), s.ad_value(1808)), A::div(s.ad_value(451), s.ad_value(1806)));
        }

        if (s.v[1884] != 0.0) {
            s.store_add_ad_lhs(1811, A::div(A::sub(A::scale(A::sub(A::add(s.ad_value(1809), s.ad_value(1810)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1809), s.ad_value(1810)), A::sub(s.ad_value(1809), s.ad_value(1810))), 38.0))), 0.5), s.ad_value(450)), s.ad_value(25)), 450);
        }

        if (s.v[1884] != 0.0) {
            s.store_add_ad_lhs(1812, A::mul(s.ad_value(222), A::add(A::sub(A::div(A::sub(s.ad_value(1811), s.ad_value(446)), s.ad_value(447)), s.ad_value(449)), s.ad_value(446))), 130);
        }

        if (s.v[1884] != 0.0) {
            s.store_mul_ad_rhs(0, 34, A::offset(s.ad_value(8), (-s.v[7])));
        }

        if (s.v[1884] != 0.0) {
            s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(23), p.p14), A::offset(s.ad_value(8), (-s.v[7]))), 252);
        }

        if (s.v[1884] != 0.0) {
            s.store_sub_ad_lhs(1830, A::offset(A::add(A::scale(A::add(A::add(s.ad_value(181), s.ad_value(1819)), s.ad_value(239)), p.p14), s.ad_value(0)), p.p34), 1822);
        }

        if (s.v[1884] != 0.0) {
            s.store_add_ad_lhs(1831, A::scale(A::add(A::add(s.ad_value(182), s.ad_value(1819)), s.ad_value(240)), p.p14), 0);
        }

        if (s.v[1884] != 0.0) {
            s.store_sub_ad_lhs(1835, A::mul(A::sub(s.ad_value(1812), s.ad_value(1830)), s.ad_value(1821)), 1834);
        }

        if (s.v[1884] != 0.0) {
            s.store_sub_ad_lhs(1836, A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(1831)), s.ad_value(1821)), 1834);
        }

        s.v[1885] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1884] != 0.0) && (s.v[1885] != 0.0)) {
            s.store_div_ad_lhs(0, A::mul(A::scale(s.ad_value(16), p.p14), A::sub(s.ad_value(1835), s.ad_value(1836))), 256);
        }

        s.v[1886] = if (s.v[0] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1884] != 0.0) && (s.v[1885] != 0.0)) && (s.v[1886] != 0.0)) {
            s.store_scale_ad(2, A::ln(A::sub_from_scalar(1.0, s.ad_value(0))), (-2.0));
        }

        if (((s.v[1884] != 0.0) && (s.v[1885] != 0.0)) && (!(s.v[1886] != 0.0))) {
            s.store_div_ad(2, A::square(s.ad_value(0)), A::offset(A::div(A::scale(s.ad_value(0), 2.0), s.ad_value(256)), 1.0));
        }

        if ((s.v[1884] != 0.0) && (s.v[1885] != 0.0)) {
            s.store_add_ad_rhs(1837, 1836, A::mul(A::scale(s.ad_value(16), p.p14), s.ad_value(2)));
        }

        if ((s.v[1884] != 0.0) && (!(s.v[1885] != 0.0))) {
            s.copy_ad(1837, 1836);
        }

        if (s.v[1884] != 0.0) {
            s.store_mul_ad_rhs(0, 244, A::sub(s.ad_value(1835), s.ad_value(1837)));
        }

        s.v[1887] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_scale_ad(1838, A::add(A::add(s.ad_value(0), s.ad_value(253)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))))), 0.5);
        }

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_scale_ad(1839, A::add(A::sub(s.ad_value(253), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(253)), A::sub(A::neg(s.ad_value(0)), s.ad_value(253))), A::square(s.ad_value(253))))), 0.5);
        }

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_mul_ad_rhs(2, 1829, A::exp(A::scale(A::ln(s.ad_value(1838)), (-0.3333333333333))));
        }

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_mul_ad_rhs(3, 1829, A::exp(A::scale(A::ln(s.ad_value(1839)), (-0.3333333333333))));
        }

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_div_ad(1841, A::mul(s.ad_value(242), s.ad_value(4)), A::offset(A::mul(s.ad_value(242), s.ad_value(2)), 1.0));
        }

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_div_ad(1842, A::mul(s.ad_value(243), s.ad_value(4)), A::offset(A::mul(s.ad_value(243), s.ad_value(3)), 1.0));
        }

        if ((s.v[1884] != 0.0) && (s.v[1887] != 0.0)) {
            s.store_div_from_scalar_ad(1843, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842))));
        }

        if ((s.v[1884] != 0.0) && (!(s.v[1887] != 0.0))) {
            s.copy_ad(1841, 242);
        }

        if ((s.v[1884] != 0.0) && (!(s.v[1887] != 0.0))) {
            s.copy_ad(1842, 243);
        }

        if ((s.v[1884] != 0.0) && (!(s.v[1887] != 0.0))) {
            s.copy_ad(1843, 244);
        }

        if (s.v[1884] != 0.0) {
            s.store_mul_ad_rhs(1844, 1843, A::sub(s.ad_value(1835), s.ad_value(1837)));
        }

        s.v[1888] = if (s.v[1844] > 0.0) { 1.0 } else { 0.0 };

        s.v[1889] = if ((-s.v[1844]) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1884] != 0.0) && (s.v[1888] != 0.0)) && (s.v[1889] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(A::neg(s.ad_value(1844))), 1.0));
        }

        if (((s.v[1884] != 0.0) && (s.v[1888] != 0.0)) && (!(s.v[1889] != 0.0))) {
            s.store_neg(0, 1844);
        }

        if ((s.v[1884] != 0.0) && (s.v[1888] != 0.0)) {
            s.store_offset_ad(1845, A::add(A::sub(s.ad_value(1835), A::div(s.ad_value(1844), s.ad_value(1841))), s.ad_value(0)), (-0.6931471805599));
        }

        s.v[1890] = if (s.v[1844] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1884] != 0.0) && (!(s.v[1888] != 0.0))) && (s.v[1890] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(s.ad_value(1844)), 1.0));
        }

        if (((s.v[1884] != 0.0) && (!(s.v[1888] != 0.0))) && (!(s.v[1890] != 0.0))) {
            s.copy_ad(0, 1844);
        }

        if ((s.v[1884] != 0.0) && (!(s.v[1888] != 0.0))) {
            s.store_offset_ad(1845, A::add(A::add(s.ad_value(1837), A::div(s.ad_value(1844), s.ad_value(1842))), s.ad_value(0)), (-0.6931471805599));
        }

        if (s.v[1884] != 0.0) {
            s.store_scale_ad(1846, A::sub(A::add(s.ad_value(1845), s.ad_value(1824)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1845), s.ad_value(1824)), A::sub(s.ad_value(1845), s.ad_value(1824))), 4.0))), 0.5);
        }

        if (s.v[1884] != 0.0) {
            s.store_offset_ad(1847, A::sqrt(A::offset(A::div(A::scale(A::sub(s.ad_value(1824), s.ad_value(1846)), 2.0), s.ad_value(1825)), 1.0)), (-1.0));
        }

        if (s.v[1884] != 0.0) {
            s.store_scale_ad(0, A::add(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5))), 0.01))), 0.5);
        }

        if (s.v[1884] != 0.0) {
            s.store_mul_ad(0, A::mul(A::mul(A::scale(s.ad_value(1828), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1836)), 1.0));
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

        s.v[525] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[525] != 0.0) {
            s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));
        }

        if (s.v[525] != 0.0) {
            s.store_scale_ad(221, A::add(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0)), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01))), 0.5);
        }

        if (!(s.v[525] != 0.0)) {
            s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));
        }

        if (!(s.v[525] != 0.0)) {
            s.store_scalar(221, 600.0);
        }

        s.v[526] = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0))) { 1.0 } else { 0.0 };

        if (s.v[526] != 0.0) {
            s.store_scalar(6, p.p5);
        }

        if (!(s.v[526] != 0.0)) {
            s.store_scalar(6, 0.0);
        }

        s.v[215] = 0.0;

        s.copy_ad(213, 8);

        s.store_square(214, 213);

        s.store_offset(216, 213, (-s.v[7]));

        s.store_scale(217, 213, 1.0 / (s.v[7]));

        s.store_div_from_scalar(218, s.v[7], 213);

        s.store_scale(219, 213, 8.617332384961e-5);

        s.store_div_from_scalar(220, 1.0, 219);

        s.v[607] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[607] != 0.0) {
            s.store_scalar(10, p.p23);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(9, p.p22);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(12, p.p25);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(11, p.p24);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(13, p.p30);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(529, p.p41);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(14, p.p42);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(15, p.p43);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(530, p.p44);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(531, 1.0);
        }

        s.v[608] = if (p.p45 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[607] != 0.0) && (s.v[608] != 0.0)) {
            s.store_scalar(531, (-1.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(532, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(16, 1.0);
        }

        s.v[609] = if (p.p46 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[607] != 0.0) && (s.v[609] != 0.0)) {
            s.store_scalar(16, (-1.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(533, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(17, p.p47);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(18, p.p48);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(19, (p.p49 * 1000000.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(20, (p.p50 * 1000000.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(179, p.p51);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(180, p.p52);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(23, p.p53);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(24, (p.p54 * 1000000.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(25, p.p55);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(26, p.p56);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(27, p.p57);
        }

        if (s.v[607] != 0.0) {
            s.store_div_ad_lhs(28, A::mul(A::scale(s.ad_value(27), p.p58), s.ad_value(530)), 529);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(29, (p.p59 * 1000000.0));
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(30, p.p60);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(534, p.p61);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(183, p.p62);
        }

        if (s.v[607] != 0.0) {
            s.store_div_ad_lhs(184, A::mul(A::scale(s.ad_value(183), p.p63), s.ad_value(530)), 529);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(34, p.p64);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(35, p.p65);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(36, p.p66);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(37, p.p67);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(187, p.p68);
        }

        if (s.v[607] != 0.0) {
            s.store_scale(188, 187, p.p69);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(40, p.p70);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(191, p.p71);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(41, p.p72);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(42, p.p73);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(43, p.p74);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(192, p.p75);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(45, p.p76);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(535, p.p77);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(536, p.p78);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(189, p.p79);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(48, p.p80);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(190, p.p81);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(49, p.p82);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(193, p.p83);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(51, p.p84);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(52, p.p85);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(537, p.p86);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(194, p.p87);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(54, p.p88);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(55, p.p89);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(56, p.p90);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(57, p.p91);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(58, p.p92);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(195, p.p93);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(60, p.p94);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(61, p.p95);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(62, p.p96);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(538, p.p97);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(63, p.p98);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(64, p.p99);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(65, p.p100);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(66, p.p101);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(67, p.p102);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(75, p.p103);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(197, p.p104);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(198, p.p105);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(199, p.p106);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(200, p.p107);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(201, p.p108);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(76, p.p109);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(77, p.p123);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(78, p.p110);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(79, p.p111);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(80, p.p112);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(81, p.p122);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(82, p.p113);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(83, p.p114);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(84, p.p115);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(85, p.p116);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(86, p.p117);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(87, p.p118);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(88, p.p119);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(89, p.p124);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(90, p.p125);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(204, p.p126);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(205, p.p127);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(93, p.p128);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(94, p.p129);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(95, p.p130);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(96, p.p131);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(97, p.p132);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(98, p.p133);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(206, p.p148);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(114, p.p149);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(115, p.p150);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(99, p.p134);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(207, p.p135);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(208, p.p136);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(102, p.p137);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(103, p.p138);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(104, p.p139);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(105, p.p140);
        }

        if (s.v[607] != 0.0) {
            s.store_div_ad_lhs(106, A::mul(A::scale(s.ad_value(105), p.p141), s.ad_value(530)), 529);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(107, p.p142);
        }

        if (s.v[607] != 0.0) {
            s.store_div_ad_lhs(108, A::mul(A::scale(s.ad_value(107), p.p143), s.ad_value(530)), 529);
        }

        if (s.v[607] != 0.0) {
            s.store_scalar(109, p.p144);
        }

    }
}
