#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
    ) {
        let mut t3: usize = 0;
        while {
            let t2: f64 = if s.v[63] <= s.v[29] { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.copy_ad(279, 310);s.store_mul(297, 120, 279);s.store_exp_neg_input(278, 297);s.b[855] = (s.v[279] < (-1e-8));s.store_scalar(855, if s.b[855] { 1.0 } else { 0.0 });
            if s.b[855] {s.store_exp_mul(280, 120, 310);s.store_mul_sqrt_mixed_ia(314, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 314, 1.0);}
            s.b[856] = (s.v[279] > (1e-8 / 10.0));s.store_scalar(856, if s.b[856] { 1.0 } else { 0.0 });
            if ((!s.b[855]) && s.b[856]) {s.store_exp_mul(280, 120, 310);s.store_mul_scaled_sqrt_ad_rhs(314, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 314, 1.0);}
            if ((!s.b[855]) && (!s.b[856])) {s.store_scaled_mul(314, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(344, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));}
            s.store_add_scaled_inputs4_indices(309, 310, 1.0, 314, (-1.0 / (s.v[294])), 50, 1.0, 298, 1.0);s.store_sub_from_scalar_scaled_input(582, 1.0, 344, 1.0 / (s.v[294]));s.store_sub(279, 308, 584);s.store_mul(297, 120, 279);s.b[857] = ((-s.v[297]) >= 80.0);s.store_scalar(857, if s.b[857] { 1.0 } else { 0.0 });
            if s.b[857] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[857]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[858] = (s.v[279] < (-1e-8));s.store_scalar(858, if s.b[858] { 1.0 } else { 0.0 });
            if s.b[858] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(576, 141, 280);s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(578, 577);s.store_scalar(313, 0.0);s.store_scalar(579, 0.0);s.store_scalar(580, 0.0);}
            s.b[859] = (s.v[279] > 1e-8);s.store_scalar(859, if s.b[859] { 1.0 } else { 0.0 });
            if ((!s.b[858]) && s.b[859]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(576, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(578, 577);s.store_exp(278, 297);s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(576), s.ad_value(576), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(576), s.ad_value(577), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(576), s.ad_value(578), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(313, 576, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(579, 577, (-1.0), 141, 537, -1.0);s.store_add_scaled_product_indices(580, 578, (-1.0), 141, 538, -1.0);}
            if ((!s.b[858]) && (!s.b[859])) {s.store_scaled_mul(576, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(577, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(578, 577);s.store_scalar(313, 0.0);s.store_scalar(579, 0.0);s.store_scalar(580, 0.0);}
            s.store_sub(279, 309, 584);s.store_mul(297, 120, 279);s.b[860] = ((-s.v[297]) >= 80.0);s.store_scalar(860, if s.b[860] { 1.0 } else { 0.0 });
            if s.b[860] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[860]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[861] = (s.v[279] < (-1e-8));s.store_scalar(861, if s.b[861] { 1.0 } else { 0.0 });
            if s.b[861] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(585, 141, 280);s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(587, 586);s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);}
            s.b[862] = (s.v[279] > 1e-8);s.store_scalar(862, if s.b[862] { 1.0 } else { 0.0 });
            if ((!s.b[861]) && s.b[862]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(585, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(587, 586);s.store_exp(278, 297);s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(585), s.ad_value(585), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(585), s.ad_value(586), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(585), s.ad_value(587), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(588, 585, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(589, 586, (-1.0), 141, 539, -1.0);s.store_add_scaled_product_indices(590, 587, (-1.0), 141, 538, -1.0);}
            if ((!s.b[861]) && (!s.b[862])) {s.store_scaled_mul(585, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(586, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(587, 586);s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);}
            s.b[863] = s.b[379];s.store_scalar(863, if s.b[863] { 1.0 } else { 0.0 });
            if s.b[863] {s.store_scalar(574, s.v[63]);s.store_scalar(63, s.v[29]);}
            if (!s.b[863]) {s.store_add_scaled_inputs3_mixed_iia(346, 308, 1.0, 76, (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(314), 1.0, s.ad_value(313), 1.0, s.ad_value(576), 1.0, s.ad_value(588), 1.0), s.ad_value(585)), s.ad_value(337)), s.ad_value(270)), -1.0);s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(579), 1.0, s.ad_value(577), 1.0, s.ad_value(270), 1.0));s.store_div_scaled_inputs_mixed_ai(348, A::add_scaled_inputs4(s.ad_value(580), 1.0, s.ad_value(578), 1.0, s.ad_value(590), 1.0, s.ad_value(587), 1.0), -1.0, 270, 1.0);s.store_div_scaled_inputs_mixed_ai(349, A::add_scaled_product(s.ad_value(344), 1.0, A::add(s.ad_value(589), s.ad_value(586)), s.ad_value(582), 1.0), -1.0, 270, 1.0);}
            s.b[864] = (s.v[314] <= s.v[599]);s.store_scalar(864, if s.b[864] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[864]) {s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(314), 2.0, s.ad_value(296), 1.0));s.store_div_scaled_product_indices(604, 296, 344, 1.0, 279, 1.0);}
            s.b[865] = (s.v[314] <= s.v[603]);s.store_scalar(865, if s.b[865] { 1.0 } else { 0.0 });
            if (((!s.b[863]) && (!s.b[864])) && s.b[865]) {s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(602)));s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(314), 3.0, s.ad_value(602), (-3.0), s.ad_value(314), 1.0, s.ad_value(603), (-1.0)), 344);}
            if (((!s.b[863]) && (!s.b[864])) && (!s.b[865])) {s.store_scalar(279, 0.0);s.store_scalar(604, 0.0);}
            if (!s.b[863]) {s.store_div_scaled_inputs_indices(281, 316, (-s.v[650]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_neg_add(279, 296, 280);s.store_scalar(604, 0.0);s.store_scaled_add(350, 576, 279, 1.0 / (s.v[535]));s.store_scale(351, 577, 1.0 / (s.v[535]));s.store_scale(352, 578, 1.0 / (s.v[535]));s.store_scale(353, 604, 1.0 / (s.v[535]));s.store_div_scaled_inputs_indices(281, 316, (-s.v[651]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_scalar(605, 0.0);s.store_scaled_add(354, 585, 280, 1.0 / (s.v[535]));s.store_scale(355, 587, 1.0 / (s.v[535]));s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 586, 582, 1.0 / (s.v[535]));s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);}
            s.b[866] = (s.v[357] > 0.0);s.store_scalar(866, if s.b[866] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[866]) {s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);}
            if ((!s.b[863]) && (!s.b[866])) {s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));}
            if (!s.b[863]) {s.store_add_scaled_products_indices(359, 352, 356, 1.0, 353, 355, (-1.0));s.store_add_scaled_products_indices(360, 349, 355, 1.0, 348, 356, (-1.0));s.store_add_scaled_products_indices(361, 348, 353, 1.0, 349, 352, (-1.0));s.store_mul_scale_offset_indices(362, 356, 351, -1.0, 0.0);s.store_mul(363, 347, 356);s.store_add_scaled_products_indices(364, 349, 351, 1.0, 347, 353, (-1.0));s.store_mul(365, 351, 355);s.store_mul_scale_offset_indices(366, 355, 347, -1.0, 0.0);s.store_add_scaled_products_indices(367, 347, 352, 1.0, 348, 351, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(368, 358, 359, 346, -1.0, 360, 350, -1.0, 361, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(369, 358, 362, 346, -1.0, 363, 350, -1.0, 364, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(370, 358, 365, 346, -1.0, 366, 350, -1.0, 367, 354, -1.0);s.store_abs(279, 368);}
            s.b[867] = (s.v[279] < ((s.v[369]) as f64).abs());s.store_scalar(867, if s.b[867] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[867]) {s.store_abs(279, 369);}
            s.b[868] = (s.v[279] < ((s.v[370]) as f64).abs());s.store_scalar(868, if s.b[868] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[868]) {s.store_abs(279, 370);}
            if (!s.b[863]) {s.store_scalar(606, 1.0);}
            s.b[869] = (s.v[63] > 80.0);s.store_scalar(869, if s.b[869] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[869]) {s.store_scalar(606, 25.0);}
            s.b[870] = (s.v[63] > 40.0);s.store_scalar(870, if s.b[870] { 1.0 } else { 0.0 });
            if (((!s.b[863]) && (!s.b[869])) && s.b[870]) {s.store_scalar(606, 25.0);}
            s.b[871] = (s.v[63] > 20.0);s.store_scalar(871, if s.b[871] { 1.0 } else { 0.0 });
            if ((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && s.b[871]) {s.store_scalar(606, 25.0);}
            s.b[872] = (s.v[63] > 10.0);s.store_scalar(872, if s.b[872] { 1.0 } else { 0.0 });
            if (((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && (!s.b[871])) && s.b[872]) {s.store_scalar(606, 5.0);}
            s.b[873] = (s.v[279] > (0.1 / s.v[606]));s.store_scalar(873, if s.b[873] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[873]) {s.store_mul_mixed_ia(368, 368, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(369, 369, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(370, 370, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));}
            if (!s.b[863]) {s.store_add(308, 308, 368);s.store_add(584, 584, 369);s.store_add(310, 310, 370);}
            let (t1,) = {
    if (!s.b[863]) {
        let t0: f64 = (1e-12 * s.v[606]);
        (t0,)
    } else {
        (s.v[607],)
    }
};
            s.store_scalar(607, t1);s.b[874] = (s.v[279] < s.v[607]);s.store_scalar(874, if s.b[874] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[874]) {s.store_scalar(379, 1.0);}
            s.store_primal_offset(63, 63, 1.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[875] = (s.v[574] > 0.0);s.store_scalar(875, if s.b[875] { 1.0 } else { 0.0 });
        if s.b[875] {s.copy_ad(63, 574);s.store_scalar(574, 0.0);}
        s.b[876] = (s.v[63] > s.v[29]);s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });
        if s.b[876] {s.copy_ad(308, 302);s.copy_ad(309, 303);s.copy_ad(310, 304);s.copy_ad(584, 581);}
        s.copy_ad(57, 308);s.store_sub(59, 57, 56);s.copy_ad(51, 396);s.b[878] = ((s.v[292] <= (-1.0)) || (s.v[305] < 0.0));s.store_scalar(878, if s.b[878] { 1.0 } else { 0.0 });
        let (t4,) = {
    if s.b[878] {
        (1.0,)
    } else {
        (s.v[34],)
    }
};
        s.store_scalar(34, t4);s.copy_ad(317, 305);s.copy_ad(318, 308);s.store_sub(59, 318, 317);s.copy_ad(322, 306);s.copy_ad(323, 309);s.store_sub(155, 323, 322);s.store_add_scaled_inputs3_mixed_iia(153, 313, 1.0, 311, (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(313), s.ad_value(311)), A::sub(s.ad_value(318), s.ad_value(317)), 0.5), -1.0);s.store_add_scaled_inputs3_mixed_iia(154, 588, 1.0, 528, (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(588), s.ad_value(528)), A::sub(s.ad_value(323), s.ad_value(322)), 0.5), -1.0);s.b[879] = ((s.v[153] < 0.0) || (s.v[51] == 0.0));s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });
        if s.b[879] {s.store_scalar(153, 0.0);}
        s.b[880] = ((s.v[154] < 0.0) || (s.v[51] == 0.0));s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });
        if s.b[880] {s.store_scalar(154, 0.0);}
        s.store_add(151, 153, 154);s.store_scaled_add(384, 576, 523, (-0.5));s.store_offset_sub(371, 308, 305, 1e-12);s.store_sub(373, 311, 313);s.b[881] = ((-s.v[373]) < 1e-18);s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });
        if s.b[881] {s.store_scalar(373, 0.0);}
        s.store_offset_div_scaled_inputs_mixed_ia(372, 373, (-2.0), A::mul(A::mul3(s.ad_value(120), s.ad_value(270), s.ad_value(371)), s.ad_value(371)), 1.0, 1.0);s.store_sub_from_scalar_ad(85, 1.0, A::div_scaled_product(s.ad_value(372), s.ad_value(371), 1.0, s.ad_value(86), 1.0));s.b[882] = (s.v[85] <= 0.0);s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });
        if s.b[882] {s.store_scalar(85, 0.0);}
        s.store_scaled_add(383, 311, 313, (-0.5));s.store_scaled_add(167, 528, 588, (-0.5));s.store_scalar(262, 0.0);s.b[883] = (s.v[34] == 0.0);s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });s.b[884] = ((s.v[446] < (10.0 * 2.220446049250313e-16)) && (p.p178 < (10.0 * 2.220446049250313e-16)));s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });
        if (s.b[883] && s.b[884]) {s.store_scalar(262, 0.0);s.copy_ad(260, 57);}
        s.b[885] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });
        if ((s.b[883] && s.b[884]) && s.b[885]) {s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));}
        if (s.b[883] && (!s.b[884])) {s.store_scalar(263, p.p227);s.store_div_from_scalar_ad(282, 1.034943e-10, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(149), p.p178, s.ad_value(263), 1.0), 1.0, s.ad_value(446), s.ad_value(126), 1.0));s.store_add_scaled_inputs3_indices(260, 51, p.p176, 56, p.p176, 57, (1.0 - p.p176));}
        s.b[886] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });
        if ((s.b[883] && (!s.b[884])) && s.b[886]) {s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));}
        if (s.b[883] && (!s.b[884])) {s.store_sub(284, 260, 57);s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(278, 284, 639, 0.5, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[883] && (!s.b[884])) {s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[887] = (s.v[284] < 0.0);s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });
        if ((s.b[883] && (!s.b[884])) && s.b[887]) {s.store_scalar(284, 0.0);s.store_scalar(278, 0.0);}
        if (s.b[883] && (!s.b[884])) {s.store_div_scaled_value_by_product_indices(283, 151, 1.0, 120, 149, 1.0);s.store_scale(288, 126, 9662367879.197212);s.store_scalar(279, 1000000000.0);s.store_div_scaled_inputs_product_mixed_iaiii(387, 283, 2.0, A::mul3_scaled_output(s.ad_value(288), s.ad_value(284), s.ad_value(282), 2.0), 1.0, 279, 282, 1.0, 123, 1.0);s.store_mul(285, 387, 282);s.store_add_scaled_product_indices(387, 279, 4.0, 288, 284, (2.0 * 4.0));s.store_mul3_lhs(286, 387, 282, 282);s.store_sqrt_square_add(287, 285, 286);s.store_scaled_sub(262, 287, 285, 0.5);s.copy_ad(279, 262);s.store_mul(262, 276, 279);}
        if s.b[883] {s.store_scale(262, 262, s.v[483]);}
        s.store_sub(386, 123, 262);s.b[888] = (s.v[386] < 1e-9);s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });
        if s.b[888] {s.store_scalar(386, 1e-9);}
        s.store_mul_add_scaled_inputs_rhs_indices(91, 123, 383, (-s.v[513]), 167, (-s.v[513]));s.store_mul_scale_offset_mixed_ai(336, A::add(s.ad_value(312), s.ad_value(314)), 123, (0.5 * s.v[513]), 0.0);s.store_scaled_sub(279, 51, 59, 0.5);s.store_scale(638, 279, (2.0 * 1.0 / (p.p217)));s.store_offset_mul_offset_rhs_mixed_ia(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(75, p.p217, 639);s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);s.b[889] = (s.v[75] < (10.0 * 2.220446049250313e-16));s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });
        if s.b[889] {s.store_scalar(75, (10.0 * 2.220446049250313e-16));}
        s.store_add(74, 56, 75);s.store_scalar(499, (1.034943e-10 / 100.0));s.store_scale(500, 313, 0.0001);s.store_scale(501, 588, 0.0001);s.store_scale(504, 531, 0.0001);s.store_scale(505, 585, 0.0001);s.store_scale(502, 383, 0.0001);s.store_scale(503, 167, 0.0001);s.store_scale(504, 531, 0.0001);s.store_scale(505, 585, 0.0001);s.store_scale(506, 384, 0.0001);s.store_scalar(507, (p.p229 * 100.0));s.store_scalar(591, ((p.p81 * (1.0 + (p.p82 / ((s.v[375]) as f64).powf(p.p83)))) / s.v[499]));s.store_scalar(592, ((p.p78 * (1.0 + (p.p79 / ((s.v[375]) as f64).powf(p.p80)))) / s.v[499]));s.store_sqrt_square_offset(639, 59, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(278, 59, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(598, 59, 0.5, 639, 0.5, (1e-10 * 1e-6));s.b[890] = (s.v[598] < 0.0);s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });
        if s.b[890] {s.store_scalar(598, 0.0);s.store_scalar(278, 0.0);}
        s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));s.store_powf(168, 168, p.p85);s.store_offset_scaled(282, 168, p.p84, 1.0);s.store_scalar(497, (p.p299 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));s.store_sub_scaled_inputs(288, 502, 1.0, 501, s.v[497]);s.store_add_scaled_inputs(283, 506, s.v[592], 288, s.v[591]);s.store_div(156, 283, 282);
        if (p.p32 != 0.0) {s.store_scaled_add(596, 306, 309, 0.5);s.store_scaled_add(597, 307, 310, 0.5);s.store_add_scaled_inputs3_indices(163, 596, (3.9 * 1.0 / ((11.7 * s.v[507]))), 597, ((-1.0) * (3.9 * 1.0 / ((11.7 * s.v[507])))), 440, (-(3.9 * 1.0 / ((11.7 * s.v[507])))));s.store_add(156, 156, 163);}
        if (p.p32 == 0.0) {s.store_scalar(596, 0.0);s.store_scalar(597, 0.0);s.store_scalar(163, 0.0);}
        s.store_sqrt_square_offset(639, 156, ((4.0 * 3000.0) * 3000.0));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_scaled_div(279, 156, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(156, 156, 0.5, 639, 0.5, (1e-10 * 3000.0));s.b[891] = (s.v[156] < 0.0);s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });
        if s.b[891] {s.store_scalar(156, 0.0);s.store_scalar(279, 0.0);}
        s.store_powf(286, 156, p.p94);s.store_powf(284, 156, s.v[470]);s.store_scale(157, 502, 6.241449993689894e18);s.store_add_scaled_inputs_mixed_ai(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[449] * 1e-11), s.v[448])), 1.0, s.ad_value(469), s.ad_value(286), 1.0), 1.0, 284, 1.0 / (p.p105));s.store_div_from_scalar(159, 1.0, 279);s.store_scale(159, 159, 0.0001);
        if (p.p32 != 0.0) {s.store_scaled_sub(163, 596, 597, (3.9 * 1.0 / ((11.7 * s.v[507]))));}
        if (p.p32 == 0.0) {s.store_sqrt_square_offset(639, 155, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(278, 155, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(598, 155, 0.5, 639, 0.5, (1e-10 * 1e-6));}
        s.b[892] = (s.v[598] < 0.0);s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });
        if ((p.p32 == 0.0) && s.b[892]) {s.store_scalar(598, 0.0);s.store_scalar(278, 0.0);}
        if (p.p32 == 0.0) {s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));s.store_powf(168, 168, p.p85);s.store_offset_scaled(282, 168, p.p84, 1.0);s.store_scalar(498, (p.p302 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));s.store_add_scaled_product_indices(288, 503, 1.0, 498, 500, (-1.0));s.store_scaled_add(508, 505, 504, (-0.5));s.store_add_scaled_inputs(283, 508, s.v[592], 288, s.v[591]);s.store_div(163, 283, 282);}
        s.store_sqrt_square_offset(639, 163, ((4.0 * 30.0) * 30.0));s.store_offset_scaled_div(279, 163, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(163, 163, 0.5, 639, 0.5, (1e-10 * 30.0));s.b[893] = (s.v[163] < 0.0);s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });
        if s.b[893] {s.store_scalar(163, 0.0);s.store_scalar(279, 0.0);}
        s.store_powf(286, 163, p.p275);s.store_powf(284, 163, s.v[594]);s.store_scale(157, 503, 6.241449993689894e18);s.store_add_scaled_inputs_mixed_ai(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[451] * 1e-11), s.v[450])), 1.0, s.ad_value(595), s.ad_value(286), 1.0), 1.0, 284, 1.0 / (p.p284));s.store_div_from_scalar(166, 1.0, 279);s.store_scale(166, 166, 0.0001);s.store_div_scaled_inputs_indices(454, 162, 0.2, 159, 1.0);s.store_div_mixed_ia(291, 153, A::mul3(s.ad_value(120), A::offset(s.ad_value(149), 1e-50), s.ad_value(386)));s.store_sqrt_square_sum(160, 291, 454);s.store_mul(161, 159, 160);s.store_div(279, 161, 162);s.b[894] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });
        if s.b[894] {s.store_scalar(281, 1.0);}
        s.b[895] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });
        if ((!s.b[894]) && s.b[895]) {s.copy_ad(281, 279);}
        if ((!s.b[894]) && (!s.b[895])) {s.store_powf(281, 279, (p.p114 - 1.0));}
        s.store_offset_mul(282, 279, 281, 1.0);s.b[896] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(896, if s.b[896] { 1.0 } else { 0.0 });
        if s.b[896] {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[897] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });
        if ((!s.b[896]) && s.b[897]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((!s.b[896]) && (!s.b[897])) {s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));s.store_mul(283, 282, 284);}
        s.store_mul(158, 159, 283);s.store_div_scaled_inputs_indices(455, 162, 0.2, 166, 1.0);s.store_div_mixed_ia(291, 154, A::mul3(s.ad_value(120), A::offset(s.ad_value(150), 1e-50), s.ad_value(386)));s.store_sqrt_square_sum(164, 291, 455);s.store_mul(161, 166, 164);s.store_div(279, 161, 162);s.b[898] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[898] {s.store_scalar(281, 1.0);}
        s.b[899] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
        if ((!s.b[898]) && s.b[899]) {s.copy_ad(281, 279);}
        if ((!s.b[898]) && (!s.b[899])) {s.store_powf(281, 279, (p.p114 - 1.0));}
        s.store_offset_mul(282, 279, 281, 1.0);s.b[900] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
        if s.b[900] {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[901] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
        if ((!s.b[900]) && s.b[901]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((!s.b[900]) && (!s.b[901])) {s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));s.store_mul(283, 282, 284);}
        s.store_mul(165, 166, 283);s.store_div_scaled_inputs_mixed_ia(189, 122, s.v[466], A::sub(s.ad_value(123), s.ad_value(262)), 1.0);s.store_mul3_lhs(96, 189, 153, 158);s.store_mul3_lhs(97, 189, 154, 165);s.store_add(95, 96, 97);s.store_scalar(173, 0.0);s.store_scalar(169, 0.0);s.store_scalar(171, 0.0);s.store_scalar(172, 0.0);s.b[902] = (p.p239 != 0.0);s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
        if s.b[902] {s.store_scaled_sub(279, 51, 59, 0.5);s.store_scale(638, 279, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(284, 0.01, 639);s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(56), s.ad_value(284)));s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(280, 279, 0.5, 639, 0.5, (1e-10 * 0.05));}
        s.b[903] = (s.v[280] < 0.0);s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });
        if (s.b[902] && s.b[903]) {s.store_scalar(280, 0.0);s.store_scalar(278, 0.0);}
        if s.b[902] {s.store_mul_ad_affine_product_rhs(287, 270, s.ad_value(120), A::powf(s.ad_value(280), p.p240), s.v[475], 0.0);s.store_add_scaled_product_mixed_aia(282, A::scale_offset(s.ad_value(71), p.p241, 1.0), 1.0, 71, A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(284), 1.0, s.ad_value(70), -1.0), s.v[476]);s.store_mul(287, 287, 282);}
        if (!s.b[902]) {s.store_scalar(287, 0.0);}
        s.b[904] = (p.p246 != 0.0);s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
        if s.b[904] {s.store_mul3_affine_lhs(286, 270, 120, s.v[477], 0.0, 71);}
        if (!s.b[904]) {s.store_scalar(286, 0.0);}
        s.b[905] = ((s.v[287] + s.v[286]) > 0.0);s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });
        if s.b[905] {s.store_mul_add_rhs(152, 59, 287, 286);s.store_mul3_lhs(173, 189, 152, 158);s.store_div_from_scalar_offset_ad(172, 1.0, A::exp_scaled_input(s.ad_value(440), (-p.p245)), 1.0);s.store_sub_from_scalar(171, 1.0, 172);s.store_mul(169, 171, 173);}
        s.store_scalar(174, 0.0);s.store_scalar(170, 0.0);s.b[906] = (p.p239 != 0.0);s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
        if s.b[906] {s.store_scaled_sub(279, 51, 155, 0.5);s.store_scale(638, 279, (2.0 * 100.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[906] {s.store_offset_mul_offset_rhs_mixed_ia(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(284, 0.01, 639);s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(322), s.ad_value(284)));s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(280, 279, 0.5, 639, 0.5, (1e-10 * 0.05));}
        s.b[907] = (s.v[280] < 0.0);s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });
        if (s.b[906] && s.b[907]) {s.store_scalar(280, 0.0);s.store_scalar(278, 0.0);}
        if s.b[906] {s.store_mul_ad_affine_product_rhs(287, 270, s.ad_value(120), A::powf(s.ad_value(280), p.p240), s.v[475], 0.0);s.store_add_scaled_product_mixed_aia(282, A::scale_offset(s.ad_value(71), p.p241, 1.0), 1.0, 71, A::add_scaled_inputs3(s.ad_value(322), 1.0, s.ad_value(284), 1.0, s.ad_value(70), -1.0), s.v[476]);s.store_mul(287, 287, 282);}
        if (!s.b[906]) {s.store_scalar(287, 0.0);}
        s.b[908] = ((s.v[287] + s.v[286]) > 0.0);s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });
        if s.b[908] {s.store_mul_add_rhs(152, 155, 287, 286);s.store_mul3_lhs(174, 189, 152, 165);}
        s.b[909] = ((s.v[174] > (s.v[173] - (s.v[173] * 0.05))) && ((s.v[173] * 0.05) >= 0.0));s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });
        if (s.b[908] && s.b[909]) {s.store_add_scaled_inputs3_indices(638, 174, 1.0, 173, (-1.0), 173, 0.05);s.store_square(642, 638);s.store_scaled_mul(643, 173, 173, (0.05 * 0.05));s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);}
        let (t5,) = {
    if (s.b[908] && s.b[909]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, t5);
        let (t6,) = {
    if (s.b[908] && s.b[909]) {
        (0.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t6);
        if (s.b[908] && s.b[909]) {s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[910] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });s.b[911] = (2.0 == 1.0);s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });
        let (t7,) = {
    if (((s.b[908] && s.b[909]) && s.b[910]) && s.b[911]) {
        (1.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t7);s.b[912] = (2.0 == 2.0);s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
        let (t8,) = {
    if ((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && s.b[912]) {
        (2.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t8);s.b[913] = (2.0 == 4.0);s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
        let (t9,) = {
    if (((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {
        (3.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t9);s.b[914] = (2.0 == 8.0);s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
        let (ta,) = {
    if ((((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && (!s.b[913])) && s.b[914]) {
        (4.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, ta);
        let (tb,) = {
    if ((s.b[908] && s.b[909]) && s.b[910]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, tb);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((s.b[908] && s.b[909]) && s.b[910]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;assert!(tf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[908] && s.b[909]) && s.b[910]) {s.store_sqrt(646, 646);}
            let (td,) = {
    if ((s.b[908] && s.b[909]) && s.b[910]) {
        let tc: f64 = (s.v[647] + 1.0);
        (tc,)
    } else {
        (s.v[647],)
    }
};
            s.store_scalar(647, td);
        }
        if ((s.b[908] && s.b[909]) && (!s.b[910])) {s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));}
        if (s.b[908] && s.b[909]) {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_mul3_affine_lhs(637, 638, 173, 0.05, 0.0, 646);s.store_div_scaled_product3_mixed_iiia(278, 173, 645, 646, 0.05, A::offset(s.ad_value(220), 1e-50), 1.0);s.store_add_scaled_inputs3_indices(174, 173, 1.0, 173, (-0.05), 637, 1.0);}
        if (s.b[908] && s.b[909]) {
        }
        if (s.b[908] && (!s.b[909])) {
        }
        if (s.b[908] && (!s.b[909])) {s.store_scalar(278, 1.0);}
        if s.b[908] {s.store_mul(170, 172, 174);}
        s.store_add(175, 169, 170);s.store_add(94, 95, 175);s.b[915] = (p.p22 != 0.0);s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });
        if s.b[915] {s.store_scale(279, 271, 1.034943e-10);s.copy_ad(280, 132);s.store_scalar(281, (s.v[133] - p.p57));s.store_div_from_scalar_square_ad(282, 1.0, s.ad_value(281));s.store_mul_ad_product_lhs_mixed_ai(283, A::mul_sub_from_scalar_lhs_scaled_output(p.p55, s.ad_value(130), s.ad_value(279), 2.0), 280, 282);s.store_mul(81, 283, 135);s.store_scalar(282, p.p158);s.store_scalar(284, p.p159);s.store_add_scaled_product_indices(279, 282, 1.0, 284, 71, 1.0);s.store_mul(98, 81, 279);s.store_sub_from_scalar_scaled_input(279, p.p160, 51, p.p161);s.store_add_scaled_inputs4_indices(99, 72, 1.0, 138, (-1.0), 279, 1.0, 98, 1.0);s.store_mul3_lhs(102, 119, 271, 271);s.store_scaled_mul(103, 102, 120, 0.5);s.store_scaled_mul(104, 103, 120, 2.0);s.store_scale(387, 120, 0.25);s.store_offset_add_scaled_inputs3_offset_mixed_aii(288, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(102), s.ad_value(387), (-1.0)), 1.0, 138, 1.0, 98, -1.0, (-p.p160), 1e-50);s.store_offset_sub(279, 72, 288, (-0.005));}
        if s.b[915] {s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));}
        if s.b[915] {s.store_sqrt_add_scaled_square_product(280, 279, 1.0, 278, 288, (4.0 * 0.005));s.store_add_scaled_inputs3_mixed_aii(281, A::offset(A::add_scaled_inputs4(s.ad_value(288), 1.0, s.ad_value(279), 0.5, s.ad_value(280), 0.5, s.ad_value(138), -1.0), p.p160), 1.0, 98, 1.0, 70, -1.0);s.store_offset_mul(282, 120, 281, (-1.0));s.store_div_from_scalar(283, 4.0, 104);s.store_offset_mul(279, 282, 283, 1.0);s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[916] = (s.v[279] < 0.0);s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[916]) {s.store_scalar(279, 0.0);s.store_scalar(280, 0.0);}
        if s.b[915] {s.store_offset(279, 279, 1e-50);s.store_sqrt(105, 279);s.store_mul_scale_offset_indices(278, 103, 105, -1.0, 1.0);s.store_add(107, 99, 278);s.store_div_from_scalar_add_ad(278, 1.0, s.ad_value(120), A::div_scalar_offset_denominator(2.0, s.ad_value(99), 1e-50, 1.0));s.store_mul_ln_mixed_ia(109, 278, A::mul(A::div_scalar_by_product(1.0, s.ad_value(101), s.ad_value(102), 1.0), A::square(s.ad_value(99))));s.store_div_scaled_value_offset_denominator(281, s.ad_value(109), 1.0, s.ad_value(99), 1e-50, 1.0);s.store_offset_sub(110, 109, 107, (-p.p136));s.store_add_scaled_inputs_mixed_ai(278, A::square(s.ad_value(110)), 1.0, 109, (4.0 * p.p136));s.store_sqrt_square_offset(639, 278, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(280, 278, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(278, 278, 0.5, 639, 0.5, (1e-10 * 1e-6));}
        s.b[917] = (s.v[278] < 0.0);s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[917]) {s.store_scalar(278, 0.0);s.store_scalar(280, 0.0);}
        if s.b[915] {s.store_sqrt(278, 278);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
    ) {
        if s.b[915] {s.store_add_scaled_inputs3_indices(111, 109, 1.0, 110, (-0.5), 278, (-0.5));s.store_div_from_scalar(279, 1.0, 278);s.store_mul_exp_mixed_ia(278, 101, A::mul(s.ad_value(120), s.ad_value(111)));s.store_add_offset_lhs_mixed_ai(279, A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0), 278);s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[918] = (s.v[279] < 0.0);s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[918]) {s.store_scalar(279, 0.0);s.store_scalar(278, 0.0);}
        if s.b[915] {s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));s.store_sqrt(113, 279);s.store_offset_mul_ad(279, s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70)), (-1.0));s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[919] = (s.v[279] < 0.0);s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[919]) {s.store_scalar(279, 0.0);s.store_scalar(278, 0.0);}
        if s.b[915] {s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));s.store_sqrt(114, 279);s.store_mul_sub_rhs(115, 100, 113, 114);s.store_sub(279, 107, 111);s.store_sqrt_square_offset(639, 279, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.1));}
        s.b[920] = (s.v[279] < 0.0);s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[920]) {s.store_scalar(279, 0.0);s.store_scalar(280, 0.0);}
        if s.b[915] {s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));s.store_div(290, 51, 279);s.store_square(642, 290);s.store_scalar(643, 1.0);s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);}
        let (t10,) = {
    if s.b[915] {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, t10);
        let (t11,) = {
    if s.b[915] {
        (0.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t11);
        if s.b[915] {s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[921] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });s.b[922] = (4.0 == 1.0);s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
        let (t12,) = {
    if ((s.b[915] && s.b[921]) && s.b[922]) {
        (1.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t12);s.b[923] = (4.0 == 2.0);s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });
        let (t13,) = {
    if (((s.b[915] && s.b[921]) && (!s.b[922])) && s.b[923]) {
        (2.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t13);s.b[924] = (4.0 == 4.0);s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
        let (t14,) = {
    if ((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && s.b[924]) {
        (3.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t14);s.b[925] = (4.0 == 8.0);s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        let (t15,) = {
    if (((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && (!s.b[924])) && s.b[925]) {
        (4.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t15);
        let (t16,) = {
    if (s.b[915] && s.b[921]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, t16);let mut t1a: usize = 0;
        while {
            let t19: f64 = if ((s.b[915] && s.b[921]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            t19 != 0.0
        } {
            t1a += 1;assert!(t1a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[915] && s.b[921]) {s.store_sqrt(646, 646);}
            let (t18,) = {
    if (s.b[915] && s.b[921]) {
        let t17: f64 = (s.v[647] + 1.0);
        (t17,)
    } else {
        (s.v[647],)
    }
};
            s.store_scalar(647, t18);
        }
        if (s.b[915] && (!s.b[921])) {s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));}
        if s.b[915] {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[915] {s.store_scaled_mul(291, 290, 646, 1.0);s.store_div_scaled_product_offset_denominator_indices(280, 645, 646, 1.0, 220, 1e-50, 1.0);s.store_scale(106, 122, ((2.0 * s.v[453]) * p.p5));s.copy_ad(279, 386);s.store_div_scaled_product_mixed_aii(116, A::mul3(s.ad_value(106), s.ad_value(158), s.ad_value(115)), 291, 1.0, 279, 1.0);s.store_add(94, 94, 116);}
        s.b[926] = ((p.p20 != 0.0) && (p.p23 != 0.0));s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });
        if s.b[926] {s.store_square(231, 86);s.store_mul3_affine_lhs(232, 122, 271, 2.0, 0.0, 151);s.store_sub(233, 231, 232);s.store_sqrt_square_offset(639, 231, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(278, 231, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(231, 231, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[927] = (s.v[231] < 0.0);s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });
        if (s.b[926] && s.b[927]) {s.store_scalar(231, 0.0);s.store_scalar(278, 0.0);}
        if s.b[926] {s.store_sqrt_square_offset(639, 233, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(278, 233, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(233, 233, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[928] = (s.v[233] < 0.0);s.store_scalar(928, if s.b[928] { 1.0 } else { 0.0 });
        if (s.b[926] && s.b[928]) {s.store_scalar(233, 0.0);s.store_scalar(278, 0.0);}
        if s.b[926] {s.store_sub(234, 231, 233);}
        s.b[929] = ((s.v[149] < (10.0 * 2.220446049250313e-16)) || (s.v[234] < (10.0 * 2.220446049250313e-16)));s.store_scalar(929, if s.b[929] { 1.0 } else { 0.0 });
        let (t1b,) = {
    if (s.b[926] && s.b[929]) {
        (0.0,)
    } else {
        (s.v[35],)
    }
};
        s.store_scalar(35, t1b);
        let (t1c,) = {
    if (s.b[926] && (!s.b[929])) {
        (1.0,)
    } else {
        (s.v[35],)
    }
};
        s.store_scalar(35, t1c);s.b[930] = (s.v[185] > 0.0);s.store_scalar(930, if s.b[930] { 1.0 } else { 0.0 });
        if s.b[930] {s.copy_ad(279, 388);s.store_square(285, 270);s.store_mul_div_from_scalar_lhs_ad_indices(282, 2.0, 472, 285);s.store_add_scaled_inputs3_indices(283, 279, 1.0, 122, (-1.0), 70, (-s.v[486]));s.store_offset_mul(284, 282, 283, 1.0);s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[931] = (s.v[284] < 0.0);s.store_scalar(931, if s.b[931] { 1.0 } else { 0.0 });
        if (s.b[930] && s.b[931]) {s.store_scalar(284, 0.0);s.store_scalar(287, 0.0);}
        if s.b[930] {s.store_offset(284, 284, 1e-50);s.store_add_scaled_inputs_mixed_ia(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))), 1.0);s.store_add_scaled_inputs3_indices(187, 71, p.p123, 339, 1.0, 186, (-(s.v[487] * s.v[485])));s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(187, 187, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[932] = (s.v[187] < 0.0);s.store_scalar(932, if s.b[932] { 1.0 } else { 0.0 });
        if (s.b[930] && s.b[932]) {s.store_scalar(187, 0.0);s.store_scalar(287, 0.0);}
        if s.b[930] {s.store_offset(187, 187, 1e-50);s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));s.store_mul3_affine_lhs(185, 187, 94, s.v[488], 0.0, 280);}
        s.b[933] = (((s.v[34] == 0.0) && (s.v[185] > 0.0)) && (p.p145 != 0.0));s.store_scalar(933, if s.b[933] { 1.0 } else { 0.0 });
        if s.b[933] {s.store_offset_scaled(278, 80, p.p146, 1.0);s.store_scaled_mul(188, 278, 185, p.p145);s.store_offset_mul(64, 120, 56, (-1.0));s.store_sqrt_square_offset(639, 64, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(64, 64, 0.5, 639, 0.5, (1e-10 * 0.1));}
        s.b[934] = (s.v[64] < 0.0);s.store_scalar(934, if s.b[934] { 1.0 } else { 0.0 });
        if (s.b[933] && s.b[934]) {s.store_scalar(64, 0.0);}
        if s.b[933] {s.store_sqrt(65, 64);s.store_mul(66, 64, 65);s.store_offset_mul(69, 120, 57, (-1.0));s.store_sqrt_square_offset(639, 69, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(69, 69, 0.5, 639, 0.5, (1e-10 * 0.1));}
        s.b[935] = (s.v[69] < 0.0);s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });
        if (s.b[933] && s.b[935]) {s.store_scalar(69, 0.0);}
        if s.b[933] {s.store_sqrt(67, 69);s.store_mul(68, 69, 67);s.store_div_scaled_product_indices(279, 120, 188, 1.0, 64, 1.0);s.store_div_scaled_product_indices(280, 120, 188, 1.0, 69, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[933] {s.store_mul_mixed_ia(190, 141, A::add_scaled_products(s.ad_value(68), s.ad_value(280), 1.0, s.ad_value(66), s.ad_value(279), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(191, 141, 67, 280, ((-1.0) * (0.5)), 65, 279, 0.5);s.store_add(192, 190, 191);s.store_mul3_lhs(193, 189, 192, 158);}
        s.store_scalar(949, (s.v[272] * 100.0));s.store_scale(950, 270, 0.0001);s.store_scale(951, 123, 100.0);s.store_scalar(952, (s.v[466] * 100.0));s.store_scale(953, 160, 0.01);s.store_scale(954, 383, 0.0001);s.store_scale(955, 141, 0.0001);s.b[956] = (p.p17 == 0.0);s.store_scalar(956, if s.b[956] { 1.0 } else { 0.0 });
        if s.b[956] {s.store_scalar(255, 0.0);s.store_scalar(250, 0.0);s.store_scalar(251, 0.0);s.store_scalar(254, 0.0);s.store_scalar(256, 0.0);}
        s.b[957] = (s.v[34] == 0.0);s.store_scalar(957, if s.b[957] { 1.0 } else { 0.0 });
        if ((!s.b[956]) && s.b[957]) {s.store_offset_add(948, 74, 71, (-(10.0 * 2.220446049250313e-16)));s.store_add_scaled_inputs4_mixed_iiai(938, 72, 1.0, 138, (-p.p256), A::div_scaled_inputs3(s.ad_value(50), (-p.p258), s.ad_value(80), p.p206, s.ad_value(267), (-p.p206), s.ad_value(951), 1.0), 1.0, 948, (-p.p205));s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[958] = (s.v[947] < 0.0);s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && s.b[957]) && s.b[958]) {s.store_scalar(947, 0.0);s.store_scalar(942, 0.0);}
        if ((!s.b[956]) && s.b[957]) {s.store_sqrt_square_offset(639, 72, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(941, 72, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(940, 72, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[959] = (s.v[940] < 0.0);s.store_scalar(959, if s.b[959] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && s.b[957]) && s.b[959]) {s.store_scalar(940, 0.0);s.store_scalar(941, 0.0);}
        if ((!s.b[956]) && s.b[957]) {s.store_scaled_offset(936, 940, (-p.p216), 10.0);s.store_sub_from_scalar_ad(938, 1.0, A::div_scalar_offset_denominator(1.0, A::square(s.ad_value(936)), 1.0, 1.0));s.store_mul(947, 947, 938);s.store_scale(937, 951, s.v[952]);s.store_div_from_scalar_offset_input(944, p.p209, 937, p.p209);s.store_scalar(943, p.p208);s.store_div_add_scaled_inputs_rhs_indices(945, 943, 943, 1.0, 71, 1.0);s.store_div_from_scalar_offset_square(941, 1.0, 947, 1e-50);s.store_scaled_mul(938, 246, 941, (-p.p204));}
        s.b[960] = (s.v[938] < (-34.0));s.store_scalar(960, if s.b[960] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && s.b[957]) && s.b[960]) {s.store_scalar(255, 0.0);}
        if (((!s.b[956]) && s.b[957]) && (!s.b[960])) {s.store_mul_scale_offset_mixed_ia(940, 937, A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19, 0.0);s.store_powf_ad(943, A::div_scaled_inputs2(s.ad_value(954), 1.0, s.ad_value(950), 1e-12, s.ad_value(955), 1.0), p.p257);s.store_mul_ad_product_lhs_mixed_ai(946, A::mul3(A::exp(s.ad_value(938)), s.ad_value(940), s.ad_value(943)), 947, 947);s.store_mul3_lhs(255, 944, 945, 946);}
        if ((!s.b[956]) && (!s.b[957])) {s.store_scalar(255, 0.0);}
        if (!s.b[956]) {s.store_offset_scaled(937, 52, (-p.p211), p.p212);s.store_exp_scaled_input(939, 937, s.v[949]);s.store_scale(938, 52, p.p260);s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));s.store_mul_square_lhs(940, 938, 937);s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));s.store_mul3_lhs(250, 941, 939, 940);}
        s.b[961] = (s.v[938] >= 0.0);s.store_scalar(961, if s.b[961] { 1.0 } else { 0.0 });
        if ((!s.b[956]) && s.b[961]) {s.store_scale(250, 250, (-1.0));}
        if (!s.b[956]) {s.store_sub(942, 52, 51);s.store_offset_scaled(937, 942, (-p.p211), p.p212);s.store_exp_scaled_input(939, 937, s.v[949]);s.store_scale(938, 942, p.p260);s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));s.store_mul_square_lhs(940, 938, 937);s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[956]) {s.store_mul3_lhs(251, 941, 939, 940);}
        s.b[962] = (s.v[938] >= 0.0);s.store_scalar(962, if s.b[962] { 1.0 } else { 0.0 });
        if ((!s.b[956]) && s.b[962]) {s.store_scale(251, 251, (-1.0));}
        if (!s.b[956]) {s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p261, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p215, 1.0 / (s.v[949]));s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[963] = (s.v[947] < 0.0);s.store_scalar(963, if s.b[963] { 1.0 } else { 0.0 });
        if ((!s.b[956]) && s.b[963]) {s.store_scalar(947, 0.0);s.store_scalar(942, 0.0);}
        if (!s.b[956]) {s.store_offset(947, 947, 1e-50);s.store_div_from_scalar_powf_ad(938, (-p.p214), s.ad_value(947), p.p263);}
        s.b[964] = (s.v[938] < (-34.0));s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });
        if ((!s.b[956]) && s.b[964]) {s.store_scalar(254, 0.0);}
        if ((!s.b[956]) && (!s.b[964])) {s.store_exp(939, 938);s.store_scalar(940, (s.v[375] + p.p264));s.store_sub_scaled_inputs_mixed_ai(638, A::offset(s.ad_value(940), (-p.p265)), 1.0, 940, 0.001);s.store_scale(639, 940, (0.001 * (4.0 * p.p265)));}
        if ((!s.b[956]) && (!s.b[964])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((!s.b[956]) && (!s.b[964])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(940, 638, 0.5, 639, 0.5, p.p265);s.store_scale(940, 940, ((p.p213 * 1e-6) * s.v[952]));s.store_mul_ad_product_lhs_mixed_ia(252, 940, A::powf(s.ad_value(947), p.p262), 939);s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p269, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p268, 1.0 / (s.v[949]));s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[965] = (s.v[947] < 0.0);s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && (!s.b[964])) && s.b[965]) {s.store_scalar(947, 0.0);s.store_scalar(942, 0.0);}
        if ((!s.b[956]) && (!s.b[964])) {s.store_offset(947, 947, 1e-50);s.store_div_from_scalar_powf_ad(938, (-p.p267), s.ad_value(947), p.p271);}
        s.b[966] = (s.v[938] < (-34.0));s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && (!s.b[964])) && s.b[966]) {s.store_scalar(253, 0.0);}
        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {s.store_exp(939, 938);s.store_scalar(940, (s.v[375] + p.p272));s.store_sub_scaled_inputs_mixed_ai(638, A::offset(s.ad_value(940), (-p.p273)), 1.0, 940, 0.001);s.store_scale(639, 940, (0.001 * (4.0 * p.p273)));}
        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(940, 638, 0.5, 639, 0.5, p.p273);s.store_scale(940, 940, ((p.p266 * 1e-6) * s.v[952]));s.store_mul_ad_product_lhs_mixed_ia(253, 940, A::powf(s.ad_value(947), p.p270), 939);}
        if ((!s.b[956]) && (!s.b[964])) {s.store_scale(938, 252, (-0.001));}
        s.b[967] = (s.v[938] < 1e-50);s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && (!s.b[964])) && s.b[967]) {s.store_scalar(938, 1e-50);}
        if ((!s.b[956]) && (!s.b[964])) {s.store_add_scaled_inputs3_indices(638, 252, -1.0, 253, 1.0, 938, -1.0);s.store_scaled_mul(639, 253, 938, (-4.0));}
        if ((!s.b[956]) && (!s.b[964])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((!s.b[956]) && (!s.b[964])) {s.store_sqrt_square_add(639, 638, 639);s.store_sub_mixed_ai(254, A::add_scaled_inputs(s.ad_value(638), 0.5, s.ad_value(639), 0.5), 253);s.store_neg(254, 254);}
        if (!s.b[956]) {s.store_scalar(256, 0.5);}
        s.b[968] = (p.p18 == 0.0);s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
        if s.b[968] {s.store_scalar(257, 0.0);}
        if (!s.b[968]) {s.store_add_scaled_inputs4_offset_indices(279, 51, p.p198, 52, (-1.0), 82, (-p.p200), 266, (-p.p200), (p.p199 * p.p198));s.store_scale(247, 279, 1.0 / (p.p228));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[968]) {s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(248, 247, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[969] = (s.v[248] < 0.0);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
        if ((!s.b[968]) && s.b[969]) {s.store_scalar(248, 0.0);s.store_scalar(283, 0.0);}
        if (!s.b[968]) {s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(248), 1e-50, 1.0);}
        s.b[970] = (s.v[278] < (-34.0));s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
        if ((!s.b[968]) && s.b[970]) {s.store_scalar(257, 0.0);}
        if ((!s.b[968]) && (!s.b[970])) {s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));s.store_mul_product3_mixed_aiii(257, A::exp(s.ad_value(278)), 280, 248, 248, 1.0);s.store_div_scaled_value_offset_denominator(257, s.ad_value(257), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, s.ad_value(51))), 1.0, 1.0);s.store_div_mixed_ia(257, 257, A::sub_from_scalar(1.0, A::exp_div_scaled_inputs(s.ad_value(123), -1.0, s.ad_value(629), 1.0)));}
        s.b[971] = (p.p18 == 0.0);s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
        if s.b[971] {s.store_scalar(258, 0.0);}
        if (!s.b[971]) {s.store_add_scaled_inputs3_mixed_aii(279, A::add_scaled_inputs3_offset(s.ad_value(51), (-p.p198), s.ad_value(52), -1.0, s.ad_value(51), 1.0, ((p.p199) * (p.p198))), 1.0, 82, (-p.p200), 266, (-p.p200));s.store_scale(247, 279, 1.0 / (p.p228));s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(249, 247, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[972] = (s.v[249] < 0.0);s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });
        if ((!s.b[971]) && s.b[972]) {s.store_scalar(249, 0.0);s.store_scalar(283, 0.0);}
        if (!s.b[971]) {s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(249), 1e-50, 1.0);}
        s.b[973] = (s.v[278] < (-34.0));s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });
        if ((!s.b[971]) && s.b[973]) {s.store_scalar(258, 0.0);}
        if ((!s.b[971]) && (!s.b[973])) {s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));s.store_mul_product3_mixed_aiii(258, A::exp(s.ad_value(278)), 280, 249, 249, 1.0);s.store_div_scaled_value_offset_denominator(258, s.ad_value(258), 1.0, A::exp(A::mul(s.ad_value(120), s.ad_value(51))), 1.0, 1.0);s.store_div_mixed_ia(258, 258, A::sub_from_scalar(1.0, A::exp_div_scaled_inputs(s.ad_value(123), -1.0, s.ad_value(629), 1.0)));}
        s.store_scalar(264, p.p176);s.store_scalar(261, 0.0);s.b[974] = (s.v[34] != 0.0);s.store_scalar(974, if s.b[974] { 1.0 } else { 0.0 });
        if s.b[974] {s.store_add(280, 51, 56);s.store_add_scaled_inputs(260, 280, s.v[264], 57, (1.0 - s.v[264]));}
        s.b[975] = (s.v[260] > ((s.v[56] + s.v[51]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(975, if s.b[975] { 1.0 } else { 0.0 });
        if (s.b[974] && s.b[975]) {s.store_offset_add(260, 56, 51, (-(10.0 * 2.220446049250313e-16)));}
        s.b[976] = (p.p45 != 0.0);s.store_scalar(976, if s.b[976] { 1.0 } else { 0.0 });s.b[977] = (s.v[151] > 1e-15);s.store_scalar(977, if s.b[977] { 1.0 } else { 0.0 });
        if (((!s.b[974]) && s.b[976]) && s.b[977]) {s.store_div_scaled_product_by_product_indices(261, 151, 122, 1.0, 123, 149, 1.0);}
        s.store_scalar(435, s.v[273]);s.store_scalar(436, (1.0 / s.v[435]));s.b[978] = (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (s.v[624] > 0.0));s.store_scalar(978, if s.b[978] { 1.0 } else { 0.0 });
        if s.b[978] {s.store_scalar(195, p.p175);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[978] {s.store_mul_sqrt_mixed_ia(437, 141, A::div_from_scalar(s.v[624], s.ad_value(457)));}
        let (t1f,) = {
    if s.b[978] {
        let t1d: f64 = (1.0 - -1.0);let t1e: f64 = (t1d / 2.0);
        (t1e,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t1f);
        let (t22,) = {
    if s.b[978] {
        let t20: f64 = (1.0 + -1.0);let t21: f64 = (t20 / 2.0);
        (t21,)
    } else {
        (s.v[400],)
    }
};
        s.store_scalar(400, t22);
        let (t26,) = {
    if s.b[978] {
        let t23: f64 = (s.v[399] * s.v[412]);let t24: f64 = (s.v[400] * s.v[413]);let t25: f64 = (t23 + t24);
        (t25,)
    } else {
        (s.v[402],)
    }
};
        s.store_scalar(402, t26);
        let (t2a,) = {
    if s.b[978] {
        let t27: f64 = (s.v[399] * s.v[413]);let t28: f64 = (s.v[400] * s.v[412]);let t29: f64 = (t27 + t28);
        (t29,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, t2a);
        if (s.b[978] && (s.v[399] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if (s.b[978] && (s.v[400] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if s.b[978] {s.store_scalar(415, 0.0);s.store_neg(278, 415);}
        s.b[979] = (s.v[278] > s.v[31]);s.store_scalar(979, if s.b[979] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[979]) {s.store_sub(279, 278, 31);s.store_sub_from_scalar(280, s.v[30], 31);s.store_div(638, 279, 280);s.store_square(639, 638);s.store_mul(640, 639, 638);s.store_square(641, 639);s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);s.store_mul_scale_offset_indices(291, 280, 291, -1.0, 1.0);s.store_neg(387, 387);s.store_add(288, 31, 291);}
        if (s.b[978] && (!s.b[979])) {s.copy_ad(288, 278);}
        if s.b[978] {s.store_offset_scaled(416, 288, -1.0, (-1e-12));s.store_scale(144, 437, s.v[436]);s.store_square(145, 144);s.store_sub_from_scalar(404, p.p39, 414);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(417, 2.0, 120, A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));}
        let (t2c,) = {
    if s.b[978] {
        let t2b: f64 = (-s.v[416]);
        (t2b,)
    } else {
        (s.v[419],)
    }
};
        s.store_scalar(419, t2c);s.b[980] = (s.v[404] < s.v[419]);s.store_scalar(980, if s.b[980] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[980]) {s.store_div_scalar_by_product_indices(291, s.v[435], 120, 437, 1.0);s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);s.store_sub(176, 137, 417);s.store_mul_add_rhs(290, 120, 404, 416);s.store_sub_from_scalar_scaled_mul_mixed_ia(183, (7.0 * 1.414213562373095), 291, A::offset(s.ad_value(290), (-2.0)), 9.0);s.store_square(181, 183);}
        s.b[981] = (s.v[182] < (s.v[181] * 1e-8));s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });
        if ((s.b[978] && s.b[980]) && s.b[981]) {s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);}
        if ((s.b[978] && s.b[980]) && (!s.b[981])) {s.store_sqrt_add(180, 182, 181);s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);}
        if (s.b[978] && s.b[980]) {s.store_powf(178, 179, 0.3333333333333333);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
    ) {
        if (s.b[978] && s.b[980]) {s.store_add_scaled_inputs_product_mixed_aiii(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);s.store_div(77, 177, 178);s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);s.store_add(279, 259, 416);s.store_div(280, 279, 176);s.store_sub_div_lhs_mixed_ia(410, 279, A::sqrt_square_offset(s.ad_value(280), 1.0), 416);s.store_scaled_sub(408, 404, 410, s.v[435]);s.copy_ad(407, 408);}
        if (s.b[978] && (!s.b[980])) {s.store_scalar(77, 3.0);s.store_sub_div_lhs_indices(319, 77, 120, 416);s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[982] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(982, if s.b[982] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[982]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[980])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[983] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[983]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[980])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[984] = (s.v[77] < 3.0);s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[984]) {s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(425, 404, -1.0, 416, -1.0, 144, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), 425, 1.0, 421, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), 1.0, 421, 421, 9.0);s.store_sqrt_add_scaled_square_cube_product(283, 426, 1.0, 424, 1.0);s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[980])) && s.b[984]) {s.store_neg_powf_add_input(428, 426, 283, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(290, 427, 1.0, 428, 1.0, 422, 1.0, 421, 3.0, -1.0);s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[985] = (p.p30 > 0.0);s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[985]) {s.store_offset_add(420, 404, 416, 0.1);s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);s.store_mul(430, 429, 203);s.store_mul(278, 121, 145);s.store_mul(434, 120, 420);s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);s.store_offset_sub(638, 434, 433, (-1.0));s.store_scale(639, 434, 4.0);}
        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((s.b[978] && (!s.b[980])) && s.b[985]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(433, 434, 1.0, 638, (-0.5), 639, (-0.5));s.store_sub(434, 434, 433);s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);s.store_sub_div_lhs_indices(320, 432, 120, 416);s.copy_ad(431, 77);s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));}
        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((s.b[978] && (!s.b[980])) && s.b[985]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(77, 432, 1.0, 638, (-0.5), 639, (-0.5));}
        if (s.b[978] && (!s.b[980])) {s.store_sub_div_lhs_indices(410, 77, 120, 416);s.store_add_offset_lhs_mixed_ia(279, 77, (-1.0), A::exp_scaled_input(s.ad_value(77), -1.0));}
        s.b[986] = (s.v[279] < (10.0 * 2.220446049250313e-16));s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[986]) {s.store_scalar(279, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[980])) {s.store_mul_sqrt_rhs(407, 437, 279);s.store_scaled_sub(408, 404, 410, s.v[435]);}
        s.b[987] = (p.p30 == 1.0);s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);s.store_mul(204, 429, 203);}
        let (t2d,) = {
    if ((s.b[978] && (!s.b[980])) && s.b[987]) {
        (0.0,)
    } else {
        (s.v[379],)
    }
};
        s.store_scalar(379, t2d);
        if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_scalar(62, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
    ) {
        let mut t31: usize = 0;
        while {
            let t2f: f64 = (40.0 + 1.0);let t30: f64 = if (((s.b[978] && (!s.b[980])) && s.b[987]) && (s.v[62] <= t2f)) { 1.0 } else { 0.0 };
            t30 != 0.0
        } {
            t31 += 1;assert!(t31 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_mul_add_rhs(77, 120, 410, 416);}
            s.b[988] = (s.v[77] < 5.0);s.store_scalar(988, if s.b[988] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[988]) {s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(207, 204, 205, 205);s.store_mul_product3_indices(208, 206, 204, 120, 205, 2.0);s.store_mul_scale_offset_mixed_ia(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(148, 77, A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, 208, 1.0, 209, 2.0);}
            s.b[989] = (s.v[77] < 80.0);s.store_scalar(989, if s.b[989] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) && s.b[989]) {s.store_exp(147, 77);s.store_mul_scale_offset_indices(207, 204, 147, 1.0, (-1.0));s.store_mul3_lhs(208, 204, 120, 147);}
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) && (!s.b[989])) {s.store_exp_mul(202, 120, 410);s.store_mul_sub_rhs(207, 429, 202, 203);s.store_mul3_lhs(208, 429, 120, 202);}
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) {s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);}
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));s.store_sub_from_scalar_scaled_mul(212, (-1.0), 144, 210, 1.0);}
            s.b[990] = (s.v[379] == 1.0);s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[990]) {s.store_scalar(62, (40.0 + 1.0));}
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {s.store_div_scaled_inputs_indices(213, 211, -1.0, 212, 1.0);}
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[991] = (((s.v[213]) as f64).abs() > s.v[214]);s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[991]) {s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {s.store_add(410, 410, 213);}
            s.b[992] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
            let (t2e,) = {
    if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[992]) {
        (1.0,)
    } else {
        (s.v[379],)
    }
};
            s.store_scalar(379, t2e);
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[994] = (s.v[77] < 5.0);s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[994]) {s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));}
        if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[994])) {s.store_offset(64, 77, (-1.0));s.store_sqrt(65, 64);}
        if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_mul(407, 437, 65);}
    }
}
