#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1214] = ((((-s.v[695])) as f64).abs() < 80.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && s.b[1214]) {
            s.store_exp_neg_input(687, 695);
        }

        s.b[1215] = ((-s.v[695]) < (-80.0));
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1214])) && s.b[1215]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(695))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1214])) && (!s.b[1215])) {
            s.store_scaled_offset_mul_offset_lhs_ad(687, A::neg(s.ad_value(695)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(695)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) {
            s.store_sub_from_scalar(693, 1.0, 687);
            s.store_add_scaled_inputs_product_right_ad(696, 700, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(700), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));
        }

        s.b[1216] = ((((-s.v[696])) as f64).abs() < 80.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && s.b[1216]) {
            s.store_exp_neg_input(689, 696);
        }

        s.b[1217] = ((-s.v[696]) < (-80.0));
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1216])) && s.b[1217]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(696))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) && (!s.b[1216])) && (!s.b[1217])) {
            s.store_scaled_offset_mul_offset_lhs_ad(689, A::neg(s.ad_value(696)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(696)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) {
            s.store_add_scaled_inputs3_mixed_iia(690, 700, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);
            s.store_add_scaled_products_mixed_aaia(691, A::sub(s.ad_value(700), s.ad_value(696)), A::sub(s.ad_value(700), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
            s.store_add(710, 696, 697);
        }

        if (s.b[1209] && (!s.b[1210])) {
            s.store_neg(710, 710);
        }

        s.b[1218] = (s.v[159] > 0.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        s.b[1219] = (((s.v[702]) as f64).abs() <= s.v[707]);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if (s.b[1218] && s.b[1219]) {
            s.store_mul_neg_lhs(712, 702, 708);
        }

        s.b[1220] = (s.v[702] < (-s.v[707]));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {
            s.store_neg(679, 702);
            s.store_scaled_mul(680, 679, 708, 1.25);
            s.store_scaled_sub_ad(681, A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(680), (-6.0), A::offset(s.ad_value(680), (-6.0))), 64.0)), 0.5);
            s.store_add_scaled_products_mixed_aaia(682, A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
            s.store_add(685, 682, 683);
            s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);
            s.store_add_ad_rhs(687, 686, A::mul3(A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), s.ad_value(683), A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0)));
            s.store_add_ad_rhs(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));
        }

        s.b[1221] = (((s.v[688]) as f64).abs() < 80.0);
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if (((s.b[1218] && (!s.b[1219])) && s.b[1220]) && s.b[1221]) {
            s.store_exp(689, 688);
        }

        s.b[1222] = (s.v[688] < (-80.0));
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        if ((((s.b[1218] && (!s.b[1219])) && s.b[1220]) && (!s.b[1221])) && s.b[1222]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(688)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1218] && (!s.b[1219])) && s.b[1220]) && (!s.b[1221])) && (!s.b[1222])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(689, 688, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(688), (-80.0)), 0.5, A::scale_offset(s.ad_value(688), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1218] && (!s.b[1219])) && s.b[1220]) {
            s.store_sub(687, 679, 688);
            s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
            s.store_neg_ad(712, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {
            s.store_mul_offset_ad_lhs(694, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), (-1.0), 709);
            s.store_mul_ad_product_rhs(695, 702, s.ad_value(708), A::offset(A::mul(s.ad_value(694), s.ad_value(702)), 1.0));
        }

        s.b[1223] = ((((-s.v[695])) as f64).abs() < 80.0);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if (((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && s.b[1223]) {
            s.store_exp_neg_input(687, 695);
        }

        s.b[1224] = ((-s.v[695]) < (-80.0));
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1223])) && s.b[1224]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(695))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1223])) && (!s.b[1224])) {
            s.store_scaled_offset_mul_offset_lhs_ad(687, A::neg(s.ad_value(695)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(695)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {
            s.store_sub_from_scalar(693, 1.0, 687);
            s.store_add_scaled_inputs_product_right_ad(696, 702, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(702), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));
        }

        s.b[1225] = ((((-s.v[696])) as f64).abs() < 80.0);
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && s.b[1225]) {
            s.store_exp_neg_input(689, 696);
        }

        s.b[1226] = ((-s.v[696]) < (-80.0));
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1225])) && s.b[1226]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(696))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) && (!s.b[1225])) && (!s.b[1226])) {
            s.store_scaled_offset_mul_offset_lhs_ad(689, A::neg(s.ad_value(696)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(696)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1218] && (!s.b[1219])) && (!s.b[1220])) {
            s.store_add_scaled_inputs3_mixed_iia(690, 702, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);
            s.store_add_scaled_products_mixed_aaia(691, A::sub(s.ad_value(702), s.ad_value(696)), A::sub(s.ad_value(702), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
            s.store_add(712, 696, 697);
        }

        if (s.b[1218] && (!s.b[1219])) {
            s.store_neg(712, 712);
        }

        s.store_div_ad_lhs(704, A::sqrt(A::mul3_scaled_output(s.ad_value(20), s.ad_value(225), s.ad_value(220), (2.0 * 1.602176565e-19))), 237);

        s.store_square(705, 704);

        s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);

        let assign24310_e24535: f64 = (1e-5 * s.v[706]);
        s.v[707] = assign24310_e24535;

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
            s.store_scaled_sub_ad(681, A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(680), (-6.0), A::offset(s.ad_value(680), (-6.0))), 64.0)), 0.5);
            s.store_add_scaled_products_mixed_aaia(682, A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
            s.store_add(685, 682, 683);
            s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);
            s.store_add_ad_rhs(687, 686, A::mul3(A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), s.ad_value(683), A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0)));
            s.store_add_ad_rhs(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));
        }

        s.b[1230] = (((s.v[688]) as f64).abs() < 80.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (((s.b[1227] && (!s.b[1228])) && s.b[1229]) && s.b[1230]) {
            s.store_exp(689, 688);
        }

        s.b[1231] = (s.v[688] < (-80.0));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && s.b[1231]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(688)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1227] && (!s.b[1228])) && s.b[1229]) && (!s.b[1230])) && (!s.b[1231])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(689, 688, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(688), (-80.0)), 0.5, A::scale_offset(s.ad_value(688), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1227] && (!s.b[1228])) && s.b[1229]) {
            s.store_sub(687, 679, 688);
            s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
            s.store_neg_ad(711, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {
            s.store_mul_offset_ad_lhs(694, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), (-1.0), 709);
            s.store_mul_ad_product_rhs(695, 701, s.ad_value(708), A::offset(A::mul(s.ad_value(694), s.ad_value(701)), 1.0));
        }

        s.b[1232] = ((((-s.v[695])) as f64).abs() < 80.0);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if (((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && s.b[1232]) {
            s.store_exp_neg_input(687, 695);
        }

        s.b[1233] = ((-s.v[695]) < (-80.0));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1232])) && s.b[1233]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(695))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1232])) && (!s.b[1233])) {
            s.store_scaled_offset_mul_offset_lhs_ad(687, A::neg(s.ad_value(695)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(695)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {
            s.store_sub_from_scalar(693, 1.0, 687);
            s.store_add_scaled_inputs_product_right_ad(696, 701, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(701), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));
        }

        s.b[1234] = ((((-s.v[696])) as f64).abs() < 80.0);
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if (((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && s.b[1234]) {
            s.store_exp_neg_input(689, 696);
        }

        s.b[1235] = ((-s.v[696]) < (-80.0));
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && s.b[1235]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(696))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) && (!s.b[1234])) && (!s.b[1235])) {
            s.store_scaled_offset_mul_offset_lhs_ad(689, A::neg(s.ad_value(696)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(696)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1227] && (!s.b[1228])) && (!s.b[1229])) {
            s.store_add_scaled_inputs3_mixed_iia(690, 701, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);
            s.store_add_scaled_products_mixed_aaia(691, A::sub(s.ad_value(701), s.ad_value(696)), A::sub(s.ad_value(701), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
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
            s.store_scaled_sub_ad(681, A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(680), (-6.0), A::offset(s.ad_value(680), (-6.0))), 64.0)), 0.5);
            s.store_add_scaled_products_mixed_aaia(682, A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
            s.store_add(685, 682, 683);
            s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);
            s.store_add_ad_rhs(687, 686, A::mul3(A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), s.ad_value(683), A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0)));
            s.store_add_ad_rhs(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));
        }

        s.b[1239] = (((s.v[688]) as f64).abs() < 80.0);
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if (((s.b[1236] && (!s.b[1237])) && s.b[1238]) && s.b[1239]) {
            s.store_exp(689, 688);
        }

        s.b[1240] = (s.v[688] < (-80.0));
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && s.b[1240]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(688)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1236] && (!s.b[1237])) && s.b[1238]) && (!s.b[1239])) && (!s.b[1240])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(689, 688, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(688), (-80.0)), 0.5, A::scale_offset(s.ad_value(688), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1236] && (!s.b[1237])) && s.b[1238]) {
            s.store_sub(687, 679, 688);
            s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
            s.store_neg_ad(713, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {
            s.store_mul_offset_ad_lhs(694, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), (-1.0), 709);
            s.store_mul_ad_product_rhs(695, 703, s.ad_value(708), A::offset(A::mul(s.ad_value(694), s.ad_value(703)), 1.0));
        }

        s.b[1241] = ((((-s.v[695])) as f64).abs() < 80.0);
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if (((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && s.b[1241]) {
            s.store_exp_neg_input(687, 695);
        }

        s.b[1242] = ((-s.v[695]) < (-80.0));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1241])) && s.b[1242]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(687, 1.80485e-35, A::neg(A::neg(s.ad_value(695))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(695))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(695))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1241])) && (!s.b[1242])) {
            s.store_scaled_offset_mul_offset_lhs_ad(687, A::neg(s.ad_value(695)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(695)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(695)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {
            s.store_sub_from_scalar(693, 1.0, 687);
            s.store_add_scaled_inputs_product_right_ad(696, 703, 1.0, 705, 0.5, 704, A::sqrt(A::add_scaled_inputs3(s.ad_value(703), 1.0, s.ad_value(705), 0.25, s.ad_value(693), -1.0)), (-1.0));
        }

        s.b[1243] = ((((-s.v[696])) as f64).abs() < 80.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && s.b[1243]) {
            s.store_exp_neg_input(689, 696);
        }

        s.b[1244] = ((-s.v[696]) < (-80.0));
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && s.b[1244]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(A::neg(s.ad_value(696))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(696))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(696))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scaled_offset_mul_offset_lhs_ad(689, A::neg(s.ad_value(696)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(696)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(696)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1236] && (!s.b[1237])) && (!s.b[1238])) {
            s.store_add_scaled_inputs3_mixed_iia(690, 703, 2.0, 696, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(705), 1.0, s.ad_value(689)), 1.0);
            s.store_add_scaled_products_mixed_aaia(691, A::sub(s.ad_value(703), s.ad_value(696)), A::sub(s.ad_value(703), s.ad_value(696)), 1.0, 705, A::add(A::offset(s.ad_value(696), (-1.0)), s.ad_value(689)), (-1.0));
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(697, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
            s.store_add(713, 696, 697);
        }

        if (s.b[1236] && (!s.b[1237])) {
            s.store_neg(713, 713);
        }

        s.store_mul_add_scaled_inputs_rhs(714, 219, s.ad_value(700), -1.0, s.ad_value(710), -1.0);

        s.store_mul_add_scaled_inputs_rhs(715, 219, s.ad_value(701), -1.0, s.ad_value(711), -1.0);

        s.store_mul_add_scaled_inputs_rhs(345, 219, s.ad_value(702), -1.0, s.ad_value(712), -1.0);

        s.store_mul_add_scaled_inputs_rhs(346, 219, s.ad_value(703), -1.0, s.ad_value(713), -1.0);

        s.v[729] = 0.0;

        s.v[730] = 0.0;

        s.v[347] = 0.0;

        s.v[348] = 0.0;

        s.v[349] = 0.0;

        s.v[749] = 0.0;

        s.v[750] = 0.0;

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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(s.ad_value(700), 0.5)), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(s.ad_value(700), 0.5)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1247])) && (!s.b[1248])) {
            s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(700), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(700), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(700), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);
            s.store_sub_from_scalar(3, 1.0, 2);
            s.store_add_scaled_products_indices(719, 83, 2, 1.0, 80, 3, 1.0);
            s.store_add_scaled_products_indices(720, 84, 2, 1.0, 82, 3, 1.0);
            s.store_add_scaled_products_indices(721, 278, 2, 1.0, 277, 3, 1.0);
            s.store_add_scaled_products_indices(722, 71, 2, 1.0, 69, 3, 1.0);
            s.store_scaled_mul(723, 73, 3, 1e-6);
            s.store_mul_div_scaled_inputs_rhs(2, 275, s.ad_value(81), (-1.0), s.ad_value(718), 1.0);
        }

        s.b[1249] = (s.v[720] < 0.0);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1249]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(718, 718, 0.5, 721, 0.5, A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(721)), A::sub(s.ad_value(718), s.ad_value(721))), 1e-6), (-0.5));
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_add_scaled_product_value_ad(724, A::offset(s.ad_value(710), 3.0), 1.0, 717, 220, 1.0);
        }

        s.b[1250] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1250]) {
            s.store_exp(725, 724);
        }

        s.b[1251] = (s.v[724] < (-80.0));
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && s.b[1251]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(724)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1250])) && (!s.b[1251])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(725, 724, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::scale_offset(s.ad_value(724), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_add_ad_lhs(724, A::add_scaled_product(A::offset(s.ad_value(710), 3.0), 1.0, s.ad_value(717), s.ad_value(220), 1.0), 700);
        }

        s.b[1252] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1252]) {
            s.store_exp(726, 724);
        }

        s.b[1253] = (s.v[724] < (-80.0));
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && s.b[1253]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(726, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(724)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1252])) && (!s.b[1253])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(726, 724, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::scale_offset(s.ad_value(724), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_mul_offset_ad_rhs(0, 275, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(719), 1.0, s.ad_value(720), s.ad_value(718), 1.0)), (-1.5));
        }

        s.b[1254] = (s.v[0] > 0.0);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1254]) {
            s.store_offset_mul_offset_rhs_ad_rhs(727, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);
        }

        s.b[1255] = (s.v[0] > (-80.0));
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1254])) && s.b[1255]) {
            s.store_exp(727, 0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1254])) && (!s.b[1255])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(727, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        s.b[1256] = (s.v[2] > 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1256]) {
            s.store_offset_mul_offset_rhs_ad_rhs(728, 2, A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0, 1.0);
        }

        s.b[1257] = (s.v[2] > (-80.0));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1256])) && s.b[1257]) {
            s.store_exp(728, 2);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1256])) && (!s.b[1257])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(728, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_div_scaled_offset_numerator(0, s.ad_value(725), 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0);
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1259])) && (!s.b[1260])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(3, 2, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(2), (-80.0)), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_add_scaled_product_indices(4, 2, 1.0, 85, 699, 1.0);
        }

        s.b[1261] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1246]) && s.b[1261]) {
            s.store_exp(5, 4);
        }

        s.b[1262] = (s.v[4] < (-80.0));
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && s.b[1262]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1246]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(5, 4, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1246]) {
            s.store_sub_ad(729, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(722), s.ad_value(727), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(723), s.ad_value(728), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(0, 1.80485e-35, A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(s.ad_value(701), 0.5)), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(s.ad_value(701), 0.5)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1264])) && (!s.b[1265])) {
            s.store_scaled_offset_ad(0, A::mul_offset_rhs(A::scale_offset(s.ad_value(701), 0.5, (-80.0)), A::mul_scaled_lhs(A::scale_offset(s.ad_value(701), 0.5, (-80.0)), 0.5, A::scale_offset(s.ad_value(701), ((0.5) * (0.3333333333333)), (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_div_from_scalar_offset_input(2, 1.0, 0, 1.0);
            s.store_sub_from_scalar(3, 1.0, 2);
            s.store_add_scaled_products_indices(719, 83, 2, 1.0, 80, 3, 1.0);
            s.store_add_scaled_products_indices(720, 84, 2, 1.0, 82, 3, 1.0);
            s.store_add_scaled_products_indices(721, 278, 2, 1.0, 277, 3, 1.0);
            s.store_add_scaled_products_indices(722, 72, 2, 1.0, 70, 3, 1.0);
            s.store_scaled_mul(723, 74, 3, 1e-6);
            s.store_mul_div_scaled_inputs_rhs(2, 275, s.ad_value(81), (-1.0), s.ad_value(718), 1.0);
        }

        s.b[1266] = (s.v[720] < 0.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1266]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(718, 718, 0.5, 721, 0.5, A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(721)), A::sub(s.ad_value(718), s.ad_value(721))), 1e-6), (-0.5));
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_add_scaled_product_value_ad(724, A::offset(s.ad_value(711), 3.0), 1.0, 717, 220, 1.0);
        }

        s.b[1267] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1267]) {
            s.store_exp(725, 724);
        }

        s.b[1268] = (s.v[724] < (-80.0));
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && s.b[1268]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(724)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1267])) && (!s.b[1268])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(725, 724, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::scale_offset(s.ad_value(724), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_add_ad_lhs(724, A::add_scaled_product(A::offset(s.ad_value(711), 3.0), 1.0, s.ad_value(717), s.ad_value(220), 1.0), 701);
        }

        s.b[1269] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1269]) {
            s.store_exp(726, 724);
        }

        s.b[1270] = (s.v[724] < (-80.0));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && s.b[1270]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(726, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(724)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1269])) && (!s.b[1270])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(726, 724, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::scale_offset(s.ad_value(724), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_mul_offset_ad_rhs(0, 275, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(719), 1.0, s.ad_value(720), s.ad_value(718), 1.0)), (-1.5));
        }

        s.b[1271] = (s.v[0] > 0.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1271]) {
            s.store_offset_mul_offset_rhs_ad_rhs(727, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);
        }

        s.b[1272] = (s.v[0] > (-80.0));
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1271])) && s.b[1272]) {
            s.store_exp(727, 0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1271])) && (!s.b[1272])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(727, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        s.b[1273] = (s.v[2] > 0.0);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1273]) {
            s.store_offset_mul_offset_rhs_ad_rhs(728, 2, A::mul_scaled_lhs(s.ad_value(2), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, 1.0)), 1.0, 1.0);
        }

        s.b[1274] = (s.v[2] > (-80.0));
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1273])) && s.b[1274]) {
            s.store_exp(728, 2);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1273])) && (!s.b[1274])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(728, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_div_scaled_offset_numerator(0, s.ad_value(725), 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1276])) && (!s.b[1277])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(3, 2, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(2), (-80.0)), 0.5, A::scale_offset(s.ad_value(2), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_add_scaled_product_indices(4, 2, 1.0, 85, 698, 1.0);
        }

        s.b[1278] = (((s.v[4]) as f64).abs() < 80.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1263]) && s.b[1278]) {
            s.store_exp(5, 4);
        }

        s.b[1279] = (s.v[4] < (-80.0));
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && s.b[1279]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1263]) && (!s.b[1278])) && (!s.b[1279])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(5, 4, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1263]) {
            s.store_sub_ad(730, A::div_scaled_product_offset_denominator(A::mul3(s.ad_value(722), s.ad_value(727), A::ln(s.ad_value(0))), A::offset(s.ad_value(3), 1.0), 1.0, s.ad_value(5), 1.0, 1.0), A::div_scaled_product3(s.ad_value(723), s.ad_value(728), A::offset(s.ad_value(3), 1.0), 1.0, A::offset(s.ad_value(5), 1.0), 1.0));
        }

        s.b[1280] = (s.v[68] > 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (s.b[1245] && s.b[1280]) {
            s.store_mul_neg_lhs(731, 432, 382);
        }

        s.b[1281] = (((((2.0 * s.v[731]) - s.v[407])) as f64).abs() < 80.0);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1281]) {
            s.store_exp_ad(0, A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0));
        }

        s.b[1282] = (((2.0 * s.v[731]) - s.v[407]) < (-80.0));
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && s.b[1282]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(0, 1.80485e-35, A::neg(A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0)), (-80.0)), 0.5, A::scale_offset(A::neg(A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1281])) && (!s.b[1282])) {
            s.store_scaled_offset_mul_offset_lhs_ad(0, A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0), (-80.0)), 0.5, A::scale_offset(A::sub_scaled_inputs(s.ad_value(731), 2.0, s.ad_value(407), 1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
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
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(718, 718, 0.5, 276, 0.5, A::offset(A::mul(A::sub(s.ad_value(718), s.ad_value(276)), A::sub(s.ad_value(718), s.ad_value(276))), 1e-6), (-0.5));
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_add(736, 396, 230);
            s.store_sub(735, 736, 733);
            s.store_mul_add_scaled_product_rhs(724, 282, s.ad_value(735), 1.0, A::add_scaled_inputs3(s.ad_value(717), 1.0, s.ad_value(279), (-1.0), s.ad_value(732), -1.0), s.ad_value(223), 1.0);
        }

        s.b[1284] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1284]) {
            s.store_exp(725, 724);
        }

        s.b[1285] = (s.v[724] < (-80.0));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && s.b[1285]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(725, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(724)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1284])) && (!s.b[1285])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(725, 724, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::scale_offset(s.ad_value(724), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_mul_ad_affine_product_lhs(724, A::sub(s.ad_value(331), s.ad_value(732)), s.ad_value(223), -1.0, 0.0, 282);
        }

        s.b[1286] = (((s.v[724]) as f64).abs() < 80.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1286]) {
            s.store_exp(0, 724);
        }

        s.b[1287] = (s.v[724] < (-80.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && s.b[1287]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(0, 1.80485e-35, A::neg(s.ad_value(724)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(724)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(724)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(0, 724, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(724), (-80.0)), 0.5, A::scale_offset(s.ad_value(724), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_mul(726, 725, 0);
            s.store_mul_offset_ad_rhs(0, 274, A::mul(s.ad_value(718), A::add_scaled_product(s.ad_value(78), 1.0, s.ad_value(79), s.ad_value(718), 1.0)), (-1.5));
        }

        s.b[1288] = (s.v[0] > 0.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1288]) {
            s.store_offset_mul_offset_rhs_ad_rhs(727, 0, A::mul_scaled_lhs(s.ad_value(0), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, 1.0)), 1.0, 1.0);
        }

        s.b[1289] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1288])) && s.b[1289]) {
            s.store_exp(727, 0);
        }

        s.b[1290] = (s.v[0] < (-80.0));
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((((s.b[1245] && s.b[1280]) && (!s.b[1288])) && (!s.b[1289])) && s.b[1290]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(727, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1245] && s.b[1280]) && (!s.b[1288])) && (!s.b[1289])) && (!s.b[1290])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(727, 0, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_mul_ad_product_rhs(737, 68, s.ad_value(727), A::ln(A::div_scaled_offset_numerator(s.ad_value(725), 1.0, 1.0, A::offset(s.ad_value(726), 1.0), 1.0)));
        }

        s.b[1291] = ((s.v[736] <= 0.0) || ((s.v[78] == 0.0) && (s.v[79] == 0.0)));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if ((s.b[1245] && s.b[1280]) && s.b[1291]) {
            s.store_scalar(738, 1.0);
            s.store_scalar(739, 0.5);
        }

        if ((s.b[1245] && s.b[1280]) && (!s.b[1291])) {
            s.store_add_scaled_product_indices(0, 78, 1.0, 79, 718, 2.0);
            s.store_mul_div_ad_lhs(740, s.ad_value(87), A::mul(s.ad_value(0), s.ad_value(274)), 223);
            s.store_div(741, 731, 740);
            s.store_mul3_lhs(742, 740, 430, 397);
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(744, 742, 1.0, 742, 1.0, 0.5);
            s.store_sub_from_scalar_scaled_input(743, 0.5, 744, 3.0);
        }

        s.b[1292] = (s.v[741] < 0.001);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && s.b[1292]) {
            s.store_square(745, 741);
            s.store_offset_mul_ad(738, s.ad_value(745), A::add_scaled_product(A::scale_offset(s.ad_value(742), 0.3333333333333, 0.1666666666667), 1.0, s.ad_value(745), A::scale_offset(s.ad_value(742), 0.2, 0.05), 0.1666666666667), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(739, 738, 0.5, 741, A::mul(s.ad_value(745), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(744), 0.25, 0.4), 1.0, s.ad_value(745), s.ad_value(744), 0.125, 0.0285714285714)), 1.0, (-0.1666666666667));
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) {
            s.store_div_from_scalar(746, 1.0, 741);
        }

        s.b[1293] = (((s.v[741]) as f64).abs() < 80.0);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if ((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {
            s.store_exp(747, 741);
        }

        s.b[1294] = (s.v[741] < (-80.0));
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(747, 1.80485e-35, A::neg(s.ad_value(741)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(741)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(741)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && (!s.b[1294])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(747, 741, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(741), (-80.0)), 0.5, A::scale_offset(s.ad_value(741), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (((s.b[1245] && s.b[1280]) && (!s.b[1291])) && (!s.b[1292])) {
            s.store_div_from_scalar(748, 1.0, 747);
            s.store_sub(0, 747, 748);
            s.store_add(3, 747, 748);
            s.store_add_scaled_products_left_left_ad(738, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(742), s.ad_value(0)), 746, 0.5, 742, 3, 0.5);
            s.store_scaled_sub_ad(739, A::add_scaled_product(s.ad_value(738), 1.0, s.ad_value(0), A::sub(s.ad_value(744), A::mul3(s.ad_value(743), s.ad_value(746), s.ad_value(746))), (-1.0)), A::mul3(s.ad_value(743), s.ad_value(3), s.ad_value(746)), 0.5);
        }

        if (s.b[1245] && s.b[1280]) {
            s.store_mul(347, 737, 738);
            s.store_mul(750, 737, 739);
            s.store_sub(749, 347, 750);
        }

        s.b[1295] = (s.v[330] < 0.0);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if (s.b[1245] && s.b[1295]) {
            s.store_add(348, 750, 729);
            s.store_add(349, 749, 730);
        }

        if (s.b[1245] && (!s.b[1295])) {
            s.store_add(348, 749, 729);
            s.store_add(349, 750, 730);
        }

        s.v[351] = 0.0;

        s.b[1296] = (((p.p4 > 0.0) && (s.v[89] > 0.0)) && (s.v[714] < 0.0));
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if s.b[1296] {
            s.store_sqrt_offset_ad(751, A::add(A::square(s.ad_value(714)), A::mul3(A::square(s.ad_value(95)), s.ad_value(327), s.ad_value(327))), 1e-6);
            s.store_div_scaled_inputs_indices(0, 91, -1.0, 751, 1.0);
        }

        s.b[1297] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if (s.b[1296] && s.b[1297]) {
            s.store_exp(3, 0);
        }

        s.b[1298] = (s.v[0] < (-80.0));
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if ((s.b[1296] && (!s.b[1297])) && s.b[1298]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1296] && (!s.b[1297])) && (!s.b[1298])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(3, 0, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1296] && (!s.b[1299])) && (!s.b[1300])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(5, 4, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if s.b[1296] {
            s.store_scaled_mul_ad(351, A::mul3(A::mul3_scaled_output(s.ad_value(89), s.ad_value(699), s.ad_value(714), -1.0), s.ad_value(751), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);
        }

        s.v[350] = 0.0;

        s.b[1301] = (((p.p4 > 0.0) && (s.v[90] > 0.0)) && (s.v[715] < 0.0));
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if s.b[1301] {
            s.store_sqrt_offset_ad(752, A::add(A::square(s.ad_value(715)), A::mul3(A::square(s.ad_value(96)), s.ad_value(329), s.ad_value(329))), 1e-6);
            s.store_div_scaled_inputs_indices(0, 92, -1.0, 752, 1.0);
        }

        s.b[1302] = (((s.v[0]) as f64).abs() < 80.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (s.b[1301] && s.b[1302]) {
            s.store_exp(3, 0);
        }

        s.b[1303] = (s.v[0] < (-80.0));
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if ((s.b[1301] && (!s.b[1302])) && s.b[1303]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(s.ad_value(0)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(0)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(0)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1301] && (!s.b[1302])) && (!s.b[1303])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(3, 0, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(0), (-80.0)), 0.5, A::scale_offset(s.ad_value(0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1301] && (!s.b[1304])) && (!s.b[1305])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(5, 4, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(4), (-80.0)), 0.5, A::scale_offset(s.ad_value(4), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if s.b[1301] {
            s.store_scaled_mul_ad(350, A::mul3(A::mul3_scaled_output(s.ad_value(90), s.ad_value(698), s.ad_value(715), -1.0), s.ad_value(752), s.ad_value(3)), A::offset(s.ad_value(5), 1.0), 0.5);
        }

        s.v[352] = 0.0;

        s.b[1306] = (p.p12 > 0.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if s.b[1306] {
            s.store_mul(754, 332, 285);
            s.store_mul_offset_ad_lhs(755, A::sqrt(A::offset(A::square(s.ad_value(332)), 0.01)), (-0.1), 285);
            s.store_scaled_sub(756, 754, 755, 0.5);
            s.store_sub_ad_lhs(757, A::add_scaled_product(s.ad_value(756), (-1.0), A::sub(s.ad_value(331), s.ad_value(100)), s.ad_value(285), 1.0), 230);
            s.store_sub_ad_lhs(758, A::add_scaled_product(s.ad_value(756), (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(101), 1.0), s.ad_value(285), 1.0), 230);
            s.store_div_from_scalar_offset_input(759, 1.0, 105, 1.0);
            s.store_div_from_scalar_offset_input(760, 1.0, 106, 1.0);
            s.store_mul(761, 109, 285);
            s.store_mul_scaled_offset_ad_rhs(0, 761, 2.0, A::sqrt(A::offset(A::div(s.ad_value(755), s.ad_value(761)), 1.0)), (-1.0));
            s.store_mul(762, 107, 0);
            s.store_mul(763, 108, 0);
            s.store_add_scaled_product_left_ad(764, 756, 1.0, A::add(s.ad_value(757), s.ad_value(762)), 759, 1.0);
            s.store_add_scaled_product_left_ad(765, 756, 1.0, A::add(s.ad_value(758), s.ad_value(763)), 760, 1.0);
        }

        if s.b[1306] {
            let assign27740_ad_e29663: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(765), 1.0, s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)), 1.0), 1.0, s.ad_value(221), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(765), 1.0, s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)), 1.0), s.ad_value(221)), A::sub(A::add_scaled_product(s.ad_value(765), 1.0, s.ad_value(103), A::sub(s.ad_value(764), s.ad_value(765)), 1.0), s.ad_value(221))), 0.01)), -1.0);
            s.store_scale_ad(766, assign27740_ad_e29663, 0.5);
        }

        if s.b[1306] {
            let assign27750_ad_e29700: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(764), 1.0, s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)), 1.0), 1.0, s.ad_value(221), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(764), 1.0, s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)), 1.0), s.ad_value(221)), A::sub(A::add_scaled_product(s.ad_value(764), 1.0, s.ad_value(104), A::sub(s.ad_value(765), s.ad_value(764)), 1.0), s.ad_value(221))), 0.01)), -1.0);
            s.store_scale_ad(767, assign27750_ad_e29700, 0.5);
        }

        if s.b[1306] {
            s.store_div(768, 242, 759);
        }

    }

    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1306] {
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
            s.store_add_scaled_sub_value_product_mixed_aii(2, 1.0, A::mul(s.ad_value(772), s.ad_value(770)), 1.0, 772, 771, (-1.0));
            s.store_mul_ad_lhs(3, A::add_scaled_inputs4(s.ad_value(771), 1.0, A::mul3_scaled_output(s.ad_value(770), s.ad_value(772), s.ad_value(770), 0.5), 1.0, A::mul3_scaled_output(s.ad_value(771), s.ad_value(772), s.ad_value(771), 0.5), -1.0, A::div_from_scalar(0.5, s.ad_value(772)), -1.0), 774);
            s.store_div_scaled_product_left_ad(4, A::sub(s.ad_value(2), s.ad_value(3)), 773, 0.5, 772, 1.0);
        }

        if (s.b[1306] && (!s.b[1307])) {
            s.store_exp_mul_scaled_lhs_indices(2, 770, -1.0, 774);
            s.store_exp_ad(3, A::mul(A::sub(s.ad_value(771), A::div_from_scalar(1.0, s.ad_value(772))), s.ad_value(774)));
            s.store_div_scaled_product_right_ad(4, 773, A::sub(s.ad_value(2), s.ad_value(3)), 1.0, 774, 2.0);
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(780, 1.80485e-35, A::neg(s.ad_value(766)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(766)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(766)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(780, 1.80485e-35, A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), (-80.0)), 0.5, A::scale_offset(A::neg(A::sub(s.ad_value(766), s.ad_value(407))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1306] && (!s.b[1311])) && s.b[1312]) {
            s.store_mul(0, 775, 780);
        }

        if ((s.b[1306] && (!s.b[1311])) && (!s.b[1312])) {
            s.store_add_scaled_inputs3_mixed_aii(780, A::ln(s.ad_value(775)), 1.0, 766, 1.0, 407, (-1.0));
            s.store_mul_sub_from_scalar_ad_rhs(0, 780, 1.0, A::div(A::ln(A::offset(s.ad_value(780), 1.0)), A::offset(s.ad_value(780), 2.0)));
        }

        if s.b[1306] {
            s.copy_ad(777, 0);
            s.store_mul_offset_lhs_ad(778, A::add_scaled_inputs(s.ad_value(776), 0.5, s.ad_value(777), 0.5), 1.0, A::sub(s.ad_value(776), s.ad_value(777)));
            s.store_mul_square_lhs(779, 284, 110);
            s.store_div_scaled_product3_indices(352, 779, 237, 778, 1.0, 418, 1.0);
        }

        s.v[353] = 0.0;

        s.v[354] = 0.0;

        s.b[1314] = (p.p8 != 0.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if s.b[1314] {
            s.store_div_scaled_add_product(753, s.ad_value(335), 1.0, s.ad_value(115), s.ad_value(407), (-1.0), s.ad_value(223), 1.0);
        }

        s.b[1315] = (s.v[753] > 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (s.b[1314] && s.b[1315]) {
            s.store_div_scaled_value_offset_denominator(3, s.ad_value(113), (-1.0), s.ad_value(753), 1e-30, 1.0);
        }

        s.b[1316] = (((s.v[3]) as f64).abs() < 80.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if ((s.b[1314] && s.b[1315]) && s.b[1316]) {
            s.store_exp(0, 3);
        }

        s.b[1317] = (s.v[3] < (-80.0));
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && s.b[1317]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(0, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(3)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1314] && s.b[1315]) && (!s.b[1316])) && (!s.b[1317])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(0, 3, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(3), (-80.0)), 0.5, A::scale_offset(s.ad_value(3), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1314] && s.b[1315]) {
            s.store_mul3_lhs(353, 112, 753, 0);
            s.store_mul_add_rhs(354, 353, 344, 352);
        }

        s.b[1318] = (s.v[6] > 0.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if s.b[1318] {
            s.store_mul_abs_ad_lhs(0, A::mul(A::add(s.ad_value(344), s.ad_value(352)), s.ad_value(332)), 168);
        }

        s.b[1319] = (s.v[0] > (100000000.0 * p.p16));
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (s.b[1318] && s.b[1319]) {
            s.store_div_from_scalar(355, (-(p.p16 + (0.25 / p.p16))), 168);
        }

        if (s.b[1318] && (!s.b[1319])) {
            s.store_div_scaled_inputs_mixed_ai(355, A::offset(A::sub_scaled_inputs(A::offset(s.ad_value(0), p.p16), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(0), (-p.p16), A::offset(s.ad_value(0), (-p.p16))), 1.0)), 0.5), (0.25 / p.p16)), -1.0, 168, 1.0);
        }

        if s.b[1318] {
            s.store_div(356, 215, 168);
        }

        if (!s.b[1318]) {
            s.store_scalar(355, 0.0);
            s.store_scaled_voltage(356, ctx, nodes, Some(4), None, 0.001);
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
            s.store_sub_ad_lhs(1423, A::add_scaled_product(s.ad_value(337), (-1.0), A::sub(s.ad_value(331), s.ad_value(1414)), s.ad_value(223), 1.0), 230);
            s.store_add_scaled_product_left_ad(1424, 337, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1415), 1.0), 223, 1.0);
            s.store_sub(1425, 1424, 230);
        }

        s.b[1605] = (p.p2 > 0.0);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1605]) {
            s.store_scale(0, 16, p.p14);
            s.store_div_scaled_offset_numerator(1426, s.ad_value(242), 1.0, 1.0, A::offset(s.ad_value(243), 1.0), 1.0);
            s.store_ln(1427, 1426);
        }

        s.b[1606] = (s.v[1427] > 1e-8);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1606]) {
            s.store_div_scaled_product_offset_denominator(1428, s.ad_value(1427), A::offset(s.ad_value(1426), 1.0), 2.0, s.ad_value(1426), (-1.0), 1.0);
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
            s.store_add_scaled_product_indices(1432, 1423, 1.0, 1459, 1430, (-1.0));
            s.store_add_scaled_product_indices(1433, 1425, 1.0, 1459, 1431, 1.0);
            s.store_div_from_scalar_offset_input(1338, 1.0, 242, 1.0);
            s.store_div_from_scalar_offset_input(1339, 1.0, 243, 1.0);
            s.store_offset_ln_ad(1341, A::div_scaled_product(A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(1339), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 1.5);
            s.store_offset_ln_ad(1342, A::div_scaled_product(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(242), s.ad_value(1338), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 1.5);
        }

        s.b[1607] = (((s.v[1341] - s.v[1432]) / 1.5) < 80.0);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1607]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.6666666666666666, s.ad_value(1432), 0.6666666666666666));
        }

        if ((s.b[1604] && s.b[1605]) && (!s.b[1607])) {
            s.store_scaled_sub(1340, 1341, 1432, 0.6666666666666666);
        }

        if (s.b[1604] && s.b[1605]) {
            s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 1.5);
            s.store_mul_add_scaled_product_rhs(1344, 1339, s.ad_value(1345), 1.0, s.ad_value(243), s.ad_value(1425), 1.0);
        }

        s.b[1608] = (((s.v[1342] - s.v[1344]) / 1.5) < 80.0);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1608]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.6666666666666666, s.ad_value(1344), 0.6666666666666666));
        }

        if ((s.b[1604] && s.b[1605]) && (!s.b[1608])) {
            s.store_scaled_sub(1340, 1342, 1344, 0.6666666666666666);
        }

        if (s.b[1604] && s.b[1605]) {
            s.store_sub_scaled_inputs(1, 1342, 1.0, 1340, 1.5);
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(1391, 1.80485e-35, A::neg(A::neg(s.ad_value(262))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(262))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1609])) && (!s.b[1610])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1391, A::neg(s.ad_value(262)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(262)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(262)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.b[1611] = (((s.v[1390]) as f64).abs() <= s.v[261]);
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1605]) && s.b[1611]) {
            s.store_scaled_square(1388, 260, (0.1666666666667 * 0.707106781186545));
            s.store_mul_ad_product_rhs(4, 1390, s.ad_value(260), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(1390), 1.0, s.ad_value(1391)), s.ad_value(256), s.ad_value(1388)), 1.0));
        }

        s.b[1612] = (s.v[1390] < (-s.v[261]));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) {
            s.store_neg(1392, 1390);
            s.store_scaled_mul(1393, 1392, 260, 1.25);
            s.store_scaled_sub_ad(1394, A::offset(s.ad_value(1393), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(1393), (-6.0), A::offset(s.ad_value(1393), (-6.0))), 64.0)), 0.5);
            s.store_sub(1387, 1392, 1394);
            s.store_add_scaled_square_product_mixed_iia(1395, 1387, 1.0, 257, A::offset(s.ad_value(1394), 1.0), 1.0);
            s.store_sub_scaled_inputs(1397, 1387, 2.0, 257, 1.0);
            s.store_sub_ad_lhs(1398, A::ln(A::mul(s.ad_value(1395), s.ad_value(258))), 1394);
            s.store_add(1385, 1395, 1397);
            s.store_add_scaled_square_product_mixed_iia(1386, 1385, 1.0, 1398, A::add_scaled_product(s.ad_value(1395), (-1.0), s.ad_value(1397), s.ad_value(1397), 0.5), 1.0);
            s.store_add_ad_rhs(1399, 1394, A::div_scaled_product3(s.ad_value(1395), s.ad_value(1385), s.ad_value(1398), 1.0, A::add(s.ad_value(1386), A::mul3(A::mul3(A::div(s.ad_value(1385), s.ad_value(1386)), s.ad_value(1398), s.ad_value(1398)), s.ad_value(1397), A::sub_scaled_inputs(A::square(s.ad_value(1397)), 0.3333333333333, s.ad_value(1395), 1.0))), 1.0));
        }

        s.b[1613] = (s.v[1399] < 80.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) && s.b[1613]) {
            s.store_exp(1400, 1399);
        }

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) && (!s.b[1613])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(1400, 1399, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(1399), (-80.0)), 0.5, A::scale_offset(s.ad_value(1399), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) {
            s.store_div_from_scalar(1401, 1.0, 1400);
            s.store_div_from_scalar_offset_ad(1387, 1.0, A::square(s.ad_value(1399)), 2.0);
            s.store_mul_square_lhs(1402, 1399, 1387);
            s.store_mul3_affine_lhs(1403, 1399, 1387, 4.0, 0.0, 1387);
            s.store_mul_ad_product_lhs(1404, A::sub_scaled_inputs(s.ad_value(1387), 8.0, s.ad_value(1402), 12.0), s.ad_value(1387), 1387);
            s.store_sub(1387, 1392, 1399);
            s.store_mul(1388, 1391, 1401);
            s.store_add_scaled_product_right_ad(1405, 1387, 2.0, 257, A::add_scaled_inputs3_offset(s.ad_value(1400), 1.0, s.ad_value(1388), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(1391), 1.0, s.ad_value(1403)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(1406, 1387, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1400), 1.0, s.ad_value(1399), (-1.0), s.ad_value(1388), 1.0, (-1.0)), 1.0, s.ad_value(1391), A::sub(A::offset(s.ad_value(1399), (-1.0)), s.ad_value(1402)), 1.0), (-1.0));
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && s.b[1612]) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1387, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(1400), 1.0, s.ad_value(1388), 1.0, s.ad_value(1391), s.ad_value(1404), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1387, 1405, 1.0, 1406, 1387, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(4, 1399, -1.0, A::div(s.ad_value(1406), A::add(s.ad_value(1405), A::sqrt(s.ad_value(1387)))), 2.0);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) {
            s.store_div_from_scalar_offset_scaled_input(1407, 1.0, 256, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(1408, A::mul_scaled_lhs(s.ad_value(259), 1.25, s.ad_value(1407)), (-1.0), 1407);
            s.store_mul_ad_product_rhs(1409, 1390, s.ad_value(260), A::offset(A::mul(s.ad_value(1408), s.ad_value(1390)), 1.0));
        }

        s.b[1614] = ((-s.v[1409]) > (-80.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) && s.b[1614]) {
            s.store_exp_neg_input(1387, 1409);
        }

        if ((((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) && (!s.b[1614])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(1387, 1.80485e-35, A::neg(A::neg(s.ad_value(1409))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(1409))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(1409))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) {
            s.store_sub_from_scalar(1410, 1.0, 1387);
            s.store_add_scaled_inputs_product_right_ad(1411, 1390, 1.0, 257, 0.5, 256, A::sqrt(A::add_scaled_inputs3(s.ad_value(1390), 1.0, s.ad_value(257), 0.25, s.ad_value(1410), -1.0)), (-1.0));
            s.store_offset(1412, 262, 3.0);
            s.store_sub_ad(1394, A::add_scaled_inputs3(s.ad_value(1411), 0.5, s.ad_value(1412), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1411), s.ad_value(1412)), A::sub(s.ad_value(1411), s.ad_value(1412))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(1412), 0.5, A::sqrt(A::offset(A::square(s.ad_value(1412)), 5.0)), 0.5));
            s.store_sub(1387, 1390, 1394);
            s.store_exp_neg_input(1388, 1394);
            s.store_div_from_scalar_offset_ad(1389, 1.0, A::square(s.ad_value(1394)), 2.0);
            s.store_mul_square_lhs(1402, 1394, 1389);
            s.store_mul3_affine_lhs(1403, 1394, 1389, 4.0, 0.0, 1389);
            s.store_mul_ad_product_lhs(1404, A::sub_scaled_inputs(s.ad_value(1389), 8.0, s.ad_value(1402), 12.0), s.ad_value(1389), 1389);
            s.store_max_from_scalar_ad(1395, 1e-40, A::add_scaled_square_product(s.ad_value(1387), 1.0, s.ad_value(257), A::add_scaled_product(A::offset(A::add(s.ad_value(1388), s.ad_value(1394)), (-1.0)), 1.0, s.ad_value(1391), A::add(A::offset(s.ad_value(1394), 1.0), s.ad_value(1402)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1396, 1.0, 257, A::add_scaled_product(s.ad_value(1388), 1.0, s.ad_value(1391), s.ad_value(1404), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(1397, 1387, 2.0, 257, A::add_scaled_sub_value_product(1.0, s.ad_value(1388), 1.0, s.ad_value(1391), A::offset(s.ad_value(1403), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(1398, 262, 1.0, 1394, (-1.0), A::ln(A::div(s.ad_value(1395), s.ad_value(257))), 1.0);
            s.store_add(1385, 1395, 1397);
            s.store_add_scaled_square_product_mixed_iia(1386, 1385, 1.0, 1398, A::add_scaled_products(s.ad_value(1397), s.ad_value(1397), 0.5, s.ad_value(1395), s.ad_value(1396), (-1.0)), 1.0);
            s.store_add_ad_rhs(1413, 1394, A::div_scaled_product3(s.ad_value(1395), s.ad_value(1385), s.ad_value(1398), 1.0, A::add(s.ad_value(1386), A::mul3(A::mul3(A::div(s.ad_value(1385), s.ad_value(1386)), s.ad_value(1398), s.ad_value(1398)), s.ad_value(1397), A::add_scaled_square_product(s.ad_value(1397), 0.3333333333333, s.ad_value(1395), s.ad_value(1396), (-1.0)))), 1.0));
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
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(1400, 1.80485e-35, A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(262), s.ad_value(1413)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(262), s.ad_value(1413)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(1401, 1.80485e-35, 1413, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(1413), (-80.0)), 0.5, A::scale_offset(s.ad_value(1413), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (((s.b[1604] && s.b[1605]) && (!s.b[1611])) && (!s.b[1612])) {
            s.store_div_from_scalar_offset_ad(1387, 1.0, A::square(s.ad_value(1413)), 2.0);
            s.store_mul_square_lhs(1402, 1413, 1387);
            s.store_mul3_affine_lhs(1403, 1413, 1387, 4.0, 0.0, 1387);
            s.store_mul_ad_product_lhs(1404, A::sub_scaled_inputs(s.ad_value(1387), 8.0, s.ad_value(1402), 12.0), s.ad_value(1387), 1387);
            s.store_sub(1387, 1390, 1413);
            s.store_add_scaled_product_right_ad(1405, 1387, 2.0, 257, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(1401)), 1.0, s.ad_value(1400), 1.0, s.ad_value(1391), A::offset(s.ad_value(1403), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(1406, 1387, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(1401), 1.0, s.ad_value(1413), 1.0, s.ad_value(1400), 1.0, (-1.0)), 1.0, s.ad_value(1391), A::add(A::offset(s.ad_value(1413), 1.0), s.ad_value(1402)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1387, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(1401), 1.0, s.ad_value(1400), 1.0, s.ad_value(1391), s.ad_value(1404), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1387, 1405, 1.0, 1406, 1387, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(4, 1413, 1.0, A::div(s.ad_value(1406), A::add(s.ad_value(1405), A::sqrt(s.ad_value(1387)))), 2.0);
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
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1435, 0, 0.5, 253, 0.5, A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253)), 1.0), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1436, 253, 0.5, 0, ((-1.0) * 0.5), A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), 1.0), 0.5);
            s.store_mul_ad_rhs(2, 254, A::exp_scaled_input(A::ln(s.ad_value(1435)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 254, A::exp_scaled_input(A::ln(s.ad_value(1436)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div(1443, 241, 4);
            s.store_offset_mul(1437, 242, 2, 1.0);
            s.store_offset_mul(1438, 243, 3, 1.0);
            s.store_div_scaled_product_indices(1439, 242, 4, 1.0, 1437, 1.0);
            s.store_div_scaled_product_indices(1440, 243, 4, 1.0, 1438, 1.0);
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
            s.store_add_scaled_inputs3_offset_mixed_iai(1444, 1423, 1.0, A::div(s.ad_value(1442), s.ad_value(1439)), (-1.0), 0, 1.0, (-0.6931471805599));
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
            s.store_add_scaled_inputs3_offset_mixed_iai(1444, 1434, 1.0, A::div(s.ad_value(1442), s.ad_value(1440)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        if s.b[1604] {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1445, 1444, 0.5, 250, 0.5, A::offset(A::mul(A::sub(s.ad_value(1444), s.ad_value(250)), A::sub(s.ad_value(1444), s.ad_value(250))), 4.0), (-0.5));
            s.store_offset_sqrt_ad(1446, A::offset(A::div_scaled_inputs2(s.ad_value(250), 2.0, s.ad_value(1445), (-2.0), s.ad_value(251), 1.0), 1.0), (-1.0));
            s.store_add_scaled_product_indices(1447, 1445, 1.0, 251, 1446, 1.0);
            s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(1424)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(1424)), ((1.0) + ((-0.5))))), 0.01)), 0.5);
            s.store_div_from_scalar_offset_ad(1448, 1.0, A::mul(s.ad_value(1416), s.ad_value(0)), 1.0);
            s.store_div_from_scalar_offset_ad(1449, 1.0, A::mul(s.ad_value(1417), s.ad_value(0)), 1.0);
            s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(325), A::offset(A::sqrt(A::offset(A::div(s.ad_value(336), s.ad_value(325)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1446)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1424)), 1.0);
            s.store_mul(1450, 1418, 0);
            s.store_mul(1451, 1419, 0);
            s.store_add_ad_lhs(1452, A::add_scaled_product(s.ad_value(1447), 1.0, A::add_scaled_inputs3(s.ad_value(1423), 1.0, s.ad_value(1447), (-1.0), s.ad_value(1450), 1.0), s.ad_value(1448), 1.0), 337);
            s.store_add_ad_lhs(1453, A::add_scaled_product(s.ad_value(1447), 1.0, A::add_scaled_inputs3(s.ad_value(1434), 1.0, s.ad_value(1447), (-1.0), s.ad_value(1451), 1.0), s.ad_value(1449), 1.0), 337);
        }

        if s.b[1604] {
            let assign30040_ad_e32824: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1453), 1.0, s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)), 1.0), 1.0, s.ad_value(221), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(1453), 1.0, s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)), 1.0), s.ad_value(221)), A::sub(A::add_scaled_product(s.ad_value(1453), 1.0, s.ad_value(25), A::sub(s.ad_value(1452), s.ad_value(1453)), 1.0), s.ad_value(221))), 0.01)), -1.0);
            s.store_scale_ad(1454, assign30040_ad_e32824, 0.5);
        }

        if s.b[1604] {
            let assign30050_ad_e32861: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1452), 1.0, s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)), 1.0), 1.0, s.ad_value(221), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(1452), 1.0, s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)), 1.0), s.ad_value(221)), A::sub(A::add_scaled_product(s.ad_value(1452), 1.0, s.ad_value(26), A::sub(s.ad_value(1453), s.ad_value(1452)), 1.0), s.ad_value(221))), 0.01)), -1.0);
            s.store_scale_ad(1455, assign30050_ad_e32861, 0.5);
        }

        if s.b[1604] {
            s.store_div(1456, 1439, 1448);
            s.store_div(1457, 1440, 1449);
            s.store_div_from_scalar(1430, 1.0, 1456);
            s.store_div_from_scalar(1431, 1.0, 1457);
            s.store_div_from_scalar_add_ad(1458, 1.0, A::offset(s.ad_value(1430), 1.0), s.ad_value(1431));
            s.store_div_ad_rhs(1429, 249, A::square(s.ad_value(1443)));
            s.store_div_scaled_offset_numerator(1426, s.ad_value(1456), 1.0, 1.0, A::offset(s.ad_value(1457), 1.0), 1.0);
            s.store_ln(1427, 1426);
        }

        s.b[1621] = (s.v[1427] > 1e-8);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1621]) {
            s.store_div_scaled_product_offset_denominator(1428, s.ad_value(1427), A::offset(s.ad_value(1426), 1.0), 2.0, s.ad_value(1426), (-1.0), 1.0);
        }

        if (s.b[1604] && (!s.b[1621])) {
            s.store_scaled_offset(1428, 1427, 2.0, 2.0);
        }

        if s.b[1604] {
            s.store_mul_sub_rhs(1459, 1458, 1454, 1455);
            s.store_square(1460, 1459);
            s.store_add_scaled_product_indices(1432, 1454, 1.0, 1459, 1430, (-1.0));
            s.store_add_scaled_product_indices(1433, 1455, 1.0, 1459, 1431, 1.0);
            s.store_div_from_scalar_offset_input(1338, 1.0, 1456, 1.0);
            s.store_div_from_scalar_offset_input(1339, 1.0, 1457, 1.0);
            s.store_offset_ln_ad(1341, A::div_scaled_product(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(1457), s.ad_value(1339), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 3.0);
            s.store_offset_ln_ad(1342, A::div_scaled_product(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1338), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0), 3.0);
        }

        s.b[1622] = (((s.v[1341] - s.v[1432]) * 0.3333333333333) < 80.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1622]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.3333333333333, s.ad_value(1432), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1622])) {
            s.store_scaled_sub(1340, 1341, 1432, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 3.0);
        }

        s.b[1623] = (((s.v[1342] - s.v[1433]) * 0.3333333333333) < 80.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1623]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.3333333333333, s.ad_value(1433), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1623])) {
            s.store_scaled_sub(1340, 1342, 1433, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1346, 1342, 1.0, 1340, 3.0);
            s.store_mul_add_scaled_product_rhs(1343, 1338, s.ad_value(1346), 1.0, s.ad_value(1456), s.ad_value(1454), 1.0);
            s.store_mul_add_scaled_product_rhs(1344, 1339, s.ad_value(1345), 1.0, s.ad_value(1457), s.ad_value(1455), 1.0);
        }

        s.b[1624] = (((s.v[1341] - s.v[1343]) * 0.3333333333333) < 80.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1624]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.3333333333333, s.ad_value(1343), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1624])) {
            s.store_scaled_sub(1340, 1341, 1343, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 3.0);
        }

        s.b[1625] = (((s.v[1342] - s.v[1344]) * 0.3333333333333) < 80.0);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1625]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.3333333333333, s.ad_value(1344), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1625])) {
            s.store_scaled_sub(1340, 1342, 1344, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1346, 1342, 1.0, 1340, 3.0);
            s.store_sub(1461, 1454, 1345);
            s.store_sub(1465, 1455, 1346);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
    ) {
        if s.b[1604] {
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
            s.store_scaled_offset_mul_offset_lhs_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1454), s.ad_value(1461)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1627] = (s.v[1349] < (-0.005));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1627]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1628] = (s.v[1349] > 0.005);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1627])) && s.b[1628]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((s.b[1604] && (!s.b[1627])) && (!s.b[1628])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1629] = (s.v[1349] > 0.005);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1629]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1630] = (s.v[1349] < (-0.005));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1629])) && s.b[1630]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1629])) && (!s.b[1630])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1632] = (s.v[1361] > 0.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1632]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (s.b[1604] && (!s.b[1632])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1461), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
            s.store_add(1461, 1461, 1376);
            s.store_mul(1347, 1456, 1461);
            s.store_mul(1377, 1457, 1465);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);
            s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);
            s.store_sqrt_ad(1381, A::add_scaled_square_product(s.ad_value(1379), 1.0, s.ad_value(1378), s.ad_value(1380), (-4.0)));
            s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1633] = (s.v[1382] > 0.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1633]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(1373, 1382, A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1461), 1.0, 0.0);
            s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);
        }

        let (assign31340_e34303,) = {
    if (s.b[1604] && s.b[1633]) {
        let assign31340_e34299: f64 = (s.v[1454] - s.v[1461]);
        let assign31340_e34301: f64 = (assign31340_e34299 - s.v[1341]);
        (assign31340_e34301,)
    } else {
        (s.v[1383],)
    }
};
        s.v[1383] = assign31340_e34303;

        s.b[1634] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1633]) && s.b[1634]) {
            s.store_sub_div_rhs_indices(1461, 1461, 1373, 1374);
        }

        if s.b[1604] {
            s.store_mul(1347, 1456, 1461);
            s.store_mul(1377, 1457, 1465);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);
            s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);
            s.store_sqrt_ad(1381, A::add_scaled_square_product(s.ad_value(1379), 1.0, s.ad_value(1378), s.ad_value(1380), (-4.0)));
            s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);
        }

        s.b[1635] = (s.v[1349] < (-0.005));
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1635]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);
        }

        s.b[1636] = (s.v[1349] > 0.005);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1635])) && s.b[1636]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);
        }

        if ((s.b[1604] && (!s.b[1635])) && (!s.b[1636])) {
            s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_ad(1354, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1349, 1349, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1370), s.ad_value(1353), 1.0, s.ad_value(1347), s.ad_value(1377), 1.0), 1.0, s.ad_value(1349), 1.0, A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0), 1.0));
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1637] = (s.v[1382] > 0.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1637]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(1373, 1382, A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1461), 1.0, 0.0);
            s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);
        }

        let (assign31610_e34632,) = {
    if (s.b[1604] && s.b[1637]) {
        let assign31610_e34628: f64 = (s.v[1454] - s.v[1461]);
        let assign31610_e34630: f64 = (assign31610_e34628 - s.v[1341]);
        (assign31610_e34630,)
    } else {
        (s.v[1383],)
    }
};
        s.v[1383] = assign31610_e34632;

        s.b[1638] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1637]) && s.b[1638]) {
            s.store_sub_div_rhs_indices(1461, 1461, 1373, 1374);
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
            s.store_scaled_offset_mul_offset_lhs_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1454), s.ad_value(1461)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1640] = (s.v[1349] < (-0.005));
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        if (s.b[1604] && s.b[1640]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1641] = (s.v[1349] > 0.005);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1640])) && s.b[1641]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((s.b[1604] && (!s.b[1640])) && (!s.b[1641])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1642] = (s.v[1349] > 0.005);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1642]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1643] = (s.v[1349] < (-0.005));
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1642])) && s.b[1643]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1642])) && (!s.b[1643])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1645] = (s.v[1361] > 0.0);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1645]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (s.b[1604] && (!s.b[1645])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1461), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
            s.store_add(1461, 1461, 1376);
            s.store_mul(1347, 1456, 1461);
        }

        s.b[1646] = ((s.v[1454] - s.v[1461]) < 80.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1646]) {
            s.store_exp_sub(1338, 1454, 1461);
        }

        if (s.b[1604] && (!s.b[1646])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1454), s.ad_value(1461)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1647] = (s.v[1349] < (-0.005));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1647]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1648] = (s.v[1349] > 0.005);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1647])) && s.b[1648]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((s.b[1604] && (!s.b[1647])) && (!s.b[1648])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1649] = (s.v[1349] > 0.005);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1649]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1650] = (s.v[1349] < (-0.005));
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1649])) && s.b[1650]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1649])) && (!s.b[1650])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_sub_lhs(1361, 1348, 1357, 1339);
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1652] = (s.v[1361] > 0.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1652]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (s.b[1604] && (!s.b[1652])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1461), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1604] {
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
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
            s.store_scaled_offset_mul_offset_lhs_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1454), s.ad_value(1461)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1604] && s.b[1653]) && s.b[1654]) {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1656] = (s.v[1349] < (-0.005));
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1656]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1657] = (s.v[1349] > 0.005);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1656])) && s.b[1657]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1656])) && (!s.b[1657])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1658] = (s.v[1349] > 0.005);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1658]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1659] = (s.v[1349] < (-0.005));
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1658])) && s.b[1659]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1658])) && (!s.b[1659])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1661] = (s.v[1361] > 0.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && s.b[1661]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (((s.b[1604] && s.b[1653]) && s.b[1654]) && (!s.b[1661])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1461);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if ((s.b[1604] && s.b[1653]) && s.b[1654]) {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1461), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
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
            s.store_scaled_offset_mul_offset_lhs_ad(1338, A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1454), s.ad_value(1461)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1454), s.ad_value(1461)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
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
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
        }

        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) {
            s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::scale(s.ad_value(1466), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
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
            s.store_sub_ad_lhs(1339, A::ln(A::div_scaled_inputs(s.ad_value(1466), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0)), 1352);
        }

        s.b[1669] = (s.v[1466] < (-0.005));
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if (((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && (!s.b[1668])) && s.b[1669]) {
            s.store_sin_scaled_input(1340, 1352, 0.5);
            s.store_ln_div_scaled_input_square_denominator(1339, 1466, -1.0, 1340, 1.0);
        }

        if (((((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) && (!s.b[1668])) && (!s.b[1669])) {
            s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::scale(s.ad_value(1466), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((s.b[1604] && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {
            s.store_sub_ad_lhs(1465, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1461), 1.0, A::ln(s.ad_value(1338)), 2.0), 1339);
            s.store_mul(1464, 1457, 1465);
            s.store_add(1462, 1463, 1464);
        }

        s.b[1670] = (s.v[1466] > 0.005);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        s.b[1671] = (((s.v[1461] - s.v[1454]) - s.v[1352]) < 80.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) && s.b[1671]) {
            s.store_exp_ad(1340, A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1352), -1.0));
        }

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) && (!s.b[1671])) {
            let assign34270_ad_e38320: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1352), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1352), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1461), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1352), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1340, assign34270_ad_e38320, 1.0, 5.54062e34);
        }

        if (((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && s.b[1670]) {
            s.store_div(1339, 1340, 1429);
            s.store_div_scaled_product_denominator_ad(1338, 1466, 1339, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
        }

        s.b[1672] = (s.v[1466] < (-0.005));
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && (!s.b[1670])) && s.b[1672]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_value_by_product(1338, s.ad_value(1466), -1.0, A::square(s.ad_value(1339)), s.ad_value(1467), 1.0);
        }

        if ((((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1672])) {
            s.store_div_ad_lhs(1338, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::scale(s.ad_value(1466), 0.0396825396825397), 0.05), 0.3333333333333)), 1467);
        }

        if ((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) {
            s.store_offset_div_scaled_inputs2_mixed_iia(1462, 1463, 1.0, 1353, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1338)), 1.0, 1e-80);
            s.store_sub(1464, 1462, 1463);
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        if ((s.b[1604] && (!s.b[1663])) && (!s.b[1666])) {
            s.store_div(1465, 1464, 1457);
        }

        s.b[1673] = ((s.v[1455] - s.v[1465]) < 80.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1673]) {
            s.store_exp_sub(1338, 1455, 1465);
        }

        if (s.b[1604] && (!s.b[1673])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1338, A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(1455), s.ad_value(1465)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(1455), s.ad_value(1465)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
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
            s.store_add_scaled_inputs(1471, 1469, 1.0, 1463, 2.0);
            s.store_add_scaled_inputs(1472, 1470, 1.0, 1464, 2.0);
            s.store_add_scaled_inputs3_indices(1473, 1462, 2.0, 1469, 1.0, 1470, 1.0);
        }

        s.b[1675] = (((s.v[1466]) as f64).abs() > 0.005);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1674]) && s.b[1675]) {
            s.store_add_scaled_products3(2, s.ad_value(1471), s.ad_value(1472), 1.0, A::offset(s.ad_value(1461), 2.0), s.ad_value(1472), 2.0, A::offset(s.ad_value(1465), 2.0), s.ad_value(1471), 2.0);
            s.store_div_scaled_product_by_product(1474, s.ad_value(1466), s.ad_value(1473), (-4.0), s.ad_value(1462), s.ad_value(2), 1.0);
        }

        if ((s.b[1604] && s.b[1674]) && (!s.b[1675])) {
            s.store_offset_scaled_ad(2, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1466), 1.0, A::scale(s.ad_value(1466), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(1471), s.ad_value(1467), 1.0, s.ad_value(1472), s.ad_value(1468), 1.0, A::mul3(s.ad_value(1471), s.ad_value(1472), s.ad_value(1462)), A::offset(A::mul(s.ad_value(1462), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(1474, s.ad_value(1467), s.ad_value(1468), s.ad_value(1473), 1.0, s.ad_value(1462), s.ad_value(3), 1.0);
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
            s.store_add_scaled_products_indices(1480, 266, 1476, 1.0, 267, 1478, 1.0);
            s.store_add_scaled_products_indices(1481, 266, 1477, 1.0, 267, 1479, 1.0);
            s.store_div_add_scaled_inputs_rhs_indices(0, 1462, 1476, 1.0, 1477, 1.0);
            s.store_mul(1482, 1476, 0);
            s.store_mul(1483, 1477, 0);
            s.store_mul_ad_product_rhs(1484, 1476, s.ad_value(187), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
            s.store_mul_ad_product_rhs(1485, 1477, s.ad_value(188), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
            s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(1478), 1.0, s.ad_value(51), s.ad_value(1479), 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(3, 2, 1.0, 1.0, 0.01, 0.5);
            s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);
            s.store_div(1486, 3, 4);
            s.store_mul_ad_product_rhs(1487, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1478)), 1.0), 1.0, s.ad_value(42), s.ad_value(1479), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1482), s.ad_value(264)), 1.0), 1.0, s.ad_value(1483), s.ad_value(265), 1.0)))));
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
            s.store_mul_ad_affine_product_rhs(1488, 268, s.ad_value(1443), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424))), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(58), s.ad_value(1424)), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(1424)))), 0.01))), 0.5, 0.0);
            s.store_mul_add_scaled_product_rhs(1489, 1488, s.ad_value(54), 1.0, s.ad_value(1462), s.ad_value(4), 1.0);
            s.store_add_scaled_inputs_product_first_ad(1490, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1480)), 1e-6)))), 1.0), 1.0, 1487, 1.0, 38, 1489, 1.0);
            s.store_add_scaled_inputs_product_first_ad(1491, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1481)), 1e-6)))), 1.0), 1.0, 1487, 1.0, 39, 1489, 1.0);
            s.store_div_scaled_product_mixed_iaa(1492, 1486, A::add(s.ad_value(1484), s.ad_value(1485)), 1.0, A::add(A::div(s.ad_value(1484), s.ad_value(1490)), A::div(s.ad_value(1485), s.ad_value(1491))), 1.0);
        }

        s.b[1680] = (((s.v[1459]) as f64).abs() > 0.007);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        s.b[1681] = (s.v[1459] > 0.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1680]) && s.b[1681]) {
            s.store_exp_neg_input(0, 1459);
            s.store_div_ad_rhs(1493, 1459, A::sub_from_scalar(1.0, s.ad_value(0)));
            s.store_mul(1494, 0, 1493);
            s.store_add_offset_ad_lhs(1495, A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1493)))), (-0.6931471805599), 1432);
        }

        if ((s.b[1604] && s.b[1680]) && (!s.b[1681])) {
            s.store_exp(0, 1459);
            s.store_div_scaled_value_offset_denominator(1494, s.ad_value(1459), 1.0, s.ad_value(0), (-1.0), 1.0);
            s.store_mul(1493, 0, 1494);
            s.store_add_offset_ad_lhs(1495, A::ln(A::div(s.ad_value(1429), A::mul(s.ad_value(1462), s.ad_value(1494)))), (-0.6931471805599), 1433);
        }

        if (s.b[1604] && s.b[1680]) {
            s.store_div_scaled_inputs_mixed_ia(1496, 1459, -1.0, A::mul(s.ad_value(1458), A::add_scaled_sub_value_product(1.0, s.ad_value(1493), 1.0, s.ad_value(1459), s.ad_value(1431), (-1.0))), 1.0);
            s.store_div_ad_rhs(1497, 1459, A::mul(s.ad_value(1458), A::add_scaled_sub_value_product(1.0, s.ad_value(1494), 1.0, s.ad_value(1459), s.ad_value(1430), 1.0)));
            s.store_div_add_scaled_inputs_rhs_ad(1498, 1459, A::div_scaled_offset_numerator(A::mul(s.ad_value(1494), s.ad_value(1431)), 1.0, 0.5, s.ad_value(1497), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(1493), s.ad_value(1430)), 1.0, 0.5, s.ad_value(1496), 1.0), -1.0);
        }

        if (s.b[1604] && (!s.b[1680])) {
            s.store_scale(0, 1460, (0.5 * 0.1666666666667));
            s.store_scale(2, 1459, 0.5);
            s.store_add_offset_lhs(1493, 2, 1.0, 0);
            s.store_add_ad_lhs(1494, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
            s.store_scale(3, 2, 0.1666666666667);
            s.store_div_from_scalar_mul_ad(1496, 1.0, s.ad_value(1458), A::add(A::offset(s.ad_value(1431), 0.5), s.ad_value(3)));
            s.store_div_from_scalar_mul_ad(1497, 1.0, s.ad_value(1458), A::sub(A::offset(s.ad_value(1430), 0.5), s.ad_value(3)));
            s.store_add_scaled_inputs3_offset_mixed_aii(1495, A::ln(A::div(s.ad_value(1429), A::mul_sub_from_scalar_rhs(s.ad_value(1462), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 1432, 0.5, 1433, 0.5, (-0.6931471805599));
            s.store_div_from_scalar_ad(1498, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(1458), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(1458), 12.0, A::mul(s.ad_value(1456), s.ad_value(1457)), 1.0), 1.0, A::mul3(s.ad_value(1458), A::sub(s.ad_value(1430), s.ad_value(1431)), s.ad_value(1459)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(1458), 0.25), s.ad_value(1460), 0.3333333333333), 1.0, 4.0));
        }

        if s.b[1604] {
            s.store_div_from_scalar(1499, 1.0, 1498);
        }

        s.b[1682] = (s.v[1462] > 1e-6);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1682]) {
            s.store_div_scaled_value_offset_denominator(1500, s.ad_value(1476), 100.0, s.ad_value(1476), 100.0, 1.0);
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
            s.store_div_scaled_value_offset_denominator(1502, s.ad_value(1477), 100.0, s.ad_value(1477), 100.0, 1.0);
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
            s.store_sub_ad(1504, A::div_scaled_product_by_product(s.ad_value(1474), s.ad_value(1473), 1.0, s.ad_value(1471), s.ad_value(1472), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1467), s.ad_value(1471)), 1.0, A::div(s.ad_value(1468), s.ad_value(1472)), 1.0, s.ad_value(1462), 1.0));
            s.store_div_scaled_product_offset_denominator(1505, s.ad_value(1504), s.ad_value(1462), 1.0, s.ad_value(1504), 1.0, 1.0);
            s.store_sub(2, 1498, 1505);
            s.store_div_scaled_add_product(1506, s.ad_value(1462), 1.0, s.ad_value(1498), s.ad_value(1495), 1.0, s.ad_value(2), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(1506, 1506, 1506, 1e-6, 0.5);
            s.store_scaled_mul_ad(1507, A::div(s.ad_value(1420), s.ad_value(1492)), A::add(s.ad_value(1501), s.ad_value(1503)), 0.5);
            s.store_sub_from_scalar_div_indices(1508, 1.0, 1462, 1505);
            s.store_offset(1509, 1495, 1.0);
            s.store_mul_sub_ad_lhs(1510, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1505), 2.0, s.ad_value(1462), 1.0), s.ad_value(1499)), (-2.0)), s.ad_value(1495), 1506);
        }

        s.b[1685] = (s.v[1507] > 1e-14);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1682]) && s.b[1685]) {
            s.store_div_from_scalar_square_ad(1511, 2.0, s.ad_value(1507));
            s.store_mul(1512, 1511, 1508);
            s.store_add(1513, 1511, 1510);
            s.store_mul(1514, 1511, 1509);
            s.store_sqrt_offset_ad(1515, A::add(A::square(s.ad_value(1512)), A::mul3_scaled_output(s.ad_value(1511), s.ad_value(1511), s.ad_value(1511), 0.148148148148)), 1e-20);
            s.store_sqrt_offset_ad(1516, A::add(A::square(s.ad_value(1514)), A::mul3_scaled_output(s.ad_value(1513), s.ad_value(1513), s.ad_value(1513), 0.148148148148)), 1e-20);
            s.store_sub_ad(1517, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1515), s.ad_value(1512)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1515), s.ad_value(1512)), 0.5), 0.3333333333333));
            s.store_sub_ad(1518, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(1516), s.ad_value(1514)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(1516), s.ad_value(1514)), 0.5), 0.3333333333333));
        }

        if ((s.b[1604] && s.b[1682]) && (!s.b[1685])) {
            s.copy_ad(1517, 1508);
            s.copy_ad(1518, 1509);
        }

        if (s.b[1604] && s.b[1682]) {
            s.store_square(4, 2);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1519, 1517, (0.94 * 0.5), 1518, (0.94 * 0.5), A::add_scaled_product(s.ad_value(4), 10.0, A::sub(s.ad_value(1517), s.ad_value(1518)), A::sub(s.ad_value(1517), s.ad_value(1518)), 1.0), (0.94 * 0.5));
            s.store_add_scaled_product_indices(1520, 1462, 1.0, 1505, 1519, 1.0);
            s.store_mul_sub_rhs(1521, 1498, 1519, 1495);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1522, 1520, 0.5, 1521, 0.5, A::add_scaled_product(s.ad_value(4), 36.0, A::sub(s.ad_value(1520), s.ad_value(1521)), A::sub(s.ad_value(1520), s.ad_value(1521)), 1.0), 0.5);
        }

        if (s.b[1604] && (!s.b[1682])) {
            s.copy_ad(1505, 1498);
            s.store_scaled_offset(1519, 1495, 1.0, 0.94);
            s.store_add_scaled_product_right_ad(1522, 1462, 0.5, 1498, A::sub_scaled_inputs(s.ad_value(1519), 1.0, s.ad_value(1495), 0.5), 1.0);
        }

        s.b[1686] = ((s.v[1522] - 0.5) < 80.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1686]) {
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(1522), (-0.5)));
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
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));
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
            s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(221), s.ad_value(4)));
        }

        if (s.b[1604] && (!s.b[1688])) {
            s.store_sub(2, 221, 4);
        }

        if s.b[1604] {
            s.store_sub(1523, 221, 2);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        if s.b[1604] {
            s.store_div(2, 335, 1523);
            s.store_square(3, 2);
            s.store_square(4, 3);
            s.store_square(5, 4);
            s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(1421), s.ad_value(4)), 1.0)), 2.666666666667);
            s.store_mul_ad_rhs(1524, 335, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));
            s.store_div_from_scalar_offset_input(1338, 1.0, 1456, 1.0);
            s.store_div_from_scalar_offset_input(1339, 1.0, 1457, 1.0);
            s.store_offset_add_ad(1341, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1456), 1.0, s.ad_value(1457), s.ad_value(1339), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0)), s.ad_value(1524), 3.0);
            s.store_offset_add_ad(1342, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1338), 1.0), s.ad_value(1428), 1.0, s.ad_value(1429), 1.0)), s.ad_value(1524), 3.0);
        }

        s.b[1689] = (((s.v[1341] - s.v[1432]) * 0.3333333333333) < 80.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1689]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.3333333333333, s.ad_value(1432), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1689])) {
            s.store_scaled_sub(1340, 1341, 1432, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 3.0);
        }

        s.b[1690] = (((s.v[1342] - s.v[1433]) * 0.3333333333333) < 80.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1690]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.3333333333333, s.ad_value(1433), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1690])) {
            s.store_scaled_sub(1340, 1342, 1433, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1346, 1342, 1.0, 1340, 3.0);
            s.store_mul_add_scaled_product_rhs(1343, 1338, s.ad_value(1346), 1.0, s.ad_value(1456), s.ad_value(1454), 1.0);
            s.store_mul_add_scaled_product_rhs(1344, 1339, s.ad_value(1345), 1.0, s.ad_value(1457), s.ad_value(1455), 1.0);
        }

        s.b[1691] = (((s.v[1341] - s.v[1343]) * 0.3333333333333) < 80.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1691]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1341), 0.3333333333333, s.ad_value(1343), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1691])) {
            s.store_scaled_sub(1340, 1341, 1343, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1345, 1341, 1.0, 1340, 3.0);
        }

        s.b[1692] = (((s.v[1342] - s.v[1344]) * 0.3333333333333) < 80.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1692]) {
            s.store_ln_one_plus_exp_ad(1340, A::sub_scaled_inputs(s.ad_value(1342), 0.3333333333333, s.ad_value(1344), 0.3333333333333));
        }

        if (s.b[1604] && (!s.b[1692])) {
            s.store_scaled_sub(1340, 1342, 1344, 0.3333333333333);
        }

        if s.b[1604] {
            s.store_sub_scaled_inputs(1346, 1342, 1.0, 1340, 3.0);
            s.store_sub(1525, 1454, 1345);
            s.store_sub(1526, 1455, 1346);
            s.store_scalar(1352, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_mul(1347, 1456, 1525);
        }

        s.b[1693] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1693]) {
            s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));
        }

        if (s.b[1604] && (!s.b[1693])) {
            let assign36020_ad_e40412: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1338, assign36020_ad_e40412, 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1694] = (s.v[1349] < (-0.005));
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1694]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1695] = (s.v[1349] > 0.005);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1694])) && s.b[1695]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((s.b[1604] && (!s.b[1694])) && (!s.b[1695])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1696] = (s.v[1349] > 0.005);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1696]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1697] = (s.v[1349] < (-0.005));
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1696])) && s.b[1697]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1696])) && (!s.b[1697])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1699] = (s.v[1361] > 0.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1699]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (s.b[1604] && (!s.b[1699])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1525), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
            s.store_add(1525, 1525, 1376);
            s.store_mul(1347, 1456, 1525);
            s.store_mul(1377, 1457, 1526);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);
            s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);
            s.store_sqrt_ad(1381, A::add_scaled_square_product(s.ad_value(1379), 1.0, s.ad_value(1378), s.ad_value(1380), (-4.0)));
            s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1700] = (s.v[1382] > 0.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1700]) {
            s.store_mul_ad_rhs(1373, 1382, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1525), 1.0));
            s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);
        }

        let (assign36860_e41423,) = {
    if (s.b[1604] && s.b[1700]) {
        let assign36860_e41419: f64 = (s.v[1454] - s.v[1525]);
        let assign36860_e41421: f64 = (assign36860_e41419 - s.v[1341]);
        (assign36860_e41421,)
    } else {
        (s.v[1383],)
    }
};
        s.v[1383] = assign36860_e41423;

        s.b[1701] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1700]) && s.b[1701]) {
            s.store_sub_div_rhs_indices(1525, 1525, 1373, 1374);
        }

        if s.b[1604] {
            s.store_mul(1347, 1456, 1525);
            s.store_mul(1377, 1457, 1526);
            s.store_add(1370, 1347, 1377);
            s.store_offset_scaled(1378, 1370, 0.065345483024, 1.0);
            s.store_add_scaled_product_value_ad(1379, A::scale_offset(s.ad_value(1370), 8.5797362674, 39.478417604), 1.0, 1347, 1377, 1.0);
            s.store_add_scaled_product_indices(1380, 1370, (2.0 * 39.478417604), 1347, 1377, 39.478417604);
            s.store_sqrt_ad(1381, A::add_scaled_square_product(s.ad_value(1379), 1.0, s.ad_value(1378), s.ad_value(1380), (-4.0)));
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        if s.b[1604] {
            s.store_div_scaled_inputs2_indices(1349, 1381, 1.0, 1379, (-1.0), 1378, 2.0);
        }

        s.b[1702] = (s.v[1349] < (-0.005));
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1702]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);
        }

        s.b[1703] = (s.v[1349] > 0.005);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1702])) && s.b[1703]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(1354, 1349, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 0.25, 1349, 1.0);
        }

        if ((s.b[1604] && (!s.b[1702])) && (!s.b[1703])) {
            s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_ad(1354, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
        }

        if s.b[1604] {
            s.store_sub_ad_rhs(1349, 1349, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(1370), s.ad_value(1353), 1.0, s.ad_value(1347), s.ad_value(1377), 1.0), 1.0, s.ad_value(1349), 1.0, A::offset(A::mul(s.ad_value(1370), s.ad_value(1354)), 1.0), 1.0));
            s.store_sub_ad_lhs(1382, A::square(s.ad_value(1347)), 1349);
        }

        s.b[1704] = (s.v[1382] > 0.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1704]) {
            s.store_mul_ad_rhs(1373, 1382, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(1382), s.ad_value(1429))), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1525), 1.0));
            s.store_add_scaled_product_indices(1374, 1382, 1.0, 1456, 1347, 2.0);
        }

        let (assign37130_e41752,) = {
    if (s.b[1604] && s.b[1704]) {
        let assign37130_e41748: f64 = (s.v[1454] - s.v[1525]);
        let assign37130_e41750: f64 = (assign37130_e41748 - s.v[1341]);
        (assign37130_e41750,)
    } else {
        (s.v[1383],)
    }
};
        s.v[1383] = assign37130_e41752;

        s.b[1705] = ((((s.v[1373] < 0.0) && (s.v[1374] > 0.0)) && (((s.v[1383] + 2.3025850929941) + ((s.v[1456]) as f64).ln()) > 0.0)) || (s.v[1383] > 1.0));
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1704]) && s.b[1705]) {
            s.store_sub_div_rhs_indices(1525, 1525, 1373, 1374);
        }

        if s.b[1604] {
            s.store_mul(1347, 1456, 1525);
        }

        s.b[1706] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1706]) {
            s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));
        }

        if (s.b[1604] && (!s.b[1706])) {
            let assign37190_ad_e41845: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1338, assign37190_ad_e41845, 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1707] = (s.v[1349] < (-0.005));
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1707]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1708] = (s.v[1349] > 0.005);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1707])) && s.b[1708]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((s.b[1604] && (!s.b[1707])) && (!s.b[1708])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1709] = (s.v[1349] > 0.005);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1709]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1710] = (s.v[1349] < (-0.005));
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1709])) && s.b[1710]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1709])) && (!s.b[1710])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1712] = (s.v[1361] > 0.0);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1712]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (s.b[1604] && (!s.b[1712])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1525), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
            s.store_add(1525, 1525, 1376);
            s.store_mul(1347, 1456, 1525);
        }

        s.b[1713] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1713]) {
            s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));
        }

        if (s.b[1604] && (!s.b[1713])) {
            let assign37940_ad_e42794: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1338, assign37940_ad_e42794, 1.0, 5.54062e34);
        }

        if s.b[1604] {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1714] = (s.v[1349] < (-0.005));
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1714]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1715] = (s.v[1349] > 0.005);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1714])) && s.b[1715]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((s.b[1604] && (!s.b[1714])) && (!s.b[1715])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1604] && (!s.b[1714])) && (!s.b[1715])) {
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1716] = (s.v[1349] > 0.005);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1716]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1717] = (s.v[1349] < (-0.005));
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if ((s.b[1604] && (!s.b[1716])) && s.b[1717]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((s.b[1604] && (!s.b[1716])) && (!s.b[1717])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1719] = (s.v[1361] > 0.0);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1719]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (s.b[1604] && (!s.b[1719])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if s.b[1604] {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1525), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
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
            s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));
        }

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1722])) {
            let assign38710_ad_e43762: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1338, assign38710_ad_e43762, 1.0, 5.54062e34);
        }

        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {
            s.store_mul(1348, 1429, 1338);
            s.store_sub_ad_lhs(1349, A::square(s.ad_value(1347)), 1348);
            s.store_add_scaled_product_indices(1350, 1348, 1.0, 1456, 1347, 2.0);
            s.store_add_scaled_product_indices(1351, 1348, (-1.0), 1456, 1456, 2.0);
        }

        s.b[1723] = (s.v[1349] < (-0.005));
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1723]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_div_ad_rhs(1353, 1352, A::tan(A::scale(s.ad_value(1352), 0.5)));
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        s.b[1724] = (s.v[1349] > 0.005);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && s.b[1724]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1349));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
            s.store_div_scaled_inputs_indices(1338, 1350, 0.25, 1349, 1.0);
            s.store_mul_add_ad_lhs(1354, s.ad_value(1349), A::mul_sub_from_scalar_rhs(s.ad_value(1353), 2.0, s.ad_value(1353)), 1338);
            s.store_add_scaled_product_mixed_aai(1356, A::div_scaled_product(s.ad_value(1354), s.ad_value(1351), 1.0, s.ad_value(1350), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(1350), 1.0, s.ad_value(1354), s.ad_value(1353), 1.0, (-2.0)), 1338, 1.0);
            s.store_sub_from_scalar_scaled_input(1339, 1.0, 1353, 0.5);
            s.store_mul_div_lhs(1359, 1350, 1349, 1339);
            s.store_div_ad_lhs(1360, A::add_scaled_products(s.ad_value(1351), s.ad_value(1339), 1.0, s.ad_value(1350), A::add_scaled_inputs(s.ad_value(1359), 1.0, s.ad_value(1354), 0.5), (-1.0)), 1349);
        }

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1723])) && (!s.b[1724])) {
            s.store_offset_scaled_ad(1340, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(1353, 1349, 1340, 2.0);
            s.store_offset_scaled_ad(1338, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(1354, 1350, 1338);
            s.store_offset_scaled_ad(1339, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(1356, 1351, 1338, 1.0, A::square(s.ad_value(1350)), 1339, (-1.0));
            s.store_scaled_mul(1359, 1350, 1340, (-0.5));
            s.store_add_scaled_product_value_ad(1360, A::mul3_scaled_output(s.ad_value(1350), s.ad_value(1350), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 2.0, A::scale(s.ad_value(1349), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 1351, 1340, (-0.5));
        }

        s.b[1725] = (s.v[1349] > 0.005);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1725]) {
            s.store_div_scaled_inputs_mixed_ia(1339, 1349, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
            s.store_mul(1357, 1339, 1355);
            s.store_sub_ad_lhs(1358, A::ln(s.ad_value(1339)), 1352);
        }

        s.b[1726] = (s.v[1349] < (-0.005));
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1725])) && s.b[1726]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_inputs_mixed_ia(1357, 1349, -1.0, A::square(s.ad_value(1339)), 1.0);
            s.store_ln(1358, 1357);
        }

        if ((((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1725])) && (!s.b[1726])) {
            s.store_sub_from_scalar_ad(1357, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1349), 1.0, A::scale(s.ad_value(1349), 0.0396825396825397), 0.05), 0.3333333333333));
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
            s.store_mul_ad_lhs(1362, A::add_scaled_value_products(s.ad_value(1348), (-1.0), s.ad_value(1340), s.ad_value(1361), 1.0, s.ad_value(1359), s.ad_value(1357), (-1.0)), 1339);
            s.store_mul_ad_lhs(1363, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(1356), s.ad_value(1361), 1.0, s.ad_value(1340), s.ad_value(1362), 2.0), 1.0, s.ad_value(1348), 1.0, A::add(s.ad_value(1360), A::square(s.ad_value(1359))), s.ad_value(1357), (-1.0)), 1339);
        }

        s.b[1728] = (s.v[1361] > 0.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && s.b[1728]) {
            s.store_ln(1364, 1361);
            s.store_div_from_scalar(1338, 1.0, 1361);
            s.store_mul(1365, 1362, 1338);
            s.store_add_scaled_square_product_indices(1366, 1365, (-1.0), 1363, 1338, 1.0);
        }

        if (((s.b[1604] && s.b[1720]) && s.b[1721]) && (!s.b[1728])) {
            s.store_add_offset_lhs_ad_rhs(1364, 1347, 0.6931471805599, A::ln_scaled_input(s.ad_value(1347), -1.0));
            s.store_div_from_scalar(1338, 1.0, 1525);
            s.store_add(1365, 1456, 1338);
            s.store_mul_neg_lhs(1366, 1338, 1338);
        }

        if ((s.b[1604] && s.b[1720]) && s.b[1721]) {
            s.store_sub_ad_lhs(1367, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1525), 1.0, s.ad_value(1364), 2.0), 1358);
            s.store_sub_ad_lhs(1368, A::scale_offset(s.ad_value(1365), 2.0, 1.0), 1359);
            s.store_sub_scaled_inputs(1369, 1366, 2.0, 1360, 1.0);
            s.store_add_scaled_product_indices(1370, 1347, 1.0, 1457, 1367, 1.0);
            s.store_add_scaled_product_indices(1371, 1456, 1.0, 1457, 1368, 1.0);
            s.store_mul(1372, 1457, 1369);
            s.store_add_scaled_product_indices(1373, 1348, (-1.0), 1370, 1361, 1.0);
            s.store_add_ad_lhs(1374, A::add_scaled_products(s.ad_value(1371), s.ad_value(1361), 1.0, s.ad_value(1370), s.ad_value(1362), 1.0), 1348);
            s.store_sub_ad_lhs(1375, A::add_scaled_products3(s.ad_value(1372), s.ad_value(1361), 1.0, s.ad_value(1371), s.ad_value(1362), 2.0, s.ad_value(1370), s.ad_value(1363), 1.0), 1348);
            s.store_add_scaled_square_product_indices(1384, 1374, 1.0, 1373, 1375, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(1376, 1373, 1374, 1384, -1.0, A::offset(A::square(s.ad_value(1384)), 1e-200), 1.0);
            s.store_add(1525, 1525, 1376);
        }

        if s.b[1604] {
            s.store_mul(1528, 1456, 1525);
        }

        s.b[1729] = (((s.v[1454] - s.v[1525]) - s.v[1524]) < 80.0);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1729]) {
            s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0));
        }

        if (s.b[1604] && (!s.b[1729])) {
            let assign39460_ad_e44971: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1454), 1.0, s.ad_value(1525), (-1.0), s.ad_value(1524), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1338, assign39460_ad_e44971, 1.0, 5.54062e34);
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

    }

    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1732] = (s.v[1530] > 0.005);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1731])) && s.b[1732]) {
            s.store_sqrt_abs_ad(1352, s.ad_value(1530));
            s.store_exp_neg_input(1355, 1352);
            s.store_div_scaled_product_offset_rhs(1353, s.ad_value(1352), s.ad_value(1355), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(1355)), 1.0);
        }

        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_offset_ad(1353, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::scale(s.ad_value(1530), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
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
            s.store_sub_ad_lhs(1339, A::ln(A::div_scaled_inputs(s.ad_value(1530), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0)), 1352);
        }

        s.b[1736] = (s.v[1530] < (-0.005));
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if (((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && (!s.b[1735])) && s.b[1736]) {
            s.store_sin_scaled_input(1340, 1352, 0.5);
            s.store_ln_div_scaled_input_square_denominator(1339, 1530, -1.0, 1340, 1.0);
        }

        if (((((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1736])) {
            s.store_ln_ad(1339, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::scale(s.ad_value(1530), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((s.b[1604] && (!s.b[1730])) && s.b[1733]) && (!s.b[1734])) {
            s.store_sub_ad_lhs(1526, A::add_scaled_inputs4(s.ad_value(1455), 1.0, s.ad_value(1454), (-1.0), s.ad_value(1525), 1.0, A::ln(s.ad_value(1338)), 2.0), 1339);
            s.store_mul(1529, 1457, 1526);
            s.store_add(1527, 1528, 1529);
        }

        s.b[1737] = (s.v[1530] > 0.005);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        s.b[1738] = ((((s.v[1525] + s.v[1524]) - s.v[1454]) - s.v[1352]) < 80.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) && s.b[1738]) {
            s.store_exp_ad(1340, A::add_scaled_inputs4(s.ad_value(1525), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1352), -1.0));
        }

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) && (!s.b[1738])) {
            let assign39790_ad_e45440: A = A::mul_offset_lhs(A::add_scaled_inputs4(s.ad_value(1525), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1352), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs4(s.ad_value(1525), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1352), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs4(s.ad_value(1525), 1.0, s.ad_value(1524), 1.0, s.ad_value(1454), -1.0, s.ad_value(1352), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1340, assign39790_ad_e45440, 1.0, 5.54062e34);
        }

        if (((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && s.b[1737]) {
            s.store_div(1339, 1340, 1429);
            s.store_div_scaled_product_denominator_ad(1338, 1530, 1339, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1355), 2.0, s.ad_value(1355))), 1.0);
        }

        s.b[1739] = (s.v[1530] < (-0.005));
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && (!s.b[1737])) && s.b[1739]) {
            s.store_sin_scaled_input(1339, 1352, 0.5);
            s.store_div_scaled_value_by_product(1338, s.ad_value(1530), -1.0, A::square(s.ad_value(1339)), s.ad_value(1531), 1.0);
        }

        if ((((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) && (!s.b[1737])) && (!s.b[1739])) {
            s.store_div_ad_lhs(1338, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::scale(s.ad_value(1530), 0.0396825396825397), 0.05), 0.3333333333333)), 1531);
        }

        if ((s.b[1604] && (!s.b[1730])) && (!s.b[1733])) {
            s.store_offset_div_scaled_inputs2_mixed_iia(1527, 1528, 1.0, 1353, (-1.0), A::sub_from_scalar(1.0, s.ad_value(1338)), 1.0, 1e-80);
            s.store_sub(1529, 1527, 1528);
            s.store_div(1526, 1529, 1457);
        }

        s.b[1740] = (((s.v[1455] - s.v[1526]) - s.v[1524]) < 80.0);
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1740]) {
            s.store_exp_ad(1338, A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(1526), (-1.0), s.ad_value(1524), -1.0));
        }

        if (s.b[1604] && (!s.b[1740])) {
            let assign39910_ad_e45657: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(1526), (-1.0), s.ad_value(1524), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(1526), (-1.0), s.ad_value(1524), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(1455), 1.0, s.ad_value(1526), (-1.0), s.ad_value(1524), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(1338, assign39910_ad_e45657, 1.0, 5.54062e34);
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
            s.store_add_scaled_inputs(1535, 1533, 1.0, 1528, 2.0);
            s.store_add_scaled_inputs(1536, 1534, 1.0, 1529, 2.0);
            s.store_add_scaled_inputs3_indices(1537, 1527, 2.0, 1533, 1.0, 1534, 1.0);
        }

        s.b[1742] = (((s.v[1530]) as f64).abs() > 0.005);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1741]) && s.b[1742]) {
            s.store_add_scaled_products3(2, s.ad_value(1535), s.ad_value(1536), 1.0, A::offset(s.ad_value(1525), 2.0), s.ad_value(1536), 2.0, A::offset(s.ad_value(1526), 2.0), s.ad_value(1535), 2.0);
            s.store_div_scaled_product_by_product(1538, s.ad_value(1530), s.ad_value(1537), (-4.0), s.ad_value(1527), s.ad_value(2), 1.0);
        }

        if ((s.b[1604] && s.b[1741]) && (!s.b[1742])) {
            s.store_offset_scaled_ad(2, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1530), 1.0, A::scale(s.ad_value(1530), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(1535), s.ad_value(1531), 1.0, s.ad_value(1536), s.ad_value(1532), 1.0, A::mul3(s.ad_value(1535), s.ad_value(1536), s.ad_value(1527)), A::offset(A::mul(s.ad_value(1527), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(1538, s.ad_value(1531), s.ad_value(1532), s.ad_value(1537), 1.0, s.ad_value(1527), s.ad_value(3), 1.0);
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
            s.store_div_scaled_inputs2_indices(1542, 1463, 0.5, 1528, 0.5, 1456, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(1542, 1542, 1e-5, (-1e-5), 1.0, 0.5);
            s.store_sub_scaled_ad_lhs(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(1542), s.ad_value(223)), 1.0, s.ad_value(246), s.ad_value(246), 0.25)), 246, 0.5);
            s.store_mul_powf_ad_lhs(1543, s.ad_value(1), 2.0, 223);
            s.store_sub_from_scalar_div_indices(1544, 1.0, 1543, 1542);
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
            s.store_add_scaled_products_indices(1549, 266, 1545, 1.0, 267, 1547, 1.0);
            s.store_add_scaled_products_indices(1550, 266, 1546, 1.0, 267, 1548, 1.0);
            s.store_scaled_add(1551, 1476, 1545, 0.5);
            s.store_scaled_add(1552, 1477, 1546, 0.5);
            s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1551), s.ad_value(1552));
            s.store_mul3_lhs(1553, 1540, 1551, 0);
            s.store_mul3_lhs(1554, 1540, 1552, 0);
            s.store_scaled_add(1555, 1478, 1547, 0.5);
            s.store_scaled_add(1556, 1479, 1548, 0.5);
            s.store_scaled_add(1557, 1480, 1549, 0.5);
            s.store_scaled_add(1558, 1481, 1550, 0.5);
            s.store_mul_product3_rhs(1559, 1544, s.ad_value(1551), s.ad_value(187), A::exp(A::mul(s.ad_value(40), s.ad_value(291))), 1.0);
            s.store_mul_ad_product_rhs(1560, 1552, s.ad_value(188), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));
            s.store_add(1561, 1559, 1560);
            s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(1555), 1.0, s.ad_value(51), s.ad_value(1556), 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(3, 2, 1.0, 1.0, 0.01, 0.5);
            s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);
            s.store_div(1562, 3, 4);
            s.store_mul_ad_product_rhs(1563, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1555)), 1.0), 1.0, s.ad_value(42), s.ad_value(1556), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1553), s.ad_value(264)), 1.0), 1.0, s.ad_value(1554), s.ad_value(265), 1.0)))));
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
            s.store_mul_add_scaled_product_rhs(1564, 1488, s.ad_value(54), 1.0, s.ad_value(1540), s.ad_value(4), 1.0);
            s.store_add_scaled_inputs_product_first_ad(1565, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1557)), 1e-6)))), 1.0), 1.0, 1563, 1.0, 38, 1564, 1.0);
            s.store_add_scaled_inputs_product_first_ad(1566, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1558)), 1e-6)))), 1.0), 1.0, 1563, 1.0, 39, 1564, 1.0);
            s.store_div_scaled_product_add_scaled_denominator(1567, 1562, 1561, 1.0, A::div(s.ad_value(1559), s.ad_value(1565)), 1.0, A::div(s.ad_value(1560), s.ad_value(1566)), 1.0, 1.0);
            s.store_div_from_scalar_offset_input(1568, 1.0, 1540, 4.0);
        }

        s.b[1748] = (s.v[65] > 0.0);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1748]) {
            s.store_div_from_scalar_offset_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1554)), 1.0);
        }

        if (s.b[1604] && (!s.b[1748])) {
            s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1554, 1.0);
        }

        if s.b[1604] {
            s.store_mul3_lhs(1569, 1540, 1568, 0);
            s.store_mul_ln_ad_lhs(1570, A::offset(A::div_scaled_inputs2(s.ad_value(335), 1.0, s.ad_value(1524), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(1540), s.ad_value(1540)), 1.0, s.ad_value(66), s.ad_value(223), 1.0), 1.0), 1.0), 1569);
            s.store_mul(1571, 1422, 1570);
            s.store_div_from_scalar_offset_ad(1572, 1.0, A::mul_offset_rhs(s.ad_value(1571), s.ad_value(1571), 1.0), 1.0);
            s.store_div_scaled_value_offset_denominator(1500, s.ad_value(1551), 100.0, s.ad_value(1551), 100.0, 1.0);
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
            s.store_div_scaled_value_offset_denominator(1502, s.ad_value(1552), 100.0, s.ad_value(1552), 100.0, 1.0);
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
            s.store_mul_ad_affine_product_rhs(1573, 1420, s.ad_value(1541), A::add(s.ad_value(1501), s.ad_value(1503)), 0.5, 0.0);
            s.store_div_ad_rhs(1574, 1573, A::mul(s.ad_value(1567), s.ad_value(1572)));
            s.store_square(1575, 1574);
            s.store_sqrt_offset_input(1576, 1575, 1.0);
            s.store_div_scaled_offset_numerator(1577, s.ad_value(1575), 1.5, 1.0, s.ad_value(1576), 1.0);
        }

        s.b[1751] = (p.p13 > 0.0);
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if (s.b[1604] && s.b[1751]) {
            s.store_mul_scaled_exp_ln_input_rhs(2, 254, 0.6, A::offset(A::square(s.ad_value(1551)), 60.0), (-0.1666666666667));
            s.store_mul_scaled_exp_ln_input_rhs(3, 254, 0.6, A::offset(A::square(s.ad_value(1552)), 60.0), (-0.1666666666667));
            s.store_div_scaled_offset_numerator(1578, A::mul(s.ad_value(1456), s.ad_value(2)), 1.0, 1.0, s.ad_value(1437), 1.0);
            s.store_div_scaled_offset_numerator(1579, A::mul(s.ad_value(1457), s.ad_value(3)), 1.0, 1.0, s.ad_value(1438), 1.0);
        }

        if (s.b[1604] && (!s.b[1751])) {
            s.store_scalar(1578, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1604] && (!s.b[1751])) {
            s.store_scalar(1579, 1.0);
        }

        s.b[1752] = (s.v[1462] > 1e-6);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        s.b[1753] = (s.v[1527] > 1e-6);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        s.b[1754] = (((s.v[1536]) as f64).abs() < 0.01);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (((s.b[1604] && s.b[1752]) && s.b[1753]) && s.b[1754]) {
            s.store_div_scaled_inputs2_mixed_aia(0, A::offset(s.ad_value(1525), 2.0), 1.0, 1535, 0.5, A::mul_offset_lhs(s.ad_value(1526), 2.0, s.ad_value(1535)), 1.0);
            s.store_mul(2, 0, 1536);
            s.store_square(3, 2);
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));
            s.store_div_scaled_inputs2_mixed_iaa(2, 1529, 1.0, A::mul3_scaled_output(s.ad_value(1530), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(1535))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(1526), 2.0), 1.0);
            s.store_div_scaled_inputs2_mixed_aii(1580, A::div_scaled_add_product(s.ad_value(1531), (-1.0), s.ad_value(1538), s.ad_value(1527), 1.0, s.ad_value(1535), 1.0), 1.0, 2, (-1.0), 1527, 1.0);
            s.store_div_scaled_product_offset_denominator(1581, s.ad_value(1580), s.ad_value(1527), 1.0, s.ad_value(1580), 1.0, 1.0);
        }

        if (((s.b[1604] && s.b[1752]) && s.b[1753]) && (!s.b[1754])) {
            s.store_sub_ad(1580, A::div_scaled_product_by_product(s.ad_value(1538), s.ad_value(1537), 1.0, s.ad_value(1535), s.ad_value(1536), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(1531), s.ad_value(1535)), 1.0, A::div(s.ad_value(1532), s.ad_value(1536)), 1.0, s.ad_value(1527), 1.0));
            s.store_div_scaled_product_offset_denominator(1581, s.ad_value(1580), s.ad_value(1527), 1.0, s.ad_value(1580), 1.0, 1.0);
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
            s.store_add_scaled_product_indices(1582, 4, 1.0, 1581, 1541, (-1.0));
            s.store_add_scaled_product_indices(1583, 4, 1.0, 1505, 1541, (-1.0));
            s.store_sqrt_square_add(1584, 1582, 3);
            s.store_sqrt_square_add(1585, 1583, 3);
            s.store_mul_ad(1586, A::div_from_scalar(0.25, s.ad_value(2)), A::add_scaled_products3(s.ad_value(1585), s.ad_value(1582), 1.0, s.ad_value(1584), s.ad_value(1583), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1583), 1.0, s.ad_value(1585), 1.0, A::add(s.ad_value(1582), s.ad_value(1584)), 1.0)), 1.0));
        }

        if ((s.b[1604] && s.b[1752]) && (!s.b[1755])) {
            s.store_mul(4, 1541, 2);
            s.store_div_scaled_product3_mixed_iiia(1586, 1541, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);
        }

        if (s.b[1604] && (!s.b[1752])) {
            s.copy_ad(1581, 1498);
            s.store_scalar(1586, 0.0);
        }

        if s.b[1604] {
            s.store_add_scaled_inputs3_mixed_aii(1587, A::add_scaled_product(s.ad_value(1586), 1.0, s.ad_value(1540), s.ad_value(1541), 1.0), 1.0, 1462, 1.0, 1527, -1.0);
        }

        s.b[1756] = (s.v[1462] > 1e-6);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        s.b[1757] = (s.v[1587] > 1e-30);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((s.b[1604] && s.b[1756]) && s.b[1757]) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1588, 1471, A::div(s.ad_value(1467), s.ad_value(1462)), 1.0, 1474, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1589, 1535, A::div(s.ad_value(1531), s.ad_value(1527)), 1.0, 1538, -1.0);
            s.store_div_scaled_inputs2_indices(1590, 1588, 1.0, 1589, (-1.0), 1587, 1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1591, 1472, A::div(s.ad_value(1468), s.ad_value(1462)), 1.0, 1474, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1592, 1536, A::div(s.ad_value(1532), s.ad_value(1527)), 1.0, 1538, -1.0);
            s.store_div_scaled_inputs2_indices(1593, 1591, 1.0, 1592, (-1.0), 1587, 1.0);
        }

        if ((s.b[1604] && s.b[1756]) && (!s.b[1757])) {
            s.store_scalar(1590, 0.0);
            s.store_scalar(1593, 0.0);
        }

        if (s.b[1604] && (!s.b[1756])) {
            s.store_mul_add_scaled_inputs_rhs(1594, 1493, A::div(s.ad_value(1430), s.ad_value(1496)), (-2.0), s.ad_value(1499), (-2.0));
            s.store_mul_add_scaled_inputs_rhs(1595, 1494, A::div(s.ad_value(1431), s.ad_value(1497)), (-2.0), s.ad_value(1499), (-2.0));
            s.store_mul_sub_lhs(0, 1595, 1594, 1499);
            s.store_mul(2, 1594, 1430);
            s.store_mul(3, 1595, 1431);
            s.store_add(4, 2, 3);
            s.store_offset_ad(5, A::add_scaled_products(s.ad_value(1493), s.ad_value(1430), 2.0, s.ad_value(1494), s.ad_value(1431), 2.0), 3.0);
            s.store_div_scaled_inputs3(1596, s.ad_value(3), 1.0, s.ad_value(0), 1.0, A::div(s.ad_value(4), s.ad_value(1496)), -1.0, s.ad_value(5), 1.0);
            s.store_div_scaled_inputs3(1597, s.ad_value(2), 1.0, s.ad_value(0), (-1.0), A::div(s.ad_value(4), s.ad_value(1497)), -1.0, s.ad_value(5), 1.0);
            s.store_mul_add_scaled_product_rhs(1590, 1496, s.ad_value(1499), -1.0, s.ad_value(1596), s.ad_value(1496), -1.0);
            s.store_mul_add_scaled_product_rhs(1593, 1497, s.ad_value(1499), -1.0, s.ad_value(1597), s.ad_value(1497), -1.0);
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
            s.copy_ad(464, 424);
            s.copy_ad(465, 426);
            s.copy_ad(466, 427);
            s.copy_ad(467, 432);
            s.copy_ad(468, 433);
            s.copy_ad(469, 434);
            s.copy_ad(470, 435);
        }

        s.store_div_scaled_product_mixed_iaa(0, 120, A::sub(s.ad_value(444), s.ad_value(442)), 1.0, A::scale_offset(s.ad_value(460), 0.25, 1.0), 1.0);

        s.store_add_scaled_inputs3_indices(1320, 454, 0.5, 457, 0.5, 0, 1.0);

        s.store_add_scaled_inputs3_indices(1321, 455, 0.5, 458, 0.5, 0, -1.0);

        s.b[1758] = (p.p13 > 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if s.b[1758] {
            s.store_add_scaled_inputs3_mixed_iai(1322, 1320, 1.0, A::div(s.ad_value(462), s.ad_value(465)), 1.0, 462, -1.0);
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1758] {
            s.store_add_scaled_inputs3_mixed_iai(1323, 1321, 1.0, A::div(s.ad_value(463), s.ad_value(466)), 1.0, 463, -1.0);
        }

        if (!s.b[1758]) {
            s.copy_ad(1322, 1320);
            s.copy_ad(1323, 1321);
        }

        s.store_scaled_mul(2, 467, 469, 0.3333333333333);

        s.store_mul_scaled_offset_ad_rhs(3, 467, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(469), 1.0, A::scale(s.ad_value(469), 0.2)), 1.0);

        s.store_add_scaled_product_indices(1324, 3, 1.0, 1322, 461, 0.5);

        s.store_add_scaled_product_indices(1322, 2, 1.0, 1322, 461, 1.0);

        s.store_scaled_mul(2, 468, 470, 0.3333333333333);

        s.store_mul_scaled_offset_ad_rhs(3, 468, 0.1666666666667, A::mul_sub_from_scalar_rhs(s.ad_value(470), 1.0, A::scale(s.ad_value(470), 0.2)), 1.0);

        s.store_add_scaled_inputs(1325, 1323, 0.5, 3, 1.0);

        s.store_add(1323, 1323, 2);

        s.store_mul(0, 443, 283);

        s.store_mul(357, 0, 1322);

        s.store_mul(358, 0, 1323);

        s.store_mul_add_scaled_inputs_rhs(359, 0, s.ad_value(1324), -1.0, s.ad_value(1325), -1.0);

        s.b[1759] = (s.v[119] > 0.0);
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if s.b[1759] {
            s.store_offset(0, 250, (2.0 * 0.6931471805599));
            s.store_add(1326, 456, 0);
            s.store_add(1327, 459, 0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1328, 1326, 0.5, 250, 0.5, A::offset(A::mul(A::sub(s.ad_value(1326), s.ad_value(250)), A::sub(s.ad_value(1326), s.ad_value(250))), 9.0), (-0.5));
            s.store_add_scaled_inputs4_mixed_iiia(1329, 1327, 0.5, 250, 0.5, 335, 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_inputs3(s.ad_value(1327), 1.0, s.ad_value(250), -1.0, s.ad_value(335), -1.0), A::add_scaled_inputs3(s.ad_value(1327), 1.0, s.ad_value(250), -1.0, s.ad_value(335), -1.0)), 9.0)), (-0.5));
            s.store_mul_sqrt_ad_rhs(1330, 290, A::mul_offset_rhs(s.ad_value(441), s.ad_value(440), 0.5));
            s.store_mul_sqrt_ad_rhs(1331, 290, A::mul_offset_rhs(A::mul3(s.ad_value(441), s.ad_value(452), s.ad_value(440)), s.ad_value(439), 0.5));
            s.store_mul_square_lhs(1332, 1330, 287);
            s.store_mul_square_lhs(1333, 1331, 287);
            s.store_sub(2, 288, 1328);
            s.store_add_scaled_inputs3_indices(3, 288, 1.0, 335, 1.0, 1329, -1.0);
            s.store_scale(0, 1332, 2.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1334, 1328, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1332)), 1.0)), (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1335, 1329, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1332)), 1.0)), (-1.0), 1.0);
            s.store_scale(0, 1333, 2.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1336, 1328, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(2), s.ad_value(1333)), 1.0)), (-1.0), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1337, 1329, 1.0, 0, A::sqrt(A::offset(A::div(s.ad_value(3), s.ad_value(1333)), 1.0)), (-1.0), 1.0);
            s.store_mul(0, 289, 443);
            s.store_mul_product3_rhs(2, 447, s.ad_value(0), s.ad_value(1330), s.ad_value(452), -1.0);
            s.store_mul_product3_rhs(3, 448, s.ad_value(0), s.ad_value(1331), s.ad_value(453), -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(0, 1334, 0.5, 1326, ((-1.0) * 0.5), A::offset(A::mul(A::sub(s.ad_value(1334), s.ad_value(1326)), A::sub(s.ad_value(1334), s.ad_value(1326))), 1.0), 0.5);
            s.store_div_scaled_product3_mixed_iiia(375, 2, 0, 0, 1.0, A::sub(s.ad_value(1334), s.ad_value(1328)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(0, 1335, 0.5, 1327, ((-1.0) * 0.5), A::offset(A::mul(A::sub(s.ad_value(1335), s.ad_value(1327)), A::sub(s.ad_value(1335), s.ad_value(1327))), 1.0), 0.5);
            s.store_div_scaled_product3_mixed_iiia(376, 2, 0, 0, 1.0, A::sub(s.ad_value(1335), s.ad_value(1329)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(0, 1336, 0.5, 1326, ((-1.0) * 0.5), A::offset(A::mul(A::sub(s.ad_value(1336), s.ad_value(1326)), A::sub(s.ad_value(1336), s.ad_value(1326))), 1.0), 0.5);
            s.store_div_scaled_product3_mixed_iiia(377, 3, 0, 0, 1.0, A::sub(s.ad_value(1336), s.ad_value(1328)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(0, 1337, 0.5, 1327, ((-1.0) * 0.5), A::offset(A::mul(A::sub(s.ad_value(1337), s.ad_value(1327)), A::sub(s.ad_value(1337), s.ad_value(1327))), 1.0), 0.5);
            s.store_div_scaled_product3_mixed_iiia(378, 3, 0, 0, 1.0, A::sub(s.ad_value(1337), s.ad_value(1329)), 1.0);
        }

        if (!s.b[1759]) {
            s.store_scalar(375, 0.0);
            s.store_scalar(376, 0.0);
            s.store_scalar(377, 0.0);
            s.store_scalar(378, 0.0);
        }

        s.store_mul(366, 164, 326);

        s.store_mul(367, 165, 328);

        let assign42690_ad_e48215: A = A::add(A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(445), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436))))), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(1.0, A::mul3(s.ad_value(161), s.ad_value(445), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436)))), A::sub_from_scalar(1.0, A::mul3(s.ad_value(161), s.ad_value(445), A::sub_from_scalar(1.0, A::mul(s.ad_value(162), s.ad_value(436)))))), 0.2)));
        s.store_scale_ad(0, assign42690_ad_e48215, 0.5);

        s.store_mul3_lhs(368, 159, 345, 0);

        s.store_mul3_lhs(369, 160, 346, 0);

        s.store_mul(370, 117, 334);

        s.store_mul(371, 166, 332);

        s.store_mul_neg_ad_lhs(373, A::add_scaled_products(s.ad_value(236), s.ad_value(9), 1.0, s.ad_value(167), s.ad_value(11), 1.0), 327);

        s.store_mul_neg_ad_lhs(372, A::add_scaled_products(s.ad_value(236), s.ad_value(10), 1.0, s.ad_value(167), s.ad_value(12), 1.0), 329);

        s.b[1760] = (s.v[6] > 0.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if s.b[1760] {
            s.store_mul(374, 170, 215);
        }

        if (!s.b[1760]) {
            s.store_scalar(374, 0.0);
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(361, 13, s.ad_value(344), p.p31, s.ad_value(352), p.p31, s.ad_value(354), p.p31, 0.0);

        s.store_scaled_mul(362, 13, 348, p.p31);

        s.store_scaled_mul(363, 13, 349, p.p31);

        s.store_scaled_mul(364, 13, 350, p.p31);

        s.store_scaled_mul(365, 13, 351, p.p31);

        s.store_mul(1761, 13, 355);

        s.store_mul(1762, 13, 356);

        s.b[1763] = (s.v[330] < 0.0);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        s.store_scaled_mul(357, 13, 357, p.p32);

        s.store_scaled_mul(358, 13, 358, p.p32);

        s.store_scaled_mul(359, 13, 359, p.p32);

        s.store_neg_ad(360, A::add_scaled_inputs3(s.ad_value(357), 1.0, s.ad_value(358), 1.0, s.ad_value(359), 1.0));

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

        s.store_max_with_scalar_ad(1781, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(1780), 12.0, s.ad_value(1779))), 1e-20);

        s.store_div_from_scalar_square_ad(1782, 1.0, s.ad_value(1781));

        s.store_div_scaled_product3_by_product(1783, A::mul3(s.ad_value(338), s.ad_value(386), s.ad_value(222)), s.ad_value(1772), s.ad_value(340), 1.0, s.ad_value(341), s.ad_value(342), 1.0);

        s.store_scale(1784, 1779, 12.0);

        s.store_add_scaled_inputs3_mixed_iia(2, 1777, 1.0, 1784, 1.0, A::mul3_scaled_output(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784), s.ad_value(1780), 2.0), -1.0);

        s.store_max_with_scalar(3, 2, 1e-40);

        let assign43330_e48538: f64 = (s.v[1783] * s.v[1782]);
        let assign43330_e48540: f64 = (assign43330_e48538 * s.v[3]);
        s.v[1785] = assign43330_e48540;

        s.b[1802] = (s.v[172] > 0.0);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        let (assign43350_e48549,) = {
    if s.b[1802] {
        let assign43350_e48547: f64 = (s.v[423] / s.v[418]);
        (assign43350_e48547,)
    } else {
        (s.v[1786],)
    }
};
        s.v[1786] = assign43350_e48549;

        let (assign43360_e48569,) = {
    if s.b[1802] {
        let assign43360_e48553: f64 = (s.v[305] * s.v[344]);
        let assign43360_e48555: f64 = (assign43360_e48553 * s.v[407]);
        let assign43360_e48557: f64 = (assign43360_e48555 * s.v[219]);
        let assign43360_e48561: f64 = (s.v[1786] * s.v[1786]);
        let assign43360_e48562: f64 = (1.0 + assign43360_e48561);
        let assign43360_e48564: f64 = (assign43360_e48562 * s.v[1781]);
        let assign43360_e48566: f64 = (assign43360_e48564 * s.v[1781]);
        let assign43360_e48567: f64 = (assign43360_e48557 / assign43360_e48566);
        (assign43360_e48567,)
    } else {
        (s.v[1787],)
    }
};
        s.v[1787] = assign43360_e48569;

        let (assign43370_e48577,) = {
    if s.b[1802] {
        let assign43370_e48574: f64 = (s.v[1787] / s.v[304]);
        let assign43370_e48575: f64 = (s.v[1785] + assign43370_e48574);
        (assign43370_e48575,)
    } else {
        (s.v[1785],)
    }
};
        s.v[1785] = assign43370_e48577;

        s.store_div_scaled_product3_indices(1789, 452, 443, 116, 1.0, 465, 1.0);

        s.store_mul_offset_lhs(1790, 464, 1.0, 1789);

        s.store_mul_sub_from_scalar_ad_rhs(1792, 1790, 0.5, A::mul_scaled_lhs(s.ad_value(330), 0.25, s.ad_value(1778)));

        s.store_sub(1791, 1790, 1792);

        s.v[1795] = 0.0;

        s.b[1803] = (p.p6 > 0.0);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if s.b[1803] {
            s.store_sub_ad(2, A::add_scaled_product(s.ad_value(1777), 0.08333333333333333, s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 0.2), s.ad_value(1784)), (-1.0)), A::mul3_scaled_output(s.ad_value(1779), A::sub(A::offset(s.ad_value(1777), 1.0), s.ad_value(1784)), s.ad_value(1780), 1.6));
            s.store_max_with_scalar(3, 2, 1e-40);
            s.store_div_scaled_product3_indices(1793, 1783, 1781, 1781, 1.0, 3, 1.0);
        }

        s.b[1804] = (s.v[1785] > 0.0);
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if (s.b[1803] && s.b[1804]) {
            s.store_mul_ad_product_rhs(1795, 1782, s.ad_value(1778), A::add_scaled_sub_value_product(1.0, s.ad_value(1784), 1.0, A::add_scaled_inputs_product(s.ad_value(1777), 1.0, s.ad_value(1779), 19.2, s.ad_value(1777), s.ad_value(1784), (-1.0)), s.ad_value(1780), (-1.0)));
        }

        if (!s.b[1803]) {
            s.store_scalar(1793, 1.0);
        }

        s.copy_ad(1773, 1770);

        s.store_mul_offset_rhs(1774, 1770, 411, 1.0);

        s.store_mul_sub_rhs(1775, 1770, 399, 409);

        s.store_mul_ad(2, A::add(A::add_scaled_product(s.ad_value(173), 1.0, s.ad_value(174), s.ad_value(1773), (-1.0)), A::mul3(s.ad_value(175), s.ad_value(1773), s.ad_value(1773))), A::ln(A::div_scaled_inputs2(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5, A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1775), 0.5), 1.0)));

        s.store_add_scaled_product_left_ad(3, 2, 1.0, A::add_scaled_product(s.ad_value(174), 1.0, s.ad_value(175), A::sub_scaled_inputs(s.ad_value(1774), 1.0, s.ad_value(1773), 2.0), 1.0), 1775, 1.0);

        s.store_offset_div_ad(0, A::add_scaled_products(s.ad_value(176), s.ad_value(413), 1.0, s.ad_value(177), s.ad_value(414), 1.0), A::offset(s.ad_value(411), 1.0), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(4, 0, 0.01, (-0.01), 0.0001, 0.5);

        s.store_mul_div_scaled_product_rhs(0, 4, A::div_scaled_product(s.ad_value(343), s.ad_value(344), 1.602176565e-19, s.ad_value(341), 1.0), s.ad_value(3), 1.0, s.ad_value(1773), 1.0);

        s.store_div_from_scalar_scaled_input(1813, 1.0, 8, 8.617332384961e-5);

        s.store_sub_from_scalar_ad(1814, 1.17, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.000473, s.ad_value(8), 636.0, 1.0));

        s.store_sub_from_scalar_ad(1815, 0.744, A::div_scaled_product_offset_denominator(s.ad_value(8), s.ad_value(8), 0.0004774, s.ad_value(8), 235.0, 1.0));

        s.store_mul_add_scaled_inputs3_offset_rhs(1816, 15, s.ad_value(1815), 1.0, s.ad_value(1814), (-1.0), s.ad_value(224), (-0.4), 0.0);

        s.store_add(1817, 1814, 1816);

        s.store_scaled_mul(1818, 1817, 1813, 0.5);

        s.store_sub_scaled_inputs(1819, 15, 0.05, 1816, 0.5);

        s.store_sqrt_scaled_input(0, 8, 0.0033333333333);

        s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);

        s.store_mul(1820, 2, 234);

        s.store_div_scaled_value_offset_denominator(1821, s.ad_value(1813), 1.0, A::div_scaled_inputs(s.ad_value(17), s.v[7], s.ad_value(8), 1.0), 1.0, 1.0);

        s.store_mul3_affine_lhs(1823, 1820, 225, (2.0 * 1.602176565e-19), 0.0, 1821);

        s.store_add_offset_ad_lhs(1824, A::ln(A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(1823), 1.0)), (-0.6931471805599), 1818);

        s.store_mul_div_scaled_product_rhs(1825, 1821, s.ad_value(29), s.ad_value(14), (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);

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

        s.store_div_scaled_value_by_product(1805, s.ad_value(398), 1.0, s.ad_value(397), A::offset(s.ad_value(398), 1.0), 1.0);

        s.store_div_scaled_value_by_product(1806, s.ad_value(397), 1.0, s.ad_value(398), A::offset(s.ad_value(397), 1.0), 1.0);

        s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(397), A::offset(s.ad_value(1805), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);

        s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(398), A::offset(s.ad_value(1806), 1.0), s.ad_value(380), 1.0, s.ad_value(381), 1.0), 2.0);

        s.store_add_scaled_products_left_left_ad(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 395, 1805, (-1.0));

        s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(395), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);

        s.store_add_ad_lhs(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1809), s.ad_value(1810)), A::sub(s.ad_value(1809), s.ad_value(1810))), 38.0)), (-0.5), s.ad_value(394), -1.0, s.ad_value(25), 1.0), 394);

        s.store_add_scaled_product_right_ad(1812, 21, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(390), (-1.0), s.ad_value(391), 1.0), 1.0, s.ad_value(393), (-1.0), s.ad_value(390), 1.0), 1.0);

        s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));

        s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p.p14);

        s.store_sub_offset_ad_lhs(1830, A::add_scaled_inputs4(s.ad_value(179), p.p14, s.ad_value(1819), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 1822);

        s.store_add_scaled_inputs4_indices(1831, 180, p.p14, 1819, p.p14, 240, p.p14, 0, 1.0);

        s.store_add_scaled_product_left_ad(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);

        s.store_add_scaled_product_left_ad(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);

        s.b[1877] = (p.p2 > 0.0);
        s.v[1877] = if s.b[1877] { 1.0 } else { 0.0 };

        if s.b[1877] {
            s.store_div_scaled_product_right_ad(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p.p14, 256, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1878] = (s.v[0] < 0.0);
        s.v[1878] = if s.b[1878] { 1.0 } else { 0.0 };

        if (s.b[1877] && s.b[1878]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if (s.b[1877] && (!s.b[1878])) {
            s.store_div_scaled_product_offset_denominator(2, s.ad_value(0), s.ad_value(0), 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);
        }

        if s.b[1877] {
            s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p.p14);
        }

        if (!s.b[1877]) {
            s.copy_ad(1837, 1836);
        }

        s.store_mul_sub_rhs(0, 244, 1835, 1837);

        s.b[1879] = (p.p13 > 0.0);
        s.v[1879] = if s.b[1879] { 1.0 } else { 0.0 };

        if s.b[1879] {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253)), 1.0), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), 1.0), 0.5);
            s.store_mul_ad_rhs(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div_scaled_product_offset_denominator(1841, s.ad_value(242), s.ad_value(4), 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);
            s.store_div_scaled_product_offset_denominator(1842, s.ad_value(243), s.ad_value(4), 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);
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
            s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1835, 1.0, A::div(s.ad_value(1844), s.ad_value(1841)), (-1.0), 0, 1.0, (-0.6931471805599));
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
            s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1837, 1.0, A::div(s.ad_value(1844), s.ad_value(1842)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1846, 1845, 0.5, 1824, 0.5, A::offset(A::mul(A::sub(s.ad_value(1845), s.ad_value(1824)), A::sub(s.ad_value(1845), s.ad_value(1824))), 4.0), (-0.5));

        s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));

        s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + ((-0.5))))), 0.01)), 0.5);

        s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0);

        s.b[1884] = (p.p11 > 0.0);
        s.v[1884] = if s.b[1884] { 1.0 } else { 0.0 };

        if s.b[1884] {
            s.store_div_scaled_value_by_product(1805, s.ad_value(453), 1.0, s.ad_value(452), A::offset(s.ad_value(453), 1.0), 1.0);
            s.store_div_scaled_value_by_product(1806, s.ad_value(452), 1.0, s.ad_value(453), A::offset(s.ad_value(452), 1.0), 1.0);
            s.store_offset_ln_ad(1807, A::div_scaled_product3(s.ad_value(452), A::offset(s.ad_value(1805), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);
            s.store_offset_ln_ad(1808, A::div_scaled_product3(s.ad_value(453), A::offset(s.ad_value(1806), 1.0), s.ad_value(437), 1.0, s.ad_value(438), 1.0), 2.0);
            s.store_add_scaled_products_left_left_ad(1809, A::offset(s.ad_value(1805), 1.0), 1807, 1.0, 451, 1805, (-1.0));
            s.store_add_scaled_offset_product_lhs_mixed_aai(1810, A::div(s.ad_value(451), s.ad_value(1806)), (-1.0), A::div_from_scalar(1.0, s.ad_value(1806)), 1.0, 1808, 1.0);
            s.store_add_ad_lhs(1811, A::div_scaled_inputs4(s.ad_value(1809), 0.5, s.ad_value(1810), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1809), s.ad_value(1810)), A::sub(s.ad_value(1809), s.ad_value(1810))), 38.0)), (-0.5), s.ad_value(450), -1.0, s.ad_value(25), 1.0), 450);
            s.store_add_scaled_product_right_ad(1812, 130, 1.0, 222, A::add_scaled_inputs3(A::div_scaled_inputs2(s.ad_value(1811), 1.0, s.ad_value(446), (-1.0), s.ad_value(447), 1.0), 1.0, s.ad_value(449), (-1.0), s.ad_value(446), 1.0), 1.0);
            s.store_mul_offset_rhs(0, 34, 8, (-s.v[7]));
            s.store_add_scaled_offset_product_rhs(0, 252, 1.0, 23, 8, (-s.v[7]), p.p14);
            s.store_sub_offset_ad_lhs(1830, A::add_scaled_inputs4(s.ad_value(181), p.p14, s.ad_value(1819), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 1822);
            s.store_add_scaled_inputs4_indices(1831, 182, p.p14, 1819, p.p14, 240, p.p14, 0, 1.0);
            s.store_add_scaled_product_left_ad(1835, 1834, (-1.0), A::sub(s.ad_value(1812), s.ad_value(1830)), 1821, 1.0);
            s.store_add_scaled_product_left_ad(1836, 1834, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(1831), 1.0), 1821, 1.0);
        }

        s.b[1885] = (p.p2 > 0.0);
        s.v[1885] = if s.b[1885] { 1.0 } else { 0.0 };

        if (s.b[1884] && s.b[1885]) {
            s.store_div_scaled_product_right_ad(0, 16, A::sub(s.ad_value(1835), s.ad_value(1836)), p.p14, 256, 1.0);
        }

        s.b[1886] = (s.v[0] < 0.0);
        s.v[1886] = if s.b[1886] { 1.0 } else { 0.0 };

        if ((s.b[1884] && s.b[1885]) && s.b[1886]) {
            s.store_scaled_ln_ad(2, A::sub_from_scalar(1.0, s.ad_value(0)), (-2.0));
        }

        if ((s.b[1884] && s.b[1885]) && (!s.b[1886])) {
            s.store_div_scaled_product_offset_denominator(2, s.ad_value(0), s.ad_value(0), 1.0, A::div_scaled_inputs(s.ad_value(0), 2.0, s.ad_value(256), 1.0), 1.0, 1.0);
        }

        if (s.b[1884] && s.b[1885]) {
            s.store_add_scaled_product_indices(1837, 1836, 1.0, 16, 2, p.p14);
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
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1838, 0, 0.5, 253, 0.5, A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253)), 1.0), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1839, 253, 0.5, 0, ((-1.0) * 0.5), A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), 1.0), 0.5);
            s.store_mul_ad_rhs(2, 1829, A::exp_scaled_input(A::ln(s.ad_value(1838)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 1829, A::exp_scaled_input(A::ln(s.ad_value(1839)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div_scaled_product_offset_denominator(1841, s.ad_value(242), s.ad_value(4), 1.0, A::mul(s.ad_value(242), s.ad_value(2)), 1.0, 1.0);
            s.store_div_scaled_product_offset_denominator(1842, s.ad_value(243), s.ad_value(4), 1.0, A::mul(s.ad_value(243), s.ad_value(3)), 1.0, 1.0);
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
            s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1835, 1.0, A::div(s.ad_value(1844), s.ad_value(1841)), (-1.0), 0, 1.0, (-0.6931471805599));
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
            s.store_add_scaled_inputs3_offset_mixed_iai(1845, 1837, 1.0, A::div(s.ad_value(1844), s.ad_value(1842)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        if s.b[1884] {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1846, 1845, 0.5, 1824, 0.5, A::offset(A::mul(A::sub(s.ad_value(1845), s.ad_value(1824)), A::sub(s.ad_value(1845), s.ad_value(1824))), 4.0), (-0.5));
            s.store_offset_sqrt_ad(1847, A::offset(A::div_scaled_inputs2(s.ad_value(1824), 2.0, s.ad_value(1846), (-2.0), s.ad_value(1825), 1.0), 1.0), (-1.0));
            s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(1836)), ((1.0) + ((-0.5))))), 0.01)), 0.5);
            s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(1828), A::offset(A::sqrt(A::offset(A::div(s.ad_value(1833), s.ad_value(1828)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(1847)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(1836)), 1.0);
        }

    }
}
