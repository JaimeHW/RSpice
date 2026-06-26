#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && s.b[1223]) {
            s.store_exp_neg_input(687, 695);
        }

        s.b[1224] = ((-s.v[695]) < (-80.0));
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1223])) && s.b[1224]) {
            s.store_div_from_scalar_offset_ad(687, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1223])) && (!s.b[1224])) {
            s.store_scaled_offset_ad(687, A::mul(A::offset(A::neg(s.ad_value(695)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {
            s.store_sub_from_scalar(693, 1.0, 687);
            s.store_sub_ad(696, A::add(s.ad_value(702), A::scale(s.ad_value(705), 0.5)), A::mul(s.ad_value(704), A::sqrt(A::sub(A::add(s.ad_value(702), A::scale(s.ad_value(705), 0.25)), s.ad_value(693)))));
        }

        s.b[1225] = ((((-s.v[696])) as f64).abs() < 80.0);
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && s.b[1225]) {
            s.store_exp_neg_input(689, 696);
        }

        s.b[1226] = ((-s.v[696]) < (-80.0));
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1225])) && s.b[1226]) {
            s.store_div_from_scalar_offset_ad(689, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1225])) && (!s.b[1226])) {
            s.store_scaled_offset_ad(689, A::mul(A::offset(A::neg(s.ad_value(696)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {
            s.store_add_ad(690, A::scale(A::sub(s.ad_value(702), s.ad_value(696)), 2.0), A::mul(s.ad_value(705), A::sub_from_scalar(1.0, s.ad_value(689))));
            s.store_sub_ad(691, A::mul(A::sub(s.ad_value(702), s.ad_value(696)), A::sub(s.ad_value(702), s.ad_value(696))), A::mul(s.ad_value(705), A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689))));
            s.store_sub_from_scalar_ad(692, 1.0, A::mul_scaled_lhs(s.ad_value(705), 0.5, s.ad_value(689)));
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::mul_scaled_output(s.ad_value(692), s.ad_value(691), 4.0));
            s.store_div_ad(697, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
            s.store_add(712, 696, 697);
        }

        if (s.b[1218] && (!s.b[1219])) {
            s.store_neg(712, 712);
        }

        s.store_div_ad_lhs(704, A::sqrt(A::mul(A::mul_scaled_lhs(s.ad_value(20), (2.0 * 1.602176565e-19), s.ad_value(225)), s.ad_value(220))), 237);

        s.store_square(705, 704);

        s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);

        s.store_scale(707, 706, 1e-5);

        s.store_div_from_scalar(708, 1.0, 706);

        s.store_div_from_scalar_offset_scaled_input(709, 1.0, 704, 0.7324648775608221, 1.25);

        s.b[1227] = (((p.p3 > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p.p4 > 0.0) && (s.v[90] > 0.0)));
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        s.b[1228] = (((s.v[701]) as f64).abs() <= s.v[707]);
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if (s.b[1227] && s.b[1228]) {
            s.store_mul_neg_lhs(711, 701, 708);
        }

        s.b[1229] = (s.v[701] < (-s.v[707]));
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if ((s.b[1227] && (!s.b[1228])) && s.b[1229]) {
            s.store_neg(679, 701);
            s.store_scaled_mul(680, 679, 708, 1.25);
            s.store_scaled_sub_ad(681, A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(680), (-6.0)), A::offset(s.ad_value(680), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(682, A::mul(A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681))), A::mul(s.ad_value(705), A::offset(s.ad_value(681), 1.0)));
            s.store_sub_ad_lhs(683, A::scale(A::sub(s.ad_value(679), s.ad_value(681)), 2.0), 705);
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
            s.store_add(685, 682, 683);
            s.store_add_ad(686, A::square(s.ad_value(685)), A::mul(s.ad_value(684), A::sub(A::mul_scaled_lhs(s.ad_value(683), 0.5, s.ad_value(683)), s.ad_value(682))));
            s.store_add_ad_rhs(687, 686, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684)), s.ad_value(684)), s.ad_value(683)), A::sub(A::scale(A::square(s.ad_value(683)), 0.3333333333333), s.ad_value(682))));
            s.store_add_ad_rhs(688, 681, A::div(A::mul(A::mul(s.ad_value(682), s.ad_value(685)), s.ad_value(684)), s.ad_value(687)));
        }

        s.b[1230] = (((s.v[688]) as f64).abs() < 80.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (((s.b[1227] && (!s.b[1228])) && s.b[1229]) && s.b[1230]) {
            s.store_exp(689, 688);
        }

        s.b[1231] = (s.v[688] < (-80.0));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && s.b[1231]) {
            s.store_div_from_scalar_offset_ad(689, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(688)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && (!s.b[1231])) {
            s.store_scaled_offset_ad(689, A::mul(A::offset(s.ad_value(688), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(688), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(688), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1227] && (!s.b[1228])) && s.b[1229]) {
            s.store_sub(687, 679, 688);
            s.store_add_scaled_ad_rhs(690, 687, 2.0, A::mul(s.ad_value(705), A::offset(s.ad_value(689), (-1.0))));
            s.store_add_ad(691, A::square(s.ad_value(687)), A::mul(s.ad_value(705), A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689))));
            s.store_sub_from_scalar_ad(692, 1.0, A::mul_scaled_lhs(s.ad_value(705), 0.5, s.ad_value(689)));
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::mul_scaled_output(s.ad_value(692), s.ad_value(691), 4.0));
            s.store_div_ad(693, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
            s.store_neg_ad(711, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {
            s.store_mul_offset_ad_lhs(694, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), (-1.0), 709);
            s.store_mul_ad(695, A::mul(s.ad_value(701), s.ad_value(708)), A::offset(A::mul(s.ad_value(694), s.ad_value(701)), 1.0));
        }

        s.b[1232] = ((((-s.v[695])) as f64).abs() < 80.0);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if (((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && s.b[1232]) {
            s.store_exp_neg_input(687, 695);
        }

        s.b[1233] = ((-s.v[695]) < (-80.0));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1232])) && s.b[1233]) {
            s.store_div_from_scalar_offset_ad(687, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1232])) && (!s.b[1233])) {
            s.store_scaled_offset_ad(687, A::mul(A::offset(A::neg(s.ad_value(695)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {
            s.store_sub_from_scalar(693, 1.0, 687);
            s.store_sub_ad(696, A::add(s.ad_value(701), A::scale(s.ad_value(705), 0.5)), A::mul(s.ad_value(704), A::sqrt(A::sub(A::add(s.ad_value(701), A::scale(s.ad_value(705), 0.25)), s.ad_value(693)))));
        }

        s.b[1234] = ((((-s.v[696])) as f64).abs() < 80.0);
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if (((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && s.b[1234]) {
            s.store_exp_neg_input(689, 696);
        }

        s.b[1235] = ((-s.v[696]) < (-80.0));
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && s.b[1235]) {
            s.store_div_from_scalar_offset_ad(689, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && (!s.b[1235])) {
            s.store_scaled_offset_ad(689, A::mul(A::offset(A::neg(s.ad_value(696)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {
            s.store_add_ad(690, A::scale(A::sub(s.ad_value(701), s.ad_value(696)), 2.0), A::mul(s.ad_value(705), A::sub_from_scalar(1.0, s.ad_value(689))));
            s.store_sub_ad(691, A::mul(A::sub(s.ad_value(701), s.ad_value(696)), A::sub(s.ad_value(701), s.ad_value(696))), A::mul(s.ad_value(705), A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689))));
            s.store_sub_from_scalar_ad(692, 1.0, A::mul_scaled_lhs(s.ad_value(705), 0.5, s.ad_value(689)));
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::mul_scaled_output(s.ad_value(692), s.ad_value(691), 4.0));
            s.store_div_ad(697, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
            s.store_add(711, 696, 697);
        }

        if (s.b[1227] && (!s.b[1228])) {
            s.store_neg(711, 711);
        }

        s.b[1236] = (s.v[160] > 0.0);
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        s.b[1237] = (((s.v[703]) as f64).abs() <= s.v[707]);
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if (s.b[1236] && s.b[1237]) {
            s.store_mul_neg_lhs(713, 703, 708);
        }

        s.b[1238] = (s.v[703] < (-s.v[707]));
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {
            s.store_neg(679, 703);
            s.store_scaled_mul(680, 679, 708, 1.25);
            s.store_scaled_sub_ad(681, A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(680), (-6.0)), A::offset(s.ad_value(680), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(682, A::mul(A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681))), A::mul(s.ad_value(705), A::offset(s.ad_value(681), 1.0)));
            s.store_sub_ad_lhs(683, A::scale(A::sub(s.ad_value(679), s.ad_value(681)), 2.0), 705);
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
            s.store_add(685, 682, 683);
            s.store_add_ad(686, A::square(s.ad_value(685)), A::mul(s.ad_value(684), A::sub(A::mul_scaled_lhs(s.ad_value(683), 0.5, s.ad_value(683)), s.ad_value(682))));
            s.store_add_ad_rhs(687, 686, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684)), s.ad_value(684)), s.ad_value(683)), A::sub(A::scale(A::square(s.ad_value(683)), 0.3333333333333), s.ad_value(682))));
            s.store_add_ad_rhs(688, 681, A::div(A::mul(A::mul(s.ad_value(682), s.ad_value(685)), s.ad_value(684)), s.ad_value(687)));
        }

        s.b[1239] = (((s.v[688]) as f64).abs() < 80.0);
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if (((s.b[1236] && (!s.b[1237])) && s.b[1238]) && s.b[1239]) {
            s.store_exp(689, 688);
        }

        s.b[1240] = (s.v[688] < (-80.0));
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && s.b[1240]) {
            s.store_div_from_scalar_offset_ad(689, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(688)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && (!s.b[1240])) {
            s.store_scaled_offset_ad(689, A::mul(A::offset(s.ad_value(688), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(688), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(688), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {
            s.store_sub(687, 679, 688);
            s.store_add_scaled_ad_rhs(690, 687, 2.0, A::mul(s.ad_value(705), A::offset(s.ad_value(689), (-1.0))));
            s.store_add_ad(691, A::square(s.ad_value(687)), A::mul(s.ad_value(705), A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689))));
            s.store_sub_from_scalar_ad(692, 1.0, A::mul_scaled_lhs(s.ad_value(705), 0.5, s.ad_value(689)));
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::mul_scaled_output(s.ad_value(692), s.ad_value(691), 4.0));
            s.store_div_ad(693, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
            s.store_neg_ad(713, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {
            s.store_mul_offset_ad_lhs(694, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), (-1.0), 709);
            s.store_mul_ad(695, A::mul(s.ad_value(703), s.ad_value(708)), A::offset(A::mul(s.ad_value(694), s.ad_value(703)), 1.0));
        }

        s.b[1241] = ((((-s.v[695])) as f64).abs() < 80.0);
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if (((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && s.b[1241]) {
            s.store_exp_neg_input(687, 695);
        }

        s.b[1242] = ((-s.v[695]) < (-80.0));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1241])) && s.b[1242]) {
            s.store_div_from_scalar_offset_ad(687, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1241])) && (!s.b[1242])) {
            s.store_scaled_offset_ad(687, A::mul(A::offset(A::neg(s.ad_value(695)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {
            s.store_sub_from_scalar(693, 1.0, 687);
            s.store_sub_ad(696, A::add(s.ad_value(703), A::scale(s.ad_value(705), 0.5)), A::mul(s.ad_value(704), A::sqrt(A::sub(A::add(s.ad_value(703), A::scale(s.ad_value(705), 0.25)), s.ad_value(693)))));
        }

        s.b[1243] = ((((-s.v[696])) as f64).abs() < 80.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && s.b[1243]) {
            s.store_exp_neg_input(689, 696);
        }

        s.b[1244] = ((-s.v[696]) < (-80.0));
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && s.b[1244]) {
            s.store_div_from_scalar_offset_ad(689, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scaled_offset_ad(689, A::mul(A::offset(A::neg(s.ad_value(696)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {
            s.store_add_ad(690, A::scale(A::sub(s.ad_value(703), s.ad_value(696)), 2.0), A::mul(s.ad_value(705), A::sub_from_scalar(1.0, s.ad_value(689))));
            s.store_sub_ad(691, A::mul(A::sub(s.ad_value(703), s.ad_value(696)), A::sub(s.ad_value(703), s.ad_value(696))), A::mul(s.ad_value(705), A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689))));
            s.store_sub_from_scalar_ad(692, 1.0, A::mul_scaled_lhs(s.ad_value(705), 0.5, s.ad_value(689)));
            s.store_sub_ad(687, A::square(s.ad_value(690)), A::mul_scaled_output(s.ad_value(692), s.ad_value(691), 4.0));
            s.store_div_ad(697, A::scale(s.ad_value(691), 2.0), A::add(s.ad_value(690), A::sqrt(s.ad_value(687))));
            s.store_add(713, 696, 697);
        }

        if (s.b[1236] && (!s.b[1237])) {
            s.store_neg(713, 713);
        }

        s.store_mul_scaled_ad_rhs(714, 219, -1.0, A::add(s.ad_value(700), s.ad_value(710)));

        s.store_mul_scaled_ad_rhs(715, 219, -1.0, A::add(s.ad_value(701), s.ad_value(711)));

        s.store_mul_scaled_ad_rhs(345, 219, -1.0, A::add(s.ad_value(702), s.ad_value(712)));

        s.store_mul_scaled_ad_rhs(346, 219, -1.0, A::add(s.ad_value(703), s.ad_value(713)));

        s.b[1245] = (p.p3 > 0.0);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        s.b[1246] = ((s.v[69] > 0.0) || (s.v[71] > 0.0));
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if (s.b[1245] && s.b[1246]) {
            s.store_add(716, 714, 281);
            s.store_scaled_sub_ad_rhs(717, 716, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(716), s.ad_value(716), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(718, A::offset(A::square(s.ad_value(714)), 0.0001), 272);
        }

        s.b[1247] = ((((0.5 * s.v[700])) as f64).abs() < 80.0);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1247]) {
            s.store_exp_scaled_input(0, 700, 0.5);
        }

        s.b[1248] = ((0.5 * s.v[700]) < (-80.0));
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1247])) && s.b[1248]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul(A::offset(A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1247])) && (!s.b[1248])) {
            s.store_scaled_offset_ad(0, A::mul(A::offset(A::scale(s.ad_value(700), 0.5), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::scale(s.ad_value(700), 0.5), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::scale(s.ad_value(700), 0.5), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);
            s.store_sub_from_scalar(3, 1.0, 2);
            s.store_add_ad(719, A::mul(s.ad_value(83), s.ad_value(2)), A::mul(s.ad_value(80), s.ad_value(3)));
            s.store_add_ad(720, A::mul(s.ad_value(84), s.ad_value(2)), A::mul(s.ad_value(82), s.ad_value(3)));
            s.store_add_ad(721, A::mul(s.ad_value(278), s.ad_value(2)), A::mul(s.ad_value(277), s.ad_value(3)));
            s.store_mul_div_ad_rhs(2, 275, A::scale(s.ad_value(81), (-1.0)), s.ad_value(718));
        }

        s.b[1249] = (s.v[720] < 0.0);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1245] && s.b[1246]) && s.b[1249]) {
            s.store_scaled_sub_ad(718, A::add(s.ad_value(718), s.ad_value(721)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(721)), A::sub(s.ad_value(718), s.ad_value(721))), 1e-6)), 0.5);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_add_ad(724, A::offset(s.ad_value(710), 3.0), A::mul(s.ad_value(717), s.ad_value(220)));
        }

        s.b[1250] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1250]) {
            s.store_exp(725, 724);
        }

        s.b[1251] = (s.v[724] < (-80.0));
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && s.b[1251]) {
            s.store_div_from_scalar_offset_ad(725, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && (!s.b[1251])) {
            s.store_scaled_offset_ad(725, A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_add_ad_lhs(724, A::add(A::offset(s.ad_value(710), 3.0), A::mul(s.ad_value(717), s.ad_value(220))), 700);
        }

        s.b[1252] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1252]) {
            s.store_exp(726, 724);
        }

        s.b[1253] = (s.v[724] < (-80.0));
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && s.b[1253]) {
            s.store_div_from_scalar_offset_ad(726, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && (!s.b[1253])) {
            s.store_scaled_offset_ad(726, A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_mul_ad_affine_product_rhs(0, 275, s.ad_value(718), A::add(s.ad_value(719), A::mul(s.ad_value(720), s.ad_value(718))), 1.0, (-1.5));
            s.store_div_ad(0, A::offset(s.ad_value(725), 1.0), A::offset(s.ad_value(726), 1.0));
        }

        s.b[1258] = (s.v[0] < 1e-80);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1258]) {
            s.store_scalar(0, 1e-80);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_mul_sub_rhs(2, 85, 328, 86);
        }

        s.b[1259] = (((s.v[2]) as f64).abs() < 80.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1259]) {
            s.store_exp(3, 2);
        }

        s.b[1260] = (s.v[2] < (-80.0));
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1259])) && s.b[1260]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1259])) && (!s.b[1260])) {
            s.store_scaled_offset_ad(3, A::mul(A::offset(s.ad_value(2), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(2), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_add_ad_lhs(4, A::mul(s.ad_value(85), s.ad_value(699)), 2);
        }

        s.b[1261] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1261]) {
            s.store_exp(5, 4);
        }

        s.b[1262] = (s.v[4] < (-80.0));
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && s.b[1262]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_scaled_offset_ad(5, A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        s.b[1263] = ((s.v[70] > 0.0) || (s.v[72] > 0.0));
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (s.b[1245] && s.b[1263]) {
            s.store_add(716, 715, 281);
            s.store_scaled_sub_ad_rhs(717, 716, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(716), s.ad_value(716), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(718, A::offset(A::square(s.ad_value(715)), 0.0001), 272);
        }

        s.b[1264] = ((((0.5 * s.v[701])) as f64).abs() < 80.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1264]) {
            s.store_exp_scaled_input(0, 701, 0.5);
        }

        s.b[1265] = ((0.5 * s.v[701]) < (-80.0));
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1264])) && s.b[1265]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul(A::offset(A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1264])) && (!s.b[1265])) {
            s.store_scaled_offset_ad(0, A::mul(A::offset(A::scale(s.ad_value(701), 0.5), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::scale(s.ad_value(701), 0.5), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::scale(s.ad_value(701), 0.5), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);
            s.store_sub_from_scalar(3, 1.0, 2);
            s.store_add_ad(719, A::mul(s.ad_value(83), s.ad_value(2)), A::mul(s.ad_value(80), s.ad_value(3)));
            s.store_add_ad(720, A::mul(s.ad_value(84), s.ad_value(2)), A::mul(s.ad_value(82), s.ad_value(3)));
            s.store_add_ad(721, A::mul(s.ad_value(278), s.ad_value(2)), A::mul(s.ad_value(277), s.ad_value(3)));
            s.store_mul_div_ad_rhs(2, 275, A::scale(s.ad_value(81), (-1.0)), s.ad_value(718));
        }

        s.b[1266] = (s.v[720] < 0.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1266]) {
            s.store_scaled_sub_ad(718, A::add(s.ad_value(718), s.ad_value(721)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(721)), A::sub(s.ad_value(718), s.ad_value(721))), 1e-6)), 0.5);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_add_ad(724, A::offset(s.ad_value(711), 3.0), A::mul(s.ad_value(717), s.ad_value(220)));
        }

        s.b[1267] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1267]) {
            s.store_exp(725, 724);
        }

        s.b[1268] = (s.v[724] < (-80.0));
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && s.b[1268]) {
            s.store_div_from_scalar_offset_ad(725, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && (!s.b[1268])) {
            s.store_scaled_offset_ad(725, A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_add_ad_lhs(724, A::add(A::offset(s.ad_value(711), 3.0), A::mul(s.ad_value(717), s.ad_value(220))), 701);
        }

        s.b[1269] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1269]) {
            s.store_exp(726, 724);
        }

        s.b[1270] = (s.v[724] < (-80.0));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && s.b[1270]) {
            s.store_div_from_scalar_offset_ad(726, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && (!s.b[1270])) {
            s.store_scaled_offset_ad(726, A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_mul_ad_affine_product_rhs(0, 275, s.ad_value(718), A::add(s.ad_value(719), A::mul(s.ad_value(720), s.ad_value(718))), 1.0, (-1.5));
            s.store_div_ad(0, A::offset(s.ad_value(725), 1.0), A::offset(s.ad_value(726), 1.0));
        }

        s.b[1275] = (s.v[0] < 1e-80);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1275]) {
            s.store_scalar(0, 1e-80);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_mul_sub_rhs(2, 85, 326, 86);
        }

        s.b[1276] = (((s.v[2]) as f64).abs() < 80.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1276]) {
            s.store_exp(3, 2);
        }

        s.b[1277] = (s.v[2] < (-80.0));
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1276])) && s.b[1277]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1276])) && (!s.b[1277])) {
            s.store_scaled_offset_ad(3, A::mul(A::offset(s.ad_value(2), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(2), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(2), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_add_ad_lhs(4, A::mul(s.ad_value(85), s.ad_value(698)), 2);
        }

        s.b[1278] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1278]) {
            s.store_exp(5, 4);
        }

        s.b[1279] = (s.v[4] < (-80.0));
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && s.b[1279]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && (!s.b[1279])) {
            s.store_scaled_offset_ad(5, A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        s.b[1280] = (s.v[68] > 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (s.b[1245] && s.b[1280]) {
            s.store_mul_neg_lhs(731, 432, 382);
        }

        s.b[1281] = (((((2.0 * s.v[731]) - s.v[407])) as f64).abs() < 80.0);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1281]) {
            s.store_exp_ad(0, A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)));
        }

        s.b[1282] = (((2.0 * s.v[731]) - s.v[407]) < (-80.0));
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && s.b[1282]) {
            let assign26610_ad_e28062: A = A::div_from_scalar(1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
            s.store_ad_value(0, assign26610_ad_e28062);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && (!s.b[1282])) {
            s.store_scaled_offset_ad(0, A::mul(A::offset(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::scale(s.ad_value(731), 2.0), s.ad_value(407)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_mul_sub_ad_rhs(732, 222, A::offset(s.ad_value(731), 0.6931471805599), A::ln(A::offset(s.ad_value(0), 1.0)));
            s.store_scaled_add(733, 388, 408, 0.5);
            s.store_mul(734, 222, 733);
            s.store_add(716, 734, 280);
            s.store_scaled_sub_ad_rhs(717, 716, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(716), s.ad_value(716), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(718, A::offset(A::square(s.ad_value(734)), 0.0001), 272);
        }

        s.b[1283] = (s.v[79] < 0.0);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1283]) {
            s.store_scaled_sub_ad(718, A::add(s.ad_value(718), s.ad_value(276)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(276)), A::sub(s.ad_value(718), s.ad_value(276))), 1e-6)), 0.5);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_add(736, 396, 230);
            s.store_sub(735, 736, 733);
            s.store_mul_add_ad_lhs(724, s.ad_value(735), A::mul(A::sub(A::sub(s.ad_value(717), s.ad_value(279)), s.ad_value(732)), s.ad_value(223)), 282);
        }

        s.b[1284] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1284]) {
            s.store_exp(725, 724);
        }

        s.b[1285] = (s.v[724] < (-80.0));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && s.b[1285]) {
            s.store_div_from_scalar_offset_ad(725, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && (!s.b[1285])) {
            s.store_scaled_offset_ad(725, A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_mul_ad_lhs(724, A::mul_scaled_lhs(A::sub(s.ad_value(331), s.ad_value(732)), -1.0, s.ad_value(223)), 282);
        }

        s.b[1286] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1286]) {
            s.store_exp(0, 724);
        }

        s.b[1287] = (s.v[724] < (-80.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && s.b[1287]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(724)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_scaled_offset_ad(0, A::mul(A::offset(s.ad_value(724), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(724), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_mul(726, 725, 0);
            s.store_mul_ad_affine_product_rhs(0, 274, s.ad_value(718), A::add(s.ad_value(78), A::mul(s.ad_value(79), s.ad_value(718))), 1.0, (-1.5));
        }

        s.b[1291] = ((s.v[736] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0)));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && (!s.b[1291])) {
            s.store_add_ad_rhs(0, 78, A::mul_scaled_lhs(s.ad_value(79), 2.0, s.ad_value(718)));
            s.store_mul_div_ad_lhs(740, s.ad_value(87), A::mul(s.ad_value(0), s.ad_value(274)), 223);
            s.store_div(741, 731, 740);
        }

        s.b[1292] = (s.v[741] < 0.001);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        s.b[1293] = (((s.v[741]) as f64).abs() < 80.0);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if ((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {
            s.store_exp(747, 741);
        }

        s.b[1294] = (s.v[741] < (-80.0));
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {
            s.store_div_from_scalar_offset_ad(747, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(741)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(741)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(741)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && (!s.b[1294])) {
            s.store_scaled_offset_ad(747, A::mul(A::offset(s.ad_value(741), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(741), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(741), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) {
            s.store_div_from_scalar(748, 1.0, 747);
            s.store_sub(0, 747, 748);
            s.store_add(3, 747, 748);
        }

        s.b[1296] = (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[714] < 0.0));
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if s.b[1296] {
            s.store_sqrt_offset_ad(751, A::add(A::square(s.ad_value(714)), A::mul(A::mul(A::square(s.ad_value(95)), s.ad_value(327)), s.ad_value(327))), 1e-6);
            s.store_div_ad_lhs(0, A::neg(s.ad_value(91)), 751);
        }

        s.b[1297] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if (s.b[1296] && s.b[1297]) {
            s.store_exp(3, 0);
        }

        s.b[1298] = (s.v[0] < (-80.0));
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if ((s.b[1296] && (!s.b[1297])) && s.b[1298]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[1296] && (!s.b[1297])) && (!s.b[1298])) {
            s.store_scaled_offset_ad(3, A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1296] {
            s.store_mul(4, 97, 699);
        }

        s.b[1299] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if (s.b[1296] && s.b[1299]) {
            s.store_exp(5, 4);
        }

        s.b[1300] = (s.v[4] < (-80.0));
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if ((s.b[1296] && (!s.b[1299])) && s.b[1300]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[1296] && (!s.b[1299])) && (!s.b[1300])) {
            s.store_scaled_offset_ad(5, A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        s.b[1301] = (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[715] < 0.0));
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if s.b[1301] {
            s.store_sqrt_offset_ad(752, A::add(A::square(s.ad_value(715)), A::mul(A::mul(A::square(s.ad_value(96)), s.ad_value(329)), s.ad_value(329))), 1e-6);
            s.store_div_ad_lhs(0, A::neg(s.ad_value(92)), 752);
        }

        s.b[1302] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (s.b[1301] && s.b[1302]) {
            s.store_exp(3, 0);
        }

        s.b[1303] = (s.v[0] < (-80.0));
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1301] && (!s.b[1302])) && s.b[1303]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(0)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[1301] && (!s.b[1302])) && (!s.b[1303])) {
            s.store_scaled_offset_ad(3, A::mul(A::offset(s.ad_value(0), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(0), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1301] {
            s.store_mul(4, 98, 698);
        }

        s.b[1304] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if (s.b[1301] && s.b[1304]) {
            s.store_exp(5, 4);
        }

        s.b[1305] = (s.v[4] < (-80.0));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1301] && (!s.b[1304])) && s.b[1305]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[1301] && (!s.b[1304])) && (!s.b[1305])) {
            s.store_scaled_offset_ad(5, A::mul(A::offset(s.ad_value(4), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(4), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        s.v[352] = 0.0;

        s.b[1306] = (p.p12 > 0.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if s.b[1306] {
            s.store_mul(754, 332, 285);
            s.store_mul_offset_ad_lhs(755, A::sqrt(A::offset(A::square(s.ad_value(332)), 0.01)), (-0.1), 285);
            s.store_scaled_sub(756, 754, 755, 0.5);
            s.store_sub_ad_lhs(757, A::sub(A::mul(A::sub(s.ad_value(331), s.ad_value(100)), s.ad_value(285)), s.ad_value(756)), 230);
            s.store_sub_ad_lhs(758, A::sub(A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(101)), s.ad_value(285)), s.ad_value(756)), 230);
            s.store_div_from_scalar_offset_input(759, 1.0, 105, 1.0);
            s.store_div_from_scalar_offset_input(760, 1.0, 106, 1.0);
            s.store_mul(761, 109, 285);
            s.store_mul_scaled_ad_rhs(0, 761, 2.0, A::offset(A::sqrt(A::offset(A::div(s.ad_value(755), s.ad_value(761)), 1.0)), (-1.0)));
            s.store_mul(762, 107, 0);
            s.store_mul(763, 108, 0);
            s.store_add_ad_lhs(764, A::mul(A::add(s.ad_value(757), s.ad_value(762)), s.ad_value(759)), 756);
            s.store_add_ad_lhs(765, A::mul(A::add(s.ad_value(758), s.ad_value(763)), s.ad_value(760)), 756);
        }

        if s.b[1306] {
            let assign27740_ad_e29663: A = A::sub(A::add(A::add(s.ad_value(765), A::mul(s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(765), A::mul(s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)))), s.ad_value(221)), A::sub(A::add(s.ad_value(765), A::mul(s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(766, assign27740_ad_e29663, 0.5);
        }

        if s.b[1306] {
            let assign27750_ad_e29700: A = A::sub(A::add(A::add(s.ad_value(764), A::mul(s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(764), A::mul(s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)))), s.ad_value(221)), A::sub(A::add(s.ad_value(764), A::mul(s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(767, assign27750_ad_e29700, 0.5);
        }

        if s.b[1306] {
            s.store_div(768, 242, 759);
            s.store_div(769, 243, 760);
            s.store_div_from_scalar(770, 1.0, 768);
            s.store_div_from_scalar(771, 1.0, 769);
            s.store_div_from_scalar_add_ad(772, 1.0, A::offset(s.ad_value(770), 1.0), s.ad_value(771));
            s.store_div_ad_rhs(773, 286, A::square(s.ad_value(386)));
            s.store_mul_sub_rhs(774, 772, 766, 767);
        }

        s.b[1307] = ((((s.v[767] - s.v[766])) as f64).abs() <= 1e-12);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if (s.b[1306] && s.b[1307]) {
            s.store_sub_ad(2, A::sub_from_scalar(1.0, A::mul(s.ad_value(772), s.ad_value(770))), A::mul(s.ad_value(772), s.ad_value(771)));
            s.store_mul_sub_ad_lhs(3, A::sub(A::add(s.ad_value(771), A::mul(A::mul_scaled_lhs(s.ad_value(770), 0.5, s.ad_value(772)), s.ad_value(770))), A::mul(A::mul_scaled_lhs(s.ad_value(771), 0.5, s.ad_value(772)), s.ad_value(771))), A::div_from_scalar(0.5, s.ad_value(772)), 774);
            s.store_div_ad_lhs(4, A::mul_scaled_lhs(A::sub(s.ad_value(2), s.ad_value(3)), 0.5, s.ad_value(773)), 772);
        }

        if (s.b[1306] && (!s.b[1307])) {
            s.store_exp_ad(2, A::mul_scaled_lhs(s.ad_value(770), -1.0, s.ad_value(774)));
            s.store_exp_ad(3, A::mul(A::sub(s.ad_value(771), A::div_from_scalar(1.0, s.ad_value(772))), s.ad_value(774)));
            s.store_div_ad(4, A::mul(s.ad_value(773), A::sub(s.ad_value(2), s.ad_value(3))), A::scale(s.ad_value(774), 2.0));
        }

        if s.b[1306] {
            s.copy_ad(775, 4);
        }

        s.b[1308] = (s.v[766] < 80.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (s.b[1306] && s.b[1308]) {
            s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(s.ad_value(766))), 1.0));
            s.store_mul_sub_from_scalar_ad_rhs(0, 780, 1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)));
        }

        s.b[1309] = (s.v[766] < 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        s.b[1310] = (s.v[766] > (-80.0));
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if (((s.b[1306] && (!s.b[1308])) && s.b[1309]) && s.b[1310]) {
            s.store_exp(780, 766);
        }

        if (((s.b[1306] && (!s.b[1308])) && s.b[1309]) && (!s.b[1310])) {
            s.store_div_from_scalar_offset_ad(780, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(766)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(766)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(766)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[1306] && (!s.b[1308])) && s.b[1309]) {
            s.store_mul(0, 775, 780);
        }

        if ((s.b[1306] && (!s.b[1308])) && (!s.b[1309])) {
            s.store_add_ad_lhs(780, A::ln(s.ad_value(775)), 766);
            s.store_mul_sub_from_scalar_ad_rhs(0, 780, 1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)));
        }

        if s.b[1306] {
            s.copy_ad(776, 0);
        }

        s.b[1311] = ((s.v[766] - s.v[407]) < 80.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if (s.b[1306] && s.b[1311]) {
            s.store_ln_ad(780, A::offset(A::mul(s.ad_value(775), A::exp(A::sub(s.ad_value(766), s.ad_value(407)))), 1.0));
            s.store_mul_sub_from_scalar_ad_rhs(0, 780, 1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)));
        }

        s.b[1312] = ((s.v[766] - s.v[407]) < 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        s.b[1313] = ((s.v[766] - s.v[407]) > (-80.0));
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if (((s.b[1306] && (!s.b[1311])) && s.b[1312]) && s.b[1313]) {
            s.store_exp_sub(780, 766, 407);
        }

        if (((s.b[1306] && (!s.b[1311])) && s.b[1312]) && (!s.b[1313])) {
            s.store_div_from_scalar_offset_ad(780, 1.80485e-35, A::mul(A::offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[1306] && (!s.b[1311])) && s.b[1312]) {
            s.store_mul(0, 775, 780);
        }

        if ((s.b[1306] && (!s.b[1311])) && (!s.b[1312])) {
            s.store_add_ad(780, A::ln(s.ad_value(775)), A::sub(s.ad_value(766), s.ad_value(407)));
            s.store_mul_sub_from_scalar_ad_rhs(0, 780, 1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)));
        }

        if s.b[1306] {
            s.copy_ad(777, 0);
            s.store_mul_ad(778, A::offset(A::scale(A::add(s.ad_value(776), s.ad_value(777)), 0.5), 1.0), A::sub(s.ad_value(776), s.ad_value(777)));
            s.store_mul_square_lhs(779, 284, 110);
            s.store_div_ad_lhs(352, A::mul(A::mul(s.ad_value(779), s.ad_value(237)), s.ad_value(778)), 418);
        }

        s.b[1314] = (p.p8 != 0.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if s.b[1314] {
            s.store_div_ad_lhs(753, A::sub(s.ad_value(335), A::mul(s.ad_value(115), s.ad_value(407))), 223);
        }

        s.b[1315] = (s.v[753] > 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (s.b[1314] && s.b[1315]) {
            s.store_div_ad(3, A::scale(s.ad_value(113), (-1.0)), A::offset(s.ad_value(753), 1e-30));
        }

        s.b[1316] = (((s.v[3]) as f64).abs() < 80.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if ((s.b[1314] && s.b[1315]) && s.b[1316]) {
            s.store_exp(0, 3);
        }

        s.b[1317] = (s.v[3] < (-80.0));
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && s.b[1317]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul(A::offset(A::neg(s.ad_value(3)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && (!s.b[1317])) {
            s.store_scaled_offset_ad(0, A::mul(A::offset(s.ad_value(3), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(3), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(3), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        s.b[1318] = (s.v[6] > 0.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if s.b[1318] {
            s.store_mul_abs_ad_lhs(0, A::mul(A::add(s.ad_value(344), s.ad_value(352)), s.ad_value(332)), 168);
        }

        s.b[1604] = (p.p11 > 0.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if s.b[1604] {
            s.copy_ad(1414, 130);
            s.copy_ad(1415, 131);
            s.copy_ad(1416, 135);
            s.copy_ad(1417, 136);
            s.copy_ad(1418, 140);
            s.copy_ad(1419, 141);
            s.copy_ad(1420, 270);
            s.copy_ad(1421, 212);
            s.copy_ad(1422, 158);
            s.store_sub_ad_lhs(1423, A::sub(A::mul(A::sub(s.ad_value(331), s.ad_value(1414)), s.ad_value(223)), s.ad_value(337)), 230);
            s.store_sub_ad_lhs(1424, A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(1415)), s.ad_value(223)), 337);
            s.store_sub(1425, 1424, 230);
        }

        s.b[1605] = (p.p2 > 0.0);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1605]) {
            s.store_scale(0, 16, p.p14);
            s.store_div_ad(1426, A::offset(s.ad_value(242), 1.0), A::offset(s.ad_value(243), 1.0));
            s.store_ln(1427, 1426);
        }

        s.b[1606] = (s.v[1427] > 1e-8);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1606]) {
            s.store_div_ad(1428, A::mul_scaled_lhs(s.ad_value(1427), 2.0, A::offset(s.ad_value(1426), 1.0)), A::offset(s.ad_value(1426), (-1.0)));
        }

        if ((s.b[1604] && s.b[1605]) && (!s.b[1606])) {
            s.store_scaled_offset(1428, 1427, 2.0, 2.0);
        }

        if (s.b[1604] && s.b[1605]) {
            s.store_div_ad_rhs(1429, 249, A::square(s.ad_value(241)));
            s.store_div_from_scalar(1430, 1.0, 242);
            s.store_div_from_scalar(1431, 1.0, 243);
            s.store_div_from_scalar_add_ad(1458, 1.0, A::offset(s.ad_value(1430), 1.0), s.ad_value(1431));
            s.store_mul_sub_rhs(1459, 1458, 1423, 1425);
            s.store_sub_ad_rhs(1432, 1423, A::mul(s.ad_value(1459), s.ad_value(1430)));
            s.store_add_ad_rhs(1433, 1425, A::mul(s.ad_value(1459), s.ad_value(1431)));
            s.store_div_from_scalar_offset_input(1338, 1.0, 242, 1.0);
            s.store_div_from_scalar_offset_input(1339, 1.0, 243, 1.0);
            s.store_offset_ln_ad(1341, A::div(A::mul(A::add(s.ad_value(242), A::mul(s.ad_value(243), s.ad_value(1339))), s.ad_value(1428)), s.ad_value(1429)), 1.5);
            s.store_offset_ln_ad(1342, A::div(A::mul(A::add(s.ad_value(243), A::mul(s.ad_value(242), s.ad_value(1338))), s.ad_value(1428)), s.ad_value(1429)), 1.5);
        }

        s.b[1607] = (((s.v[1341] - s.v[1432]) / 1.5) < 80.0);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1607]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1432)), 0.6666666666666666)));
        }

        if ((s.b[1604] && s.b[1605]) && (!s.b[1607])) {
            s.store_scaled_sub(1340, 1341, 1432, 0.6666666666666666);
        }

        if (s.b[1604] && s.b[1605]) {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 1.5));
            s.store_mul_add_ad_lhs(1344, A::mul(s.ad_value(243), s.ad_value(1425)), s.ad_value(1345), 1339);
        }

        s.b[1608] = (((s.v[1342] - s.v[1344]) / 1.5) < 80.0);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1608]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1344)), 0.6666666666666666)));
        }

        if ((s.b[1604] && s.b[1605]) && (!s.b[1608])) {
            s.store_scaled_sub(1340, 1342, 1344, 0.6666666666666666);
        }

        if (s.b[1604] && s.b[1605]) {
            s.store_sub_ad_rhs(1, 1342, A::scale(s.ad_value(1340), 1.5));
            s.store_mul(2, 0, 1);
            s.store_mul(3, 0, 1425);
            s.store_sub(1390, 2, 3);
        }

        s.b[1609] = ((((-s.v[262])) as f64).abs() < 80.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1609]) {
            s.store_exp_neg_input(1391, 262);
        }

        s.b[1610] = ((-s.v[262]) < (-80.0));
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1605]) && (!s.b[1609])) && s.b[1610]) {
            s.store_div_from_scalar_offset_ad(1391, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1609])) && (!s.b[1610])) {
            s.store_scaled_offset_ad(1391, A::mul(A::offset(A::neg(s.ad_value(262)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(262)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(s.ad_value(262)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        s.b[1611] = (((s.v[1390]) as f64).abs() <= s.v[261]);
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1611]) {
            s.store_scaled_square(1388, 260, (0.1666666666667 * 0.707106781186545));
            s.store_mul_ad(4, A::mul(s.ad_value(1390), s.ad_value(260)), A::offset(A::mul(A::mul(A::mul(s.ad_value(1390), A::sub_from_scalar(1.0, s.ad_value(1391))), s.ad_value(256)), s.ad_value(1388)), 1.0));
        }

        s.b[1612] = (s.v[1390] < (-s.v[261]));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) {
            s.store_neg(1392, 1390);
            s.store_scaled_mul(1393, 1392, 260, 1.25);
            s.store_scaled_sub_ad(1394, A::offset(s.ad_value(1393), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1393), (-6.0)), A::offset(s.ad_value(1393), (-6.0))), 64.0)), 0.5);
            s.store_sub(1387, 1392, 1394);
            s.store_add_ad(1395, A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::offset(s.ad_value(1394), 1.0)));
            s.store_sub_ad_lhs(1397, A::scale(s.ad_value(1387), 2.0), 257);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) {
            s.store_sub_ad_lhs(1398, A::ln(A::mul(s.ad_value(1395), s.ad_value(258))), 1394);
            s.store_add(1385, 1395, 1397);
            s.store_add_ad(1386, A::square(s.ad_value(1385)), A::mul(s.ad_value(1398), A::sub(A::mul_scaled_lhs(s.ad_value(1397), 0.5, s.ad_value(1397)), s.ad_value(1395))));
            s.store_add_ad_rhs(1399, 1394, A::div(A::mul(A::mul(s.ad_value(1395), s.ad_value(1385)), s.ad_value(1398)), A::add(s.ad_value(1386), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(1385), s.ad_value(1386)), s.ad_value(1398)), s.ad_value(1398)), s.ad_value(1397)), A::sub(A::scale(A::square(s.ad_value(1397)), 0.3333333333333), s.ad_value(1395))))));
        }

        s.b[1613] = (s.v[1399] < 80.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) && s.b[1613]) {
            s.store_exp(1400, 1399);
        }

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) && (!s.b[1613])) {
            s.store_scaled_offset_ad(1400, A::mul(A::offset(s.ad_value(1399), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(1399), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(1399), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) {
            s.store_div_from_scalar(1401, 1.0, 1400);
            s.store_div_from_scalar_offset_ad(1387, 1.0, A::square(s.ad_value(1399)), 2.0);
            s.store_mul_square_lhs(1402, 1399, 1387);
            s.store_mul_scaled_ad_lhs(1403, A::mul(s.ad_value(1399), s.ad_value(1387)), 1387, 4.0);
            s.store_mul_ad_product_lhs(1404, A::sub(A::scale(s.ad_value(1387), 8.0), A::scale(s.ad_value(1402), 12.0)), s.ad_value(1387), 1387);
            s.store_sub(1387, 1392, 1399);
            s.store_mul(1388, 1391, 1401);
            s.store_add_scaled_ad_rhs(1405, 1387, 2.0, A::mul(s.ad_value(257), A::add(A::sub(A::offset(s.ad_value(1400), (-1.0)), s.ad_value(1388)), A::mul(s.ad_value(1391), A::sub_from_scalar(1.0, s.ad_value(1403))))));
            s.store_sub_ad(1406, A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::add(A::add(A::offset(A::sub(s.ad_value(1400), s.ad_value(1399)), (-1.0)), s.ad_value(1388)), A::mul(s.ad_value(1391), A::sub(A::offset(s.ad_value(1399), (-1.0)), s.ad_value(1402))))));
            s.store_sub_from_scalar_ad(1387, 2.0, A::mul(s.ad_value(257), A::sub(A::add(s.ad_value(1400), s.ad_value(1388)), A::mul(s.ad_value(1391), s.ad_value(1404)))));
            s.store_sub_ad(1387, A::square(s.ad_value(1405)), A::mul_scaled_output(s.ad_value(1406), s.ad_value(1387), 2.0));
            s.store_sub_scaled_ad_rhs(4, 1399, -1.0, A::scale(A::div(s.ad_value(1406), A::add(s.ad_value(1405), A::sqrt(s.ad_value(1387)))), 2.0));
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) {
            s.store_div_from_scalar_offset_scaled_input(1407, 1.0, 256, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(1408, A::mul_scaled_lhs(s.ad_value(259), 1.25, s.ad_value(1407)), (-1.0), 1407);
            s.store_mul_ad(1409, A::mul(s.ad_value(1390), s.ad_value(260)), A::offset(A::mul(s.ad_value(1408), s.ad_value(1390)), 1.0));
        }

        s.b[1614] = ((-s.v[1409]) > (-80.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) && s.b[1614]) {
            s.store_exp_neg_input(1387, 1409);
        }

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) && (!s.b[1614])) {
            s.store_div_from_scalar_offset_ad(1387, 1.80485e-35, A::mul(A::offset(A::neg(A::neg(s.ad_value(1409))), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(1409))), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(1409))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) {
            s.store_sub_from_scalar(1410, 1.0, 1387);
            s.store_sub_ad(1411, A::add(s.ad_value(1390), A::scale(s.ad_value(257), 0.5)), A::mul(s.ad_value(256), A::sqrt(A::sub(A::add(s.ad_value(1390), A::scale(s.ad_value(257), 0.25)), s.ad_value(1410)))));
            s.store_offset(1412, 262, 3.0);
            s.store_sub_ad(1394, A::scale(A::sub(A::add(s.ad_value(1411), s.ad_value(1412)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1411), s.ad_value(1412)), A::sub(s.ad_value(1411), s.ad_value(1412))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(1412), A::sqrt(A::offset(A::square(s.ad_value(1412)), 5.0))), 0.5));
            s.store_sub(1387, 1390, 1394);
            s.store_exp_neg_input(1388, 1394);
            s.store_div_from_scalar_offset_ad(1389, 1.0, A::square(s.ad_value(1394)), 2.0);
            s.store_mul_square_lhs(1402, 1394, 1389);
            s.store_mul_scaled_ad_lhs(1403, A::mul(s.ad_value(1394), s.ad_value(1389)), 1389, 4.0);
            s.store_mul_ad_product_lhs(1404, A::sub(A::scale(s.ad_value(1389), 8.0), A::scale(s.ad_value(1402), 12.0)), s.ad_value(1389), 1389);
            s.store_max_from_scalar_ad(1395, 1e-40, A::sub(A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::sub(A::offset(A::add(s.ad_value(1388), s.ad_value(1394)), (-1.0)), A::mul(s.ad_value(1391), A::add(A::offset(s.ad_value(1394), 1.0), s.ad_value(1402)))))));
            s.store_sub_from_scalar_ad(1396, 1.0, A::mul_scaled_output(s.ad_value(257), A::sub(s.ad_value(1388), A::mul(s.ad_value(1391), s.ad_value(1404))), 0.5));
            s.store_add_scaled_ad_rhs(1397, 1387, 2.0, A::mul(s.ad_value(257), A::sub(A::sub_from_scalar(1.0, s.ad_value(1388)), A::mul(s.ad_value(1391), A::offset(s.ad_value(1403), 1.0)))));
            s.store_add_ad(1398, A::sub(s.ad_value(262), s.ad_value(1394)), A::ln(A::div(s.ad_value(1395), s.ad_value(257))));
            s.store_add(1385, 1395, 1397);
            s.store_add_ad(1386, A::square(s.ad_value(1385)), A::mul(s.ad_value(1398), A::sub(A::mul_scaled_lhs(s.ad_value(1397), 0.5, s.ad_value(1397)), A::mul(s.ad_value(1395), s.ad_value(1396)))));
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) {
            let assign29400_ad_e31905: A = A::add(s.ad_value(1394), A::div(A::mul(A::mul(s.ad_value(1395), s.ad_value(1385)), s.ad_value(1398)), A::add(s.ad_value(1386), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(1385), s.ad_value(1386)), s.ad_value(1398)), s.ad_value(1398)), s.ad_value(1397)), A::sub(A::scale(A::square(s.ad_value(1397)), 0.3333333333333), A::mul(s.ad_value(1395), s.ad_value(1396)))))));
            s.store_ad_value(1413, assign29400_ad_e31905);
        }

        s.b[1615] = (s.v[1413] < 80.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) && s.b[1615]) {
            s.store_exp(1400, 1413);
            s.store_div_from_scalar(1401, 1.0, 1400);
            s.store_mul(1400, 1391, 1400);
        }

        s.b[1616] = (s.v[1413] > (s.v[262] - 80.0));
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if (((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) && (!s.b[1615])) && s.b[1616]) {
            s.store_exp_sub(1400, 1413, 262);
            s.store_div(1401, 1391, 1400);
        }

        if (((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_div_from_scalar_offset_ad(1400, 1.80485e-35, A::mul(A::offset(A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(1401, 1.80485e-35, A::mul(A::offset(s.ad_value(1413), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(1413), (-80.0)), 0.5, A::offset(A::scale(A::offset(s.ad_value(1413), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) {
            s.store_div_from_scalar_offset_ad(1387, 1.0, A::square(s.ad_value(1413)), 2.0);
            s.store_mul_square_lhs(1402, 1413, 1387);
            s.store_mul_scaled_ad_lhs(1403, A::mul(s.ad_value(1413), s.ad_value(1387)), 1387, 4.0);
            s.store_mul_ad_product_lhs(1404, A::sub(A::scale(s.ad_value(1387), 8.0), A::scale(s.ad_value(1402), 12.0)), s.ad_value(1387), 1387);
            s.store_sub(1387, 1390, 1413);
            s.store_add_scaled_ad_rhs(1405, 1387, 2.0, A::mul(s.ad_value(257), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(1401)), s.ad_value(1400)), A::mul(s.ad_value(1391), A::offset(s.ad_value(1403), 1.0)))));
            s.store_sub_ad(1406, A::square(s.ad_value(1387)), A::mul(s.ad_value(257), A::sub(A::add(A::offset(A::add(s.ad_value(1401), s.ad_value(1413)), (-1.0)), s.ad_value(1400)), A::mul(s.ad_value(1391), A::add(A::offset(s.ad_value(1413), 1.0), s.ad_value(1402))))));
            s.store_sub_from_scalar_ad(1387, 2.0, A::mul(s.ad_value(257), A::sub(A::add(s.ad_value(1401), s.ad_value(1400)), A::mul(s.ad_value(1391), s.ad_value(1404)))));
            s.store_sub_ad(1387, A::square(s.ad_value(1405)), A::mul_scaled_output(s.ad_value(1406), s.ad_value(1387), 2.0));
            s.store_add_ad_rhs(4, 1413, A::scale(A::div(s.ad_value(1406), A::add(s.ad_value(1405), A::sqrt(s.ad_value(1387)))), 2.0));
        }

        if (s.b[1604] && s.b[1605]) {
            s.store_mul_add_rhs(1434, 0, 4, 3);
        }

        if (s.b[1604] && (!s.b[1605])) {
            s.copy_ad(1434, 1425);
        }

        if s.b[1604] {
            s.store_mul_sub_rhs(0, 244, 1423, 1434);
        }

        s.b[1617] = (p.p13 > 0.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1617]) {
            s.store_scaled_add_ad(1435, A::add(s.ad_value(0), s.ad_value(253)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253)))), 0.5);
            s.store_scaled_add_ad(1436, A::sub(s.ad_value(253), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(253)), A::sub(A::neg(s.ad_value(0)), s.ad_value(253))), A::square(s.ad_value(253)))), 0.5);
            s.store_mul_ad_rhs(2, 254, A::exp_scaled_input(A::ln(s.ad_value(1435)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 254, A::exp_scaled_input(A::ln(s.ad_value(1436)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div(1443, 241, 4);
            s.store_offset_mul(1437, 242, 2, 1.0);
            s.store_offset_mul(1438, 243, 3, 1.0);
            s.store_div_ad_lhs(1439, A::mul(s.ad_value(242), s.ad_value(4)), 1437);
            s.store_div_ad_lhs(1440, A::mul(s.ad_value(243), s.ad_value(4)), 1438);
            s.store_div_from_scalar_add_ad(1441, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1439)), 1.0), A::div_from_scalar(1.0, s.ad_value(1440)));
            s.store_offset_mul(1437, 1439, 2, 1.0);
            s.store_offset_mul(1438, 1440, 3, 1.0);
        }

        if (s.b[1604] && (!s.b[1617])) {
            s.copy_ad(1443, 241);
            s.copy_ad(1439, 242);
            s.copy_ad(1440, 243);
            s.copy_ad(1441, 244);
            s.store_scalar(1437, 1.0);
            s.store_scalar(1438, 1.0);
        }

        if s.b[1604] {
            s.store_mul_sub_rhs(1442, 1441, 1423, 1434);
        }

        s.b[1618] = (s.v[1442] > 0.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        s.b[1619] = ((-s.v[1442]) < 80.0);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1618]) && s.b[1619]) {
            s.store_ln_one_plus_exp_neg_input(0, 1442);
        }

        if ((s.b[1604] && s.b[1618]) && (!s.b[1619])) {
            s.store_neg(0, 1442);
        }

        if (s.b[1604] && s.b[1618]) {
            s.store_offset_add_ad(1444, A::sub(s.ad_value(1423), A::div(s.ad_value(1442), s.ad_value(1439))), s.ad_value(0), (-0.6931471805599));
        }

        s.b[1620] = (s.v[1442] < 80.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1618])) && s.b[1620]) {
            s.store_ln_one_plus_exp(0, 1442);
        }

        if ((s.b[1604] && (!s.b[1618])) && (!s.b[1620])) {
            s.copy_ad(0, 1442);
        }

        if (s.b[1604] && (!s.b[1618])) {
            s.store_offset_add_ad(1444, A::add(s.ad_value(1434), A::div(s.ad_value(1442), s.ad_value(1440))), s.ad_value(0), (-0.6931471805599));
        }

        if s.b[1604] {
            s.store_scaled_sub_ad(1445, A::add(s.ad_value(1444), s.ad_value(250)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1444), s.ad_value(250)), A::sub(s.ad_value(1444), s.ad_value(250))), 4.0)), 0.5);
            s.store_offset_sqrt_ad(1446, A::offset(A::div(A::scale(A::sub(s.ad_value(250), s.ad_value(1445)), 2.0), s.ad_value(251)), 1.0), (-1.0));
            s.store_add_ad_rhs(1447, 1445, A::mul(s.ad_value(251), s.ad_value(1446)));
            s.store_scaled_add_ad(0, A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), 1.0), (-0.5))), 0.01)), 0.5);
            s.store_div_from_scalar_offset_ad(1448, 1.0, A::mul(s.ad_value(1416), s.ad_value(0)), 1.0);
            s.store_div_from_scalar_offset_ad(1449, 1.0, A::mul(s.ad_value(1417), s.ad_value(0)), 1.0);
            s.store_mul_ad(0, A::mul(A::mul_scaled_lhs(s.ad_value(325), 2.0, A::offset(A::sqrt(A::offset(A::div(s.ad_value(336), s.ad_value(325)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1446)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1424)), 1.0));
            s.store_mul(1450, 1418, 0);
            s.store_mul(1451, 1419, 0);
            s.store_add_ad_lhs(1452, A::add(A::mul(A::add(A::sub(s.ad_value(1423), s.ad_value(1447)), s.ad_value(1450)), s.ad_value(1448)), s.ad_value(1447)), 337);
            s.store_add_ad_lhs(1453, A::add(A::mul(A::add(A::sub(s.ad_value(1434), s.ad_value(1447)), s.ad_value(1451)), s.ad_value(1449)), s.ad_value(1447)), 337);
        }

        if s.b[1604] {
            let assign30040_ad_e32824: A = A::sub(A::add(A::add(s.ad_value(1453), A::mul(s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(1453), A::mul(s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)))), s.ad_value(221)), A::sub(A::add(s.ad_value(1453), A::mul(s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(1454, assign30040_ad_e32824, 0.5);
        }

        if s.b[1604] {
            let assign30050_ad_e32861: A = A::sub(A::add(A::add(s.ad_value(1452), A::mul(s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)))), s.ad_value(221)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(1452), A::mul(s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)))), s.ad_value(221)), A::sub(A::add(s.ad_value(1452), A::mul(s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)))), s.ad_value(221))), 0.01)));
            s.store_scale_ad(1455, assign30050_ad_e32861, 0.5);
        }

        if s.b[1604] {
            s.store_div(1456, 1439, 1448);
            s.store_div(1457, 1440, 1449);
            s.store_div_from_scalar(1430, 1.0, 1456);
            s.store_div_from_scalar(1431, 1.0, 1457);
            s.store_div_from_scalar_add_ad(1458, 1.0, A::offset(s.ad_value(1430), 1.0), s.ad_value(1431));
            s.store_div_ad_rhs(1429, 249, A::square(s.ad_value(1443)));
            s.store_div_ad(1426, A::offset(s.ad_value(1456), 1.0), A::offset(s.ad_value(1457), 1.0));
            s.store_ln(1427, 1426);
        }

        s.b[1621] = (s.v[1427] > 1e-8);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1621]) {
            s.store_div_ad(1428, A::mul_scaled_lhs(s.ad_value(1427), 2.0, A::offset(s.ad_value(1426), 1.0)), A::offset(s.ad_value(1426), (-1.0)));
        }

        if (s.b[1604] && (!s.b[1621])) {
            s.store_scaled_offset(1428, 1427, 2.0, 2.0);
        }

        if s.b[1604] {
            s.store_mul_sub_rhs(1459, 1458, 1454, 1455);
            s.store_square(1460, 1459);
            s.store_sub_ad_rhs(1432, 1454, A::mul(s.ad_value(1459), s.ad_value(1430)));
            s.store_add_ad_rhs(1433, 1455, A::mul(s.ad_value(1459), s.ad_value(1431)));
            s.store_div_from_scalar_offset_input(1338, 1.0, 1456, 1.0);
            s.store_div_from_scalar_offset_input(1339, 1.0, 1457, 1.0);
            s.store_offset_ln_ad(1341, A::div(A::mul(A::add(s.ad_value(1456), A::mul(s.ad_value(1457), s.ad_value(1339))), s.ad_value(1428)), s.ad_value(1429)), 3.0);
            s.store_offset_ln_ad(1342, A::div(A::mul(A::add(s.ad_value(1457), A::mul(s.ad_value(1456), s.ad_value(1338))), s.ad_value(1428)), s.ad_value(1429)), 3.0);
        }

        s.b[1622] = (((s.v[1341] - s.v[1432]) * 0.3333333333333) < 80.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1622]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1432)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1622])) {
            s.store_scaled_sub(1340, 1341, 1432, 0.3333333333333);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1604] {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.b[1623] = (((s.v[1342] - s.v[1433]) * 0.3333333333333) < 80.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1623]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1433)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1623])) {
            s.store_scaled_sub(1340, 1342, 1433, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
            s.store_mul_add_ad_lhs(1343, A::mul(s.ad_value(1456), s.ad_value(1454)), s.ad_value(1346), 1338);
            s.store_mul_add_ad_lhs(1344, A::mul(s.ad_value(1457), s.ad_value(1455)), s.ad_value(1345), 1339);
        }

        s.b[1624] = (((s.v[1341] - s.v[1343]) * 0.3333333333333) < 80.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1624]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1343)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1624])) {
            s.store_scaled_sub(1340, 1341, 1343, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.b[1625] = (((s.v[1342] - s.v[1344]) * 0.3333333333333) < 80.0);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1625]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1344)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1625])) {
            s.store_scaled_sub(1340, 1342, 1344, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
            s.store_sub(1461, 1454, 1345);
            s.store_sub(1465, 1455, 1346);
            s.store_scalar(1352, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_mul(1347, 1456, 1461);
        }

        s.b[1626] = ((s.v[1454] - s.v[1461]) < 80.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1626]) {
            s.store_exp_sub(1338, 1454, 1461);
        }

        if (s.b[1604] && (!s.b[1626])) {
            s.store_scaled_offset_ad(1338, A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1627] = (s.v[1349] < (-0.005));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1627]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1628] = (s.v[1349] > 0.005);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1627])) && s.b[1628]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((s.b[1604] && (!s.b[1627])) && (!s.b[1628])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1629] = (s.v[1349] > 0.005);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1629]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1630] = (s.v[1349] < (-0.005));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1629])) && s.b[1630]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1629])) && (!s.b[1630])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1631] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1631]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (s.b[1604] && (!s.b[1631])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1632] = (s.v[1361] > 0.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1632]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (s.b[1604] && (!s.b[1632])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1461, 1461, 1376);
            s.store_mul(1347, 1456, 1461);
            s.store_mul(1377, 1457, 1465);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
            s.store_sqrt_sub_ad(1381, A::square(s.ad_value(1379)), A::mul_scaled_lhs(s.ad_value(1378), 4.0, s.ad_value(1380)));
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1633] = (s.v[1382] > 0.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1633]) {
            s.store_mul_add_ad_rhs(1373, 1382, A::sub(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1454)), s.ad_value(1461));
            s.store_add_ad_lhs(1374, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1382);
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1461)), 1341);
        }

        s.b[1634] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1633]) && s.b[1634]) {
            s.store_sub_ad_rhs(1461, 1461, A::div(s.ad_value(1373), s.ad_value(1374)));
        }

        if s.b[1604] {
            s.store_mul(1347, 1456, 1461);
            s.store_mul(1377, 1457, 1465);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
            s.store_sqrt_sub_ad(1381, A::square(s.ad_value(1379)), A::mul_scaled_lhs(s.ad_value(1378), 4.0, s.ad_value(1380)));
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
        }

        s.b[1635] = (s.v[1349] < (-0.005));
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1635]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        s.b[1636] = (s.v[1349] > 0.005);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1635])) && s.b[1636]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        if ((s.b[1604] && (!s.b[1635])) && (!s.b[1636])) {
            s.store_offset_ad(1353, A::mul_scaled_lhs(s.ad_value(1349), 0.1666666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0238095238095))))), 2.0);
            s.store_scaled_sub_from_scalar_ad(1354, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1349, 1349, A::div(A::add(A::add(A::mul(s.ad_value(1370), s.ad_value(1353)), A::mul(s.ad_value(1347), s.ad_value(1377))), s.ad_value(1349)), A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0)));
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1637] = (s.v[1382] > 0.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && s.b[1637]) {
            s.store_mul_add_ad_rhs(1373, 1382, A::sub(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1454)), s.ad_value(1461));
            s.store_add_ad_lhs(1374, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1382);
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1461)), 1341);
        }

        s.b[1638] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1637]) && s.b[1638]) {
            s.store_sub_ad_rhs(1461, 1461, A::div(s.ad_value(1373), s.ad_value(1374)));
        }

        if s.b[1604] {
            s.store_mul(1347, 1456, 1461);
        }

        s.b[1639] = ((s.v[1454] - s.v[1461]) < 80.0);
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1639]) {
            s.store_exp_sub(1338, 1454, 1461);
        }

        if (s.b[1604] && (!s.b[1639])) {
            s.store_scaled_offset_ad(1338, A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1640] = (s.v[1349] < (-0.005));
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1640]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1641] = (s.v[1349] > 0.005);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1640])) && s.b[1641]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((s.b[1604] && (!s.b[1640])) && (!s.b[1641])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1642] = (s.v[1349] > 0.005);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1642]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1643] = (s.v[1349] < (-0.005));
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1642])) && s.b[1643]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1642])) && (!s.b[1643])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1644] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1644]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (s.b[1604] && (!s.b[1644])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1645] = (s.v[1361] > 0.0);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1645]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (s.b[1604] && (!s.b[1645])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1461, 1461, 1376);
            s.store_mul(1347, 1456, 1461);
        }

        s.b[1646] = ((s.v[1454] - s.v[1461]) < 80.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1646]) {
            s.store_exp_sub(1338, 1454, 1461);
        }

        if (s.b[1604] && (!s.b[1646])) {
            s.store_scaled_offset_ad(1338, A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1647] = (s.v[1349] < (-0.005));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1647]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1648] = (s.v[1349] > 0.005);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1647])) && s.b[1648]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((s.b[1604] && (!s.b[1647])) && (!s.b[1648])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1649] = (s.v[1349] > 0.005);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1649]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1650] = (s.v[1349] < (-0.005));
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1649])) && s.b[1650]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1649])) && (!s.b[1650])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1651] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1651]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (s.b[1604] && (!s.b[1651])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1604] && (!s.b[1651])) {
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1652] = (s.v[1361] > 0.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1652]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (s.b[1604] && (!s.b[1652])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1461, 1461, 1376);
        }

        s.b[1653] = (p.p10 == 1.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        s.b[1654] = (((s.v[1376]) as f64).abs() > 0.01);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1653]) && s.b[1654]) {
            s.store_mul(1347, 1456, 1461);
        }

        s.b[1655] = ((s.v[1454] - s.v[1461]) < 80.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1655]) {
            s.store_exp_sub(1338, 1454, 1461);
        }

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1655])) {
            s.store_scaled_offset_ad(1338, A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1604] && s.b[1653]) && s.b[1654]) {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1656] = (s.v[1349] < (-0.005));
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1656]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1657] = (s.v[1349] > 0.005);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1656])) && s.b[1657]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1656])) && (!s.b[1657])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1658] = (s.v[1349] > 0.005);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1658]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1659] = (s.v[1349] < (-0.005));
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1658])) && s.b[1659]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1658])) && (!s.b[1659])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1660] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1660]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1660])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1661] = (s.v[1361] > 0.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1661]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1661])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if ((s.b[1604] && s.b[1653]) && s.b[1654]) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1461, 1461, 1376);
        }

        if s.b[1604] {
            s.store_mul(1463, 1456, 1461);
        }

        s.b[1662] = ((s.v[1454] - s.v[1461]) < 80.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1662]) {
            s.store_exp_sub(1338, 1454, 1461);
        }

        if (s.b[1604] && (!s.b[1662])) {
            s.store_scaled_offset_ad(1338, A::mul(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1467, 1429, 1338);
            s.store_sub_ad_lhs(1466, A::square(s.ad_value(1463)), 1467);
        }

        s.b[1663] = (s.v[1467] <= 0.0);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1663]) {
            s.store_scalar(1462, 1e-80);
            s.store_sub(1464, 1462, 1463);
            s.store_div(1465, 1464, 1457);
        }

        s.b[1664] = (s.v[1466] < (-0.005));
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1663])) && s.b[1664]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1466));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        s.b[1665] = (s.v[1466] > 0.005);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1466));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) {
            s.store_offset_ad(1353, A::mul_scaled_lhs(s.ad_value(1466), 0.1666666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1466), 0.0166666666667, A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0238095238095))))), 2.0);
        }

        s.b[1666] = (((1.01 * s.v[1463]) + s.v[1353]) > 0.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1663])) && s.b[1666]) {
            s.store_add(1338, 1463, 1353);
        }

        s.b[1667] = ((s.v[1467] * s.v[1463]) < (((0.9 * s.v[1463]) * s.v[1463]) * s.v[1338]));
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if (((s.b[1604] && (!s.b[1663])) && s.b[1666]) && s.b[1667]) {
            s.store_offset_div(1462, 1467, 1338, 1e-80);
            s.store_sub(1464, 1462, 1463);
            s.store_div(1465, 1464, 1457);
        }

        s.b[1668] = (s.v[1466] > 0.005);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && s.b[1668]) {
            s.store_sub_ad_lhs(1339, A::ln(A::div(A::scale(s.ad_value(1466), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))))), 1352);
        }

        s.b[1669] = (s.v[1466] < (-0.005));
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if (((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {
            s.store_sin_scaled_input(1340, 1352, 0.5);
            s.store_ln_ad(1339, A::div(A::neg(s.ad_value(1466)), A::square(s.ad_value(1340))));
        }

        if (((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && (!s.b[1668])) && (!s.b[1669])) {
            s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul_scaled_lhs(s.ad_value(1466), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1466), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0396825396825397)))))));
        }

        if (((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {
            s.store_sub_ad_lhs(1465, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1461)), A::scale(A::ln(s.ad_value(1338)), 2.0)), 1339);
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {
            s.store_mul(1464, 1457, 1465);
            s.store_add(1462, 1463, 1464);
        }

        s.b[1670] = (s.v[1466] > 0.005);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        s.b[1671] = (((s.v[1461] - s.v[1454]) - s.v[1352]) < 80.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) && s.b[1671]) {
            s.store_exp_ad(1340, A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)));
        }

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) && (!s.b[1671])) {
            let assign34270_ad_e38321: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1461), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1340, assign34270_ad_e38321, 5.54062e34);
        }

        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) {
            s.store_div(1339, 1340, 1429);
            s.store_div_ad(1338, A::mul_scaled_lhs(s.ad_value(1466), 4.0, s.ad_value(1339)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        s.b[1672] = (s.v[1466] < (-0.005));
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && (!s.b[1670])) && s.b[1672]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad_lhs(1338, A::div(A::neg(s.ad_value(1466)), A::square(s.ad_value(1339))), 1467);
        }

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1672])) {
            s.store_div_ad_lhs(1338, A::sub_from_scalar(4.0, A::mul_scaled_lhs(s.ad_value(1466), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1466), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0396825396825397)))))), 1467);
        }

        if ((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) {
            s.store_offset_div_ad(1462, A::sub(s.ad_value(1463), s.ad_value(1353)), A::sub_from_scalar(1.0, s.ad_value(1338)), 1e-80);
            s.store_sub(1464, 1462, 1463);
            s.store_div(1465, 1464, 1457);
        }

        s.b[1673] = ((s.v[1455] - s.v[1465]) < 80.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1673]) {
            s.store_exp_sub(1338, 1455, 1465);
        }

        if (s.b[1604] && (!s.b[1673])) {
            s.store_scaled_offset_ad(1338, A::mul(A::offset(A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1468, 1429, 1338);
            s.store_scalar(1471, 0.0);
            s.store_scalar(1472, 0.0);
            s.store_scalar(1469, 0.0);
            s.store_scalar(1470, 0.0);
            s.store_scalar(1473, 0.0);
            s.store_scalar(1474, 0.0);
        }

        s.b[1674] = (s.v[1462] > 1e-6);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1674]) {
            s.store_mul(1469, 1467, 1430);
            s.store_mul(1470, 1468, 1431);
            s.store_add_ad_rhs(1471, 1469, A::scale(s.ad_value(1463), 2.0));
            s.store_add_ad_rhs(1472, 1470, A::scale(s.ad_value(1464), 2.0));
            s.store_add_ad_lhs(1473, A::add(A::scale(s.ad_value(1462), 2.0), s.ad_value(1469)), 1470);
        }

        s.b[1675] = (((s.v[1466]) as f64).abs() > 0.005);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1674]) && s.b[1675]) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(1471), s.ad_value(1472)), A::mul_scaled_lhs(A::offset(s.ad_value(1461), 2.0), 2.0, s.ad_value(1472))), A::mul_scaled_lhs(A::offset(s.ad_value(1465), 2.0), 2.0, s.ad_value(1471)));
            s.store_div_ad(1474, A::mul_scaled_lhs(s.ad_value(1466), (-4.0), s.ad_value(1473)), A::mul(s.ad_value(1462), s.ad_value(2)));
        }

        if ((s.b[1604] && s.b[1674]) && (!s.b[1675])) {
            s.store_scaled_sub_from_scalar_ad(2, 1.0, A::mul_scaled_lhs(s.ad_value(1466), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1466), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1466), 0.0333333333333))))), 0.1666666666667);
            s.store_add_ad(3, A::add(A::mul(s.ad_value(1471), s.ad_value(1467)), A::mul(s.ad_value(1472), s.ad_value(1468))), A::mul(A::mul(A::mul(s.ad_value(1471), s.ad_value(1472)), s.ad_value(1462)), A::offset(A::mul(s.ad_value(1462), s.ad_value(2)), 1.0)));
            s.store_div_ad(1474, A::mul(A::mul(s.ad_value(1467), s.ad_value(1468)), s.ad_value(1473)), A::mul(s.ad_value(1462), s.ad_value(3)));
        }

        if s.b[1604] {
            s.store_ln(1475, 1462);
        }

        s.b[1676] = ((s.v[1463] / 2.0) < 80.0);
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1676]) {
            s.store_ln_one_plus_exp_scaled_input(2, 1463, 0.5);
        }

        if (s.b[1604] && (!s.b[1676])) {
            s.store_scale(2, 1463, 0.5);
        }

        if s.b[1604] {
            s.store_scale(1476, 2, 2.0);
        }

        s.b[1677] = ((s.v[1464] / 2.0) < 80.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1677]) {
            s.store_ln_one_plus_exp_scaled_input(3, 1464, 0.5);
        }

        if (s.b[1604] && (!s.b[1677])) {
            s.store_scale(3, 1464, 0.5);
        }

        if s.b[1604] {
            s.store_scale(1477, 3, 2.0);
            s.store_sub(1478, 1477, 1464);
            s.store_sub(1479, 1476, 1463);
            s.store_add_ad(1480, A::mul(s.ad_value(266), s.ad_value(1476)), A::mul(s.ad_value(267), s.ad_value(1478)));
            s.store_add_ad(1481, A::mul(s.ad_value(266), s.ad_value(1477)), A::mul(s.ad_value(267), s.ad_value(1479)));
            s.store_div_ad_rhs(0, 1462, A::add(s.ad_value(1476), s.ad_value(1477)));
            s.store_mul(1482, 1476, 0);
            s.store_mul(1483, 1477, 0);
            s.store_mul_ad(1484, A::mul(s.ad_value(1476), s.ad_value(187)), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
            s.store_mul_ad(1485, A::mul(s.ad_value(1477), s.ad_value(188)), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
            s.store_mul_add_ad_rhs(2, 50, s.ad_value(1478), A::mul(s.ad_value(51), s.ad_value(1479)));
            s.store_scaled_add_ad(3, A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01)), 0.5);
            s.store_scaled_add_ad(4, A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01)), 0.5);
            s.store_div(1486, 3, 4);
            s.store_mul_ad(1487, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(1478)), 1.0), A::mul(s.ad_value(42), s.ad_value(1479)))), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add(A::offset(A::mul(s.ad_value(1482), s.ad_value(264)), 1.0), A::mul(s.ad_value(1483), s.ad_value(265)))))));
        }

        s.b[1678] = (s.v[56] == 0.0);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1678]) {
            s.store_scalar(4, 1.0);
        }

        s.b[1679] = (s.v[56] < 0.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1678])) && s.b[1679]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1462), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((s.b[1604] && (!s.b[1678])) && (!s.b[1679])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1462), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        if s.b[1604] {
            s.store_scaled_mul_ad(1488, A::mul(s.ad_value(268), s.ad_value(1443)), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424))), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424)))), 0.01))), 0.5);
            s.store_mul_add_ad_rhs(1489, 1488, A::mul(s.ad_value(1462), s.ad_value(4)), s.ad_value(54));
            s.store_add_ad(1490, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1480)), 1e-6)))), 1.0), s.ad_value(1487)), A::mul(s.ad_value(38), s.ad_value(1489)));
            s.store_add_ad(1491, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1481)), 1e-6)))), 1.0), s.ad_value(1487)), A::mul(s.ad_value(39), s.ad_value(1489)));
            s.store_div_ad(1492, A::mul(s.ad_value(1486), A::add(s.ad_value(1484), s.ad_value(1485))), A::add(A::div(s.ad_value(1484), s.ad_value(1490)), A::div(s.ad_value(1485), s.ad_value(1491))));
        }

        s.b[1680] = (((s.v[1459]) as f64).abs() > 0.007);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        s.b[1681] = (s.v[1459] > 0.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1680]) && s.b[1681]) {
            s.store_exp_neg_input(0, 1459);
            s.store_div_ad_rhs(1493, 1459, A::sub_from_scalar(1.0, s.ad_value(0)));
            s.store_mul(1494, 0, 1493);
            s.store_add_ad_lhs(1495, A::offset(A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1493)))), (-0.6931471805599)), 1432);
        }

        if ((s.b[1604] && s.b[1680]) && (!s.b[1681])) {
            s.store_exp(0, 1459);
            s.store_div_ad_rhs(1494, 1459, A::offset(s.ad_value(0), (-1.0)));
            s.store_mul(1493, 0, 1494);
            s.store_add_ad_lhs(1495, A::offset(A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1494)))), (-0.6931471805599)), 1433);
        }

        if (s.b[1604] && s.b[1680]) {
            s.store_div_ad(1496, A::neg(s.ad_value(1459)), A::mul(s.ad_value(1458), A::sub(A::sub_from_scalar(1.0, s.ad_value(1493)), A::mul(s.ad_value(1459), s.ad_value(1431)))));
            s.store_div_ad_rhs(1497, 1459, A::mul(s.ad_value(1458), A::add(A::sub_from_scalar(1.0, s.ad_value(1494)), A::mul(s.ad_value(1459), s.ad_value(1430)))));
            s.store_div_ad_rhs(1498, 1459, A::sub(A::div(A::offset(A::mul(s.ad_value(1494), s.ad_value(1431)), 0.5), s.ad_value(1497)), A::div(A::offset(A::mul(s.ad_value(1493), s.ad_value(1430)), 0.5), s.ad_value(1496))));
        }

        if (s.b[1604] && (!s.b[1680])) {
            s.store_scale(0, 1460, (0.5 * 0.1666666666667));
            s.store_scale(2, 1459, 0.5);
            s.store_add_ad_lhs(1493, A::offset(s.ad_value(2), 1.0), 0);
            s.store_add_ad_lhs(1494, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
            s.store_scale(3, 2, 0.1666666666667);
            s.store_div_from_scalar_mul_ad(1496, 1.0, s.ad_value(1458), A::add(A::offset(s.ad_value(1431), 0.5), s.ad_value(3)));
            s.store_div_from_scalar_mul_ad(1497, 1.0, s.ad_value(1458), A::sub(A::offset(s.ad_value(1430), 0.5), s.ad_value(3)));
            s.store_add_ad(1495, A::offset(A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), A::sub_from_scalar(1.0, A::scale(s.ad_value(0), 0.5))))), (-0.6931471805599)), A::scale(A::add(s.ad_value(1432), s.ad_value(1433)), 0.5));
        }

        if (s.b[1604] && (!s.b[1680])) {
            let assign35150_ad_e39462: A = A::add(A::add(A::add(A::sub_from_scalar(4.0, A::scale(s.ad_value(1458), 3.0)), A::div(A::scale(s.ad_value(1458), 12.0), A::mul(s.ad_value(1456), s.ad_value(1457)))), A::mul(A::mul(s.ad_value(1458), A::sub(s.ad_value(1430), s.ad_value(1431))), s.ad_value(1459))), A::mul_scaled_lhs(A::sub_from_scalar(0.2, A::scale(s.ad_value(1458), 0.25)), 0.3333333333333, s.ad_value(1460)));
            s.store_div_from_scalar_ad(1498, (-12.0), assign35150_ad_e39462);
        }

        if s.b[1604] {
            s.store_div_from_scalar(1499, 1.0, 1498);
        }

        s.b[1682] = (s.v[1462] > 1e-6);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1682]) {
            s.store_div_ad(1500, A::scale(s.ad_value(1476), 100.0), A::offset(s.ad_value(1476), 100.0));
        }

        s.b[1683] = (s.v[61] < 0.0);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1682]) && s.b[1683]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1501, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1500)));
        }

        if ((s.b[1604] && s.b[1682]) && (!s.b[1683])) {
            s.store_offset_mul(1501, 61, 1500, 1.0);
        }

        if (s.b[1604] && s.b[1682]) {
            s.store_div_ad(1502, A::scale(s.ad_value(1477), 100.0), A::offset(s.ad_value(1477), 100.0));
        }

        s.b[1684] = (s.v[62] < 0.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1682]) && s.b[1684]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1503, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1502)));
        }

        if ((s.b[1604] && s.b[1682]) && (!s.b[1684])) {
            s.store_offset_mul(1503, 62, 1502, 1.0);
        }

        if (s.b[1604] && s.b[1682]) {
            s.store_sub_ad(1504, A::div(A::mul(s.ad_value(1474), s.ad_value(1473)), A::mul(s.ad_value(1471), s.ad_value(1472))), A::div(A::add(A::div(s.ad_value(1467), s.ad_value(1471)), A::div(s.ad_value(1468), s.ad_value(1472))), s.ad_value(1462)));
            s.store_div_ad(1505, A::mul(s.ad_value(1504), s.ad_value(1462)), A::offset(s.ad_value(1504), 1.0));
            s.store_sub(2, 1498, 1505);
            s.store_div_ad_lhs(1506, A::add(s.ad_value(1462), A::mul(s.ad_value(1498), s.ad_value(1495))), 2);
            s.store_scaled_add_ad_rhs(1506, 1506, A::sqrt(A::offset(A::square(s.ad_value(1506)), 1e-6)), 0.5);
            s.store_scaled_mul_ad(1507, A::div(s.ad_value(1420), s.ad_value(1492)), A::add(s.ad_value(1501), s.ad_value(1503)), 0.5);
            s.store_sub_from_scalar_ad(1508, 1.0, A::div(s.ad_value(1462), s.ad_value(1505)));
            s.store_offset(1509, 1495, 1.0);
            s.store_mul_sub_ad_lhs(1510, A::offset(A::mul(A::sub(A::scale(s.ad_value(1505), 2.0), s.ad_value(1462)), s.ad_value(1499)), (-2.0)), s.ad_value(1495), 1506);
        }

        s.b[1685] = (s.v[1507] > 1e-14);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1682]) && s.b[1685]) {
            s.store_div_from_scalar_square_ad(1511, 2.0, s.ad_value(1507));
            s.store_mul(1512, 1511, 1508);
            s.store_add(1513, 1511, 1510);
            s.store_mul(1514, 1511, 1509);
            s.store_sqrt_offset_ad(1515, A::add(A::square(s.ad_value(1512)), A::mul(A::mul_scaled_lhs(s.ad_value(1511), 0.148148148148, s.ad_value(1511)), s.ad_value(1511))), 1e-20);
            s.store_sqrt_offset_ad(1516, A::add(A::square(s.ad_value(1514)), A::mul(A::mul_scaled_lhs(s.ad_value(1513), 0.148148148148, s.ad_value(1513)), s.ad_value(1513))), 1e-20);
            s.store_sub_ad(1517, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1515), s.ad_value(1512)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1515), s.ad_value(1512)), 0.5), 0.3333333333333));
            s.store_sub_ad(1518, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1516), s.ad_value(1514)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1516), s.ad_value(1514)), 0.5), 0.3333333333333));
        }

        if ((s.b[1604] && s.b[1682]) && (!s.b[1685])) {
            s.copy_ad(1517, 1508);
            s.copy_ad(1518, 1509);
        }

        if (s.b[1604] && s.b[1682]) {
            s.store_square(4, 2);
            s.store_scaled_add_ad(1519, A::add(s.ad_value(1517), s.ad_value(1518)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(1517), s.ad_value(1518)), A::sub(s.ad_value(1517), s.ad_value(1518))), A::scale(s.ad_value(4), 10.0))), (0.94 * 0.5));
            s.store_add_ad_rhs(1520, 1462, A::mul(s.ad_value(1505), s.ad_value(1519)));
            s.store_mul_sub_rhs(1521, 1498, 1519, 1495);
            s.store_scaled_add_ad(1522, A::add(s.ad_value(1520), s.ad_value(1521)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(1520), s.ad_value(1521)), A::sub(s.ad_value(1520), s.ad_value(1521))), A::scale(s.ad_value(4), 36.0))), 0.5);
        }

        if (s.b[1604] && (!s.b[1682])) {
            s.copy_ad(1505, 1498);
            s.store_scaled_offset(1519, 1495, 1.0, 0.94);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && (!s.b[1682])) {
            s.store_add_scaled_ad_rhs(1522, 1462, 0.5, A::mul(s.ad_value(1498), A::sub(s.ad_value(1519), A::scale(s.ad_value(1495), 0.5))));
        }

        s.b[1686] = ((s.v[1522] - 0.5) < 80.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1686]) {
            s.store_ad_value(2, A::ln_one_plus_exp(A::offset(s.ad_value(1522), (-0.5))));
        }

        if (s.b[1604] && (!s.b[1686])) {
            s.store_offset(2, 1522, (-0.5));
        }

        if s.b[1604] {
            s.store_offset(3, 2, 0.5);
            s.store_add_ad_rhs(4, 1519, A::ln(A::div(s.ad_value(1462), s.ad_value(3))));
        }

        s.b[1687] = ((s.v[4] - 6.0) < 80.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1687]) {
            s.store_ad_value(2, A::ln_one_plus_exp(A::offset(s.ad_value(4), (-6.0))));
        }

        if (s.b[1604] && (!s.b[1687])) {
            s.store_offset(2, 4, (-6.0));
        }

        if s.b[1604] {
            s.store_offset(4, 2, 6.0);
        }

        s.b[1688] = ((s.v[221] - s.v[4]) < 80.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1688]) {
            s.store_ad_value(2, A::ln_one_plus_exp(A::sub(s.ad_value(221), s.ad_value(4))));
        }

        if (s.b[1604] && (!s.b[1688])) {
            s.store_sub(2, 221, 4);
        }

        if s.b[1604] {
            s.store_sub(1523, 221, 2);
            s.store_div(2, 335, 1523);
            s.store_square(3, 2);
            s.store_square(4, 3);
            s.store_square(5, 4);
            s.store_ad_value(0, A::exp_scaled_input(A::ln(A::offset(A::mul(s.ad_value(1421), s.ad_value(4)), 1.0)), 2.666666666667));
            s.store_mul_ad_rhs(1524, 335, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));
            s.store_div_from_scalar_offset_input(1338, 1.0, 1456, 1.0);
            s.store_div_from_scalar_offset_input(1339, 1.0, 1457, 1.0);
            s.store_offset_add_ad(1341, A::ln(A::div(A::mul(A::add(s.ad_value(1456), A::mul(s.ad_value(1457), s.ad_value(1339))), s.ad_value(1428)), s.ad_value(1429))), s.ad_value(1524), 3.0);
            s.store_offset_add_ad(1342, A::ln(A::div(A::mul(A::add(s.ad_value(1457), A::mul(s.ad_value(1456), s.ad_value(1338))), s.ad_value(1428)), s.ad_value(1429))), s.ad_value(1524), 3.0);
        }

        s.b[1689] = (((s.v[1341] - s.v[1432]) * 0.3333333333333) < 80.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1689]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1432)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1689])) {
            s.store_scaled_sub(1340, 1341, 1432, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.b[1690] = (((s.v[1342] - s.v[1433]) * 0.3333333333333) < 80.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1690]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1433)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1690])) {
            s.store_scaled_sub(1340, 1342, 1433, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
            s.store_mul_add_ad_lhs(1343, A::mul(s.ad_value(1456), s.ad_value(1454)), s.ad_value(1346), 1338);
            s.store_mul_add_ad_lhs(1344, A::mul(s.ad_value(1457), s.ad_value(1455)), s.ad_value(1345), 1339);
        }

        s.b[1691] = (((s.v[1341] - s.v[1343]) * 0.3333333333333) < 80.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1691]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1341), s.ad_value(1343)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1691])) {
            s.store_scaled_sub(1340, 1341, 1343, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1345, 1341, A::scale(s.ad_value(1340), 3.0));
        }

        s.b[1692] = (((s.v[1342] - s.v[1344]) * 0.3333333333333) < 80.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1692]) {
            s.store_ad_value(1340, A::ln_one_plus_exp(A::scale(A::sub(s.ad_value(1342), s.ad_value(1344)), 0.3333333333333)));
        }

        if (s.b[1604] && (!s.b[1692])) {
            s.store_scaled_sub(1340, 1342, 1344, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1346, 1342, A::scale(s.ad_value(1340), 3.0));
            s.store_sub(1525, 1454, 1345);
            s.store_sub(1526, 1455, 1346);
            s.store_scalar(1352, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_mul(1347, 1456, 1525);
        }

        s.b[1693] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1693]) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if (s.b[1604] && (!s.b[1693])) {
            let assign36020_ad_e40413: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign36020_ad_e40413, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1694] = (s.v[1349] < (-0.005));
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1694]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1695] = (s.v[1349] > 0.005);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1694])) && s.b[1695]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((s.b[1604] && (!s.b[1694])) && (!s.b[1695])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1696] = (s.v[1349] > 0.005);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1696]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1697] = (s.v[1349] < (-0.005));
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1696])) && s.b[1697]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1696])) && (!s.b[1697])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1698] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1698]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (s.b[1604] && (!s.b[1698])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1699] = (s.v[1361] > 0.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1699]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (s.b[1604] && (!s.b[1699])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1525, 1525, 1376);
            s.store_mul(1347, 1456, 1525);
            s.store_mul(1377, 1457, 1526);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
            s.store_sqrt_sub_ad(1381, A::square(s.ad_value(1379)), A::mul_scaled_lhs(s.ad_value(1378), 4.0, s.ad_value(1380)));
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1700] = (s.v[1382] > 0.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1604] && s.b[1700]) {
            s.store_mul_add_ad_rhs(1373, 1382, A::sub(A::add(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1525));
            s.store_add_ad_lhs(1374, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1382);
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1525)), 1341);
        }

        s.b[1701] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1700]) && s.b[1701]) {
            s.store_sub_ad_rhs(1525, 1525, A::div(s.ad_value(1373), s.ad_value(1374)));
        }

        if s.b[1604] {
            s.store_mul(1347, 1456, 1525);
            s.store_mul(1377, 1457, 1526);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_ad(1379, A::offset(A::scale(s.ad_value(1370), 8.5797362674), 39.478417604), A::mul(s.ad_value(1347), s.ad_value(1377)));
            s.store_scale_ad(1380, A::add(A::scale(s.ad_value(1370), 2.0), A::mul(s.ad_value(1347), s.ad_value(1377))), 39.478417604);
            s.store_sqrt_sub_ad(1381, A::square(s.ad_value(1379)), A::mul_scaled_lhs(s.ad_value(1378), 4.0, s.ad_value(1380)));
            s.store_div_ad(1349, A::sub(s.ad_value(1381), s.ad_value(1379)), A::scale(s.ad_value(1378), 2.0));
        }

        s.b[1702] = (s.v[1349] < (-0.005));
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1702]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        s.b[1703] = (s.v[1349] > 0.005);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1702])) && s.b[1703]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_div_ad_lhs(1354, A::scale(A::add(s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353)))), 0.25), 1349);
        }

        if ((s.b[1604] && (!s.b[1702])) && (!s.b[1703])) {
            s.store_offset_ad(1353, A::mul_scaled_lhs(s.ad_value(1349), 0.1666666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0238095238095))))), 2.0);
            s.store_scaled_sub_from_scalar_ad(1354, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1349, 1349, A::div(A::add(A::add(A::mul(s.ad_value(1370), s.ad_value(1353)), A::mul(s.ad_value(1347), s.ad_value(1377))), s.ad_value(1349)), A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0)));
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1704] = (s.v[1382] > 0.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1704]) {
            s.store_mul_add_ad_rhs(1373, 1382, A::sub(A::add(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1525));
            s.store_add_ad_lhs(1374, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1382);
            s.store_sub_ad_lhs(1383, A::sub(s.ad_value(1454), s.ad_value(1525)), 1341);
        }

        s.b[1705] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1704]) && s.b[1705]) {
            s.store_sub_ad_rhs(1525, 1525, A::div(s.ad_value(1373), s.ad_value(1374)));
        }

        if s.b[1604] {
            s.store_mul(1347, 1456, 1525);
        }

        s.b[1706] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1706]) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if (s.b[1604] && (!s.b[1706])) {
            let assign37190_ad_e41846: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign37190_ad_e41846, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1707] = (s.v[1349] < (-0.005));
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1707]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1708] = (s.v[1349] > 0.005);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1707])) && s.b[1708]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((s.b[1604] && (!s.b[1707])) && (!s.b[1708])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1709] = (s.v[1349] > 0.005);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1709]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1710] = (s.v[1349] < (-0.005));
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1709])) && s.b[1710]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1709])) && (!s.b[1710])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1711] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1711]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (s.b[1604] && (!s.b[1711])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1712] = (s.v[1361] > 0.0);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1712]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (s.b[1604] && (!s.b[1712])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1525, 1525, 1376);
            s.store_mul(1347, 1456, 1525);
        }

        s.b[1713] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1713]) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if (s.b[1604] && (!s.b[1713])) {
            let assign37940_ad_e42795: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign37940_ad_e42795, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1714] = (s.v[1349] < (-0.005));
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1714]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1715] = (s.v[1349] > 0.005);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1714])) && s.b[1715]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1604] && (!s.b[1714])) && s.b[1715]) {
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((s.b[1604] && (!s.b[1714])) && (!s.b[1715])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1716] = (s.v[1349] > 0.005);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1716]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1717] = (s.v[1349] < (-0.005));
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1716])) && s.b[1717]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1716])) && (!s.b[1717])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1718] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1718]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (s.b[1604] && (!s.b[1718])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1719] = (s.v[1361] > 0.0);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1719]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (s.b[1604] && (!s.b[1719])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1525, 1525, 1376);
        }

        s.b[1720] = (p.p10 == 1.0);
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        s.b[1721] = (((s.v[1376]) as f64).abs() > 0.01);
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {
            s.store_mul(1347, 1456, 1525);
        }

        s.b[1722] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1722]) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1722])) {
            let assign38710_ad_e43763: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign38710_ad_e43763, 5.54062e34);
        }

        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_ad_lhs(1350, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1347)), 1348);
            s.store_sub_ad_lhs(1351, A::mul_scaled_lhs(s.ad_value(1456), 2.0, s.ad_value(1456)), 1348);
        }

        s.b[1723] = (s.v[1349] < (-0.005));
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1723]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        s.b[1724] = (s.v[1349] > 0.005);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && s.b[1724]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
            s.store_scaled_div(1338, 1350, 1349, 0.25);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul(s.ad_value(1353), A::sub_from_scalar(2.0, s.ad_value(1353))), 1338);
            s.store_add_ad(1356, A::mul(A::sub(s.ad_value(1350), A::mul_scaled_lhs(s.ad_value(1354), 2.0, A::offset(s.ad_value(1353), 1.0))), s.ad_value(1338)), A::div(A::mul(s.ad_value(1354), s.ad_value(1351)), s.ad_value(1350)));
            s.store_sub_from_scalar_ad(1339, 1.0, A::scale(s.ad_value(1353), 0.5));
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::sub(A::mul(s.ad_value(1351), s.ad_value(1339)), A::mul(s.ad_value(1350), A::add(s.ad_value(1359), A::scale(s.ad_value(1354), 0.5)))), 1349);
        }

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && (!s.b[1724])) {
            s.store_scaled_sub_from_scalar_ad(1340, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0166666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.025))))), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_scaled_sub_from_scalar_ad(1338, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0333333333333))))), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_scaled_sub_from_scalar_ad(1339, 1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0714285714286, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0420875420875421))))), 0.0055555555556);
            s.store_sub_ad(1356, A::mul(s.ad_value(1351), s.ad_value(1338)), A::mul(A::square(s.ad_value(1350)), s.ad_value(1339)));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_ad(1360, A::mul_scaled_lhs(s.ad_value(1351), (-0.5), s.ad_value(1340)), A::mul(A::mul_scaled_lhs(s.ad_value(1350), (0.25 * 0.0055555555556), s.ad_value(1350)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.0238095238095, A::sub_from_scalar(2.0, A::scale(s.ad_value(1349), 0.075))))));
        }

        s.b[1725] = (s.v[1349] > 0.005);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1725]) {
            s.store_div_ad(1339, A::scale(s.ad_value(1349), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1726] = (s.v[1349] < (-0.005));
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1725])) && s.b[1726]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad(1357, A::neg(s.ad_value(1349)), A::square(s.ad_value(1339)));
            s.store_ln(1358, 1357);
        }

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1725])) && (!s.b[1726])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_scaled_lhs(s.ad_value(1349), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1349), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1349), 0.0396825396825397))))));
            s.store_ln(1358, 1357);
        }

        s.b[1727] = (((1.01 * s.v[1347]) + s.v[1353]) > 0.0);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1727]) {
            s.store_add(1361, 1347, 1353);
            s.store_add(1362, 1456, 1354);
            s.copy_ad(1363, 1356);
        }

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1727])) {
            s.store_div_from_scalar_sub_ad(1339, 1.0, s.ad_value(1347), s.ad_value(1353));
            s.store_sub(1340, 1354, 1456);
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_sub_ad_lhs(1362, A::sub(A::mul(s.ad_value(1340), s.ad_value(1361)), s.ad_value(1348)), A::mul(s.ad_value(1359), s.ad_value(1357)), 1339);
            s.store_mul_sub_ad_lhs(1363, A::add(A::add(A::mul(s.ad_value(1356), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1340), 2.0, s.ad_value(1362))), s.ad_value(1348)), A::mul(A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357)), 1339);
        }

        s.b[1728] = (s.v[1361] > 0.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1728]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_sub_ad(1366, A::mul(s.ad_value(1363), s.ad_value(1338)), A::square(s.ad_value(1365)));
        }

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1728])) {
            s.store_add_ad(1364, A::offset(s.ad_value(1347), 0.6931471805599), A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {
            s.store_sub_ad_lhs(1367, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(s.ad_value(1364), 2.0)), 1358);
            s.store_sub_ad_lhs(1368, A::offset(A::scale(s.ad_value(1365), 2.0), 1.0), 1359);
            s.store_sub_ad_lhs(1369, A::scale(s.ad_value(1366), 2.0), 1360);
            s.store_add_ad_rhs(1370, 1347, A::mul(s.ad_value(1457), s.ad_value(1367)));
            s.store_add_ad_rhs(1371, 1456, A::mul(s.ad_value(1457), s.ad_value(1368)));
            s.store_mul(1372, 1457, 1369);
            s.store_sub_ad_lhs(1373, A::mul(s.ad_value(1370), s.ad_value(1361)), 1348);
            s.store_add_ad_lhs(1374, A::add(A::mul(s.ad_value(1371), s.ad_value(1361)), A::mul(s.ad_value(1370), s.ad_value(1362))), 1348);
            s.store_sub_ad_lhs(1375, A::add(A::add(A::mul(s.ad_value(1372), s.ad_value(1361)), A::mul_scaled_lhs(s.ad_value(1371), 2.0, s.ad_value(1362))), A::mul(s.ad_value(1370), s.ad_value(1363))), 1348);
            s.store_sub_ad(1384, A::square(s.ad_value(1374)), A::mul_scaled_lhs(s.ad_value(1373), 0.5, s.ad_value(1375)));
            s.store_div_ad(1376, A::mul(A::mul_scaled_lhs(s.ad_value(1373), -1.0, s.ad_value(1374)), s.ad_value(1384)), A::offset(A::square(s.ad_value(1384)), 1e-200));
            s.store_add(1525, 1525, 1376);
        }

        if s.b[1604] {
            s.store_mul(1528, 1456, 1525);
        }

        s.b[1729] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1729]) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)));
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1604] && (!s.b[1729])) {
            let assign39460_ad_e44972: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1454), s.ad_value(1525)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign39460_ad_e44972, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1531, 1429, 1338);
            s.store_sub_ad_lhs(1530, A::square(s.ad_value(1528)), 1531);
        }

        s.b[1730] = (s.v[1531] <= 0.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1730]) {
            s.store_scalar(1527, 1e-80);
            s.store_sub(1529, 1527, 1528);
            s.store_div(1526, 1529, 1457);
        }

        s.b[1731] = (s.v[1530] < (-0.005));
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1730])) && s.b[1731]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1530));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
        }

        s.b[1732] = (s.v[1530] > 0.005);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1731])) && s.b[1732]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1530));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_ad(1353, A::mul(s.ad_value(1352), A::offset(s.ad_value(1355), 1.0)), A::sub_from_scalar(1.0, s.ad_value(1355)));
        }

        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_offset_ad(1353, A::mul_scaled_lhs(s.ad_value(1530), 0.1666666666667, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1530), 0.0166666666667, A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0238095238095))))), 2.0);
        }

        s.b[1733] = (((1.01 * s.v[1528]) + s.v[1353]) > 0.0);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1730])) && s.b[1733]) {
            s.store_add(1338, 1528, 1353);
        }

        s.b[1734] = ((s.v[1531] * s.v[1528]) < (((0.9 * s.v[1528]) * s.v[1528]) * s.v[1338]));
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if (((s.b[1604] && (!s.b[1730])) && s.b[1733]) && s.b[1734]) {
            s.store_offset_div(1527, 1531, 1338, 1e-80);
            s.store_sub(1529, 1527, 1528);
            s.store_div(1526, 1529, 1457);
        }

        s.b[1735] = (s.v[1530] > 0.005);
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && s.b[1735]) {
            s.store_sub_ad_lhs(1339, A::ln(A::div(A::scale(s.ad_value(1530), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))))), 1352);
        }

        s.b[1736] = (s.v[1530] < (-0.005));
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if (((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && (!s.b[1735])) && s.b[1736]) {
            s.store_sin_scaled_input(1340, 1352, 0.5);
            s.store_ln_ad(1339, A::div(A::neg(s.ad_value(1530)), A::square(s.ad_value(1340))));
        }

        if (((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1736])) {
            s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul_scaled_lhs(s.ad_value(1530), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1530), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0396825396825397)))))));
        }

        if (((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) {
            s.store_sub_ad_lhs(1526, A::add(A::add(A::sub(s.ad_value(1455), s.ad_value(1454)), s.ad_value(1525)), A::scale(A::ln(s.ad_value(1338)), 2.0)), 1339);
            s.store_mul(1529, 1457, 1526);
            s.store_add(1527, 1528, 1529);
        }

        s.b[1737] = (s.v[1530] > 0.005);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        s.b[1738] = ((((s.v[1525] + s.v[1524]) - s.v[1454]) - s.v[1352]) < 80.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) && s.b[1738]) {
            s.store_exp_ad(1340, A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)));
        }

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) && (!s.b[1738])) {
            let assign39790_ad_e45440: A = A::mul(A::offset(A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(A::add(s.ad_value(1525), s.ad_value(1524)), s.ad_value(1454)), s.ad_value(1352)), (-80.0)), 0.3333333333333), 1.0)), 1.0));
            s.store_scaled_offset_ad(1340, assign39790_ad_e45440, 1.0, 5.54062e34);
        }

        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) {
            s.store_div(1339, 1340, 1429);
            s.store_div_ad(1338, A::mul_scaled_lhs(s.ad_value(1530), 4.0, s.ad_value(1339)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1355), A::sub_from_scalar(2.0, s.ad_value(1355)))));
        }

        s.b[1739] = (s.v[1530] < (-0.005));
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && (!s.b[1737])) && s.b[1739]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_ad_lhs(1338, A::div(A::neg(s.ad_value(1530)), A::square(s.ad_value(1339))), 1531);
        }

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && (!s.b[1737])) && (!s.b[1739])) {
            s.store_div_ad_lhs(1338, A::sub_from_scalar(4.0, A::mul_scaled_lhs(s.ad_value(1530), 0.3333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1530), 0.05, A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0396825396825397)))))), 1531);
        }

        if ((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) {
            s.store_offset_div_ad(1527, A::sub(s.ad_value(1528), s.ad_value(1353)), A::sub_from_scalar(1.0, s.ad_value(1338)), 1e-80);
            s.store_sub(1529, 1527, 1528);
            s.store_div(1526, 1529, 1457);
        }

        s.b[1740] = (((s.v[1455] - s.v[1526]) - s.v[1524]) < 80.0);
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1740]) {
            s.store_exp_ad(1338, A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)));
        }

        if (s.b[1604] && (!s.b[1740])) {
            let assign39910_ad_e45658: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)), (-80.0)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)), (-80.0)), 0.5, A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(1455), s.ad_value(1526)), s.ad_value(1524)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(1338, assign39910_ad_e45658, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1532, 1429, 1338);
            s.store_scalar(1535, 0.0);
            s.store_scalar(1536, 0.0);
            s.store_scalar(1533, 0.0);
            s.store_scalar(1534, 0.0);
            s.store_scalar(1537, 0.0);
            s.store_scalar(1538, 0.0);
        }

        s.b[1741] = (s.v[1462] > 1e-6);
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1741]) {
            s.store_mul(1533, 1531, 1430);
            s.store_mul(1534, 1532, 1431);
            s.store_add_ad_rhs(1535, 1533, A::scale(s.ad_value(1528), 2.0));
            s.store_add_ad_rhs(1536, 1534, A::scale(s.ad_value(1529), 2.0));
            s.store_add_ad_lhs(1537, A::add(A::scale(s.ad_value(1527), 2.0), s.ad_value(1533)), 1534);
        }

        s.b[1742] = (((s.v[1530]) as f64).abs() > 0.005);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1741]) && s.b[1742]) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(1535), s.ad_value(1536)), A::mul_scaled_lhs(A::offset(s.ad_value(1525), 2.0), 2.0, s.ad_value(1536))), A::mul_scaled_lhs(A::offset(s.ad_value(1526), 2.0), 2.0, s.ad_value(1535)));
            s.store_div_ad(1538, A::mul_scaled_lhs(s.ad_value(1530), (-4.0), s.ad_value(1537)), A::mul(s.ad_value(1527), s.ad_value(2)));
        }

        if ((s.b[1604] && s.b[1741]) && (!s.b[1742])) {
            s.store_scaled_sub_from_scalar_ad(2, 1.0, A::mul_scaled_lhs(s.ad_value(1530), 0.0333333333333, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1530), 0.0357142857143, A::sub_from_scalar(1.0, A::scale(s.ad_value(1530), 0.0333333333333))))), 0.1666666666667);
            s.store_add_ad(3, A::add(A::mul(s.ad_value(1535), s.ad_value(1531)), A::mul(s.ad_value(1536), s.ad_value(1532))), A::mul(A::mul(A::mul(s.ad_value(1535), s.ad_value(1536)), s.ad_value(1527)), A::offset(A::mul(s.ad_value(1527), s.ad_value(2)), 1.0)));
            s.store_div_ad(1538, A::mul(A::mul(s.ad_value(1531), s.ad_value(1532)), s.ad_value(1537)), A::mul(s.ad_value(1527), s.ad_value(3)));
        }

        if s.b[1604] {
            s.store_add_ad_rhs(1539, 1524, A::ln(s.ad_value(1527)));
            s.store_scaled_add(1540, 1462, 1527, 0.5);
            s.store_sub(1541, 1539, 1475);
            s.store_scalar(1544, 1.0);
        }

        s.b[1743] = (p.p9 > 0.0);
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1743]) {
            s.store_div_ad_lhs(1542, A::scale(A::add(s.ad_value(1463), s.ad_value(1528)), 0.5), 1456);
            s.store_scaled_add_ad(1542, A::offset(s.ad_value(1542), 1e-5), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1542), (-1e-5)), A::offset(s.ad_value(1542), (-1e-5))), 1.0)), 0.5);
            s.store_sub_scaled_ad_lhs(1, A::sqrt(A::add(A::div(s.ad_value(1542), s.ad_value(223)), A::mul_scaled_lhs(s.ad_value(246), 0.25, s.ad_value(246)))), 246, 0.5);
            s.store_mul_powf_ad_lhs(1543, s.ad_value(1), 2.0, 223);
            s.store_sub_from_scalar_ad(1544, 1.0, A::div(s.ad_value(1543), s.ad_value(1542)));
        }

        s.b[1744] = ((s.v[1528] / 2.0) < 80.0);
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1744]) {
            s.store_ln_one_plus_exp_scaled_input(2, 1528, 0.5);
        }

        if (s.b[1604] && (!s.b[1744])) {
            s.store_scale(2, 1528, 0.5);
        }

        if s.b[1604] {
            s.store_scale(1545, 2, 2.0);
        }

        s.b[1745] = ((s.v[1529] / 2.0) < 80.0);
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1745]) {
            s.store_ln_one_plus_exp_scaled_input(3, 1529, 0.5);
        }

        if (s.b[1604] && (!s.b[1745])) {
            s.store_scale(3, 1529, 0.5);
        }

        if s.b[1604] {
            s.store_scale(1546, 3, 2.0);
            s.store_sub(1547, 1546, 1529);
            s.store_sub(1548, 1545, 1528);
            s.store_add_ad(1549, A::mul(s.ad_value(266), s.ad_value(1545)), A::mul(s.ad_value(267), s.ad_value(1547)));
            s.store_add_ad(1550, A::mul(s.ad_value(266), s.ad_value(1546)), A::mul(s.ad_value(267), s.ad_value(1548)));
            s.store_scaled_add(1551, 1476, 1545, 0.5);
            s.store_scaled_add(1552, 1477, 1546, 0.5);
            s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1551), s.ad_value(1552));
            s.store_mul3_lhs(1553, 1540, 1551, 0);
            s.store_mul3_lhs(1554, 1540, 1552, 0);
            s.store_scaled_add(1555, 1478, 1547, 0.5);
            s.store_scaled_add(1556, 1479, 1548, 0.5);
            s.store_scaled_add(1557, 1480, 1549, 0.5);
            s.store_scaled_add(1558, 1481, 1550, 0.5);
            s.store_mul_ad_product_lhs(1559, A::mul(s.ad_value(1551), s.ad_value(187)), A::exp(A::mul(s.ad_value(40), s.ad_value(291))), 1544);
            s.store_mul_ad(1560, A::mul(s.ad_value(1552), s.ad_value(188)), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
            s.store_add(1561, 1559, 1560);
            s.store_mul_add_ad_rhs(2, 50, s.ad_value(1555), A::mul(s.ad_value(51), s.ad_value(1556)));
            s.store_scaled_add_ad(3, A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01)), 0.5);
            s.store_scaled_add_ad(4, A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01)), 0.5);
            s.store_div(1562, 3, 4);
            s.store_mul_ad(1563, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(1555)), 1.0), A::mul(s.ad_value(42), s.ad_value(1556)))), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add(A::offset(A::mul(s.ad_value(1553), s.ad_value(264)), 1.0), A::mul(s.ad_value(1554), s.ad_value(265)))))));
        }

        s.b[1746] = (s.v[56] == 0.0);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1746]) {
            s.store_scalar(4, 1.0);
        }

        s.b[1747] = (s.v[56] < 0.0);
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1746])) && s.b[1747]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1540), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((s.b[1604] && (!s.b[1746])) && (!s.b[1747])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1540), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        if s.b[1604] {
            s.store_mul_add_ad_rhs(1564, 1488, A::mul(s.ad_value(1540), s.ad_value(4)), s.ad_value(54));
            s.store_add_ad(1565, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1557)), 1e-6)))), 1.0), s.ad_value(1563)), A::mul(s.ad_value(38), s.ad_value(1564)));
            s.store_add_ad(1566, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1558)), 1e-6)))), 1.0), s.ad_value(1563)), A::mul(s.ad_value(39), s.ad_value(1564)));
            s.store_div_ad(1567, A::mul(s.ad_value(1562), s.ad_value(1561)), A::add(A::div(s.ad_value(1559), s.ad_value(1565)), A::div(s.ad_value(1560), s.ad_value(1566))));
            s.store_div_from_scalar_offset_input(1568, 1.0, 1540, 4.0);
        }

        s.b[1748] = (s.v[65] > 0.0);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1748]) {
            s.store_div_from_scalar_offset_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1554)), 1.0);
        }

        if (s.b[1604] && (!s.b[1748])) {
            s.store_sub_from_scalar_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1554)));
        }

        if s.b[1604] {
            s.store_mul3_lhs(1569, 1540, 1568, 0);
            s.store_mul_ln_ad_lhs(1570, A::offset(A::div(A::sub(s.ad_value(335), s.ad_value(1524)), A::add(A::mul(s.ad_value(66), s.ad_value(223)), A::mul(A::mul(s.ad_value(67), s.ad_value(1540)), s.ad_value(1540)))), 1.0), 1569);
            s.store_mul(1571, 1422, 1570);
            s.store_div_from_scalar_offset_ad(1572, 1.0, A::mul(s.ad_value(1571), A::offset(s.ad_value(1571), 1.0)), 1.0);
            s.store_div_ad(1500, A::scale(s.ad_value(1551), 100.0), A::offset(s.ad_value(1551), 100.0));
        }

        s.b[1749] = (s.v[61] < 0.0);
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1749]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1501, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1500)));
        }

        if (s.b[1604] && (!s.b[1749])) {
            s.store_offset_mul(1501, 61, 1500, 1.0);
        }

        if s.b[1604] {
            s.store_div_ad(1502, A::scale(s.ad_value(1552), 100.0), A::offset(s.ad_value(1552), 100.0));
        }

        s.b[1750] = (s.v[62] < 0.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1750]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1503, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1502)));
        }

        if (s.b[1604] && (!s.b[1750])) {
            s.store_offset_mul(1503, 62, 1502, 1.0);
        }

        if s.b[1604] {
            s.store_scaled_mul_ad(1573, A::mul(s.ad_value(1420), s.ad_value(1541)), A::add(s.ad_value(1501), s.ad_value(1503)), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1604] {
            s.store_div_ad_rhs(1574, 1573, A::mul(s.ad_value(1567), s.ad_value(1572)));
            s.store_square(1575, 1574);
            s.store_sqrt_offset_input(1576, 1575, 1.0);
            s.store_div_ad_lhs(1577, A::offset(A::scale(s.ad_value(1575), 1.5), 1.0), 1576);
        }

        s.b[1751] = (p.p13 > 0.0);
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1751]) {
            s.store_mul_scaled_ad_rhs(2, 254, 0.6, A::exp_scaled_input(A::ln(A::offset(A::square(s.ad_value(1551)), 60.0)), (-0.1666666666667)));
            s.store_mul_scaled_ad_rhs(3, 254, 0.6, A::exp_scaled_input(A::ln(A::offset(A::square(s.ad_value(1552)), 60.0)), (-0.1666666666667)));
            s.store_div_ad_lhs(1578, A::offset(A::mul(s.ad_value(1456), s.ad_value(2)), 1.0), 1437);
            s.store_div_ad_lhs(1579, A::offset(A::mul(s.ad_value(1457), s.ad_value(3)), 1.0), 1438);
        }

        if (s.b[1604] && (!s.b[1751])) {
            s.store_scalar(1578, 1.0);
            s.store_scalar(1579, 1.0);
        }

        s.b[1752] = (s.v[1462] > 1e-6);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        s.b[1753] = (s.v[1527] > 1e-6);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        s.b[1754] = (((s.v[1536]) as f64).abs() < 0.01);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1752]) && s.b[1753]) && s.b[1754]) {
            s.store_div_ad(0, A::add(A::offset(s.ad_value(1525), 2.0), A::scale(s.ad_value(1535), 0.5)), A::mul(A::offset(s.ad_value(1526), 2.0), s.ad_value(1535)));
            s.store_mul(2, 0, 1536);
            s.store_square(3, 2);
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_sub_ad_rhs(5, 4, A::mul(s.ad_value(2), s.ad_value(3)));
            s.store_div_ad(2, A::sub(s.ad_value(1529), A::mul(A::mul_scaled_lhs(s.ad_value(1530), 2.0, A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1535)))), s.ad_value(5))), A::offset(s.ad_value(1526), 2.0));
            s.store_div_ad_lhs(1580, A::sub(A::div(A::sub(A::mul(s.ad_value(1538), s.ad_value(1527)), s.ad_value(1531)), s.ad_value(1535)), s.ad_value(2)), 1527);
            s.store_div_ad(1581, A::mul(s.ad_value(1580), s.ad_value(1527)), A::offset(s.ad_value(1580), 1.0));
        }

        if (((s.b[1604] && s.b[1752]) && s.b[1753]) && (!s.b[1754])) {
            s.store_sub_ad(1580, A::div(A::mul(s.ad_value(1538), s.ad_value(1537)), A::mul(s.ad_value(1535), s.ad_value(1536))), A::div(A::add(A::div(s.ad_value(1531), s.ad_value(1535)), A::div(s.ad_value(1532), s.ad_value(1536))), s.ad_value(1527)));
            s.store_div_ad(1581, A::mul(s.ad_value(1580), s.ad_value(1527)), A::offset(s.ad_value(1580), 1.0));
        }

        if ((s.b[1604] && s.b[1752]) && (!s.b[1753])) {
            s.copy_ad(1581, 1498);
        }

        if (s.b[1604] && s.b[1752]) {
            s.store_sub(2, 1581, 1505);
            s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);
        }

        s.b[1755] = (((s.v[2]) as f64).abs() > 0.001);
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1752]) && s.b[1755]) {
            s.store_sub(4, 1527, 1462);
            s.store_sub_ad_rhs(1582, 4, A::mul(s.ad_value(1581), s.ad_value(1541)));
            s.store_sub_ad_rhs(1583, 4, A::mul(s.ad_value(1505), s.ad_value(1541)));
            s.store_sqrt_square_add(1584, 1582, 3);
            s.store_sqrt_square_add(1585, 1583, 3);
            s.store_mul_ad(1586, A::div_from_scalar(0.25, s.ad_value(2)), A::add(A::sub(A::mul(s.ad_value(1585), s.ad_value(1582)), A::mul(s.ad_value(1584), s.ad_value(1583))), A::mul(s.ad_value(3), A::ln(A::div(A::add(s.ad_value(1583), s.ad_value(1585)), A::add(s.ad_value(1582), s.ad_value(1584)))))));
        }

        if ((s.b[1604] && s.b[1752]) && (!s.b[1755])) {
            s.store_mul(4, 1541, 2);
            s.store_div_ad(1586, A::mul(A::mul_scaled_lhs(s.ad_value(1541), ((-0.25) * 0.1666666666667), s.ad_value(4)), s.ad_value(4)), A::sqrt(s.ad_value(3)));
        }

        if (s.b[1604] && (!s.b[1752])) {
            s.copy_ad(1581, 1498);
            s.store_scalar(1586, 0.0);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1587, A::add(A::add(A::mul(s.ad_value(1540), s.ad_value(1541)), s.ad_value(1586)), s.ad_value(1462)), 1527);
        }

        s.b[1756] = (s.v[1462] > 1e-6);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        s.b[1757] = (s.v[1587] > 1e-30);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1756]) && s.b[1757]) {
            s.store_div_ad_rhs(1588, 1471, A::sub(A::div(s.ad_value(1467), s.ad_value(1462)), s.ad_value(1474)));
            s.store_div_ad_rhs(1589, 1535, A::sub(A::div(s.ad_value(1531), s.ad_value(1527)), s.ad_value(1538)));
            s.store_div_ad_lhs(1590, A::sub(s.ad_value(1588), s.ad_value(1589)), 1587);
            s.store_div_ad_rhs(1591, 1472, A::sub(A::div(s.ad_value(1468), s.ad_value(1462)), s.ad_value(1474)));
            s.store_div_ad_rhs(1592, 1536, A::sub(A::div(s.ad_value(1532), s.ad_value(1527)), s.ad_value(1538)));
            s.store_div_ad_lhs(1593, A::sub(s.ad_value(1591), s.ad_value(1592)), 1587);
        }

        if ((s.b[1604] && s.b[1756]) && (!s.b[1757])) {
            s.store_scalar(1590, 0.0);
            s.store_scalar(1593, 0.0);
        }

        if (s.b[1604] && (!s.b[1756])) {
            s.store_mul_scaled_ad_rhs(1594, 1493, (-2.0), A::add(A::div(s.ad_value(1430), s.ad_value(1496)), s.ad_value(1499)));
            s.store_mul_scaled_ad_rhs(1595, 1494, (-2.0), A::add(A::div(s.ad_value(1431), s.ad_value(1497)), s.ad_value(1499)));
            s.store_mul_sub_lhs(0, 1595, 1594, 1499);
            s.store_mul(2, 1594, 1430);
            s.store_mul(3, 1595, 1431);
            s.store_add(4, 2, 3);
            s.store_offset_scaled_ad(5, A::add(A::mul(s.ad_value(1493), s.ad_value(1430)), A::mul(s.ad_value(1494), s.ad_value(1431))), 2.0, 3.0);
            s.store_div_ad_lhs(1596, A::sub(A::add(s.ad_value(3), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(1496))), 5);
            s.store_div_ad_lhs(1597, A::sub(A::sub(s.ad_value(2), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(1497))), 5);
            s.store_mul_scaled_ad_rhs(1590, 1496, -1.0, A::add(A::mul(s.ad_value(1596), s.ad_value(1496)), s.ad_value(1499)));
            s.store_mul_scaled_ad_rhs(1593, 1497, -1.0, A::add(A::mul(s.ad_value(1597), s.ad_value(1497)), s.ad_value(1499)));
        }

        if s.b[1604] {
            s.store_mul(1598, 1590, 1577);
            s.store_mul(1599, 1593, 1577);
            s.store_scaled_sub(1600, 1528, 1463, 0.5);
            s.store_scaled_sub(1601, 1529, 1464, 0.5);
            s.store_mul(1602, 1600, 1598);
            s.store_mul(1603, 1601, 1599);
            s.copy_ad(436, 1424);
            s.copy_ad(437, 1428);
            s.copy_ad(438, 1429);
            s.copy_ad(439, 1430);
            s.copy_ad(440, 1431);
            s.copy_ad(441, 1458);
            s.copy_ad(442, 1459);
            s.copy_ad(443, 1443);
            s.copy_ad(444, 1442);
            s.copy_ad(445, 1446);
            s.copy_ad(446, 1447);
            s.copy_ad(447, 1448);
            s.copy_ad(448, 1449);
            s.copy_ad(449, 1450);
            s.copy_ad(450, 1453);
            s.copy_ad(451, 1455);
            s.copy_ad(452, 1456);
            s.copy_ad(453, 1457);
            s.copy_ad(454, 1463);
            s.copy_ad(455, 1464);
            s.copy_ad(456, 1475);
            s.copy_ad(457, 1528);
            s.copy_ad(458, 1529);
            s.copy_ad(459, 1539);
            s.copy_ad(460, 1540);
            s.copy_ad(461, 1544);
            s.copy_ad(462, 1553);
            s.copy_ad(463, 1554);
            s.copy_ad(464, 1575);
            s.copy_ad(465, 1578);
            s.copy_ad(466, 1579);
            s.copy_ad(467, 1600);
            s.copy_ad(468, 1601);
            s.copy_ad(469, 1602);
            s.copy_ad(470, 1603);
        }

        if (!s.b[1604]) {
            s.copy_ad(436, 379);
            s.copy_ad(437, 380);
            s.copy_ad(438, 381);
            s.copy_ad(439, 382);
            s.copy_ad(440, 383);
            s.copy_ad(441, 384);
            s.copy_ad(442, 385);
            s.copy_ad(443, 386);
            s.copy_ad(444, 387);
            s.copy_ad(445, 389);
            s.copy_ad(446, 390);
            s.copy_ad(447, 391);
            s.copy_ad(448, 392);
            s.copy_ad(449, 393);
            s.copy_ad(450, 394);
            s.copy_ad(451, 395);
            s.copy_ad(452, 397);
            s.copy_ad(453, 398);
            s.copy_ad(454, 400);
            s.copy_ad(455, 401);
            s.copy_ad(456, 402);
            s.copy_ad(457, 404);
            s.copy_ad(458, 405);
            s.copy_ad(459, 410);
            s.copy_ad(460, 411);
            s.copy_ad(461, 412);
            s.copy_ad(462, 415);
            s.copy_ad(463, 416);
        }

    }

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1604]) {
            s.copy_ad(464, 424);
            s.copy_ad(465, 426);
            s.copy_ad(466, 427);
            s.copy_ad(467, 432);
            s.copy_ad(468, 433);
            s.copy_ad(469, 434);
            s.copy_ad(470, 435);
        }

        s.store_div_ad(0, A::mul(s.ad_value(120), A::sub(s.ad_value(444), s.ad_value(442))), A::offset(A::scale(s.ad_value(460), 0.25), 1.0));

        s.store_add_ad_lhs(1320, A::scale(A::add(s.ad_value(454), s.ad_value(457)), 0.5), 0);

        s.store_sub_ad_lhs(1321, A::scale(A::add(s.ad_value(455), s.ad_value(458)), 0.5), 0);

        s.b[1758] = (p.p13 > 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if s.b[1758] {
            s.store_sub_ad_lhs(1322, A::add(s.ad_value(1320), A::div(s.ad_value(462), s.ad_value(465))), 462);
            s.store_sub_ad_lhs(1323, A::add(s.ad_value(1321), A::div(s.ad_value(463), s.ad_value(466))), 463);
        }

        if (!s.b[1758]) {
            s.copy_ad(1322, 1320);
            s.copy_ad(1323, 1321);
        }

        s.store_scaled_mul(2, 467, 469, 0.3333333333333);

        s.store_mul_scaled_ad_rhs(3, 467, 0.1666666666667, A::offset(A::mul(s.ad_value(469), A::sub_from_scalar(1.0, A::scale(s.ad_value(469), 0.2))), 1.0));

        s.store_add_ad_lhs(1324, A::mul_scaled_lhs(s.ad_value(1322), 0.5, s.ad_value(461)), 3);

        s.store_add_ad_lhs(1322, A::mul(s.ad_value(1322), s.ad_value(461)), 2);

        s.store_scaled_mul(2, 468, 470, 0.3333333333333);

        s.store_mul_scaled_ad_rhs(3, 468, 0.1666666666667, A::offset(A::mul(s.ad_value(470), A::sub_from_scalar(1.0, A::scale(s.ad_value(470), 0.2))), 1.0));

        s.store_add_ad_lhs(1325, A::scale(s.ad_value(1323), 0.5), 3);

        s.store_add(1323, 1323, 2);

        s.store_mul(0, 443, 283);

        s.store_mul(357, 0, 1322);

        s.store_mul(358, 0, 1323);

        s.store_mul_scaled_ad_rhs(359, 0, -1.0, A::add(s.ad_value(1324), s.ad_value(1325)));

        s.b[1759] = (s.v[119] > 0.0);
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if s.b[1759] {
            s.store_offset(0, 250, (2.0 * 0.6931471805599));
            s.store_add(1326, 456, 0);
            s.store_add(1327, 459, 0);
            s.store_scaled_sub_ad(1328, A::add(s.ad_value(1326), s.ad_value(250)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1326), s.ad_value(250)), A::sub(s.ad_value(1326), s.ad_value(250))), 9.0)), 0.5);
            s.store_scaled_sub_ad(1329, A::add(s.ad_value(1327), A::add(s.ad_value(250), s.ad_value(335))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1327), A::add(s.ad_value(250), s.ad_value(335))), A::sub(s.ad_value(1327), A::add(s.ad_value(250), s.ad_value(335)))), 9.0)), 0.5);
            s.store_mul_sqrt_ad_rhs(1330, 290, A::mul(s.ad_value(441), A::offset(s.ad_value(440), 0.5)));
            s.store_mul_sqrt_ad_rhs(1331, 290, A::mul(A::mul(A::mul(s.ad_value(441), s.ad_value(452)), s.ad_value(440)), A::offset(s.ad_value(439), 0.5)));
            s.store_mul_square_lhs(1332, 1330, 287);
            s.store_mul_square_lhs(1333, 1331, 287);
            s.store_sub(2, 288, 1328);
            s.store_sub_ad_lhs(3, A::add(s.ad_value(288), s.ad_value(335)), 1329);
            s.store_scale(0, 1332, 2.0);
            s.store_add_ad_rhs(1334, 1328, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1332)), 1.0)), (-1.0))));
            s.store_add_ad_rhs(1335, 1329, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1332)), 1.0)), (-1.0))));
            s.store_scale(0, 1333, 2.0);
            s.store_add_ad_rhs(1336, 1328, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1333)), 1.0)), (-1.0))));
            s.store_add_ad_rhs(1337, 1329, A::mul(s.ad_value(0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1333)), 1.0)), (-1.0))));
            s.store_mul(0, 289, 443);
            s.store_mul_ad_product_lhs(2, A::mul_scaled_lhs(s.ad_value(0), -1.0, s.ad_value(1330)), s.ad_value(452), 447);
            s.store_mul_ad_product_lhs(3, A::mul_scaled_lhs(s.ad_value(0), -1.0, s.ad_value(1331)), s.ad_value(453), 448);
            s.store_scaled_add_ad(0, A::sub(s.ad_value(1334), s.ad_value(1326)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1334), s.ad_value(1326)), A::sub(s.ad_value(1334), s.ad_value(1326))), 1.0)), 0.5);
            s.store_div_ad(375, A::mul(A::mul(s.ad_value(2), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1334), s.ad_value(1328)));
            s.store_scaled_add_ad(0, A::sub(s.ad_value(1335), s.ad_value(1327)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1335), s.ad_value(1327)), A::sub(s.ad_value(1335), s.ad_value(1327))), 1.0)), 0.5);
            s.store_div_ad(376, A::mul(A::mul(s.ad_value(2), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1335), s.ad_value(1329)));
            s.store_scaled_add_ad(0, A::sub(s.ad_value(1336), s.ad_value(1326)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1336), s.ad_value(1326)), A::sub(s.ad_value(1336), s.ad_value(1326))), 1.0)), 0.5);
            s.store_div_ad(377, A::mul(A::mul(s.ad_value(3), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1336), s.ad_value(1328)));
            s.store_scaled_add_ad(0, A::sub(s.ad_value(1337), s.ad_value(1327)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1337), s.ad_value(1327)), A::sub(s.ad_value(1337), s.ad_value(1327))), 1.0)), 0.5);
            s.store_div_ad(378, A::mul(A::mul(s.ad_value(3), s.ad_value(0)), s.ad_value(0)), A::sub(s.ad_value(1337), s.ad_value(1329)));
        }

        if (!s.b[1759]) {
            s.store_scalar(375, 0.0);
            s.store_scalar(376, 0.0);
            s.store_scalar(377, 0.0);
            s.store_scalar(378, 0.0);
        }

        s.store_mul(366, 164, 326);

        s.store_mul(367, 165, 328);

        let assign42690_ad_e48215: A = A::add(A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(445)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436))))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(445)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436))))), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(161), s.ad_value(445)), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436)))))), 0.2)));
        s.store_scale_ad(0, assign42690_ad_e48215, 0.5);

        s.store_mul3_lhs(368, 159, 345, 0);

        s.store_mul3_lhs(369, 160, 346, 0);

        s.store_mul(370, 117, 334);

        s.store_mul(371, 166, 332);

        s.store_mul_neg_ad_lhs(373, A::add(A::mul(s.ad_value(236), s.ad_value(9)), A::mul(s.ad_value(167), s.ad_value(11))), 327);

        s.store_mul_neg_ad_lhs(372, A::add(A::mul(s.ad_value(236), s.ad_value(10)), A::mul(s.ad_value(167), s.ad_value(12))), 329);

        s.b[1760] = (s.v[6] > 0.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if s.b[1760] {
            s.store_mul(374, 170, 215);
        }

        if (!s.b[1760]) {
            s.store_scalar(374, 0.0);
        }

        s.store_scaled_mul(357, 13, 357, p.p32);

        s.store_scaled_mul(358, 13, 358, p.p32);

        s.store_scaled_mul(359, 13, 359, p.p32);

        s.store_neg_ad(360, A::add(A::add(s.ad_value(357), s.ad_value(358)), s.ad_value(359)));

        s.store_scaled_mul(375, 13, 375, p.p32);

        s.store_scaled_mul(376, 13, 376, p.p32);

        s.store_scaled_mul(377, 13, 377, p.p32);

        s.store_scaled_mul(378, 13, 378, p.p32);

        s.store_scaled_mul(366, 13, 366, p.p32);

        s.store_scaled_mul(367, 13, 367, p.p32);

        s.store_scaled_mul(368, 13, 368, p.p32);

        s.store_scaled_mul(369, 13, 369, p.p32);

        s.store_scaled_mul(370, 13, 370, p.p32);

        s.store_scaled_mul(373, 13, 373, p.p32);

        s.store_scaled_mul(372, 13, 372, p.p32);

        s.store_scaled_mul(371, 13, 371, p.p32);

        s.store_mul(374, 13, 374);

        s.b[1769] = (s.v[330] < 0.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if s.b[1769] {
            s.copy_ad(1768, 359);
            s.copy_ad(359, 360);
            s.copy_ad(360, 1768);
            s.store_neg(371, 371);
            s.copy_ad(1768, 376);
            s.copy_ad(376, 375);
            s.copy_ad(375, 1768);
            s.copy_ad(1768, 378);
            s.copy_ad(378, 377);
            s.copy_ad(377, 1768);
        }

        s.store_scaled_mul(1770, 386, 222, 1.0 / (1.602176565e-19));

        s.store_scaled_add(1771, 403, 428, (-0.5));

        s.store_add(1772, 411, 1771);

        s.store_div(0, 411, 1772);

        s.store_scaled_add_ad_rhs(1777, 0, A::sqrt(A::offset(A::mul(s.ad_value(0), s.ad_value(0)), 1e-20)), 0.5);

        s.store_scaled_mul(1778, 432, 431, (-0.1666666666667));

        s.store_square(1779, 1778);

        s.store_offset(1780, 425, (-1.0));

        s.store_scale(1784, 1779, 12.0);

        s.store_sub_ad(2, A::add(s.ad_value(1777), s.ad_value(1784)), A::mul(A::mul_scaled_lhs(A::offset(s.ad_value(1777), 1.0), 2.0, s.ad_value(1784)), s.ad_value(1780)));

        s.store_max_with_scalar(3, 2, 1e-40);

        s.store_div_ad_lhs(1789, A::mul(A::mul(s.ad_value(452), s.ad_value(443)), s.ad_value(116)), 465);

        s.store_mul_offset_lhs(1790, 464, 1.0, 1789);

        s.store_mul_sub_from_scalar_ad_rhs(1792, 1790, 0.5, A::mul_scaled_lhs(s.ad_value(330), 0.25, s.ad_value(1778)));

        s.store_sub(1791, 1790, 1792);

        s.b[1803] = (p.p6 > 0.0);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if s.b[1803] {
            s.store_sub_ad(2, A::sub(A::scale(s.ad_value(1777), 0.08333333333333333), A::mul(s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 0.2), s.ad_value(1784)))), A::mul(A::mul_scaled_lhs(s.ad_value(1779), 1.6, A::sub(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784))), s.ad_value(1780)));
            s.store_max_with_scalar(3, 2, 1e-40);
        }

        s.copy_ad(1773, 1770);

        s.store_mul_offset_rhs(1774, 1770, 411, 1.0);

        s.store_mul_sub_rhs(1775, 1770, 399, 409);

        s.store_mul_ad(2, A::add(A::sub(s.ad_value(173), A::mul(s.ad_value(174), s.ad_value(1773))), A::mul(A::mul(s.ad_value(175), s.ad_value(1773)), s.ad_value(1773))), A::ln(A::div(A::add(s.ad_value(1774), A::scale(s.ad_value(1775), 0.5)), A::sub(s.ad_value(1774), A::scale(s.ad_value(1775), 0.5)))));

        s.store_add_ad_rhs(3, 2, A::mul(A::add(s.ad_value(174), A::mul(s.ad_value(175), A::sub(s.ad_value(1774), A::scale(s.ad_value(1773), 2.0)))), s.ad_value(1775)));

        s.store_offset_div_ad(0, A::add(A::mul(s.ad_value(176), s.ad_value(413)), A::mul(s.ad_value(177), s.ad_value(414))), A::offset(s.ad_value(411), 1.0), 1.0);

        s.store_scaled_add_ad(4, A::offset(s.ad_value(0), 0.01), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(0), (-0.01)), A::offset(s.ad_value(0), (-0.01))), 0.0001)), 0.5);

        s.store_mul_div_ad_lhs(0, A::mul(A::div(A::mul_scaled_lhs(s.ad_value(343), 1.602176565e-19, s.ad_value(344)), s.ad_value(341)), s.ad_value(3)), s.ad_value(1773), 4);

        s.store_div_from_scalar_scaled_input(1813, 1.0, 8, 8.617332384961e-5);

        s.store_sub_from_scalar_ad(1814, 1.17, A::div(A::mul_scaled_lhs(s.ad_value(8), 0.000473, s.ad_value(8)), A::offset(s.ad_value(8), 636.0)));

        s.store_sub_from_scalar_ad(1815, 0.744, A::div(A::mul_scaled_lhs(s.ad_value(8), 0.0004774, s.ad_value(8)), A::offset(s.ad_value(8), 235.0)));

        s.store_mul_add_ad_lhs(1816, A::sub(s.ad_value(1815), s.ad_value(1814)), A::scale(s.ad_value(224), (-0.4)), 15);

        s.store_add(1817, 1814, 1816);

        s.store_scaled_mul(1818, 1817, 1813, 0.5);

        s.store_sub_scaled_inputs(1819, 15, 0.05, 1816, 0.5);

        s.store_sqrt_scaled_input(0, 8, 0.0033333333333);

        s.store_mul_ad_lhs(2, A::mul_scaled_lhs(s.ad_value(0), 4.05e25, s.ad_value(0)), 0);

        s.store_mul(1820, 2, 234);

        s.store_div_ad_rhs(1821, 1813, A::offset(A::div(A::scale(s.ad_value(17), s.v[7]), s.ad_value(8)), 1.0));

        s.store_mul_ad_lhs(1823, A::mul_scaled_lhs(s.ad_value(1820), (2.0 * 1.602176565e-19), s.ad_value(225)), 1821);

        s.store_add_ad_lhs(1824, A::offset(A::ln(A::div(A::square(s.ad_value(241)), s.ad_value(1823))), (-0.6931471805599)), 1818);

        s.store_mul_div_ad_lhs(1825, A::mul_scaled_lhs(s.ad_value(29), (0.5 * 1.602176565e-19), s.ad_value(14)), A::add(s.ad_value(237), s.ad_value(238)), 1821);

        s.store_mul(1828, 35, 1821);

        s.v[1829] = 0.0;

        s.v[1822] = 0.0;

        s.b[1874] = (p.p9 > 0.0);
        s.v[1874] = if s.b[1874] { 1.0 } else { 0.0 };

        if s.b[1874] {
            s.store_mul_ad(1822, A::div_from_scalar(1.0, s.ad_value(1813)), A::ln(A::div(s.ad_value(24), s.ad_value(247))));
        }

        s.b[1875] = (p.p13 > 0.0);
        s.v[1875] = if s.b[1875] { 1.0 } else { 0.0 };

        s.b[1876] = (p.p14 == 1.0);
        s.v[1876] = if s.b[1876] { 1.0 } else { 0.0 };

        if (s.b[1875] && s.b[1876]) {
            s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));
        }

        if (s.b[1875] && (!s.b[1876])) {
            s.store_scale_ad(1829, A::exp_scaled_input(A::ln(A::div(s.ad_value(255), s.ad_value(1821))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_mul(1832, 332, 1821);

        s.store_mul_offset_ad_lhs(1833, A::sqrt(A::offset(A::square(s.ad_value(332)), 0.01)), (-0.1), 1821);

        s.store_scaled_sub(1834, 1832, 1833, 0.5);

        s.store_div_ad(1805, A::div(s.ad_value(398), s.ad_value(397)), A::offset(s.ad_value(398), 1.0));

        s.store_div_ad(1806, A::div(s.ad_value(397), s.ad_value(398)), A::offset(s.ad_value(397), 1.0));

        s.store_offset_ln_ad(1807, A::div(A::mul(A::mul(s.ad_value(397), A::offset(s.ad_value(1805), 1.0)), s.ad_value(380)), s.ad_value(381)), 2.0);

        s.store_offset_ln_ad(1808, A::div(A::mul(A::mul(s.ad_value(398), A::offset(s.ad_value(1806), 1.0)), s.ad_value(380)), s.ad_value(381)), 2.0);

        s.store_sub_ad(1809, A::mul(A::offset(s.ad_value(1805), 1.0), s.ad_value(1807)), A::mul(s.ad_value(395), s.ad_value(1805)));

        s.store_sub_ad(1810, A::mul(A::offset(A::div_from_scalar(1.0, s.ad_value(1806)), 1.0), s.ad_value(1808)), A::div(s.ad_value(395), s.ad_value(1806)));

        s.store_add_ad_lhs(1811, A::div(A::sub(A::scale(A::sub(A::add(s.ad_value(1809), s.ad_value(1810)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1809), s.ad_value(1810)), A::sub(s.ad_value(1809), s.ad_value(1810))), 38.0))), 0.5), s.ad_value(394)), s.ad_value(25)), 394);

        s.store_add_ad_lhs(1812, A::mul(s.ad_value(222), A::add(A::sub(A::div(A::sub(s.ad_value(1811), s.ad_value(390)), s.ad_value(391)), s.ad_value(393)), s.ad_value(390))), 21);

        s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));

        s.store_add_ad_lhs(0, A::mul_scaled_lhs(s.ad_value(23), p.p14, A::offset(s.ad_value(8), (-s.v[7]))), 252);

        s.store_sub_ad_lhs(1830, A::offset(A::add(A::scale(A::add(A::add(s.ad_value(179), s.ad_value(1819)), s.ad_value(239)), p.p14), s.ad_value(0)), p.p34), 1822);

        s.store_add_ad_lhs(1831, A::scale(A::add(A::add(s.ad_value(180), s.ad_value(1819)), s.ad_value(240)), p.p14), 0);

        s.store_sub_ad_lhs(1835, A::mul(A::sub(s.ad_value(1812), s.ad_value(1830)), s.ad_value(1821)), 1834);

        s.store_sub_ad_lhs(1836, A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(1831)), s.ad_value(1821)), 1834);

        s.b[1877] = (p.p2 > 0.0);
        s.v[1877] = if s.b[1877] { 1.0 } else { 0.0 };

        if s.b[1877] {
            s.store_div_ad_lhs(0, A::mul_scaled_lhs(s.ad_value(16), p.p14, A::sub(s.ad_value(1835), s.ad_value(1836))), 256);
        }

        s.b[1878] = (s.v[0] < 0.0);
        s.v[1878] = if s.b[1878] { 1.0 } else { 0.0 };

        if (s.b[1877] && s.b[1878]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if (s.b[1877] && (!s.b[1878])) {
            s.store_div_ad(2, A::square(s.ad_value(0)), A::offset(A::div(A::scale(s.ad_value(0), 2.0), s.ad_value(256)), 1.0));
        }

        if s.b[1877] {
            s.store_add_ad_rhs(1837, 1836, A::mul_scaled_lhs(s.ad_value(16), p.p14, s.ad_value(2)));
        }

        if (!s.b[1877]) {
            s.copy_ad(1837, 1836);
        }

        s.store_mul_sub_rhs(0, 244, 1835, 1837);

        s.b[1879] = (p.p13 > 0.0);
        s.v[1879] = if s.b[1879] { 1.0 } else { 0.0 };

        if s.b[1879] {
            s.store_scaled_add_ad(1838, A::add(s.ad_value(0), s.ad_value(253)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253)))), 0.5);
            s.store_scaled_add_ad(1839, A::sub(s.ad_value(253), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(253)), A::sub(A::neg(s.ad_value(0)), s.ad_value(253))), A::square(s.ad_value(253)))), 0.5);
            s.store_mul_ad_rhs(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

    }

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1879] {
            s.store_div_ad(1841, A::mul(s.ad_value(242), s.ad_value(4)), A::offset(A::mul(s.ad_value(242), s.ad_value(2)), 1.0));
            s.store_div_ad(1842, A::mul(s.ad_value(243), s.ad_value(4)), A::offset(A::mul(s.ad_value(243), s.ad_value(3)), 1.0));
            s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));
        }

        if (!s.b[1879]) {
            s.copy_ad(1841, 242);
            s.copy_ad(1842, 243);
            s.copy_ad(1843, 244);
        }

        s.store_mul_sub_rhs(1844, 1843, 1835, 1837);

        s.b[1880] = (s.v[1844] > 0.0);
        s.v[1880] = if s.b[1880] { 1.0 } else { 0.0 };

        s.b[1881] = ((-s.v[1844]) < 80.0);
        s.v[1881] = if s.b[1881] { 1.0 } else { 0.0 };

        if (s.b[1880] && s.b[1881]) {
            s.store_ln_one_plus_exp_neg_input(0, 1844);
        }

        if (s.b[1880] && (!s.b[1881])) {
            s.store_neg(0, 1844);
        }

        if s.b[1880] {
            s.store_offset_add_ad(1845, A::sub(s.ad_value(1835), A::div(s.ad_value(1844), s.ad_value(1841))), s.ad_value(0), (-0.6931471805599));
        }

        s.b[1882] = (s.v[1844] < 80.0);
        s.v[1882] = if s.b[1882] { 1.0 } else { 0.0 };

        if ((!s.b[1880]) && s.b[1882]) {
            s.store_ln_one_plus_exp(0, 1844);
        }

        if ((!s.b[1880]) && (!s.b[1882])) {
            s.copy_ad(0, 1844);
        }

        if (!s.b[1880]) {
            s.store_offset_add_ad(1845, A::add(s.ad_value(1837), A::div(s.ad_value(1844), s.ad_value(1842))), s.ad_value(0), (-0.6931471805599));
        }

        s.store_scaled_sub_ad(1846, A::add(s.ad_value(1845), s.ad_value(1824)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1845), s.ad_value(1824)), A::sub(s.ad_value(1845), s.ad_value(1824))), 4.0)), 0.5);

        s.store_offset_sqrt_ad(1847, A::offset(A::div(A::scale(A::sub(s.ad_value(1824), s.ad_value(1846)), 2.0), s.ad_value(1825)), 1.0), (-1.0));

        s.store_scaled_add_ad(0, A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5))), 0.01)), 0.5);

        s.store_mul_ad(0, A::mul(A::mul_scaled_lhs(s.ad_value(1828), 2.0, A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1836)), 1.0));

        s.b[1884] = (p.p11 > 0.0);
        s.v[1884] = if s.b[1884] { 1.0 } else { 0.0 };

        if s.b[1884] {
            s.store_div_ad(1805, A::div(s.ad_value(453), s.ad_value(452)), A::offset(s.ad_value(453), 1.0));
            s.store_div_ad(1806, A::div(s.ad_value(452), s.ad_value(453)), A::offset(s.ad_value(452), 1.0));
            s.store_offset_ln_ad(1807, A::div(A::mul(A::mul(s.ad_value(452), A::offset(s.ad_value(1805), 1.0)), s.ad_value(437)), s.ad_value(438)), 2.0);
            s.store_offset_ln_ad(1808, A::div(A::mul(A::mul(s.ad_value(453), A::offset(s.ad_value(1806), 1.0)), s.ad_value(437)), s.ad_value(438)), 2.0);
            s.store_sub_ad(1809, A::mul(A::offset(s.ad_value(1805), 1.0), s.ad_value(1807)), A::mul(s.ad_value(451), s.ad_value(1805)));
            s.store_sub_ad(1810, A::mul(A::offset(A::div_from_scalar(1.0, s.ad_value(1806)), 1.0), s.ad_value(1808)), A::div(s.ad_value(451), s.ad_value(1806)));
            s.store_add_ad_lhs(1811, A::div(A::sub(A::scale(A::sub(A::add(s.ad_value(1809), s.ad_value(1810)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1809), s.ad_value(1810)), A::sub(s.ad_value(1809), s.ad_value(1810))), 38.0))), 0.5), s.ad_value(450)), s.ad_value(25)), 450);
            s.store_add_ad_lhs(1812, A::mul(s.ad_value(222), A::add(A::sub(A::div(A::sub(s.ad_value(1811), s.ad_value(446)), s.ad_value(447)), s.ad_value(449)), s.ad_value(446))), 130);
            s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));
            s.store_add_ad_lhs(0, A::mul_scaled_lhs(s.ad_value(23), p.p14, A::offset(s.ad_value(8), (-s.v[7]))), 252);
            s.store_sub_ad_lhs(1830, A::offset(A::add(A::scale(A::add(A::add(s.ad_value(181), s.ad_value(1819)), s.ad_value(239)), p.p14), s.ad_value(0)), p.p34), 1822);
            s.store_add_ad_lhs(1831, A::scale(A::add(A::add(s.ad_value(182), s.ad_value(1819)), s.ad_value(240)), p.p14), 0);
            s.store_sub_ad_lhs(1835, A::mul(A::sub(s.ad_value(1812), s.ad_value(1830)), s.ad_value(1821)), 1834);
            s.store_sub_ad_lhs(1836, A::mul(A::sub(A::neg(s.ad_value(333)), s.ad_value(1831)), s.ad_value(1821)), 1834);
        }

        s.b[1885] = (p.p2 > 0.0);
        s.v[1885] = if s.b[1885] { 1.0 } else { 0.0 };

        if (s.b[1884] && s.b[1885]) {
            s.store_div_ad_lhs(0, A::mul_scaled_lhs(s.ad_value(16), p.p14, A::sub(s.ad_value(1835), s.ad_value(1836))), 256);
        }

        s.b[1886] = (s.v[0] < 0.0);
        s.v[1886] = if s.b[1886] { 1.0 } else { 0.0 };

        if ((s.b[1884] && s.b[1885]) && s.b[1886]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if ((s.b[1884] && s.b[1885]) && (!s.b[1886])) {
            s.store_div_ad(2, A::square(s.ad_value(0)), A::offset(A::div(A::scale(s.ad_value(0), 2.0), s.ad_value(256)), 1.0));
        }

        if (s.b[1884] && s.b[1885]) {
            s.store_add_ad_rhs(1837, 1836, A::mul_scaled_lhs(s.ad_value(16), p.p14, s.ad_value(2)));
        }

        if (s.b[1884] && (!s.b[1885])) {
            s.copy_ad(1837, 1836);
        }

        if s.b[1884] {
            s.store_mul_sub_rhs(0, 244, 1835, 1837);
        }

        s.b[1887] = (p.p13 > 0.0);
        s.v[1887] = if s.b[1887] { 1.0 } else { 0.0 };

        if (s.b[1884] && s.b[1887]) {
            s.store_scaled_add_ad(1838, A::add(s.ad_value(0), s.ad_value(253)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253)))), 0.5);
            s.store_scaled_add_ad(1839, A::sub(s.ad_value(253), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(253)), A::sub(A::neg(s.ad_value(0)), s.ad_value(253))), A::square(s.ad_value(253)))), 0.5);
            s.store_mul_ad_rhs(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div_ad(1841, A::mul(s.ad_value(242), s.ad_value(4)), A::offset(A::mul(s.ad_value(242), s.ad_value(2)), 1.0));
            s.store_div_ad(1842, A::mul(s.ad_value(243), s.ad_value(4)), A::offset(A::mul(s.ad_value(243), s.ad_value(3)), 1.0));
            s.store_div_from_scalar_add_ad(1843, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1841)), 1.0), A::div_from_scalar(1.0, s.ad_value(1842)));
        }

        if (s.b[1884] && (!s.b[1887])) {
            s.copy_ad(1841, 242);
            s.copy_ad(1842, 243);
            s.copy_ad(1843, 244);
        }

        if s.b[1884] {
            s.store_mul_sub_rhs(1844, 1843, 1835, 1837);
        }

        s.b[1888] = (s.v[1844] > 0.0);
        s.v[1888] = if s.b[1888] { 1.0 } else { 0.0 };

        s.b[1889] = ((-s.v[1844]) < 80.0);
        s.v[1889] = if s.b[1889] { 1.0 } else { 0.0 };

        if ((s.b[1884] && s.b[1888]) && s.b[1889]) {
            s.store_ln_one_plus_exp_neg_input(0, 1844);
        }

        if ((s.b[1884] && s.b[1888]) && (!s.b[1889])) {
            s.store_neg(0, 1844);
        }

        if (s.b[1884] && s.b[1888]) {
            s.store_offset_add_ad(1845, A::sub(s.ad_value(1835), A::div(s.ad_value(1844), s.ad_value(1841))), s.ad_value(0), (-0.6931471805599));
        }

        s.b[1890] = (s.v[1844] < 80.0);
        s.v[1890] = if s.b[1890] { 1.0 } else { 0.0 };

        if ((s.b[1884] && (!s.b[1888])) && s.b[1890]) {
            s.store_ln_one_plus_exp(0, 1844);
        }

        if ((s.b[1884] && (!s.b[1888])) && (!s.b[1890])) {
            s.copy_ad(0, 1844);
        }

        if (s.b[1884] && (!s.b[1888])) {
            s.store_offset_add_ad(1845, A::add(s.ad_value(1837), A::div(s.ad_value(1844), s.ad_value(1842))), s.ad_value(0), (-0.6931471805599));
        }

        if s.b[1884] {
            s.store_scaled_sub_ad(1846, A::add(s.ad_value(1845), s.ad_value(1824)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1845), s.ad_value(1824)), A::sub(s.ad_value(1845), s.ad_value(1824))), 4.0)), 0.5);
            s.store_offset_sqrt_ad(1847, A::offset(A::div(A::scale(A::sub(s.ad_value(1824), s.ad_value(1846)), 2.0), s.ad_value(1825)), 1.0), (-1.0));
            s.store_scaled_add_ad(0, A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), 1.0), (-0.5))), 0.01)), 0.5);
            s.store_mul_ad(0, A::mul(A::mul_scaled_lhs(s.ad_value(1828), 2.0, A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(1836)), 1.0));
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq0_e500, eq0_e500_d_n0, eq0_e500_d_n1, eq0_e500_d_n2, eq0_e500_d_n3, eq0_e500_d_n4, eq0_e500_d_n5, eq0_e500_d_n6, eq0_e500_d_n7, eq0_e500_d_n8, eq0_e500_d_n9, eq0_e500_d_b0, eq0_e500_d_b1, eq0_e500_d_b2, eq0_e500_d_b3,) = {
    if s.b[1763] {
        let eq0_e498: f64 = (p.p14 * s.v[361]);
        let eq0_e498_d_n0: f64 = (p.p14 * s.dn[361][0]);
        let eq0_e498_d_n1: f64 = (p.p14 * s.dn[361][1]);
        let eq0_e498_d_n2: f64 = (p.p14 * s.dn[361][2]);
        let eq0_e498_d_n3: f64 = (p.p14 * s.dn[361][3]);
        let eq0_e498_d_n4: f64 = (p.p14 * s.dn[361][4]);
        let eq0_e498_d_n5: f64 = (p.p14 * s.dn[361][5]);
        let eq0_e498_d_n6: f64 = (p.p14 * s.dn[361][6]);
        let eq0_e498_d_n7: f64 = (p.p14 * s.dn[361][7]);
        let eq0_e498_d_n8: f64 = (p.p14 * s.dn[361][8]);
        let eq0_e498_d_n9: f64 = (p.p14 * s.dn[361][9]);
        let eq0_e498_d_b0: f64 = (p.p14 * s.db[361][0]);
        let eq0_e498_d_b1: f64 = (p.p14 * s.db[361][1]);
        let eq0_e498_d_b2: f64 = (p.p14 * s.db[361][2]);
        let eq0_e498_d_b3: f64 = (p.p14 * s.db[361][3]);
        (eq0_e498, eq0_e498_d_n0, eq0_e498_d_n1, eq0_e498_d_n2, eq0_e498_d_n3, eq0_e498_d_n4, eq0_e498_d_n5, eq0_e498_d_n6, eq0_e498_d_n7, eq0_e498_d_n8, eq0_e498_d_n9, eq0_e498_d_b0, eq0_e498_d_b1, eq0_e498_d_b2, eq0_e498_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e500;
        let eq0_node_derivatives: [f64; 10] = [eq0_e500_d_n0, eq0_e500_d_n1, eq0_e500_d_n2, eq0_e500_d_n3, eq0_e500_d_n4, eq0_e500_d_n5, eq0_e500_d_n6, eq0_e500_d_n7, eq0_e500_d_n8, eq0_e500_d_n9];
        let eq0_branch_derivatives: [f64; 4] = [eq0_e500_d_b0, eq0_e500_d_b1, eq0_e500_d_b2, eq0_e500_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e507, eq1_e507_d_n0, eq1_e507_d_n1, eq1_e507_d_n2, eq1_e507_d_n3, eq1_e507_d_n4, eq1_e507_d_n5, eq1_e507_d_n6, eq1_e507_d_n7, eq1_e507_d_n8, eq1_e507_d_n9, eq1_e507_d_b0, eq1_e507_d_b1, eq1_e507_d_b2, eq1_e507_d_b3,) = {
    if (!s.b[1763]) {
        let eq1_e505: f64 = (p.p14 * s.v[361]);
        let eq1_e505_d_n0: f64 = (p.p14 * s.dn[361][0]);
        let eq1_e505_d_n1: f64 = (p.p14 * s.dn[361][1]);
        let eq1_e505_d_n2: f64 = (p.p14 * s.dn[361][2]);
        let eq1_e505_d_n3: f64 = (p.p14 * s.dn[361][3]);
        let eq1_e505_d_n4: f64 = (p.p14 * s.dn[361][4]);
        let eq1_e505_d_n5: f64 = (p.p14 * s.dn[361][5]);
        let eq1_e505_d_n6: f64 = (p.p14 * s.dn[361][6]);
        let eq1_e505_d_n7: f64 = (p.p14 * s.dn[361][7]);
        let eq1_e505_d_n8: f64 = (p.p14 * s.dn[361][8]);
        let eq1_e505_d_n9: f64 = (p.p14 * s.dn[361][9]);
        let eq1_e505_d_b0: f64 = (p.p14 * s.db[361][0]);
        let eq1_e505_d_b1: f64 = (p.p14 * s.db[361][1]);
        let eq1_e505_d_b2: f64 = (p.p14 * s.db[361][2]);
        let eq1_e505_d_b3: f64 = (p.p14 * s.db[361][3]);
        (eq1_e505, eq1_e505_d_n0, eq1_e505_d_n1, eq1_e505_d_n2, eq1_e505_d_n3, eq1_e505_d_n4, eq1_e505_d_n5, eq1_e505_d_n6, eq1_e505_d_n7, eq1_e505_d_n8, eq1_e505_d_n9, eq1_e505_d_b0, eq1_e505_d_b1, eq1_e505_d_b2, eq1_e505_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e507;
        let eq1_node_derivatives: [f64; 10] = [eq1_e507_d_n0, eq1_e507_d_n1, eq1_e507_d_n2, eq1_e507_d_n3, eq1_e507_d_n4, eq1_e507_d_n5, eq1_e507_d_n6, eq1_e507_d_n7, eq1_e507_d_n8, eq1_e507_d_n9];
        let eq1_branch_derivatives: [f64; 4] = [eq1_e507_d_b0, eq1_e507_d_b1, eq1_e507_d_b2, eq1_e507_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e511: f64 = (s.v[364] - s.v[365]);
        let eq2_e511_d_n0: f64 = (s.dn[364][0] - s.dn[365][0]);
        let eq2_e511_d_n1: f64 = (s.dn[364][1] - s.dn[365][1]);
        let eq2_e511_d_n2: f64 = (s.dn[364][2] - s.dn[365][2]);
        let eq2_e511_d_n3: f64 = (s.dn[364][3] - s.dn[365][3]);
        let eq2_e511_d_n4: f64 = (s.dn[364][4] - s.dn[365][4]);
        let eq2_e511_d_n5: f64 = (s.dn[364][5] - s.dn[365][5]);
        let eq2_e511_d_n6: f64 = (s.dn[364][6] - s.dn[365][6]);
        let eq2_e511_d_n7: f64 = (s.dn[364][7] - s.dn[365][7]);
        let eq2_e511_d_n8: f64 = (s.dn[364][8] - s.dn[365][8]);
        let eq2_e511_d_n9: f64 = (s.dn[364][9] - s.dn[365][9]);
        let eq2_e511_d_b0: f64 = (s.db[364][0] - s.db[365][0]);
        let eq2_e511_d_b1: f64 = (s.db[364][1] - s.db[365][1]);
        let eq2_e511_d_b2: f64 = (s.db[364][2] - s.db[365][2]);
        let eq2_e511_d_b3: f64 = (s.db[364][3] - s.db[365][3]);
        let eq2_e512: f64 = (p.p14 * eq2_e511);
        let eq2_e512_d_n0: f64 = (p.p14 * eq2_e511_d_n0);
        let eq2_e512_d_n1: f64 = (p.p14 * eq2_e511_d_n1);
        let eq2_e512_d_n2: f64 = (p.p14 * eq2_e511_d_n2);
        let eq2_e512_d_n3: f64 = (p.p14 * eq2_e511_d_n3);
        let eq2_e512_d_n4: f64 = (p.p14 * eq2_e511_d_n4);
        let eq2_e512_d_n5: f64 = (p.p14 * eq2_e511_d_n5);
        let eq2_e512_d_n6: f64 = (p.p14 * eq2_e511_d_n6);
        let eq2_e512_d_n7: f64 = (p.p14 * eq2_e511_d_n7);
        let eq2_e512_d_n8: f64 = (p.p14 * eq2_e511_d_n8);
        let eq2_e512_d_n9: f64 = (p.p14 * eq2_e511_d_n9);
        let eq2_e512_d_b0: f64 = (p.p14 * eq2_e511_d_b0);
        let eq2_e512_d_b1: f64 = (p.p14 * eq2_e511_d_b1);
        let eq2_e512_d_b2: f64 = (p.p14 * eq2_e511_d_b2);
        let eq2_e512_d_b3: f64 = (p.p14 * eq2_e511_d_b3);
        let eq2_value: f64 = eq2_e512;
        let eq2_node_derivatives: [f64; 10] = [eq2_e512_d_n0, eq2_e512_d_n1, eq2_e512_d_n2, eq2_e512_d_n3, eq2_e512_d_n4, eq2_e512_d_n5, eq2_e512_d_n6, eq2_e512_d_n7, eq2_e512_d_n8, eq2_e512_d_n9];
        let eq2_branch_derivatives: [f64; 4] = [eq2_e512_d_b0, eq2_e512_d_b1, eq2_e512_d_b2, eq2_e512_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e515: f64 = (p.p14 * s.v[362]);
        let eq3_e515_d_n0: f64 = (p.p14 * s.dn[362][0]);
        let eq3_e515_d_n1: f64 = (p.p14 * s.dn[362][1]);
        let eq3_e515_d_n2: f64 = (p.p14 * s.dn[362][2]);
        let eq3_e515_d_n3: f64 = (p.p14 * s.dn[362][3]);
        let eq3_e515_d_n4: f64 = (p.p14 * s.dn[362][4]);
        let eq3_e515_d_n5: f64 = (p.p14 * s.dn[362][5]);
        let eq3_e515_d_n6: f64 = (p.p14 * s.dn[362][6]);
        let eq3_e515_d_n7: f64 = (p.p14 * s.dn[362][7]);
        let eq3_e515_d_n8: f64 = (p.p14 * s.dn[362][8]);
        let eq3_e515_d_n9: f64 = (p.p14 * s.dn[362][9]);
        let eq3_e515_d_b0: f64 = (p.p14 * s.db[362][0]);
        let eq3_e515_d_b1: f64 = (p.p14 * s.db[362][1]);
        let eq3_e515_d_b2: f64 = (p.p14 * s.db[362][2]);
        let eq3_e515_d_b3: f64 = (p.p14 * s.db[362][3]);
        let eq3_value: f64 = eq3_e515;
        let eq3_node_derivatives: [f64; 10] = [eq3_e515_d_n0, eq3_e515_d_n1, eq3_e515_d_n2, eq3_e515_d_n3, eq3_e515_d_n4, eq3_e515_d_n5, eq3_e515_d_n6, eq3_e515_d_n7, eq3_e515_d_n8, eq3_e515_d_n9];
        let eq3_branch_derivatives: [f64; 4] = [eq3_e515_d_b0, eq3_e515_d_b1, eq3_e515_d_b2, eq3_e515_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_e518: f64 = (p.p14 * s.v[363]);
        let eq4_e518_d_n0: f64 = (p.p14 * s.dn[363][0]);
        let eq4_e518_d_n1: f64 = (p.p14 * s.dn[363][1]);
        let eq4_e518_d_n2: f64 = (p.p14 * s.dn[363][2]);
        let eq4_e518_d_n3: f64 = (p.p14 * s.dn[363][3]);
        let eq4_e518_d_n4: f64 = (p.p14 * s.dn[363][4]);
        let eq4_e518_d_n5: f64 = (p.p14 * s.dn[363][5]);
        let eq4_e518_d_n6: f64 = (p.p14 * s.dn[363][6]);
        let eq4_e518_d_n7: f64 = (p.p14 * s.dn[363][7]);
        let eq4_e518_d_n8: f64 = (p.p14 * s.dn[363][8]);
        let eq4_e518_d_n9: f64 = (p.p14 * s.dn[363][9]);
        let eq4_e518_d_b0: f64 = (p.p14 * s.db[363][0]);
        let eq4_e518_d_b1: f64 = (p.p14 * s.db[363][1]);
        let eq4_e518_d_b2: f64 = (p.p14 * s.db[363][2]);
        let eq4_e518_d_b3: f64 = (p.p14 * s.db[363][3]);
        let eq4_value: f64 = eq4_e518;
        let eq4_node_derivatives: [f64; 10] = [eq4_e518_d_n0, eq4_e518_d_n1, eq4_e518_d_n2, eq4_e518_d_n3, eq4_e518_d_n4, eq4_e518_d_n5, eq4_e518_d_n6, eq4_e518_d_n7, eq4_e518_d_n8, eq4_e518_d_n9];
        let eq4_branch_derivatives: [f64; 4] = [eq4_e518_d_b0, eq4_e518_d_b1, eq4_e518_d_b2, eq4_e518_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(8),
            multiplicity * (eq5_value),
        );
        let eq6_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (eq6_value),
        );
        let eq7_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(9),
            Some(8),
            multiplicity * (eq7_value),
        );
        let eq8_e524: f64 = (p.p31 * s.v[471]);
        let eq8_e526: f64 = (eq8_e524 * (nv7 - nv6));
        let eq8_e526_d_n6: f64 = (-eq8_e524);
        let eq8_value: f64 = eq8_e526;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(6),
            multiplicity * (eq8_value),
            6,
            multiplicity * (eq8_e526_d_n6),
            7,
            multiplicity * (eq8_e524),
        );
        let eq9_value: f64 = s.v[1761];
        let eq9_node_derivatives: [f64; 10] = [s.dn[1761][0], s.dn[1761][1], s.dn[1761][2], s.dn[1761][3], s.dn[1761][4], s.dn[1761][5], s.dn[1761][6], s.dn[1761][7], s.dn[1761][8], s.dn[1761][9]];
        let eq9_branch_derivatives: [f64; 4] = [s.db[1761][0], s.db[1761][1], s.db[1761][2], s.db[1761][3]];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_value: f64 = s.v[1762];
        let eq10_node_derivatives: [f64; 10] = [s.dn[1762][0], s.dn[1762][1], s.dn[1762][2], s.dn[1762][3], s.dn[1762][4], s.dn[1762][5], s.dn[1762][6], s.dn[1762][7], s.dn[1762][8], s.dn[1762][9]];
        let eq10_branch_derivatives: [f64; 4] = [s.db[1762][0], s.db[1762][1], s.db[1762][2], s.db[1762][3]];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e538, eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9, eq11_e538_d_b0, eq11_e538_d_b1, eq11_e538_d_b2, eq11_e538_d_b3,) = {
    if s.b[1764] {
        let eq11_e532: f64 = (p.p31 * s.v[13]);
        let eq11_e532_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq11_e532_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq11_e532_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq11_e532_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq11_e532_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq11_e532_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq11_e532_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq11_e532_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq11_e532_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq11_e532_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq11_e532_d_b0: f64 = (p.p31 * s.db[13][0]);
        let eq11_e532_d_b1: f64 = (p.p31 * s.db[13][1]);
        let eq11_e532_d_b2: f64 = (p.p31 * s.db[13][2]);
        let eq11_e532_d_b3: f64 = (p.p31 * s.db[13][3]);
        let eq11_e534: f64 = (eq11_e532 * s.v[312]);
        let eq11_e534_d_n0: f64 = ((eq11_e532_d_n0 * s.v[312]) + (eq11_e532 * s.dn[312][0]));
        let eq11_e534_d_n1: f64 = ((eq11_e532_d_n1 * s.v[312]) + (eq11_e532 * s.dn[312][1]));
        let eq11_e534_d_n2: f64 = ((eq11_e532_d_n2 * s.v[312]) + (eq11_e532 * s.dn[312][2]));
        let eq11_e534_d_n3: f64 = ((eq11_e532_d_n3 * s.v[312]) + (eq11_e532 * s.dn[312][3]));
        let eq11_e534_d_n4: f64 = ((eq11_e532_d_n4 * s.v[312]) + (eq11_e532 * s.dn[312][4]));
        let eq11_e534_d_n5: f64 = ((eq11_e532_d_n5 * s.v[312]) + (eq11_e532 * s.dn[312][5]));
        let eq11_e534_d_n6: f64 = ((eq11_e532_d_n6 * s.v[312]) + (eq11_e532 * s.dn[312][6]));
        let eq11_e534_d_n7: f64 = ((eq11_e532_d_n7 * s.v[312]) + (eq11_e532 * s.dn[312][7]));
        let eq11_e534_d_n8: f64 = ((eq11_e532_d_n8 * s.v[312]) + (eq11_e532 * s.dn[312][8]));
        let eq11_e534_d_n9: f64 = ((eq11_e532_d_n9 * s.v[312]) + (eq11_e532 * s.dn[312][9]));
        let eq11_e534_d_b0: f64 = ((eq11_e532_d_b0 * s.v[312]) + (eq11_e532 * s.db[312][0]));
        let eq11_e534_d_b1: f64 = ((eq11_e532_d_b1 * s.v[312]) + (eq11_e532 * s.db[312][1]));
        let eq11_e534_d_b2: f64 = ((eq11_e532_d_b2 * s.v[312]) + (eq11_e532 * s.db[312][2]));
        let eq11_e534_d_b3: f64 = ((eq11_e532_d_b3 * s.v[312]) + (eq11_e532 * s.db[312][3]));
        let eq11_e536: f64 = (eq11_e534 * (nv1 - nv9));
        let eq11_e536_d_n0: f64 = (eq11_e534_d_n0 * (nv1 - nv9));
        let eq11_e536_d_n1: f64 = ((eq11_e534_d_n1 * (nv1 - nv9)) + eq11_e534);
        let eq11_e536_d_n2: f64 = (eq11_e534_d_n2 * (nv1 - nv9));
        let eq11_e536_d_n3: f64 = (eq11_e534_d_n3 * (nv1 - nv9));
        let eq11_e536_d_n4: f64 = (eq11_e534_d_n4 * (nv1 - nv9));
        let eq11_e536_d_n5: f64 = (eq11_e534_d_n5 * (nv1 - nv9));
        let eq11_e536_d_n6: f64 = (eq11_e534_d_n6 * (nv1 - nv9));
        let eq11_e536_d_n7: f64 = (eq11_e534_d_n7 * (nv1 - nv9));
        let eq11_e536_d_n8: f64 = (eq11_e534_d_n8 * (nv1 - nv9));
        let eq11_e536_d_n9: f64 = ((eq11_e534_d_n9 * (nv1 - nv9)) + (-eq11_e534));
        let eq11_e536_d_b0: f64 = (eq11_e534_d_b0 * (nv1 - nv9));
        let eq11_e536_d_b1: f64 = (eq11_e534_d_b1 * (nv1 - nv9));
        let eq11_e536_d_b2: f64 = (eq11_e534_d_b2 * (nv1 - nv9));
        let eq11_e536_d_b3: f64 = (eq11_e534_d_b3 * (nv1 - nv9));
        (eq11_e536, eq11_e536_d_n0, eq11_e536_d_n1, eq11_e536_d_n2, eq11_e536_d_n3, eq11_e536_d_n4, eq11_e536_d_n5, eq11_e536_d_n6, eq11_e536_d_n7, eq11_e536_d_n8, eq11_e536_d_n9, eq11_e536_d_b0, eq11_e536_d_b1, eq11_e536_d_b2, eq11_e536_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e538;
        let eq11_node_derivatives: [f64; 10] = [eq11_e538_d_n0, eq11_e538_d_n1, eq11_e538_d_n2, eq11_e538_d_n3, eq11_e538_d_n4, eq11_e538_d_n5, eq11_e538_d_n6, eq11_e538_d_n7, eq11_e538_d_n8, eq11_e538_d_n9];
        let eq11_branch_derivatives: [f64; 4] = [eq11_e538_d_b0, eq11_e538_d_b1, eq11_e538_d_b2, eq11_e538_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e548,) = {
    if s.b[1764] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e548;
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (eq12_value),
        );
        let (eq13_e553,) = {
    if (!s.b[1764]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e553;
        stamper.stamp_potential_const_local(
            0,
            eq13_value,
        );
        let (eq14_e563, eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9, eq14_e563_d_b0, eq14_e563_d_b1, eq14_e563_d_b2, eq14_e563_d_b3,) = {
    if s.b[1765] {
        let eq14_e557: f64 = (p.p31 * s.v[13]);
        let eq14_e557_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq14_e557_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq14_e557_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq14_e557_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq14_e557_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq14_e557_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq14_e557_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq14_e557_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq14_e557_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq14_e557_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq14_e557_d_b0: f64 = (p.p31 * s.db[13][0]);
        let eq14_e557_d_b1: f64 = (p.p31 * s.db[13][1]);
        let eq14_e557_d_b2: f64 = (p.p31 * s.db[13][2]);
        let eq14_e557_d_b3: f64 = (p.p31 * s.db[13][3]);
        let eq14_e559: f64 = (eq14_e557 * s.v[316]);
        let eq14_e559_d_n0: f64 = ((eq14_e557_d_n0 * s.v[316]) + (eq14_e557 * s.dn[316][0]));
        let eq14_e559_d_n1: f64 = ((eq14_e557_d_n1 * s.v[316]) + (eq14_e557 * s.dn[316][1]));
        let eq14_e559_d_n2: f64 = ((eq14_e557_d_n2 * s.v[316]) + (eq14_e557 * s.dn[316][2]));
        let eq14_e559_d_n3: f64 = ((eq14_e557_d_n3 * s.v[316]) + (eq14_e557 * s.dn[316][3]));
        let eq14_e559_d_n4: f64 = ((eq14_e557_d_n4 * s.v[316]) + (eq14_e557 * s.dn[316][4]));
        let eq14_e559_d_n5: f64 = ((eq14_e557_d_n5 * s.v[316]) + (eq14_e557 * s.dn[316][5]));
        let eq14_e559_d_n6: f64 = ((eq14_e557_d_n6 * s.v[316]) + (eq14_e557 * s.dn[316][6]));
        let eq14_e559_d_n7: f64 = ((eq14_e557_d_n7 * s.v[316]) + (eq14_e557 * s.dn[316][7]));
        let eq14_e559_d_n8: f64 = ((eq14_e557_d_n8 * s.v[316]) + (eq14_e557 * s.dn[316][8]));
        let eq14_e559_d_n9: f64 = ((eq14_e557_d_n9 * s.v[316]) + (eq14_e557 * s.dn[316][9]));
        let eq14_e559_d_b0: f64 = ((eq14_e557_d_b0 * s.v[316]) + (eq14_e557 * s.db[316][0]));
        let eq14_e559_d_b1: f64 = ((eq14_e557_d_b1 * s.v[316]) + (eq14_e557 * s.db[316][1]));
        let eq14_e559_d_b2: f64 = ((eq14_e557_d_b2 * s.v[316]) + (eq14_e557 * s.db[316][2]));
        let eq14_e559_d_b3: f64 = ((eq14_e557_d_b3 * s.v[316]) + (eq14_e557 * s.db[316][3]));
        let eq14_e561: f64 = (eq14_e559 * (nv2 - nv6));
        let eq14_e561_d_n0: f64 = (eq14_e559_d_n0 * (nv2 - nv6));
        let eq14_e561_d_n1: f64 = (eq14_e559_d_n1 * (nv2 - nv6));
        let eq14_e561_d_n2: f64 = ((eq14_e559_d_n2 * (nv2 - nv6)) + eq14_e559);
        let eq14_e561_d_n3: f64 = (eq14_e559_d_n3 * (nv2 - nv6));
        let eq14_e561_d_n4: f64 = (eq14_e559_d_n4 * (nv2 - nv6));
        let eq14_e561_d_n5: f64 = (eq14_e559_d_n5 * (nv2 - nv6));
        let eq14_e561_d_n6: f64 = ((eq14_e559_d_n6 * (nv2 - nv6)) + (-eq14_e559));
        let eq14_e561_d_n7: f64 = (eq14_e559_d_n7 * (nv2 - nv6));
        let eq14_e561_d_n8: f64 = (eq14_e559_d_n8 * (nv2 - nv6));
        let eq14_e561_d_n9: f64 = (eq14_e559_d_n9 * (nv2 - nv6));
        let eq14_e561_d_b0: f64 = (eq14_e559_d_b0 * (nv2 - nv6));
        let eq14_e561_d_b1: f64 = (eq14_e559_d_b1 * (nv2 - nv6));
        let eq14_e561_d_b2: f64 = (eq14_e559_d_b2 * (nv2 - nv6));
        let eq14_e561_d_b3: f64 = (eq14_e559_d_b3 * (nv2 - nv6));
        (eq14_e561, eq14_e561_d_n0, eq14_e561_d_n1, eq14_e561_d_n2, eq14_e561_d_n3, eq14_e561_d_n4, eq14_e561_d_n5, eq14_e561_d_n6, eq14_e561_d_n7, eq14_e561_d_n8, eq14_e561_d_n9, eq14_e561_d_b0, eq14_e561_d_b1, eq14_e561_d_b2, eq14_e561_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e563;
        let eq14_node_derivatives: [f64; 10] = [eq14_e563_d_n0, eq14_e563_d_n1, eq14_e563_d_n2, eq14_e563_d_n3, eq14_e563_d_n4, eq14_e563_d_n5, eq14_e563_d_n6, eq14_e563_d_n7, eq14_e563_d_n8, eq14_e563_d_n9];
        let eq14_branch_derivatives: [f64; 4] = [eq14_e563_d_b0, eq14_e563_d_b1, eq14_e563_d_b2, eq14_e563_d_b3];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e573,) = {
    if s.b[1765] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e573;
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (eq15_value),
        );
        let (eq16_e578,) = {
    if (!s.b[1765]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e578;
        stamper.stamp_potential_const_local(
            1,
            eq16_value,
        );
        let (eq17_e588, eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9, eq17_e588_d_b0, eq17_e588_d_b1, eq17_e588_d_b2, eq17_e588_d_b3,) = {
    if s.b[1766] {
        let eq17_e582: f64 = (p.p31 * s.v[13]);
        let eq17_e582_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq17_e582_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq17_e582_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq17_e582_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq17_e582_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq17_e582_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq17_e582_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq17_e582_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq17_e582_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq17_e582_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq17_e582_d_b0: f64 = (p.p31 * s.db[13][0]);
        let eq17_e582_d_b1: f64 = (p.p31 * s.db[13][1]);
        let eq17_e582_d_b2: f64 = (p.p31 * s.db[13][2]);
        let eq17_e582_d_b3: f64 = (p.p31 * s.db[13][3]);
        let eq17_e584: f64 = (eq17_e582 * s.v[320]);
        let eq17_e584_d_n0: f64 = ((eq17_e582_d_n0 * s.v[320]) + (eq17_e582 * s.dn[320][0]));
        let eq17_e584_d_n1: f64 = ((eq17_e582_d_n1 * s.v[320]) + (eq17_e582 * s.dn[320][1]));
        let eq17_e584_d_n2: f64 = ((eq17_e582_d_n2 * s.v[320]) + (eq17_e582 * s.dn[320][2]));
        let eq17_e584_d_n3: f64 = ((eq17_e582_d_n3 * s.v[320]) + (eq17_e582 * s.dn[320][3]));
        let eq17_e584_d_n4: f64 = ((eq17_e582_d_n4 * s.v[320]) + (eq17_e582 * s.dn[320][4]));
        let eq17_e584_d_n5: f64 = ((eq17_e582_d_n5 * s.v[320]) + (eq17_e582 * s.dn[320][5]));
        let eq17_e584_d_n6: f64 = ((eq17_e582_d_n6 * s.v[320]) + (eq17_e582 * s.dn[320][6]));
        let eq17_e584_d_n7: f64 = ((eq17_e582_d_n7 * s.v[320]) + (eq17_e582 * s.dn[320][7]));
        let eq17_e584_d_n8: f64 = ((eq17_e582_d_n8 * s.v[320]) + (eq17_e582 * s.dn[320][8]));
        let eq17_e584_d_n9: f64 = ((eq17_e582_d_n9 * s.v[320]) + (eq17_e582 * s.dn[320][9]));
        let eq17_e584_d_b0: f64 = ((eq17_e582_d_b0 * s.v[320]) + (eq17_e582 * s.db[320][0]));
        let eq17_e584_d_b1: f64 = ((eq17_e582_d_b1 * s.v[320]) + (eq17_e582 * s.db[320][1]));
        let eq17_e584_d_b2: f64 = ((eq17_e582_d_b2 * s.v[320]) + (eq17_e582 * s.db[320][2]));
        let eq17_e584_d_b3: f64 = ((eq17_e582_d_b3 * s.v[320]) + (eq17_e582 * s.db[320][3]));
        let eq17_e586: f64 = (eq17_e584 * (nv0 - nv7));
        let eq17_e586_d_n0: f64 = ((eq17_e584_d_n0 * (nv0 - nv7)) + eq17_e584);
        let eq17_e586_d_n1: f64 = (eq17_e584_d_n1 * (nv0 - nv7));
        let eq17_e586_d_n2: f64 = (eq17_e584_d_n2 * (nv0 - nv7));
        let eq17_e586_d_n3: f64 = (eq17_e584_d_n3 * (nv0 - nv7));
        let eq17_e586_d_n4: f64 = (eq17_e584_d_n4 * (nv0 - nv7));
        let eq17_e586_d_n5: f64 = (eq17_e584_d_n5 * (nv0 - nv7));
        let eq17_e586_d_n6: f64 = (eq17_e584_d_n6 * (nv0 - nv7));
        let eq17_e586_d_n7: f64 = ((eq17_e584_d_n7 * (nv0 - nv7)) + (-eq17_e584));
        let eq17_e586_d_n8: f64 = (eq17_e584_d_n8 * (nv0 - nv7));
        let eq17_e586_d_n9: f64 = (eq17_e584_d_n9 * (nv0 - nv7));
        let eq17_e586_d_b0: f64 = (eq17_e584_d_b0 * (nv0 - nv7));
        let eq17_e586_d_b1: f64 = (eq17_e584_d_b1 * (nv0 - nv7));
        let eq17_e586_d_b2: f64 = (eq17_e584_d_b2 * (nv0 - nv7));
        let eq17_e586_d_b3: f64 = (eq17_e584_d_b3 * (nv0 - nv7));
        (eq17_e586, eq17_e586_d_n0, eq17_e586_d_n1, eq17_e586_d_n2, eq17_e586_d_n3, eq17_e586_d_n4, eq17_e586_d_n5, eq17_e586_d_n6, eq17_e586_d_n7, eq17_e586_d_n8, eq17_e586_d_n9, eq17_e586_d_b0, eq17_e586_d_b1, eq17_e586_d_b2, eq17_e586_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e588;
        let eq17_node_derivatives: [f64; 10] = [eq17_e588_d_n0, eq17_e588_d_n1, eq17_e588_d_n2, eq17_e588_d_n3, eq17_e588_d_n4, eq17_e588_d_n5, eq17_e588_d_n6, eq17_e588_d_n7, eq17_e588_d_n8, eq17_e588_d_n9];
        let eq17_branch_derivatives: [f64; 4] = [eq17_e588_d_b0, eq17_e588_d_b1, eq17_e588_d_b2, eq17_e588_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e598,) = {
    if s.b[1766] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e598;
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (eq18_value),
        );
        let (eq19_e603,) = {
    if (!s.b[1766]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e603;
        stamper.stamp_potential_const_local(
            2,
            eq19_value,
        );
    }
}
