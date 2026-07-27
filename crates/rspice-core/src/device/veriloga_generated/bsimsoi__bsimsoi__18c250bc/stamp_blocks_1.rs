#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_div_mixed_ia(391, 380, A::exp_scaled_input(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) })));s.b[612] = (s.v[44] < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if s.b[612] {s.store_scalar(44, 0.0);}
        s.store_scalar(467, ((s.v[474]) as f64).powf(p[239]));s.store_primal_offset(489, 44, s.v[475]);s.store_powf(468, 489, p[240]);s.store_add_ad(463, A::offset(A::div_from_scalar(p[244], s.ad_value(468)), (p[243] / s.v[467])), A::div_from_scalar(p[245], A::scale(s.ad_value(468), s.v[467])));s.store_offset(231, 463, 1.0);s.store_scalar(467, ((s.v[474]) as f64).powf(p[241]));s.store_powf(468, 489, p[242]);s.store_add_ad(463, A::offset(A::div_from_scalar(p[247], s.ad_value(468)), (p[246] / s.v[467])), A::div_from_scalar(p[248], A::scale(s.ad_value(468), s.v[467])));s.store_offset(232, 463, 1.0);s.store_sqrt_square_offset(232, 232, 1e-9);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_scaled(233, 231, (1.0 + (p[238] * s.v[430])), 1e-9);s.store_scalar(483, (1.0 / (p[232] + (0.5 * s.v[474]))));s.store_scalar(484, (1.0 / (p[233] + (0.5 * s.v[474]))));s.store_scalar(235, (s.v[483] + s.v[484]));s.store_scale_ad(234, A::div_from_scalar(p[235], s.ad_value(233)), s.v[235]);s.b[613] = (((p[4] > 0.0) && (p[5] > 0.0)) && ((p[3] == 1.0) || ((p[3] > 1.0) && (p[6] > 0.0))));s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if s.b[613] {s.store_scalar(485, 0.0);s.store_scalar(486, 0.0);}
        s.b[614] = (s.v[45] < (-1.0));s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if (s.b[613] && s.b[614]) {s.store_scalar(45, (-1.0));}
        s.b[615] = (s.v[45] > 1.0);s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[613] && (!s.b[614])) && s.b[615]) {s.store_scalar(45, 1.0);}
        if ((s.b[613] && (!s.b[614])) && (!s.b[615])) {
        }
        if s.b[613] {s.store_scalar(495, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (s.b[613] && (s.v[495] < p[3])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if s.b[613] {s.store_primal_div_from_scalar_offset_scaled_input(616, (1.0 / p[3]), 495, (p[6] + s.v[474]), (p[4] + (0.5 * s.v[474])));s.store_primal_div_from_scalar_offset_scaled_input(617, (1.0 / p[3]), 495, (p[6] + s.v[474]), (p[5] + (0.5 * s.v[474])));s.store_primal_add(485, 485, 616);s.store_primal_add(486, 486, 617);s.store_primal_offset(495, 495, 1.0);}
        }
        if s.b[613] {s.store_primal_add(490, 485, 486);s.copy_ad(51, 490);s.store_mul_div_from_scalar_lhs_ad_indices(487, p[235], 233, 490);s.store_div_scaled_offset_numerator_mixed_ia(467, 487, 1.0, 1.0, A::offset(s.ad_value(234), 1.0), 1.0);s.store_mul(404, 337, 467);s.store_div_scaled_offset_numerator(468, A::mul(s.ad_value(45), s.ad_value(487)), 1.0, 1.0, A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0), 1.0);s.store_mul(407, 338, 468);s.store_primal_offset(491, 490, (-s.v[235]));s.store_mul_div_from_scalar_lhs_ad_indices(488, p[237], 232, 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(492, p[249], A::powf(s.ad_value(232), p[250]), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(493, p[251], A::powf(s.ad_value(232), p[252]), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(494, p[253], A::powf(s.ad_value(232), p[254]), 491);s.store_add(408, 137, 488);s.store_add(402, 124, 492);s.store_add(400, 187, 493);s.store_add(401, 189, 494);}
        if (!s.b[613]) {s.copy_ad(404, 337);s.copy_ad(408, 137);s.copy_ad(407, 338);s.copy_ad(402, 124);s.copy_ad(400, 187);s.copy_ad(401, 189);s.store_scalar(51, 0.0);s.store_scalar(235, 0.0);s.store_scalar(45, 0.0);}
        s.store_scale(403, 402, (p[66] * 1.0 / (p[67])));s.store_offset(408, 408, p[20]);s.store_offset(406, 152, (p[37] * p[20]));s.store_scalar(52, (s.v[392] * p[8]));s.store_scale(53, 43, p[8]);s.store_scalar(54, (s.v[392] * p[7]));s.store_scale(55, 43, p[7]);s.b[618] = (s.v[43] > 0.0);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });s.b[619] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if (s.b[618] && s.b[619]) {s.store_sub(467, 323, 322);}
        let (t4,) = {
    if (s.b[618] && s.b[619]) {
        let t2: f64 = (p[356] * s.v[467]);let t3: f64 = (s.v[322] + t2);
        (t3,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t4);
        if (s.b[618] && s.b[619]) {s.store_sub_from_scalar(468, s.v[52], 53);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p[356]));s.store_scale(177, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_products_indices(56, 467, 468, ((1.0 + p[356]) * 0.3333333333333333), 53, 322, (-1.0));s.store_sub_from_scalar(468, s.v[54], 55);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p[356]));s.store_scale(179, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_products_indices(57, 467, 468, ((1.0 + p[356]) * 0.3333333333333333), 55, 322, (-1.0));}
        if (s.b[618] && (!s.b[619])) {s.store_sub(467, 322, 323);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t7,) = {
    if (s.b[618] && (!s.b[619])) {
        let t5: f64 = (p[356] * s.v[467]);let t6: f64 = (s.v[323] + t5);
        (t6,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t7);
        if (s.b[618] && (!s.b[619])) {s.store_offset(468, 53, (-s.v[52]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p[356]));s.store_scale(177, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_product_indices(56, 323, (-s.v[52]), 467, 468, ((1.0 + p[356]) * 0.3333333333333333));s.store_offset(468, 55, (-s.v[54]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p[356]));s.store_scale(179, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_product_indices(57, 323, (-s.v[54]), 467, 468, ((1.0 + p[356]) * 0.3333333333333333));}
        let (t8,) = {
    if (!s.b[618]) {
        (0.0,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t8);
        if (!s.b[618]) {s.store_scalar(176, 0.0);s.store_scalar(177, 0.0);s.store_scalar(56, 0.0);s.store_scalar(178, 0.0);s.store_scalar(179, 0.0);s.store_scalar(57, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[620] = ((s.v[46] < 1.0) || (s.v[46] > 2.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if s.b[620] {s.store_scalar(46, 1.0);}
        s.store_scale_ad(467, {
            if ((s.v[46] * (1.0 + (p[155] / p[154]))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(46), (1.0 + (p[155] / p[154])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p[357]);s.store_scalar(468, (p[10] - p[2]));s.b[621] = (s.v[468] > 0.0);s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if s.b[621] {s.store_scale(58, 467, s.v[468]);}
        if (!s.b[621]) {s.store_scalar(58, 0.0);}
        s.store_scalar(468, (p[9] - p[2]));s.b[622] = (s.v[468] > 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if s.b[622] {s.store_scale(59, 467, s.v[468]);}
        if (!s.b[622]) {s.store_scalar(59, 0.0);}
        s.store_scalar(61, (p[131] * p[11]));s.b[623] = ((p[429] == 1.0) && (s.v[61] < p[431]));s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if s.b[623] {s.store_scalar(61, p[431]);}
        s.store_scalar(60, (p[131] * p[12]));s.b[624] = ((p[429] == 1.0) && (s.v[60] < p[431]));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if s.b[624] {s.store_scalar(60, p[431]);}
        s.b[625] = (s.v[36] < 1e-15);s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if s.b[625] {s.store_scalar(36, 1e-15);}
        s.store_div_scalar_by_product_indices(467, (((-0.5) * s.v[327]) * s.v[327]), 36, 36, 1.0);s.b[626] = (s.v[467] > 100.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if s.b[626] {s.store_scaled_offset(468, 467, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[627] = (s.v[467] < (-100.0));s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if ((!s.b[626]) && s.b[627]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[626]) && (!s.b[627])) {s.store_exp(468, 467);}
        s.copy_ad(351, 468);s.store_mul_scale_offset_mixed_ia(467, 319, A::div_from_scalar(1.0, s.ad_value(36)), 1.0, (1.0 / s.v[327]));s.store_pow_indices(352, 467, 318);s.store_offset_scaled_ad(353, A::pow(s.ad_value(467), s.ad_value(253)), p[343], 1.0);s.store_add_scaled_inputs(354, 320, 1.0, 321, s.v[327]);s.b[628] = (s.v[354] < 1.0);s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if s.b[628] {s.store_scalar(354, 1.0);}
        s.b[629] = (p[41] == 0.0);s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if s.b[629] {s.store_scalar(62, (p[66] - p[68]));}
        if (!s.b[629]) {s.store_scalar(498, (8.617087e-5 * p[57]));s.copy_ad(499, 498);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(500, 498, {
                if ((1e20 * s.v[108]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(108), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, 530, 2.0);
        }
        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(501, 498, {
                if (s.v[108] > 1e-38) {
                    A::ln(s.ad_value(108))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 530, 2.0);
        }
        if (!s.b[629]) {s.store_sqrt(502, 501);s.store_add(464, 406, 501);s.store_scalar(503, (p[37] * p[56]));s.store_scalar(467, (p[60] * 8.85418e-12));}
        s.b[630] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0));s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[630]) {s.store_div_scaled_product_mixed_iia(468, 417, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(471, A::div_scaled_inputs2(s.ad_value(503), 2.0, s.ad_value(467), (-2.0), s.ad_value(468), 1.0), 1.0);s.store_mul_scale_offset_indices(469, 468, 471, 1.0, (-1.0));s.store_div_scaled_product_indices(470, 469, 469, 0.5, 468, 1.0);s.store_offset_sub_from_scalar_ad(532, p[1034], s.ad_value(470), (-0.05));s.store_sqrt_square_offset(473, 532, 0.224);s.store_offset_add_scaled_inputs_indices(472, 532, (-0.5), 473, (-0.5), p[1034]);s.store_sub(504, 503, 472);}
        if ((!s.b[629]) && (!s.b[630])) {s.copy_ad(504, 503);}
        if (!s.b[629]) {s.store_sub(506, 500, 501);s.copy_ad(470, 341);s.store_mul(509, 397, 470);s.store_mul(510, 397, 470);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * p[54]), 509, 1.0);}
        s.b[631] = (s.v[467] > (-100.0));s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[631]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[631])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_div_scaled_product_indices(469, 100, 417, 1.0, 340, 1.0);s.copy_ad(470, 96);s.store_div_scaled_inputs2_mixed_aii(471, A::add_scaled_product(s.ad_value(469), 1.0, s.ad_value(470), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[632] = (s.v[471] >= (-0.5));s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[632]) {s.store_offset(511, 471, 1.0);}
        if ((!s.b[629]) && (!s.b[632])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 471, 8.0, 3.0);s.store_mul_scale_offset_rhs(511, 467, 471, 3.0, 1.0);}
        s.b[633] = (s.v[378] > 0.0);s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[633]) {s.store_offset_scaled(470, 378, 2.0, p[54]);}
        if ((!s.b[629]) && s.b[633]) {
            s.store_mul_mixed_ia(471, 499, {
                            if ((p[54] / s.v[470]) > 1e-38) {
                                A::ln(A::div_from_scalar(p[54], s.ad_value(470)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[629]) && s.b[633]) {s.store_mul(519, 511, 471);}
        if ((!s.b[629]) && (!s.b[633])) {s.store_scalar(519, 0.0);}
        if (!s.b[629]) {s.store_mul(63, 129, 522);s.store_mul(523, 63, 506);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (p[55] * p[54])), 510, 1.0);}
        s.b[634] = (s.v[467] > (-100.0));s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[634]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[634])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_mul(467, 132, 469);s.store_mul(524, 467, 506);s.store_scalar(430, ((p[57] / s.v[429]) - 1.0));s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (p[54]), 1.0);s.store_add_scaled_inputs(468, 121, 1.0, 122, 1.0 / (p[54]));s.store_add_scaled_product_mixed_aii(520, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(502)), 1.0, 468, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(464, 415, 501, 1.0, 127, p[55], 1.0);s.store_scalar(517, 0.0);s.store_scalar(521, 0.0);s.store_sqrt_offset_scaled_input(518, 377, 1.0 / (p[54]), 1.0);s.copy_ad(514, 502);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[629]) {s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(507, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(408), p[37], A::add_scaled_products(s.ad_value(376), s.ad_value(514), 1.0, s.ad_value(346), s.ad_value(502), (-1.0)), s.ad_value(518), 1.0), 1.0, s.ad_value(523), (-1.0), s.ad_value(524), -1.0), 1.0, s.ad_value(125), s.ad_value(464), 1.0), 1.0, 520, 1.0, 517, -1.0, 519, -1.0, 521);s.store_sub(508, 504, 507);s.store_mul(497, 511, 499);s.store_div_scaled_product_indices(512, 384, 508, 1.0, 497, 1.0);s.store_div_scaled_inputs2_mixed_iai(513, 151, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(384), s.ad_value(508)), (-1.0), 497, 1.0);}
        s.b[635] = (s.v[512] > 100.0);s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[635]) {s.copy_ad(505, 508);}
        s.b[636] = (s.v[513] > 100.0);s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[629]) && (!s.b[635])) && s.b[636]) {s.store_div_scaled_inputs2_by_product_indices(467, 508, 1.0, 151, (-1.0), 511, 499, 1.0);s.store_exp(515, 467);s.store_mul_div_scaled_product_indices(505, 515, 499, 367, 1.0, 396, 1.0);}
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {s.store_exp(515, 512);}
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_mul_mixed_ia(468, 497, {
                            if ((1.0 + s.v[515]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(515), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {s.store_mul3_ad(471, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(498), s.ad_value(367)), 1.0), A::exp(s.ad_value(513)), A::sub_from_scalar(1.0, s.ad_value(384)));s.store_sub_mixed_ia(469, 384, A::div_scaled_product(s.ad_value(497), s.ad_value(471), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));s.store_div(505, 468, 469);}
        if (!s.b[629]) {s.store_add_scaled_inputs3_indices(470, 408, p[37], 406, (-1.0), 501, -1.0);s.store_scale(516, 470, 4.0);}
        s.b[637] = (s.v[516] < 0.0);s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[637]) {s.store_scalar(516, 0.0);}
        let (t9,) = {
    if (!s.b[629]) {
        (0.0,)
    } else {
        (s.v[525],)
    }
};
        s.store_scalar(525, t9);
        if (!s.b[629]) {s.copy_ad(526, 415);}
        let (ta,) = {
    if (!s.b[629]) {
        (1000000.0,)
    } else {
        (s.v[527],)
    }
};
        s.store_scalar(527, ta);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t31: usize = 0;
        while {
            let te: f64 = (s.v[526] - s.v[527]);let tf: f64 = (te).abs();let t22: f64 = if te >= 0.0 { s.dn[526][0] } else { (-s.dn[526][0]) };let t23: f64 = if te >= 0.0 { s.dn[526][1] } else { (-s.dn[526][1]) };let t28: f64 = if te >= 0.0 { s.dn[526][2] } else { (-s.dn[526][2]) };let t29: f64 = if te >= 0.0 { s.dn[526][3] } else { (-s.dn[526][3]) };let t2a: f64 = if te >= 0.0 { s.dn[526][4] } else { (-s.dn[526][4]) };let t2b: f64 = if te >= 0.0 { s.dn[526][5] } else { (-s.dn[526][5]) };let t2c: f64 = if te >= 0.0 { s.dn[526][6] } else { (-s.dn[526][6]) };let t2d: f64 = if te >= 0.0 { s.dn[526][7] } else { (-s.dn[526][7]) };let t2e: f64 = if te >= 0.0 { s.dn[526][8] } else { (-s.dn[526][8]) };let t2f: f64 = if te >= 0.0 { s.dn[526][9] } else { (-s.dn[526][9]) };let t24: f64 = if te >= 0.0 { s.dn[526][10] } else { (-s.dn[526][10]) };let t25: f64 = if te >= 0.0 { s.dn[526][11] } else { (-s.dn[526][11]) };let t26: f64 = if te >= 0.0 { s.dn[526][12] } else { (-s.dn[526][12]) };let t27: f64 = if te >= 0.0 { s.dn[526][13] } else { (-s.dn[526][13]) };let t10: f64 = if te >= 0.0 { s.db[526][0] } else { (-s.db[526][0]) };let t11: f64 = if te >= 0.0 { s.db[526][1] } else { (-s.db[526][1]) };let t1a: f64 = if te >= 0.0 { s.db[526][2] } else { (-s.db[526][2]) };let t1b: f64 = if te >= 0.0 { s.db[526][3] } else { (-s.db[526][3]) };let t1c: f64 = if te >= 0.0 { s.db[526][4] } else { (-s.db[526][4]) };let t1d: f64 = if te >= 0.0 { s.db[526][5] } else { (-s.db[526][5]) };let t1e: f64 = if te >= 0.0 { s.db[526][6] } else { (-s.db[526][6]) };let t1f: f64 = if te >= 0.0 { s.db[526][7] } else { (-s.db[526][7]) };let t20: f64 = if te >= 0.0 { s.db[526][8] } else { (-s.db[526][8]) };let t21: f64 = if te >= 0.0 { s.db[526][9] } else { (-s.db[526][9]) };let t12: f64 = if te >= 0.0 { s.db[526][10] } else { (-s.db[526][10]) };let t13: f64 = if te >= 0.0 { s.db[526][11] } else { (-s.db[526][11]) };let t14: f64 = if te >= 0.0 { s.db[526][12] } else { (-s.db[526][12]) };let t15: f64 = if te >= 0.0 { s.db[526][13] } else { (-s.db[526][13]) };let t16: f64 = if te >= 0.0 { s.db[526][14] } else { (-s.db[526][14]) };let t17: f64 = if te >= 0.0 { s.db[526][15] } else { (-s.db[526][15]) };let t18: f64 = if te >= 0.0 { s.db[526][16] } else { (-s.db[526][16]) };let t19: f64 = if te >= 0.0 { s.db[526][17] } else { (-s.db[526][17]) };let t30: f64 = if ((!s.b[629]) && ((s.v[525] <= 4.0) && (tf > 1e-12))) { 1.0 } else { 0.0 };
            t30 != 0.0
        } {
            t31 += 1;
            if t31 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t31, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            let (tb,) = {
    if (!s.b[629]) {
        (s.v[526],)
    } else {
        (s.v[527],)
    }
};
            s.store_scalar(527, tb);
            if (!s.b[629]) {s.store_scale(464, 526, 200000000.0);s.store_div_scaled_inputs2_indices(638, 505, 1.0, 516, 1.0, 464, 1.0);}
            if (!s.b[629]) {
                s.store_offset_ad(639, A::exp_scaled_input({
                    if (s.v[638] > 1e-38) {
                        A::ln(s.ad_value(638))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (p[59] * 0.7)), 1.0);
            }
            if (!s.b[629]) {s.store_div_from_scalar(528, (p[58] * 1.9e-9), 639);s.store_add_scaled_product_indices(526, 415, 1.0, 416, 528, (-1.0 / (p[47])));}
            let (td,) = {
    if (!s.b[629]) {
        let tc: f64 = (s.v[525] + 1.0);
        (tc,)
    } else {
        (s.v[525],)
    }
};
            s.store_scalar(525, td);
        }
        if (!s.b[629]) {s.copy_ad(62, 526);}
        s.copy_ad(462, 341);s.store_sub(463, 115, 118);s.store_mul(464, 397, 462);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (s.v[328] * s.v[327])), 464, 1.0);s.b[640] = (s.v[467] > (-100.0));s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if s.b[640] {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[640]) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        s.store_mul(467, 132, 469);s.store_mul(469, 467, 463);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * s.v[327]), 464, 1.0);s.b[641] = (s.v[467] > (-100.0));s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if s.b[641] {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        if (!s.b[641]) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        s.store_mul3_lhs(470, 129, 470, 463);s.store_div_scaled_product_offset_denominator_indices(471, 62, 118, 1.0, 127, s.v[328], 1.0);s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (s.v[327]), 1.0);s.store_add_scaled_product_mixed_aai(472, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(339)), 1.0, A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[327])), 430, 1.0);s.store_add_mixed_ai(531, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p[37], s.ad_value(469), (-1.0), s.ad_value(470), -1.0), 1.0, s.ad_value(125), s.ad_value(471), 1.0), 472);s.store_add_scaled_inputs_product_indices(359, 531, 1.0, 118, (-1.0), 120, 339, (-1.0));s.store_mul_scale_offset_rhs(344, 108, 128, ((1.0 / (s.v[327])) * ((1.602176462e-19 * (1000000.0 * p[155])))), (1.602176462e-19 * (1000000.0 * p[155])));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(64, (((p[424] * (p[427] + (((s.v[328] / p[23]) / 3.0) / p[425]))) / ((p[425] * p[3]) * (p[1] - p[428]))) + (p[426] / ((p[1] * s.v[328]) * p[3]))));s.b[642] = (s.v[64] > 0.0);s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if s.b[642] {s.store_scalar(64, (1.0 / s.v[64]));}
        if (!s.b[642]) {s.store_scalar(64, 1000.0);}
        s.b[644] = (p[18] < 0.001);s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if ((p[40] != 0.0) && s.b[644]) {s.store_scalar(65, 1000.0);}
        if ((p[40] != 0.0) && (!s.b[644])) {s.store_scalar(65, (p[255] + (1.0 / p[18])));}
        s.b[645] = (p[19] < 0.001);s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if ((p[40] != 0.0) && s.b[645]) {s.store_scalar(66, 1000.0);}
        if ((p[40] != 0.0) && (!s.b[645])) {s.store_scalar(66, (p[255] + (1.0 / p[19])));}
        if (p[40] == 0.0) {s.store_scalar(65, 0.0);s.store_scalar(66, 0.0);}
        s.store_offset(67, 359, (p[37] * p[20]));s.store_scaled_sqrt_ad(360, A::div_scaled_product(s.ad_value(417), s.ad_value(480), 1.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 0.3333333333333333);s.store_add_scaled_inputs3_indices(468, 408, p[37], 406, (-1.0), 118, -1.0);s.store_scale(469, 468, 2.0);s.store_scale(470, 468, 2.5);
        if (p[37] == 1.0) {
            s.copy_ad(68, 469);
        } else {
            s.copy_ad(68, 470);
        }
        s.b[646] = (s.v[68] < 0.0);s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if s.b[646] {s.store_scalar(68, 0.0);}
        s.b[647] = (p[62] == 4.0);s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if s.b[647] {s.store_mul(509, 397, 341);s.store_div_scaled_inputs_indices(467, 130, s.v[327], 509, 1.0);}
        s.b[648] = (s.v[467] < 100.0);s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[648]) {s.store_exp(468, 467);s.store_offset(469, 468, (-1.0));s.store_square(470, 469);s.store_add_scaled_inputs(471, 470, 1.0, 468, (2.0 * 3.720075976e-44));s.store_div(522, 468, 471);}
        if (s.b[647] && (!s.b[648])) {s.store_scalar(522, (1.0 / (2.688117142e43 - 2.0)));}
        if s.b[647] {s.store_div(463, 417, 340);s.store_mul(464, 100, 463);s.store_div_scaled_inputs2_mixed_aii(531, A::add_scaled_product(s.ad_value(464), 1.0, s.ad_value(96), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[649] = (s.v[531] >= (-0.5));s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[649]) {s.store_offset(529, 531, 1.0);}
        if (s.b[647] && (!s.b[649])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 531, 8.0, 3.0);s.store_mul_scale_offset_rhs(529, 467, 531, 3.0, 1.0);}
        if s.b[647] {s.store_mul(467, 529, 480);s.copy_ad(468, 151);s.store_div(469, 468, 467);}
        s.b[650] = (s.v[469] < (-100.0));s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[650]) {s.store_div_scaled_inputs_indices(470, 396, 3.720075976e-44, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        s.b[651] = (s.v[469] > 100.0);s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if ((s.b[647] && (!s.b[650])) && s.b[651]) {s.store_div_scaled_inputs_indices(470, 396, 2.688117142e43, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        if ((s.b[647] && (!s.b[650])) && (!s.b[651])) {s.store_div_scaled_product_mixed_aii(470, A::exp(s.ad_value(469)), 396, 1.0, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        if s.b[647] {s.store_div_scaled_inputs_indices(69, 467, 0.6931471805599453, 471, 1.0);}
        if (!s.b[647]) {s.store_scalar(69, 0.0);}
        s.b[704] = ((p[38] >= 4.4) || (p[63] != 0.0));s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });s.b[705] = (s.v[106] < 0.01);s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });
        if (s.b[704] && s.b[705]) {s.store_scalar(106, 0.01);}
        s.b[706] = (s.v[106] > 1.0);s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });
        if ((s.b[704] && (!s.b[705])) && s.b[706]) {s.store_scalar(106, 1.0);s.store_scalar(105, 0.0);}
        s.b[707] = (s.v[181] < 0.0);s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });
        if s.b[707] {s.store_scalar(181, 0.0);s.store_scalar(182, 0.0);}
        s.b[708] = ((s.v[182] < 0.001) && (s.v[182] != 0.0));s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });
        if ((!s.b[707]) && s.b[708]) {s.store_scalar(182, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[738] = (s.v[308] < 0.0);s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[738]) {s.store_scalar(308, 0.0);}
        s.b[739] = (s.v[309] < 0.0);s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[739]) {s.store_scalar(309, 0.0);}
        s.b[740] = (s.v[310] < 0.0);s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[740]) {s.store_scalar(310, 0.0);}
        s.b[741] = (s.v[311] < 0.0);s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[741]) {s.store_scalar(311, 0.0);}
        s.b[742] = (s.v[312] < 0.0);s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[742]) {s.store_scalar(312, 0.0);}
        s.b[743] = (s.v[313] < 0.0);s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[743]) {s.store_scalar(313, 0.0);}
        s.store_scalar(410, 0.0);s.b[805] = ((p[36] == 1.0) && (p[14] != 0.0));s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });s.b[806] = ((p[35] != 0.0) && (!true));s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });s.b[807] = true;s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[805] && s.b[806]) && s.b[807]) {s.store_voltage(410, ctx, nodes, Some(5), None);}
        s.b[808] = true;s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && s.b[808]) {s.store_voltage(410, ctx, nodes, Some(4), None);}
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && (!s.b[808])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        if (s.b[805] && (!s.b[806])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        s.store_offset(409, 410, s.v[409]);s.store_scale(411, 409, 1.0 / (s.v[429]));s.store_offset(430, 411, (-1.0));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        s.store_scalar(1133, 0.0);s.store_scalar(1134, 0.0);s.store_scalar(1135, 0.0);s.store_scalar(1136, 0.0);s.store_scalar(1131, 0.0);s.store_scalar(1121, 0.0);s.store_scalar(855, 0.0);s.store_scalar(1122, 0.0);s.store_scalar(1130, 0.0);s.store_scalar(1127, 0.0);s.store_scalar(1128, 0.0);s.store_scalar(1126, 0.0);s.store_scalar(1118, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.copy_ad(955, 182);s.copy_ad(1095, 173);s.copy_ad(1096, 174);s.copy_ad(1097, 171);s.copy_ad(1098, 172);s.b[1159] = ((p[36] == 1.0) && (p[14] != 0.0));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });s.b[1160] = (p[41] == 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1160]) {s.store_scale(832, 409, 8.617087e-5);s.store_offset(843, 409, 1108.0);s.store_square(848, 409);s.store_sub_from_scalar_ad(912, 1.16, A::div_scaled_inputs(s.ad_value(848), 0.000702, s.ad_value(843), 1.0));s.store_scalar(845, 0.00019230584);s.store_sqrt(848, 409);s.store_mul3_affine_lhs(846, 409, 848, 14500000000.0, 0.0, 845);s.store_sub_from_scalar_ad(849, 21.5565981, A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0));}
        s.b[1161] = (s.v[849] > (-100.0));s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1160]) && s.b[1161]) {s.store_exp(847, 849);}
        if ((s.b[1159] && s.b[1160]) && (!s.b[1161])) {s.store_scalar(847, (((-100.0)) as f64).exp());}
        if (s.b[1159] && s.b[1160]) {s.store_mul(911, 846, 847);}
        if (s.b[1159] && s.b[1160]) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && s.b[1160]) {s.store_mul(940, 832, 843);}
        if (s.b[1159] && (!s.b[1160])) {s.store_scalar(429, (p[126] + 273.15));s.store_scale(832, 409, 8.617087e-5);s.store_primal_scale(1104, 429, 8.617087e-5);s.copy_ad(1103, 394);s.store_sub_from_scalar_ad(912, p[49], A::div_scaled_product_offset_denominator(s.ad_value(409), s.ad_value(409), p[50], s.ad_value(409), p[51], 1.0));s.store_div_from_scalar_sqrt_ad(845, 1.0, A::mul(A::square(s.ad_value(429)), s.ad_value(429)));s.store_sqrt(848, 409);s.store_mul3_affine_lhs(846, 409, 848, p[48], 0.0, 845);s.store_exp_ad(847, A::sub(A::div_scaled_inputs(s.ad_value(1103), 1.0, s.ad_value(1104), 2.0), A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0)));s.store_mul(911, 846, 847);}
        if (s.b[1159] && (!s.b[1160])) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && (!s.b[1160])) {s.store_mul(940, 832, 843);}
        s.b[1162] = (s.v[109] > 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1162]) {
            if ((s.v[108] / s.v[109]) > 1e-38) {
                s.store_ln_div(843, 108, 109);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && s.b[1162]) {s.store_scaled_mul(941, 832, 843, (-p[37]));}
        if (s.b[1159] && (!s.b[1162])) {
            if (((((-s.v[108]) * s.v[109]) / s.v[911]) / s.v[911]) > 1e-38) {
                s.store_ln_ad(843, A::div_scaled_product_by_product(s.ad_value(108), s.ad_value(109), -1.0, s.ad_value(911), s.ad_value(911), 1.0));
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && (!s.b[1162])) {s.store_scaled_mul(941, 832, 843, (-p[37]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        if s.b[1159] {
            s.store_mul_scale_offset_mixed_ia(942, 832, {
                if ((s.v[108] / s.v[911]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 0.0);
        }
        if s.b[1159] {s.store_sqrt(943, 942);s.store_mul_sqrt_mixed_ia(944, 943, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)));s.store_div_mixed_ai(1140, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(417), 1.602176462e-19, s.ad_value(108)), (1000000.0 * 1.0 / (2.0))), 943);s.store_sqrt_ad(844, A::mul3(A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415), s.ad_value(944)));s.store_ad_value(843, A::exp_div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));s.store_add_scaled_product_indices(1141, 843, 1.0, 843, 843, 2.0);s.store_ad_value(843, A::exp_div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));s.store_add_scaled_product_indices(845, 843, 1.0, 843, 843, 2.0);s.store_add_scaled_product_indices(1142, 193, 1.0, 192, 845, 1.0);s.copy_ad(49, 832);s.store_mul_div_from_scalar_lhs_ad_indices(847, 1.115, 832, 430);s.store_div_scaled_product_indices(850, 256, 847, 1.0, 300, 1.0);}
        s.b[1163] = (s.v[850] > 100.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1163]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1164] = (s.v[850] < (-100.0));s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {s.store_exp(843, 850);}
        s.b[1165] = (s.v[256] == s.v[257]);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1165]) {s.copy_ad(844, 843);}
        if (s.b[1159] && (!s.b[1165])) {s.store_div_scaled_product_indices(850, 257, 847, 1.0, 300, 1.0);}
        s.b[1166] = (s.v[850] > 100.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1165])) && s.b[1166]) {s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1167] = (s.v[850] < (-100.0));s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && s.b[1167]) {s.store_scalar(844, 3.720075976e-44);}
        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && (!s.b[1167])) {s.store_exp(844, 850);}
        if s.b[1159] {s.store_div_scaled_product_indices(850, 258, 847, 1.0, 302, 1.0);}
        s.b[1168] = (s.v[850] > 100.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1168]) {s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1169] = (s.v[850] < (-100.0));s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1168])) && s.b[1169]) {s.store_scalar(845, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1168])) && (!s.b[1169])) {s.store_exp(845, 850);}
        if s.b[1159] {s.store_mul(972, 355, 843);s.store_mul(949, 306, 843);s.store_mul(947, 308, 844);s.store_mul(951, 310, 845);s.store_mul(850, 259, 430);}
        s.b[1170] = (s.v[850] > 100.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1170]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1171] = (s.v[850] < (-100.0));s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1170])) && s.b[1171]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1170])) && (!s.b[1171])) {s.store_exp(843, 850);}
        if s.b[1159] {s.store_mul(953, 312, 843);s.store_div_scaled_product_indices(850, 256, 847, 1.0, 301, 1.0);}
        s.b[1172] = (s.v[850] > 100.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1172]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1173] = (s.v[850] < (-100.0));s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1172])) && s.b[1173]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1172])) && (!s.b[1173])) {s.store_exp(843, 850);}
        s.b[1174] = (s.v[256] == s.v[260]);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1174]) {s.copy_ad(844, 843);}
        if (s.b[1159] && (!s.b[1174])) {s.store_div_scaled_product_indices(850, 260, 847, 1.0, 301, 1.0);}
        s.b[1175] = (s.v[850] > 100.0);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1174])) && s.b[1175]) {s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1176] = (s.v[850] < (-100.0));s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && s.b[1176]) {s.store_scalar(844, 3.720075976e-44);}
        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && (!s.b[1176])) {s.store_exp(844, 850);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1159] {s.store_div_scaled_product_indices(850, 261, 847, 1.0, 303, 1.0);}
        s.b[1177] = (s.v[850] > 100.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1177]) {s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1178] = (s.v[850] < (-100.0));s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1177])) && s.b[1178]) {s.store_scalar(845, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1177])) && (!s.b[1178])) {s.store_exp(845, 850);}
        if s.b[1159] {s.store_mul(973, 356, 843);s.store_mul(950, 307, 843);s.store_mul(948, 309, 844);s.store_mul(952, 311, 845);s.store_mul(850, 262, 430);}
        s.b[1179] = (s.v[850] > 100.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1179]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1180] = (s.v[850] < (-100.0));s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1179])) && s.b[1180]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1179])) && (!s.b[1180])) {s.store_exp(843, 850);}
        if s.b[1159] {s.store_mul(954, 313, 843);s.store_mul_pow_indices(945, 144, 411, 145);}
        s.b[1181] = (p[38] < 4.2);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1181]) {s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(411), p[238], 1.0), 1e-9);}
        if (s.b[1159] && (!s.b[1181])) {s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(430), p[238], 1.0), 1e-9);}
        if s.b[1159] {s.store_scale(850, 235, p[235]);s.store_div(960, 850, 961);s.store_scale(847, 51, p[235]);s.store_div(959, 847, 961);s.store_offset(845, 959, 1.0);s.store_offset(850, 960, 1.0);s.store_div(843, 845, 850);s.store_mul(945, 945, 843);s.store_add_scaled_product_indices(946, 101, 1.0, 102, 430, (-1.0));s.store_offset_mul(845, 45, 959, 1.0);s.store_offset_mul(850, 45, 960, 1.0);s.store_div(843, 845, 850);s.store_mul(946, 946, 843);}
        s.b[1182] = (p[429] != 1.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1182]) {s.store_div_scaled_add_product_indices(955, 181, 1.0, 186, 430, 1.0, 159, 1.0);s.store_scalar(1095, 0.0);s.store_scalar(1096, 0.0);}
        if (s.b[1159] && (!s.b[1182])) {s.store_scalar(955, 0.0);s.store_scale(1094, 159, p[3]);s.store_mul(853, 186, 430);s.store_add(844, 169, 853);s.store_offset(845, 853, p[140]);s.store_div(1095, 844, 1094);s.store_div(1097, 845, 1094);s.store_add(850, 170, 853);s.store_offset(847, 853, p[139]);s.store_div(1096, 850, 1094);s.store_div(1098, 847, 1094);}
        if s.b[1159] {s.store_add_scaled_product_indices(956, 153, 1.0, 139, 430, 1.0);s.store_add_scaled_product_indices(957, 154, 1.0, 141, 430, 1.0);s.store_add_scaled_product_indices(958, 155, 1.0, 143, 430, 1.0);}
        if (!s.b[1159]) {s.copy_ad(940, 115);s.copy_ad(941, 160);s.copy_ad(942, 118);s.copy_ad(943, 339);s.copy_ad(944, 340);s.copy_ad(912, 395);s.copy_ad(1140, 367);s.copy_ad(1141, 342);s.copy_ad(1142, 343);s.copy_ad(949, 161);s.copy_ad(950, 162);s.copy_ad(947, 163);s.copy_ad(948, 164);s.copy_ad(951, 165);s.copy_ad(952, 166);s.copy_ad(953, 167);s.copy_ad(954, 168);s.copy_ad(972, 357);s.copy_ad(973, 358);s.copy_ad(945, 404);s.copy_ad(946, 407);s.copy_ad(956, 138);s.copy_ad(957, 140);s.copy_ad(958, 142);}
        s.b[1183] = (param_given[90] || param_given[94]);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });s.b[1184] = (!param_given[90]);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if (s.b[1183] && s.b[1184]) {s.store_scalar(120, 0.53);}
        s.b[1185] = (!param_given[94]);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1183] && s.b[1185]) {s.store_scalar(124, (-0.0186));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1186] = (!param_given[87]);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (((!s.b[1183]) && s.b[1186]) && (p[41] != 0.0)) {s.store_scaled_div_from_scalar_ad(843, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);}
        if (((!s.b[1183]) && s.b[1186]) && (p[41] == 0.0)) {s.store_scalar(843, 0.00077348);}
        if ((!s.b[1183]) && s.b[1186]) {s.store_add_scaled_product_indices(114, 942, 1.0, 843, 108, (-(s.v[117] * s.v[117])));}
        s.b[1187] = (s.v[114] > 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1187]) {s.store_neg(114, 114);}
        s.b[1188] = (s.v[116] > 0.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1188]) {s.store_primal_neg(116, 116);}
        s.b[1189] = (!param_given[85]);s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1189]) {s.store_div_scaled_product_mixed_iai(112, 419, A::sqrt(s.ad_value(108)), 1.0, 396, 1.0);}
        s.b[1190] = (!param_given[86]);s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1190]) {s.store_div_scaled_product_mixed_iai(113, 419, A::sqrt(s.ad_value(109)), 1.0, 396, 1.0);}
        if (!s.b[1183]) {s.store_sub(843, 112, 113);s.store_sub_mixed_ai(844, A::sqrt(A::sub(s.ad_value(942), s.ad_value(114))), 943);s.store_mul_sub_mixed_iai(845, 943, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), 943);s.store_div_scaled_product_add_scaled_denominator_indices(846, 843, 844, 1.0, 845, 2.0, 116, 1.0, 1.0);s.store_add_scaled_inputs3_indices(402, 402, 1.0, 124, (-1.0), 846, 1.0);s.store_add_scaled_product_mixed_iia(120, 113, 1.0, 402, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), (-2.0));}
        s.store_offset(843, 265, s.v[328]);s.b[1191] = (s.v[843] < 1e-8);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });
        if s.b[1191] {s.store_scalar(843, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(405, 120, A::div(s.ad_value(264), s.ad_value(843)), 1.0, 1.0);s.store_scale(376, 405, (p[66] * 1.0 / (p[67])));s.store_scale(403, 402, (p[66] * 1.0 / (p[67])));s.b[1192] = (!param_given[109]);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });s.b[1193] = (param_given[108] || param_given[107]);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });
        if (s.b[1192] && s.b[1193]) {s.store_add_scaled_product_mixed_aii(406, A::add_scaled_inputs4(s.ad_value(406), 1.0, s.ad_value(152), (-1.0), s.ad_value(408), p[37], s.ad_value(942), -1.0), 1.0, 405, 943, (-1.0));}
        if (s.b[1192] && (!s.b[1193])) {
        }
        s.b[1194] = (!param_given[108]);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if s.b[1194] {s.store_add_scaled_inputs_product_indices(408, 406, p[37], 942, p[37], 405, 943, p[37]);}
        s.b[1195] = (p[38] < 4.2);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if s.b[1195] {s.copy_ad(1095, 173);s.copy_ad(1097, 171);s.copy_ad(1140, 367);s.copy_ad(1141, 342);s.copy_ad(1142, 343);}
        s.b[1196] = (p[62] == 4.0);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if (s.b[1195] && s.b[1196]) {s.copy_ad(956, 138);s.copy_ad(958, 142);}
        s.store_scaled_voltage(819, ctx, nodes, Some(7), Some(8), p[37]);s.store_scaled_voltage(818, ctx, nodes, Some(5), Some(8), p[37]);s.store_scaled_voltage(821, ctx, nodes, Some(9), Some(8), p[37]);s.store_scaled_voltage(897, ctx, nodes, Some(3), Some(8), p[37]);s.store_scaled_voltage(899, ctx, nodes, Some(5), Some(4), p[37]);s.store_scaled_voltage(1114, ctx, nodes, Some(9), Some(4), p[37]);s.store_scaled_voltage(1087, ctx, nodes, Some(11), Some(8), p[37]);s.store_scaled_voltage(1088, ctx, nodes, Some(12), Some(7), p[37]);s.store_scaled_voltage(1018, ctx, nodes, Some(10), Some(8), p[37]);s.store_sub(817, 818, 819);s.store_sub(820, 821, 819);s.store_sub(898, 897, 819);s.store_sub(1019, 1018, 819);s.b[1197] = (s.v[819] >= 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if s.b[1197] {s.store_scalar(398, 1.0);s.copy_ad(822, 819);s.copy_ad(823, 821);s.copy_ad(824, 818);s.copy_ad(900, 817);s.copy_ad(901, 897);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1197] {s.copy_ad(1110, 820);s.store_scalar(995, s.v[347]);s.store_scalar(996, s.v[348]);s.copy_ad(1143, 282);s.store_add_scaled_product_indices(1144, 283, 1.0, 284, 430, 1.0);s.copy_ad(1145, 285);s.copy_ad(1146, 286);s.copy_ad(1147, 287);s.copy_ad(1148, 288);s.copy_ad(1149, 289);s.copy_ad(1150, 290);s.store_add_scaled_product_indices(1151, 291, 1.0, 292, 430, 1.0);s.copy_ad(1152, 293);s.copy_ad(1153, 294);s.copy_ad(1154, 295);s.copy_ad(1155, 296);s.copy_ad(1156, 297);}
        if (!s.b[1197]) {s.store_scalar(398, (-1.0));s.store_neg(822, 819);s.copy_ad(823, 820);s.copy_ad(824, 817);s.copy_ad(900, 818);s.copy_ad(901, 898);s.copy_ad(1110, 821);s.store_scalar(995, s.v[348]);s.store_scalar(996, s.v[347]);s.copy_ad(1143, 290);s.store_add_scaled_product_indices(1144, 291, 1.0, 292, 430, 1.0);s.copy_ad(1145, 293);s.copy_ad(1146, 294);s.copy_ad(1147, 295);s.copy_ad(1148, 296);s.copy_ad(1149, 297);s.copy_ad(1150, 282);s.store_add_scaled_product_indices(1151, 283, 1.0, 284, 430, 1.0);s.copy_ad(1152, 285);s.copy_ad(1153, 286);s.copy_ad(1154, 287);s.copy_ad(1155, 288);s.copy_ad(1156, 289);}
        s.store_sub(902, 901, 941);s.store_scalar(913, s.v[392]);s.store_add(843, 406, 942);s.b[1198] = (p[41] == 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.copy_ad(418, 417);}
        if (!s.b[1198]) {s.store_scalar(418, (p[60] * 8.85418e-12));}
        s.b[1199] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[823] > s.v[843])) && (s.v[418] != 0.0));s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if s.b[1199] {s.store_div_scaled_product_mixed_iia(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(823), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);s.store_mul_scale_offset_indices(845, 844, 847, 1.0, (-1.0));s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);s.store_offset_sub_from_scalar_ad(850, p[1034], s.ad_value(846), (-0.05));s.store_sqrt_square_offset(849, 850, 0.224);s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p[1034]);s.store_sub(825, 823, 848);}
        if (!s.b[1199]) {s.copy_ad(825, 823);}
        s.b[1200] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[1110] > s.v[843])) && (s.v[418] != 0.0));s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_div_scaled_product_mixed_iia(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(1110), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);s.store_mul_scale_offset_indices(845, 844, 847, 1.0, (-1.0));s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);s.store_offset_sub_from_scalar_ad(850, p[1034], s.ad_value(846), (-0.05));s.store_sqrt_square_offset(849, 850, 0.224);s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p[1034]);s.store_sub(1111, 1110, 848);}
        if (!s.b[1200]) {s.copy_ad(1111, 1110);}
        s.copy_ad(1125, 823);s.store_scalar(892, s.v[327]);s.b[1201] = ((p[36] == 1.0) && (p[14] != 0.0));s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if s.b[1201] {s.store_scale(832, 409, 8.617087e-5);}
        if (!s.b[1201]) {s.copy_ad(832, 49);}
        s.store_sub(834, 940, 942);s.b[1202] = (s.v[37] == 0.0);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });
        if s.b[1202] {s.copy_ad(1033, 824);s.copy_ad(1048, 824);}
        s.b[1203] = (p[432] == 0.0);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });
    }
}
