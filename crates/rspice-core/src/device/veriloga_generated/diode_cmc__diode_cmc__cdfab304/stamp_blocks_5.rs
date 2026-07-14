#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) && s.b[924]) {s.store_mul3_ad(229, A::square(A::abs(A::mul(s.ad_value(228), s.ad_value(163)))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))));}
        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) && (!s.b[924])) {s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(163))), s.v[42]);}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) {s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));}
        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && (!s.b[923])) {s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);}
        if ((!s.b[858]) && (!s.b[907])) {s.store_mul_add_scaled_inputs4_indices_rhs(270, 255, 230, 1.0, 231, 1.0, 238, 1.0, 253, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(292, 255, 231, 1.0, 238, 1.0, 253, 1.0, 0.0);}
        s.b[925] = (s.v[258] == 0.0);s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if ((!s.b[858]) && s.b[925]) {s.store_scalar(272, 0.0);s.store_scalar(293, 0.0);s.store_scalar(273, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
    ) {
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
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {s.store_div_scaled_inputs_indices(239, 236, (s.v[124] * s.v[154]), 232, 1.0);s.store_div_from_scalar(240, (0.666666666666667 * s.v[151]), 239);s.store_square(241, 240);s.store_sqrt_div_scaled_square_offset_denominator(242, 241, 1.0, 1.0, 1.0);s.store_sqrt_abs_ad(243, s.ad_value(242));s.store_mul(244, 242, 243);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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
        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) && (!s.b[942])) {s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(164))), s.v[43]);}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) {s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));}
        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);}
        if ((!s.b[858]) && (!s.b[925])) {s.store_mul_add_scaled_inputs4_indices_rhs(272, 255, 230, 1.0, 231, 1.0, 238, 1.0, 253, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(293, 255, 231, 1.0, 238, 1.0, 253, 1.0, 0.0);}
        if (!s.b[858]) {s.store_add_scaled_inputs3_indices(274, 268, s.v[256], 270, s.v[257], 272, s.v[258]);s.store_add_scaled_inputs3_indices(290, 291, s.v[256], 292, s.v[257], 293, s.v[258]);}
        s.store_add_scaled_inputs3_indices(275, 269, s.v[256], 271, s.v[257], 273, s.v[258]);s.store_voltage(284, ctx, nodes, Some(2), Some(1));s.b[945] = (p.p84 > 0.0);s.store_scalar(945, if s.b[945] { 1.0 } else { 0.0 });s.b[946] = (s.v[313] < p.p85);s.store_scalar(946, if s.b[946] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[946]) {s.store_offset_sub_scaled_inputs_indices(349, 277, p.p86, 348, p.p86, s.v[313]);s.store_sub_from_scalar_scaled_input(350, s.v[313], 348, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(349), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (s.b[945] && s.b[946]) {s.store_sqrt_square_add(315, 314, 315);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[945] && s.b[946]) {s.store_offset_add_scaled_inputs_indices(351, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 351, (((-s.v[313])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));}
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
        if (((s.b[945] && s.b[947]) && (!s.b[948])) && (!s.b[949])) {s.store_scaled_softlimit_poly_offset_lhs_ad(354, A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[945] && (!s.b[947])) {s.store_scalar(354, 1.0);}
        s.b[950] = ((p.p91 == 0.0) || (s.v[277] < s.v[347]));s.store_scalar(950, if s.b[950] { 1.0 } else { 0.0 });
        if (s.b[945] && s.b[950]) {s.store_scale(357, 353, p.p90);}
        if (s.b[945] && (!s.b[950])) {s.store_mul_scaled_exp_ad_rhs(357, 353, p.p90, A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(347)), A::sub(s.ad_value(277), s.ad_value(347)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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
        if s.b[945] {s.store_scalar(327, ((-((s.v[307] * s.v[256]) * 1.6021918e-19)) * p.p94));s.store_mul_ad_product_rhs_mixed_ia(328, 323, 340, A::sub(A::exp(A::div_from_scalar((-p.p94), s.ad_value(323))), A::exp_div_scaled_inputs(s.ad_value(346), -1.0, s.ad_value(323), 1.0)));s.store_mul_ad_product_rhs_mixed_ia(329, 323, 341, A::offset(A::exp_div_scaled_inputs(A::sub_from_scalar(p.p94, s.ad_value(346)), -1.0, s.ad_value(323), 1.0), (-1.0)));s.store_add_scaled_inputs3_indices(330, 327, (-1.0), 328, (-1.0), 329, (-1.0));s.store_add(275, 275, 330);s.store_scalar(55, 0.0);}
        if (!s.b[945]) {s.store_mul_sub_rhs(330, 55, 274, 290);}
        s.b[957] = ((s.v[171] > 0.0) && (s.v[171] >= p.p4));s.store_scalar(957, if s.b[957] { 1.0 } else { 0.0 });s.b[958] = ((p.p84 > 0.0) && (p.p92 > 0.0));s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });s.b[959] = ((p.p84 > 0.0) && (p.p95 > 0.0));s.store_scalar(959, if s.b[959] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(0, (8.8541878176e-12 * 11.8));s.store_scalar(1, (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) }));s.b[388] = ((!param_given[6]) && param_given[96]);s.store_scalar(388, if s.b[388] { 1.0 } else { 0.0 });
        if s.b[388] {s.store_scalar(1, (if (p.p96 > (-250.0)) { p.p96 } else { (-250.0) }));}
        s.store_scalar(2, (if (p.p5 > 1e-12) { p.p5 } else { 1e-12 }));s.store_scalar(3, (if (p.p8 > 1e-12) { p.p8 } else { 1e-12 }));s.store_scalar(4, (if (p.p9 > 1e-18) { p.p9 } else { 1e-18 }));s.store_scalar(5, (if (p.p10 > 1e-18) { p.p10 } else { 1e-18 }));s.store_scalar(6, (if (p.p11 > 0.05) { p.p11 } else { 0.05 }));s.store_scalar(7, (if (p.p12 > 0.05) { p.p12 } else { 0.05 }));s.store_scalar(8, (if (p.p13 > 0.05) { p.p13 } else { 0.05 }));s.store_scalar(9, (if (p.p14 > 0.05) { (if (p.p14 < 0.95) { p.p14 } else { 0.95 }) } else { 0.05 }));s.store_scalar(10, (if (p.p15 > 0.05) { (if (p.p15 < 0.95) { p.p15 } else { 0.95 }) } else { 0.05 }));s.store_scalar(11, (if (p.p16 > 0.05) { (if (p.p16 < 0.95) { p.p16 } else { 0.95 }) } else { 0.05 }));s.store_scalar(12, p.p17);s.store_scalar(13, p.p18);s.store_scalar(14, p.p19);s.store_scalar(15, (if (p.p20 > 0.0) { p.p20 } else { 0.0 }));s.store_scalar(16, (if (p.p21 > 0.0) { p.p21 } else { 0.0 }));s.store_scalar(17, (if (p.p22 > 0.0) { p.p22 } else { 0.0 }));s.store_scalar(20, (if (p.p23 > 0.0) { p.p23 } else { 0.0 }));s.store_scalar(21, (if (p.p24 > 0.0) { p.p24 } else { 0.0 }));s.store_scalar(22, (if (p.p25 > 0.0) { p.p25 } else { 0.0 }));s.store_scalar(18, (if (p.p26 > 1e-9) { p.p26 } else { 1e-9 }));s.store_scalar(19, (if (p.p27 > 1e-9) { p.p27 } else { 1e-9 }));s.store_scalar(23, (if (p.p28 > 0.0) { p.p28 } else { 0.0 }));s.store_scalar(24, (if (p.p29 > 0.0) { p.p29 } else { 0.0 }));s.store_scalar(25, (if (p.p30 > 0.0) { p.p30 } else { 0.0 }));s.store_scalar(26, (if (p.p31 > 0.01) { p.p31 } else { 0.01 }));s.store_scalar(27, (if (p.p32 > 0.01) { p.p32 } else { 0.01 }));s.store_scalar(28, (if (p.p33 > 0.01) { p.p33 } else { 0.01 }));s.store_scalar(29, (if (p.p34 > 0.0) { p.p34 } else { 0.0 }));s.store_scalar(30, (if (p.p35 > 0.0) { p.p35 } else { 0.0 }));s.store_scalar(31, (if (p.p36 > 0.0) { p.p36 } else { 0.0 }));s.store_scalar(32, p.p37);s.store_scalar(33, p.p38);s.store_scalar(34, p.p39);s.store_scalar(35, p.p40);s.store_scalar(36, p.p41);s.store_scalar(37, p.p42);s.store_scalar(38, (if (p.p43 > 0.1) { p.p43 } else { 0.1 }));s.store_scalar(39, (if (p.p44 > 0.1) { p.p44 } else { 0.1 }));s.store_scalar(40, (if (p.p45 > 0.1) { p.p45 } else { 0.1 }));s.store_scalar(41, (if (p.p46 > 0.1) { p.p46 } else { 0.1 }));s.store_scalar(42, (if (p.p47 > 0.1) { p.p47 } else { 0.1 }));s.store_scalar(43, (if (p.p48 > 0.1) { p.p48 } else { 0.1 }));s.store_scalar(44, p.p7);s.store_scalar(55, (if (p.p56 > 0.0) { p.p56 } else { 0.0 }));s.store_scalar(56, p.p57);s.store_scalar(57, p.p58);s.store_scalar(58, p.p59);s.store_scalar(59, p.p60);s.store_scalar(60, p.p61);s.store_scalar(61, p.p62);s.store_scalar(62, (if (p.p63 > 0.1) { p.p63 } else { 0.1 }));s.store_scalar(64, (if (p.p64 > 0.1) { p.p64 } else { 0.1 }));s.store_scalar(63, (if (p.p65 > 0.1) { p.p65 } else { 0.1 }));s.store_scalar(75, (if (p.p76 > 0.1) { p.p76 } else { 0.1 }));s.store_scalar(76, (if (p.p77 > 0.0) { p.p77 } else { 0.0 }));s.store_scalar(77, (if (p.p78 > 0.0) { p.p78 } else { 0.0 }));s.store_scalar(45, 0.0);s.b[389] = (p.p81 > 0.5);s.store_scalar(389, if s.b[389] { 1.0 } else { 0.0 });
        if s.b[389] {s.store_scalar(45, 1.0);}
        if (!s.b[389]) {s.store_scalar(45, 0.0);}
        s.store_scalar(46, (if (p.p82 > 0.5) { p.p82 } else { 0.5 }));s.store_scalar(47, (if (p.p83 > 0.0) { p.p83 } else { 0.0 }));s.store_primal_offset(78, 1, 273.15);s.store_scalar(79, ((ctx_temp + p.p102)).max((273.15 + (-250.0))));s.store_primal_div_from_scalar(80, s.v[79], 78);s.store_scalar(81, (1.3806505e-23 / 1.6021918e-19));s.store_primal_scale(82, 78, s.v[81]);s.store_primal_div_from_scalar(83, 1.0, 82);s.store_scalar(84, (s.v[81] * s.v[79]));s.store_scalar(85, (1.0 / s.v[84]));s.store_primal_div_scaled_inputs(89, A::mul_scaled_lhs(s.ad_value(78), 0.000702, s.ad_value(78)), -1.0, A::offset(s.ad_value(78), 1108.0), 1.0);s.store_primal_offset(92, 89, s.v[12]);s.store_primal_offset(93, 89, s.v[13]);s.store_primal_offset(94, 89, s.v[14]);s.store_scalar(90, ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79])));s.store_scalar(95, (s.v[12] + s.v[90]));s.store_scalar(96, (s.v[13] + s.v[90]));s.store_scalar(97, (s.v[14] + s.v[90]));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
    ) {
        s.store_primal_mul_powf_mixed_ai(98, A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), 0.5), 80, (s.v[75] / 2.0));s.store_primal_mul_powf_mixed_ai(99, A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), 0.5), 80, (s.v[75] / 2.0));s.store_primal_mul_powf_mixed_ai(100, A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), 0.5), 80, (s.v[75] / 2.0));s.store_primal_mul_powf_mixed_ai(176, A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62]))), 80, ((s.v[75] / 2.0) / s.v[62]));s.store_primal_mul_powf_mixed_ai(177, A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64]))), 80, ((s.v[75] / 2.0) / s.v[64]));s.store_primal_mul_powf_mixed_ai(178, A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63]))), 80, ((s.v[75] / 2.0) / s.v[63]));s.store_primal_scaled_mul(101, 176, 176, s.v[15]);s.store_primal_scaled_mul(102, 177, 177, s.v[16]);s.store_primal_scaled_mul(103, 178, 178, s.v[17]);s.store_primal_sub_scaled_inputs_ln_rhs(104, 80, s.v[6], 98, (2.0 * s.v[84]));s.store_primal_sub_scaled_inputs_ln_rhs(105, 80, s.v[7], 99, (2.0 * s.v[84]));s.store_primal_sub_scaled_inputs_ln_rhs(106, 80, s.v[8], 100, (2.0 * s.v[84]));s.store_primal_add_scaled_inputs_mixed_ia(107, 104, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(104), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]);s.store_primal_add_scaled_inputs_mixed_ia(108, 105, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(105), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]);s.store_primal_add_scaled_inputs_mixed_ia(109, 106, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(106), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]);s.store_primal_div_from_scalar(119, 1.0, 107);s.store_primal_div_from_scalar(120, 1.0, 108);s.store_primal_div_from_scalar(121, 1.0, 109);s.store_scalar(122, (1.0 - s.v[9]));s.store_scalar(123, (1.0 - s.v[10]));s.store_scalar(124, (1.0 - s.v[11]));s.store_scalar(125, (1.0 / s.v[122]));s.store_scalar(126, (1.0 / s.v[123]));s.store_scalar(127, (1.0 / s.v[124]));s.store_primal_scaled_powf_ad(128, A::scale(s.ad_value(119), s.v[6]), s.v[9], s.v[3]);s.store_primal_scaled_powf_ad(129, A::scale(s.ad_value(120), s.v[7]), s.v[10], s.v[4]);s.store_primal_scaled_powf_ad(130, A::scale(s.ad_value(121), s.v[8]), s.v[11], s.v[5]);s.store_primal_scaled_mul(131, 128, 107, s.v[125]);s.store_primal_scaled_mul(132, 129, 108, s.v[126]);s.store_primal_scaled_mul(133, 130, 109, s.v[127]);s.store_primal_scale(134, 128, 2.0);s.store_primal_scale(135, 129, 2.0);s.store_primal_scale(136, 130, 2.0);s.store_scalar(137, (s.v[0] / s.v[3]));s.store_scalar(138, ((s.v[18] * s.v[0]) / s.v[4]));s.store_scalar(139, ((s.v[19] * s.v[0]) / s.v[5]));s.store_scalar(140, (1.0 / s.v[137]));s.store_scalar(141, (1.0 / s.v[138]));s.store_scalar(142, (1.0 / s.v[139]));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(143, (1.0 / s.v[6]));s.store_scalar(144, (1.0 / s.v[7]));s.store_scalar(145, (1.0 / s.v[8]));s.store_scalar(86, (1.772453850905516 * 0.29214664));s.store_scalar(87, (((((-5.0) * 0.29214664) + 6.0) - ((s.v[86]) as f64).powi(((-2.0) as i32))) / 3.0));s.store_scalar(88, ((1.0 - 0.29214664) - s.v[87]));s.store_scalar(146, ((0.5 * s.v[95])).max(s.v[84]));s.store_scalar(147, ((0.5 * s.v[96])).max(s.v[84]));s.store_scalar(148, ((0.5 * s.v[97])).max(s.v[84]));s.store_scalar(149, (s.v[146] * s.v[85]));s.store_scalar(150, (s.v[147] * s.v[85]));s.store_scalar(151, (s.v[148] * s.v[85]));s.store_scalar(152, (((((((32.0 * s.v[26]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[146] * s.v[146]) * s.v[146]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(153, (((((((32.0 * s.v[27]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[147] * s.v[147]) * s.v[147]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(154, (((((((32.0 * s.v[28]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[148] * s.v[148]) * s.v[148]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_primal_offset_scaled(155, 78, (((-s.v[35])) * (s.v[32])), ((((((s.v[79]) * (s.v[35]))) + (1.0))) * (s.v[32])));s.store_primal_offset_scaled(156, 78, (((-s.v[36])) * (s.v[33])), ((((((s.v[79]) * (s.v[36]))) + (1.0))) * (s.v[33])));s.store_primal_offset_scaled(157, 78, (((-s.v[37])) * (s.v[34])), ((((((s.v[79]) * (s.v[37]))) + (1.0))) * (s.v[34])));
        if (!(s.v[155] > 0.0)) {s.store_scalar(155, 0.0);}
        if (!(s.v[156] > 0.0)) {s.store_scalar(156, 0.0);}
        if (!(s.v[157] > 0.0)) {s.store_scalar(157, 0.0);}
        s.store_scalar(158, ((s.v[44] - 1.0) / s.v[44]));s.store_scalar(159, (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[41]))));s.store_scalar(160, (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[42]))));s.store_scalar(161, (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[43]))));s.store_primal_scaled_offset_ad(38, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[57], s.v[56], 1.0), 1.0, s.v[38]);s.store_primal_scaled_offset_ad(39, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[59], s.v[58], 1.0), 1.0, s.v[39]);s.store_primal_scaled_offset_ad(40, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[61], s.v[60], 1.0), 1.0, s.v[40]);s.b[390] = (s.v[38] <= 0.1);s.store_scalar(390, if s.b[390] { 1.0 } else { 0.0 });
        if s.b[390] {s.store_scalar(38, 0.1);s.store_scalar(162, 10.0);}
        if (!s.b[390]) {s.store_primal_div_from_scalar(162, 1.0, 38);}
        s.b[391] = (s.v[39] <= 0.1);s.store_scalar(391, if s.b[391] { 1.0 } else { 0.0 });
        if s.b[391] {s.store_scalar(39, 0.1);s.store_scalar(163, 10.0);}
        if (!s.b[391]) {s.store_primal_div_from_scalar(163, 1.0, 39);}
        s.b[392] = (s.v[40] <= 0.1);s.store_scalar(392, if s.b[392] { 1.0 } else { 0.0 });
        if s.b[392] {s.store_scalar(40, 0.1);s.store_scalar(164, 10.0);}
        if (!s.b[392]) {s.store_primal_div_from_scalar(164, 1.0, 40);}
        s.store_scalar(179, (1.0 - (0.01 * s.v[77])));s.store_primal_scale(165, 162, ((-((s.v[159] * s.v[159]) * ((s.v[158]) as f64).powf((s.v[41] - 1.0)))) * s.v[41]));s.store_primal_scale(166, 163, ((-((s.v[160] * s.v[160]) * ((s.v[158]) as f64).powf((s.v[42] - 1.0)))) * s.v[42]));s.store_primal_scale(167, 164, ((-((s.v[161] * s.v[161]) * ((s.v[158]) as f64).powf((s.v[43] - 1.0)))) * s.v[43]));s.store_scalar(308, (p.p87 * 1000000.0));s.store_scalar(310, (p.p89 * 1000000.0));s.store_scalar(309, (p.p88 * 1000000.0));s.store_scalar(307, s.v[308]);s.store_scalar(313, s.v[62]);s.store_scalar(311, (1450.0 * 0.0001));s.store_scalar(312, (500.0 * 0.0001));s.store_scalar(368, 0.6);s.store_scalar(369, 0.001);s.store_primal_scale(318, 176, 1.45e16);s.store_primal_scaled_square(319, 318, 1.0 / (s.v[307]));s.store_primal_powf(316, 80, (-1.5));s.store_primal_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));s.store_primal_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));s.store_primal_div_scaled_product_add_scaled_denominator_indices(322, 320, 321, 2.0, 320, 1.0, 321, 1.0, 1.0);s.store_primal_powf(317, 80, p.p97);s.store_primal_scale(324, 317, p.p93);s.store_primal_sqrt_mul(323, 324, 322);s.store_primal_scaled_ln_ad(347, A::div_from_scalar(s.v[307], s.ad_value(319)), (s.v[313] / s.v[85]));s.store_primal_scaled_add_ad(348, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323)), (s.v[313] / s.v[85]));s.store_scalar(256, (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]));s.store_scalar(257, (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]));s.store_scalar(258, (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]));s.store_scalar(263, 0.0);s.store_scalar(281, 0.0);s.store_scalar(282, 0.0);s.store_scalar(283, 0.0);s.b[393] = ((s.v[101] * s.v[256]) > 0.0);s.store_scalar(393, if s.b[393] { 1.0 } else { 0.0 });
        if s.b[393] {s.store_primal_scaled_ln_ad(168, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0), (s.v[84] * s.v[62]));}
        if (!s.b[393]) {s.store_scalar(168, 100000000.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
    ) {
        s.b[394] = ((s.v[102] * s.v[257]) > 0.0);s.store_scalar(394, if s.b[394] { 1.0 } else { 0.0 });
        if s.b[394] {s.store_primal_scaled_ln_ad(169, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0), (s.v[84] * s.v[64]));}
        if (!s.b[394]) {s.store_scalar(169, 100000000.0);}
        s.b[395] = ((s.v[103] * s.v[258]) > 0.0);s.store_scalar(395, if s.b[395] { 1.0 } else { 0.0 });
        if s.b[395] {s.store_primal_scaled_ln_ad(170, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0), (s.v[84] * s.v[63]));}
        if (!s.b[395]) {s.store_scalar(170, 100000000.0);}
        s.store_min3(262, 168, 169, 170);s.b[396] = ((((s.v[262] * s.v[85])) as f64).abs() < 230.25850929940458);s.store_scalar(396, if s.b[396] { 1.0 } else { 0.0 });
        if s.b[396] {s.store_primal_exp_scaled_input(263, 262, s.v[85]);}
        s.b[397] = ((s.v[262] * s.v[85]) < (-230.25850929940458));s.store_scalar(397, if s.b[397] { 1.0 } else { 0.0 });
        if ((!s.b[396]) && s.b[397]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(263, 1e-100, (-230.25850929940458), A::scale(s.ad_value(262), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((!s.b[396]) && (!s.b[397])) {s.store_primal_scaled_offset_ad(263, A::mul_offset_rhs(A::scale_offset(s.ad_value(262), s.v[85], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(262), s.v[85], (-230.25850929940458)), A::scale_offset(s.ad_value(262), ((s.v[85]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        s.copy_ad(110, 107);s.copy_ad(111, 108);s.copy_ad(112, 109);s.store_scalar(113, s.v[9]);s.store_scalar(114, s.v[10]);s.store_scalar(115, s.v[11]);s.store_scalar(116, s.v[6]);s.store_scalar(117, s.v[7]);s.store_scalar(118, s.v[8]);s.b[398] = (s.v[256] == 0.0);s.store_scalar(398, if s.b[398] { 1.0 } else { 0.0 });
        if s.b[398] {s.store_primal_add(110, 108, 109);s.store_scalar(113, (0.9 * (s.v[10]).min(s.v[11])));s.store_scalar(116, (s.v[7] + s.v[8]));}
        s.b[399] = (s.v[257] == 0.0);s.store_scalar(399, if s.b[399] { 1.0 } else { 0.0 });
        if s.b[399] {s.store_primal_add(111, 107, 109);s.store_scalar(114, (0.9 * (s.v[9]).min(s.v[11])));s.store_scalar(117, (s.v[6] + s.v[8]));}
        s.b[400] = (s.v[258] == 0.0);s.store_scalar(400, if s.b[400] { 1.0 } else { 0.0 });
        if s.b[400] {s.store_primal_add(112, 107, 108);s.store_scalar(115, (0.9 * (s.v[9]).min(s.v[10])));s.store_scalar(118, (s.v[6] + s.v[7]));}
        s.store_min3(264, 110, 111, 112);s.store_primal_scale(265, 264, 0.1);s.store_max3(91, 113, 114, 115);s.store_primal_mul_scale_offset_mixed_ia(266, 264, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(91))), -1.0, 1.0);s.store_primal_offset_min_ad(267, A::min(s.ad_value(116), s.ad_value(117)), s.ad_value(118), (-0.05));s.store_primal_add_scaled_inputs3_indices(289, 101, s.v[256], 102, s.v[257], 103, s.v[258]);s.store_scalar(300, 0.0);s.store_scalar(301, 1.0);s.store_scalar(303, 1.0);s.store_scalar(302, 0.0);s.store_scalar(305, 1.0);s.store_scalar(304, 0.0);s.store_scalar(306, 0.0);s.store_scalar(294, 0.0);s.store_scalar(295, 0.0);s.store_scalar(296, 0.0);s.store_scalar(297, 0.0);s.store_scalar(298, 0.0);s.store_scalar(299, 0.0);s.store_scalar(196, 0.0);s.store_scalar(197, 0.0);s.store_scalar(185, 0.0);s.store_scalar(186, 0.0);s.store_scalar(187, 0.0);s.store_scalar(188, 0.0);s.store_scalar(189, 0.0);s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);s.store_scalar(200, 0.0);s.store_scalar(208, 0.0);s.store_scalar(259, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(260, 1.0);s.store_scalar(261, 1.0);s.store_scalar(195, 0.0);s.store_scalar(203, 0.0);s.store_scalar(204, 0.0);s.store_scalar(370, 0.0);s.store_scalar(372, 0.0);s.store_scalar(371, 0.0);s.store_scalar(345, 0.0);s.store_scalar(338, 0.0);s.store_scalar(339, 0.0);s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);s.store_scalar(344, 0.0);s.store_scalar(333, (1.6021918e-19 * s.v[256]));s.store_scalar(343, ((((2.0 * s.v[0]) / (1.6021918e-19 * s.v[307]))) as f64).sqrt());s.store_scalar(314, ((p.p94 - s.v[343]) - 1e-7));s.store_scalar(315, ((4.0 * p.p94) * 1e-7));
        if (!(s.v[315] > 0.0)) {s.store_scalar(315, (-s.v[315]));}
        s.store_sqrt_offset_input(315, 315, (s.v[314] * s.v[314]));s.store_sub_from_scalar_ad(343, p.p94, A::scaled_offset(s.ad_value(315), s.v[314], 0.5));s.b[413] = (s.v[45] > 0.9);s.store_scalar(413, if s.b[413] { 1.0 } else { 0.0 });s.b[414] = ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0)));s.store_scalar(414, if s.b[414] { 1.0 } else { 0.0 });
        if (s.b[413] && s.b[414]) {s.store_scalar(45, 0.0);}
        s.b[415] = (s.v[256] > 0.0);s.store_scalar(415, if s.b[415] { 1.0 } else { 0.0 });
        if ((s.b[413] && (!s.b[414])) && s.b[415]) {s.store_scalar(301, s.v[62]);}
        s.b[416] = (s.v[258] > 0.0);s.store_scalar(416, if s.b[416] { 1.0 } else { 0.0 });
        if ((s.b[413] && (!s.b[414])) && s.b[416]) {s.store_scalar(301, s.v[63]);}
        s.b[417] = (s.v[257] > 0.0);s.store_scalar(417, if s.b[417] { 1.0 } else { 0.0 });
        if ((s.b[413] && (!s.b[414])) && s.b[417]) {s.store_scalar(301, s.v[64]);}
        s.b[418] = (s.v[45] == 1.0);s.store_scalar(418, if s.b[418] { 1.0 } else { 0.0 });
        if s.b[418] {s.store_scalar(419, 0.0);s.store_scalar(420, 0.0);s.store_scalar(421, 0.0);s.store_scalar(422, 0.0);s.store_scalar(423, 0.0);s.store_scalar(424, 0.0);s.store_scalar(425, 0.0);s.store_scalar(426, 0.0);s.store_scalar(427, 0.0);s.store_scalar(277, 0.0);s.store_scalar(428, 0.0);s.store_scalar(429, 0.0);s.store_scalar(430, 0.0);s.store_scalar(431, 0.0);s.store_scalar(432, 0.0);s.store_scalar(433, 0.0);s.store_scalar(434, 0.0);s.store_scalar(435, 0.0);s.store_scalar(436, 0.0);s.store_scalar(437, 0.0);s.store_scalar(438, 0.0);s.store_scalar(439, 0.0);s.store_scalar(440, 0.0);s.store_scalar(441, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);s.store_scalar(447, 0.0);s.store_scalar(448, 0.0);s.store_scalar(449, 0.0);s.store_scalar(450, 0.0);s.store_scalar(451, 0.0);s.store_scalar(452, 0.0);s.store_scalar(453, 0.0);s.store_scalar(454, 0.0);s.store_scalar(455, 0.0);s.store_scalar(456, 0.0);s.store_scalar(457, 0.0);s.store_scalar(458, 0.0);s.store_scalar(459, 0.0);s.store_scalar(460, 0.0);s.store_scalar(461, 0.0);s.store_scalar(462, 0.0);s.store_scalar(205, 0.4);s.store_scalar(206, 0.65);s.store_scalar(207, 0.8);s.store_primal_scale(190, 205, (-s.v[46]));s.store_primal_scale(191, 206, (-s.v[46]));s.store_primal_scale(192, 207, (-s.v[46]));s.store_scalar(193, 0.1);s.store_scalar(194, 0.2);}
        s.b[463] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[463]) {s.store_primal_scaled_mul(422, 265, 265, 4.0);s.store_primal_div(423, 265, 266);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[418] && s.b[463]) {s.store_primal_add_scaled_product_indices(424, 190, 1.0, 265, 423, 1.0);s.store_primal_add(425, 266, 424);s.store_primal_sub(426, 266, 424);s.store_primal_sqrt_square_add(427, 426, 422);s.store_primal_div_scaled_product_add_scaled_denominator_indices(428, 190, 266, 2.0, 425, 1.0, 427, 1.0, 1.0);}
        s.b[464] = (s.v[190] < s.v[262]);s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });s.b[465] = ((((0.5 * (s.v[190] * s.v[85]))) as f64).abs() < 230.25850929940458);s.store_scalar(465, if s.b[465] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[465]) {s.store_primal_exp_scaled_input(430, 190, (s.v[85] * 0.5));}
        s.b[466] = ((0.5 * (s.v[190] * s.v[85])) < (-230.25850929940458));s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && s.b[466]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(430, 1e-100, (-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && (!s.b[466])) {s.store_primal_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(190), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(190), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(190), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && s.b[464]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[467] = (s.v[62] < p.p85);s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);}
        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[467])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);}
        s.b[468] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[468]) {s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[469] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(469, if s.b[469] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && s.b[469]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && (!s.b[469])) {s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && s.b[464]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[470] = (s.v[64] < p.p85);s.store_scalar(470, if s.b[470] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);}
        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[470])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);}
        s.b[471] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(471, if s.b[471] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[471]) {s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[472] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(472, if s.b[472] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && s.b[472]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(371, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && (!s.b[472])) {s.store_scaled_softlimit_poly_offset_lhs_ad(371, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && s.b[464]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[473] = (s.v[63] < p.p85);s.store_scalar(473, if s.b[473] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);}
        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[473])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);}
        s.b[474] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(474, if s.b[474] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[474]) {s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[475] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(475, if s.b[475] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && s.b[475]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(372, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && (!s.b[475])) {s.store_scaled_softlimit_poly_offset_lhs_ad(372, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_primal_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(190), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[476] = (s.v[62] < p.p85);s.store_scalar(476, if s.b[476] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[476])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);s.store_scalar(366, 0.0);}
        s.b[477] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(477, if s.b[477] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[477]) {s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[478] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(478, if s.b[478] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && s.b[478]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && (!s.b[478])) {s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(370, 281, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[479] = (s.v[64] < p.p85);s.store_scalar(479, if s.b[479] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[479])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);s.store_scalar(366, 0.0);}
        s.b[480] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(480, if s.b[480] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[480]) {s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[481] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(481, if s.b[481] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && s.b[481]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(282, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && (!s.b[481])) {s.store_scaled_softlimit_poly_offset_lhs_ad(282, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_mul_scale_offset_mixed_ia(371, 282, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[482] = (s.v[63] < p.p85);s.store_scalar(482, if s.b[482] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[482])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);s.store_scalar(366, 0.0);}
        s.b[483] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(483, if s.b[483] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[483]) {s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[484] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(484, if s.b[484] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && s.b[484]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(283, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && (!s.b[484])) {s.store_scaled_softlimit_poly_offset_lhs_ad(283, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
}
