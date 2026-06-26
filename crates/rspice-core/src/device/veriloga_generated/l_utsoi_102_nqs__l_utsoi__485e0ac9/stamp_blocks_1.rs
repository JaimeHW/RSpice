#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {
            s.store_ad_value(690, A::add_scaled_square_product(s.ad_value(689), 1.0, s.ad_value(688), A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0));
            s.store_add_ad_rhs(691, 690, A::mul3(A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), s.ad_value(687), A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0)));
            s.store_add_ad_rhs(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));
        }

        s.b[1216] = (((s.v[692]) as f64).abs() < 80.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((s.b[1213] && (!s.b[1214])) && s.b[1215]) && s.b[1216]) {
            s.store_exp(693, 692);
        }

        s.b[1217] = (s.v[692] < (-80.0));
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((((s.b[1213] && (!s.b[1214])) && s.b[1215]) && (!s.b[1216])) && s.b[1217]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(692)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(692)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1213] && (!s.b[1214])) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(s.ad_value(692), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(692), (-80.0)), 0.5, A::scale_offset(s.ad_value(692), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {
            s.store_sub(691, 683, 692);
            s.store_ad_value(694, A::add_scaled_offset_product_rhs(s.ad_value(691), 2.0, s.ad_value(709), s.ad_value(693), (-1.0), 1.0));
            s.store_ad_value(695, A::add_scaled_square_product(s.ad_value(691), 1.0, s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(697, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_neg_ad(714, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if ((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) {
            s.store_mul_offset_ad_lhs(698, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), (-1.0), 713);
            s.store_mul_ad_product_rhs(699, 704, s.ad_value(712), A::offset(A::mul(s.ad_value(698), s.ad_value(704)), 1.0));
        }

        s.b[1218] = ((((-s.v[699])) as f64).abs() < 80.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if (((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && s.b[1218]) {
            s.store_exp_neg_input(691, 699);
        }

        s.b[1219] = ((-s.v[699]) < (-80.0));
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1218])) && s.b[1219]) {
            s.store_div_from_scalar_offset_ad(691, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(699))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(699))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1218])) && (!s.b[1219])) {
            s.store_scaled_offset_ad(691, A::mul_offset_lhs(A::neg(s.ad_value(699)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(699)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) {
            s.store_sub_from_scalar(697, 1.0, 691);
            s.store_ad_value(700, A::add_scaled_inputs_product(s.ad_value(704), 1.0, s.ad_value(709), 0.5, s.ad_value(708), A::sqrt(A::add_scaled_inputs3(s.ad_value(704), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0)));
        }

        s.b[1220] = ((((-s.v[700])) as f64).abs() < 80.0);
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if (((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && s.b[1220]) {
            s.store_exp_neg_input(693, 700);
        }

        s.b[1221] = ((-s.v[700]) < (-80.0));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1220])) && s.b[1221]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(700))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(700))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1220])) && (!s.b[1221])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(A::neg(s.ad_value(700)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(700)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1213] && (!s.b[1214])) && (!s.b[1215])) {
            s.store_ad_value(694, A::add_scaled_inputs3(s.ad_value(704), 2.0, s.ad_value(700), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0));
            s.store_ad_value(695, A::add_scaled_products(A::sub(s.ad_value(704), s.ad_value(700)), A::sub(s.ad_value(704), s.ad_value(700)), 1.0, s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0)));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(701, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_add(714, 700, 701);
        }

        if (s.b[1213] && (!s.b[1214])) {
            s.store_neg(714, 714);
        }

        s.b[1222] = (s.v[159] > 0.0);
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        s.b[1223] = (((s.v[706]) as f64).abs() <= s.v[711]);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1223]) {
            s.store_mul_neg_lhs(716, 706, 712);
        }

        s.b[1224] = (s.v[706] < (-s.v[711]));
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {
            s.store_neg(683, 706);
            s.store_scaled_mul(684, 683, 712, 1.25);
            s.store_scaled_sub_ad(685, A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(684), (-6.0), A::offset(s.ad_value(684), (-6.0))), 64.0)), 0.5);
            s.store_ad_value(686, A::add_scaled_products(A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685)), 1.0, s.ad_value(709), A::offset(s.ad_value(685), 1.0), 1.0));
            s.store_ad_value(687, A::add_scaled_inputs3(s.ad_value(683), 2.0, s.ad_value(685), (-2.0), s.ad_value(709), -1.0));
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
            s.store_add(689, 686, 687);
            s.store_ad_value(690, A::add_scaled_square_product(s.ad_value(689), 1.0, s.ad_value(688), A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0));
            s.store_add_ad_rhs(691, 690, A::mul3(A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), s.ad_value(687), A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0)));
            s.store_add_ad_rhs(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));
        }

        s.b[1225] = (((s.v[692]) as f64).abs() < 80.0);
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (((s.b[1222] && (!s.b[1223])) && s.b[1224]) && s.b[1225]) {
            s.store_exp(693, 692);
        }

        s.b[1226] = (s.v[692] < (-80.0));
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && (!s.b[1223])) && s.b[1224]) && (!s.b[1225])) && s.b[1226]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(692)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(692)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1222] && (!s.b[1223])) && s.b[1224]) && (!s.b[1225])) && (!s.b[1226])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(s.ad_value(692), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(692), (-80.0)), 0.5, A::scale_offset(s.ad_value(692), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1222] && (!s.b[1223])) && s.b[1224]) {
            s.store_sub(691, 683, 692);
            s.store_ad_value(694, A::add_scaled_offset_product_rhs(s.ad_value(691), 2.0, s.ad_value(709), s.ad_value(693), (-1.0), 1.0));
            s.store_ad_value(695, A::add_scaled_square_product(s.ad_value(691), 1.0, s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(697, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_neg_ad(716, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if ((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) {
            s.store_mul_offset_ad_lhs(698, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), (-1.0), 713);
            s.store_mul_ad_product_rhs(699, 706, s.ad_value(712), A::offset(A::mul(s.ad_value(698), s.ad_value(706)), 1.0));
        }

        s.b[1227] = ((((-s.v[699])) as f64).abs() < 80.0);
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        if (((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && s.b[1227]) {
            s.store_exp_neg_input(691, 699);
        }

        s.b[1228] = ((-s.v[699]) < (-80.0));
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1227])) && s.b[1228]) {
            s.store_div_from_scalar_offset_ad(691, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(699))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(699))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1227])) && (!s.b[1228])) {
            s.store_scaled_offset_ad(691, A::mul_offset_lhs(A::neg(s.ad_value(699)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(699)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) {
            s.store_sub_from_scalar(697, 1.0, 691);
            s.store_ad_value(700, A::add_scaled_inputs_product(s.ad_value(706), 1.0, s.ad_value(709), 0.5, s.ad_value(708), A::sqrt(A::add_scaled_inputs3(s.ad_value(706), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0)));
        }

        s.b[1229] = ((((-s.v[700])) as f64).abs() < 80.0);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if (((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && s.b[1229]) {
            s.store_exp_neg_input(693, 700);
        }

        s.b[1230] = ((-s.v[700]) < (-80.0));
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1229])) && s.b[1230]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(700))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(700))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) && (!s.b[1229])) && (!s.b[1230])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(A::neg(s.ad_value(700)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(700)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1222] && (!s.b[1223])) && (!s.b[1224])) {
            s.store_ad_value(694, A::add_scaled_inputs3(s.ad_value(706), 2.0, s.ad_value(700), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0));
            s.store_ad_value(695, A::add_scaled_products(A::sub(s.ad_value(706), s.ad_value(700)), A::sub(s.ad_value(706), s.ad_value(700)), 1.0, s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0)));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(701, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_add(716, 700, 701);
        }

        if (s.b[1222] && (!s.b[1223])) {
            s.store_neg(716, 716);
        }

        s.store_div_ad_lhs(708, A::sqrt(A::mul3_scaled_output(s.ad_value(20), s.ad_value(229), s.ad_value(224), (2.0 * 1.602176565e-19))), 241);

        s.store_square(709, 708);

        s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);

        s.store_scale(711, 710, 1e-5);

        s.store_div_from_scalar(712, 1.0, 710);

        s.store_div_from_scalar_offset_scaled_input(713, 1.0, 708, 0.7324648775608221, 1.25);

        s.b[1231] = (((p.p3 > 0.0) && ((s.v[70] > 0.0) || (s.v[72] > 0.0))) || ((p.p4 > 0.0) && (s.v[90] > 0.0)));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        s.b[1232] = (((s.v[705]) as f64).abs() <= s.v[711]);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if (s.b[1231] && s.b[1232]) {
            s.store_mul_neg_lhs(715, 705, 712);
        }

        s.b[1233] = (s.v[705] < (-s.v[711]));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if ((s.b[1231] && (!s.b[1232])) && s.b[1233]) {
            s.store_neg(683, 705);
            s.store_scaled_mul(684, 683, 712, 1.25);
            s.store_scaled_sub_ad(685, A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(684), (-6.0), A::offset(s.ad_value(684), (-6.0))), 64.0)), 0.5);
            s.store_ad_value(686, A::add_scaled_products(A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685)), 1.0, s.ad_value(709), A::offset(s.ad_value(685), 1.0), 1.0));
            s.store_ad_value(687, A::add_scaled_inputs3(s.ad_value(683), 2.0, s.ad_value(685), (-2.0), s.ad_value(709), -1.0));
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
            s.store_add(689, 686, 687);
            s.store_ad_value(690, A::add_scaled_square_product(s.ad_value(689), 1.0, s.ad_value(688), A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0));
            s.store_add_ad_rhs(691, 690, A::mul3(A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), s.ad_value(687), A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0)));
            s.store_add_ad_rhs(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));
        }

        s.b[1234] = (((s.v[692]) as f64).abs() < 80.0);
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if (((s.b[1231] && (!s.b[1232])) && s.b[1233]) && s.b[1234]) {
            s.store_exp(693, 692);
        }

        s.b[1235] = (s.v[692] < (-80.0));
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if ((((s.b[1231] && (!s.b[1232])) && s.b[1233]) && (!s.b[1234])) && s.b[1235]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(692)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(692)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1231] && (!s.b[1232])) && s.b[1233]) && (!s.b[1234])) && (!s.b[1235])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(s.ad_value(692), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(692), (-80.0)), 0.5, A::scale_offset(s.ad_value(692), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1231] && (!s.b[1232])) && s.b[1233]) {
            s.store_sub(691, 683, 692);
            s.store_ad_value(694, A::add_scaled_offset_product_rhs(s.ad_value(691), 2.0, s.ad_value(709), s.ad_value(693), (-1.0), 1.0));
            s.store_ad_value(695, A::add_scaled_square_product(s.ad_value(691), 1.0, s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(697, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_neg_ad(715, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if ((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) {
            s.store_mul_offset_ad_lhs(698, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), (-1.0), 713);
            s.store_mul_ad_product_rhs(699, 705, s.ad_value(712), A::offset(A::mul(s.ad_value(698), s.ad_value(705)), 1.0));
        }

        s.b[1236] = ((((-s.v[699])) as f64).abs() < 80.0);
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if (((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && s.b[1236]) {
            s.store_exp_neg_input(691, 699);
        }

        s.b[1237] = ((-s.v[699]) < (-80.0));
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1236])) && s.b[1237]) {
            s.store_div_from_scalar_offset_ad(691, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(699))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(699))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1236])) && (!s.b[1237])) {
            s.store_scaled_offset_ad(691, A::mul_offset_lhs(A::neg(s.ad_value(699)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(699)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) {
            s.store_sub_from_scalar(697, 1.0, 691);
            s.store_ad_value(700, A::add_scaled_inputs_product(s.ad_value(705), 1.0, s.ad_value(709), 0.5, s.ad_value(708), A::sqrt(A::add_scaled_inputs3(s.ad_value(705), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0)));
        }

        s.b[1238] = ((((-s.v[700])) as f64).abs() < 80.0);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if (((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && s.b[1238]) {
            s.store_exp_neg_input(693, 700);
        }

        s.b[1239] = ((-s.v[700]) < (-80.0));
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1238])) && s.b[1239]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(700))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(700))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) && (!s.b[1238])) && (!s.b[1239])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(A::neg(s.ad_value(700)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(700)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1231] && (!s.b[1232])) && (!s.b[1233])) {
            s.store_ad_value(694, A::add_scaled_inputs3(s.ad_value(705), 2.0, s.ad_value(700), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0));
            s.store_ad_value(695, A::add_scaled_products(A::sub(s.ad_value(705), s.ad_value(700)), A::sub(s.ad_value(705), s.ad_value(700)), 1.0, s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0)));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(701, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_add(715, 700, 701);
        }

        if (s.b[1231] && (!s.b[1232])) {
            s.store_neg(715, 715);
        }

        s.b[1240] = (s.v[160] > 0.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1241] = (((s.v[707]) as f64).abs() <= s.v[711]);
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if (s.b[1240] && s.b[1241]) {
            s.store_mul_neg_lhs(717, 707, 712);
        }

        s.b[1242] = (s.v[707] < (-s.v[711]));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {
            s.store_neg(683, 707);
            s.store_scaled_mul(684, 683, 712, 1.25);
            s.store_scaled_sub_ad(685, A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(684), (-6.0), A::offset(s.ad_value(684), (-6.0))), 64.0)), 0.5);
            s.store_ad_value(686, A::add_scaled_products(A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685)), 1.0, s.ad_value(709), A::offset(s.ad_value(685), 1.0), 1.0));
            s.store_ad_value(687, A::add_scaled_inputs3(s.ad_value(683), 2.0, s.ad_value(685), (-2.0), s.ad_value(709), -1.0));
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
            s.store_add(689, 686, 687);
            s.store_ad_value(690, A::add_scaled_square_product(s.ad_value(689), 1.0, s.ad_value(688), A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0));
            s.store_add_ad_rhs(691, 690, A::mul3(A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), s.ad_value(687), A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0)));
            s.store_add_ad_rhs(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));
        }

        s.b[1243] = (((s.v[692]) as f64).abs() < 80.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (((s.b[1240] && (!s.b[1241])) && s.b[1242]) && s.b[1243]) {
            s.store_exp(693, 692);
        }

        s.b[1244] = (s.v[692] < (-80.0));
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if ((((s.b[1240] && (!s.b[1241])) && s.b[1242]) && (!s.b[1243])) && s.b[1244]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(692)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(692)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1240] && (!s.b[1241])) && s.b[1242]) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(s.ad_value(692), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(692), (-80.0)), 0.5, A::scale_offset(s.ad_value(692), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1240] && (!s.b[1241])) && s.b[1242]) {
            s.store_sub(691, 683, 692);
            s.store_ad_value(694, A::add_scaled_offset_product_rhs(s.ad_value(691), 2.0, s.ad_value(709), s.ad_value(693), (-1.0), 1.0));
            s.store_ad_value(695, A::add_scaled_square_product(s.ad_value(691), 1.0, s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693)), 1.0));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(697, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_neg_ad(717, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {
            s.store_mul_offset_ad_lhs(698, A::mul_scaled_lhs(s.ad_value(710), 1.25, s.ad_value(713)), (-1.0), 713);
            s.store_mul_ad_product_rhs(699, 707, s.ad_value(712), A::offset(A::mul(s.ad_value(698), s.ad_value(707)), 1.0));
        }

        s.b[1245] = ((((-s.v[699])) as f64).abs() < 80.0);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if (((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && s.b[1245]) {
            s.store_exp_neg_input(691, 699);
        }

        s.b[1246] = ((-s.v[699]) < (-80.0));
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1245])) && s.b[1246]) {
            s.store_div_from_scalar_offset_ad(691, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(699))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(699))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1245])) && (!s.b[1246])) {
            s.store_scaled_offset_ad(691, A::mul_offset_lhs(A::neg(s.ad_value(699)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(699)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {
            s.store_sub_from_scalar(697, 1.0, 691);
            s.store_ad_value(700, A::add_scaled_inputs_product(s.ad_value(707), 1.0, s.ad_value(709), 0.5, s.ad_value(708), A::sqrt(A::add_scaled_inputs3(s.ad_value(707), 1.0, s.ad_value(709), 0.25, s.ad_value(697), -1.0)), (-1.0)));
        }

        s.b[1247] = ((((-s.v[700])) as f64).abs() < 80.0);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if (((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && s.b[1247]) {
            s.store_exp_neg_input(693, 700);
        }

        s.b[1248] = ((-s.v[700]) < (-80.0));
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && s.b[1248]) {
            s.store_div_from_scalar_offset_ad(693, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(700))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(700))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) && (!s.b[1247])) && (!s.b[1248])) {
            s.store_scaled_offset_ad(693, A::mul_offset_lhs(A::neg(s.ad_value(700)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(700)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1240] && (!s.b[1241])) && (!s.b[1242])) {
            s.store_ad_value(694, A::add_scaled_inputs3(s.ad_value(707), 2.0, s.ad_value(700), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(709), 1.0, s.ad_value(693)), 1.0));
            s.store_ad_value(695, A::add_scaled_products(A::sub(s.ad_value(707), s.ad_value(700)), A::sub(s.ad_value(707), s.ad_value(700)), 1.0, s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693)), (-1.0)));
            s.store_sub_from_scalar_ad(696, 1.0, A::mul_scaled_lhs(s.ad_value(709), 0.5, s.ad_value(693)));
            s.store_ad_value(691, A::add_scaled_square_product(s.ad_value(694), 1.0, s.ad_value(696), s.ad_value(695), (-4.0)));
            s.store_ad_value(701, A::div_scaled_inputs(s.ad_value(695), 2.0, A::add(s.ad_value(694), A::sqrt(s.ad_value(691))), 1.0));
            s.store_add(717, 700, 701);
        }

        if (s.b[1240] && (!s.b[1241])) {
            s.store_neg(717, 717);
        }

        s.store_mul_scaled_ad_rhs(718, 223, -1.0, A::add(s.ad_value(704), s.ad_value(714)));

        s.store_mul_scaled_ad_rhs(719, 223, -1.0, A::add(s.ad_value(705), s.ad_value(715)));

        s.store_mul_scaled_ad_rhs(349, 223, -1.0, A::add(s.ad_value(706), s.ad_value(716)));

        s.store_mul_scaled_ad_rhs(350, 223, -1.0, A::add(s.ad_value(707), s.ad_value(717)));

        s.v[733] = 0.0;

        s.v[734] = 0.0;

        s.v[351] = 0.0;

        s.v[352] = 0.0;

        s.v[353] = 0.0;

        s.v[753] = 0.0;

        s.v[754] = 0.0;

        s.b[1249] = (p.p3 > 0.0);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        s.b[1250] = ((s.v[69] > 0.0) || (s.v[71] > 0.0));
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if (s.b[1249] && s.b[1250]) {
            s.store_add(720, 718, 285);
            s.store_scaled_sub_ad_rhs(721, 720, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(720), s.ad_value(720), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(722, A::offset(A::square(s.ad_value(718)), 0.0001), 276);
        }

        s.b[1251] = ((((0.5 * s.v[704])) as f64).abs() < 80.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1251]) {
            s.store_exp_scaled_input(0, 704, 0.5);
        }

        s.b[1252] = ((0.5 * s.v[704]) < (-80.0));
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1250]) && (!s.b[1251])) && s.b[1252]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul_offset_lhs(A::neg(A::scale(s.ad_value(704), 0.5)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(s.ad_value(704), 0.5)), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(s.ad_value(704), 0.5)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {
            s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(704), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(704), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(704), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);
            s.store_sub_from_scalar(3, 1.0, 2);
            s.store_ad_value(723, A::add_scaled_products(s.ad_value(83), s.ad_value(2), 1.0, s.ad_value(80), s.ad_value(3), 1.0));
            s.store_ad_value(724, A::add_scaled_products(s.ad_value(84), s.ad_value(2), 1.0, s.ad_value(82), s.ad_value(3), 1.0));
            s.store_ad_value(725, A::add_scaled_products(s.ad_value(282), s.ad_value(2), 1.0, s.ad_value(281), s.ad_value(3), 1.0));
            s.store_ad_value(726, A::add_scaled_products(s.ad_value(71), s.ad_value(2), 1.0, s.ad_value(69), s.ad_value(3), 1.0));
            s.store_scaled_mul(727, 73, 3, 1e-6);
            s.store_mul_ad_rhs(2, 279, A::div_scaled_inputs(s.ad_value(81), (-1.0), s.ad_value(722), 1.0));
        }

        s.b[1253] = (s.v[724] < 0.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1253]) {
            s.store_ad_value(722, A::add_scaled_inputs3(s.ad_value(722), 0.5, s.ad_value(725), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(722), s.ad_value(725)), A::sub(s.ad_value(722), s.ad_value(725))), 1e-6)), (-0.5)));
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_ad_value(728, A::add_scaled_product(A::offset(s.ad_value(714), 3.0), 1.0, s.ad_value(721), s.ad_value(224), 1.0));
        }

        s.b[1254] = (((s.v[728]) as f64).abs() < 80.0);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1254]) {
            s.store_exp(729, 728);
        }

        s.b[1255] = (s.v[728] < (-80.0));
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1250]) && (!s.b[1254])) && s.b[1255]) {
            s.store_div_from_scalar_offset_ad(729, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(728)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(728)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1250]) && (!s.b[1254])) && (!s.b[1255])) {
            s.store_scaled_offset_ad(729, A::mul_offset_lhs(s.ad_value(728), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(728), (-80.0)), 0.5, A::scale_offset(s.ad_value(728), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_add_ad_lhs(728, A::add_scaled_product(A::offset(s.ad_value(714), 3.0), 1.0, s.ad_value(721), s.ad_value(224), 1.0), 704);
        }

        s.b[1256] = (((s.v[728]) as f64).abs() < 80.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1256]) {
            s.store_exp(730, 728);
        }

        s.b[1257] = (s.v[728] < (-80.0));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1250]) && (!s.b[1256])) && s.b[1257]) {
            s.store_div_from_scalar_offset_ad(730, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(728)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(728)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1250]) && (!s.b[1256])) && (!s.b[1257])) {
            s.store_scaled_offset_ad(730, A::mul_offset_lhs(s.ad_value(728), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(728), (-80.0)), 0.5, A::scale_offset(s.ad_value(728), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_mul_offset_ad_rhs(0, 279, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(723), 1.0, s.ad_value(724), s.ad_value(722), 1.0)), (-1.5));
        }

        s.b[1258] = (s.v[0] > 0.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1258]) {
            s.store_offset_ad(731, A::mul_offset_rhs(s.ad_value(0), A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0), 1.0);
        }

        s.b[1259] = (s.v[0] > (-80.0));
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1250]) && (!s.b[1258])) && s.b[1259]) {
            s.store_exp(731, 0);
        }

        if (((s.b[1249] && s.b[1250]) && (!s.b[1258])) && (!s.b[1259])) {
            s.store_div_from_scalar_offset_ad(731, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        s.b[1260] = (s.v[2] > 0.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1260]) {
            s.store_offset_ad(732, A::mul_offset_rhs(s.ad_value(2), A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0), 1.0);
        }

        s.b[1261] = (s.v[2] > (-80.0));
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1250]) && (!s.b[1260])) && s.b[1261]) {
            s.store_exp(732, 2);
        }

        if (((s.b[1249] && s.b[1250]) && (!s.b[1260])) && (!s.b[1261])) {
            s.store_div_from_scalar_offset_ad(732, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_div_ad(0, A::offset(s.ad_value(729), 1.0), A::offset(s.ad_value(730), 1.0));
        }

        s.b[1262] = (s.v[0] < 1e-80);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1262]) {
            s.store_scalar(0, 1e-80);
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_mul_sub_rhs(2, 85, 332, 86);
        }

        s.b[1263] = (((s.v[2]) as f64).abs() < 80.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1263]) {
            s.store_exp(3, 2);
        }

        s.b[1264] = (s.v[2] < (-80.0));
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1250]) && (!s.b[1263])) && s.b[1264]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1250]) && (!s.b[1263])) && (!s.b[1264])) {
            s.store_scaled_offset_ad(3, A::mul_offset_lhs(s.ad_value(2), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(2), (-80.0)), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_ad_value(4, A::add_scaled_product(s.ad_value(2), 1.0, s.ad_value(85), s.ad_value(703), 1.0));
        }

        s.b[1265] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1250]) && s.b[1265]) {
            s.store_exp(5, 4);
        }

        s.b[1266] = (s.v[4] < (-80.0));
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1250]) && (!s.b[1265])) && s.b[1266]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1250]) && (!s.b[1265])) && (!s.b[1266])) {
            s.store_scaled_offset_ad(5, A::mul_offset_lhs(s.ad_value(4), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1250]) {
            s.store_sub_ad(733, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(726), s.ad_value(731), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(727), s.ad_value(732), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));
        }

        s.b[1267] = ((s.v[70] > 0.0) || (s.v[72] > 0.0));
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if (s.b[1249] && s.b[1267]) {
            s.store_add(720, 719, 285);
            s.store_scaled_sub_ad_rhs(721, 720, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(720), s.ad_value(720), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(722, A::offset(A::square(s.ad_value(719)), 0.0001), 276);
        }

        s.b[1268] = ((((0.5 * s.v[705])) as f64).abs() < 80.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1268]) {
            s.store_exp_scaled_input(0, 705, 0.5);
        }

        s.b[1269] = ((0.5 * s.v[705]) < (-80.0));
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1267]) && (!s.b[1268])) && s.b[1269]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul_offset_lhs(A::neg(A::scale(s.ad_value(705), 0.5)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(s.ad_value(705), 0.5)), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(s.ad_value(705), 0.5)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1267]) && (!s.b[1268])) && (!s.b[1269])) {
            s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(705), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(705), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(705), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1267]) {
            s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);
            s.store_sub_from_scalar(3, 1.0, 2);
            s.store_ad_value(723, A::add_scaled_products(s.ad_value(83), s.ad_value(2), 1.0, s.ad_value(80), s.ad_value(3), 1.0));
            s.store_ad_value(724, A::add_scaled_products(s.ad_value(84), s.ad_value(2), 1.0, s.ad_value(82), s.ad_value(3), 1.0));
            s.store_ad_value(725, A::add_scaled_products(s.ad_value(282), s.ad_value(2), 1.0, s.ad_value(281), s.ad_value(3), 1.0));
            s.store_ad_value(726, A::add_scaled_products(s.ad_value(72), s.ad_value(2), 1.0, s.ad_value(70), s.ad_value(3), 1.0));
            s.store_scaled_mul(727, 74, 3, 1e-6);
            s.store_mul_ad_rhs(2, 279, A::div_scaled_inputs(s.ad_value(81), (-1.0), s.ad_value(722), 1.0));
        }

        s.b[1270] = (s.v[724] < 0.0);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1270]) {
            s.store_ad_value(722, A::add_scaled_inputs3(s.ad_value(722), 0.5, s.ad_value(725), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(722), s.ad_value(725)), A::sub(s.ad_value(722), s.ad_value(725))), 1e-6)), (-0.5)));
        }

        if (s.b[1249] && s.b[1267]) {
            s.store_ad_value(728, A::add_scaled_product(A::offset(s.ad_value(715), 3.0), 1.0, s.ad_value(721), s.ad_value(224), 1.0));
        }

        s.b[1271] = (((s.v[728]) as f64).abs() < 80.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1271]) {
            s.store_exp(729, 728);
        }

        s.b[1272] = (s.v[728] < (-80.0));
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1267]) && (!s.b[1271])) && s.b[1272]) {
            s.store_div_from_scalar_offset_ad(729, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(728)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(728)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1267]) && (!s.b[1271])) && (!s.b[1272])) {
            s.store_scaled_offset_ad(729, A::mul_offset_lhs(s.ad_value(728), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(728), (-80.0)), 0.5, A::scale_offset(s.ad_value(728), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1249] && s.b[1267]) {
            s.store_add_ad_lhs(728, A::add_scaled_product(A::offset(s.ad_value(715), 3.0), 1.0, s.ad_value(721), s.ad_value(224), 1.0), 705);
        }

        s.b[1273] = (((s.v[728]) as f64).abs() < 80.0);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1273]) {
            s.store_exp(730, 728);
        }

        s.b[1274] = (s.v[728] < (-80.0));
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1267]) && (!s.b[1273])) && s.b[1274]) {
            s.store_div_from_scalar_offset_ad(730, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(728)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(728)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1267]) && (!s.b[1273])) && (!s.b[1274])) {
            s.store_scaled_offset_ad(730, A::mul_offset_lhs(s.ad_value(728), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(728), (-80.0)), 0.5, A::scale_offset(s.ad_value(728), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1267]) {
            s.store_mul_offset_ad_rhs(0, 279, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(723), 1.0, s.ad_value(724), s.ad_value(722), 1.0)), (-1.5));
        }

        s.b[1275] = (s.v[0] > 0.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1275]) {
            s.store_offset_ad(731, A::mul_offset_rhs(s.ad_value(0), A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0), 1.0);
        }

        s.b[1276] = (s.v[0] > (-80.0));
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1267]) && (!s.b[1275])) && s.b[1276]) {
            s.store_exp(731, 0);
        }

        if (((s.b[1249] && s.b[1267]) && (!s.b[1275])) && (!s.b[1276])) {
            s.store_div_from_scalar_offset_ad(731, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        s.b[1277] = (s.v[2] > 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1277]) {
            s.store_offset_ad(732, A::mul_offset_rhs(s.ad_value(2), A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0), 1.0);
        }

        s.b[1278] = (s.v[2] > (-80.0));
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1267]) && (!s.b[1277])) && s.b[1278]) {
            s.store_exp(732, 2);
        }

        if (((s.b[1249] && s.b[1267]) && (!s.b[1277])) && (!s.b[1278])) {
            s.store_div_from_scalar_offset_ad(732, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (s.b[1249] && s.b[1267]) {
            s.store_div_ad(0, A::offset(s.ad_value(729), 1.0), A::offset(s.ad_value(730), 1.0));
        }

        s.b[1279] = (s.v[0] < 1e-80);
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1279]) {
            s.store_scalar(0, 1e-80);
        }

        if (s.b[1249] && s.b[1267]) {
            s.store_mul_sub_rhs(2, 85, 330, 86);
        }

        s.b[1280] = (((s.v[2]) as f64).abs() < 80.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1280]) {
            s.store_exp(3, 2);
        }

        s.b[1281] = (s.v[2] < (-80.0));
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1267]) && (!s.b[1280])) && s.b[1281]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1267]) && (!s.b[1280])) && (!s.b[1281])) {
            s.store_scaled_offset_ad(3, A::mul_offset_lhs(s.ad_value(2), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(2), (-80.0)), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1267]) {
            s.store_ad_value(4, A::add_scaled_product(s.ad_value(2), 1.0, s.ad_value(85), s.ad_value(702), 1.0));
        }

        s.b[1282] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1267]) && s.b[1282]) {
            s.store_exp(5, 4);
        }

        s.b[1283] = (s.v[4] < (-80.0));
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1267]) && (!s.b[1282])) && s.b[1283]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1267]) && (!s.b[1282])) && (!s.b[1283])) {
            s.store_scaled_offset_ad(5, A::mul_offset_lhs(s.ad_value(4), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1267]) {
            s.store_sub_ad(734, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(726), s.ad_value(731), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(727), s.ad_value(732), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));
        }

        s.b[1284] = (s.v[68] > 0.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (s.b[1249] && s.b[1284]) {
            s.store_mul_neg_lhs(735, 436, 386);
        }

        s.b[1285] = (((((2.0 * s.v[735]) - s.v[411])) as f64).abs() < 80.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1284]) && s.b[1285]) {
            s.store_exp_ad(0, A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0));
        }

        s.b[1286] = (((2.0 * s.v[735]) - s.v[411]) < (-80.0));
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1284]) && (!s.b[1285])) && s.b[1286]) {
            let assign26710_ad_e28192: A = A::div_from_scalar(1.80485e-35, A::offset(A::mul_offset_lhs(A::neg(A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0)), (-80.0)), 0.5, A::scale_offset(A::neg(A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0));
            s.store_ad_value(0, assign26710_ad_e28192);
        }

        if (((s.b[1249] && s.b[1284]) && (!s.b[1285])) && (!s.b[1286])) {
            s.store_scaled_offset_ad(0, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0), (-80.0)), 0.5, A::scale_offset(A::sub_scaled_inputs(s.ad_value(735), 2.0, s.ad_value(411), 1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1284]) {
            s.store_mul_sub_ad_rhs(736, 226, A::offset(s.ad_value(735), 0.6931471805599), A::ln(A::offset(s.ad_value(0), 1.0)));
            s.store_scaled_add(737, 392, 412, 0.5);
            s.store_mul(738, 226, 737);
            s.store_add(720, 738, 284);
            s.store_scaled_sub_ad_rhs(721, 720, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(720), s.ad_value(720), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(722, A::offset(A::square(s.ad_value(738)), 0.0001), 276);
        }

        s.b[1287] = (s.v[79] < 0.0);
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1284]) && s.b[1287]) {
            s.store_ad_value(722, A::add_scaled_inputs3(s.ad_value(722), 0.5, s.ad_value(280), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(722), s.ad_value(280)), A::sub(s.ad_value(722), s.ad_value(280))), 1e-6)), (-0.5)));
        }

        if (s.b[1249] && s.b[1284]) {
            s.store_add(740, 400, 234);
            s.store_sub(739, 740, 737);
            s.store_mul_ad_lhs(728, A::add_scaled_product(s.ad_value(739), 1.0, A::add_scaled_inputs3(s.ad_value(721), 1.0, s.ad_value(283), (-1.0), s.ad_value(736), -1.0), s.ad_value(227), 1.0), 286);
        }

        s.b[1288] = (((s.v[728]) as f64).abs() < 80.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1284]) && s.b[1288]) {
            s.store_exp(729, 728);
        }

        s.b[1289] = (s.v[728] < (-80.0));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1284]) && (!s.b[1288])) && s.b[1289]) {
            s.store_div_from_scalar_offset_ad(729, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(728)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(728)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1284]) && (!s.b[1288])) && (!s.b[1289])) {
            s.store_scaled_offset_ad(729, A::mul_offset_lhs(s.ad_value(728), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(728), (-80.0)), 0.5, A::scale_offset(s.ad_value(728), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1284]) {
            s.store_mul_ad_affine_product_lhs(728, A::sub(s.ad_value(335), s.ad_value(736)), s.ad_value(227), -1.0, 0.0, 286);
        }

        s.b[1290] = (((s.v[728]) as f64).abs() < 80.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1284]) && s.b[1290]) {
            s.store_exp(0, 728);
        }

        s.b[1291] = (s.v[728] < (-80.0));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1284]) && (!s.b[1290])) && s.b[1291]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(728)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(728)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(728)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1249] && s.b[1284]) && (!s.b[1290])) && (!s.b[1291])) {
            s.store_scaled_offset_ad(0, A::mul_offset_lhs(s.ad_value(728), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(728), (-80.0)), 0.5, A::scale_offset(s.ad_value(728), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1284]) {
            s.store_mul(730, 729, 0);
            s.store_mul_offset_ad_rhs(0, 278, A::mul(s.ad_value(722), A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(722), 1.0)), (-1.5));
        }

        s.b[1292] = (s.v[0] > 0.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1284]) && s.b[1292]) {
            s.store_offset_ad(731, A::mul_offset_rhs(s.ad_value(0), A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0), 1.0);
        }

        s.b[1293] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1284]) && (!s.b[1292])) && s.b[1293]) {
            s.store_exp(731, 0);
        }

        s.b[1294] = (s.v[0] < (-80.0));
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((((s.b[1249] && s.b[1284]) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {
            s.store_div_from_scalar_offset_ad(731, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((((s.b[1249] && s.b[1284]) && (!s.b[1292])) && (!s.b[1293])) && (!s.b[1294])) {
            s.store_scaled_offset_ad(731, A::mul_offset_lhs(s.ad_value(0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1249] && s.b[1284]) {
            s.store_mul_ad_product_rhs(741, 68, s.ad_value(731), A::ln(A::div(A::offset(s.ad_value(729), 1.0), A::offset(s.ad_value(730), 1.0))));
        }

        s.b[1295] = ((s.v[740] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0)));
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if ((s.b[1249] && s.b[1284]) && s.b[1295]) {
            s.store_scalar(742, 1.0);
            s.store_scalar(743, 0.5);
        }

        if ((s.b[1249] && s.b[1284]) && (!s.b[1295])) {
            s.store_ad_value(0, A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(722), 2.0));
            s.store_mul_div_ad_lhs(744, s.ad_value(87), A::mul(s.ad_value(0), s.ad_value(278)), 227);
            s.store_div(745, 735, 744);
            s.store_mul3_lhs(746, 744, 434, 401);
            s.store_ad_value(748, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(746), 1.0, s.ad_value(746), 0.5));
            s.store_sub_from_scalar_ad(747, 0.5, A::scale(s.ad_value(748), 3.0));
        }

        s.b[1296] = (s.v[745] < 0.001);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && s.b[1296]) {
            s.store_square(749, 745);
            s.store_offset_mul_ad(742, s.ad_value(749), A::add_scaled_product(A::scale_offset(s.ad_value(746), 0.3333333333333, 0.1666666666667), 1.0, s.ad_value(749), A::scale_offset(s.ad_value(746), 0.2, 0.05), 0.1666666666667), 1.0);
            s.store_ad_value(743, A::add_scaled_offset_product_rhs(s.ad_value(742), 0.5, s.ad_value(745), A::mul(s.ad_value(749), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(748), 0.25, 0.4), 1.0, s.ad_value(749), s.ad_value(748), 0.125, 0.0285714285714)), 1.0, (-0.1666666666667)));
        }

        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_div_from_scalar(750, 1.0, 745);
        }

        s.b[1297] = (((s.v[745]) as f64).abs() < 80.0);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if ((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && s.b[1297]) {
            s.store_exp(751, 745);
        }

        s.b[1298] = (s.v[745] < (-80.0));
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if (((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && (!s.b[1297])) && s.b[1298]) {
            s.store_div_from_scalar_offset_ad(751, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(745)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(745)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(745)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) && (!s.b[1297])) && (!s.b[1298])) {
            s.store_scaled_offset_ad(751, A::mul_offset_lhs(s.ad_value(745), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(745), (-80.0)), 0.5, A::scale_offset(s.ad_value(745), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (((s.b[1249] && s.b[1284]) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_div_from_scalar(752, 1.0, 751);
            s.store_sub(0, 751, 752);
            s.store_add(3, 751, 752);
            s.store_ad_value(742, A::add_scaled_products(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(746), s.ad_value(0)), s.ad_value(750), 0.5, s.ad_value(746), s.ad_value(3), 0.5));
            s.store_scaled_sub_ad(743, A::add_scaled_product(s.ad_value(742), 1.0, s.ad_value(0), A::sub(s.ad_value(748), A::mul3(s.ad_value(747), s.ad_value(750), s.ad_value(750))), (-1.0)), A::mul3(s.ad_value(747), s.ad_value(3), s.ad_value(750)), 0.5);
        }

        if (s.b[1249] && s.b[1284]) {
            s.store_mul(351, 741, 742);
            s.store_mul(754, 741, 743);
            s.store_sub(753, 351, 754);
        }

        s.b[1299] = (s.v[334] < 0.0);
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if (s.b[1249] && s.b[1299]) {
            s.store_add(352, 754, 733);
            s.store_add(353, 753, 734);
        }

        if (s.b[1249] && (!s.b[1299])) {
            s.store_add(352, 753, 733);
            s.store_add(353, 754, 734);
        }

        s.v[355] = 0.0;

        s.b[1300] = (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[718] < 0.0));
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if s.b[1300] {
            s.store_sqrt_offset_ad(755, A::add(A::square(s.ad_value(718)), A::mul3(A::square(s.ad_value(95)), s.ad_value(331), s.ad_value(331))), 1e-6);
            s.store_scaled_div(0, 91, 755, -1.0);
        }

        s.b[1301] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if (s.b[1300] && s.b[1301]) {
            s.store_exp(3, 0);
        }

        s.b[1302] = (s.v[0] < (-80.0));
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if ((s.b[1300] && (!s.b[1301])) && s.b[1302]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((s.b[1300] && (!s.b[1301])) && (!s.b[1302])) {
            s.store_scaled_offset_ad(3, A::mul_offset_lhs(s.ad_value(0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1300] {
            s.store_mul(4, 97, 703);
        }

        s.b[1303] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if (s.b[1300] && s.b[1303]) {
            s.store_exp(5, 4);
        }

        s.b[1304] = (s.v[4] < (-80.0));
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if ((s.b[1300] && (!s.b[1303])) && s.b[1304]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((s.b[1300] && (!s.b[1303])) && (!s.b[1304])) {
            s.store_scaled_offset_ad(5, A::mul_offset_lhs(s.ad_value(4), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1300] {
            s.store_scaled_mul_ad(355, A::mul3(A::mul3_scaled_output(s.ad_value(89), s.ad_value(703), s.ad_value(718), -1.0), s.ad_value(755), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);
        }

        s.v[354] = 0.0;

        s.b[1305] = (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[719] < 0.0));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if s.b[1305] {
            s.store_sqrt_offset_ad(756, A::add(A::square(s.ad_value(719)), A::mul3(A::square(s.ad_value(96)), s.ad_value(333), s.ad_value(333))), 1e-6);
            s.store_scaled_div(0, 92, 756, -1.0);
        }

        s.b[1306] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (s.b[1305] && s.b[1306]) {
            s.store_exp(3, 0);
        }

        s.b[1307] = (s.v[0] < (-80.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((s.b[1305] && (!s.b[1306])) && s.b[1307]) {
            s.store_div_from_scalar_offset_ad(3, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((s.b[1305] && (!s.b[1306])) && (!s.b[1307])) {
            s.store_scaled_offset_ad(3, A::mul_offset_lhs(s.ad_value(0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1305] {
            s.store_mul(4, 98, 702);
        }

        s.b[1308] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (s.b[1305] && s.b[1308]) {
            s.store_exp(5, 4);
        }

        s.b[1309] = (s.v[4] < (-80.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((s.b[1305] && (!s.b[1308])) && s.b[1309]) {
            s.store_div_from_scalar_offset_ad(5, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((s.b[1305] && (!s.b[1308])) && (!s.b[1309])) {
            s.store_scaled_offset_ad(5, A::mul_offset_lhs(s.ad_value(4), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1305] {
            s.store_scaled_mul_ad(354, A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(702), s.ad_value(719), -1.0), s.ad_value(756), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);
        }

        s.v[356] = 0.0;

        s.b[1310] = (p.p12 > 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1310] {
            s.store_mul(758, 336, 289);
            s.store_mul_offset_ad_lhs(759, A::sqrt(A::offset(A::square(s.ad_value(336)), 0.01)), (-0.1), 289);
            s.store_scaled_sub(760, 758, 759, 0.5);
            s.store_sub_ad_lhs(761, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub(s.ad_value(335), s.ad_value(100)), s.ad_value(289), 1.0), 234);
            s.store_sub_ad_lhs(762, A::add_scaled_product(s.ad_value(760), (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(101), 1.0), s.ad_value(289), 1.0), 234);
            s.store_div_from_scalar_offset_input(763, 1.0, 105, 1.0);
            s.store_div_from_scalar_offset_input(764, 1.0, 106, 1.0);
            s.store_mul(765, 109, 289);
            s.store_mul_scaled_ad_rhs(0, 765, 2.0, A::offset(A::sqrt(A::offset(A::div(s.ad_value(759), s.ad_value(765)), 1.0)), (-1.0)));
            s.store_mul(766, 107, 0);
            s.store_mul(767, 108, 0);
            s.store_ad_value(768, A::add_scaled_product(s.ad_value(760), 1.0, A::add(s.ad_value(761), s.ad_value(766)), s.ad_value(763), 1.0));
            s.store_ad_value(769, A::add_scaled_product(s.ad_value(760), 1.0, A::add(s.ad_value(762), s.ad_value(767)), s.ad_value(764), 1.0));
        }

        if s.b[1310] {
            let assign27840_ad_e29793: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), 1.0, s.ad_value(225), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), s.ad_value(225)), A::sub(A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(103), A::sub(s.ad_value(768), s.ad_value(769)), 1.0), s.ad_value(225))), 0.01)), -1.0);
            s.store_scale_ad(770, assign27840_ad_e29793, 0.5);
        }

        if s.b[1310] {
            let assign27850_ad_e29830: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), 1.0, s.ad_value(225), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), s.ad_value(225)), A::sub(A::add_scaled_product(s.ad_value(768), 1.0, s.ad_value(104), A::sub(s.ad_value(769), s.ad_value(768)), 1.0), s.ad_value(225))), 0.01)), -1.0);
            s.store_scale_ad(771, assign27850_ad_e29830, 0.5);
        }

        if s.b[1310] {
            s.store_div(772, 246, 763);
            s.store_div(773, 247, 764);
            s.store_div_from_scalar(774, 1.0, 772);
            s.store_div_from_scalar(775, 1.0, 773);
            s.store_div_from_scalar_add_ad(776, 1.0, A::offset(s.ad_value(774), 1.0), s.ad_value(775));
            s.store_div_ad_rhs(777, 290, A::square(s.ad_value(390)));
            s.store_mul_sub_rhs(778, 776, 770, 771);
        }

        s.b[1311] = ((((s.v[771] - s.v[770])) as f64).abs() <= 1e-12);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if (s.b[1310] && s.b[1311]) {
            s.store_ad_value(2, A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(776), s.ad_value(774)), 1.0, s.ad_value(776), s.ad_value(775), (-1.0)));
            s.store_mul_sub_ad_lhs(3, A::add_scaled_inputs3(s.ad_value(775), 1.0, A::mul3_scaled_output(s.ad_value(774), s.ad_value(776), s.ad_value(774), 0.5), 1.0, A::mul3_scaled_output(s.ad_value(775), s.ad_value(776), s.ad_value(775), 0.5), -1.0), A::div_from_scalar(0.5, s.ad_value(776)), 778);
            s.store_ad_value(4, A::div_scaled_product(A::sub(s.ad_value(2), s.ad_value(3)), s.ad_value(777), 0.5, s.ad_value(776), 1.0));
        }

        if (s.b[1310] && (!s.b[1311])) {
            s.store_exp_ad(2, A::mul_scaled_lhs(s.ad_value(774), -1.0, s.ad_value(778)));
            s.store_exp_ad(3, A::mul(A::sub(s.ad_value(775), A::div_from_scalar(1.0, s.ad_value(776))), s.ad_value(778)));
            s.store_ad_value(4, A::div_scaled_product(s.ad_value(777), A::sub(s.ad_value(2), s.ad_value(3)), 1.0, s.ad_value(778), 2.0));
        }

        if s.b[1310] {
            s.copy_ad(779, 4);
        }

        s.b[1312] = (s.v[770] < 80.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (s.b[1310] && s.b[1312]) {
            s.store_ln_ad(784, A::offset(A::mul(s.ad_value(779), A::exp(s.ad_value(770))), 1.0));
            s.store_mul_sub_from_scalar_ad_rhs(0, 784, 1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)));
        }

        s.b[1313] = (s.v[770] < 0.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        s.b[1314] = (s.v[770] > (-80.0));
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((s.b[1310] && (!s.b[1312])) && s.b[1313]) && s.b[1314]) {
            s.store_exp(784, 770);
        }

        if (((s.b[1310] && (!s.b[1312])) && s.b[1313]) && (!s.b[1314])) {
            s.store_div_from_scalar_offset_ad(784, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(770)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(770)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(770)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((s.b[1310] && (!s.b[1312])) && s.b[1313]) {
            s.store_mul(0, 779, 784);
        }

        if ((s.b[1310] && (!s.b[1312])) && (!s.b[1313])) {
            s.store_add_ad_lhs(784, A::ln(s.ad_value(779)), 770);
            s.store_mul_sub_from_scalar_ad_rhs(0, 784, 1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)));
        }

        if s.b[1310] {
            s.copy_ad(780, 0);
        }

        s.b[1315] = ((s.v[770] - s.v[411]) < 80.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (s.b[1310] && s.b[1315]) {
            s.store_ln_ad(784, A::offset(A::mul(s.ad_value(779), A::exp(A::sub(s.ad_value(770), s.ad_value(411)))), 1.0));
            s.store_mul_sub_from_scalar_ad_rhs(0, 784, 1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)));
        }

        s.b[1316] = ((s.v[770] - s.v[411]) < 0.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        s.b[1317] = ((s.v[770] - s.v[411]) > (-80.0));
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1310] && (!s.b[1315])) && s.b[1316]) && s.b[1317]) {
            s.store_exp_sub(784, 770, 411);
        }

        if (((s.b[1310] && (!s.b[1315])) && s.b[1316]) && (!s.b[1317])) {
            s.store_div_from_scalar_offset_ad(784, 1.80485e-35, A::mul_offset_lhs(A::neg(A::sub(s.ad_value(770), s.ad_value(411))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::sub(s.ad_value(770), s.ad_value(411))), (-80.0)), 0.5, A::scale_offset(A::neg(A::sub(s.ad_value(770), s.ad_value(411))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if ((s.b[1310] && (!s.b[1315])) && s.b[1316]) {
            s.store_mul(0, 779, 784);
        }

        if ((s.b[1310] && (!s.b[1315])) && (!s.b[1316])) {
            s.store_ad_value(784, A::add_scaled_inputs3(A::ln(s.ad_value(779)), 1.0, s.ad_value(770), 1.0, s.ad_value(411), (-1.0)));
            s.store_mul_sub_from_scalar_ad_rhs(0, 784, 1.0, A::div(A::ln(A::offset(s.ad_value(784), 1.0)), A::offset(s.ad_value(784), 2.0)));
        }

        if s.b[1310] {
            s.copy_ad(781, 0);
            s.store_ad_value(782, A::mul_offset_lhs(A::add_scaled_inputs(s.ad_value(780), 0.5, s.ad_value(781), 0.5), 1.0, A::sub(s.ad_value(780), s.ad_value(781))));
            s.store_mul_square_lhs(783, 288, 110);
            s.store_ad_value(356, A::div_scaled_product3(s.ad_value(783), s.ad_value(241), s.ad_value(782), 1.0, s.ad_value(422), 1.0));
        }

        s.v[357] = 0.0;

        s.v[358] = 0.0;

        s.b[1318] = (p.p8 != 0.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if s.b[1318] {
            s.store_div_ad_lhs(757, A::add_scaled_product(s.ad_value(339), 1.0, s.ad_value(115), s.ad_value(411), (-1.0)), 227);
        }

        s.b[1319] = (s.v[757] > 0.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (s.b[1318] && s.b[1319]) {
            s.store_ad_value(3, A::div_scaled_inputs(s.ad_value(113), (-1.0), A::offset(s.ad_value(757), 1e-30), 1.0));
        }

        s.b[1320] = (((s.v[3]) as f64).abs() < 80.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if ((s.b[1318] && s.b[1319]) && s.b[1320]) {
            s.store_exp(0, 3);
        }

        s.b[1321] = (s.v[3] < (-80.0));
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (((s.b[1318] && s.b[1319]) && (!s.b[1320])) && s.b[1321]) {
            s.store_div_from_scalar_offset_ad(0, 1.80485e-35, A::mul_offset_lhs(A::neg(s.ad_value(3)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(3)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1318] && s.b[1319]) && (!s.b[1320])) && (!s.b[1321])) {
            s.store_scaled_offset_ad(0, A::mul_offset_lhs(s.ad_value(3), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(3), (-80.0)), 0.5, A::scale_offset(s.ad_value(3), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (s.b[1318] && s.b[1319]) {
            s.store_mul3_lhs(357, 112, 757, 0);
            s.store_mul_add_rhs(358, 357, 348, 356);
        }

        s.b[1322] = (s.v[6] > 0.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if s.b[1322] {
            s.store_mul_abs_ad_lhs(0, A::mul(A::add(s.ad_value(348), s.ad_value(356)), s.ad_value(336)), 168);
        }

        s.b[1323] = (s.v[0] > (100000000.0 * p.p16));
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if (s.b[1322] && s.b[1323]) {
            s.store_div_from_scalar(359, (-(p.p16 + (0.25 / p.p16))), 168);
        }

        if (s.b[1322] && (!s.b[1323])) {
            s.store_ad_value(359, A::div_scaled_inputs(A::offset(A::sub_scaled_inputs(A::offset(s.ad_value(0), p.p16), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(0), (-p.p16), A::offset(s.ad_value(0), (-p.p16))), 1.0)), 0.5), (0.25 / p.p16)), -1.0, s.ad_value(168), 1.0));
        }

        if s.b[1322] {
            s.store_div(360, 219, 168);
        }

        if (!s.b[1322]) {
            s.store_scalar(359, 0.0);
            s.store_scaled_voltage(360, ctx, nodes, Some(4), None, 0.001);
        }

        s.store_mul(317, 306, 316);

        s.store_mul(321, 306, 320);

        s.store_mul(325, 306, 324);

        s.store_mul(328, 306, 327);

        s.b[1608] = (p.p11 > 0.0);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if s.b[1608] {
            s.copy_ad(1418, 130);
            s.copy_ad(1419, 131);
            s.copy_ad(1420, 135);
            s.copy_ad(1421, 136);
            s.copy_ad(1422, 140);
            s.copy_ad(1423, 141);
            s.copy_ad(1424, 274);
            s.copy_ad(1425, 216);
            s.copy_ad(1426, 158);
            s.store_sub_ad_lhs(1427, A::add_scaled_product(s.ad_value(341), (-1.0), A::sub(s.ad_value(335), s.ad_value(1418)), s.ad_value(227), 1.0), 234);
            s.store_ad_value(1428, A::add_scaled_product(s.ad_value(341), (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1419), 1.0), s.ad_value(227), 1.0));
            s.store_sub(1429, 1428, 234);
        }

        s.b[1609] = (p.p2 > 0.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1609]) {
            s.store_scale(0, 16, p.p14);
            s.store_div_ad(1430, A::offset(s.ad_value(246), 1.0), A::offset(s.ad_value(247), 1.0));
            s.store_ln(1431, 1430);
        }

        s.b[1610] = (s.v[1431] > 1e-8);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1609]) && s.b[1610]) {
            s.store_ad_value(1432, A::div_scaled_product_offset_denominator(s.ad_value(1431), A::offset(s.ad_value(1430), 1.0), 2.0, s.ad_value(1430), (-1.0), 1.0));
        }

        if ((s.b[1608] && s.b[1609]) && (!s.b[1610])) {
            s.store_scaled_offset(1432, 1431, 2.0, 2.0);
        }

        if (s.b[1608] && s.b[1609]) {
            s.store_div_ad_rhs(1433, 253, A::square(s.ad_value(245)));
            s.store_div_from_scalar(1434, 1.0, 246);
            s.store_div_from_scalar(1435, 1.0, 247);
            s.store_div_from_scalar_add_ad(1462, 1.0, A::offset(s.ad_value(1434), 1.0), s.ad_value(1435));
            s.store_mul_sub_rhs(1463, 1462, 1427, 1429);
            s.store_ad_value(1436, A::add_scaled_product(s.ad_value(1427), 1.0, s.ad_value(1463), s.ad_value(1434), (-1.0)));
            s.store_ad_value(1437, A::add_scaled_product(s.ad_value(1429), 1.0, s.ad_value(1463), s.ad_value(1435), 1.0));
            s.store_div_from_scalar_offset_input(1342, 1.0, 246, 1.0);
            s.store_div_from_scalar_offset_input(1343, 1.0, 247, 1.0);
            s.store_offset_ln_ad(1345, A::div_scaled_product(A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(247), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 1.5);
            s.store_offset_ln_ad(1346, A::div_scaled_product(A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(246), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 1.5);
        }

        s.b[1611] = (((s.v[1345] - s.v[1436]) / 1.5) < 80.0);
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1609]) && s.b[1611]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1345), 0.6666666666666666, s.ad_value(1436), 0.6666666666666666)));
        }

        if ((s.b[1608] && s.b[1609]) && (!s.b[1611])) {
            s.store_scaled_sub(1344, 1345, 1436, 0.6666666666666666);
        }

        if (s.b[1608] && s.b[1609]) {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 1.5);
            s.store_mul_ad_lhs(1348, A::add_scaled_product(s.ad_value(1349), 1.0, s.ad_value(247), s.ad_value(1429), 1.0), 1343);
        }

        s.b[1612] = (((s.v[1346] - s.v[1348]) / 1.5) < 80.0);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1609]) && s.b[1612]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1346), 0.6666666666666666, s.ad_value(1348), 0.6666666666666666)));
        }

        if ((s.b[1608] && s.b[1609]) && (!s.b[1612])) {
            s.store_scaled_sub(1344, 1346, 1348, 0.6666666666666666);
        }

        if (s.b[1608] && s.b[1609]) {
            s.store_sub_scaled_inputs(1, 1346, 1.0, 1344, 1.5);
            s.store_mul(2, 0, 1);
            s.store_mul(3, 0, 1429);
            s.store_sub(1394, 2, 3);
        }

        s.b[1613] = ((((-s.v[266])) as f64).abs() < 80.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1609]) && s.b[1613]) {
            s.store_exp_neg_input(1395, 266);
        }

        s.b[1614] = ((-s.v[266]) < (-80.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1609]) && (!s.b[1613])) && s.b[1614]) {
            s.store_div_from_scalar_offset_ad(1395, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(266))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(266))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1613])) && (!s.b[1614])) {
            s.store_scaled_offset_ad(1395, A::mul_offset_lhs(A::neg(s.ad_value(266)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(266)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(266)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        s.b[1615] = (((s.v[1394]) as f64).abs() <= s.v[265]);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1609]) && s.b[1615]) {
            s.store_scaled_square(1392, 264, (0.1666666666667 * 0.707106781186545));
            s.store_mul_ad_product_rhs(4, 1394, s.ad_value(264), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(1394), 1.0, s.ad_value(1395)), s.ad_value(260), s.ad_value(1392)), 1.0));
        }

        s.b[1616] = (s.v[1394] < (-s.v[265]));
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {
            s.store_neg(1396, 1394);
            s.store_scaled_mul(1397, 1396, 264, 1.25);
            s.store_scaled_sub_ad(1398, A::offset(s.ad_value(1397), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1397), (-6.0), A::offset(s.ad_value(1397), (-6.0))), 64.0)), 0.5);
            s.store_sub(1391, 1396, 1398);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {
            s.store_ad_value(1399, A::add_scaled_square_product(s.ad_value(1391), 1.0, s.ad_value(261), A::offset(s.ad_value(1398), 1.0), 1.0));
            s.store_sub_scaled_inputs(1401, 1391, 2.0, 261, 1.0);
            s.store_sub_ad_lhs(1402, A::ln(A::mul(s.ad_value(1399), s.ad_value(262))), 1398);
            s.store_add(1389, 1399, 1401);
            s.store_ad_value(1390, A::add_scaled_square_product(s.ad_value(1389), 1.0, s.ad_value(1402), A::add_scaled_product(s.ad_value(1399), (-1.0), s.ad_value(1401), s.ad_value(1401), 0.5), 1.0));
            s.store_add_ad_rhs(1403, 1398, A::div_scaled_product3(s.ad_value(1399), s.ad_value(1389), s.ad_value(1402), 1.0, A::add(s.ad_value(1390), A::mul3(A::mul3(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402), s.ad_value(1402)), s.ad_value(1401), A::sub_scaled_inputs(A::square(s.ad_value(1401)), 0.3333333333333, s.ad_value(1399), 1.0))), 1.0));
        }

        s.b[1617] = (s.v[1403] < 80.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) && s.b[1617]) {
            s.store_exp(1404, 1403);
        }

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) && (!s.b[1617])) {
            s.store_scaled_offset_ad(1404, A::mul_offset_lhs(s.ad_value(1403), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(1403), (-80.0)), 0.5, A::scale_offset(s.ad_value(1403), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && s.b[1616]) {
            s.store_div_from_scalar(1405, 1.0, 1404);
            s.store_div_from_scalar_offset_ad(1391, 1.0, A::square(s.ad_value(1403)), 2.0);
            s.store_mul_square_lhs(1406, 1403, 1391);
            s.store_mul3_affine_lhs(1407, 1403, 1391, 4.0, 0.0, 1391);
            s.store_mul_ad_product_lhs(1408, A::sub_scaled_inputs(s.ad_value(1391), 8.0, s.ad_value(1406), 12.0), s.ad_value(1391), 1391);
            s.store_sub(1391, 1396, 1403);
            s.store_mul(1392, 1395, 1405);
            s.store_ad_value(1409, A::add_scaled_product(s.ad_value(1391), 2.0, s.ad_value(261), A::add_scaled_inputs3_offset(s.ad_value(1404), 1.0, s.ad_value(1392), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(1395), 1.0, s.ad_value(1407)), 1.0, (-1.0)), 1.0));
            s.store_ad_value(1410, A::add_scaled_square_product(s.ad_value(1391), 1.0, s.ad_value(261), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1404), 1.0, s.ad_value(1403), (-1.0), s.ad_value(1392), 1.0, (-1.0)), 1.0, s.ad_value(1395), A::sub(A::offset(s.ad_value(1403), (-1.0)), s.ad_value(1406)), 1.0), (-1.0)));
            s.store_sub_from_scalar_ad(1391, 2.0, A::mul(s.ad_value(261), A::add_scaled_inputs_product(s.ad_value(1404), 1.0, s.ad_value(1392), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0))));
            s.store_ad_value(1391, A::add_scaled_square_product(s.ad_value(1409), 1.0, s.ad_value(1410), s.ad_value(1391), (-2.0)));
            s.store_ad_value(4, A::sub_scaled_inputs(s.ad_value(1403), -1.0, A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0));
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_div_from_scalar_offset_scaled_input(1411, 1.0, 260, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(1412, A::mul_scaled_lhs(s.ad_value(263), 1.25, s.ad_value(1411)), (-1.0), 1411);
            s.store_mul_ad_product_rhs(1413, 1394, s.ad_value(264), A::offset(A::mul(s.ad_value(1412), s.ad_value(1394)), 1.0));
        }

        s.b[1618] = ((-s.v[1413]) > (-80.0));
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1618]) {
            s.store_exp_neg_input(1391, 1413);
        }

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1618])) {
            s.store_div_from_scalar_offset_ad(1391, 1.80485e-35, A::mul_offset_lhs(A::neg(A::neg(s.ad_value(1413))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(1413))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(1413))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_sub_from_scalar(1414, 1.0, 1391);
            s.store_ad_value(1415, A::add_scaled_inputs_product(s.ad_value(1394), 1.0, s.ad_value(261), 0.5, s.ad_value(260), A::sqrt(A::add_scaled_inputs3(s.ad_value(1394), 1.0, s.ad_value(261), 0.25, s.ad_value(1414), -1.0)), (-1.0)));
            s.store_offset(1416, 266, 3.0);
            s.store_ad_value(1398, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1415), 0.5, s.ad_value(1416), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1415), s.ad_value(1416)), A::sub(s.ad_value(1415), s.ad_value(1416))), 5.0)), (-0.5)), 1.0, s.ad_value(1416), (-0.5), A::sqrt(A::offset(A::square(s.ad_value(1416)), 5.0)), (-(-0.5))));
            s.store_sub(1391, 1394, 1398);
            s.store_exp_neg_input(1392, 1398);
            s.store_div_from_scalar_offset_ad(1393, 1.0, A::square(s.ad_value(1398)), 2.0);
            s.store_mul_square_lhs(1406, 1398, 1393);
            s.store_mul3_affine_lhs(1407, 1398, 1393, 4.0, 0.0, 1393);
            s.store_mul_ad_product_lhs(1408, A::sub_scaled_inputs(s.ad_value(1393), 8.0, s.ad_value(1406), 12.0), s.ad_value(1393), 1393);
            s.store_max_from_scalar_ad(1399, 1e-40, A::add_scaled_square_product(s.ad_value(1391), 1.0, s.ad_value(261), A::add_scaled_product(A::offset(A::add(s.ad_value(1392), s.ad_value(1398)), (-1.0)), 1.0, s.ad_value(1395), A::add(A::offset(s.ad_value(1398), 1.0), s.ad_value(1406)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(1400, 1.0, A::mul_scaled_output(s.ad_value(261), A::add_scaled_product(s.ad_value(1392), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0)), 0.5));
            s.store_ad_value(1401, A::add_scaled_product(s.ad_value(1391), 2.0, s.ad_value(261), A::add_scaled_sub_value_product(1.0, s.ad_value(1392), 1.0, s.ad_value(1395), A::offset(s.ad_value(1407), 1.0), (-1.0)), 1.0));
            s.store_ad_value(1402, A::add_scaled_inputs3(s.ad_value(266), 1.0, s.ad_value(1398), (-1.0), A::ln(A::div(s.ad_value(1399), s.ad_value(261))), 1.0));
            s.store_add(1389, 1399, 1401);
            s.store_ad_value(1390, A::add_scaled_square_product(s.ad_value(1389), 1.0, s.ad_value(1402), A::add_scaled_products(s.ad_value(1401), s.ad_value(1401), 0.5, s.ad_value(1399), s.ad_value(1400), (-1.0)), 1.0));
            s.store_add_ad_rhs(1417, 1398, A::div_scaled_product3(s.ad_value(1399), s.ad_value(1389), s.ad_value(1402), 1.0, A::add(s.ad_value(1390), A::mul3(A::mul3(A::div(s.ad_value(1389), s.ad_value(1390)), s.ad_value(1402), s.ad_value(1402)), s.ad_value(1401), A::add_scaled_square_product(s.ad_value(1401), 0.3333333333333, s.ad_value(1399), s.ad_value(1400), (-1.0)))), 1.0));
        }

        s.b[1619] = (s.v[1417] < 80.0);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && s.b[1619]) {
            s.store_exp(1404, 1417);
            s.store_div_from_scalar(1405, 1.0, 1404);
            s.store_mul(1404, 1395, 1404);
        }

        s.b[1620] = (s.v[1417] > (s.v[266] - 80.0));
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if (((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1619])) && s.b[1620]) {
            s.store_exp_sub(1404, 1417, 266);
            s.store_div(1405, 1395, 1404);
        }

        if (((((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) && (!s.b[1619])) && (!s.b[1620])) {
            s.store_div_from_scalar_offset_ad(1404, 1.80485e-35, A::mul_offset_lhs(A::sub(s.ad_value(266), s.ad_value(1417)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(266), s.ad_value(1417)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(266), s.ad_value(1417)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(1405, 1.80485e-35, A::mul_offset_lhs(s.ad_value(1417), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(1417), (-80.0)), 0.5, A::scale_offset(s.ad_value(1417), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0);
        }

        if (((s.b[1608] && s.b[1609]) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_div_from_scalar_offset_ad(1391, 1.0, A::square(s.ad_value(1417)), 2.0);
            s.store_mul_square_lhs(1406, 1417, 1391);
            s.store_mul3_affine_lhs(1407, 1417, 1391, 4.0, 0.0, 1391);
            s.store_mul_ad_product_lhs(1408, A::sub_scaled_inputs(s.ad_value(1391), 8.0, s.ad_value(1406), 12.0), s.ad_value(1391), 1391);
            s.store_sub(1391, 1394, 1417);
            s.store_ad_value(1409, A::add_scaled_product(s.ad_value(1391), 2.0, s.ad_value(261), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(1405)), 1.0, s.ad_value(1404), 1.0, s.ad_value(1395), A::offset(s.ad_value(1407), 1.0), (-1.0)), 1.0));
            s.store_ad_value(1410, A::add_scaled_square_product(s.ad_value(1391), 1.0, s.ad_value(261), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1405), 1.0, s.ad_value(1417), 1.0, s.ad_value(1404), 1.0, (-1.0)), 1.0, s.ad_value(1395), A::add(A::offset(s.ad_value(1417), 1.0), s.ad_value(1406)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(1391, 2.0, A::mul(s.ad_value(261), A::add_scaled_inputs_product(s.ad_value(1405), 1.0, s.ad_value(1404), 1.0, s.ad_value(1395), s.ad_value(1408), (-1.0))));
            s.store_ad_value(1391, A::add_scaled_square_product(s.ad_value(1409), 1.0, s.ad_value(1410), s.ad_value(1391), (-2.0)));
            s.store_ad_value(4, A::add_scaled_inputs(s.ad_value(1417), 1.0, A::div(s.ad_value(1410), A::add(s.ad_value(1409), A::sqrt(s.ad_value(1391)))), 2.0));
        }

        if (s.b[1608] && s.b[1609]) {
            s.store_mul_add_rhs(1438, 0, 4, 3);
        }

        if (s.b[1608] && (!s.b[1609])) {
            s.copy_ad(1438, 1429);
        }

        if s.b[1608] {
            s.store_mul_sub_rhs(0, 248, 1427, 1438);
        }

        s.b[1621] = (p.p13 > 0.0);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1621]) {
            s.store_ad_value(1439, A::add_scaled_inputs3(s.ad_value(0), 0.5, s.ad_value(257), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257)), 1.0)), 0.5));
            s.store_ad_value(1440, A::add_scaled_inputs3(s.ad_value(257), 0.5, s.ad_value(0), ((-1.0) * 0.5), A::sqrt(A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), 1.0)), 0.5));
            s.store_mul_ad_rhs(2, 258, A::exp_scaled_input(A::ln(s.ad_value(1439)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 258, A::exp_scaled_input(A::ln(s.ad_value(1440)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div(1447, 245, 4);
            s.store_offset_mul(1441, 246, 2, 1.0);
            s.store_offset_mul(1442, 247, 3, 1.0);
            s.store_ad_value(1443, A::div_scaled_product(s.ad_value(246), s.ad_value(4), 1.0, s.ad_value(1441), 1.0));
            s.store_ad_value(1444, A::div_scaled_product(s.ad_value(247), s.ad_value(4), 1.0, s.ad_value(1442), 1.0));
            s.store_div_from_scalar_add_ad(1445, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1443)), 1.0), A::div_from_scalar(1.0, s.ad_value(1444)));
            s.store_offset_mul(1441, 1443, 2, 1.0);
            s.store_offset_mul(1442, 1444, 3, 1.0);
        }

        if (s.b[1608] && (!s.b[1621])) {
            s.copy_ad(1447, 245);
            s.copy_ad(1443, 246);
            s.copy_ad(1444, 247);
            s.copy_ad(1445, 248);
            s.store_scalar(1441, 1.0);
            s.store_scalar(1442, 1.0);
        }

        if s.b[1608] {
            s.store_mul_sub_rhs(1446, 1445, 1427, 1438);
        }

        s.b[1622] = (s.v[1446] > 0.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        s.b[1623] = ((-s.v[1446]) < 80.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1622]) && s.b[1623]) {
            s.store_ln_one_plus_exp_neg_input(0, 1446);
        }

        if ((s.b[1608] && s.b[1622]) && (!s.b[1623])) {
            s.store_neg(0, 1446);
        }

        if (s.b[1608] && s.b[1622]) {
            s.store_offset_ad(1448, A::add_scaled_inputs3(s.ad_value(1427), 1.0, A::div(s.ad_value(1446), s.ad_value(1443)), (-1.0), s.ad_value(0), 1.0), (-0.6931471805599));
        }

        s.b[1624] = (s.v[1446] < 80.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1622])) && s.b[1624]) {
            s.store_ln_one_plus_exp(0, 1446);
        }

        if ((s.b[1608] && (!s.b[1622])) && (!s.b[1624])) {
            s.copy_ad(0, 1446);
        }

        if (s.b[1608] && (!s.b[1622])) {
            s.store_offset_ad(1448, A::add_scaled_inputs3(s.ad_value(1438), 1.0, A::div(s.ad_value(1446), s.ad_value(1444)), 1.0, s.ad_value(0), 1.0), (-0.6931471805599));
        }

        if s.b[1608] {
            s.store_ad_value(1449, A::add_scaled_inputs3(s.ad_value(1448), 0.5, s.ad_value(254), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1448), s.ad_value(254)), A::sub(s.ad_value(1448), s.ad_value(254))), 4.0)), (-0.5)));
            s.store_offset_sqrt_ad(1450, A::offset(A::div(A::sub_scaled_inputs(s.ad_value(254), 2.0, s.ad_value(1449), 2.0), s.ad_value(255)), 1.0), (-1.0));
            s.store_ad_value(1451, A::add_scaled_product(s.ad_value(1449), 1.0, s.ad_value(255), s.ad_value(1450), 1.0));
            s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(1428)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(1428)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(1428)), ((1.0) + ((-0.5))))), 0.01)), 0.5);
            s.store_div_from_scalar_offset_ad(1452, 1.0, A::mul(s.ad_value(1420), s.ad_value(0)), 1.0);
            s.store_div_from_scalar_offset_ad(1453, 1.0, A::mul(s.ad_value(1421), s.ad_value(0)), 1.0);
            s.store_ad_value(0, A::mul_offset_rhs(A::mul3_scaled_output(s.ad_value(329), A::offset(A::sqrt(A::offset(A::div(s.ad_value(340), s.ad_value(329)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1450)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1428)), 1.0));
            s.store_mul(1454, 1422, 0);
            s.store_mul(1455, 1423, 0);
            s.store_add_ad_lhs(1456, A::add_scaled_product(s.ad_value(1451), 1.0, A::add_scaled_inputs3(s.ad_value(1427), 1.0, s.ad_value(1451), (-1.0), s.ad_value(1454), 1.0), s.ad_value(1452), 1.0), 341);
            s.store_add_ad_lhs(1457, A::add_scaled_product(s.ad_value(1451), 1.0, A::add_scaled_inputs3(s.ad_value(1438), 1.0, s.ad_value(1451), (-1.0), s.ad_value(1455), 1.0), s.ad_value(1453), 1.0), 341);
        }

        if s.b[1608] {
            let assign30140_ad_e32954: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)), 1.0), 1.0, s.ad_value(225), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)), 1.0), s.ad_value(225)), A::sub(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(25), A::sub(s.ad_value(1456), s.ad_value(1457)), 1.0), s.ad_value(225))), 0.01)), -1.0);
            s.store_scale_ad(1458, assign30140_ad_e32954, 0.5);
        }

        if s.b[1608] {
            let assign30150_ad_e32991: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)), 1.0), 1.0, s.ad_value(225), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)), 1.0), s.ad_value(225)), A::sub(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(26), A::sub(s.ad_value(1457), s.ad_value(1456)), 1.0), s.ad_value(225))), 0.01)), -1.0);
            s.store_scale_ad(1459, assign30150_ad_e32991, 0.5);
        }

        if s.b[1608] {
            s.store_div(1460, 1443, 1452);
            s.store_div(1461, 1444, 1453);
            s.store_div_from_scalar(1434, 1.0, 1460);
            s.store_div_from_scalar(1435, 1.0, 1461);
            s.store_div_from_scalar_add_ad(1462, 1.0, A::offset(s.ad_value(1434), 1.0), s.ad_value(1435));
            s.store_div_ad_rhs(1433, 253, A::square(s.ad_value(1447)));
            s.store_div_ad(1430, A::offset(s.ad_value(1460), 1.0), A::offset(s.ad_value(1461), 1.0));
            s.store_ln(1431, 1430);
        }

        s.b[1625] = (s.v[1431] > 1e-8);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1625]) {
            s.store_ad_value(1432, A::div_scaled_product_offset_denominator(s.ad_value(1431), A::offset(s.ad_value(1430), 1.0), 2.0, s.ad_value(1430), (-1.0), 1.0));
        }

        if (s.b[1608] && (!s.b[1625])) {
            s.store_scaled_offset(1432, 1431, 2.0, 2.0);
        }

        if s.b[1608] {
            s.store_mul_sub_rhs(1463, 1462, 1458, 1459);
            s.store_square(1464, 1463);
            s.store_ad_value(1436, A::add_scaled_product(s.ad_value(1458), 1.0, s.ad_value(1463), s.ad_value(1434), (-1.0)));
            s.store_ad_value(1437, A::add_scaled_product(s.ad_value(1459), 1.0, s.ad_value(1463), s.ad_value(1435), 1.0));
            s.store_div_from_scalar_offset_input(1342, 1.0, 1460, 1.0);
            s.store_div_from_scalar_offset_input(1343, 1.0, 1461, 1.0);
            s.store_offset_ln_ad(1345, A::div_scaled_product(A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 3.0);
            s.store_offset_ln_ad(1346, A::div_scaled_product(A::add_scaled_product(s.ad_value(1461), 1.0, s.ad_value(1460), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0), 3.0);
        }

        s.b[1626] = (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1626]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1436), 0.3333333333333)));
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && (!s.b[1626])) {
            s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1627] = (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1627]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1437), 0.3333333333333)));
        }

        if (s.b[1608] && (!s.b[1627])) {
            s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_mul_ad_lhs(1347, A::add_scaled_product(s.ad_value(1350), 1.0, s.ad_value(1460), s.ad_value(1458), 1.0), 1342);
            s.store_mul_ad_lhs(1348, A::add_scaled_product(s.ad_value(1349), 1.0, s.ad_value(1461), s.ad_value(1459), 1.0), 1343);
        }

        s.b[1628] = (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1628]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1347), 0.3333333333333)));
        }

        if (s.b[1608] && (!s.b[1628])) {
            s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1629] = (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1629]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1348), 0.3333333333333)));
        }

        if (s.b[1608] && (!s.b[1629])) {
            s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_sub(1465, 1458, 1349);
            s.store_sub(1469, 1459, 1350);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1359, 0.0);
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1630] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1630]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1630])) {
            s.store_scaled_offset_ad(1342, A::mul_offset_lhs(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1458), s.ad_value(1465)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1631] = (s.v[1353] < (-0.005));
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1631]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1632] = (s.v[1353] > 0.005);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1631])) && s.b[1632]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1631])) && (!s.b[1632])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1633] = (s.v[1353] > 0.005);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1633]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1634] = (s.v[1353] < (-0.005));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1633])) && s.b[1634]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1633])) && (!s.b[1634])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1635] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1635]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1635])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1636] = (s.v[1365] > 0.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1636]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (s.b[1608] && (!s.b[1636])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1465, 1465, 1380);
            s.store_mul(1351, 1460, 1465);
            s.store_mul(1381, 1461, 1469);
            s.store_add(1374, 1351, 1381);
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_ad_value(1383, A::add_scaled_product(A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0));
            s.store_ad_value(1384, A::add_scaled_product(s.ad_value(1374), (2.0 * 39.478417604), s.ad_value(1351), s.ad_value(1381), 39.478417604));
            s.store_sqrt_ad(1385, A::add_scaled_square_product(s.ad_value(1383), 1.0, s.ad_value(1382), s.ad_value(1384), (-4.0)));
            s.store_ad_value(1353, A::div_scaled_inputs(A::sub(s.ad_value(1385), s.ad_value(1383)), 1.0, s.ad_value(1382), 2.0));
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

        s.b[1637] = (s.v[1386] > 0.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1637]) {
            s.store_mul_ad_rhs(1377, 1386, A::add_scaled_inputs3(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0));
            s.store_ad_value(1378, A::add_scaled_product(s.ad_value(1386), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1387, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1465), (-1.0), s.ad_value(1345), -1.0));
        }

        s.b[1638] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1637]) && s.b[1638]) {
            s.store_sub_ad_rhs(1465, 1465, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1465);
            s.store_mul(1381, 1461, 1469);
            s.store_add(1374, 1351, 1381);
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_ad_value(1383, A::add_scaled_product(A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0));
            s.store_ad_value(1384, A::add_scaled_product(s.ad_value(1374), (2.0 * 39.478417604), s.ad_value(1351), s.ad_value(1381), 39.478417604));
            s.store_sqrt_ad(1385, A::add_scaled_square_product(s.ad_value(1383), 1.0, s.ad_value(1382), s.ad_value(1384), (-4.0)));
            s.store_ad_value(1353, A::div_scaled_inputs(A::sub(s.ad_value(1385), s.ad_value(1383)), 1.0, s.ad_value(1382), 2.0));
        }

        s.b[1639] = (s.v[1353] < (-0.005));
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1639]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_ad_lhs(1358, A::add_scaled_inputs(s.ad_value(1353), 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25), 1353);
        }

        s.b[1640] = (s.v[1353] > 0.005);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1639])) && s.b[1640]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_div_ad_lhs(1358, A::add_scaled_inputs(s.ad_value(1353), 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25), 1353);
        }

        if ((s.b[1608] && (!s.b[1639])) && (!s.b[1640])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_scaled_sub_from_scalar_ad(1358, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
        }

        if s.b[1608] {
            s.store_sub_ad_rhs(1353, 1353, A::div(A::add(A::add_scaled_products(s.ad_value(1374), s.ad_value(1357), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0), s.ad_value(1353)), A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0)));
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        s.b[1641] = (s.v[1386] > 0.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1641]) {
            s.store_mul_ad_rhs(1377, 1386, A::add_scaled_inputs3(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0));
            s.store_ad_value(1378, A::add_scaled_product(s.ad_value(1386), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1387, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1465), (-1.0), s.ad_value(1345), -1.0));
        }

        s.b[1642] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1641]) && s.b[1642]) {
            s.store_sub_ad_rhs(1465, 1465, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1643] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1643]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1643])) {
            s.store_scaled_offset_ad(1342, A::mul_offset_lhs(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1458), s.ad_value(1465)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1644] = (s.v[1353] < (-0.005));
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1644]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1645] = (s.v[1353] > 0.005);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1644])) && s.b[1645]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1644])) && (!s.b[1645])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1646] = (s.v[1353] > 0.005);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1646]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1647] = (s.v[1353] < (-0.005));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1646])) && s.b[1647]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1646])) && (!s.b[1647])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1648] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1648]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1648])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1649] = (s.v[1365] > 0.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1649]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (s.b[1608] && (!s.b[1649])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1465, 1465, 1380);
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1650] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1650]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1650])) {
            s.store_scaled_offset_ad(1342, A::mul_offset_lhs(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1458), s.ad_value(1465)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1651] = (s.v[1353] < (-0.005));
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1651]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1652] = (s.v[1353] > 0.005);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1651])) && s.b[1652]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1651])) && (!s.b[1652])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1653] = (s.v[1353] > 0.005);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1653]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1654] = (s.v[1353] < (-0.005));
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1653])) && s.b[1654]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1653])) && (!s.b[1654])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1655] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1655]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1655])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1608] && (!s.b[1655])) {
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1656] = (s.v[1365] > 0.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1656]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (s.b[1608] && (!s.b[1656])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1465, 1465, 1380);
        }

        s.b[1657] = (p.p10 == 1.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        s.b[1658] = (((s.v[1380]) as f64).abs() > 0.01);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {
            s.store_mul(1351, 1460, 1465);
        }

        s.b[1659] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1659]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1659])) {
            s.store_scaled_offset_ad(1342, A::mul_offset_lhs(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1458), s.ad_value(1465)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1660] = (s.v[1353] < (-0.005));
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1660]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1661] = (s.v[1353] > 0.005);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1660])) && s.b[1661]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1660])) && (!s.b[1661])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1662] = (s.v[1353] > 0.005);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1662]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1663] = (s.v[1353] < (-0.005));
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1662])) && s.b[1663]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1664] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1664]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1664])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1665] = (s.v[1365] > 0.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && s.b[1665]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (((s.b[1608] && s.b[1657]) && s.b[1658]) && (!s.b[1665])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1465);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if ((s.b[1608] && s.b[1657]) && s.b[1658]) {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1465, 1465, 1380);
        }

        if s.b[1608] {
            s.store_mul(1467, 1460, 1465);
        }

        s.b[1666] = ((s.v[1458] - s.v[1465]) < 80.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1666]) {
            s.store_exp_sub(1342, 1458, 1465);
        }

        if (s.b[1608] && (!s.b[1666])) {
            s.store_scaled_offset_ad(1342, A::mul_offset_lhs(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1458), s.ad_value(1465)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1458), s.ad_value(1465)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1471, 1433, 1342);
            s.store_sub_ad_lhs(1470, A::square(s.ad_value(1467)), 1471);
        }

        s.b[1667] = (s.v[1471] <= 0.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1667]) {
            s.store_scalar(1466, 1e-80);
            s.store_sub(1468, 1466, 1467);
            s.store_div(1469, 1468, 1461);
        }

        s.b[1668] = (s.v[1470] < (-0.005));
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1667])) && s.b[1668]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1470));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        s.b[1669] = (s.v[1470] > 0.005);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1470));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
        }

        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1668])) && (!s.b[1669])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1670] = (((1.01 * s.v[1467]) + s.v[1357]) > 0.0);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1667])) && s.b[1670]) {
            s.store_add(1342, 1467, 1357);
        }

        s.b[1671] = ((s.v[1471] * s.v[1467]) < (((0.9 * s.v[1467]) * s.v[1467]) * s.v[1342]));
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if (((s.b[1608] && (!s.b[1667])) && s.b[1670]) && s.b[1671]) {
            s.store_offset_div(1466, 1471, 1342, 1e-80);
            s.store_sub(1468, 1466, 1467);
            s.store_div(1469, 1468, 1461);
        }

        s.b[1672] = (s.v[1470] > 0.005);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && s.b[1672]) {
            s.store_sub_ad_lhs(1343, A::ln(A::div_scaled_inputs(s.ad_value(1470), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0)), 1356);
        }

        s.b[1673] = (s.v[1470] < (-0.005));
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if (((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {
            s.store_sin_scaled_input(1344, 1356, 0.5);
            s.store_ln_ad(1343, A::div_scaled_inputs(s.ad_value(1470), -1.0, A::square(s.ad_value(1344)), 1.0));
        }

        if (((((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) && (!s.b[1672])) && (!s.b[1673])) {
            s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        if (((s.b[1608] && (!s.b[1667])) && s.b[1670]) && (!s.b[1671])) {
            s.store_ad_value(1469, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1465), 1.0), 1.0, A::ln(s.ad_value(1342)), 2.0, s.ad_value(1343), -1.0));
            s.store_mul(1468, 1461, 1469);
            s.store_add(1466, 1467, 1468);
        }

        s.b[1674] = (s.v[1470] > 0.005);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        s.b[1675] = (((s.v[1465] - s.v[1458]) - s.v[1356]) < 80.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) && s.b[1675]) {
            s.store_exp_ad(1344, A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0));
        }

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) && (!s.b[1675])) {
            let assign34370_ad_e38450: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1356), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1344, assign34370_ad_e38450, 1.0, 5.54062e34);
        }

        if (((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && s.b[1674]) {
            s.store_div(1343, 1344, 1433);
            s.store_ad_value(1342, A::div_scaled_product(s.ad_value(1470), s.ad_value(1343), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
        }

        s.b[1676] = (s.v[1470] < (-0.005));
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && (!s.b[1674])) && s.b[1676]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_ad_lhs(1342, A::div_scaled_inputs(s.ad_value(1470), -1.0, A::square(s.ad_value(1343)), 1.0), 1471);
        }

        if ((((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) && (!s.b[1674])) && (!s.b[1676])) {
            s.store_div_ad_lhs(1342, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0396825396825397), 0.05), 0.3333333333333)), 1471);
        }

        if ((s.b[1608] && (!s.b[1667])) && (!s.b[1670])) {
            s.store_offset_div_ad(1466, A::sub(s.ad_value(1467), s.ad_value(1357)), A::sub_from_scalar(1.0, s.ad_value(1342)), 1e-80);
            s.store_sub(1468, 1466, 1467);
            s.store_div(1469, 1468, 1461);
        }

        s.b[1677] = ((s.v[1459] - s.v[1469]) < 80.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1677]) {
            s.store_exp_sub(1342, 1459, 1469);
        }

        if (s.b[1608] && (!s.b[1677])) {
            s.store_scaled_offset_ad(1342, A::mul_offset_lhs(A::sub(s.ad_value(1459), s.ad_value(1469)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1459), s.ad_value(1469)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1459), s.ad_value(1469)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0)), 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1472, 1433, 1342);
            s.store_scalar(1475, 0.0);
            s.store_scalar(1476, 0.0);
            s.store_scalar(1473, 0.0);
            s.store_scalar(1474, 0.0);
            s.store_scalar(1477, 0.0);
            s.store_scalar(1478, 0.0);
        }

        s.b[1678] = (s.v[1466] > 1e-6);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1678]) {
            s.store_mul(1473, 1471, 1434);
            s.store_mul(1474, 1472, 1435);
            s.store_add_scaled_inputs(1475, 1473, 1.0, 1467, 2.0);
            s.store_add_scaled_inputs(1476, 1474, 1.0, 1468, 2.0);
            s.store_ad_value(1477, A::add_scaled_inputs3(s.ad_value(1466), 2.0, s.ad_value(1473), 1.0, s.ad_value(1474), 1.0));
        }

        s.b[1679] = (((s.v[1470]) as f64).abs() > 0.005);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1678]) && s.b[1679]) {
            s.store_ad_value(2, A::add_scaled_offset_product_lhs(A::add_scaled_products(s.ad_value(1475), s.ad_value(1476), 1.0, A::offset(s.ad_value(1465), 2.0), s.ad_value(1476), 2.0), 1.0, s.ad_value(1469), 2.0, s.ad_value(1475), 2.0));
            s.store_ad_value(1478, A::div_scaled_product(s.ad_value(1470), s.ad_value(1477), (-4.0), A::mul(s.ad_value(1466), s.ad_value(2)), 1.0));
        }

        if ((s.b[1608] && s.b[1678]) && (!s.b[1679])) {
            s.store_scaled_sub_from_scalar_ad(2, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1470), 1.0, A::scale(s.ad_value(1470), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_ad_value(3, A::add_scaled_offset_product_rhs(A::add_scaled_products(s.ad_value(1475), s.ad_value(1471), 1.0, s.ad_value(1476), s.ad_value(1472), 1.0), 1.0, A::mul3(s.ad_value(1475), s.ad_value(1476), s.ad_value(1466)), A::mul(s.ad_value(1466), s.ad_value(2)), 1.0, 1.0));
            s.store_ad_value(1478, A::div_scaled_product3(s.ad_value(1471), s.ad_value(1472), s.ad_value(1477), 1.0, A::mul(s.ad_value(1466), s.ad_value(3)), 1.0));
        }

        if s.b[1608] {
            s.store_ln(1479, 1466);
        }

        s.b[1680] = ((s.v[1467] / 2.0) < 80.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1680]) {
            s.store_ln_one_plus_exp_scaled_input(2, 1467, 0.5);
        }

        if (s.b[1608] && (!s.b[1680])) {
            s.store_scale(2, 1467, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1480, 2, 2.0);
        }

        s.b[1681] = ((s.v[1468] / 2.0) < 80.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1681]) {
            s.store_ln_one_plus_exp_scaled_input(3, 1468, 0.5);
        }

        if (s.b[1608] && (!s.b[1681])) {
            s.store_scale(3, 1468, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1481, 3, 2.0);
            s.store_sub(1482, 1481, 1468);
            s.store_sub(1483, 1480, 1467);
            s.store_ad_value(1484, A::add_scaled_products(s.ad_value(270), s.ad_value(1480), 1.0, s.ad_value(271), s.ad_value(1482), 1.0));
            s.store_ad_value(1485, A::add_scaled_products(s.ad_value(270), s.ad_value(1481), 1.0, s.ad_value(271), s.ad_value(1483), 1.0));
            s.store_div_ad_rhs(0, 1466, A::add(s.ad_value(1480), s.ad_value(1481)));
            s.store_mul(1486, 1480, 0);
            s.store_mul(1487, 1481, 0);
            s.store_mul_ad_product_rhs(1488, 1480, s.ad_value(191), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
            s.store_mul_ad_product_rhs(1489, 1481, s.ad_value(192), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
            s.store_mul_ad_rhs(2, 50, A::add_scaled_product(s.ad_value(1482), 1.0, s.ad_value(51), s.ad_value(1483), 1.0));
            s.store_scaled_add_ad(3, A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2), 1.0, A::offset(s.ad_value(2), 1.0)), 0.01)), 0.5);
            s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);
            s.store_div(1490, 3, 4);
            s.store_mul_ad_product_rhs(1491, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1482)), 1.0), 1.0, s.ad_value(42), s.ad_value(1483), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1486), s.ad_value(268)), 1.0), 1.0, s.ad_value(1487), s.ad_value(269), 1.0)))));
        }

        s.b[1682] = (s.v[56] == 0.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1682]) {
            s.store_scalar(4, 1.0);
        }

        s.b[1683] = (s.v[56] < 0.0);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1682])) && s.b[1683]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((s.b[1608] && (!s.b[1682])) && (!s.b[1683])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1466), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        if s.b[1608] {
            s.store_mul_ad_affine_product_rhs(1492, 272, s.ad_value(1447), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428))), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(58), s.ad_value(1428)), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1428)))), 0.01))), 0.5, 0.0);
            s.store_mul_ad_rhs(1493, 1492, A::add_scaled_product(s.ad_value(54), 1.0, s.ad_value(1466), s.ad_value(4), 1.0));
            s.store_ad_value(1494, A::add_scaled_inputs_product(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1484)), 1e-6)))), 1.0), 1.0, s.ad_value(1491), 1.0, s.ad_value(38), s.ad_value(1493), 1.0));
            s.store_ad_value(1495, A::add_scaled_inputs_product(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1485)), 1e-6)))), 1.0), 1.0, s.ad_value(1491), 1.0, s.ad_value(39), s.ad_value(1493), 1.0));
            s.store_ad_value(1496, A::div_scaled_product(s.ad_value(1490), A::add(s.ad_value(1488), s.ad_value(1489)), 1.0, A::add(A::div(s.ad_value(1488), s.ad_value(1494)), A::div(s.ad_value(1489), s.ad_value(1495))), 1.0));
        }

        s.b[1684] = (((s.v[1463]) as f64).abs() > 0.007);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        s.b[1685] = (s.v[1463] > 0.0);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1684]) && s.b[1685]) {
            s.store_exp_neg_input(0, 1463);
            s.store_div_ad_rhs(1497, 1463, A::sub_from_scalar(1.0, s.ad_value(0)));
            s.store_mul(1498, 0, 1497);
            s.store_add_ad_lhs(1499, A::offset(A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1497)))), (-0.6931471805599)), 1436);
        }

        if ((s.b[1608] && s.b[1684]) && (!s.b[1685])) {
            s.store_exp(0, 1463);
            s.store_div_ad_rhs(1498, 1463, A::offset(s.ad_value(0), (-1.0)));
            s.store_mul(1497, 0, 1498);
            s.store_add_ad_lhs(1499, A::offset(A::ln(A::div(s.ad_value(1433), A::mul(s.ad_value(1466), s.ad_value(1498)))), (-0.6931471805599)), 1437);
        }

        if (s.b[1608] && s.b[1684]) {
            s.store_ad_value(1500, A::div_scaled_inputs(s.ad_value(1463), -1.0, A::mul(s.ad_value(1462), A::add_scaled_sub_value_product(1.0, s.ad_value(1497), 1.0, s.ad_value(1463), s.ad_value(1435), (-1.0))), 1.0));
            s.store_div_ad_rhs(1501, 1463, A::mul(s.ad_value(1462), A::add_scaled_sub_value_product(1.0, s.ad_value(1498), 1.0, s.ad_value(1463), s.ad_value(1434), 1.0)));
            s.store_div_ad_rhs(1502, 1463, A::sub(A::div(A::offset(A::mul(s.ad_value(1498), s.ad_value(1435)), 0.5), s.ad_value(1501)), A::div(A::offset(A::mul(s.ad_value(1497), s.ad_value(1434)), 0.5), s.ad_value(1500))));
        }

        if (s.b[1608] && (!s.b[1684])) {
            s.store_scale(0, 1464, (0.5 * 0.1666666666667));
            s.store_scale(2, 1463, 0.5);
            s.store_add_ad_lhs(1497, A::offset(s.ad_value(2), 1.0), 0);
            s.store_add_ad_lhs(1498, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
            s.store_scale(3, 2, 0.1666666666667);
            s.store_div_from_scalar_mul_ad(1500, 1.0, s.ad_value(1462), A::add(A::offset(s.ad_value(1435), 0.5), s.ad_value(3)));
            s.store_div_from_scalar_mul_ad(1501, 1.0, s.ad_value(1462), A::sub(A::offset(s.ad_value(1434), 0.5), s.ad_value(3)));
            s.store_ad_value(1499, A::add_scaled_inputs3_offset(A::ln(A::div(s.ad_value(1433), A::mul_sub_from_scalar_rhs(s.ad_value(1466), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, s.ad_value(1436), 0.5, s.ad_value(1437), 0.5, (-0.6931471805599)));
        }

        if (s.b[1608] && (!s.b[1684])) {
            let assign35250_ad_e39593: A = A::div_from_scalar((-12.0), A::add(A::add_scaled_inputs3_offset(s.ad_value(1462), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(1462), 12.0, A::mul(s.ad_value(1460), s.ad_value(1461)), 1.0), 1.0, A::mul3(s.ad_value(1462), A::sub(s.ad_value(1434), s.ad_value(1435)), s.ad_value(1463)), 1.0, 4.0), A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(1462), 0.25), s.ad_value(1464), 0.3333333333333)));
            s.store_ad_value(1502, assign35250_ad_e39593);
        }

        if s.b[1608] {
            s.store_div_from_scalar(1503, 1.0, 1502);
        }

        s.b[1686] = (s.v[1466] > 1e-6);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1686]) {
            s.store_ad_value(1504, A::div_scaled_inputs(s.ad_value(1480), 100.0, A::offset(s.ad_value(1480), 100.0), 1.0));
        }

        s.b[1687] = (s.v[61] < 0.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1686]) && s.b[1687]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1505, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1504)));
        }

        if ((s.b[1608] && s.b[1686]) && (!s.b[1687])) {
            s.store_offset_mul(1505, 61, 1504, 1.0);
        }

        if (s.b[1608] && s.b[1686]) {
            s.store_ad_value(1506, A::div_scaled_inputs(s.ad_value(1481), 100.0, A::offset(s.ad_value(1481), 100.0), 1.0));
        }

        s.b[1688] = (s.v[62] < 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1686]) && s.b[1688]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1507, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1506)));
        }

        if ((s.b[1608] && s.b[1686]) && (!s.b[1688])) {
            s.store_offset_mul(1507, 62, 1506, 1.0);
        }

        if (s.b[1608] && s.b[1686]) {
            s.store_sub_ad(1508, A::div_scaled_product(s.ad_value(1478), s.ad_value(1477), 1.0, A::mul(s.ad_value(1475), s.ad_value(1476)), 1.0), A::div(A::add(A::div(s.ad_value(1471), s.ad_value(1475)), A::div(s.ad_value(1472), s.ad_value(1476))), s.ad_value(1466)));
            s.store_ad_value(1509, A::div_scaled_product_offset_denominator(s.ad_value(1508), s.ad_value(1466), 1.0, s.ad_value(1508), 1.0, 1.0));
            s.store_sub(2, 1502, 1509);
            s.store_div_ad_lhs(1510, A::add_scaled_product(s.ad_value(1466), 1.0, s.ad_value(1502), s.ad_value(1499), 1.0), 2);
            s.store_scaled_add_ad_rhs(1510, 1510, A::sqrt(A::offset(A::square(s.ad_value(1510)), 1e-6)), 0.5);
            s.store_scaled_mul_ad(1511, A::div(s.ad_value(1424), s.ad_value(1496)), A::add(s.ad_value(1505), s.ad_value(1507)), 0.5);
            s.store_sub_from_scalar_ad(1512, 1.0, A::div(s.ad_value(1466), s.ad_value(1509)));
            s.store_offset(1513, 1499, 1.0);
            s.store_mul_sub_ad_lhs(1514, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1509), 2.0, s.ad_value(1466), 1.0), s.ad_value(1503)), (-2.0)), s.ad_value(1499), 1510);
        }

        s.b[1689] = (s.v[1511] > 1e-14);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1686]) && s.b[1689]) {
            s.store_div_from_scalar_square_ad(1515, 2.0, s.ad_value(1511));
            s.store_mul(1516, 1515, 1512);
            s.store_add(1517, 1515, 1514);
            s.store_mul(1518, 1515, 1513);
            s.store_sqrt_offset_ad(1519, A::add(A::square(s.ad_value(1516)), A::mul3_scaled_output(s.ad_value(1515), s.ad_value(1515), s.ad_value(1515), 0.148148148148)), 1e-20);
            s.store_sqrt_offset_ad(1520, A::add(A::square(s.ad_value(1518)), A::mul3_scaled_output(s.ad_value(1517), s.ad_value(1517), s.ad_value(1517), 0.148148148148)), 1e-20);
            s.store_sub_ad(1521, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1519), s.ad_value(1516)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1519), s.ad_value(1516)), 0.5), 0.3333333333333));
            s.store_sub_ad(1522, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1520), s.ad_value(1518)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1520), s.ad_value(1518)), 0.5), 0.3333333333333));
        }

        if ((s.b[1608] && s.b[1686]) && (!s.b[1689])) {
            s.copy_ad(1521, 1512);
            s.copy_ad(1522, 1513);
        }

        if (s.b[1608] && s.b[1686]) {
            s.store_square(4, 2);
            s.store_ad_value(1523, A::add_scaled_inputs3(s.ad_value(1521), (0.94 * 0.5), s.ad_value(1522), (0.94 * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(4), 10.0, A::sub(s.ad_value(1521), s.ad_value(1522)), A::sub(s.ad_value(1521), s.ad_value(1522)), 1.0)), (0.94 * 0.5)));
            s.store_ad_value(1524, A::add_scaled_product(s.ad_value(1466), 1.0, s.ad_value(1509), s.ad_value(1523), 1.0));
            s.store_mul_sub_rhs(1525, 1502, 1523, 1499);
            s.store_ad_value(1526, A::add_scaled_inputs3(s.ad_value(1524), 0.5, s.ad_value(1525), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(4), 36.0, A::sub(s.ad_value(1524), s.ad_value(1525)), A::sub(s.ad_value(1524), s.ad_value(1525)), 1.0)), 0.5));
        }

        if (s.b[1608] && (!s.b[1686])) {
            s.copy_ad(1509, 1502);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        if (s.b[1608] && (!s.b[1686])) {
            s.store_scaled_offset(1523, 1499, 1.0, 0.94);
            s.store_ad_value(1526, A::add_scaled_product(s.ad_value(1466), 0.5, s.ad_value(1502), A::sub_scaled_inputs(s.ad_value(1523), 1.0, s.ad_value(1499), 0.5), 1.0));
        }

        s.b[1690] = ((s.v[1526] - 0.5) < 80.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1690]) {
            s.store_ad_value(2, A::ln_one_plus_exp(A::offset(s.ad_value(1526), (-0.5))));
        }

        if (s.b[1608] && (!s.b[1690])) {
            s.store_offset(2, 1526, (-0.5));
        }

        if s.b[1608] {
            s.store_offset(3, 2, 0.5);
            s.store_add_ad_rhs(4, 1523, A::ln(A::div(s.ad_value(1466), s.ad_value(3))));
        }

        s.b[1691] = ((s.v[4] - 6.0) < 80.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1691]) {
            s.store_ad_value(2, A::ln_one_plus_exp(A::offset(s.ad_value(4), (-6.0))));
        }

        if (s.b[1608] && (!s.b[1691])) {
            s.store_offset(2, 4, (-6.0));
        }

        if s.b[1608] {
            s.store_offset(4, 2, 6.0);
        }

        s.b[1692] = ((s.v[225] - s.v[4]) < 80.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1692]) {
            s.store_ad_value(2, A::ln_one_plus_exp(A::sub(s.ad_value(225), s.ad_value(4))));
        }

        if (s.b[1608] && (!s.b[1692])) {
            s.store_sub(2, 225, 4);
        }

        if s.b[1608] {
            s.store_sub(1527, 225, 2);
            s.store_div(2, 339, 1527);
            s.store_square(3, 2);
            s.store_square(4, 3);
            s.store_square(5, 4);
            s.store_ad_value(0, A::exp_scaled_input(A::ln(A::offset(A::mul(s.ad_value(1425), s.ad_value(4)), 1.0)), 2.666666666667));
            s.store_mul_ad_rhs(1528, 339, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));
            s.store_div_from_scalar_offset_input(1342, 1.0, 1460, 1.0);
            s.store_div_from_scalar_offset_input(1343, 1.0, 1461, 1.0);
            s.store_offset_add_ad(1345, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1343), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0)), s.ad_value(1528), 3.0);
            s.store_offset_add_ad(1346, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1461), 1.0, s.ad_value(1460), s.ad_value(1342), 1.0), s.ad_value(1432), 1.0, s.ad_value(1433), 1.0)), s.ad_value(1528), 3.0);
        }

        s.b[1693] = (((s.v[1345] - s.v[1436]) * 0.3333333333333) < 80.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1693]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1436), 0.3333333333333)));
        }

        if (s.b[1608] && (!s.b[1693])) {
            s.store_scaled_sub(1344, 1345, 1436, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1694] = (((s.v[1346] - s.v[1437]) * 0.3333333333333) < 80.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1694]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1437), 0.3333333333333)));
        }

        if (s.b[1608] && (!s.b[1694])) {
            s.store_scaled_sub(1344, 1346, 1437, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_mul_ad_lhs(1347, A::add_scaled_product(s.ad_value(1350), 1.0, s.ad_value(1460), s.ad_value(1458), 1.0), 1342);
            s.store_mul_ad_lhs(1348, A::add_scaled_product(s.ad_value(1349), 1.0, s.ad_value(1461), s.ad_value(1459), 1.0), 1343);
        }

        s.b[1695] = (((s.v[1345] - s.v[1347]) * 0.3333333333333) < 80.0);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1695]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1345), 0.3333333333333, s.ad_value(1347), 0.3333333333333)));
        }

        if (s.b[1608] && (!s.b[1695])) {
            s.store_scaled_sub(1344, 1345, 1347, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1349, 1345, 1.0, 1344, 3.0);
        }

        s.b[1696] = (((s.v[1346] - s.v[1348]) * 0.3333333333333) < 80.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1696]) {
            s.store_ad_value(1344, A::ln_one_plus_exp(A::sub_scaled_inputs(s.ad_value(1346), 0.3333333333333, s.ad_value(1348), 0.3333333333333)));
        }

        if (s.b[1608] && (!s.b[1696])) {
            s.store_scaled_sub(1344, 1346, 1348, 0.3333333333333);
        }

        if s.b[1608] {
            s.store_sub_scaled_inputs(1350, 1346, 1.0, 1344, 3.0);
            s.store_sub(1529, 1458, 1349);
            s.store_sub(1530, 1459, 1350);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1359, 0.0);
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1697] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1697]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1697])) {
            let assign36120_ad_e40542: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1342, assign36120_ad_e40542, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1698] = (s.v[1353] < (-0.005));
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1698]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1699] = (s.v[1353] > 0.005);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1698])) && s.b[1699]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1698])) && (!s.b[1699])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1700] = (s.v[1353] > 0.005);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1700]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1701] = (s.v[1353] < (-0.005));
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1700])) && s.b[1701]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1700])) && (!s.b[1701])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1702] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1702]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1702])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1703] = (s.v[1365] > 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1703]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (s.b[1608] && (!s.b[1703])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1529), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1529, 1529, 1380);
            s.store_mul(1351, 1460, 1529);
            s.store_mul(1381, 1461, 1530);
            s.store_add(1374, 1351, 1381);
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_ad_value(1383, A::add_scaled_product(A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0));
            s.store_ad_value(1384, A::add_scaled_product(s.ad_value(1374), (2.0 * 39.478417604), s.ad_value(1351), s.ad_value(1381), 39.478417604));
            s.store_sqrt_ad(1385, A::add_scaled_square_product(s.ad_value(1383), 1.0, s.ad_value(1382), s.ad_value(1384), (-4.0)));
            s.store_ad_value(1353, A::div_scaled_inputs(A::sub(s.ad_value(1385), s.ad_value(1383)), 1.0, s.ad_value(1382), 2.0));
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        s.b[1704] = (s.v[1386] > 0.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1704]) {
            s.store_mul_add_ad_rhs(1377, 1386, A::add_scaled_inputs3(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0), s.ad_value(1529));
            s.store_ad_value(1378, A::add_scaled_product(s.ad_value(1386), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1387, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1345), -1.0));
        }

        s.b[1705] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1704]) && s.b[1705]) {
            s.store_sub_ad_rhs(1529, 1529, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1529);
            s.store_mul(1381, 1461, 1530);
            s.store_add(1374, 1351, 1381);
            s.store_offset_scaled(1382, 1374, 0.065345483024, 1.0);
            s.store_ad_value(1383, A::add_scaled_product(A::scale_offset(s.ad_value(1374), 8.5797362674, 39.478417604), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0));
            s.store_ad_value(1384, A::add_scaled_product(s.ad_value(1374), (2.0 * 39.478417604), s.ad_value(1351), s.ad_value(1381), 39.478417604));
            s.store_sqrt_ad(1385, A::add_scaled_square_product(s.ad_value(1383), 1.0, s.ad_value(1382), s.ad_value(1384), (-4.0)));
            s.store_ad_value(1353, A::div_scaled_inputs(A::sub(s.ad_value(1385), s.ad_value(1383)), 1.0, s.ad_value(1382), 2.0));
        }

        s.b[1706] = (s.v[1353] < (-0.005));
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1706]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_div_ad_lhs(1358, A::add_scaled_inputs(s.ad_value(1353), 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25), 1353);
        }

        s.b[1707] = (s.v[1353] > 0.005);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1706])) && s.b[1707]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_div_ad_lhs(1358, A::add_scaled_inputs(s.ad_value(1353), 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 0.25), 1353);
        }

        if ((s.b[1608] && (!s.b[1706])) && (!s.b[1707])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_scaled_sub_from_scalar_ad(1358, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
        }

        if s.b[1608] {
            s.store_sub_ad_rhs(1353, 1353, A::div(A::add(A::add_scaled_products(s.ad_value(1374), s.ad_value(1357), 1.0, s.ad_value(1351), s.ad_value(1381), 1.0), s.ad_value(1353)), A::offset(A::mul(s.ad_value(1374), s.ad_value(1358)), 1.0)));
            s.store_sub_ad_lhs(1386, A::square(s.ad_value(1351)), 1353);
        }

        s.b[1708] = (s.v[1386] > 0.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1708]) {
            s.store_mul_add_ad_rhs(1377, 1386, A::add_scaled_inputs3(A::ln(A::div(s.ad_value(1386), s.ad_value(1433))), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0), s.ad_value(1529));
            s.store_ad_value(1378, A::add_scaled_product(s.ad_value(1386), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1387, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1345), -1.0));
        }

        s.b[1709] = ((((s.v[1377] < 0.0) && (s.v[1378] > 0.0)) && (((s.v[1387] + 2.3025850929941) + ((s.v[1460]) as f64).ln()) > 0.0)) || (s.v[1387] > 1.0));
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1708]) && s.b[1709]) {
            s.store_sub_ad_rhs(1529, 1529, A::div(s.ad_value(1377), s.ad_value(1378)));
        }

        if s.b[1608] {
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1710] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1710]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1710])) {
            let assign37290_ad_e41975: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1342, assign37290_ad_e41975, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1711] = (s.v[1353] < (-0.005));
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1711]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1712] = (s.v[1353] > 0.005);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1711])) && s.b[1712]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1711])) && (!s.b[1712])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1713] = (s.v[1353] > 0.005);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1713]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1714] = (s.v[1353] < (-0.005));
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1713])) && s.b[1714]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1713])) && (!s.b[1714])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1715] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1715]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1715])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1716] = (s.v[1365] > 0.0);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1716]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (s.b[1608] && (!s.b[1716])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1529), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1529, 1529, 1380);
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1717] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1717]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1717])) {
            let assign38040_ad_e42924: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1342, assign38040_ad_e42924, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1718] = (s.v[1353] < (-0.005));
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1718]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1719] = (s.v[1353] > 0.005);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1718])) && s.b[1719]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1608] && (!s.b[1718])) && s.b[1719]) {
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((s.b[1608] && (!s.b[1718])) && (!s.b[1719])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1720] = (s.v[1353] > 0.005);
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1720]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1721] = (s.v[1353] < (-0.005));
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1720])) && s.b[1721]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((s.b[1608] && (!s.b[1720])) && (!s.b[1721])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1722] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1722]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (s.b[1608] && (!s.b[1722])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1723] = (s.v[1365] > 0.0);
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1723]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (s.b[1608] && (!s.b[1723])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if s.b[1608] {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1529), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1529, 1529, 1380);
        }

        s.b[1724] = (p.p10 == 1.0);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        s.b[1725] = (((s.v[1380]) as f64).abs() > 0.01);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {
            s.store_mul(1351, 1460, 1529);
        }

        s.b[1726] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1726]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1726])) {
            let assign38810_ad_e43892: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1342, assign38810_ad_e43892, 1.0, 5.54062e34);
        }

        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {
            s.store_mul(1352, 1433, 1342);
            s.store_sub_ad_lhs(1353, A::square(s.ad_value(1351)), 1352);
            s.store_ad_value(1354, A::add_scaled_product(s.ad_value(1352), 1.0, s.ad_value(1460), s.ad_value(1351), 2.0));
            s.store_ad_value(1355, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1460), s.ad_value(1460), 2.0));
        }

        s.b[1727] = (s.v[1353] < (-0.005));
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1727]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        s.b[1728] = (s.v[1353] > 0.005);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && s.b[1728]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1353));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
            s.store_scaled_div(1342, 1354, 1353, 0.25);
            s.store_mul_add_ad_lhs(1358, s.ad_value(1353), A::mul_sub_from_scalar_rhs(s.ad_value(1357), 2.0, s.ad_value(1357)), 1342);
            s.store_ad_value(1360, A::add_scaled_product(A::div_scaled_product(s.ad_value(1358), s.ad_value(1355), 1.0, s.ad_value(1354), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1354), 1.0, s.ad_value(1358), s.ad_value(1357), 1.0, (-2.0)), s.ad_value(1342), 1.0));
            s.store_sub_from_scalar_ad(1343, 1.0, A::scale(s.ad_value(1357), 0.5));
            s.store_mul_div_lhs(1363, 1354, 1353, 1343);
            s.store_div_ad_lhs(1364, A::add_scaled_products(s.ad_value(1355), s.ad_value(1343), 1.0, s.ad_value(1354), A::add_scaled_inputs(s.ad_value(1363), 1.0, s.ad_value(1358), 0.5), (-1.0)), 1353);
        }

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1727])) && (!s.b[1728])) {
            s.store_scaled_sub_from_scalar_ad(1344, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.025), 0.0238095238095), 0.0166666666667), 0.1666666666667);
            s.store_offset_mul(1357, 1353, 1344, 2.0);
            s.store_scaled_sub_from_scalar_ad(1342, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_mul(1358, 1354, 1342);
            s.store_scaled_sub_from_scalar_ad(1343, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0420875420875421), 0.05), 0.0714285714286), 0.0055555555556);
            s.store_ad_value(1360, A::add_scaled_products(s.ad_value(1355), s.ad_value(1342), 1.0, A::square(s.ad_value(1354)), s.ad_value(1343), (-1.0)));
            s.store_scaled_mul(1363, 1354, 1344, (-0.5));
            s.store_ad_value(1364, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(1354), s.ad_value(1354), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 2.0, A::scale(s.ad_value(1353), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, s.ad_value(1355), s.ad_value(1344), (-0.5)));
        }

        s.b[1729] = (s.v[1353] > 0.005);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1729]) {
            s.store_ad_value(1343, A::div_scaled_inputs(s.ad_value(1353), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
            s.store_mul(1361, 1343, 1359);
            s.store_sub_ad_lhs(1362, A::ln(s.ad_value(1343)), 1356);
        }

        s.b[1730] = (s.v[1353] < (-0.005));
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1729])) && s.b[1730]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_ad_value(1361, A::div_scaled_inputs(s.ad_value(1353), -1.0, A::square(s.ad_value(1343)), 1.0));
            s.store_ln(1362, 1361);
        }

        if ((((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1729])) && (!s.b[1730])) {
            s.store_sub_from_scalar_ad(1361, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1353), 1.0, A::scale(s.ad_value(1353), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(1362, 1361);
        }

        s.b[1731] = (((1.01 * s.v[1351]) + s.v[1357]) > 0.0);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1731]) {
            s.store_add(1365, 1351, 1357);
            s.store_add(1366, 1460, 1358);
            s.copy_ad(1367, 1360);
        }

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1731])) {
            s.store_div_from_scalar_sub_ad(1343, 1.0, s.ad_value(1351), s.ad_value(1357));
            s.store_sub(1344, 1358, 1460);
            s.store_mul_sub_lhs(1365, 1352, 1361, 1343);
            s.store_mul_ad_lhs(1366, A::add_scaled_product(A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1344), s.ad_value(1365), 1.0), 1.0, s.ad_value(1363), s.ad_value(1361), (-1.0)), 1343);
            s.store_mul_ad_lhs(1367, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1360), s.ad_value(1365), 1.0, s.ad_value(1344), s.ad_value(1366), 2.0), 1.0, s.ad_value(1352), 1.0, A::add(s.ad_value(1364), A::square(s.ad_value(1363))), s.ad_value(1361), (-1.0)), 1343);
        }

        s.b[1732] = (s.v[1365] > 0.0);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && s.b[1732]) {
            s.store_ln(1368, 1365);
            s.store_div_from_scalar(1342, 1.0, 1365);
            s.store_mul(1369, 1366, 1342);
            s.store_ad_value(1370, A::add_scaled_square_product(s.ad_value(1369), (-1.0), s.ad_value(1367), s.ad_value(1342), 1.0));
        }

        if (((s.b[1608] && s.b[1724]) && s.b[1725]) && (!s.b[1732])) {
            s.store_add_ad(1368, A::offset(s.ad_value(1351), 0.6931471805599), A::ln_scaled_input(s.ad_value(1351), -1.0));
            s.store_div_from_scalar(1342, 1.0, 1529);
            s.store_add(1369, 1460, 1342);
            s.store_mul_neg_lhs(1370, 1342, 1342);
        }

        if ((s.b[1608] && s.b[1724]) && s.b[1725]) {
            s.store_ad_value(1371, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1529), 1.0), 1.0, s.ad_value(1368), 2.0, s.ad_value(1362), -1.0));
            s.store_sub_ad_lhs(1372, A::scale_offset(s.ad_value(1369), 2.0, 1.0), 1363);
            s.store_sub_scaled_inputs(1373, 1370, 2.0, 1364, 1.0);
            s.store_ad_value(1374, A::add_scaled_product(s.ad_value(1351), 1.0, s.ad_value(1461), s.ad_value(1371), 1.0));
            s.store_ad_value(1375, A::add_scaled_product(s.ad_value(1460), 1.0, s.ad_value(1461), s.ad_value(1372), 1.0));
            s.store_mul(1376, 1461, 1373);
            s.store_ad_value(1377, A::add_scaled_product(s.ad_value(1352), (-1.0), s.ad_value(1374), s.ad_value(1365), 1.0));
            s.store_add_ad_lhs(1378, A::add_scaled_products(s.ad_value(1375), s.ad_value(1365), 1.0, s.ad_value(1374), s.ad_value(1366), 1.0), 1352);
            s.store_sub_ad_lhs(1379, A::add_scaled_product(A::add_scaled_products(s.ad_value(1376), s.ad_value(1365), 1.0, s.ad_value(1375), s.ad_value(1366), 2.0), 1.0, s.ad_value(1374), s.ad_value(1367), 1.0), 1352);
            s.store_ad_value(1388, A::add_scaled_square_product(s.ad_value(1378), 1.0, s.ad_value(1377), s.ad_value(1379), (-0.5)));
            s.store_ad_value(1380, A::div_scaled_product3(s.ad_value(1377), s.ad_value(1378), s.ad_value(1388), -1.0, A::offset(A::square(s.ad_value(1388)), 1e-200), 1.0));
            s.store_add(1529, 1529, 1380);
        }

        if s.b[1608] {
            s.store_mul(1532, 1460, 1529);
        }

        s.b[1733] = (((s.v[1458] - s.v[1529]) - s.v[1528]) < 80.0);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1608] && s.b[1733]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1733])) {
            let assign39560_ad_e45101: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1458), 1.0, s.ad_value(1529), (-1.0), s.ad_value(1528), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1342, assign39560_ad_e45101, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1535, 1433, 1342);
            s.store_sub_ad_lhs(1534, A::square(s.ad_value(1532)), 1535);
        }

        s.b[1734] = (s.v[1535] <= 0.0);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1734]) {
            s.store_scalar(1531, 1e-80);
            s.store_sub(1533, 1531, 1532);
            s.store_div(1530, 1533, 1461);
        }

        s.b[1735] = (s.v[1534] < (-0.005));
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1734])) && s.b[1735]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1534));
            s.store_div_ad_rhs(1357, 1356, A::tan(A::scale(s.ad_value(1356), 0.5)));
        }

        s.b[1736] = (s.v[1534] > 0.005);
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1735])) && s.b[1736]) {
            s.store_sqrt_abs_ad(1356, s.ad_value(1534));
            s.store_exp_neg_input(1359, 1356);
            s.store_ad_value(1357, A::div_scaled_product_offset_rhs(s.ad_value(1356), s.ad_value(1359), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1359)), 1.0));
        }

        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1736])) {
            s.store_offset_ad(1357, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1737] = (((1.01 * s.v[1532]) + s.v[1357]) > 0.0);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1734])) && s.b[1737]) {
            s.store_add(1342, 1532, 1357);
        }

        s.b[1738] = ((s.v[1535] * s.v[1532]) < (((0.9 * s.v[1532]) * s.v[1532]) * s.v[1342]));
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if (((s.b[1608] && (!s.b[1734])) && s.b[1737]) && s.b[1738]) {
            s.store_offset_div(1531, 1535, 1342, 1e-80);
            s.store_sub(1533, 1531, 1532);
            s.store_div(1530, 1533, 1461);
        }

        s.b[1739] = (s.v[1534] > 0.005);
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && s.b[1739]) {
            s.store_sub_ad_lhs(1343, A::ln(A::div_scaled_inputs(s.ad_value(1534), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0)), 1356);
        }

        s.b[1740] = (s.v[1534] < (-0.005));
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if (((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && (!s.b[1739])) && s.b[1740]) {
            s.store_sin_scaled_input(1344, 1356, 0.5);
            s.store_ln_ad(1343, A::div_scaled_inputs(s.ad_value(1534), -1.0, A::square(s.ad_value(1344)), 1.0));
        }

        if (((((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) && (!s.b[1739])) && (!s.b[1740])) {
            s.store_ln_ad(1343, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((s.b[1608] && (!s.b[1734])) && s.b[1737]) && (!s.b[1738])) {
            s.store_ad_value(1530, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1458), (-1.0), s.ad_value(1529), 1.0), 1.0, A::ln(s.ad_value(1342)), 2.0, s.ad_value(1343), -1.0));
            s.store_mul(1533, 1461, 1530);
            s.store_add(1531, 1532, 1533);
        }

        s.b[1741] = (s.v[1534] > 0.005);
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        s.b[1742] = ((((s.v[1529] + s.v[1528]) - s.v[1458]) - s.v[1356]) < 80.0);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) && s.b[1742]) {
            s.store_exp_ad(1344, A::sub(A::add_scaled_inputs3(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0), s.ad_value(1356)));
        }

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) && (!s.b[1742])) {
            let assign39890_ad_e45570: A = A::mul_offset_lhs(A::sub(A::add_scaled_inputs3(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0), s.ad_value(1356)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(A::add_scaled_inputs3(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0), s.ad_value(1356)), (-80.0)), 0.5, A::scale_offset(A::sub(A::add_scaled_inputs3(s.ad_value(1529), 1.0, s.ad_value(1528), 1.0, s.ad_value(1458), -1.0), s.ad_value(1356)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1344, assign39890_ad_e45570, 1.0, 5.54062e34);
        }

        if (((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && s.b[1741]) {
            s.store_div(1343, 1344, 1433);
            s.store_ad_value(1342, A::div_scaled_product(s.ad_value(1534), s.ad_value(1343), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1359), 2.0, s.ad_value(1359))), 1.0));
        }

        s.b[1743] = (s.v[1534] < (-0.005));
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && (!s.b[1741])) && s.b[1743]) {
            s.store_sin_scaled_input(1343, 1356, 0.5);
            s.store_div_ad_lhs(1342, A::div_scaled_inputs(s.ad_value(1534), -1.0, A::square(s.ad_value(1343)), 1.0), 1535);
        }

        if ((((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) && (!s.b[1741])) && (!s.b[1743])) {
            s.store_div_ad_lhs(1342, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0396825396825397), 0.05), 0.3333333333333)), 1535);
        }

        if ((s.b[1608] && (!s.b[1734])) && (!s.b[1737])) {
            s.store_offset_div_ad(1531, A::sub(s.ad_value(1532), s.ad_value(1357)), A::sub_from_scalar(1.0, s.ad_value(1342)), 1e-80);
            s.store_sub(1533, 1531, 1532);
            s.store_div(1530, 1533, 1461);
        }

        s.b[1744] = (((s.v[1459] - s.v[1530]) - s.v[1528]) < 80.0);
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1744]) {
            s.store_exp_ad(1342, A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0));
        }

        if (s.b[1608] && (!s.b[1744])) {
            let assign40010_ad_e45787: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1459), 1.0, s.ad_value(1530), (-1.0), s.ad_value(1528), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1342, assign40010_ad_e45787, 1.0, 5.54062e34);
        }

        if s.b[1608] {
            s.store_mul(1536, 1433, 1342);
            s.store_scalar(1539, 0.0);
            s.store_scalar(1540, 0.0);
            s.store_scalar(1537, 0.0);
            s.store_scalar(1538, 0.0);
            s.store_scalar(1541, 0.0);
            s.store_scalar(1542, 0.0);
        }

        s.b[1745] = (s.v[1466] > 1e-6);
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1745]) {
            s.store_mul(1537, 1535, 1434);
            s.store_mul(1538, 1536, 1435);
            s.store_add_scaled_inputs(1539, 1537, 1.0, 1532, 2.0);
            s.store_add_scaled_inputs(1540, 1538, 1.0, 1533, 2.0);
            s.store_ad_value(1541, A::add_scaled_inputs3(s.ad_value(1531), 2.0, s.ad_value(1537), 1.0, s.ad_value(1538), 1.0));
        }

        s.b[1746] = (((s.v[1534]) as f64).abs() > 0.005);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1745]) && s.b[1746]) {
            s.store_ad_value(2, A::add_scaled_offset_product_lhs(A::add_scaled_products(s.ad_value(1539), s.ad_value(1540), 1.0, A::offset(s.ad_value(1529), 2.0), s.ad_value(1540), 2.0), 1.0, s.ad_value(1530), 2.0, s.ad_value(1539), 2.0));
            s.store_ad_value(1542, A::div_scaled_product(s.ad_value(1534), s.ad_value(1541), (-4.0), A::mul(s.ad_value(1531), s.ad_value(2)), 1.0));
        }

        if ((s.b[1608] && s.b[1745]) && (!s.b[1746])) {
            s.store_scaled_sub_from_scalar_ad(2, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1534), 1.0, A::scale(s.ad_value(1534), 0.0333333333333), 0.0357142857143), 0.0333333333333), 0.1666666666667);
            s.store_ad_value(3, A::add_scaled_offset_product_rhs(A::add_scaled_products(s.ad_value(1539), s.ad_value(1535), 1.0, s.ad_value(1540), s.ad_value(1536), 1.0), 1.0, A::mul3(s.ad_value(1539), s.ad_value(1540), s.ad_value(1531)), A::mul(s.ad_value(1531), s.ad_value(2)), 1.0, 1.0));
            s.store_ad_value(1542, A::div_scaled_product3(s.ad_value(1535), s.ad_value(1536), s.ad_value(1541), 1.0, A::mul(s.ad_value(1531), s.ad_value(3)), 1.0));
        }

        if s.b[1608] {
            s.store_add_ad_rhs(1543, 1528, A::ln(s.ad_value(1531)));
            s.store_scaled_add(1544, 1466, 1531, 0.5);
            s.store_sub(1545, 1543, 1479);
            s.store_scalar(1548, 1.0);
        }

        s.b[1747] = (p.p9 > 0.0);
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1747]) {
            s.store_div_ad_lhs(1546, A::add_scaled_inputs(s.ad_value(1467), 0.5, s.ad_value(1532), 0.5), 1460);
            s.store_scaled_add_ad(1546, A::offset(s.ad_value(1546), 1e-5), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1546), (-1e-5), A::offset(s.ad_value(1546), (-1e-5))), 1.0)), 0.5);
            s.store_sub_scaled_ad_lhs(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(1546), s.ad_value(227)), 1.0, s.ad_value(250), s.ad_value(250), 0.25)), 250, 0.5);
            s.store_mul_powf_ad_lhs(1547, s.ad_value(1), 2.0, 227);
            s.store_sub_from_scalar_ad(1548, 1.0, A::div(s.ad_value(1547), s.ad_value(1546)));
        }

        s.b[1748] = ((s.v[1532] / 2.0) < 80.0);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1748]) {
            s.store_ln_one_plus_exp_scaled_input(2, 1532, 0.5);
        }

        if (s.b[1608] && (!s.b[1748])) {
            s.store_scale(2, 1532, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1549, 2, 2.0);
        }

        s.b[1749] = ((s.v[1533] / 2.0) < 80.0);
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1749]) {
            s.store_ln_one_plus_exp_scaled_input(3, 1533, 0.5);
        }

        if (s.b[1608] && (!s.b[1749])) {
            s.store_scale(3, 1533, 0.5);
        }

        if s.b[1608] {
            s.store_scale(1550, 3, 2.0);
            s.store_sub(1551, 1550, 1533);
            s.store_sub(1552, 1549, 1532);
            s.store_ad_value(1553, A::add_scaled_products(s.ad_value(270), s.ad_value(1549), 1.0, s.ad_value(271), s.ad_value(1551), 1.0));
            s.store_ad_value(1554, A::add_scaled_products(s.ad_value(270), s.ad_value(1550), 1.0, s.ad_value(271), s.ad_value(1552), 1.0));
            s.store_scaled_add(1555, 1480, 1549, 0.5);
            s.store_scaled_add(1556, 1481, 1550, 0.5);
            s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1555), s.ad_value(1556));
            s.store_mul3_lhs(1557, 1544, 1555, 0);
            s.store_mul3_lhs(1558, 1544, 1556, 0);
            s.store_scaled_add(1559, 1482, 1551, 0.5);
            s.store_scaled_add(1560, 1483, 1552, 0.5);
            s.store_scaled_add(1561, 1484, 1553, 0.5);
            s.store_scaled_add(1562, 1485, 1554, 0.5);
            s.store_mul_ad_lhs(1563, A::mul3(s.ad_value(1555), s.ad_value(191), A::exp(A::mul(s.ad_value(40), s.ad_value(295)))), 1548);
            s.store_mul_ad_product_rhs(1564, 1556, s.ad_value(192), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));
            s.store_add(1565, 1563, 1564);
            s.store_mul_ad_rhs(2, 50, A::add_scaled_product(s.ad_value(1559), 1.0, s.ad_value(51), s.ad_value(1560), 1.0));
            s.store_scaled_add_ad(3, A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2), 1.0, A::offset(s.ad_value(2), 1.0)), 0.01)), 0.5);
            s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);
            s.store_div(1566, 3, 4);
            s.store_mul_ad_product_rhs(1567, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1559)), 1.0), 1.0, s.ad_value(42), s.ad_value(1560), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1557), s.ad_value(268)), 1.0), 1.0, s.ad_value(1558), s.ad_value(269), 1.0)))));
        }

        s.b[1750] = (s.v[56] == 0.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1750]) {
            s.store_scalar(4, 1.0);
        }

        s.b[1751] = (s.v[56] < 0.0);
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if ((s.b[1608] && (!s.b[1750])) && s.b[1751]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((s.b[1608] && (!s.b[1750])) && (!s.b[1751])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(1544), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        if s.b[1608] {
            s.store_mul_ad_rhs(1568, 1492, A::add_scaled_product(s.ad_value(54), 1.0, s.ad_value(1544), s.ad_value(4), 1.0));
            s.store_ad_value(1569, A::add_scaled_inputs_product(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1561)), 1e-6)))), 1.0), 1.0, s.ad_value(1567), 1.0, s.ad_value(38), s.ad_value(1568), 1.0));
            s.store_ad_value(1570, A::add_scaled_inputs_product(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1562)), 1e-6)))), 1.0), 1.0, s.ad_value(1567), 1.0, s.ad_value(39), s.ad_value(1568), 1.0));
            s.store_ad_value(1571, A::div_scaled_product(s.ad_value(1566), s.ad_value(1565), 1.0, A::add(A::div(s.ad_value(1563), s.ad_value(1569)), A::div(s.ad_value(1564), s.ad_value(1570))), 1.0));
            s.store_div_from_scalar_offset_input(1572, 1.0, 1544, 4.0);
        }

        s.b[1752] = (s.v[65] > 0.0);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1752]) {
            s.store_div_from_scalar_offset_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1558)), 1.0);
        }

        if (s.b[1608] && (!s.b[1752])) {
            s.store_sub_from_scalar_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1558)));
        }

        if s.b[1608] {
            s.store_mul3_lhs(1573, 1544, 1572, 0);
            s.store_mul_ln_ad_lhs(1574, A::offset(A::div(A::sub(s.ad_value(339), s.ad_value(1528)), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(1544), s.ad_value(1544)), 1.0, s.ad_value(66), s.ad_value(227), 1.0)), 1.0), 1573);
            s.store_mul(1575, 1426, 1574);
            s.store_div_from_scalar_offset_ad(1576, 1.0, A::mul_offset_rhs(s.ad_value(1575), s.ad_value(1575), 1.0), 1.0);
            s.store_ad_value(1504, A::div_scaled_inputs(s.ad_value(1555), 100.0, A::offset(s.ad_value(1555), 100.0), 1.0));
        }

        s.b[1753] = (s.v[61] < 0.0);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1753]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1505, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(1504)));
        }

        if (s.b[1608] && (!s.b[1753])) {
            s.store_offset_mul(1505, 61, 1504, 1.0);
        }

        if s.b[1608] {
            s.store_ad_value(1506, A::div_scaled_inputs(s.ad_value(1556), 100.0, A::offset(s.ad_value(1556), 100.0), 1.0));
        }

        s.b[1754] = (s.v[62] < 0.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1754]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1507, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(1506)));
        }

        if (s.b[1608] && (!s.b[1754])) {
            s.store_offset_mul(1507, 62, 1506, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1608] {
            s.store_mul_ad_affine_product_rhs(1577, 1424, s.ad_value(1545), A::add(s.ad_value(1505), s.ad_value(1507)), 0.5, 0.0);
            s.store_div_ad_rhs(1578, 1577, A::mul(s.ad_value(1571), s.ad_value(1576)));
            s.store_square(1579, 1578);
            s.store_sqrt_offset_input(1580, 1579, 1.0);
            s.store_div_ad_lhs(1581, A::scale_offset(s.ad_value(1579), 1.5, 1.0), 1580);
        }

        s.b[1755] = (p.p13 > 0.0);
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if (s.b[1608] && s.b[1755]) {
            s.store_mul_scaled_ad_rhs(2, 258, 0.6, A::exp_scaled_input(A::ln(A::offset(A::square(s.ad_value(1555)), 60.0)), (-0.1666666666667)));
            s.store_mul_scaled_ad_rhs(3, 258, 0.6, A::exp_scaled_input(A::ln(A::offset(A::square(s.ad_value(1556)), 60.0)), (-0.1666666666667)));
            s.store_div_ad_lhs(1582, A::offset(A::mul(s.ad_value(1460), s.ad_value(2)), 1.0), 1441);
            s.store_div_ad_lhs(1583, A::offset(A::mul(s.ad_value(1461), s.ad_value(3)), 1.0), 1442);
        }

        if (s.b[1608] && (!s.b[1755])) {
            s.store_scalar(1582, 1.0);
            s.store_scalar(1583, 1.0);
        }

        s.b[1756] = (s.v[1466] > 1e-6);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        s.b[1757] = (s.v[1531] > 1e-6);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        s.b[1758] = (((s.v[1540]) as f64).abs() < 0.01);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (((s.b[1608] && s.b[1756]) && s.b[1757]) && s.b[1758]) {
            s.store_div_ad(0, A::add_scaled_inputs(A::offset(s.ad_value(1529), 2.0), 1.0, s.ad_value(1539), 0.5), A::mul_offset_lhs(s.ad_value(1530), 2.0, s.ad_value(1539)));
            s.store_mul(2, 0, 1540);
            s.store_square(3, 2);
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_ad_value(5, A::add_scaled_product(s.ad_value(4), 1.0, s.ad_value(2), s.ad_value(3), (-1.0)));
            s.store_div_ad(2, A::sub(s.ad_value(1533), A::mul3_scaled_output(s.ad_value(1534), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1539))), s.ad_value(5), 2.0)), A::offset(s.ad_value(1530), 2.0));
            s.store_div_ad_lhs(1584, A::sub(A::div(A::add_scaled_product(s.ad_value(1535), (-1.0), s.ad_value(1542), s.ad_value(1531), 1.0), s.ad_value(1539)), s.ad_value(2)), 1531);
            s.store_ad_value(1585, A::div_scaled_product_offset_denominator(s.ad_value(1584), s.ad_value(1531), 1.0, s.ad_value(1584), 1.0, 1.0));
        }

        if (((s.b[1608] && s.b[1756]) && s.b[1757]) && (!s.b[1758])) {
            s.store_sub_ad(1584, A::div_scaled_product(s.ad_value(1542), s.ad_value(1541), 1.0, A::mul(s.ad_value(1539), s.ad_value(1540)), 1.0), A::div(A::add(A::div(s.ad_value(1535), s.ad_value(1539)), A::div(s.ad_value(1536), s.ad_value(1540))), s.ad_value(1531)));
            s.store_ad_value(1585, A::div_scaled_product_offset_denominator(s.ad_value(1584), s.ad_value(1531), 1.0, s.ad_value(1584), 1.0, 1.0));
        }

        if ((s.b[1608] && s.b[1756]) && (!s.b[1757])) {
            s.copy_ad(1585, 1502);
        }

        if (s.b[1608] && s.b[1756]) {
            s.store_sub(2, 1585, 1509);
            s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);
        }

        s.b[1759] = (((s.v[2]) as f64).abs() > 0.001);
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1756]) && s.b[1759]) {
            s.store_sub(4, 1531, 1466);
            s.store_ad_value(1586, A::add_scaled_product(s.ad_value(4), 1.0, s.ad_value(1585), s.ad_value(1545), (-1.0)));
            s.store_ad_value(1587, A::add_scaled_product(s.ad_value(4), 1.0, s.ad_value(1509), s.ad_value(1545), (-1.0)));
            s.store_sqrt_square_add(1588, 1586, 3);
            s.store_sqrt_square_add(1589, 1587, 3);
            s.store_mul_ad(1590, A::div_from_scalar(0.25, s.ad_value(2)), A::add_scaled_product(A::add_scaled_products(s.ad_value(1589), s.ad_value(1586), 1.0, s.ad_value(1588), s.ad_value(1587), (-1.0)), 1.0, s.ad_value(3), A::ln(A::div(A::add(s.ad_value(1587), s.ad_value(1589)), A::add(s.ad_value(1586), s.ad_value(1588)))), 1.0));
        }

        if ((s.b[1608] && s.b[1756]) && (!s.b[1759])) {
            s.store_mul(4, 1545, 2);
            s.store_ad_value(1590, A::div_scaled_product3(s.ad_value(1545), s.ad_value(4), s.ad_value(4), ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0));
        }

        if (s.b[1608] && (!s.b[1756])) {
            s.copy_ad(1585, 1502);
            s.store_scalar(1590, 0.0);
        }

        if s.b[1608] {
            s.store_ad_value(1591, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1590), 1.0, s.ad_value(1544), s.ad_value(1545), 1.0), 1.0, s.ad_value(1466), 1.0, s.ad_value(1531), -1.0));
        }

        s.b[1760] = (s.v[1466] > 1e-6);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        s.b[1761] = (s.v[1591] > 1e-30);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((s.b[1608] && s.b[1760]) && s.b[1761]) {
            s.store_div_ad_rhs(1592, 1475, A::sub(A::div(s.ad_value(1471), s.ad_value(1466)), s.ad_value(1478)));
            s.store_div_ad_rhs(1593, 1539, A::sub(A::div(s.ad_value(1535), s.ad_value(1531)), s.ad_value(1542)));
            s.store_div_ad_lhs(1594, A::sub(s.ad_value(1592), s.ad_value(1593)), 1591);
            s.store_div_ad_rhs(1595, 1476, A::sub(A::div(s.ad_value(1472), s.ad_value(1466)), s.ad_value(1478)));
            s.store_div_ad_rhs(1596, 1540, A::sub(A::div(s.ad_value(1536), s.ad_value(1531)), s.ad_value(1542)));
            s.store_div_ad_lhs(1597, A::sub(s.ad_value(1595), s.ad_value(1596)), 1591);
        }

        if ((s.b[1608] && s.b[1760]) && (!s.b[1761])) {
            s.store_scalar(1594, 0.0);
            s.store_scalar(1597, 0.0);
        }

        if (s.b[1608] && (!s.b[1760])) {
            s.store_mul_scaled_ad_rhs(1598, 1497, (-2.0), A::add(A::div(s.ad_value(1434), s.ad_value(1500)), s.ad_value(1503)));
            s.store_mul_scaled_ad_rhs(1599, 1498, (-2.0), A::add(A::div(s.ad_value(1435), s.ad_value(1501)), s.ad_value(1503)));
            s.store_mul_sub_lhs(0, 1599, 1598, 1503);
            s.store_mul(2, 1598, 1434);
            s.store_mul(3, 1599, 1435);
            s.store_add(4, 2, 3);
            s.store_offset_ad(5, A::add_scaled_products(s.ad_value(1497), s.ad_value(1434), 2.0, s.ad_value(1498), s.ad_value(1435), 2.0), 3.0);
            s.store_div_ad_lhs(1600, A::add_scaled_inputs3(s.ad_value(3), 1.0, s.ad_value(0), 1.0, A::div(s.ad_value(4), s.ad_value(1500)), -1.0), 5);
            s.store_div_ad_lhs(1601, A::add_scaled_inputs3(s.ad_value(2), 1.0, s.ad_value(0), (-1.0), A::div(s.ad_value(4), s.ad_value(1501)), -1.0), 5);
            s.store_mul_scaled_ad_rhs(1594, 1500, -1.0, A::add_scaled_product(s.ad_value(1503), 1.0, s.ad_value(1600), s.ad_value(1500), 1.0));
            s.store_mul_scaled_ad_rhs(1597, 1501, -1.0, A::add_scaled_product(s.ad_value(1503), 1.0, s.ad_value(1601), s.ad_value(1501), 1.0));
        }

        if s.b[1608] {
            s.store_mul(1602, 1594, 1581);
            s.store_mul(1603, 1597, 1581);
            s.store_scaled_sub(1604, 1532, 1467, 0.5);
            s.store_scaled_sub(1605, 1533, 1468, 0.5);
            s.store_mul(1606, 1604, 1602);
            s.store_mul(1607, 1605, 1603);
            s.copy_ad(440, 1428);
            s.copy_ad(441, 1432);
            s.copy_ad(442, 1433);
            s.copy_ad(443, 1434);
            s.copy_ad(444, 1435);
            s.copy_ad(445, 1462);
            s.copy_ad(446, 1463);
            s.copy_ad(447, 1447);
            s.copy_ad(448, 1446);
            s.copy_ad(449, 1450);
            s.copy_ad(450, 1451);
            s.copy_ad(451, 1452);
            s.copy_ad(452, 1453);
            s.copy_ad(453, 1454);
            s.copy_ad(454, 1457);
            s.copy_ad(455, 1459);
            s.copy_ad(456, 1460);
            s.copy_ad(457, 1461);
            s.copy_ad(458, 1467);
            s.copy_ad(459, 1468);
            s.copy_ad(460, 1479);
            s.copy_ad(461, 1532);
            s.copy_ad(462, 1533);
            s.copy_ad(463, 1543);
            s.copy_ad(464, 1544);
            s.copy_ad(465, 1548);
            s.copy_ad(466, 1557);
            s.copy_ad(467, 1558);
            s.copy_ad(468, 1579);
            s.copy_ad(469, 1582);
            s.copy_ad(470, 1583);
            s.copy_ad(471, 1604);
            s.copy_ad(472, 1605);
            s.copy_ad(473, 1606);
            s.copy_ad(474, 1607);
        }

        if (!s.b[1608]) {
            s.copy_ad(440, 383);
            s.copy_ad(441, 384);
            s.copy_ad(442, 385);
            s.copy_ad(443, 386);
            s.copy_ad(444, 387);
            s.copy_ad(445, 388);
            s.copy_ad(446, 389);
            s.copy_ad(447, 390);
            s.copy_ad(448, 391);
            s.copy_ad(449, 393);
            s.copy_ad(450, 394);
            s.copy_ad(451, 395);
            s.copy_ad(452, 396);
            s.copy_ad(453, 397);
            s.copy_ad(454, 398);
            s.copy_ad(455, 399);
            s.copy_ad(456, 401);
            s.copy_ad(457, 402);
            s.copy_ad(458, 404);
            s.copy_ad(459, 405);
            s.copy_ad(460, 406);
            s.copy_ad(461, 408);
            s.copy_ad(462, 409);
            s.copy_ad(463, 414);
            s.copy_ad(464, 415);
            s.copy_ad(465, 416);
            s.copy_ad(466, 419);
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1608]) {
            s.copy_ad(467, 420);
            s.copy_ad(468, 428);
            s.copy_ad(469, 430);
            s.copy_ad(470, 431);
            s.copy_ad(471, 436);
            s.copy_ad(472, 437);
            s.copy_ad(473, 438);
            s.copy_ad(474, 439);
        }

        s.store_ad_value(0, A::div_scaled_product(s.ad_value(120), A::sub(s.ad_value(448), s.ad_value(446)), 1.0, A::scale_offset(s.ad_value(464), 0.25, 1.0), 1.0));

        s.store_ad_value(1324, A::add_scaled_inputs3(s.ad_value(458), 0.5, s.ad_value(461), 0.5, s.ad_value(0), 1.0));

        s.store_ad_value(1325, A::add_scaled_inputs3(s.ad_value(459), 0.5, s.ad_value(462), 0.5, s.ad_value(0), -1.0));

        s.b[1762] = (p.p13 > 0.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if s.b[1762] {
            s.store_ad_value(1326, A::add_scaled_inputs3(s.ad_value(1324), 1.0, A::div(s.ad_value(466), s.ad_value(469)), 1.0, s.ad_value(466), -1.0));
            s.store_ad_value(1327, A::add_scaled_inputs3(s.ad_value(1325), 1.0, A::div(s.ad_value(467), s.ad_value(470)), 1.0, s.ad_value(467), -1.0));
        }

        if (!s.b[1762]) {
            s.copy_ad(1326, 1324);
            s.copy_ad(1327, 1325);
        }

        s.store_scaled_mul(2, 471, 473, 0.3333333333333);

        s.store_mul_scaled_ad_rhs(3, 471, 0.1666666666667, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(473), 1.0, A::scale(s.ad_value(473), 0.2)), 1.0));

        s.store_ad_value(1328, A::add_scaled_product(s.ad_value(3), 1.0, s.ad_value(1326), s.ad_value(465), 0.5));

        s.store_ad_value(1326, A::add_scaled_product(s.ad_value(2), 1.0, s.ad_value(1326), s.ad_value(465), 1.0));

        s.store_scaled_mul(2, 472, 474, 0.3333333333333);

        s.store_mul_scaled_ad_rhs(3, 472, 0.1666666666667, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(474), 1.0, A::scale(s.ad_value(474), 0.2)), 1.0));

        s.store_add_scaled_inputs(1329, 1327, 0.5, 3, 1.0);

        s.store_add(1327, 1327, 2);

        s.store_mul(0, 447, 287);

        s.store_mul(361, 0, 1326);

        s.store_mul(362, 0, 1327);

        s.store_mul_scaled_ad_rhs(363, 0, -1.0, A::add(s.ad_value(1328), s.ad_value(1329)));

        s.b[1763] = (s.v[119] > 0.0);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if s.b[1763] {
            s.store_offset(0, 254, (2.0 * 0.6931471805599));
            s.store_add(1330, 460, 0);
            s.store_add(1331, 463, 0);
            s.store_ad_value(1332, A::add_scaled_inputs3(s.ad_value(1330), 0.5, s.ad_value(254), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1330), s.ad_value(254)), A::sub(s.ad_value(1330), s.ad_value(254))), 9.0)), (-0.5)));
            s.store_scaled_sub_ad(1333, A::add_scaled_inputs3(s.ad_value(1331), 1.0, s.ad_value(254), 1.0, s.ad_value(339), 1.0), A::sqrt(A::offset(A::mul(A::add_scaled_inputs3(s.ad_value(1331), 1.0, s.ad_value(254), -1.0, s.ad_value(339), -1.0), A::add_scaled_inputs3(s.ad_value(1331), 1.0, s.ad_value(254), -1.0, s.ad_value(339), -1.0)), 9.0)), 0.5);
            s.store_mul_sqrt_ad_rhs(1334, 294, A::mul_offset_rhs(s.ad_value(445), s.ad_value(444), 0.5));
            s.store_mul_sqrt_ad_rhs(1335, 294, A::mul_offset_rhs(A::mul3(s.ad_value(445), s.ad_value(456), s.ad_value(444)), s.ad_value(443), 0.5));
            s.store_mul_square_lhs(1336, 1334, 291);
            s.store_mul_square_lhs(1337, 1335, 291);
            s.store_sub(2, 292, 1332);
            s.store_ad_value(3, A::add_scaled_inputs3(s.ad_value(292), 1.0, s.ad_value(339), 1.0, s.ad_value(1333), -1.0));
            s.store_scale(0, 1336, 2.0);
            s.store_ad_value(1338, A::add_scaled_offset_product_rhs(s.ad_value(1332), 1.0, s.ad_value(0), A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1336)), 1.0)), (-1.0), 1.0));
            s.store_ad_value(1339, A::add_scaled_offset_product_rhs(s.ad_value(1333), 1.0, s.ad_value(0), A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1336)), 1.0)), (-1.0), 1.0));
            s.store_scale(0, 1337, 2.0);
            s.store_ad_value(1340, A::add_scaled_offset_product_rhs(s.ad_value(1332), 1.0, s.ad_value(0), A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1337)), 1.0)), (-1.0), 1.0));
            s.store_ad_value(1341, A::add_scaled_offset_product_rhs(s.ad_value(1333), 1.0, s.ad_value(0), A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1337)), 1.0)), (-1.0), 1.0));
            s.store_mul(0, 293, 447);
            s.store_mul_ad_lhs(2, A::mul3_scaled_output(s.ad_value(0), s.ad_value(1334), s.ad_value(456), -1.0), 451);
            s.store_mul_ad_lhs(3, A::mul3_scaled_output(s.ad_value(0), s.ad_value(1335), s.ad_value(457), -1.0), 452);
            s.store_ad_value(0, A::add_scaled_inputs3(s.ad_value(1338), 0.5, s.ad_value(1330), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1338), s.ad_value(1330)), A::sub(s.ad_value(1338), s.ad_value(1330))), 1.0)), 0.5));
            s.store_ad_value(379, A::div_scaled_product3(s.ad_value(2), s.ad_value(0), s.ad_value(0), 1.0, A::sub(s.ad_value(1338), s.ad_value(1332)), 1.0));
            s.store_ad_value(0, A::add_scaled_inputs3(s.ad_value(1339), 0.5, s.ad_value(1331), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1339), s.ad_value(1331)), A::sub(s.ad_value(1339), s.ad_value(1331))), 1.0)), 0.5));
            s.store_ad_value(380, A::div_scaled_product3(s.ad_value(2), s.ad_value(0), s.ad_value(0), 1.0, A::sub(s.ad_value(1339), s.ad_value(1333)), 1.0));
            s.store_ad_value(0, A::add_scaled_inputs3(s.ad_value(1340), 0.5, s.ad_value(1330), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1340), s.ad_value(1330)), A::sub(s.ad_value(1340), s.ad_value(1330))), 1.0)), 0.5));
            s.store_ad_value(381, A::div_scaled_product3(s.ad_value(3), s.ad_value(0), s.ad_value(0), 1.0, A::sub(s.ad_value(1340), s.ad_value(1332)), 1.0));
            s.store_ad_value(0, A::add_scaled_inputs3(s.ad_value(1341), 0.5, s.ad_value(1331), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1341), s.ad_value(1331)), A::sub(s.ad_value(1341), s.ad_value(1331))), 1.0)), 0.5));
            s.store_ad_value(382, A::div_scaled_product3(s.ad_value(3), s.ad_value(0), s.ad_value(0), 1.0, A::sub(s.ad_value(1341), s.ad_value(1333)), 1.0));
        }

        if (!s.b[1763]) {
            s.store_scalar(379, 0.0);
            s.store_scalar(380, 0.0);
            s.store_scalar(381, 0.0);
            s.store_scalar(382, 0.0);
        }

        s.store_mul(370, 164, 330);

        s.store_mul(371, 165, 332);

        let assign42790_ad_e48345: A = A::add(A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(449), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440))))), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(1.0, A::mul3(s.ad_value(161), s.ad_value(449), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440)))), A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(449), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(440)))))), 0.2)));
        s.store_scale_ad(0, assign42790_ad_e48345, 0.5);

        s.store_mul3_lhs(372, 159, 349, 0);

        s.store_mul3_lhs(373, 160, 350, 0);

        s.store_mul(374, 117, 338);

        s.store_mul(375, 166, 336);

        s.store_mul_neg_ad_lhs(377, A::add_scaled_products(s.ad_value(240), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), 331);

        s.store_mul_neg_ad_lhs(376, A::add_scaled_products(s.ad_value(240), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), 333);

        s.b[1764] = (s.v[6] > 0.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if s.b[1764] {
            s.store_mul(378, 170, 219);
        }

        if (!s.b[1764]) {
            s.store_scalar(378, 0.0);
        }

        s.store_mul_scaled_ad_rhs(365, 13, p.p31, A::add_scaled_inputs3(s.ad_value(348), 1.0, s.ad_value(356), 1.0, s.ad_value(358), 1.0));

        s.store_scaled_mul(366, 13, 352, p.p31);

        s.store_scaled_mul(367, 13, 353, p.p31);

        s.store_scaled_mul(368, 13, 354, p.p31);

        s.store_scaled_mul(369, 13, 355, p.p31);

        s.store_mul(1765, 13, 359);

        s.store_mul(1766, 13, 360);

        s.b[1767] = (s.v[334] < 0.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        s.b[1768] = (s.v[311] > 0.0);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        s.b[1769] = (s.v[318] > 0.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        s.b[1770] = (s.v[322] > 0.0);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        s.b[1771] = (s.v[326] > 0.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        s.copy_ad(1774, 361);

        s.copy_ad(1775, 362);

        s.copy_ad(1776, 363);

        s.store_neg_ad(364, A::add_scaled_inputs3(s.ad_value(361), 1.0, s.ad_value(362), 1.0, s.ad_value(363), 1.0));

        s.b[1777] = (s.v[334] < 0.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if s.b[1777] {
            s.copy_ad(1776, 364);
        }

        s.store_scaled_mul(361, 13, 361, p.p32);

        s.store_scaled_mul(362, 13, 362, p.p32);

        s.store_scaled_mul(363, 13, 363, p.p32);

        s.store_neg_ad(364, A::add_scaled_inputs3(s.ad_value(361), 1.0, s.ad_value(362), 1.0, s.ad_value(363), 1.0));

        s.store_scaled_mul(379, 13, 379, p.p32);

        s.store_scaled_mul(380, 13, 380, p.p32);

        s.store_scaled_mul(381, 13, 381, p.p32);

        s.store_scaled_mul(382, 13, 382, p.p32);

        s.store_scaled_mul(370, 13, 370, p.p32);

        s.store_scaled_mul(371, 13, 371, p.p32);

        s.store_scaled_mul(372, 13, 372, p.p32);

        s.store_scaled_mul(373, 13, 373, p.p32);

        s.store_scaled_mul(374, 13, 374, p.p32);

        s.store_scaled_mul(377, 13, 377, p.p32);

        s.store_scaled_mul(376, 13, 376, p.p32);

        s.store_scaled_mul(375, 13, 375, p.p32);

        s.store_mul(378, 13, 378);

        s.b[1778] = (s.v[334] < 0.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if s.b[1778] {
            s.copy_ad(1772, 363);
            s.copy_ad(363, 364);
            s.copy_ad(364, 1772);
            s.store_neg(375, 375);
            s.copy_ad(1772, 380);
            s.copy_ad(380, 379);
            s.copy_ad(379, 1772);
            s.copy_ad(1772, 382);
            s.copy_ad(382, 381);
            s.copy_ad(381, 1772);
        }

        s.b[1779] = (s.v[13] > 0.0);
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if s.b[1779] {
            s.store_mul_ad(1773, A::div_scaled_inputs(s.ad_value(342), 1e-9, A::mul(s.ad_value(345), s.ad_value(116)), 1.0), A::add_scaled_product(A::div_scaled_product(s.ad_value(179), A::add(s.ad_value(1774), s.ad_value(1775)), 1.0, A::mul(s.ad_value(116), s.ad_value(239)), 1.0), 1.0, s.ad_value(180), s.ad_value(226), 1.0));
        }

        if (!s.b[1779]) {
            s.store_scalar(1773, 0.0);
        }

        s.store_scaled_mul(1780, 390, 226, 1.0 / (1.602176565e-19));

        s.store_scaled_add(1781, 407, 432, (-0.5));

        s.store_add(1782, 415, 1781);

        s.store_div(0, 415, 1782);

        s.store_scaled_add_ad_rhs(1787, 0, A::sqrt(A::offset(A::mul(s.ad_value(0), s.ad_value(0)), 1e-20)), 0.5);

        s.store_scaled_mul(1788, 436, 435, (-0.1666666666667));

        s.store_square(1789, 1788);

        s.store_offset(1790, 429, (-1.0));

        s.store_max_with_scalar_ad(1791, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1790), 12.0, s.ad_value(1789))), 1e-20);

        s.store_div_from_scalar_square_ad(1792, 1.0, s.ad_value(1791));

        s.store_div_ad_lhs(1793, A::div_scaled_product3(A::mul3(s.ad_value(342), s.ad_value(390), s.ad_value(226)), s.ad_value(1782), s.ad_value(344), 1.0, s.ad_value(345), 1.0), 346);

        s.store_scale(1794, 1789, 12.0);

        s.store_ad_value(2, A::add_scaled_inputs3(s.ad_value(1787), 1.0, s.ad_value(1794), 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794), s.ad_value(1790), 2.0), -1.0));

        s.store_max_with_scalar(3, 2, 1e-40);

        s.store_mul3_lhs(1795, 1793, 1792, 3);

        s.b[1812] = (s.v[172] > 0.0);
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        if s.b[1812] {
            s.store_div(1796, 427, 422);
            s.store_ad_value(1797, A::div_scaled_product(A::mul3(s.ad_value(309), s.ad_value(348), s.ad_value(411)), s.ad_value(223), 1.0, A::mul3(A::offset(A::square(s.ad_value(1796)), 1.0), s.ad_value(1791), s.ad_value(1791)), 1.0));
            s.store_add_ad_rhs(1795, 1795, A::div(s.ad_value(1797), s.ad_value(308)));
        }

        s.store_mul3_affine_lhs(1798, 13, 307, p.p31, 0.0, 1795);

        s.store_ad_value(1799, A::div_scaled_product3(s.ad_value(456), s.ad_value(447), s.ad_value(116), 1.0, s.ad_value(469), 1.0));

        s.store_mul_offset_lhs(1800, 468, 1.0, 1799);

        s.store_mul_sub_from_scalar_ad_rhs(1802, 1800, 0.5, A::mul_scaled_lhs(s.ad_value(334), 0.25, s.ad_value(1788)));

        s.store_sub(1801, 1800, 1802);

        s.v[1805] = 0.0;

        s.v[1806] = 0.0;

        s.b[1813] = (p.p6 > 0.0);
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if s.b[1813] {
            s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1787), 0.08333333333333333, s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 0.2), s.ad_value(1794)), (-1.0)), A::mul3_scaled_output(s.ad_value(1789), A::sub(A::offset(s.ad_value(1787), 1.0), s.ad_value(1794)), s.ad_value(1790), 1.6));
            s.store_max_with_scalar(3, 2, 1e-40);
            s.store_ad_value(1803, A::div_scaled_product3(s.ad_value(1793), s.ad_value(1791), s.ad_value(1791), 1.0, s.ad_value(3), 1.0));
            s.store_mul3_affine_lhs(1804, 13, 307, p.p31, 0.0, 1803);
        }

        s.b[1814] = (s.v[1795] > 0.0);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        if (s.b[1813] && s.b[1814]) {
            s.store_mul_ad_product_rhs(1805, 1792, s.ad_value(1788), A::add_scaled_sub_value_product(1.0, s.ad_value(1794), 1.0, A::add_scaled_inputs_product(s.ad_value(1787), 1.0, s.ad_value(1789), 19.2, s.ad_value(1787), s.ad_value(1794), (-1.0)), s.ad_value(1790), (-1.0)));
            s.store_ad_value(1806, A::div_scaled_product(A::square(s.ad_value(1805)), s.ad_value(1803), 1.0, s.ad_value(1795), 1.0));
        }

        if (s.b[1813] && s.b[1814]) {
            let assign43720_ad_e48945: A = A::add_scaled_inputs3_offset(s.ad_value(1806), 0.5, A::sqrt(A::offset(A::mul(s.ad_value(1806), s.ad_value(1806)), 1e-40)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::add_scaled_inputs(s.ad_value(1806), 0.5, A::sqrt(A::offset(A::mul(s.ad_value(1806), s.ad_value(1806)), 1e-40)), 0.5), (-1.0), A::offset(A::add_scaled_inputs(s.ad_value(1806), 0.5, A::sqrt(A::offset(A::mul(s.ad_value(1806), s.ad_value(1806)), 1e-40)), 0.5), (-1.0))), 1e-40)), -1.0, 1.0);
            s.store_scale_ad(1806, assign43720_ad_e48945, 0.5);
        }

        if (!s.b[1813]) {
            s.store_scalar(1803, 1.0);
            s.store_scalar(1804, 0.0);
        }

        s.store_mul_sub_from_scalar_rhs(1807, 1798, 1.0, 1806);

        s.copy_ad(1783, 1780);

        s.store_mul_offset_rhs(1784, 1780, 415, 1.0);

        s.store_mul_sub_rhs(1785, 1780, 403, 413);

        s.store_mul_ad(2, A::add(A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1783), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1783), s.ad_value(1783))), A::ln(A::div(A::add_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5), A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1785), 0.5))));

        s.store_ad_value(3, A::add_scaled_product(s.ad_value(2), 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1784), 1.0, s.ad_value(1783), 2.0), 1.0), s.ad_value(1785), 1.0));

        s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(417), 1.0, s.ad_value(177), s.ad_value(418), 1.0), A::offset(s.ad_value(415), 1.0), 1.0);

        s.store_scaled_add_ad(4, A::offset(s.ad_value(0), 0.01), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(0), (-0.01), A::offset(s.ad_value(0), (-0.01))), 0.0001)), 0.5);

        s.store_mul_ad_lhs(0, A::div_scaled_product(A::div_scaled_product(s.ad_value(347), s.ad_value(348), 1.602176565e-19, s.ad_value(345), 1.0), s.ad_value(3), 1.0, s.ad_value(1783), 1.0), 4);

        s.store_mul_scaled_ad_rhs(1786, 13, p.p33, A::max_with_scalar(s.ad_value(0), 0.0));

        s.store_mul_scaled_ad_rhs(1808, 13, ((2.0 * 1.602176565e-19) * p.p31), A::abs(s.ad_value(352)));

        s.store_mul_scaled_ad_rhs(1809, 13, ((2.0 * 1.602176565e-19) * p.p31), A::abs(s.ad_value(353)));

        s.store_ad_value(1811, A::mul_offset_lhs_scaled_output(s.ad_value(357), 1.0, A::abs(s.ad_value(358)), (2.0 * 1.602176565e-19)));

        s.store_ad_value(1810, A::add_scaled_products(s.ad_value(13), A::abs(A::sub(s.ad_value(354), s.ad_value(355))), ((2.0 * 1.602176565e-19) * p.p31), s.ad_value(13), s.ad_value(1811), p.p31));

        s.store_div_from_scalar_scaled_input(1823, 1.0, 8, 8.617332384961e-5);

        s.store_sub_from_scalar_ad(1824, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));

        s.store_sub_from_scalar_ad(1825, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));

        s.store_mul_ad_lhs(1826, A::add_scaled_inputs3(s.ad_value(1825), 1.0, s.ad_value(1824), (-1.0), s.ad_value(228), (-0.4)), 15);

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add(1827, 1824, 1826);

        s.store_scaled_mul(1828, 1827, 1823, 0.5);

        s.store_sub_scaled_inputs(1829, 15, 0.05, 1826, 0.5);

        s.store_sqrt_scaled_input(0, 8, 0.0033333333333);

        s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);

        s.store_mul(1830, 2, 238);

        s.store_div_ad_rhs(1831, 1823, A::offset(A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0));

        s.store_mul3_affine_lhs(1833, 1830, 229, (2.0 * 1.602176565e-19), 0.0, 1831);

        s.store_add_ad_lhs(1834, A::offset(A::ln(A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(1833), 1.0)), (-0.6931471805599)), 1828);

        s.store_mul_ad_lhs(1835, A::div_scaled_product(s.ad_value(29), s.ad_value(14), (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0), 1831);

        s.store_mul(1838, 35, 1831);

        s.v[1839] = 0.0;

        s.v[1832] = 0.0;

        s.b[1884] = (p.p9 > 0.0);
        s.v[1884] = if s.b[1884] { 1.0 } else { 0.0 };

        if s.b[1884] {
            s.store_mul_ad(1832, A::div_from_scalar(1.0, s.ad_value(1823)), A::ln(A::div(s.ad_value(24), s.ad_value(251))));
        }

        s.b[1885] = (p.p13 > 0.0);
        s.v[1885] = if s.b[1885] { 1.0 } else { 0.0 };

        s.b[1886] = (p.p14 == 1.0);
        s.v[1886] = if s.b[1886] { 1.0 } else { 0.0 };

        if (s.b[1885] && s.b[1886]) {
            s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));
        }

        if (s.b[1885] && (!s.b[1886])) {
            s.store_scale_ad(1839, A::exp_scaled_input(A::ln(A::div(s.ad_value(259), s.ad_value(1831))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_mul(1842, 336, 1831);

        s.store_mul_offset_ad_lhs(1843, A::sqrt(A::offset(A::square(s.ad_value(336)), 0.01)), (-0.1), 1831);

        s.store_scaled_sub(1844, 1842, 1843, 0.5);

        s.store_div_ad(1815, A::div(s.ad_value(402), s.ad_value(401)), A::offset(s.ad_value(402), 1.0));

        s.store_div_ad(1816, A::div(s.ad_value(401), s.ad_value(402)), A::offset(s.ad_value(401), 1.0));

        s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(401), A::offset(s.ad_value(1815), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);

        s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(402), A::offset(s.ad_value(1816), 1.0), s.ad_value(384), 1.0, s.ad_value(385), 1.0), 2.0);

        s.store_ad_value(1819, A::add_scaled_products(A::offset(s.ad_value(1815), 1.0), s.ad_value(1817), 1.0, s.ad_value(399), s.ad_value(1815), (-1.0)));

        s.store_ad_value(1820, A::add_scaled_offset_product_lhs(A::div(s.ad_value(399), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, s.ad_value(1818), 1.0));

        s.store_add_ad_lhs(1821, A::div(A::sub(A::add_scaled_inputs3(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1819), s.ad_value(1820)), A::sub(s.ad_value(1819), s.ad_value(1820))), 38.0)), (-0.5)), s.ad_value(398)), s.ad_value(25)), 398);

        s.store_ad_value(1822, A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(226), A::add_scaled_inputs3(A::div(A::sub(s.ad_value(1821), s.ad_value(394)), s.ad_value(395)), 1.0, s.ad_value(397), (-1.0), s.ad_value(394), 1.0), 1.0));

        s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));

        s.store_ad_value(0, A::add_scaled_offset_product_rhs(s.ad_value(256), 1.0, s.ad_value(23), s.ad_value(8), (-s.v[7]), p.p14));

        s.store_ad_value(1840, A::add_scaled_inputs3_offset(A::add_scaled_inputs3(s.ad_value(183), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14), 1.0, s.ad_value(0), 1.0, s.ad_value(1832), -1.0, p.p34));

        s.store_add_ad_lhs(1841, A::add_scaled_inputs3(s.ad_value(184), p.p14, s.ad_value(1829), p.p14, s.ad_value(244), p.p14), 0);

        s.store_ad_value(1845, A::add_scaled_product(s.ad_value(1844), (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), s.ad_value(1831), 1.0));

        s.store_ad_value(1846, A::add_scaled_product(s.ad_value(1844), (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), s.ad_value(1831), 1.0));

        s.b[1887] = (p.p2 > 0.0);
        s.v[1887] = if s.b[1887] { 1.0 } else { 0.0 };

        if s.b[1887] {
            s.store_ad_value(0, A::div_scaled_product(s.ad_value(16), A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, s.ad_value(260), 1.0));
        }

        s.b[1888] = (s.v[0] < 0.0);
        s.v[1888] = if s.b[1888] { 1.0 } else { 0.0 };

        if (s.b[1887] && s.b[1888]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if (s.b[1887] && (!s.b[1888])) {
            s.store_ad_value(2, A::div_scaled_product_offset_denominator(s.ad_value(0), s.ad_value(0), 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0));
        }

        if s.b[1887] {
            s.store_ad_value(1847, A::add_scaled_product(s.ad_value(1846), 1.0, s.ad_value(16), s.ad_value(2), p.p14));
        }

        if (!s.b[1887]) {
            s.copy_ad(1847, 1846);
        }

        s.store_mul_sub_rhs(0, 248, 1845, 1847);

        s.b[1889] = (p.p13 > 0.0);
        s.v[1889] = if s.b[1889] { 1.0 } else { 0.0 };

        if s.b[1889] {
            s.store_ad_value(1848, A::add_scaled_inputs3(s.ad_value(0), 0.5, s.ad_value(257), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257)), 1.0)), 0.5));
            s.store_ad_value(1849, A::add_scaled_inputs3(s.ad_value(257), 0.5, s.ad_value(0), ((-1.0) * 0.5), A::sqrt(A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), 1.0)), 0.5));
            s.store_mul_ad_rhs(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_ad_value(1851, A::div_scaled_product_offset_denominator(s.ad_value(246), s.ad_value(4), 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0));
            s.store_ad_value(1852, A::div_scaled_product_offset_denominator(s.ad_value(247), s.ad_value(4), 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0));
            s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));
        }

        if (!s.b[1889]) {
            s.copy_ad(1851, 246);
            s.copy_ad(1852, 247);
            s.copy_ad(1853, 248);
        }

        s.store_mul_sub_rhs(1854, 1853, 1845, 1847);

        s.b[1890] = (s.v[1854] > 0.0);
        s.v[1890] = if s.b[1890] { 1.0 } else { 0.0 };

        s.b[1891] = ((-s.v[1854]) < 80.0);
        s.v[1891] = if s.b[1891] { 1.0 } else { 0.0 };

        if (s.b[1890] && s.b[1891]) {
            s.store_ln_one_plus_exp_neg_input(0, 1854);
        }

        if (s.b[1890] && (!s.b[1891])) {
            s.store_neg(0, 1854);
        }

        if s.b[1890] {
            s.store_offset_ad(1855, A::add_scaled_inputs3(s.ad_value(1845), 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), s.ad_value(0), 1.0), (-0.6931471805599));
        }

        s.b[1892] = (s.v[1854] < 80.0);
        s.v[1892] = if s.b[1892] { 1.0 } else { 0.0 };

        if ((!s.b[1890]) && s.b[1892]) {
            s.store_ln_one_plus_exp(0, 1854);
        }

        if ((!s.b[1890]) && (!s.b[1892])) {
            s.copy_ad(0, 1854);
        }

        if (!s.b[1890]) {
            s.store_offset_ad(1855, A::add_scaled_inputs3(s.ad_value(1847), 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, s.ad_value(0), 1.0), (-0.6931471805599));
        }

        s.store_ad_value(1856, A::add_scaled_inputs3(s.ad_value(1855), 0.5, s.ad_value(1834), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1855), s.ad_value(1834)), A::sub(s.ad_value(1855), s.ad_value(1834))), 4.0)), (-0.5)));

        s.store_offset_sqrt_ad(1857, A::offset(A::div(A::sub_scaled_inputs(s.ad_value(1834), 2.0, s.ad_value(1856), 2.0), s.ad_value(1835)), 1.0), (-1.0));

        s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + ((-0.5))))), 0.01)), 0.5);

        s.store_ad_value(0, A::mul_offset_rhs(A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0));

        s.b[1894] = (p.p11 > 0.0);
        s.v[1894] = if s.b[1894] { 1.0 } else { 0.0 };

        if s.b[1894] {
            s.store_div_ad(1815, A::div(s.ad_value(457), s.ad_value(456)), A::offset(s.ad_value(457), 1.0));
            s.store_div_ad(1816, A::div(s.ad_value(456), s.ad_value(457)), A::offset(s.ad_value(456), 1.0));
            s.store_offset_ln_ad(1817, A::div_scaled_product3(s.ad_value(456), A::offset(s.ad_value(1815), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);
            s.store_offset_ln_ad(1818, A::div_scaled_product3(s.ad_value(457), A::offset(s.ad_value(1816), 1.0), s.ad_value(441), 1.0, s.ad_value(442), 1.0), 2.0);
            s.store_ad_value(1819, A::add_scaled_products(A::offset(s.ad_value(1815), 1.0), s.ad_value(1817), 1.0, s.ad_value(455), s.ad_value(1815), (-1.0)));
            s.store_ad_value(1820, A::add_scaled_offset_product_lhs(A::div(s.ad_value(455), s.ad_value(1816)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1816)), 1.0, s.ad_value(1818), 1.0));
            s.store_add_ad_lhs(1821, A::div(A::sub(A::add_scaled_inputs3(s.ad_value(1819), 0.5, s.ad_value(1820), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1819), s.ad_value(1820)), A::sub(s.ad_value(1819), s.ad_value(1820))), 38.0)), (-0.5)), s.ad_value(454)), s.ad_value(25)), 454);
            s.store_ad_value(1822, A::add_scaled_product(s.ad_value(130), 1.0, s.ad_value(226), A::add_scaled_inputs3(A::div(A::sub(s.ad_value(1821), s.ad_value(450)), s.ad_value(451)), 1.0, s.ad_value(453), (-1.0), s.ad_value(450), 1.0), 1.0));
            s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));
            s.store_ad_value(0, A::add_scaled_offset_product_rhs(s.ad_value(256), 1.0, s.ad_value(23), s.ad_value(8), (-s.v[7]), p.p14));
            s.store_ad_value(1840, A::add_scaled_inputs3_offset(A::add_scaled_inputs3(s.ad_value(185), p.p14, s.ad_value(1829), p.p14, s.ad_value(243), p.p14), 1.0, s.ad_value(0), 1.0, s.ad_value(1832), -1.0, p.p34));
            s.store_add_ad_lhs(1841, A::add_scaled_inputs3(s.ad_value(186), p.p14, s.ad_value(1829), p.p14, s.ad_value(244), p.p14), 0);
            s.store_ad_value(1845, A::add_scaled_product(s.ad_value(1844), (-1.0), A::sub(s.ad_value(1822), s.ad_value(1840)), s.ad_value(1831), 1.0));
            s.store_ad_value(1846, A::add_scaled_product(s.ad_value(1844), (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(1841), 1.0), s.ad_value(1831), 1.0));
        }

        s.b[1895] = (p.p2 > 0.0);
        s.v[1895] = if s.b[1895] { 1.0 } else { 0.0 };

        if (s.b[1894] && s.b[1895]) {
            s.store_ad_value(0, A::div_scaled_product(s.ad_value(16), A::sub(s.ad_value(1845), s.ad_value(1846)), p.p14, s.ad_value(260), 1.0));
        }

        s.b[1896] = (s.v[0] < 0.0);
        s.v[1896] = if s.b[1896] { 1.0 } else { 0.0 };

        if ((s.b[1894] && s.b[1895]) && s.b[1896]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if ((s.b[1894] && s.b[1895]) && (!s.b[1896])) {
            s.store_ad_value(2, A::div_scaled_product_offset_denominator(s.ad_value(0), s.ad_value(0), 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(260), 1.0), 1.0, 1.0));
        }

        if (s.b[1894] && s.b[1895]) {
            s.store_ad_value(1847, A::add_scaled_product(s.ad_value(1846), 1.0, s.ad_value(16), s.ad_value(2), p.p14));
        }

        if (s.b[1894] && (!s.b[1895])) {
            s.copy_ad(1847, 1846);
        }

        if s.b[1894] {
            s.store_mul_sub_rhs(0, 248, 1845, 1847);
        }

        s.b[1897] = (p.p13 > 0.0);
        s.v[1897] = if s.b[1897] { 1.0 } else { 0.0 };

        if (s.b[1894] && s.b[1897]) {
            s.store_ad_value(1848, A::add_scaled_inputs3(s.ad_value(0), 0.5, s.ad_value(257), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257)), 1.0)), 0.5));
            s.store_ad_value(1849, A::add_scaled_inputs3(s.ad_value(257), 0.5, s.ad_value(0), ((-1.0) * 0.5), A::sqrt(A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), 1.0)), 0.5));
            s.store_mul_ad_rhs(2, 1839, A::exp_scaled_input(A::ln(s.ad_value(1848)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1839, A::exp_scaled_input(A::ln(s.ad_value(1849)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_ad_value(1851, A::div_scaled_product_offset_denominator(s.ad_value(246), s.ad_value(4), 1.0, A::mul(s.ad_value(246), s.ad_value(2)), 1.0, 1.0));
            s.store_ad_value(1852, A::div_scaled_product_offset_denominator(s.ad_value(247), s.ad_value(4), 1.0, A::mul(s.ad_value(247), s.ad_value(3)), 1.0, 1.0));
            s.store_div_from_scalar_add_ad(1853, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(1851)), 1.0), A::div_from_scalar(1.0, s.ad_value(1852)));
        }

        if (s.b[1894] && (!s.b[1897])) {
            s.copy_ad(1851, 246);
            s.copy_ad(1852, 247);
            s.copy_ad(1853, 248);
        }

        if s.b[1894] {
            s.store_mul_sub_rhs(1854, 1853, 1845, 1847);
        }

        s.b[1898] = (s.v[1854] > 0.0);
        s.v[1898] = if s.b[1898] { 1.0 } else { 0.0 };

        s.b[1899] = ((-s.v[1854]) < 80.0);
        s.v[1899] = if s.b[1899] { 1.0 } else { 0.0 };

        if ((s.b[1894] && s.b[1898]) && s.b[1899]) {
            s.store_ln_one_plus_exp_neg_input(0, 1854);
        }

        if ((s.b[1894] && s.b[1898]) && (!s.b[1899])) {
            s.store_neg(0, 1854);
        }

        if (s.b[1894] && s.b[1898]) {
            s.store_offset_ad(1855, A::add_scaled_inputs3(s.ad_value(1845), 1.0, A::div(s.ad_value(1854), s.ad_value(1851)), (-1.0), s.ad_value(0), 1.0), (-0.6931471805599));
        }

        s.b[1900] = (s.v[1854] < 80.0);
        s.v[1900] = if s.b[1900] { 1.0 } else { 0.0 };

        if ((s.b[1894] && (!s.b[1898])) && s.b[1900]) {
            s.store_ln_one_plus_exp(0, 1854);
        }

        if ((s.b[1894] && (!s.b[1898])) && (!s.b[1900])) {
            s.copy_ad(0, 1854);
        }

        if (s.b[1894] && (!s.b[1898])) {
            s.store_offset_ad(1855, A::add_scaled_inputs3(s.ad_value(1847), 1.0, A::div(s.ad_value(1854), s.ad_value(1852)), 1.0, s.ad_value(0), 1.0), (-0.6931471805599));
        }

        if s.b[1894] {
            s.store_ad_value(1856, A::add_scaled_inputs3(s.ad_value(1855), 0.5, s.ad_value(1834), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1855), s.ad_value(1834)), A::sub(s.ad_value(1855), s.ad_value(1834))), 4.0)), (-0.5)));
            s.store_offset_sqrt_ad(1857, A::offset(A::div(A::sub_scaled_inputs(s.ad_value(1834), 2.0, s.ad_value(1856), 2.0), s.ad_value(1835)), 1.0), (-1.0));
            s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(1846)), ((1.0) + ((-0.5))))), 0.01)), 0.5);
            s.store_ad_value(0, A::mul_offset_rhs(A::mul3_scaled_output(s.ad_value(1838), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1843), s.ad_value(1838)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1857)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1846)), 1.0));
        }

    }
}
