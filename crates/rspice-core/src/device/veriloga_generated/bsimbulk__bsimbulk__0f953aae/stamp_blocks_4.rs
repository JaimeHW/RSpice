#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1524] && s.b[1525]) && s.b[1526]) {s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p718)), 0.0);}
        if ((s.b[1524] && s.b[1525]) && (!s.b[1526])) {s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if (s.b[1524] && (!s.b[1525])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p718)) * (s.v[328]), ((((((-1.0)) * ((5.0 * p.p718)))) + ((1.0 + p.p718)))) * (s.v[328]));s.store_mul_ad_product_rhs_mixed_ia(337, 434, 327, A::add(s.ad_value(14), s.ad_value(329)));}
        if (!s.b[1524]) {s.store_scalar(337, 0.0);}
        s.store_add_scaled_inputs3_indices(334, 335, 1.0, 336, 1.0, 337, 1.0);s.b[1528] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if s.b[1528] {s.store_scaled_mul(321, 426, 251, p.p1128);}
        s.b[1529] = (s.v[301] > (s.v[35] * p.p2));s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if (s.b[1528] && s.b[1529]) {s.store_mul_scale_offset_rhs(324, 427, 301, p.p1128, (((((-(s.v[35] * p.p2))) * (p.p1128))) + ((s.v[35] * p.p2))));}
        if (s.b[1528] && (!s.b[1529])) {s.store_scaled_mul(324, 427, 301, p.p1128);}
        s.b[1530] = (s.v[321] > 0.0);s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });
        if (s.b[1528] && s.b[1530]) {s.store_div(13, 309, 432);}
        s.b[1531] = (s.v[13] < 0.9);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });
        if ((s.b[1528] && s.b[1530]) && s.b[1531]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1532] = (p.p714 != 1.0);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });s.b[1533] = (p.p714 == 0.5);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });
        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && s.b[1533]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && (!s.b[1533])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p714));}
        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) {s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p714)), 0.0);}
        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && (!s.b[1532])) {s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if ((s.b[1528] && s.b[1530]) && (!s.b[1531])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p714)) * (s.v[322]), ((((((-1.0)) * ((5.0 * p.p714)))) + ((1.0 + p.p714)))) * (s.v[322]));s.store_mul_ad_product_rhs_mixed_ia(339, 432, 321, A::add(s.ad_value(14), s.ad_value(323)));}
        if (s.b[1528] && (!s.b[1530])) {s.store_scalar(339, 0.0);}
        s.b[1534] = (s.v[324] > 0.0);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if (s.b[1528] && s.b[1534]) {s.store_div(13, 309, 433);}
        s.b[1535] = (s.v[13] < 0.9);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if ((s.b[1528] && s.b[1534]) && s.b[1535]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1536] = (p.p716 != 1.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });s.b[1537] = (p.p716 == 0.5);s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && s.b[1537]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && (!s.b[1537])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p716));}
        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) {s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p716)), 0.0);}
        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && (!s.b[1536])) {s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if ((s.b[1528] && s.b[1534]) && (!s.b[1535])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p716)) * (s.v[325]), ((((((-1.0)) * ((5.0 * p.p716)))) + ((1.0 + p.p716)))) * (s.v[325]));s.store_mul_ad_product_rhs_mixed_ia(340, 433, 324, A::add(s.ad_value(14), s.ad_value(326)));}
        if (s.b[1528] && (!s.b[1534])) {s.store_scalar(340, 0.0);}
        if s.b[1528] {s.store_add(338, 339, 340);}
        if (!s.b[1528]) {s.store_scalar(338, 0.0);}
        s.b[1538] = (p.p38 != 0.0);s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1538] {s.store_powf_scaled_input(13, 481, 1.0000000000000001e-23, p.p954);s.store_powf_ad(14, A::div_from_scalar(300.0, s.ad_value(391)), p.p955);s.store_div_scaled_product_mixed_iai(15, 187, A::voltage(ctx, nodes, Some(11), Some(7)), p.p953, 108, 1.0);s.store_scaled_limited_exp_ad(707, A::mul_scaled_lhs(s.ad_value(13), -1.0, s.ad_value(14)), p.p948);s.store_scaled_mul(708, 14, 13, p.p949);s.store_scale_ad(709, A::tanh(A::limited_exp(A::mul_scaled_lhs(s.ad_value(187), p.p952, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(11)), 1.0, s.ad_value(857), (-1.0), A::voltage(ctx, nodes, Some(7), Some(11)), -1.0)))), p.p951);s.store_mul_scale_offset(706, A::mul3(A::mul3_scaled_output(s.ad_value(57), s.ad_value(707), A::limited_exp(s.ad_value(15)), (p.p2 * s.v[29])), A::limited_exp_scaled_input(s.ad_value(708), (-s.v[30])), A::limited_exp(A::div(s.ad_value(709), s.ad_value(108)))), A::limited_exp_div_scaled_inputs(s.ad_value(76), p.p950, s.ad_value(108), 1.0), 1.0, (-1.0));}
        s.store_scale(377, 108, (4.0 * 1.60219e-19));s.store_div_scaled_inputs_indices(360, 502, 2.0, 157, 1.0);s.b[1539] = (p.p784 <= 0.0);s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if s.b[1539] {s.store_scalar(363, 0.0);}
        if (!s.b[1539]) {s.store_div_scaled_offset_numerator_mixed_ai(12, A::div(s.ad_value(167), s.ad_value(129)), 1.0, p.p784, 360, 1.0);s.store_mul_ln_mixed_ia(363, 129, A::max_with_scalar(s.ad_value(12), 1e-38));}
        s.b[1540] = (s.v[363] < 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if ((!s.b[1539]) && s.b[1540]) {s.store_scalar(363, 0.0);}
        s.store_mul_add_scaled_inputs_rhs_mixed_ai(367, 108, A::offset(s.ad_value(97), s.v[46]), 1.0 / (1.60219e-19), 483, 1.0 / (1.60219e-19));s.store_mul_ad_affine_product_lhs(366, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(144), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(188)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);s.store_mul3_affine_lhs(737, 108, 188, 1.60219e-19, 0.0, 188);s.store_add_scaled_product_mixed_aii(738, A::scale_offset(s.ad_value(366), p.p799, p.p785), 1.0, 366, 366, p.p800);s.store_square_ad(739, A::add(s.ad_value(366), s.ad_value(367)));s.store_scale(740, 108, (p.p785 * 1.60219e-19));s.b[1541] = (p.p1065 == 1.0);s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if s.b[1541] {s.store_scalar(745, s.v[30]);s.store_div_scaled_inputs2_indices(712, 64, 1.0, 482, (-1.0), 108, 1.0);s.store_scaled_sqrt_ad(713, A::div_from_scalar((((2.0 * 1.60219e-19) * s.v[26]) * p.p1068), s.ad_value(108)), 1.0 / (s.v[46]));s.store_ln_ad(714, A::div_from_scalar(p.p1068, s.ad_value(28)));s.store_scalar(13, 1.0);s.store_div(204, 712, 13);s.store_div(205, 713, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
    ) {
        s.b[1542] = (s.v[204] < 0.0);s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1542]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(715, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1541] && (!s.b[1542])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(715, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1541] {s.store_scaled_add_offset_sqrt_square_offset(20, 715, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(713), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 713, 1.0);s.store_add_scaled_inputs3_indices(13, 715, 1.0, 714, (-2.0), 73, -1.0);s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1543] = (s.v[20] <= (-68.0));s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1543]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1544] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if ((s.b[1541] && s.b[1543]) && s.b[1544]) {s.store_limited_exp(15, 16);}
        s.b[1545] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && s.b[1545]) {s.store_limited_exp(15, 20);}
        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && (!s.b[1545])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1541] && s.b[1543]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(717, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (s.b[1541] && (!s.b[1543])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
    ) {
        if (s.b[1541] && (!s.b[1543])) {s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(717, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1546] = ((1.0 == 0.0) && (s.v[715] < ((-2500.0) * 2.0)));s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1546]) {s.store_div_from_scalar_scaled_input(716, ((-2.0) * 2.0), 715, 16.0);}
        if (s.b[1541] && (!s.b[1546])) {s.store_scaled_add_offset_sqrt_square_offset(716, 715, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1541] {s.store_offset_div_scaled_inputs_sqrt_rhs(718, 713, 1.0, 716, 2.0, 1.0);s.copy_ad(719, 157);s.store_scale(726, 719, (s.v[46] * s.v[29]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1541] {s.store_scale(725, 157, (s.v[46] * s.v[29]));s.store_div_scaled_product_by_product_mixed_iiai(720, 188, 746, 1.0, A::mul3_scaled_output(s.ad_value(718), s.ad_value(726), s.ad_value(108), 2.0), 108, 1.0);s.store_div_scaled_product_by_product_mixed_iaai(722, 188, A::sub(s.ad_value(745), s.ad_value(746)), 1.0, A::mul3_scaled_output(s.ad_value(90), s.ad_value(725), s.ad_value(106), 2.0), 106, 1.0);s.store_add_scaled_inputs3_offset_mixed_aii(12, A::square(s.ad_value(717)), 4.0, 717, 4.0, 720, (-4.0), 1.0);s.store_offset_scaled_ad(723, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(144)), 4.0, s.ad_value(144), 4.0, s.ad_value(722), 4.0), 1.0)), 0.5, (-0.5));}
        s.b[1548] = (s.v[30] != s.v[746]);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1548]) {s.store_mul3_affine_lhs(724, 90, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 723);s.store_primal_add_scaled_inputs3_indices(361, 745, 1.0, 359, (-2.0), 746, -1.0);s.store_primal_square(362, 361);s.store_scale(13, 362, (10000000000.0 * s.v[46]));s.store_scaled_ln_ad(14, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(724), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38), p.p785);s.store_scaled_sub(15, 724, 366, p.p799);s.store_scaled_sub_ad(16, A::square(s.ad_value(724)), A::square(s.ad_value(366)), (0.5 * p.p800));s.store_scale(17, 362, (10000000000.0 * (s.v[29] * p.p2)));s.store_add_scaled_product(732, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(17), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(13)), A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0), 1.0);s.store_mul3_affine_lhs(18, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);s.store_mul_ad_product_lhs_mixed_ai(733, A::div(s.ad_value(740), s.ad_value(18)), 188, 188);s.store_add(19, 733, 732);}
        if s.b[1541] {s.store_scale(20, 108, (p.p1067 * 1.60219e-19));s.store_mul3_affine_lhs(21, 746, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);s.store_mul_ad_product_lhs_mixed_ai(741, A::div(s.ad_value(20), s.ad_value(21)), 188, 188);s.copy_ad(22, 741);}
        s.b[1551] = (p.p801 >= (s.v[30] / 2.0));s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if ((!s.b[1541]) && s.b[1551]) {s.store_scalar(359, 0.0);}
        if ((!s.b[1541]) && (!s.b[1551])) {s.store_scalar(359, p.p801);}
        s.b[1552] = (((p.p785 > 0.0) || (p.p799 > 0.0)) || (p.p800 > 0.0));s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });s.b[1553] = ((p.p786 != 0.0) && (p.p785 > 0.0));s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if (((!s.b[1541]) && s.b[1552]) && s.b[1553]) {s.store_div(13, 80, 641);s.store_offset_pow_ad(14, s.ad_value(13), s.ad_value(642), 1.0);s.store_div(15, 640, 14);s.store_scale(16, 15, 1.0 / (p.p785));s.store_scaled_add_offset_sqrt_square_offset(17, 16, 1.0, (-1.0), ((0.25 * p.p798) * p.p798), 0.5);s.store_scale(364, 17, p.p785);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1541]) && s.b[1552]) && (!s.b[1553])) {s.store_scalar(364, p.p785);}
        if ((!s.b[1541]) && s.b[1552]) {s.store_primal_sub_from_scalar_scaled_input(361, s.v[30], 359, 2.0);s.store_primal_square(362, 361);s.store_scale(12, 362, (10000000000.0 * s.v[46]));s.store_mul_ad_affine_product_lhs(365, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(200), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);s.store_mul_ln_mixed_ia(13, 364, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));s.store_scaled_sub(14, 365, 366, p.p799);s.store_scaled_sub_ad(15, A::square(s.ad_value(365)), A::square(s.ad_value(366)), (0.5 * p.p800));s.store_scale(16, 362, (10000000000.0 * (s.v[29] * p.p2)));s.store_add_scaled_product(368, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0);s.store_mul3_affine_lhs(17, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);s.store_scaled_mul(740, 364, 108, 1.60219e-19);s.store_mul_ad_product_lhs_mixed_ai(369, A::div(s.ad_value(740), s.ad_value(17)), 188, 188);s.store_add(18, 369, 368);}
        s.store_scaled_div(12, 80, 360, 1.0 / (s.v[30]));s.store_square(13, 12);s.store_offset_scaled(15, 13, (((p.p814 * s.v[30])) * (p.p811)), p.p811);s.store_offset_scaled(16, 13, (((p.p815 * s.v[30])) * (p.p812)), p.p812);s.store_offset_scaled(17, 13, (((p.p1044 * s.v[30])) * (p.p1043)), p.p1043);s.store_scaled_mul(387, 15, 15, 3.0);s.store_offset_scaled(387, 387, ((((-s.v[30]) / p.p1042)) as f64).exp(), (((((-1.0)) * (((((-s.v[30]) / p.p1042)) as f64).exp()))) + (1.0)));s.store_square(389, 17);s.store_square(388, 16);s.b[1555] = (p.p48 == 0.0);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });s.b[1556] = (p.p48 == 1.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
        if s.b[1555] {s.store_scaled_mul(196, 108, 190, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));s.store_scaled_mul(197, 108, 193, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));s.store_mul_abs_mixed_ia(12, 157, A::add(s.ad_value(196), s.ad_value(197)));s.store_offset_mul(13, 12, 244, (s.v[30] * s.v[30]));}
        if (s.b[1556] && (!s.b[1555])) {s.store_scaled_mul(382, 90, 106, 2.0);s.store_mul_scale_offset_mixed_ia(12, 382, A::mul3(s.ad_value(157), s.ad_value(163), s.ad_value(175)), s.v[46], 0.0);s.store_scaled_add(13, 200, 144, 0.5);s.store_offset(15, 13, 0.5);s.store_square(16, 15);s.store_mul(17, 16, 15);s.store_sub(18, 200, 144);s.store_square(19, 18);s.store_mul(20, 19, 18);s.store_mul_scale_offset_rhs(21, 19, 13, 6.0, 0.5);s.store_scale(381, 163, s.v[30]);s.store_scale(22, 381, 1.0 / (s.v[30]));s.store_offset_ad(24, A::div_scaled_product_by_product(s.ad_value(389), s.ad_value(139), 1.0, s.ad_value(140), A::offset(s.ad_value(80), p.p1045), 1.0), 1.0);s.store_offset_scaled(24, 24, ((((-s.v[30]) / p.p1042)) as f64).exp(), (((((-1.0)) * (((((-s.v[30]) / p.p1042)) as f64).exp()))) + (1.0)));}
        s.b[1557] = ((0.0 == 0.0) && (s.v[24] < ((-2500.0) * 0.1)));s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((s.b[1556] && (!s.b[1555])) && s.b[1557]) {s.store_div_from_scalar_scaled_input(24, ((-0.1) * 0.1), 24, 16.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1556] && (!s.b[1555])) && (!s.b[1557])) {s.store_scaled_add_mixed_ia(24, 24, A::sqrt_square_offset(s.ad_value(24), ((0.25 * 0.1) * 0.1)), 0.5);}
        if (s.b[1556] && (!s.b[1555])) {s.store_mul_div_scaled_inputs_mixed_aii(380, A::add_scaled_product(A::div_scaled_product(s.ad_value(19), s.ad_value(387), 1.0, s.ad_value(15), 12.0), 1.0, s.ad_value(13), s.ad_value(24), 1.0), 12, (p.p2 * s.v[29]), 381, 1.0);s.store_div_scaled_product3_mixed_aaii(378, A::mul3(s.ad_value(381), s.ad_value(22), s.ad_value(22)), A::add_scaled_inputs3(A::div(s.ad_value(13), s.ad_value(16)), 1.0, A::div(s.ad_value(21), A::mul_scaled_lhs(s.ad_value(16), 60.0, s.ad_value(16))), (-1.0), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(19), 1.0, s.ad_value(16), s.ad_value(17), 144.0), 1.0), 388, (15.0 * 1.0 / (4.0)), 12, ((p.p2 * s.v[29]) * 12.0));s.store_sqrt_mul(384, 377, 380);}
        s.b[1558] = (s.v[378] > 0.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
        if ((s.b[1556] && (!s.b[1555])) && s.b[1558]) {s.store_sqrt_div(385, 377, 378);}
        if ((s.b[1556] && (!s.b[1555])) && (!s.b[1558])) {s.store_scalar(385, 0.0);}
        s.b[1560] = (p.p46 != 0.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });s.b[1561] = (p.p47 != 0.0);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.copy_ad(60, 59);s.store_scalar(218, 0.0);s.b[1562] = (p.p40 == 1.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
        if s.b[1562] {s.store_offset(549, 549, p.p35);s.store_mul(65, 64, 109);s.store_mul(73, 72, 109);s.store_mul(58, 549, 109);s.store_sub(60, 65, 58);s.store_ln_ad(233, A::max_with_scalar(A::div(s.ad_value(550), s.ad_value(28)), 1e-38));s.store_scaled_sqrt_mul_scaled_lhs(234, 550, ((2.0 * 1.60219e-19) * s.v[26]), 109, 1.0 / (s.v[46]));s.store_div_from_scalar(126, 1.0, 234);s.store_div_scaled_inputs_indices(206, 479, ((2.0 * 1.60219e-19) * s.v[26]), 108, (s.v[46] * s.v[46]));}
        if s.b[1562] {
            if (s.v[479] > 0.0) {
                s.store_div_from_scalar(218, 1.0, 206);
            } else {
                s.store_scalar(218, 0.0);
            }
        }
        if s.b[1562] {
            if (s.v[479] > 0.0) {
                s.store_div(203, 550, 479);
            } else {
                s.store_scalar(203, 0.0);
            }
        }
        if s.b[1562] {s.store_offset(13, 203, 1.0);s.store_div(204, 60, 13);s.store_div(205, 234, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1563] = (s.v[204] < 0.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1563]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1562] && (!s.b[1563])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1563])) {s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1562] {s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 234, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 233, (-2.0), 73, -1.0);s.store_sub_scaled_inputs_mixed_ia(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)), 1.0);s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1564] = (s.v[20] <= (-68.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1564]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1565] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if ((s.b[1562] && s.b[1564]) && s.b[1565]) {s.store_limited_exp(15, 16);}
        s.b[1566] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && s.b[1566]) {s.store_limited_exp(15, 20);}
        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && (!s.b[1566])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1562] && s.b[1564]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(200, 15, 13, 1.0, 20, (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0);}
        if (s.b[1562] && (!s.b[1564])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1564])) {s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(200, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1567] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1567]) {s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);}
        if (s.b[1562] && (!s.b[1567])) {s.store_scaled_add_offset_sqrt_square_offset(93, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_sqrt(96, 93);s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);}
        s.b[1568] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1568]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (s.b[1562] && (!s.b[1568])) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_offset_div_ad(90, s.ad_value(234), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1562] {s.store_mul_mixed_ia(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1569] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1569]) {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
        if (s.b[1562] && (!s.b[1569])) {s.store_scaled_add_mixed_ia(84, 12, A::sqrt_square_offset(s.ad_value(12), ((0.25 * 0.1) * 0.1)), 0.5);}
        if s.b[1562] {s.store_mul3_affine_lhs(130, 90, 108, 2.0, 0.0, 200);s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));s.store_mul_add_scaled_product_pow_rhs(15, 506, 1.0, 516, 62, 1.0, 132, 407);s.store_offset(16, 15, 1.0);}
        s.b[1570] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1570]) {s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);}
        if (s.b[1562] && (!s.b[1570])) {s.store_scaled_add_offset_sqrt_square_offset(133, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        if s.b[1562] {s.store_div_scaled_product_by_product_indices(137, 499, 108, 1.0, 133, 411, s.v[34]);s.store_div_scaled_product_offset_denominator_mixed_iaa(131, 137, A::add(A::square(s.ad_value(200)), s.ad_value(200)), 1.0, A::mul_offset_rhs(s.ad_value(137), s.ad_value(200), 1.0), 1.0, 1.0);s.store_add_scaled_inputs4_mixed_iiia(145, 91, 1.0, 233, (-2.0), 131, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(234), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0);s.store_mul(146, 145, 108);}
        s.b[1571] = ((0.0 == 0.0) && ((s.v[146] - s.v[72]) < ((-2500.0) * 0.001)));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1571]) {s.store_div_from_scalar_ad(141, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(146), 16.0, s.ad_value(72), 16.0));}
        if (s.b[1562] && (!s.b[1571])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(141, 146, 0.5, 72, ((-1.0) * 0.5), 146, 72, ((0.25 * 0.001) * 0.001), 0.5);}
        s.b[1572] = ((p.p1134 == 0.0) && (p.p1135 == 0.0));s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1572]) {s.store_scalar(783, p.p1129);}
        if (s.b[1562] && (!s.b[1572])) {s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);s.store_offset_div_scaled_inputs2_mixed_iaa(783, 13, p.p1134, A::mul3_scaled_output(s.ad_value(13), s.ad_value(200), s.ad_value(106), p.p1135), (-1.0), A::scale_offset(s.ad_value(61), p.p1136, 1.0), 1.0, 1.0);}
        s.b[1573] = ((0.1 == 0.0) && (s.v[783] < ((-2500.0) * 0.0005)));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if ((s.b[1562] && (!s.b[1572])) && s.b[1573]) {s.store_div_from_scalar_scaled_input(783, ((-0.0005) * 0.0005), 783, 16.0);}
        if ((s.b[1562] && (!s.b[1572])) && (!s.b[1573])) {s.store_scaled_add_offset_sqrt_square_offset(783, 783, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1562] {s.store_div(141, 141, 783);s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(141)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));s.store_mul(139, 75, 20);s.store_mul_add_lhs(142, 139, 72, 109);s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 234, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 233, (-2.0), 142, -1.0);s.store_sub_scaled_inputs_mixed_ia(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)), 1.0);s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1574] = (s.v[20] <= (-68.0));s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1574]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1575] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((s.b[1562] && s.b[1574]) && s.b[1575]) {s.store_limited_exp(15, 16);}
        s.b[1576] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && s.b[1576]) {s.store_limited_exp(15, 20);}
        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && (!s.b[1576])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1562] && s.b[1574]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(144, 15, 13, 1.0, 20, (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0);}
        if (s.b[1562] && (!s.b[1574])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1574])) {s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(144, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        if s.b[1562] {s.store_add_scaled_inputs3_offset_indices(92, 91, 1.0, 200, (-1.0), 144, -1.0, (-1.0));}
        s.b[1577] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1577]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (s.b[1562] && (!s.b[1577])) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_sqrt(14, 12);s.store_add_offset_lhs_mixed_ia(15, 203, 1.0, A::div(s.ad_value(234), A::add(s.ad_value(96), s.ad_value(14))));s.store_offset_product3(16, s.ad_value(203), s.ad_value(14), s.ad_value(126), 1.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1562] {s.store_sqrt_add_ad(17, A::square(s.ad_value(16)), A::mul3(s.ad_value(15), A::add(s.ad_value(200), s.ad_value(144)), s.ad_value(218)));s.store_div_add_scaled_inputs_rhs_indices(90, 15, 16, 1.0, 17, 1.0);s.store_mul_mixed_ia(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1578] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1578]) {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
        if (s.b[1562] && (!s.b[1578])) {s.store_scaled_add_mixed_ia(84, 12, A::sqrt_square_offset(s.ad_value(12), ((0.25 * 0.1) * 0.1)), 0.5);}
        if s.b[1562] {s.store_mul_mixed_ia(13, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(144), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1579] = ((0.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.1)));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1579]) {s.store_div_from_scalar_scaled_input(85, ((-0.1) * 0.1), 13, 16.0);}
        if (s.b[1562] && (!s.b[1579])) {s.store_scaled_add_mixed_ia(85, 13, A::sqrt_square_offset(s.ad_value(13), ((0.25 * 0.1) * 0.1)), 0.5);}
        if s.b[1562] {s.store_scaled_add(86, 84, 85, 0.5);s.store_mul_ad_product_rhs_mixed_ia(80, 90, 108, A::add(s.ad_value(200), s.ad_value(144)));s.store_add_scaled_inputs(156, 86, s.v[155], 80, (s.v[158] * s.v[155]));s.store_offset(13, 203, 1.0);s.store_div_scaled_inputs2_indices(204, 60, 1.0, 109, p.p136, 13, 1.0);s.store_div(205, 234, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1580] = (s.v[204] < 0.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1580]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1562] && (!s.b[1580])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1562] {s.store_mul_add_scaled_product_pow_rhs(15, 506, 1.0, 516, 62, 1.0, 156, 407);s.store_offset(16, 15, 1.0);}
        s.b[1581] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1581]) {s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
    ) {
        if (s.b[1562] && (!s.b[1581])) {s.store_scaled_add_offset_sqrt_square_offset(159, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        if s.b[1562] {s.store_div_scaled_product_by_product_indices(138, 499, 108, 2.0, 159, 411, s.v[34]);s.store_sub(87, 200, 144);s.store_mul_ad_affine_product_rhs(13, 138, s.ad_value(87), A::mul(s.ad_value(138), s.ad_value(87)), 2.0, 0.0);s.store_sqrt_offset_input(161, 13, 1.0);s.store_scaled_offset(162, 161, 1.0, 0.5);s.store_div_scaled_inputs_mixed_ia(134, 411, 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);s.store_scale(135, 134, s.v[34]);s.store_add(170, 141, 135);s.store_sub(167, 75, 139);}
        s.b[1582] = (s.v[542] != 0.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if s.b[1582] {s.store_offset_mul_ad(176, s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(542), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);}
        if (!s.b[1582]) {s.store_scalar(176, 1.0);}
        s.store_square(207, 176);s.store_div_from_scalar(208, 1.0, 176);s.store_div_from_scalar(209, 1.0, 207);s.store_offset(210, 176, (-1.0));s.store_sub(213, 60, 91);s.store_sub(216, 200, 144);s.store_square_ad(217, A::sub(s.ad_value(200), s.ad_value(144)));s.store_add_scaled_inputs(211, 213, 1.0, 200, 2.0);s.store_add_scaled_inputs(212, 213, 1.0, 144, 2.0);s.b[1583] = ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5)));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if s.b[1583] {s.store_div_from_scalar_scaled_input(13, ((-0.5) * 0.5), 211, 16.0);}
        if (!s.b[1583]) {s.store_scaled_add_mixed_ia(13, 211, A::sqrt_square_offset(s.ad_value(211), ((0.25 * 0.5) * 0.5)), 0.5);}
        s.b[1584] = ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5)));s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
        if s.b[1584] {s.store_div_from_scalar_scaled_input(14, ((-0.5) * 0.5), 212, 16.0);}
        if (!s.b[1584]) {s.store_scaled_add_mixed_ia(14, 212, A::sqrt_square_offset(s.ad_value(212), ((0.25 * 0.5) * 0.5)), 0.5);}
        s.store_sqrt_offset_ad(214, A::mul(s.ad_value(13), s.ad_value(218)), 0.25);s.store_sqrt_offset_ad(215, A::mul(s.ad_value(14), s.ad_value(218)), 0.25);s.store_div_mixed_ia(13, 211, A::scale_offset(s.ad_value(214), 2.0, 1.0));s.store_div_mixed_ia(14, 212, A::scale_offset(s.ad_value(215), 2.0, 1.0));s.store_add(15, 214, 215);s.store_div_scaled_value_by_product_mixed_iai(16, 217, 0.3333333333333333, A::square(s.ad_value(15)), 15, 1.0);s.store_div_scaled_product3_mixed_iiia(17, 783, 162, 208, 1.0, A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)), 1.0);s.store_mul_scale_offset_mixed_ia(18, 17, A::add_scaled_square_product(s.ad_value(15), 1.0, s.ad_value(214), s.ad_value(215), 1.0), 0.8, 0.0);s.store_add_scaled_inputs(19, 18, 1.0, 218, 2.0);s.store_scaled_mul(20, 217, 17, 0.3333333333333333);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_div_scaled_product_mixed_iaa(202, 212, A::scale_offset(s.ad_value(215), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(215), 2.0, 1.0), 1.0);s.store_add_mixed_ai(201, A::add_scaled_offset_product_lhs(s.ad_value(213), 1.0, s.ad_value(90), (-1.0), s.ad_value(144), (-2.0)), 202);s.store_add_scaled_products_mixed_iaii(189, 208, A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, A::add_scaled_products(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(90), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(20), 1.0), (-1.0)), 1.0), 1.0, 210, 201, 1.0);s.store_add(21, 200, 144);s.store_mul3_lhs(22, 217, 17, 17);s.store_add_ad(194, A::mul3(s.ad_value(90), s.ad_value(208), A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(217), s.ad_value(17), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(90), s.ad_value(210), s.ad_value(144), 2.0));s.store_mul_ad_product_rhs_mixed_ia(191, 90, 209, A::add_scaled_product(s.ad_value(21), 0.5, s.ad_value(216), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(17))), 1.0, s.ad_value(22), 0.2), (-1.0 / (6.0))));s.store_mul_ad_product_lhs_mixed_ia(192, 90, A::sub(s.ad_value(176), s.ad_value(208)), 144);s.store_add(193, 191, 192);s.store_sub(190, 194, 193);s.b[1585] = ((0.0 == 0.0) && ((s.v[108] * s.v[189]) < ((-2500.0) * p.p694)));s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if s.b[1585] {s.store_div_scalar_by_product_indices(83, ((-p.p694) * p.p694), 108, 189, 16.0);}
        if (!s.b[1585]) {s.store_add_scaled_product_mixed_aii(83, A::sqrt_square_offset(A::mul(s.ad_value(108), s.ad_value(189)), ((0.25 * p.p694) * p.p694)), 0.5, 108, 189, 0.5);}
        s.store_mul_add_rhs(82, 108, 190, 193);s.store_add_scaled_inputs(12, 82, 1.0 / (p.p207), 83, (p.p208 * 1.0 / (p.p207)));s.store_offset_powf_ad(13, s.ad_value(12), (0.7 * p.p206), 1.0);s.store_div_from_scalar(227, (p.p205 * 1.9e-9), 13);s.store_div_from_scalar_ad(228, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(227), 1.0 / (s.v[47])));s.store_mul_ad_affine_product_lhs(195, A::div_from_scalar((8.85418e-12 * p.p111), s.ad_value(229)), s.ad_value(108), (((-p.p2) * s.v[33]) * s.v[34]), 0.0, 189);s.store_scaled_mul(199, 228, 108, ((p.p2 * s.v[33]) * s.v[34]));s.store_mul_scale_offset_indices(196, 190, 199, -1.0, 0.0);s.store_mul_scale_offset_indices(197, 193, 199, -1.0, 0.0);s.store_add_scaled_inputs3_indices(198, 195, (-1.0), 196, (-1.0), 197, (-1.0));s.b[1586] = (!param_given[666]);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if s.b[1586] {s.store_scalar(544, ((((2.0 * p.p111) * 8.85418e-12) / 3.141592653589793) * ((((p.p670 * (1.0 + (4e-7 / p.p77)))).max(1e-38)) as f64).ln()));}
        s.store_primal_offset(225, 544, p.p671);s.store_primal_offset(226, 544, p.p672);s.b[1587] = (p.p41 == 0.0);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
        if s.b[1587] {s.store_scaled_mul(223, 225, 231, ((-s.v[33]) * p.p2));s.store_scaled_mul(224, 226, 232, ((-s.v[33]) * p.p2));}
        if (!s.b[1587]) {s.store_sqrt_offset_ad(12, A::square(A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(219, 231, 0.5, 63, ((-1.0) * 0.5), 12, (-0.5), (0.02 * 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[1587]) {s.store_div_mixed_ia(18, 219, A::powf(A::offset(A::powf(A::scale(s.ad_value(219), (-1.0 / (p.p692))), p.p693), 1.0), (1.0 / p.p693)));s.store_sqrt_sub_from_scalar_ad(13, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(547), 1.0));s.store_add_scaled_products_mixed_iiia(223, 225, 231, ((-s.v[33]) * p.p2), 545, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(63), (-1.0), s.ad_value(219), -1.0), 1.0, s.ad_value(547), s.ad_value(13), (-1.0), (-0.5)), ((-s.v[33]) * p.p2));s.store_sqrt_offset_ad(12, A::square(A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02)), (4.0 * 0.02));s.store_add_scaled_inputs3_offset_indices(220, 232, 0.5, 63, ((-1.0) * 0.5), 12, (-0.5), (0.02 * 0.5));s.store_div_mixed_ia(18, 220, A::powf(A::offset(A::powf(A::scale(s.ad_value(220), (-1.0 / (p.p690))), p.p691), 1.0), (1.0 / p.p691)));s.store_sqrt_sub_from_scalar_ad(14, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(548), 1.0));s.store_add_scaled_products_mixed_iiia(224, 226, 232, ((-s.v[33]) * p.p2), 546, A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(232), 1.0, s.ad_value(63), (-1.0), s.ad_value(220), -1.0), 1.0, s.ad_value(548), s.ad_value(14), (-1.0), (-0.5)), ((-s.v[33]) * p.p2));}
        s.store_mul_scaled_voltage(221, 187, (((-p.p2) * s.v[34]) * p.p673), ctx, nodes, Some(10), Some(11));s.b[1588] = (p.p37 == 1.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if s.b[1588] {s.store_ln_ad(684, A::max_with_scalar(A::div(s.ad_value(686), s.ad_value(28)), 1e-38));s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(684)), 0.4), s.ad_value(489)), 0.4);s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(686), 1.60219e-19));}
        if s.b[1588] {
            s.store_mul_mixed_ia(674, 612, {
                            if (!((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0, 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1588] {s.store_mul_scale_offset_mixed_ia(673, 616, A::mul_offset_rhs(s.ad_value(623), s.ad_value(395), (-1.0)), 1.0, 1.0);}
        s.b[1589] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if (s.b[1588] && s.b[1589]) {s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));}
    }
}
