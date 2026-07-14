#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[885])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);s.store_scalar(366, 0.0);}
        s.b[886] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[886]) {s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[887] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[886])) && s.b[887]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(283, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[886])) && (!s.b[887])) {s.store_scaled_softlimit_poly_offset_lhs_ad(283, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(372, 283, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);}
        if ((!s.b[858]) && s.b[866]) {s.store_offset(370, 370, (-1.0));s.store_offset(371, 371, (-1.0));s.store_offset(372, 372, (-1.0));s.store_div_from_scalar(222, 1.0, 223);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
    ) {
        s.b[888] = (s.v[277] > 0.0);s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && s.b[866]) && s.b[888]) {s.store_scaled_ln_ad(224, A::add(A::offset(s.ad_value(222), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(222), 1.0, A::offset(s.ad_value(222), 3.0)))), (s.v[84] * 2.0));}
        if (((!s.b[858]) && s.b[866]) && (!s.b[888])) {s.store_sub_mixed_ai(224, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(223), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(223), 1.0, A::scale_offset(s.ad_value(223), 3.0, 1.0))))), (s.v[84] * 2.0)), 277);}
        if ((!s.b[858]) && s.b[866]) {s.store_sub(225, 264, 224);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(226, 277, 0.5, 225, 0.5, 277, 225, ((4.0 * s.v[84]) * s.v[84]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(227, 277, 0.5, 267, 0.5, A::add_scaled_square_product(A::sub(s.ad_value(277), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0), (-0.5));s.store_scaled_sub_mixed_ia(228, 277, A::sqrt_square_offset(s.ad_value(277), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        if ((!s.b[858]) && (!s.b[866])) {s.store_scalar(370, 0.0);s.store_scalar(371, 0.0);s.store_scalar(372, 0.0);s.store_scalar(224, 0.0);s.store_scalar(221, 0.0);s.store_scalar(223, 0.0);s.store_scalar(226, 0.0);s.store_scalar(227, 0.0);s.store_scalar(228, 0.0);}
        s.b[889] = (s.v[256] == 0.0);s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });
        if ((!s.b[858]) && s.b[889]) {s.store_scalar(268, 0.0);s.store_scalar(291, 0.0);s.store_scalar(269, 0.0);}
        s.b[890] = (s.v[122] == 0.5);s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[889])) && s.b[890]) {s.store_sqrt_sub_from_scalar_ad(229, 1.0, A::mul(s.ad_value(221), s.ad_value(119)));}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[890])) {s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(119))), s.v[122]);}
        if ((!s.b[858]) && (!s.b[889])) {s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(229)), 1.0, 134, A::sub(s.ad_value(277), s.ad_value(221)), 1.0);s.store_mul(230, 101, 370);}
        s.b[891] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[889])) && s.b[891]) {s.store_scalar(232, 0.0);s.store_scalar(235, 0.0);s.store_scalar(236, 0.0);s.store_scalar(237, 0.0);s.store_scalar(231, 0.0);}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[891])) {s.store_sub(232, 107, 226);s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));}
        s.b[892] = (s.v[9] == 0.5);s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && s.b[892]) {s.store_scalar(234, 0.0);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && (!s.b[892])) {s.store_scaled_add_mixed_ai(234, A::div_scaled_product(A::square(s.ad_value(233)), A::ln(s.ad_value(233)), 1.0, A::sub_from_scalar(1.0, s.ad_value(233)), 1.0), 233, (1.0 - (2.0 * s.v[9])));}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[891])) {s.store_add(235, 233, 234);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[893] = (s.v[9] == 0.5);s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && s.b[893]) {s.store_sqrt_scaled_input(229, 232, s.v[143]);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && (!s.b[893])) {s.store_powf_scaled_input(229, 232, s.v[143], s.v[9]);}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[891])) {s.store_scale(236, 229, s.v[137]);s.store_mul_ad_product_lhs_mixed_ia(237, 98, A::offset(s.ad_value(223), (-1.0)), 236);s.store_scaled_mul(231, 237, 235, s.v[20]);}
        s.b[894] = (s.v[23] == 0.0);s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[889])) && s.b[894]) {s.store_scalar(238, 0.0);}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {s.store_div_scaled_inputs_indices(239, 236, (s.v[122] * s.v[152]), 232, 1.0);s.store_div_from_scalar(240, (0.666666666666667 * s.v[149]), 239);s.store_square(241, 240);s.store_sqrt_div_scaled_square_offset_denominator(242, 241, 1.0, 1.0, 1.0);s.store_sqrt_abs_ad(243, s.ad_value(242));s.store_mul(244, 242, 243);}
        s.b[895] = (((-s.v[9]) * s.v[125]) == (-1.0));s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[895]) {s.store_div_from_scalar_offset_product(245, 1.0, 239, 244, 1.0);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[895])) {s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[9]) * s.v[125]));}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {s.store_div_scaled_product_add_scaled_denominator_indices(246, 235, 245, 1.0, 235, 1.0, 245, 1.0, 1.0);s.store_sqrt_scaled_input_ad(247, A::div(s.ad_value(239), s.ad_value(243)), 0.375);s.store_add_scaled_product_indices(248, 242, (-1.0), 240, 243, 2.0);s.store_add_scaled_value_products_indices(249, 242, (-s.v[149]), 240, 243, s.v[149], 239, 244, 0.5);s.store_mul_scale_offset_indices(250, 247, 248, 1.0, (-1.0));s.store_square(212, 250);}
        s.b[896] = (s.v[250] > 0.0);s.store_scalar(896, if s.b[896] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[896]) {s.store_div_from_scalar_offset_scaled_input(213, 1.0, 250, s.v[86], 1.0);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[896])) {s.store_div_from_scalar_sub_from_scalar_ad(213, 1.0, 1.0, A::scale(s.ad_value(250), s.v[86]));}
        s.b[897] = (((-s.v[212]) + s.v[249]) > (-230.25850929940458));s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[897]) {s.store_exp_sub(229, 249, 212);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[897])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(229, 1e-100, (-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {s.store_mul_mixed_ai(214, A::add_scaled_inputs_product(s.ad_value(213), 0.29214664, A::square(s.ad_value(213)), s.v[87], A::square(s.ad_value(213)), s.ad_value(213), s.v[88]), 229);}
        s.b[898] = (s.v[250] > 0.0);s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[898]) {s.copy_ad(251, 214);}
        s.b[899] = (s.v[249] > (-230.25850929940458));s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[898])) && s.b[899]) {s.store_exp(229, 249);}
        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[898])) && (!s.b[899])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(229, 1e-100, (-230.25850929940458), 249, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[898])) {s.store_sub_scaled_inputs(251, 229, 2.0, 214, 1.0);}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {s.store_div_scaled_inputs_indices(252, 251, (s.v[149] * (1.772453850905516 * 0.5)), 247, 1.0);s.store_mul3_affine_lhs(238, 237, 252, s.v[23], 0.0, 246);}
        s.b[900] = (s.v[29] == 0.0);s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[889])) && s.b[900]) {s.store_scalar(253, 0.0);}
        s.b[901] = (s.v[9] == 0.5);s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && s.b[901]) {s.store_sqrt_scaled_input_ad(229, A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[143]);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && (!s.b[901])) {s.store_powf_scale_offset_input(229, 227, (-s.v[143]), ((s.v[6]) * (s.v[143])), s.v[9]);}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[900])) {s.store_div_scaled_offset_numerator_indices(254, 227, ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), 229, 1.0);}
        s.b[902] = (((((-s.v[155]) / s.v[254])) as f64).abs() < 230.25850929940458);s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && s.b[902]) {s.store_ad_value(229, A::exp_div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(254), 1.0));}
        s.b[903] = (((-s.v[155]) / s.v[254]) < (-230.25850929940458));s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && (!s.b[902])) && s.b[903]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(229, 1e-100, (-230.25850929940458), 155, -1.0, 254, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && (!s.b[902])) && (!s.b[903])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(229, 155, -1.0, 254, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[900])) {s.store_mul_scale_offset_mixed_ai(253, A::mul3(s.ad_value(277), s.ad_value(254), s.ad_value(254)), 229, s.v[29], 0.0);}
        s.b[904] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[889])) && s.b[904]) {s.store_scalar(255, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
    ) {
        s.b[905] = (s.v[228] > ((-s.v[158]) * s.v[38]));s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });s.b[906] = (s.v[41] == 4.0);s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && s.b[905]) && s.b[906]) {s.store_mul3_ad(229, A::square(A::abs(A::mul(s.ad_value(228), s.ad_value(162)))), A::abs(A::mul(s.ad_value(228), s.ad_value(162))), A::abs(A::mul(s.ad_value(228), s.ad_value(162))));}
        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && s.b[905]) && (!s.b[906])) {s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(162))), s.v[41]);}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && s.b[905]) {s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));}
        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && (!s.b[905])) {s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);}
        if ((!s.b[858]) && (!s.b[889])) {s.store_mul_add_scaled_inputs4_indices_rhs(268, 255, 230, 1.0, 231, 1.0, 238, 1.0, 253, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(291, 255, 231, 1.0, 238, 1.0, 253, 1.0, 0.0);}
        s.b[907] = (s.v[257] == 0.0);s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });
        if ((!s.b[858]) && s.b[907]) {s.store_scalar(270, 0.0);s.store_scalar(292, 0.0);s.store_scalar(271, 0.0);}
        s.b[908] = (s.v[123] == 0.5);s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[907])) && s.b[908]) {s.store_sqrt_sub_from_scalar_ad(229, 1.0, A::mul(s.ad_value(221), s.ad_value(120)));}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[908])) {s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(120))), s.v[123]);}
        if ((!s.b[858]) && (!s.b[907])) {s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(229)), 1.0, 135, A::sub(s.ad_value(277), s.ad_value(221)), 1.0);s.store_mul(230, 102, 371);}
        s.b[909] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[907])) && s.b[909]) {s.store_scalar(232, 0.0);s.store_scalar(235, 0.0);s.store_scalar(236, 0.0);s.store_scalar(237, 0.0);s.store_scalar(231, 0.0);}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[909])) {s.store_sub(232, 108, 226);s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));}
        s.b[910] = (s.v[10] == 0.5);s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && s.b[910]) {s.store_scalar(234, 0.0);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && (!s.b[910])) {s.store_scaled_add_mixed_ai(234, A::div_scaled_product(A::square(s.ad_value(233)), A::ln(s.ad_value(233)), 1.0, A::sub_from_scalar(1.0, s.ad_value(233)), 1.0), 233, (1.0 - (2.0 * s.v[10])));}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[909])) {s.store_add(235, 233, 234);}
        s.b[911] = (s.v[10] == 0.5);s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && s.b[911]) {s.store_sqrt_scaled_input(229, 232, s.v[144]);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && (!s.b[911])) {s.store_powf_scaled_input(229, 232, s.v[144], s.v[10]);}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[909])) {s.store_scale(236, 229, s.v[138]);s.store_mul_ad_product_lhs_mixed_ia(237, 99, A::offset(s.ad_value(223), (-1.0)), 236);s.store_scaled_mul(231, 237, 235, s.v[21]);}
        s.b[912] = (s.v[24] == 0.0);s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[907])) && s.b[912]) {s.store_scalar(238, 0.0);}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {s.store_div_scaled_inputs_indices(239, 236, (s.v[123] * s.v[153]), 232, 1.0);s.store_div_from_scalar(240, (0.666666666666667 * s.v[150]), 239);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {s.store_square(241, 240);s.store_sqrt_div_scaled_square_offset_denominator(242, 241, 1.0, 1.0, 1.0);s.store_sqrt_abs_ad(243, s.ad_value(242));s.store_mul(244, 242, 243);}
        s.b[913] = (((-s.v[10]) * s.v[126]) == (-1.0));s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[913]) {s.store_div_from_scalar_offset_product(245, 1.0, 239, 244, 1.0);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[913])) {s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[10]) * s.v[126]));}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {s.store_div_scaled_product_add_scaled_denominator_indices(246, 235, 245, 1.0, 235, 1.0, 245, 1.0, 1.0);s.store_sqrt_scaled_input_ad(247, A::div(s.ad_value(239), s.ad_value(243)), 0.375);s.store_add_scaled_product_indices(248, 242, (-1.0), 240, 243, 2.0);s.store_add_scaled_value_products_indices(249, 242, (-s.v[150]), 240, 243, s.v[150], 239, 244, 0.5);s.store_mul_scale_offset_indices(250, 247, 248, 1.0, (-1.0));s.store_square(212, 250);}
        s.b[914] = (s.v[250] > 0.0);s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[914]) {s.store_div_from_scalar_offset_scaled_input(213, 1.0, 250, s.v[86], 1.0);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[914])) {s.store_div_from_scalar_sub_from_scalar_ad(213, 1.0, 1.0, A::scale(s.ad_value(250), s.v[86]));}
        s.b[915] = (((-s.v[212]) + s.v[249]) > (-230.25850929940458));s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[915]) {s.store_exp_sub(229, 249, 212);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[915])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(229, 1e-100, (-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {s.store_mul_mixed_ai(214, A::add_scaled_inputs_product(s.ad_value(213), 0.29214664, A::square(s.ad_value(213)), s.v[87], A::square(s.ad_value(213)), s.ad_value(213), s.v[88]), 229);}
        s.b[916] = (s.v[250] > 0.0);s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[916]) {s.copy_ad(251, 214);}
        s.b[917] = (s.v[249] > (-230.25850929940458));s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[916])) && s.b[917]) {s.store_exp(229, 249);}
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[916])) && (!s.b[917])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(229, 1e-100, (-230.25850929940458), 249, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[916])) {s.store_sub_scaled_inputs(251, 229, 2.0, 214, 1.0);}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {s.store_div_scaled_inputs_indices(252, 251, (s.v[150] * (1.772453850905516 * 0.5)), 247, 1.0);s.store_mul3_affine_lhs(238, 237, 252, s.v[24], 0.0, 246);}
        s.b[918] = (s.v[30] == 0.0);s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[907])) && s.b[918]) {s.store_scalar(253, 0.0);}
        s.b[919] = (s.v[10] == 0.5);s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && s.b[919]) {s.store_sqrt_scaled_input_ad(229, A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[144]);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && (!s.b[919])) {s.store_powf_scale_offset_input(229, 227, (-s.v[144]), ((s.v[7]) * (s.v[144])), s.v[10]);}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[918])) {s.store_div_scaled_offset_numerator_indices(254, 227, ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), 229, 1.0);}
        s.b[920] = (((((-s.v[156]) / s.v[254])) as f64).abs() < 230.25850929940458);s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && s.b[920]) {s.store_ad_value(229, A::exp_div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(254), 1.0));}
        s.b[921] = (((-s.v[156]) / s.v[254]) < (-230.25850929940458));s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && (!s.b[920])) && s.b[921]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(229, 1e-100, (-230.25850929940458), 156, -1.0, 254, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && (!s.b[920])) && (!s.b[921])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(229, 156, -1.0, 254, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && (!s.b[907])) && (!s.b[918])) {s.store_mul_scale_offset_mixed_ai(253, A::mul3(s.ad_value(277), s.ad_value(254), s.ad_value(254)), 229, s.v[30], 0.0);}
        s.b[922] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[907])) && s.b[922]) {s.store_scalar(255, 1.0);}
        s.b[923] = (s.v[228] > ((-s.v[158]) * s.v[39]));s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });s.b[924] = (s.v[42] == 4.0);s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
    ) {
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) && s.b[924]) {s.store_mul3_ad(229, A::square(A::abs(A::mul(s.ad_value(228), s.ad_value(163)))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))));}
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) && (!s.b[924])) {s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(163))), s.v[42]);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) {s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && (!s.b[923])) {s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);}
        if ((!s.b[858]) && (!s.b[907])) {s.store_mul_add_scaled_inputs4_indices_rhs(270, 255, 230, 1.0, 231, 1.0, 238, 1.0, 253, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(292, 255, 231, 1.0, 238, 1.0, 253, 1.0, 0.0);}
        s.b[925] = (s.v[258] == 0.0);s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if ((!s.b[858]) && s.b[925]) {s.store_scalar(272, 0.0);s.store_scalar(293, 0.0);s.store_scalar(273, 0.0);}
        s.b[926] = (s.v[124] == 0.5);s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[925])) && s.b[926]) {s.store_sqrt_sub_from_scalar_ad(229, 1.0, A::mul(s.ad_value(221), s.ad_value(121)));}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[926])) {s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(121))), s.v[124]);}
        if ((!s.b[858]) && (!s.b[925])) {s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(229)), 1.0, 136, A::sub(s.ad_value(277), s.ad_value(221)), 1.0);s.store_mul(230, 103, 372);}
        s.b[927] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[925])) && s.b[927]) {s.store_scalar(232, 0.0);s.store_scalar(235, 0.0);s.store_scalar(236, 0.0);s.store_scalar(237, 0.0);s.store_scalar(231, 0.0);}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[927])) {s.store_sub(232, 109, 226);s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));}
        s.b[928] = (s.v[11] == 0.5);s.store_scalar(928, if s.b[928] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && s.b[928]) {s.store_scalar(234, 0.0);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && (!s.b[928])) {s.store_scaled_add_mixed_ai(234, A::div_scaled_product(A::square(s.ad_value(233)), A::ln(s.ad_value(233)), 1.0, A::sub_from_scalar(1.0, s.ad_value(233)), 1.0), 233, (1.0 - (2.0 * s.v[11])));}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[927])) {s.store_add(235, 233, 234);}
        s.b[929] = (s.v[11] == 0.5);s.store_scalar(929, if s.b[929] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && s.b[929]) {s.store_sqrt_scaled_input(229, 232, s.v[145]);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && (!s.b[929])) {s.store_powf_scaled_input(229, 232, s.v[145], s.v[11]);}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[927])) {s.store_scale(236, 229, s.v[139]);s.store_mul_ad_product_lhs_mixed_ia(237, 100, A::offset(s.ad_value(223), (-1.0)), 236);s.store_scaled_mul(231, 237, 235, s.v[22]);}
        s.b[930] = (s.v[25] == 0.0);s.store_scalar(930, if s.b[930] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[925])) && s.b[930]) {s.store_scalar(238, 0.0);}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {s.store_div_scaled_inputs_indices(239, 236, (s.v[124] * s.v[154]), 232, 1.0);s.store_div_from_scalar(240, (0.666666666666667 * s.v[151]), 239);s.store_square(241, 240);s.store_sqrt_div_scaled_square_offset_denominator(242, 241, 1.0, 1.0, 1.0);s.store_sqrt_abs_ad(243, s.ad_value(242));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {s.store_mul(244, 242, 243);}
        s.b[931] = (((-s.v[11]) * s.v[127]) == (-1.0));s.store_scalar(931, if s.b[931] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[931]) {s.store_div_from_scalar_offset_product(245, 1.0, 239, 244, 1.0);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[931])) {s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[11]) * s.v[127]));}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {s.store_div_scaled_product_add_scaled_denominator_indices(246, 235, 245, 1.0, 235, 1.0, 245, 1.0, 1.0);s.store_sqrt_scaled_input_ad(247, A::div(s.ad_value(239), s.ad_value(243)), 0.375);s.store_add_scaled_product_indices(248, 242, (-1.0), 240, 243, 2.0);s.store_add_scaled_value_products_indices(249, 242, (-s.v[151]), 240, 243, s.v[151], 239, 244, 0.5);s.store_mul_scale_offset_indices(250, 247, 248, 1.0, (-1.0));s.store_square(212, 250);}
        s.b[932] = (s.v[250] > 0.0);s.store_scalar(932, if s.b[932] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[932]) {s.store_div_from_scalar_offset_scaled_input(213, 1.0, 250, s.v[86], 1.0);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[932])) {s.store_div_from_scalar_sub_from_scalar_ad(213, 1.0, 1.0, A::scale(s.ad_value(250), s.v[86]));}
        s.b[933] = (((-s.v[212]) + s.v[249]) > (-230.25850929940458));s.store_scalar(933, if s.b[933] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[933]) {s.store_exp_sub(229, 249, 212);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[933])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(229, 1e-100, (-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {s.store_mul_mixed_ai(214, A::add_scaled_inputs_product(s.ad_value(213), 0.29214664, A::square(s.ad_value(213)), s.v[87], A::square(s.ad_value(213)), s.ad_value(213), s.v[88]), 229);}
        s.b[934] = (s.v[250] > 0.0);s.store_scalar(934, if s.b[934] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[934]) {s.copy_ad(251, 214);}
        s.b[935] = (s.v[249] > (-230.25850929940458));s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[934])) && s.b[935]) {s.store_exp(229, 249);}
        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[934])) && (!s.b[935])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(229, 1e-100, (-230.25850929940458), 249, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[934])) {s.store_sub_scaled_inputs(251, 229, 2.0, 214, 1.0);}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {s.store_div_scaled_inputs_indices(252, 251, (s.v[151] * (1.772453850905516 * 0.5)), 247, 1.0);s.store_mul3_affine_lhs(238, 237, 252, s.v[25], 0.0, 246);}
        s.b[936] = (s.v[31] == 0.0);s.store_scalar(936, if s.b[936] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[925])) && s.b[936]) {s.store_scalar(253, 0.0);}
        s.b[937] = (s.v[11] == 0.5);s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && s.b[937]) {s.store_sqrt_scaled_input_ad(229, A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[145]);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && (!s.b[937])) {s.store_powf_scale_offset_input(229, 227, (-s.v[145]), ((s.v[8]) * (s.v[145])), s.v[11]);}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[936])) {s.store_div_scaled_offset_numerator_indices(254, 227, ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), 229, 1.0);}
        s.b[938] = (((((-s.v[157]) / s.v[254])) as f64).abs() < 230.25850929940458);s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && s.b[938]) {s.store_ad_value(229, A::exp_div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(254), 1.0));}
        s.b[939] = (((-s.v[157]) / s.v[254]) < (-230.25850929940458));s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && (!s.b[938])) && s.b[939]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(229, 1e-100, (-230.25850929940458), 157, -1.0, 254, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && (!s.b[938])) && (!s.b[939])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(229, 157, -1.0, 254, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[936])) {s.store_mul_scale_offset_mixed_ai(253, A::mul3(s.ad_value(277), s.ad_value(254), s.ad_value(254)), 229, s.v[31], 0.0);}
        s.b[940] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });
        if (((!s.b[858]) && (!s.b[925])) && s.b[940]) {s.store_scalar(255, 1.0);}
        s.b[941] = (s.v[228] > ((-s.v[158]) * s.v[40]));s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });s.b[942] = (s.v[43] == 4.0);s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });
        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) && s.b[942]) {s.store_mul3_ad(229, A::square(A::abs(A::mul(s.ad_value(228), s.ad_value(164)))), A::abs(A::mul(s.ad_value(228), s.ad_value(164))), A::abs(A::mul(s.ad_value(228), s.ad_value(164))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_82(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) && (!s.b[942])) {s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(164))), s.v[43]);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) {s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);}
        if ((!s.b[858]) && (!s.b[925])) {s.store_mul_add_scaled_inputs4_indices_rhs(272, 255, 230, 1.0, 231, 1.0, 238, 1.0, 253, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(293, 255, 231, 1.0, 238, 1.0, 253, 1.0, 0.0);}
        if (!s.b[858]) {s.store_add_scaled_inputs3_indices(274, 268, s.v[256], 270, s.v[257], 272, s.v[258]);s.store_add_scaled_inputs3_indices(290, 291, s.v[256], 292, s.v[257], 293, s.v[258]);}
        s.store_add_scaled_inputs3_indices(275, 269, s.v[256], 271, s.v[257], 273, s.v[258]);s.b[945] = (p.p84 > 0.0);s.store_scalar(945, if s.b[945] { 1.0 } else { 0.0 });s.b[946] = (s.v[313] < p.p85);s.store_scalar(946, if s.b[946] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[946]) {s.store_offset_sub_scaled_inputs_indices(349, 277, p.p86, 348, p.p86, s.v[313]);s.store_sub_from_scalar_scaled_input(350, s.v[313], 348, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(349), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (s.b[945] && s.b[946]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(351, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 351, (((-s.v[313])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));}
        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (s.b[945] && s.b[946]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(352, 314, 0.5, 315, 0.5, s.v[313]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (s.b[945] && s.b[946]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[313])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));}
        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (s.b[945] && s.b[946]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[313]);}
        if (s.b[945] && (!s.b[946])) {s.store_scalar(352, s.v[313]);s.store_scalar(350, s.v[313]);}
        if s.b[945] {s.copy_ad(353, 370);}
        s.b[947] = ((s.v[277] - (s.v[348] - s.v[347])) > 0.0);s.store_scalar(947, if s.b[947] { 1.0 } else { 0.0 });s.b[948] = ((((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(948, if s.b[948] { 1.0 } else { 0.0 });
        if ((s.b[945] && s.b[947]) && s.b[948]) {s.store_exp_scaled_input_ad(354, A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), 1.0, A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), (-1.0), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), 1.0), s.v[85]);}
        s.b[949] = ((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(949, if s.b[949] { 1.0 } else { 0.0 });
        if (((s.b[945] && s.b[947]) && (!s.b[948])) && s.b[949]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(354, 1e-100, (-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_83(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[945] && s.b[947]) && (!s.b[948])) && (!s.b[949])) {s.store_scaled_softlimit_poly_offset_lhs_ad(354, A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[945] && (!s.b[947])) {s.store_scalar(354, 1.0);}
        s.b[950] = ((p.p91 == 0.0) || (s.v[277] < s.v[347]));s.store_scalar(950, if s.b[950] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[950]) {s.store_scale(357, 353, p.p90);}
        if (s.b[945] && (!s.b[950])) {s.store_mul_scaled_exp_ad_rhs(357, 353, p.p90, A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(347)), A::sub(s.ad_value(277), s.ad_value(347)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91)));}
        if s.b[945] {
            if (s.v[357] > p.p79) {
                s.store_scalar(357, p.p79);
            } else {
            }
        }
        if s.b[945] {s.store_mul(355, 319, 357);s.store_scaled_sub(331, 355, 319, (1.6021918e-19 * s.v[256]));}
        s.b[951] = (p.p92 > 0.0);s.store_scalar(951, if s.b[951] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[951]) {s.store_scale(334, 331, (1e-23 / s.v[333]));s.store_voltage(336, ctx, nodes, Some(3), None);s.store_scaled_sub(338, 336, 334, 1.0 / (p.p92));s.store_scale(340, 336, 1.0 / ((1e-23 / s.v[333])));}
        if (s.b[945] && (!s.b[951])) {s.copy_ad(334, 331);s.copy_ad(340, 334);}
        s.b[952] = ((p.p91 == 0.0) || (s.v[277] < s.v[348]));s.store_scalar(952, if s.b[952] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[952]) {s.store_scale(358, 354, p.p90);}
        if (s.b[945] && (!s.b[952])) {s.store_mul_scaled_exp_ad_rhs(358, 354, p.p90, A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(348)), A::sub(s.ad_value(277), s.ad_value(348)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91)));}
        if s.b[945] {
            if (s.v[358] > p.p79) {
                s.store_scalar(358, p.p79);
            } else {
            }
        }
        if s.b[945] {s.store_mul(356, 319, 358);s.store_scaled_sub(332, 356, 319, (1.6021918e-19 * s.v[256]));}
        s.b[953] = (p.p92 > 0.0);s.store_scalar(953, if s.b[953] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[953]) {s.store_scale(335, 332, (1e-23 / s.v[333]));s.store_voltage(337, ctx, nodes, Some(4), None);s.store_scaled_sub(339, 337, 335, 1.0 / (p.p92));s.store_scale(341, 337, 1.0 / ((1e-23 / s.v[333])));}
        if (s.b[945] && (!s.b[953])) {s.copy_ad(335, 332);s.copy_ad(341, 335);}
        if s.b[945] {s.store_sub_from_scalar(325, s.v[368], 277);s.store_sqrt_square_offset(315, 325, ((4.0 * s.v[369]) * s.v[369]));s.store_scaled_add(325, 325, 315, 0.5);}
        s.b[954] = (s.v[325] < 0.0);s.store_scalar(954, if s.b[954] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[954]) {s.store_scalar(325, 0.0);}
        if s.b[945] {s.store_sqrt_scaled_input(326, 325, ((2.0 * s.v[0]) * 1.0 / ((1.6021918e-19 * s.v[307]))));s.store_offset_sub_from_scalar_ad(314, p.p94, s.ad_value(326), (-1e-7));s.store_scalar(315, ((4.0 * p.p94) * 1e-7));}
        if s.b[945] {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if s.b[945] {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(326, 314, (-0.5), 315, (-0.5), p.p94);}
        s.b[955] = (p.p95 > 0.0);s.store_scalar(955, if s.b[955] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[955]) {s.store_mul_div_from_scalar_lhs_ad_indices(342, 1.0, 343, 326);s.store_voltage(344, ctx, nodes, Some(5), None);s.store_scaled_sub(345, 344, 342, 1.0 / (p.p95));s.store_div_mixed_ia(346, 344, A::div_from_scalar(1.0, s.ad_value(343)));}
        if (s.b[945] && (!s.b[955])) {s.copy_ad(342, 326);s.copy_ad(346, 342);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_84(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[945] {s.store_scalar(327, ((-((s.v[307] * s.v[256]) * 1.6021918e-19)) * p.p94));s.store_mul_ad_product_rhs_mixed_ia(328, 323, 340, A::sub(A::exp(A::div_from_scalar((-p.p94), s.ad_value(323))), A::exp_div_scaled_inputs(s.ad_value(346), -1.0, s.ad_value(323), 1.0)));s.store_mul_ad_product_rhs_mixed_ia(329, 323, 341, A::offset(A::exp_div_scaled_inputs(A::sub_from_scalar(p.p94, s.ad_value(346)), -1.0, s.ad_value(323), 1.0), (-1.0)));s.store_add_scaled_inputs3_indices(330, 327, (-1.0), 328, (-1.0), 329, (-1.0));s.store_add(275, 275, 330);s.store_scalar(55, 0.0);}
        if (!s.b[945]) {s.store_mul_sub_rhs(330, 55, 274, 290);}
        s.b[958] = ((p.p84 > 0.0) && (p.p92 > 0.0));s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });s.b[959] = ((p.p84 > 0.0) && (p.p95 > 0.0));s.store_scalar(959, if s.b[959] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let eq3_value: f64 = s.v[274];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq3_value),
            &s.dn[274],
            &s.db[274],
            multiplicity,
        );let eq4_e122: f64 = 0.0;let eq4_e124: f64 = (eq4_e122 * (nv0 - nv2));let eq4_value: f64 = eq4_e124;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq4_value),
            0,
            multiplicity * (eq4_e122),
            2,
            multiplicity * ((-eq4_e122)),
        );
        let (eq5_e130, eq5_e130_d_n0, eq5_e130_d_n1, eq5_e130_d_n2, eq5_e130_d_n3, eq5_e130_d_n4, eq5_e130_d_n5, eq5_e130_d_b0, eq5_e130_d_b1, eq5_e130_d_b2, eq5_e130_d_b3,) = {
    if s.b[957] {
        let eq5_e128: f64 = (s.v[284] / s.v[171]);let __rspice_inv_cse_0: f64 = 1.0 / (s.v[171] * s.v[171]);let eq5_e128_d_n0: f64 = (((s.dn[284][0] * s.v[171]) - (s.v[284] * s.dn[171][0])) * __rspice_inv_cse_0);let eq5_e128_d_n1: f64 = (((s.dn[284][1] * s.v[171]) - (s.v[284] * s.dn[171][1])) * __rspice_inv_cse_0);let eq5_e128_d_n2: f64 = (((s.dn[284][2] * s.v[171]) - (s.v[284] * s.dn[171][2])) * __rspice_inv_cse_0);let eq5_e128_d_n3: f64 = (((s.dn[284][3] * s.v[171]) - (s.v[284] * s.dn[171][3])) * __rspice_inv_cse_0);let eq5_e128_d_n4: f64 = (((s.dn[284][4] * s.v[171]) - (s.v[284] * s.dn[171][4])) * __rspice_inv_cse_0);let eq5_e128_d_n5: f64 = (((s.dn[284][5] * s.v[171]) - (s.v[284] * s.dn[171][5])) * __rspice_inv_cse_0);let eq5_e128_d_b0: f64 = (((s.db[284][0] * s.v[171]) - (s.v[284] * s.db[171][0])) * __rspice_inv_cse_0);let eq5_e128_d_b1: f64 = (((s.db[284][1] * s.v[171]) - (s.v[284] * s.db[171][1])) * __rspice_inv_cse_0);let eq5_e128_d_b2: f64 = (((s.db[284][2] * s.v[171]) - (s.v[284] * s.db[171][2])) * __rspice_inv_cse_0);let eq5_e128_d_b3: f64 = (((s.db[284][3] * s.v[171]) - (s.v[284] * s.db[171][3])) * __rspice_inv_cse_0);
        (eq5_e128, eq5_e128_d_n0, eq5_e128_d_n1, eq5_e128_d_n2, eq5_e128_d_n3, eq5_e128_d_n4, eq5_e128_d_n5, eq5_e128_d_b0, eq5_e128_d_b1, eq5_e128_d_b2, eq5_e128_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e130;let eq5_node_derivatives: [f64; 6] = [eq5_e130_d_n0, eq5_e130_d_n1, eq5_e130_d_n2, eq5_e130_d_n3, eq5_e130_d_n4, eq5_e130_d_n5];let eq5_branch_derivatives: [f64; 4] = [eq5_e130_d_b0, eq5_e130_d_b1, eq5_e130_d_b2, eq5_e130_d_b3];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(1),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e135,) = {
    if (!s.b[957]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e135;
        stamper.stamp_potential_const_local(
            0,
            eq6_value,
        );
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3,) = {
    if s.b[958] {
        let eq7_e140: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[336]);let eq7_e141: f64 = (s.v[338] + eq7_e140);let eq7_e141_d_n0: f64 = (s.dn[338][0] + (s.dn[336][0] * ddt_scale));let eq7_e141_d_n1: f64 = (s.dn[338][1] + (s.dn[336][1] * ddt_scale));let eq7_e141_d_n2: f64 = (s.dn[338][2] + (s.dn[336][2] * ddt_scale));let eq7_e141_d_n3: f64 = (s.dn[338][3] + (s.dn[336][3] * ddt_scale));let eq7_e141_d_n4: f64 = (s.dn[338][4] + (s.dn[336][4] * ddt_scale));let eq7_e141_d_n5: f64 = (s.dn[338][5] + (s.dn[336][5] * ddt_scale));let eq7_e141_d_b0: f64 = (s.db[338][0] + (s.db[336][0] * ddt_scale));let eq7_e141_d_b1: f64 = (s.db[338][1] + (s.db[336][1] * ddt_scale));let eq7_e141_d_b2: f64 = (s.db[338][2] + (s.db[336][2] * ddt_scale));let eq7_e141_d_b3: f64 = (s.db[338][3] + (s.db[336][3] * ddt_scale));let eq7_e142: f64 = (1e-12 * eq7_e141);let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e144;let eq7_node_derivatives: [f64; 6] = [eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5];let eq7_branch_derivatives: [f64; 4] = [eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3,) = {
    if s.b[958] {
        let eq8_e149: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[337]);let eq8_e150: f64 = (s.v[339] + eq8_e149);let eq8_e150_d_n0: f64 = (s.dn[339][0] + (s.dn[337][0] * ddt_scale));let eq8_e150_d_n1: f64 = (s.dn[339][1] + (s.dn[337][1] * ddt_scale));let eq8_e150_d_n2: f64 = (s.dn[339][2] + (s.dn[337][2] * ddt_scale));let eq8_e150_d_n3: f64 = (s.dn[339][3] + (s.dn[337][3] * ddt_scale));let eq8_e150_d_n4: f64 = (s.dn[339][4] + (s.dn[337][4] * ddt_scale));let eq8_e150_d_n5: f64 = (s.dn[339][5] + (s.dn[337][5] * ddt_scale));let eq8_e150_d_b0: f64 = (s.db[339][0] + (s.db[337][0] * ddt_scale));let eq8_e150_d_b1: f64 = (s.db[339][1] + (s.db[337][1] * ddt_scale));let eq8_e150_d_b2: f64 = (s.db[339][2] + (s.db[337][2] * ddt_scale));let eq8_e150_d_b3: f64 = (s.db[339][3] + (s.db[337][3] * ddt_scale));let eq8_e151: f64 = (1e-12 * eq8_e150);let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e153;let eq8_node_derivatives: [f64; 6] = [eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5];let eq8_branch_derivatives: [f64; 4] = [eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e158,) = {
    if (!s.b[958]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e158;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
        let (eq10_e163,) = {
    if (!s.b[958]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e163;
        stamper.stamp_potential_const_local(
            2,
            eq10_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3,) = {
    if s.b[959] {
        let eq11_e168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[344]);let eq11_e169: f64 = (s.v[345] + eq11_e168);let eq11_e169_d_n0: f64 = (s.dn[345][0] + (s.dn[344][0] * ddt_scale));let eq11_e169_d_n1: f64 = (s.dn[345][1] + (s.dn[344][1] * ddt_scale));let eq11_e169_d_n2: f64 = (s.dn[345][2] + (s.dn[344][2] * ddt_scale));let eq11_e169_d_n3: f64 = (s.dn[345][3] + (s.dn[344][3] * ddt_scale));let eq11_e169_d_n4: f64 = (s.dn[345][4] + (s.dn[344][4] * ddt_scale));let eq11_e169_d_n5: f64 = (s.dn[345][5] + (s.dn[344][5] * ddt_scale));let eq11_e169_d_b0: f64 = (s.db[345][0] + (s.db[344][0] * ddt_scale));let eq11_e169_d_b1: f64 = (s.db[345][1] + (s.db[344][1] * ddt_scale));let eq11_e169_d_b2: f64 = (s.db[345][2] + (s.db[344][2] * ddt_scale));let eq11_e169_d_b3: f64 = (s.db[345][3] + (s.db[344][3] * ddt_scale));let eq11_e170: f64 = (1e-13 * eq11_e169);let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e172;let eq11_node_derivatives: [f64; 6] = [eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5];let eq11_branch_derivatives: [f64; 4] = [eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e177,) = {
    if (!s.b[959]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e177;
        stamper.stamp_potential_const_local(
            3,
            eq12_value,
        );let eq13_e179: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[275]);let eq13_value: f64 = eq13_e179;
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq13_value),
            &s.dn[275],
            &s.db[275],
            (multiplicity) * (ddt_scale),
        );let eq14_e183: f64 = (s.v[274] - s.v[290]);let eq14_e183_d_n0: f64 = (s.dn[274][0] - s.dn[290][0]);let eq14_e183_d_n1: f64 = (s.dn[274][1] - s.dn[290][1]);let eq14_e183_d_n2: f64 = (s.dn[274][2] - s.dn[290][2]);let eq14_e183_d_n3: f64 = (s.dn[274][3] - s.dn[290][3]);let eq14_e183_d_n4: f64 = (s.dn[274][4] - s.dn[290][4]);let eq14_e183_d_n5: f64 = (s.dn[274][5] - s.dn[290][5]);let eq14_e183_d_b0: f64 = (s.db[274][0] - s.db[290][0]);let eq14_e183_d_b1: f64 = (s.db[274][1] - s.db[290][1]);let eq14_e183_d_b2: f64 = (s.db[274][2] - s.db[290][2]);let eq14_e183_d_b3: f64 = (s.db[274][3] - s.db[290][3]);let eq14_e184: f64 = (s.v[55] * eq14_e183);let eq14_e184_d_n0: f64 = ((s.dn[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_n0));let eq14_e184_d_n1: f64 = ((s.dn[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_n1));let eq14_e184_d_n2: f64 = ((s.dn[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_n2));let eq14_e184_d_n3: f64 = ((s.dn[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_n3));let eq14_e184_d_n4: f64 = ((s.dn[55][4] * eq14_e183) + (s.v[55] * eq14_e183_d_n4));let eq14_e184_d_n5: f64 = ((s.dn[55][5] * eq14_e183) + (s.v[55] * eq14_e183_d_n5));let eq14_e184_d_b0: f64 = ((s.db[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_b0));let eq14_e184_d_b1: f64 = ((s.db[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_b1));let eq14_e184_d_b2: f64 = ((s.db[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_b2));let eq14_e184_d_b3: f64 = ((s.db[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_b3));let eq14_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq14_e184);let eq14_value: f64 = eq14_e185;let eq14_node_derivatives: [f64; 6] = [(eq14_e184_d_n0 * ddt_scale), (eq14_e184_d_n1 * ddt_scale), (eq14_e184_d_n2 * ddt_scale), (eq14_e184_d_n3 * ddt_scale), (eq14_e184_d_n4 * ddt_scale), (eq14_e184_d_n5 * ddt_scale)];let eq14_branch_derivatives: [f64; 4] = [(eq14_e184_d_b0 * ddt_scale), (eq14_e184_d_b1 * ddt_scale), (eq14_e184_d_b2 * ddt_scale), (eq14_e184_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        multiplicity: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3, eq7_e144_q, eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5, eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3,) = {
    if s.b[958] {
        let eq7_e140_q: f64 = s.v[336];let eq7_e141: f64 = (s.v[338] + s.v[336]);let eq7_e141_d_n0: f64 = (s.dn[338][0] + s.dn[336][0]);let eq7_e141_d_n1: f64 = (s.dn[338][1] + s.dn[336][1]);let eq7_e141_d_n2: f64 = (s.dn[338][2] + s.dn[336][2]);let eq7_e141_d_n3: f64 = (s.dn[338][3] + s.dn[336][3]);let eq7_e141_d_n4: f64 = (s.dn[338][4] + s.dn[336][4]);let eq7_e141_d_n5: f64 = (s.dn[338][5] + s.dn[336][5]);let eq7_e141_d_b0: f64 = (s.db[338][0] + s.db[336][0]);let eq7_e141_d_b1: f64 = (s.db[338][1] + s.db[336][1]);let eq7_e141_d_b2: f64 = (s.db[338][2] + s.db[336][2]);let eq7_e141_d_b3: f64 = (s.db[338][3] + s.db[336][3]);let eq7_e141_q: f64 = eq7_e140_q;let eq7_e142: f64 = (1e-12 * eq7_e141);let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3, eq7_e142_q, (1e-12 * s.dn[336][0]), (1e-12 * s.dn[336][1]), (1e-12 * s.dn[336][2]), (1e-12 * s.dn[336][3]), (1e-12 * s.dn[336][4]), (1e-12 * s.dn[336][5]), (1e-12 * s.db[336][0]), (1e-12 * s.db[336][1]), (1e-12 * s.db[336][2]), (1e-12 * s.db[336][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 6] = [eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5];let eq7_reactive_branch_derivatives: [f64; 4] = [eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            None,
            &eq7_reactive_node_derivatives,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3, eq8_e153_q, eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5, eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3,) = {
    if s.b[958] {
        let eq8_e149_q: f64 = s.v[337];let eq8_e150: f64 = (s.v[339] + s.v[337]);let eq8_e150_d_n0: f64 = (s.dn[339][0] + s.dn[337][0]);let eq8_e150_d_n1: f64 = (s.dn[339][1] + s.dn[337][1]);let eq8_e150_d_n2: f64 = (s.dn[339][2] + s.dn[337][2]);let eq8_e150_d_n3: f64 = (s.dn[339][3] + s.dn[337][3]);let eq8_e150_d_n4: f64 = (s.dn[339][4] + s.dn[337][4]);let eq8_e150_d_n5: f64 = (s.dn[339][5] + s.dn[337][5]);let eq8_e150_d_b0: f64 = (s.db[339][0] + s.db[337][0]);let eq8_e150_d_b1: f64 = (s.db[339][1] + s.db[337][1]);let eq8_e150_d_b2: f64 = (s.db[339][2] + s.db[337][2]);let eq8_e150_d_b3: f64 = (s.db[339][3] + s.db[337][3]);let eq8_e150_q: f64 = eq8_e149_q;let eq8_e151: f64 = (1e-12 * eq8_e150);let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3, eq8_e151_q, (1e-12 * s.dn[337][0]), (1e-12 * s.dn[337][1]), (1e-12 * s.dn[337][2]), (1e-12 * s.dn[337][3]), (1e-12 * s.dn[337][4]), (1e-12 * s.dn[337][5]), (1e-12 * s.db[337][0]), (1e-12 * s.db[337][1]), (1e-12 * s.db[337][2]), (1e-12 * s.db[337][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 6] = [eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5];let eq8_reactive_branch_derivatives: [f64; 4] = [eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(4),
            None,
            &eq8_reactive_node_derivatives,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3, eq11_e172_q, eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5, eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3,) = {
    if s.b[959] {
        let eq11_e168_q: f64 = s.v[344];let eq11_e169: f64 = (s.v[345] + s.v[344]);let eq11_e169_d_n0: f64 = (s.dn[345][0] + s.dn[344][0]);let eq11_e169_d_n1: f64 = (s.dn[345][1] + s.dn[344][1]);let eq11_e169_d_n2: f64 = (s.dn[345][2] + s.dn[344][2]);let eq11_e169_d_n3: f64 = (s.dn[345][3] + s.dn[344][3]);let eq11_e169_d_n4: f64 = (s.dn[345][4] + s.dn[344][4]);let eq11_e169_d_n5: f64 = (s.dn[345][5] + s.dn[344][5]);let eq11_e169_d_b0: f64 = (s.db[345][0] + s.db[344][0]);let eq11_e169_d_b1: f64 = (s.db[345][1] + s.db[344][1]);let eq11_e169_d_b2: f64 = (s.db[345][2] + s.db[344][2]);let eq11_e169_d_b3: f64 = (s.db[345][3] + s.db[344][3]);let eq11_e169_q: f64 = eq11_e168_q;let eq11_e170: f64 = (1e-13 * eq11_e169);let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3, eq11_e170_q, (1e-13 * s.dn[344][0]), (1e-13 * s.dn[344][1]), (1e-13 * s.dn[344][2]), (1e-13 * s.dn[344][3]), (1e-13 * s.dn[344][4]), (1e-13 * s.dn[344][5]), (1e-13 * s.db[344][0]), (1e-13 * s.db[344][1]), (1e-13 * s.db[344][2]), (1e-13 * s.db[344][3]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 6] = [eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5];let eq11_reactive_branch_derivatives: [f64; 4] = [eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            None,
            &eq11_reactive_node_derivatives,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );let eq13_e179_q: f64 = s.v[275];
        stamper.stamp_current_reactive_dense_local(
            Some(0),
            Some(2),
            &s.dn[275],
            &s.db[275],
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        multiplicity: f64,
    ) {
        let eq14_e183: f64 = (s.v[274] - s.v[290]);let eq14_e183_d_n0: f64 = (s.dn[274][0] - s.dn[290][0]);let eq14_e183_d_n1: f64 = (s.dn[274][1] - s.dn[290][1]);let eq14_e183_d_n2: f64 = (s.dn[274][2] - s.dn[290][2]);let eq14_e183_d_n3: f64 = (s.dn[274][3] - s.dn[290][3]);let eq14_e183_d_n4: f64 = (s.dn[274][4] - s.dn[290][4]);let eq14_e183_d_n5: f64 = (s.dn[274][5] - s.dn[290][5]);let eq14_e183_d_b0: f64 = (s.db[274][0] - s.db[290][0]);let eq14_e183_d_b1: f64 = (s.db[274][1] - s.db[290][1]);let eq14_e183_d_b2: f64 = (s.db[274][2] - s.db[290][2]);let eq14_e183_d_b3: f64 = (s.db[274][3] - s.db[290][3]);let eq14_e184: f64 = (s.v[55] * eq14_e183);let eq14_e184_d_n0: f64 = ((s.dn[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_n0));let eq14_e184_d_n1: f64 = ((s.dn[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_n1));let eq14_e184_d_n2: f64 = ((s.dn[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_n2));let eq14_e184_d_n3: f64 = ((s.dn[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_n3));let eq14_e184_d_n4: f64 = ((s.dn[55][4] * eq14_e183) + (s.v[55] * eq14_e183_d_n4));let eq14_e184_d_n5: f64 = ((s.dn[55][5] * eq14_e183) + (s.v[55] * eq14_e183_d_n5));let eq14_e184_d_b0: f64 = ((s.db[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_b0));let eq14_e184_d_b1: f64 = ((s.db[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_b1));let eq14_e184_d_b2: f64 = ((s.db[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_b2));let eq14_e184_d_b3: f64 = ((s.db[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_b3));let eq14_e185_q: f64 = eq14_e184;let eq14_reactive_node_derivatives: [f64; 6] = [eq14_e184_d_n0, eq14_e184_d_n1, eq14_e184_d_n2, eq14_e184_d_n3, eq14_e184_d_n4, eq14_e184_d_n5];let eq14_reactive_branch_derivatives: [f64; 4] = [eq14_e184_d_b0, eq14_e184_d_b1, eq14_e184_d_b2, eq14_e184_d_b3];
        stamper.stamp_current_reactive_dense_local(
            Some(0),
            Some(2),
            &eq14_reactive_node_derivatives,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
