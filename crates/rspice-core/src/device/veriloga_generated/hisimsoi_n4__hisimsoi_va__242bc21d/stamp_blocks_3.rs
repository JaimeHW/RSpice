#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[733] && (!s.b[925])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }

        s.b[1035] = (s.v[430] == 0.0);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1035]) {
            s.copy_ad(352, 346);
            s.copy_ad(353, 347);
            s.copy_ad(354, 348);
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(162, 352);
            s.copy_ad(157, 453);
        }

        s.b[1036] = (s.v[349] < 0.0);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1036]) {
            s.store_scalar(145, 1.0);
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(374, 349);
            s.copy_ad(375, 352);
            s.store_sub(164, 375, 374);
            s.copy_ad(373, 351);
            s.store_scale(400, 401, 9662367879.197212);
            s.store_add_scaled_inputs3_mixed_iia(246, 358, 1.0, 355, (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);
        }

        s.b[1037] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1037]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_sub(411, 352, 349);
            s.store_offset(411, 411, 5e-12);
            s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);
            s.store_div_scaled_inputs2_mixed_aai(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), 410, 1.0);
        }

        s.b[1038] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 409, -1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1039] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        s.b[1040] = (2.0 == 1.0);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && s.b[1040]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1041] = (2.0 == 2.0);
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && s.b[1041]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1042] = (2.0 == 4.0);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && s.b[1042]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1043] = (2.0 == 8.0);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13380_loop_guard: usize = 0;
        while {
            let assign13380_cond_e19017: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13380_cond_e19017 != 0.0
        } {
            assign13380_loop_guard += 1;
            assert!(assign13380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1038]) && (!s.b[1039])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[733] && (!s.b[925])) && (!s.b[1038])) {
            s.store_neg(328, 409);
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_neg(409, 328);
        }

        s.b[1044] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1044]) {
            s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_neg_ad(414, A::sub(s.ad_value(358), s.ad_value(355)));
        }

        s.b[1045] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 414, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1046] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        s.b[1047] = (2.0 == 1.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && s.b[1047]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1048] = (2.0 == 2.0);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1049] = (2.0 == 4.0);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && s.b[1049]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1050] = (2.0 == 8.0);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && (!s.b[1049])) && s.b[1050]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13740_loop_guard: usize = 0;
        while {
            let assign13740_cond_e19443: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13740_cond_e19443 != 0.0
        } {
            assign13740_loop_guard += 1;
            assert!(assign13740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1045]) && (!s.b[1046])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[733] && (!s.b[925])) && (!s.b[1045])) {
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_offset_div_scaled_inputs_mixed_ia(412, 414, (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);
            s.store_mul_ad_product_lhs(328, A::square(s.ad_value(411)), s.ad_value(411), 411);
            s.store_mul(415, 412, 411);
            s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);
        }

        s.b[1051] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
            s.store_sub_from_scalar(44, 1e-5, 413);
            s.store_square(49, 44);
            s.store_scalar(50, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1052] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        s.b[1053] = (2.0 == 1.0);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && s.b[1053]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1054] = (2.0 == 2.0);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && s.b[1054]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1055] = (2.0 == 4.0);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && (!s.b[1054])) && s.b[1055]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1056] = (2.0 == 8.0);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && (!s.b[1054])) && (!s.b[1055])) && s.b[1056]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign14100_loop_guard: usize = 0;
        while {
            let assign14100_cond_e19872: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign14100_cond_e19872 != 0.0
        } {
            assign14100_loop_guard += 1;
            assert!(assign14100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1051]) && (!s.b[1052])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 1e-5);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
            s.store_sub_from_scalar(413, 1e-5, 43);
        }

        if ((s.b[733] && (!s.b[925])) && (!s.b[1051])) {
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(190, 413);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if (s.b[733] && (!s.b[925])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        if (!s.b[733]) {
            s.copy_ad(515, 154);
        }

        s.b[1063] = (s.v[416] < p.p237);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1063]) {
            s.store_scalar(339, 1.0);
        }

        if ((!s.b[733]) && (!s.b[1063])) {
            s.store_scalar(339, 2.0);
        }

        if (!s.b[733]) {
            s.store_add_scaled_inputs3_offset_indices(160, 185, (-1.0), 320, 1.0, 515, 1.0, s.v[123]);
        }

        s.b[1064] = (s.v[158] < s.v[160]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1064]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));
            s.store_mul_sub_rhs(336, 225, 159, 515);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1065] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1064]) && s.b[1065]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((!s.b[733]) && s.b[1064]) && (!s.b[1065])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);
        }

        if ((!s.b[733]) && s.b[1064]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 515, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 515);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_div_lhs_indices(161, 328, 330, 515);
        }

        s.b[1066] = (s.v[144] >= 1.0);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && (!s.b[1064])) && s.b[1066]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(378, s.v[619]);
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            s.store_add_ad_rhs(376, 159, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
            s.store_mul_sub_rhs(181, 225, 376, 515);
        }

        s.b[1067] = (s.v[181] < 3.0);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && s.b[1067]) {
            s.store_mul_sub_rhs(337, 225, 159, 515);
            s.store_div_from_scalar_ad(328, 1.0, A::mul_scaled_lhs(s.ad_value(225), (1.414213562373095 / 108.0), s.ad_value(240)));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 515, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1068] = (s.v[158] <= s.v[182]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && s.b[1068]) {
            s.copy_ad(378, 376);
        }

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul3_lhs(329, 328, 159, 159);
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, s.ad_value(159)));
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            s.store_offset(336, 515, (5e-12 / 2.0));
        }

        s.b[1069] = (s.v[378] < s.v[336]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && s.b[1069]) {
            s.copy_ad(378, 336);
        }

        if ((!s.b[733]) && (!s.b[1064])) {
            s.copy_ad(161, 378);
            s.copy_ad(163, 376);
        }

        s.b[1070] = ((p.p25 == 1.0) && (p.p26 == 2.0));
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1070]) {
            s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
        }

        if ((!s.b[733]) && (!s.b[1070])) {
            s.store_scalar(393, 0.0);
        }

        if (!s.b[733]) {
            s.store_exp_mul(486, 225, 515);
            s.store_mul(487, 379, 486);
            s.store_scalar(430, 0.0);
            s.copy_ad(349, 161);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_ad(327, A::mul_scaled_lhs(s.ad_value(225), 2.0, s.ad_value(419)));
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign15030_loop_guard: usize = 0;
        while {
            let assign15030_cond_e21040: f64 = (s.v[57] + 1.0);
            let assign15030_cond_e21042: f64 = if ((!s.b[733]) && (s.v[167] <= assign15030_cond_e21040)) { 1.0 } else { 0.0 };
            assign15030_cond_e21042 != 0.0
        } {
            assign15030_loop_guard += 1;
            assert!(assign15030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[733]) {
                s.store_sub(417, 349, 515);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1071] = (s.v[337] < 80.0);
            s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1071]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if ((!s.b[733]) && (!s.b[1071])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if (!s.b[733]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1072] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1072]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1073] = (s.v[181] < 0.0);
            s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && s.b[1072]) && s.b[1073]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1074] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1072])) && s.b[1074]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if (((!s.b[733]) && (!s.b[1072])) && (!s.b[1074])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1075] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1075]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1076] = (s.v[338] == (-1.0));
            s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1076]) {
                s.store_scalar(401, 0.0);
            }
            if ((!s.b[733]) && (!s.b[1076])) {
                s.store_mul(401, 444, 242);
            }
            s.b[1077] = (s.v[401] < (p.p237 * 1.01));
            s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1077]) {
                s.store_scalar(339, 1.0);
            }
            if ((!s.b[733]) && (!s.b[1077])) {
                s.store_scalar(339, 2.0);
            }
            if (!s.b[733]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1078] = (s.v[181] < 0.0);
            s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1078]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1079] = (s.v[181] < 1e-7);
            s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1078])) && s.b[1079]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            s.b[1080] = (s.v[181] < 80.0);
            s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };
            if ((((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) && s.b[1080]) {
                s.store_exp(243, 181);
                s.store_mul_sub_ad_rhs(488, 487, s.ad_value(243), A::offset(s.ad_value(181), 1.0));
                s.store_mul_ad_product_rhs(489, 487, s.ad_value(225), A::offset(s.ad_value(243), (-1.0)));
            }
            if ((((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) && (!s.b[1080])) {
                s.store_exp_mul(485, 225, 349);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(485), s.ad_value(486)));
            }
            if (((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) {
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if (!s.b[733]) {
                s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1081] = (s.v[430] == 1.0);
            s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1081]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((!s.b[733]) && (!s.b[1081])) {
                s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);
            }
            if ((!s.b[733]) && (!s.b[1081])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1082] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1081])) && s.b[1082]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((!s.b[733]) && (!s.b[1081])) {
                s.store_add(349, 349, 494);
            }
            s.b[1083] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1081])) && s.b[1083]) {
                s.store_scalar(430, 1.0);
            }
            if (!s.b[733]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (!s.b[733]) {
            s.store_offset(167, 167, (-1.0));
            s.copy_ad(371, 370);
            s.copy_ad(356, 371);
            s.copy_ad(161, 349);
            s.store_div(568, 371, 238);
            s.store_offset_square(169, 568, (10.0 * 2.220446049250313e-16));
            s.store_scale(328, 568, 2.0);
            s.store_offset(170, 568, (10.0 * 2.220446049250313e-16));
            s.store_mul(245, 238, 170);
            s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(490), s.ad_value(170));
            s.store_mul3_lhs(244, 238, 488, 328);
            s.store_neg(355, 244);
            s.store_mul(192, 244, 324);
        }

        s.b[1084] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1084]) {
            s.store_scalar(338, 4.0);
            s.store_scalar(145, 1.0);
            s.store_sub(329, 159, 161);
            s.store_mul(437, 323, 329);
            s.store_scale(327, 108, (-s.v[98]));
            s.store_mul(196, 327, 437);
            s.store_scalar(197, 0.0);
            s.store_scalar(198, 0.0);
            s.store_mul_neg_lhs(329, 534, 437);
            s.store_scale(468, 329, s.v[438]);
            s.store_sub(467, 329, 468);
            s.store_scalar(470, 0.0);
            s.store_scalar(469, 0.0);
            s.store_scalar(199, 0.0);
            s.store_scalar(192, 0.0);
            s.store_scalar(145, 1.0);
            s.copy_ad(352, 349);
            s.copy_ad(162, 161);
            s.copy_ad(314, 162);
            s.store_scalar(612, 1.0);
        }

        s.b[1085] = (s.v[612] == 0.0);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1085]) {
            s.copy_ad(453, 157);
            s.store_scalar(1092, 1e-50);
            s.store_div_ad_rhs(1087, 545, A::square(s.ad_value(323)));
            s.store_offset_mul_ad(1089, A::div_from_scalar(2.0, s.ad_value(1087)), A::sub(s.ad_value(159), s.ad_value(1092)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1087), 1.0);
        }

        s.b[1093] = ((s.v[1089] < s.v[332]) && (s.v[332] >= 0.0));
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {
            s.store_sub(44, 332, 1089);
            s.store_square(49, 44);
            s.store_square(50, 332);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1094] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        s.b[1095] = (4.0 == 1.0);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && s.b[1095]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1096] = (4.0 == 2.0);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if ((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && s.b[1096]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1097] = (4.0 == 4.0);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && (!s.b[1096])) && s.b[1097]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1098] = (4.0 == 8.0);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if ((((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && (!s.b[1096])) && (!s.b[1097])) && s.b[1098]) {
            s.store_scalar(55, 4.0);
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign15750_loop_guard: usize = 0;
        while {
            let assign15750_cond_e22455: f64 = if (((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign15750_cond_e22455 != 0.0
        } {
            assign15750_loop_guard += 1;
            assert!(assign15750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && (!s.b[1094])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(1089, 332, 43);
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1093])) {
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_sqrt(1088, 1089);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[733]) && s.b[1085]) {
            s.store_add_ad_rhs(1092, 159, A::mul_sub_from_scalar_rhs(s.ad_value(1087), 1.0, s.ad_value(1088)));
            s.store_sqrt_square_offset(44, 1092, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1092, 1092, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1099] = (s.v[1092] < 0.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1085]) && s.b[1099]) {
            s.store_scalar(1092, 0.0);
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_div(1086, 157, 1092);
            s.store_pow_ad(1087, s.ad_value(1086), A::offset(s.ad_value(138), (-1.0)));
            s.store_mul(1091, 1087, 1086);
            s.store_offset(1088, 1091, 1.0);
            s.store_pow_ad(1089, s.ad_value(1088), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));
            s.store_mul(1090, 1089, 1088);
            s.store_div(452, 157, 1090);
            s.copy_ad(157, 452);
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));
        }

        s.b[1100] = (s.v[157] <= 0.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1085]) && s.b[1100]) {
            s.store_scalar(164, 0.0);
            s.copy_ad(162, 161);
            s.store_scalar(430, 0.0);
        }

        s.b[1101] = (s.v[144] >= 1.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1101]) {
            s.store_scalar(352, s.v[622]);
            s.store_sub_from_scalar(165, s.v[622], 161);
        }

        s.b[1102] = (s.v[144] == 0.0);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            if ((s.v[163] - s.v[161]) >= 0.0) {
                s.store_sub(166, 163, 161);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            s.store_offset_sub_scaled_inputs(44, s.ad_value(166), (1.0 + 0.3), s.ad_value(157), 1.0, (-0.03));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));
        }

        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[1103] = (s.v[165] < 0.0);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1103]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1104] = (s.v[165] > s.v[157]);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && (!s.b[1103])) && s.b[1104]) {
            s.copy_ad(165, 157);
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1100])) {
            s.copy_ad(164, 165);
            s.store_add(162, 161, 164);
            s.store_scalar(430, 0.0);
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.copy_ad(352, 162);
            s.store_scalar(168, 1.0);
        }

        let mut assign16210_loop_guard: usize = 0;
        while {
            let assign16210_cond_e22988: f64 = (s.v[58] + 1.0);
            let assign16210_cond_e22990: f64 = if (((!s.b[733]) && s.b[1085]) && (s.v[168] <= assign16210_cond_e22988)) { 1.0 } else { 0.0 };
            assign16210_cond_e22990 != 0.0
        } {
            assign16210_loop_guard += 1;
            assert!(assign16210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[733]) && s.b[1085]) {
                s.store_sub(418, 352, 515);
                s.store_mul(181, 225, 418);
                s.store_mul_sub_rhs(337, 420, 418, 419);
            }
            s.b[1105] = (s.v[337] < 80.0);
            s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && s.b[1085]) && s.b[1105]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1105])) {
                s.store_sub(422, 418, 419);
                s.store_scalar(423, 1.0);
            }
            if ((!s.b[733]) && s.b[1085]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1106] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && s.b[1085]) && s.b[1106]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1107] = (s.v[181] < 0.0);
            s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };
            if ((((!s.b[733]) && s.b[1085]) && s.b[1106]) && s.b[1107]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1108] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1106])) && s.b[1108]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1106])) && (!s.b[1108])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1109] = (s.v[338] == (-1.0));
            s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && s.b[1085]) && s.b[1109]) {
                s.store_scalar(401, 0.0);
            }
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1109])) {
                s.store_mul(401, 444, 242);
            }
            if ((!s.b[733]) && s.b[1085]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1110] = (s.v[181] < 0.0);
            s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && s.b[1085]) && s.b[1110]) {
                s.store_neg(499, 242);
                s.store_neg(500, 443);
            }
            s.b[1111] = (s.v[181] < 1e-7);
            s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1110])) && s.b[1111]) {
                s.copy_ad(499, 242);
                s.copy_ad(500, 443);
            }
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1110])) && (!s.b[1111])) {
                s.store_mul_sub_rhs(501, 225, 352, 157);
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(497, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(498, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(499, 242, 497);
                s.store_div_scaled_add_product(500, s.ad_value(498), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(499), 1.0);
            }
            if ((!s.b[733]) && s.b[1085]) {
                s.store_add_scaled_inputs_products_indices(503, 352, 1.0, 159, (-1.0), 240, 499, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(504, 240, 500, 1.0);
            }
            s.b[1112] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));
            s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && s.b[1085]) && s.b[1112]) {
                s.store_scalar(168, (s.v[58] + 1.0));
            }
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1112])) {
                s.store_div_scaled_inputs_indices(495, 503, -1.0, 504, 1.0);
            }
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1112])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1113] = (((s.v[495]) as f64).abs() > s.v[496]);
            s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1112])) && s.b[1113]) {
                s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1112])) {
                s.store_add(352, 352, 495);
            }
            s.b[1114] = ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8));
            s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1112])) && s.b[1114]) {
                s.store_scalar(430, 1.0);
            }
            if ((!s.b[733]) && s.b[1085]) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_offset(168, 168, (-1.0));
            s.copy_ad(372, 370);
            s.copy_ad(359, 372);
            s.copy_ad(162, 352);
            s.store_div(569, 372, 238);
            s.store_offset(171, 569, (10.0 * 2.220446049250313e-16));
            s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(499), s.ad_value(171));
            s.store_mul3_lhs(358, 238, 497, 328);
            s.store_neg(358, 358);
            s.store_sub(164, 162, 161);
            s.copy_ad(157, 453);
            s.store_div(328, 225, 169);
            s.store_mul(505, 328, 164);
            s.store_offset(506, 505, 1.0);
            s.store_sqrt(507, 506);
            s.store_div_from_scalar_offset_input(508, 1.0, 507, 1.0);
            s.store_div(509, 508, 170);
            s.store_scaled_add(510, 568, 569, 0.5);
            s.store_add_scaled_inputs4_indices(328, 159, 1.0, 227, 1.0, 161, (-(2.0 * 0.5)), 164, (-0.5));
            s.store_sub(329, 509, 510);
            s.store_mul(330, 225, 323);
            s.store_mul(331, 225, 238);
            s.store_add_scaled_products_indices(511, 330, 328, 1.0, 331, 329, 1.0);
            s.store_scaled_add(424, 359, 356, 0.5);
            s.store_scaled_add(425, 358, 355, (-0.5));
            s.store_sub(426, 359, 356);
            s.store_neg_ad(427, A::sub(s.ad_value(358), s.ad_value(355)));
            s.store_square(428, 238);
        }

        s.b[1115] = (s.v[339] <= 1.0);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1085]) && s.b[1115]) {
            s.store_add_scaled_inputs3_mixed_aia(246, A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), 1.0, 427, (-1.0), A::div_scaled_product(A::square(s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), -1.0);
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1115])) {
            s.store_mul(246, 164, 511);
        }

        s.b[1116] = ((s.v[84] >= 1.0) && (s.v[246] < 0.0));
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1085]) && s.b[1116]) {
            s.store_scalar(246, 0.0);
        }

        s.b[1117] = (s.v[339] <= 1.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        s.b[1118] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && s.b[1117]) && s.b[1118]) {
            let assign16580_ad_e24165: A = A::mul(A::mul3(A::add_scaled_inputs_product(s.ad_value(425), 1.0, s.ad_value(424), (-2.0), A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div_scaled_product(s.ad_value(424), s.ad_value(424), 2.0, s.ad_value(428), 1.0)), A::div_scaled_product(s.ad_value(426), s.ad_value(426), 0.1, s.ad_value(428), 1.0)), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426));
            s.store_add_scaled_product_mixed_aia(437, A::div(assign16580_ad_e24165, s.ad_value(428)), 0.16666666666666666, 424, A::sub(A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), s.ad_value(427)), 1.0);
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1117]) && s.b[1118]) {
            s.store_div(437, 437, 246);
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1117]) && (!s.b[1118])) {
            s.copy_ad(437, 424);
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1117])) {
            s.store_scaled_add(437, 359, 356, 0.5);
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_scale(328, 240, 2.0);
            s.store_mul_sub_rhs(512, 328, 510, 170);
            s.store_add(191, 164, 512);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[733]) && s.b[1085]) {
            s.store_div_from_scalar(328, 1.0, 192);
            s.store_mul(329, 191, 328);
            s.store_sub_from_scalar(330, 1.0, 329);
            s.store_sub_from_scalar(336, 1.0, 330);
            s.store_square(49, 336);
            s.store_scalar(50, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1119] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        s.b[1120] = (4.0 == 1.0);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && s.b[1119]) && s.b[1120]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1121] = (4.0 == 2.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && s.b[1121]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1122] = (4.0 == 4.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && (!s.b[1121])) && s.b[1122]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1123] = (4.0 == 8.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (((((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && (!s.b[1121])) && (!s.b[1122])) && s.b[1123]) {
            s.store_scalar(55, 4.0);
        }

        if (((!s.b[733]) && s.b[1085]) && s.b[1119]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign16970_loop_guard: usize = 0;
        while {
            let assign16970_cond_e24532: f64 = if ((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign16970_cond_e24532 != 0.0
        } {
            assign16970_loop_guard += 1;
            assert!(assign16970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[733]) && s.b[1085]) && s.b[1119]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1119])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(337, 336, 53, 1.0);
            s.store_sub_from_scalar(190, 1.0, 337);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if ((!s.b[733]) && s.b[1085]) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_div_scaled_product_indices(328, 192, 478, 0.6666666666666667, 479, 1.0);
        }

        s.b[1124] = (s.v[339] <= 1.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        s.b[1125] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && s.b[1085]) && s.b[1124]) && s.b[1125]) {
            s.store_sub_ad(436, A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));
            s.store_div(436, 436, 246);
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1124]) && (!s.b[1125])) {
            s.copy_ad(436, 425);
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1124])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.b[1129] = (s.v[612] == 0.0);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if s.b[1129] {
            s.store_offset(480, 190, 0.5);
            s.store_mul(481, 479, 478);
            s.store_div_scaled_inputs_indices(482, 480, 0.4, 481, 1.0);
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.b[1130] = (s.v[438] > (0.5 + 1e-8));
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1130]) {
            s.store_scalar(438, 0.5);
        }

        if s.b[1129] {
            s.copy_ad(439, 438);
            s.store_scalar(438, 0.5);
        }

        s.b[1132] = (s.v[145] == 0.0);
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        s.b[1148] = ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16)));
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1132]) && s.b[1148]) {
            s.store_scalar(316, 0.0);
            s.copy_ad(314, 162);
        }

        s.b[1149] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && s.b[1148]) && s.b[1149]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_scalar(1147, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_div_from_scalar(1133, 1.0, 1147);
            s.store_mul(1134, 244, 1133);
            s.store_scale(1135, 1134, p.p191);
            s.store_add_scaled_product_indices(1138, 1135, 1.0, 80, 229, 1.0);
            s.store_div_from_scalar(1134, 1.0, 1138);
            s.store_scale(1137, 1134, 1.034943e-10);
            s.store_scalar(1134, (1.0 - p.p189));
            s.store_add_scaled_inputs_product_indices(314, 157, p.p189, 161, p.p189, 1134, 162, 1.0);
        }

        s.b[1150] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1150]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_sub(1140, 314, 162);
            s.store_sqrt_square_offset(44, 1140, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1139, 1140, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1151] = (s.v[1139] < 0.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1151]) {
            s.store_scalar(1139, 0.0);
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_mul(1136, 225, 244);
            s.store_div_from_scalar(1134, 1.0, 1136);
            s.store_mul(1138, 246, 1134);
        }

        s.b[1152] = (s.v[1138] < s.v[227]);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1152]) {
            s.copy_ad(1138, 227);
        }

        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {
            s.store_scale(1144, 229, 9662367879.197212);
            s.store_scalar(1134, (100000.0 * 10000.0));
            s.store_scalar(1135, (1.0 / s.v[97]));
            s.store_mul_ad_lhs(1146, A::add_scaled_inputs_product(s.ad_value(1138), 2.0, A::mul3_scaled_output(s.ad_value(1144), s.ad_value(1139), s.ad_value(1137), 2.0), 1.0, s.ad_value(1134), s.ad_value(1137), 1.0), 1135);
            s.store_mul(1141, 1146, 1137);
            s.store_add_scaled_product_indices(1145, 1134, 4.0, 1144, 1139, (2.0 * 4.0));
            s.store_mul3_lhs(1142, 1145, 1137, 1137);
            s.store_sqrt_square_add(1143, 1141, 1142);
            s.store_mul_sub_scaled_inputs_rhs(316, 326, s.ad_value(1143), 0.5, s.ad_value(1141), 0.5);
        }

        if (s.b[1129] && s.b[1132]) {
            s.store_scale(316, 316, s.v[127]);
        }

        if s.b[1129] {
            s.store_sub_from_scalar(441, s.v[97], 316);
        }

        s.b[1153] = (s.v[441] < 1e-9);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1153]) {
            s.store_scalar(441, 1e-9);
        }

        if s.b[1129] {
            s.store_scale(328, 108, (-s.v[98]));
            s.store_mul(196, 328, 437);
            s.store_mul(197, 328, 436);
            s.store_mul(198, 197, 438);
        }

        s.b[1154] = (p.p43 == 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1154]) {
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_mul_scale_ad_lhs(392, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 108);
        }

        if s.b[1129] {
            s.store_scaled_sub(1155, 157, 164, 0.5);
            s.store_scale(44, 1155, (2.0 * 1.0 / (p.p227)));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.b[1156] = (s.v[177] < (10.0 * 2.220446049250313e-16));
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1156]) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if s.b[1129] {
            s.store_add(176, 161, 177);
            s.store_scalar(1166, (1.034943e-10 / 100.0));
            s.store_scale(1167, 437, 0.0001);
            s.store_scale(1168, 436, 0.0001);
            s.store_div_from_scalar(1157, p.p92, 1166);
            s.store_div_from_scalar(1158, p.p93, 1166);
            s.store_scalar(1159, p.p94);
            s.store_offset_mul_ad(1160, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1159), 1.0);
            s.store_add_scaled_products_indices(1161, 1157, 1167, 1.0, 1158, 1168, 1.0);
            s.store_div(1162, 1161, 1160);
            s.copy_ad(248, 1162);
            s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));
            s.store_offset_add_scaled_inputs_indices(1159, 248, 0.5, 44, 0.5, (1e-10 * 3000.0));
        }

        s.b[1169] = (s.v[1159] < 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1169]) {
            s.store_scalar(1159, 0.0);
        }

        if s.b[1129] {
            s.store_powf(1161, 1159, (p.p97 - 1.0));
            s.store_mul(1163, 1161, 1159);
            s.store_powf(1164, 1159, (s.v[111] - 1.0));
            s.store_mul(1165, 1164, 1159);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1129] {
            s.store_scale(249, 1168, 6.241449993689894e18);
            s.store_add_scaled_ad_lhs(1157, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p.p96 * 1e-11), p.p95)), 1.0, s.ad_value(543), s.ad_value(1163), 1.0), 1165, 1.0 / (p.p106));
            s.store_div_from_scalar(251, 1.0, 1157);
            s.store_scale(251, 251, 0.0001);
            s.store_mul3_lhs(1170, 225, 244, 441);
            s.store_sqrt_square_offset(44, 1170, ((4.0 * 1e-50) * 1e-50));
            s.store_offset_add_scaled_inputs_indices(1170, 1170, 0.5, 44, 0.5, (1e-10 * 1e-50));
        }

        s.b[1178] = (s.v[1170] < 0.0);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1178]) {
            s.store_scalar(1170, 0.0);
        }

        if s.b[1129] {
            s.store_div_from_scalar(1171, 1.0, 1170);
            s.store_mul(1172, 246, 1171);
            s.store_div_scaled_inputs_indices(1170, 253, 0.2, 251, 1.0);
            s.store_sqrt_square_sum(252, 1172, 1170);
            s.store_mul(1173, 251, 252);
            s.store_div(1171, 1173, 253);
        }

        s.b[1179] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1179]) {
            s.store_scalar(1174, 1.0);
        }

        s.b[1180] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if ((s.b[1129] && (!s.b[1179])) && s.b[1180]) {
            s.copy_ad(1174, 1171);
        }

        if ((s.b[1129] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_powf(1174, 1171, (p.p113 - 1.0));
        }

        if s.b[1129] {
            s.store_mul(1170, 1171, 1174);
            s.store_offset(1175, 1170, 1.0);
        }

        s.b[1181] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1181]) {
            s.store_div_from_scalar(1176, 1.0, 1175);
        }

        s.b[1182] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if ((s.b[1129] && (!s.b[1181])) && s.b[1182]) {
            s.store_div_from_scalar_sqrt_ad(1176, 1.0, s.ad_value(1175));
        }

        if ((s.b[1129] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_powf(1177, 1175, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1176, 1175, 1177);
        }

        if s.b[1129] {
            s.store_mul(250, 251, 1176);
            s.store_div_scaled_product_denominator_ad(264, 107, 227, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0);
            s.store_mul3_lhs(200, 264, 246, 250);
            s.store_scalar(201, 0.0);
        }

        s.b[1192] = ((p.p281 > 0.0) && (p.p244 != 0.0));
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1192]) {
            s.store_scaled_sub(1183, 157, 164, 0.5);
            s.store_scale(44, 1183, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(1189, 0.01, 45);
            s.store_sub_from_scalar_ad(1183, 1.1, A::add(s.ad_value(161), s.ad_value(1189)));
            s.store_sqrt_square_offset(44, 1183, ((4.0 * 0.05) * 0.05));
            s.store_offset_add_scaled_inputs_indices(1191, 1183, 0.5, 44, 0.5, (1e-10 * 0.05));
        }

        s.b[1193] = (s.v[1191] < 0.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1192]) && s.b[1193]) {
            s.store_scalar(1191, 0.0);
        }

        if (s.b[1129] && s.b[1192]) {
            s.store_scale(1184, 225, s.v[116]);
            s.store_mul(1185, 323, 1184);
            s.store_powf(1184, 1191, p.p245);
            s.store_mul(1186, 1185, 1184);
            s.store_offset_scaled(1187, 173, p.p246, 1.0);
            s.store_scalar(1184, s.v[117]);
        }

        s.b[1194] = ((s.v[56] < 3.0) || (p.p43 == 1.0));
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1192]) && s.b[1194]) {
            s.store_add_scaled_inputs3_indices(1188, 161, 1.0, 1189, 1.0, 172, -1.0);
        }

        if ((s.b[1129] && s.b[1192]) && (!s.b[1194])) {
            s.store_add_scaled_inputs3_indices(1188, 161, 1.0, 1189, 1.0, 350, -1.0);
        }

        if (s.b[1129] && s.b[1192]) {
            s.store_add_ad_rhs(1187, 1187, A::mul3(s.ad_value(173), s.ad_value(1184), s.ad_value(1188)));
            s.store_mul(1189, 1186, 1187);
            s.copy_ad(1186, 1189);
        }

        if (s.b[1129] && (!s.b[1192])) {
            s.store_scalar(1186, 0.0);
        }

        s.b[1195] = (p.p248 != 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1195]) {
            s.store_scale(1183, 225, s.v[118]);
            s.store_mul(1191, 323, 1183);
            s.store_mul(1190, 1191, 173);
        }

        if (s.b[1129] && (!s.b[1195])) {
            s.store_scalar(1190, 0.0);
        }

        s.b[1196] = ((s.v[1186] + s.v[1190]) > 0.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1196]) {
            s.store_mul_add_rhs(247, 164, 1186, 1190);
            s.store_mul3_lhs(201, 264, 247, 250);
        }

        if s.b[1129] {
            s.store_add(199, 200, 201);
            s.copy_ad(203, 201);
        }

        s.b[1206] = (p.p33 != 0.0);
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1206]) {
            s.copy_ad(1199, 549);
            s.store_scalar(1200, (s.v[124] - p.p71));
            s.store_div_from_scalar_square_ad(1201, 1.0, s.ad_value(1200));
            s.store_mul_ad_product_lhs(1202, A::mul_sub_from_scalar_lhs_scaled_output(p.p69, s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), s.ad_value(1199), 1201);
            s.store_mul(186, 1202, 235);
            s.store_offset_scaled(1198, 173, p.p155, p.p154);
            s.store_mul(206, 186, 1198);
            s.store_sub_from_scalar_scaled_input(1197, p.p156, 157, p.p157);
            s.store_add_scaled_inputs3_offset_indices(207, 174, 1.0, 1197, 1.0, 206, 1.0, (-s.v[123]));
            s.store_mul3_lhs(210, 205, 324, 324);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_offset_sub_ad(1203, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p.p156)))), s.ad_value(206), 1e-50);
            s.store_offset_sub(1197, 174, 1203, (-0.005));
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_scalar(327, (if (s.v[1203] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_ad(1199, A::add_scaled_square_product(s.ad_value(1197), 1.0, s.ad_value(327), s.ad_value(1203), (4.0 * 0.005)));
            s.store_sub_ad_lhs(1200, A::add_scaled_inputs4_offset(s.ad_value(1203), 1.0, s.ad_value(1197), 0.5, s.ad_value(1199), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p.p156))), 514);
            s.store_offset_mul(1201, 225, 1200, (-1.0));
            s.store_div_from_scalar(1202, 4.0, 212);
            s.store_offset_mul(1198, 1201, 1202, 1.0);
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1197, 1198, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1207] = (s.v[1197] < 0.0);
        s.v[1207] = if s.b[1207] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1207]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_offset_input(213, 1197, 1e-50);
            s.store_add_ad_rhs(215, 207, A::mul_sub_from_scalar_rhs(s.ad_value(211), 1.0, s.ad_value(213)));
            s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));
            s.store_mul_ln_ad_lhs(216, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))), 327);
            s.store_div_scaled_value_offset_denominator(1200, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);
            s.store_offset_sub(217, 216, 215, (-0.002));
            s.store_sqrt_add_scaled_square_input(327, 217, 1.0, 216, (4.0 * 0.002));
            s.store_add_scaled_inputs3_indices(218, 216, 1.0, 217, (-0.5), 327, (-0.5));
            s.store_div_from_scalar(1197, 1.0, 327);
            s.store_mul_exp_ad_rhs(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));
            s.store_add_offset_ad_lhs(1198, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0), 327);
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1197, 1198, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1208] = (s.v[1197] < 0.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1208]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_offset_input(219, 1197, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(1198, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1197, 1198, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1209] = (s.v[1197] < 0.0);
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1209]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_sqrt_offset_input(220, 1197, (10.0 * 2.220446049250313e-16));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_sub(1198, 215, 218);
            s.store_sqrt_square_offset(44, 1198, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1197, 1198, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1210] = (s.v[1197] < 0.0);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1206]) && s.b[1210]) {
            s.store_scalar(1197, 0.0);
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_div_scaled_value_offset_denominator(1204, s.ad_value(157), 1.0, s.ad_value(1197), (10.0 * 2.220446049250313e-16), 1.0);
            s.store_square(49, 1204);
            s.store_scalar(50, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1129] && s.b[1206]) {
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1211] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        s.b[1212] = (4.0 == 1.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if (((s.b[1129] && s.b[1206]) && s.b[1211]) && s.b[1212]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1213] = (4.0 == 2.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if ((((s.b[1129] && s.b[1206]) && s.b[1211]) && (!s.b[1212])) && s.b[1213]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1214] = (4.0 == 4.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (((((s.b[1129] && s.b[1206]) && s.b[1211]) && (!s.b[1212])) && (!s.b[1213])) && s.b[1214]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1215] = (4.0 == 8.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((((((s.b[1129] && s.b[1206]) && s.b[1211]) && (!s.b[1212])) && (!s.b[1213])) && (!s.b[1214])) && s.b[1215]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1129] && s.b[1206]) && s.b[1211]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19450_loop_guard: usize = 0;
        while {
            let assign19450_cond_e26957: f64 = if (((s.b[1129] && s.b[1206]) && s.b[1211]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19450_cond_e26957 != 0.0
        } {
            assign19450_loop_guard += 1;
            assert!(assign19450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1129] && s.b[1206]) && s.b[1211]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1129] && s.b[1206]) && (!s.b[1211])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (s.b[1129] && s.b[1206]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(1205, 1204, 53, 1.0);
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
            s.store_div_scaled_product_left_ad(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1205, 1.0, 441, 1.0);
            s.store_add(199, 199, 222);
        }

        s.b[1216] = ((p.p30 != 0.0) && (p.p32 != 0.0));
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (s.b[1129] && s.b[1216]) {
            s.store_square(294, 192);
            s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);
            s.store_sub(296, 294, 295);
            s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(294, 294, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1217] = (s.v[294] < 0.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1216]) && s.b[1217]) {
            s.store_scalar(294, 0.0);
        }

        if (s.b[1129] && s.b[1216]) {
            s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(296, 296, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1218] = (s.v[296] < 0.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1216]) && s.b[1218]) {
            s.store_scalar(296, 0.0);
        }

        if (s.b[1129] && s.b[1216]) {
            s.store_sub(297, 294, 296);
        }

        s.b[1219] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((s.b[1129] && s.b[1216]) && s.b[1219]) {
            s.store_scalar(146, 0.0);
        }

        if ((s.b[1129] && s.b[1216]) && (!s.b[1219])) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.b[1220] = ((p.p281 > 0.0) && (p.p285 > 0.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if s.b[1220] {
            s.store_scalar(1227, s.v[99]);
            s.store_scalar(1231, p.p237);
            s.store_offset_add_scaled_inputs3_offset_indices(1232, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]), (-p.p286));
            s.store_offset(1233, 182, p.p286);
            s.store_scalar(1235, p.p285);
            s.store_scalar(1234, p.p283);
            s.store_scalar(1225, s.v[70]);
            s.store_mul_ln_ad_rhs(1226, 227, A::div_scaled_product_by_product(s.ad_value(1225), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));
        }

        if s.b[1220] {
            if (p.p43 == 1.0) {
                s.copy_ad(1223, 435);
            } else {
                s.copy_ad(1223, 350);
            }
        }

        if s.b[1220] {
            s.store_sqrt_ad(1228, A::div_scaled_product3(A::sub(s.ad_value(1226), s.ad_value(1223)), s.ad_value(536), s.ad_value(1225), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1225)), 1.0));
            s.store_mul(1222, 1228, 1227);
            s.store_div_scaled_product_add_scaled_denominator_indices(1221, 1222, 1222, (-0.25), 157, 1.0, 1222, 1.0, 1.0);
            s.copy_ad(1247, 1221);
            s.copy_ad(1248, 1233);
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(1232), s.ad_value(1247))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if s.b[1220] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if s.b[1220] {
            s.store_add_ad_rhs(376, 1232, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
        }

        s.b[1249] = (s.v[158] < ((s.v[123] + s.v[1248]) * 0.5));
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1249]) {
            s.store_scalar(144, 0.0);
        }

        s.b[1250] = ((s.v[144] == 0.0) || (1.0 != 0.0));
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1250]) {
            s.store_mul_sub_rhs(181, 225, 376, 1247);
        }

        s.b[1251] = (s.v[181] < 3.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if ((s.b[1220] && s.b[1250]) && s.b[1251]) {
            s.store_mul_sub_rhs(337, 225, 1232, 1247);
            s.store_div_from_scalar_ad(328, 1.0, A::mul_scaled_lhs(s.ad_value(225), (1.414213562373095 / 108.0), s.ad_value(240)));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 1247, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1252] = ((s.v[158] - s.v[383]) <= s.v[1248]);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        s.b[1253] = (p.p43 == 0.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((((s.b[1220] && s.b[1250]) && (!s.b[1251])) && s.b[1252]) && s.b[1253]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1231, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1232), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1232, 331, 323);
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && s.b[1252]) {
            s.copy_ad(378, 376);
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1232), s.ad_value(383)), A::sub(s.ad_value(1232), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1232), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1220] && s.b[1250]) && (!s.b[1251])) && (!s.b[1252])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        s.b[1254] = (p.p43 == 0.0);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        s.b[1255] = ((s.v[158] - s.v[383]) <= s.v[1248]);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if (((s.b[1220] && s.b[1250]) && s.b[1254]) && s.b[1255]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1231, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1232), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1232, 331, 323);
            s.copy_ad(378, 376);
        }

        if (((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1231, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1232), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1232, 331, 323);
            s.copy_ad(378, 376);
        }

        s.b[1256] = ((s.v[1232] - s.v[383]) > 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if ((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1232), s.ad_value(383)), A::sub(s.ad_value(1232), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1232), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
        }

        s.b[1257] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_offset_sub_scaled_inputs(44, s.ad_value(376), 1.0, s.ad_value(377), 0.98, 0.4);
            s.store_square(49, 44);
            s.store_scalar(50, (0.4 * 0.4));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1258] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        s.b[1259] = (2.0 == 1.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && s.b[1259]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1260] = (2.0 == 2.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if ((((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1261] = (2.0 == 4.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1261]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1262] = (2.0 == 8.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if ((((((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1261])) && s.b[1262]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20680_loop_guard: usize = 0;
        while {
            let assign20680_cond_e28529: f64 = if (((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20680_cond_e28529 != 0.0
        } {
            assign20680_loop_guard += 1;
            assert!(assign20680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && s.b[1258]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) && (!s.b[1258])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if (((((s.b[1220] && s.b[1250]) && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && (!s.b[1257])) {
            s.copy_ad(378, 376);
        }

        if s.b[1220] {
            s.store_offset(336, 1247, (5e-12 / 2.0));
        }

        s.b[1263] = (s.v[378] < s.v[336]);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1263]) {
            s.copy_ad(378, 336);
        }

        if s.b[1220] {
            s.copy_ad(1230, 378);
            s.copy_ad(163, 376);
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            if ((s.v[376] - s.v[1230]) >= 0.0) {
                s.store_sub(166, 376, 1230);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p.p287)) + ((-0.03))));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[1264] = (s.v[165] < 0.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((s.b[1220] && (0.0 != 0.0)) && s.b[1264]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1265] = (s.v[165] > s.v[157]);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((s.b[1220] && (0.0 != 0.0)) && (!s.b[1264])) && s.b[1265]) {
            s.copy_ad(165, 157);
        }

        if (s.b[1220] && (0.0 != 0.0)) {
            s.store_add(163, 1230, 165);
        }

        s.b[1266] = (p.p282 == 1.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1266]) {
            s.copy_ad(378, 1230);
            s.copy_ad(1267, 1221);
            s.store_offset_add_scaled_inputs3_offset_indices(160, 185, (-1.0), 320, 1.0, 1267, 1.0, s.v[123], p.p286);
        }

        s.b[1269] = (s.v[158] < s.v[160]);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((s.b[1220] && s.b[1266]) && s.b[1269]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));
            s.store_mul_sub_rhs(336, 225, 1232, 1267);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1270] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (((s.b[1220] && s.b[1266]) && s.b[1269]) && s.b[1270]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((s.b[1220] && s.b[1266]) && s.b[1269]) && (!s.b[1270])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);
        }

        if ((s.b[1220] && s.b[1266]) && s.b[1269]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 1267, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 1267);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_div_lhs_indices(1230, 328, 330, 1267);
        }

        if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
            s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1267), (-p.p287)));
            s.store_scalar(430, 0.0);
            s.copy_ad(1268, 378);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_ad(327, A::mul_scaled_lhs(s.ad_value(225), 2.0, s.ad_value(419)));
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
            s.store_scalar(167, 1.0);
        }

        let mut assign21280_loop_guard: usize = 0;
        while {
            let assign21280_cond_e29259: f64 = (s.v[57] + 1.0);
            let assign21280_cond_e29261: f64 = if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (s.v[167] <= assign21280_cond_e29259)) { 1.0 } else { 0.0 };
            assign21280_cond_e29261 != 0.0
        } {
            assign21280_loop_guard += 1;
            assert!(assign21280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_sub(417, 1268, 1267);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1271] = (s.v[337] < 80.0);
            s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1271]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1271])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_mul(421, 225, 422);
            }
            s.b[1272] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1272]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1273] = (s.v[181] < 0.0);
            s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1272]) && s.b[1273]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1274] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1272])) && s.b[1274]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1272])) && (!s.b[1274])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1275] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1275]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1276] = (s.v[181] < 0.0);
            s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1276]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1277] = (s.v[181] < 1e-7);
            s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1276])) && s.b[1277]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1276])) && (!s.b[1277])) {
                s.store_mul_offset_rhs(501, 225, 1268, (-p.p287));
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_add_scaled_inputs_product_indices(492, 1268, 1.0, 1232, (-1.0), 240, 490, 1.0);
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1278] = (s.v[430] == 1.0);
            s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && s.b[1278]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) {
                s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1268]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1268))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1279] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) && s.b[1279]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) {
                s.store_add(1268, 1268, 494);
            }
            s.b[1280] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };
            if ((((s.b[1220] && s.b[1266]) && (!s.b[1269])) && (!s.b[1278])) && s.b[1280]) {
                s.store_scalar(430, 1.0);
            }
            if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if ((s.b[1220] && s.b[1266]) && (!s.b[1269])) {
            s.copy_ad(1230, 1268);
        }

        if s.b[1220] {
            s.store_mul_sub_scaled_inputs_rhs(332, 225, s.ad_value(1230), -1.0, s.ad_value(1221), -1.0);
        }

        if s.b[1220] {
            s.store_scalar(1245, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[1220] {
            s.store_mul(1246, 1245, 332);
            s.store_exp(333, 332);
            s.store_sub_offset_lhs(334, 333, (-1.0), 332);
        }

        s.b[1281] = (s.v[332] > 1e-7);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1220] && s.b[1281]) {
            s.store_mul_scaled_sqrt_rhs(437, 238, -1.0, 334);
        }

        s.b[1282] = (s.v[1246] > 1e-7);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((s.b[1220] && (!s.b[1281])) && s.b[1282]) {
            s.store_mul_sqrt_rhs(437, 238, 334);
        }

        if ((s.b[1220] && (!s.b[1281])) && (!s.b[1282])) {
            s.store_mul_ad_affine_product_rhs(437, 1245, s.ad_value(1246), A::sqrt_scaled_lhs_product_offset(s.ad_value(1246), 0.3333333333333333, A::scale_offset(s.ad_value(1246), 0.25, 1.0), 1.0), (-0.7071067811865475), 0.0);
        }

        if s.b[1220] {
            s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_add_scaled_inputs_indices(1242, 437, 0.5, 44, 0.5, (1e-10 * 1e-6));
        }

        s.b[1283] = (s.v[1242] < 0.0);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1283]) {
            s.store_scalar(1242, 0.0);
        }

        if s.b[1220] {
            s.store_div_scaled_inputs_indices(1243, 1242, 1.0, 536, 1.6021918e-19);
            s.store_sub(328, 1243, 1234);
            s.store_scale(1244, 1243, 0.01);
            s.store_sqrt_ad(44, A::add_scaled_square_product(s.ad_value(328), 1.0, s.ad_value(1244), s.ad_value(1244), 4.0));
            s.store_add_scaled_inputs3_indices(329, 328, 0.5, 44, 0.5, 1244, 1e-10);
        }

        s.b[1284] = (s.v[329] < 0.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1284]) {
            s.store_scalar(329, 0.0);
        }

        if s.b[1220] {
            s.store_div_scaled_product_by_product(1241, s.ad_value(329), s.ad_value(329), 1.0, s.ad_value(1243), s.ad_value(1243), 1.0);
            s.store_add_scaled_product_left_ad(1224, 1221, 1.0, A::sub(s.ad_value(1230), s.ad_value(1221)), 1241, 1.0);
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1224))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1224), s.ad_value(157)))));
            s.store_sqrt_scaled_input(1237, 1225, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_mul_sqrt_rhs(1238, 1237, 227);
            s.store_mul_sub_rhs(1229, 225, 1224, 1221);
        }

        s.b[1285] = ((s.v[1229] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (s.b[1220] && s.b[1285]) {
            s.store_sub_scaled_inputs(44, 225, 0.2, 1229, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 225, 225, (0.2 * 0.2));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1286] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        s.b[1287] = (1.0 == 1.0);
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (((s.b[1220] && s.b[1285]) && s.b[1286]) && s.b[1287]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1288] = (1.0 == 2.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((((s.b[1220] && s.b[1285]) && s.b[1286]) && (!s.b[1287])) && s.b[1288]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1289] = (1.0 == 4.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (((((s.b[1220] && s.b[1285]) && s.b[1286]) && (!s.b[1287])) && (!s.b[1288])) && s.b[1289]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1290] = (1.0 == 8.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((((((s.b[1220] && s.b[1285]) && s.b[1286]) && (!s.b[1287])) && (!s.b[1288])) && (!s.b[1289])) && s.b[1290]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1220] && s.b[1285]) && s.b[1286]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21810_loop_guard: usize = 0;
        while {
            let assign21810_cond_e30576: f64 = if (((s.b[1220] && s.b[1285]) && s.b[1286]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21810_cond_e30576 != 0.0
        } {
            assign21810_loop_guard += 1;
            assert!(assign21810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1220] && s.b[1285]) && s.b[1286]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1220] && s.b[1285]) && (!s.b[1286])) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if (s.b[1220] && s.b[1285]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);
            s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);
        }

        if (s.b[1220] && (!s.b[1285])) {
            s.copy_ad(328, 1229);
        }

        if s.b[1220] {
            s.store_sqrt_offset_input(1239, 328, (10.0 * 2.220446049250313e-16));
            s.store_mul(1240, 1238, 1239);
            s.store_mul_div_scaled_inputs_rhs(1236, 1240, s.ad_value(227), 2.0, s.ad_value(1227), 1.0);
            s.store_mul_product3_rhs(204, 337, s.ad_value(1236), s.ad_value(1235), s.ad_value(107), 1.0);
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.b[1291] = ((p.p43 == 1.0) || (p.p45 == 1.0));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        s.b[1304] = ((s.v[145] == 1.0) || (p.p25 == 0.0));
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if (s.b[1291] && s.b[1304]) {
            s.store_scalar(263, 0.0);
        }

        s.b[1305] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1291] && (!s.b[1304])) && s.b[1305]) {
            s.store_scalar(263, 0.0);
        }

        if ((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) {
            s.store_offset_add_scaled_inputs3_offset_indices(445, 174, 1.0, 185, 1.0, 320, -1.0, (-s.v[136]), p.p48);
        }

        s.b[1306] = (p.p44 <= 0.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) {
            s.copy_ad(1292, 445);
            s.store_square(1299, 323);
            s.copy_ad(1300, 545);
            s.store_div(1294, 1300, 1299);
            s.store_div_from_scalar(1301, 2.0, 1300);
            s.store_mul(1295, 1301, 1299);
            s.store_add_scaled_inputs_product_indices(1296, 1292, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1296, 1296, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1298, 1295, 1296, 1.0);
            s.store_sqrt_square_offset(44, 1298, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1297, 1298, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1307] = (s.v[1297] < 0.0);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) && s.b[1307]) {
            s.store_scalar(1297, 0.0);
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) {
            s.store_offset(1297, 1297, 1e-50);
            s.store_sqrt(1297, 1297);
            s.store_add_scaled_product_value_ad(1302, A::mul_sub_from_scalar_rhs(s.ad_value(1294), 1.0, s.ad_value(1297)), 1.0, 1292, 137, 1.0);
            s.store_add_scaled_inputs3_mixed_iia(1303, 173, p.p122, 176, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1302)), -1.0);
            s.store_sqrt_square_offset(44, 1303, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1303, 1303, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1308] = (s.v[1303] < 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && s.b[1306]) && s.b[1308]) {
            s.store_scalar(1303, 0.0);
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) {
            s.store_mul(1292, 134, 445);
            s.store_div_ad_rhs(1294, 545, A::square(s.ad_value(323)));
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1295, 2.0, 545, A::square(s.ad_value(323)));
            s.store_add_scaled_inputs_product_indices(1296, 1292, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1296, 1296, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1297, 1295, 1296, 1.0);
            s.store_scaled_offset(1299, 1295, 1.0, 2.0);
        }

        s.b[1309] = ((s.v[1297] < (1e-50 + s.v[1299])) && (s.v[1299] >= 0.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) {
            s.store_sub_offset_lhs(44, 1299, 1e-50, 1297);
            s.store_square(49, 44);
            s.store_square(50, 1299);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1310] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        s.b[1311] = (4.0 == 1.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1312] = (4.0 == 2.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (!s.b[1311])) && s.b[1312]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1313] = (4.0 == 4.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (!s.b[1311])) && (!s.b[1312])) && s.b[1313]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1314] = (4.0 == 8.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (!s.b[1311])) && (!s.b[1312])) && (!s.b[1313])) && s.b[1314]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22600_loop_guard: usize = 0;
        while {
            let assign22600_cond_e31695: f64 = if ((((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22600_cond_e31695 != 0.0
        } {
            assign22600_loop_guard += 1;
            assert!(assign22600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && s.b[1310]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) && (!s.b[1310])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 1299, 53);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1309]) {
            s.store_sub_offset_lhs(1297, 1299, 1e-50, 43);
        }

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && (!s.b[1309])) {
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) {
            if (s.v[1297] <= 0.0) {
                s.store_scalar(1297, 0.0);
            } else {
                s.store_sqrt(1297, 1297);
            }
        }

        if (((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) {
            s.store_add_ad_rhs(1302, 1292, A::mul_sub_from_scalar_rhs(s.ad_value(1294), 1.0, s.ad_value(1297)));
            s.store_div_from_scalar_offset_input(1293, s.v[100], 131, s.v[100]);
            s.store_add_scaled_inputs_product_indices(1303, 173, p.p122, 176, 1.0, 1293, 1302, (-1.0));
            s.store_sqrt_square_offset(44, 1303, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1303, 1303, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1315] = (s.v[1303] < 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) && (!s.b[1306])) && s.b[1315]) {
            s.store_scalar(1303, 0.0);
        }

        if ((s.b[1291] && (!s.b[1304])) && (!s.b[1305])) {
            s.store_offset(1303, 1303, 1e-50);
            s.store_exp_div_scaled_inputs_indices(1293, 133, -1.0, 1303, 1.0);
            s.store_mul_product3_rhs(263, 1293, s.ad_value(132), s.ad_value(1303), s.ad_value(199), 1.0);
        }

        s.b[1316] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if s.b[1316] {
            s.store_scale(1320, 227, 0.0);
            s.store_add_scaled_inputs3_indices(44, 231, 1.0, 1320, (-1.0), 231, (-0.01));
            s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));
        }

        if s.b[1316] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1316] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(1320, 231, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sqrt_ad(1321, A::mul_scaled_lhs(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(227)));
            s.store_mul_sub_rhs(1322, 225, 176, 1320);
        }

        if s.b[1316] {
            if (s.v[1322] > 0.0) {
                s.store_sqrt(1322, 1322);
            } else {
                s.store_neg_ad(1322, A::sqrt_scaled_input(s.ad_value(1322), -1.0));
            }
        }

        if s.b[1316] {
            s.store_sqrt_mul(1323, 225, 176);
            s.store_mul_sub_scaled_inputs_rhs(1324, 1321, s.ad_value(1322), -1.0, s.ad_value(1323), -1.0);
            s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1324), (-(p.p47 * 0.01)));
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if s.b[1316] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1316] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p.p47);
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
        }

        s.b[1338] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        s.b[1339] = (s.v[56] < 3.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1339]) {
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
        }

        if (s.b[1338] && (!s.b[1339])) {
            if (p.p43 == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }

        if (s.b[1338] && (!s.b[1339])) {
            if (p.p43 == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }

        if s.b[1338] {
            s.store_offset_scaled(1325, 185, p.p147, 1.0);
            s.store_scaled_mul(1326, 1325, 263, p.p146);
            s.store_offset_mul_ad(1327, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));
            s.store_sqrt_square_offset(44, 1327, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1327, 1327, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1340] = (s.v[1327] < 0.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1340]) {
            s.store_scalar(1327, 0.0);
        }

        if s.b[1338] {
            s.store_sqrt(1328, 1327);
            s.store_mul(1329, 1327, 1328);
            s.store_offset_mul_ad(1330, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));
            s.store_sqrt_square_offset(44, 1330, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1330, 1330, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1341] = (s.v[1330] < 0.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1341]) {
            s.store_scalar(1330, 0.0);
        }

        if s.b[1338] {
            s.store_sqrt(1331, 1330);
            s.store_mul(1332, 1330, 1331);
            s.store_div_from_scalar(1333, 1.0, 1327);
            s.store_mul3_lhs(328, 225, 1326, 1333);
            s.store_div_from_scalar(1333, 1.0, 1330);
            s.store_mul3_lhs(1334, 225, 1326, 1333);
            s.store_mul_ad_rhs(1335, 238, A::add_scaled_products(s.ad_value(1332), s.ad_value(1334), 1.0, s.ad_value(1329), s.ad_value(328), (-1.0)));
            s.store_mul_add_scaled_products_indices_rhs(1336, 238, 1331, 1334, ((-1.0) * (0.5)), 1328, 328, 0.5);
            s.store_add(1337, 1335, 1336);
            s.store_mul3_lhs(265, 264, 1337, 250);
        }

        s.v[1355] = (s.v[88] * 100.0);

        s.store_scale(1356, 323, 0.0001);

        s.v[1357] = (s.v[97] * 100.0);

        s.store_scale(1358, 107, 100.0);

        s.store_scale(1359, 252, 0.01);

        s.store_scale(1360, 436, 0.0001);

        s.store_scale(1361, 238, 0.0001);

        s.b[1362] = (p.p27 == 0.0);
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        s.b[1363] = (s.v[145] == 0.0);
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && s.b[1363]) {
            s.store_offset_add(1354, 176, 173, (-(10.0 * 2.220446049250313e-16)));
            s.store_add_scaled_inputs4_offset_indices(1344, 174, 1.0, 185, (p.p216 * s.v[1357]), 320, (-(p.p216 * s.v[1357])), 1354, (-p.p215), (-s.v[123]));
            s.store_scalar(1346, (1.0 / s.v[1355]));
            s.store_mul(1345, 1344, 1346);
            s.store_scalar(1346, (1.0 / p.p217));
            s.store_offset_mul(1350, 1359, 1346, 1.0);
            s.store_mul(1353, 1345, 1350);
            s.store_sqrt_square_offset(44, 1353, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1353, 1353, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1364] = (s.v[1353] < 0.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if (((!s.b[1362]) && s.b[1363]) && s.b[1364]) {
            s.store_scalar(1353, 0.0);
        }

        if ((!s.b[1362]) && s.b[1363]) {
            s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1346, 174, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1365] = (s.v[1346] < 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if (((!s.b[1362]) && s.b[1363]) && s.b[1365]) {
            s.store_scalar(1346, 0.0);
        }

        if ((!s.b[1362]) && s.b[1363]) {
            s.store_offset(1346, 1346, (-p.p226));
            s.store_scale(1342, 1346, 10.0);
            s.store_offset_square(1345, 1342, 1.0);
            s.store_sub_from_scalar_ad(1344, 1.0, A::div_from_scalar(1.0, s.ad_value(1345)));
            s.store_mul(1353, 1353, 1344);
            s.store_scale(1343, 1358, s.v[1357]);
            s.store_div_from_scalar_offset_input(1350, p.p219, 1343, p.p219);
            s.store_scalar(1349, p.p218);
            s.store_div_from_scalar_offset_input(1347, 1.0, 1353, 1e-50);
            s.store_scaled_mul(1344, 303, 1347, (-p.p214));
        }

        s.b[1366] = (s.v[1344] < (-34.0));
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if (((!s.b[1362]) && s.b[1363]) && (!s.b[1366])) {
            s.store_exp(1345, 1344);
            s.store_mul_scale_ad_lhs(1346, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 1343);
            s.store_div_from_scalar(1348, 1.0, 1361);
            s.store_sqrt_mul_ad(1349, A::add_scaled_inputs(s.ad_value(1360), 1.0, s.ad_value(1356), 1e-12), s.ad_value(1348));
            s.store_mul3_lhs(1347, 1345, 1346, 1349);
        }

        if (!s.b[1362]) {
            s.store_offset_scaled(1343, 158, (-p.p221), p.p222);
            s.store_exp_scaled_input(1345, 1343, s.v[1355]);
            s.store_scale(1343, 158, (1.0 / (s.v[1355]) * 1.0 / (s.v[1355])));
            s.store_mul(1346, 158, 1343);
            s.store_scale(1347, 1358, (p.p220 / 1000000.0));
            s.store_sub(1344, 158, 157);
            s.store_offset_scaled(1343, 1344, (-p.p221), p.p222);
            s.store_exp_scaled_input(1345, 1343, s.v[1355]);
            s.store_scale(1343, 1344, (1.0 / (s.v[1355]) * 1.0 / (s.v[1355])));
            s.store_mul(1346, 1344, 1343);
            s.store_scale(1347, 1358, (p.p220 / 1000000.0));
            s.store_offset_scaled_sub(1353, 513, 158, 1.0 / (s.v[1355]), ((((s.v[123]) + (p.p225))) * (1.0 / (s.v[1355]))));
            s.store_sqrt_square_offset(44, 1353, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1353, 1353, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1369] = (s.v[1353] < 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && s.b[1369]) {
            s.store_scalar(1353, 0.0);
        }

        if (!s.b[1362]) {
            s.store_offset(1353, 1353, 1e-50);
            s.store_div_from_scalar(1344, (-p.p224), 1353);
        }

        s.b[1370] = (s.v[1344] < (-34.0));
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if ((!s.b[1362]) && (!s.b[1370])) {
            s.store_exp(1345, 1344);
            s.store_scale(1346, 1358, (p.p223 * s.v[1357]));
        }

        s.b[1378] = (p.p28 == 0.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if (!s.b[1378]) {
            s.store_add_scaled_inputs4_offset_indices(1371, 157, p.p209, 158, (-1.0), 187, p.p211, 319, p.p211, (p.p210 * p.p209));
            s.store_scalar(1372, (1.0 / s.v[88]));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1378]) {
            s.store_mul(1373, 1371, 1372);
            s.store_sqrt_square_offset(44, 1373, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(304, 1373, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1379] = (s.v[304] < 0.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && s.b[1379]) {
            s.store_scalar(304, 0.0);
        }

        if (!s.b[1378]) {
            s.store_div_from_scalar_offset_input(1374, 1.0, 304, 1e-50);
            s.store_scaled_mul(1375, 303, 1374, (-p.p208));
        }

        s.b[1380] = (s.v[1375] < (-34.0));
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && (!s.b[1380])) {
            s.store_exp(1371, 1375);
            s.store_mul_scale_ad_lhs(1372, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 107);
        }

        if (!s.b[1378]) {
            s.store_sub(1377, 157, 513);
        }

        s.b[1381] = (s.v[1377] > 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if ((!s.b[1378]) && s.b[1381]) {
            s.store_square(1372, 1377);
            s.store_mul(331, 1372, 1377);
            s.store_offset(1375, 331, p.p212);
        }

        s.b[1389] = (p.p28 == 0.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if (!s.b[1389]) {
            s.store_add_scaled_inputs3_mixed_aii(1382, A::add_scaled_inputs3_offset(s.ad_value(157), (-p.p209), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p.p210) * (p.p209))), 1.0, 187, p.p211, 319, p.p211);
            s.store_scalar(1383, (1.0 / s.v[88]));
            s.store_mul(1384, 1382, 1383);
            s.store_sqrt_square_offset(44, 1384, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(305, 1384, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1390] = (s.v[305] < 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        if ((!s.b[1389]) && s.b[1390]) {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[1389]) {
            s.store_div_from_scalar_offset_input(1385, 1.0, 305, 1e-50);
            s.store_scaled_mul(1386, 303, 1385, (-p.p208));
        }

        s.b[1391] = (s.v[1386] < (-34.0));
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if ((!s.b[1389]) && (!s.b[1391])) {
            s.store_exp(1382, 1386);
            s.store_div_from_scalar(1385, 1.0, 302);
            s.store_scaled_mul(1383, 1385, 107, (p.p207 * 1.6021918e-19));
        }

        if (!s.b[1389]) {
            s.store_neg(1388, 513);
        }

        s.b[1392] = (s.v[1388] > 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if ((!s.b[1389]) && s.b[1392]) {
            s.store_square(1383, 1388);
            s.store_mul(331, 1383, 1388);
            s.store_offset(1386, 331, p.p212);
        }

        s.b[1393] = (p.p43 == 1.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if s.b[1393] {
            s.store_scalar(1403, s.v[91]);
            s.store_div_from_scalar(1404, 1.0, 1403);
            s.store_scalar(1460, 0.0);
            s.store_scalar(1462, 0.0);
            s.store_scalar(1464, 0.0);
            s.store_neg(1396, 534);
            s.store_mul(1397, 1396, 436);
            s.store_add_scaled_product_indices(331, 1397, 1.0, 1396, 437, 1.0);
            s.store_mul(470, 1397, 438);
            s.store_sub(469, 1397, 470);
            s.store_mul(468, 331, 438);
            s.store_sub(467, 331, 468);
        }

        if (s.b[1393] && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
            s.store_scalar(528, 0.0);
        }

        s.b[1473] = (1.0 == 1.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        s.b[1474] = (1.0 == 2.0);
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1473]) {
            s.store_scale(522, 533, 0.5);
            s.store_scalar(523, p.p292);
            s.store_scalar(528, s.v[525]);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && (s.b[1474] && (!s.b[1473]))) {
            s.store_scale(522, 534, 0.5);
            s.store_scalar(523, p.p68);
            s.store_scalar(528, s.v[524]);
            s.store_scalar(528, 1.0);
        }

        s.b[1475] = (s.v[528] == 0.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_mul_sqrt_ad_rhs(1423, 238, A::div(s.ad_value(521), s.ad_value(536)));
            s.store_scalar(1405, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1406, ((1.0 + -1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1416, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1417, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1418, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1419, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1420, 1417, 1416);
            s.store_neg(1421, 1416);
            s.store_add_scaled_products_indices(1407, 1405, 461, 1.0, 1406, 462, 1.0);
            s.store_add_scaled_products_indices(1408, 1405, 462, 1.0, 1406, 461, 1.0);
            s.store_add_scaled_products_indices(1422, 1407, 1418, 1.0, 1408, 1419, 1.0);
            s.store_offset_ad(1414, A::add_scaled_products(s.ad_value(1407), s.ad_value(1421), 1.0, s.ad_value(1408), s.ad_value(1420), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1394, 1414);
        }

        s.b[1476] = (s.v[1394] > s.v[141]);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1476]) {
            s.store_sub(1395, 1394, 141);
            s.store_sub(1396, 140, 141);
            s.store_div(44, 1395, 1396);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1402, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1402, 1396, 1.0, 1402);
            s.store_add(1399, 141, 1402);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1476])) {
            s.copy_ad(1399, 1394);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_offset_scaled(1415, 1399, -1.0, (-1e-12));
            s.store_mul(1424, 1423, 1404);
            s.store_square(1425, 1424);
            s.store_sub(1426, 1422, 523);
            s.store_div(1394, 521, 230);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1427, 2.0, 225, A::ln(s.ad_value(1394)));
            s.store_neg(1428, 1415);
        }

        s.b[1477] = (s.v[1426] < s.v[1428]);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) {
            s.store_div_from_scalar_mul_ad(1395, 1.0, s.ad_value(225), s.ad_value(1423));
            s.store_mul(1402, 1395, 1403);
            s.store_offset_scaled(1429, 1402, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1430, 1429, 1429, 8.0, 0.0, 1429);
            s.store_sub(1431, 237, 1427);
            s.store_mul_add_rhs(1401, 225, 1426, 1415);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1432, (7.0 * 1.414213562373095), 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);
            s.store_square(1433, 1432);
        }

        s.b[1478] = (s.v[1430] < (s.v[1433] * 1e-8));
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) && s.b[1478]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1435, A::offset(s.ad_value(1432), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1430), 0.5, s.ad_value(1432), 1.0), 1.0, 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) && (!s.b[1478])) {
            s.store_sqrt_add(1434, 1430, 1433);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1435, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, 1402, 1401, (-2.0), 9.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1477]) {
            s.store_powf(1436, 1435, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1437, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1402), 12.0)), 1.0, 1436, 2.0, 1436, 1436, 1.414213562373095);
            s.store_div(1438, 1437, 1436);
            s.store_add_scaled_product_indices(1439, 1415, (-1.0), 1438, 227, 1.0);
            s.store_add(1395, 1439, 1415);
            s.store_div(1396, 1395, 1431);
            s.store_sqrt_square_offset(1397, 1396, 1.0);
            s.store_sub_div_lhs_indices(1440, 1395, 1397, 1415);
            s.store_sub(1396, 1426, 1440);
            s.store_mul(459, 1403, 1396);
            s.copy_ad(458, 459);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_scalar(1438, 3.0);
            s.store_sub_div_lhs_indices(1441, 1438, 225, 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_div_scaled_inputs2_mixed_aia(1401, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, 1402, 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1479] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1479]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_div_scaled_inputs2_mixed_aia(1401, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, 1402, 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1480] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1480]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        s.b[1481] = (s.v[1438] < 3.0);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1481]) {
            s.store_scalar(1442, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1443, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1481]) {
            s.store_offset_div_from_scalar_ad(1444, 1.0, A::mul(s.ad_value(225), s.ad_value(1424)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(1445, 1426, -1.0, 1415, -1.0, 1424, 1.0);
            s.store_add_scaled_inputs3(1446, A::div_scaled_product(A::square(s.ad_value(1443)), s.ad_value(1443), 1.0, A::mul3_scaled_output(s.ad_value(1442), s.ad_value(1442), s.ad_value(1442), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1443), s.ad_value(1444), 1.0, s.ad_value(1442), s.ad_value(1442), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1445), 1.0, s.ad_value(1442), 2.0), 1.0);
            s.store_div_ad(1447, A::add_scaled_square_product(s.ad_value(1443), (-1.0), s.ad_value(1442), s.ad_value(1444), 3.0), A::mul_scaled_lhs(s.ad_value(1442), 9.0, s.ad_value(1442)));
            s.store_sqrt_ad(1398, A::add_scaled_square_product(s.ad_value(1446), 1.0, A::square(s.ad_value(1447)), s.ad_value(1447), 1.0));
            s.store_powf_ad(1448, A::sub(s.ad_value(1398), s.ad_value(1446)), 0.3333333333333333);
            s.store_neg_ad(1449, A::powf(A::add(s.ad_value(1446), s.ad_value(1398)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(1401, 1448, 1.0, 1449, 1.0, A::div_scaled_inputs(s.ad_value(1443), 1.0, s.ad_value(1442), 3.0), -1.0);
            s.store_add_scaled_product_indices(1441, 1415, (-1.0), 1401, 227, 1.0);
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_offset_add(1450, 1426, 1415, 0.1);
            s.store_offset_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0), 1e-50);
            s.store_div(1394, 230, 521);
            s.store_square(1451, 1394);
            s.store_mul(1452, 1451, 1457);
            s.store_mul(1394, 226, 1425);
            s.store_mul(1453, 225, 1450);
            s.store_add_scaled_inputs_product_mixed_aaii(1454, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.store_offset_sub(44, 1453, 1454, (-1.0));
            s.store_scale(45, 1453, 4.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1454, 1453, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1453, 1453, 1454);
            s.store_add_scaled_inputs(1453, 1453, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1455, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.copy_ad(1456, 1438);
            s.store_offset_sub(44, 1455, 1456, (-(0.0008 * 75.0)));
            s.store_scale(45, 1455, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1438, 1455, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub_div_lhs_indices(1440, 1438, 225, 1415);
            s.store_add_offset_lhs_ad_rhs(1395, 1438, (-1.0), A::exp_scaled_input(s.ad_value(1438), -1.0));
        }

        s.b[1482] = (s.v[1395] < (10.0 * 2.220446049250313e-16));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1482]) {
            s.store_scalar(1395, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) {
            s.store_sqrt(1396, 1395);
            s.store_mul(458, 1423, 1396);
            s.store_mul_sub_rhs(459, 1403, 1426, 1440);
        }

        s.b[1483] = (p.p42 == 1.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
            s.store_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0));
            s.store_div(1394, 230, 521);
            s.store_square(1451, 1394);
            s.store_mul(1466, 1451, 1457);
            s.store_scalar(1411, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign26200_loop_guard: usize = 0;
        while {
            let assign26200_cond_e35786: f64 = (2.0 * 20.0);
            let assign26200_cond_e35788: f64 = (assign26200_cond_e35786 + 1.0);
            let assign26200_cond_e35790: f64 = if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (s.v[167] <= assign26200_cond_e35788)) { 1.0 } else { 0.0 };
            assign26200_cond_e35790 != 0.0
        } {
            assign26200_loop_guard += 1;
            assert!(assign26200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
                s.store_scalar(1462, 0.0);
                s.store_mul_add_rhs(1438, 225, 1440, 1415);
            }
            s.b[1484] = (s.v[1438] < 5.0);
            s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1484]) {
                s.store_mul3_ad_middle(1458, A::square(s.ad_value(1438)), 1438, A::offset(A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1459, A::square(s.ad_value(1438)), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1460, 1466, 1458, 1458);
                s.store_mul_product3_rhs(1461, 1459, s.ad_value(1466), s.ad_value(225), s.ad_value(1458), 2.0);
                s.store_mul_offset_ad_rhs(1462, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1463, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1464, A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1465, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1463), s.ad_value(1462), 2.0), 1.0, 1461, 1.0, 1464, 2.0);
            }
            s.b[1485] = (s.v[1438] < 80.0);
            s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) && s.b[1485]) {
                s.store_exp(243, 1438);
                s.store_mul_offset_rhs(1460, 1466, 243, (-1.0));
                s.store_mul3_lhs(1461, 1466, 225, 243);
            }
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) && (!s.b[1485])) {
                s.store_exp_mul(1467, 225, 1440);
                s.store_mul_sub_rhs(1460, 1451, 1467, 1457);
                s.store_mul3_lhs(1461, 1451, 225, 1467);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) {
                s.store_sqrt_add_ad(1464, A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460));
                s.store_scale_ad(1465, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 1.0), 0.5);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
                s.store_add_scaled_inputs_product_indices(1468, 1426, 1.0, 1440, (-1.0), 1424, 1464, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1469, (-1.0), 1424, 1465, 1.0);
            }
            s.b[1486] = (s.v[1411] == 1.0);
            s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1486]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {
                s.store_div_scaled_inputs_indices(494, 1468, -1.0, 1469, 1.0);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {
                s.store_scaled_offset_ad(1470, {
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1487] = (((s.v[494]) as f64).abs() > s.v[1470]);
            s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) && s.b[1487]) {
                s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {
                s.store_add(1440, 1440, 494);
            }
            s.b[1488] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8));
            s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) && s.b[1488]) {
                s.store_scalar(1411, 1.0);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1490] = (s.v[1438] < 5.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1490]) {
            s.store_offset_square(1471, 1462, (10.0 * 2.220446049250313e-16));
            s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1490])) {
            s.store_offset(1471, 1438, (-1.0));
            s.store_sqrt(1472, 1471);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {
            s.store_mul(458, 1423, 1472);
            s.store_div_from_scalar_add_ad(1395, 1.0, s.ad_value(1464), s.ad_value(1472));
            s.store_mul3_lhs(460, 1423, 1460, 1395);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1492] = (1.0 == 1.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        s.b[1493] = (1.0 == 2.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1492]) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1492]) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1493] && (!s.b[1492]))) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1493] && (!s.b[1492]))) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_scalar(1405, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1406, ((1.0 + 1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1416, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1417, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1418, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1419, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1420, 1417, 1416);
            s.store_neg(1421, 1416);
            s.store_add_scaled_products_indices(1407, 1405, 461, 1.0, 1406, 462, 1.0);
            s.store_add_scaled_products_indices(1408, 1405, 462, 1.0, 1406, 461, 1.0);
            s.store_add_scaled_products_indices(1422, 1407, 1418, 1.0, 1408, 1419, 1.0);
            s.store_offset_ad(1414, A::add_scaled_products(s.ad_value(1407), s.ad_value(1421), 1.0, s.ad_value(1408), s.ad_value(1420), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1394, 1414);
        }

        s.b[1494] = (s.v[1394] > s.v[141]);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1494]) {
            s.store_sub(1395, 1394, 141);
            s.store_sub(1396, 140, 141);
            s.store_div(44, 1395, 1396);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1402, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1402, 1396, 1.0, 1402);
            s.store_add(1399, 141, 1402);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1494])) {
            s.copy_ad(1399, 1394);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_offset_scaled(1415, 1399, -1.0, (-1e-12));
            s.store_mul(1424, 1423, 1404);
            s.store_square(1425, 1424);
            s.store_sub(1426, 1422, 523);
            s.store_div(1394, 521, 230);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1427, 2.0, 225, A::ln(s.ad_value(1394)));
            s.store_neg(1428, 1415);
        }

        s.b[1495] = (s.v[1426] < s.v[1428]);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) {
            s.store_div_from_scalar_mul_ad(1395, 1.0, s.ad_value(225), s.ad_value(1423));
            s.store_mul(1402, 1395, 1403);
            s.store_offset_scaled(1429, 1402, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1430, 1429, 1429, 8.0, 0.0, 1429);
            s.store_sub(1431, 237, 1427);
            s.store_mul_add_rhs(1401, 225, 1426, 1415);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1432, (7.0 * 1.414213562373095), 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);
            s.store_square(1433, 1432);
        }

        s.b[1496] = (s.v[1430] < (s.v[1433] * 1e-8));
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) && s.b[1496]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1435, A::offset(s.ad_value(1432), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1430), 0.5, s.ad_value(1432), 1.0), 1.0, 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) && (!s.b[1496])) {
            s.store_sqrt_add(1434, 1430, 1433);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1435, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, 1402, 1401, (-2.0), 9.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1495]) {
            s.store_powf(1436, 1435, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1437, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1402), 12.0)), 1.0, 1436, 2.0, 1436, 1436, 1.414213562373095);
            s.store_div(1438, 1437, 1436);
            s.store_add_scaled_product_indices(1439, 1415, (-1.0), 1438, 227, 1.0);
            s.store_add(1395, 1439, 1415);
            s.store_div(1396, 1395, 1431);
            s.store_sqrt_square_offset(1397, 1396, 1.0);
            s.store_sub_div_lhs_indices(1440, 1395, 1397, 1415);
            s.store_sub(1396, 1426, 1440);
            s.store_mul(459, 1403, 1396);
            s.copy_ad(458, 459);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_scalar(1438, 3.0);
            s.store_sub_div_lhs_indices(1441, 1438, 225, 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_div_scaled_inputs2_mixed_aia(1401, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, 1402, 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1497] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1497]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
            s.store_exp_neg_input(1402, 1438);
            s.store_offset_div_scaled_inputs2_mixed_aia(1401, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, 1402, 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1498] = (s.v[1401] < (10.0 * 2.220446049250313e-16));
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1498]) {
            s.store_scalar(1401, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_add_ad_rhs(1441, 1426, A::mul3_scaled_output(s.ad_value(1425), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        s.b[1499] = (s.v[1438] < 3.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1499]) {
            s.store_scalar(1442, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1443, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1444, 1.0, A::mul(s.ad_value(225), s.ad_value(1424)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(1445, 1426, -1.0, 1415, -1.0, 1424, 1.0);
            s.store_add_scaled_inputs3(1446, A::div_scaled_product(A::square(s.ad_value(1443)), s.ad_value(1443), 1.0, A::mul3_scaled_output(s.ad_value(1442), s.ad_value(1442), s.ad_value(1442), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1443), s.ad_value(1444), 1.0, s.ad_value(1442), s.ad_value(1442), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1445), 1.0, s.ad_value(1442), 2.0), 1.0);
            s.store_div_ad(1447, A::add_scaled_square_product(s.ad_value(1443), (-1.0), s.ad_value(1442), s.ad_value(1444), 3.0), A::mul_scaled_lhs(s.ad_value(1442), 9.0, s.ad_value(1442)));
            s.store_sqrt_ad(1398, A::add_scaled_square_product(s.ad_value(1446), 1.0, A::square(s.ad_value(1447)), s.ad_value(1447), 1.0));
            s.store_powf_ad(1448, A::sub(s.ad_value(1398), s.ad_value(1446)), 0.3333333333333333);
            s.store_neg_ad(1449, A::powf(A::add(s.ad_value(1446), s.ad_value(1398)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(1401, 1448, 1.0, 1449, 1.0, A::div_scaled_inputs(s.ad_value(1443), 1.0, s.ad_value(1442), 3.0), -1.0);
            s.store_add_scaled_product_indices(1441, 1415, (-1.0), 1401, 227, 1.0);
            s.store_mul_add_rhs(1438, 225, 1441, 1415);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_offset_add(1450, 1426, 1415, 0.1);
            s.store_offset_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0), 1e-50);
            s.store_div(1394, 230, 521);
            s.store_square(1451, 1394);
            s.store_mul(1452, 1451, 1457);
            s.store_mul(1394, 226, 1425);
            s.store_mul(1453, 225, 1450);
            s.store_add_scaled_inputs_product_mixed_aaii(1454, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.store_offset_sub(44, 1453, 1454, (-1.0));
            s.store_scale(45, 1453, 4.0);
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1454, 1453, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1453, 1453, 1454);
            s.store_add_scaled_inputs(1453, 1453, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1455, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);
            s.copy_ad(1456, 1438);
            s.store_offset_sub(44, 1455, 1456, (-(0.0008 * 75.0)));
            s.store_scale(45, 1455, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1438, 1455, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub_div_lhs_indices(1440, 1438, 225, 1415);
            s.store_add_offset_lhs_ad_rhs(1395, 1438, (-1.0), A::exp_scaled_input(s.ad_value(1438), -1.0));
        }

        s.b[1500] = (s.v[1395] < (10.0 * 2.220446049250313e-16));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1500]) {
            s.store_scalar(1395, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            s.store_sqrt(1396, 1395);
            s.store_mul(458, 1423, 1396);
            s.store_mul_sub_rhs(459, 1403, 1426, 1440);
        }

        s.b[1501] = (p.p42 == 1.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
            s.store_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0));
            s.store_div(1394, 230, 521);
            s.store_square(1451, 1394);
            s.store_mul(1466, 1451, 1457);
            s.store_scalar(1411, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign27750_loop_guard: usize = 0;
        while {
            let assign27750_cond_e38729: f64 = (2.0 * 20.0);
            let assign27750_cond_e38731: f64 = (assign27750_cond_e38729 + 1.0);
            let assign27750_cond_e38733: f64 = if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (s.v[167] <= assign27750_cond_e38731)) { 1.0 } else { 0.0 };
            assign27750_cond_e38733 != 0.0
        } {
            assign27750_loop_guard += 1;
            assert!(assign27750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
                s.store_scalar(1462, 0.0);
                s.store_mul_add_rhs(1438, 225, 1440, 1415);
            }
            s.b[1502] = (s.v[1438] < 5.0);
            s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1502]) {
                s.store_mul3_ad_middle(1458, A::square(s.ad_value(1438)), 1438, A::offset(A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1459, A::square(s.ad_value(1438)), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1460, 1466, 1458, 1458);
                s.store_mul_product3_rhs(1461, 1459, s.ad_value(1466), s.ad_value(225), s.ad_value(1458), 2.0);
                s.store_mul_offset_ad_rhs(1462, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1463, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1464, A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1465, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1463), s.ad_value(1462), 2.0), 1.0, 1461, 1.0, 1464, 2.0);
            }
            s.b[1503] = (s.v[1438] < 80.0);
            s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && s.b[1503]) {
                s.store_exp(243, 1438);
                s.store_mul_offset_rhs(1460, 1466, 243, (-1.0));
                s.store_mul3_lhs(1461, 1466, 225, 243);
            }
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && (!s.b[1503])) {
                s.store_exp_mul(1467, 225, 1440);
                s.store_mul_sub_rhs(1460, 1451, 1467, 1457);
                s.store_mul3_lhs(1461, 1451, 225, 1467);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) {
                s.store_sqrt_add_ad(1464, A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460));
                s.store_scale_ad(1465, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 1.0), 0.5);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
                s.store_add_scaled_inputs_product_indices(1468, 1426, 1.0, 1440, (-1.0), 1424, 1464, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1469, (-1.0), 1424, 1465, 1.0);
            }
            s.b[1504] = (s.v[1411] == 1.0);
            s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1504]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_div_scaled_inputs_indices(494, 1468, -1.0, 1469, 1.0);
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_scaled_offset_ad(1470, {
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1505] = (((s.v[494]) as f64).abs() > s.v[1470]);
            s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1505]) {
                s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_add(1440, 1440, 494);
            }
            s.b[1506] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8));
            s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };
            if ((((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1506]) {
                s.store_scalar(1411, 1.0);
            }
            if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1508] = (s.v[1438] < 5.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1508]) {
            s.store_offset_square(1471, 1462, (10.0 * 2.220446049250313e-16));
            s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1508])) {
            s.store_offset(1471, 1438, (-1.0));
            s.store_sqrt(1472, 1471);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
            s.store_mul(458, 1423, 1472);
            s.store_div_from_scalar_add_ad(1395, 1.0, s.ad_value(1464), s.ad_value(1472));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
            s.store_mul3_lhs(460, 1423, 1460, 1395);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1510] = (1.0 == 1.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        s.b[1511] = (1.0 == 2.0);
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1405] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1393] && (p.p24 != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1406] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        s.v[317] = p.p189;

        s.b[1514] = (s.v[145] != 0.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if s.b[1514] {
            s.store_add(1513, 157, 161);
            s.store_add_scaled_inputs(314, 1513, s.v[317], 162, (1.0 - s.v[317]));
        }

        s.b[1515] = (p.p64 != 0.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if (s.b[1514] && s.b[1515]) {
            s.store_scalar(315, 0.0);
        }

        s.b[1516] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if (s.b[1514] && s.b[1516]) {
            s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[1517] = (p.p64 != 0.0);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        s.b[1518] = (s.v[246] < 1e-15);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if (((!s.b[1514]) && s.b[1517]) && s.b[1518]) {
            s.store_scalar(315, 0.0);
        }

        if (((!s.b[1514]) && s.b[1517]) && (!s.b[1518])) {
            s.store_scale(1512, 227, 1.0 / (s.v[97]));
            s.store_div_from_scalar(1513, 1.0, 244);
            s.store_mul3_lhs(315, 246, 1512, 1513);
        }

        s.v[1530] = s.v[91];

        s.v[1531] = (1.0 / s.v[1530]);

        s.v[1551] = 0.0;

        s.v[1591] = 0.0;

        s.v[1589] = 0.0;

        s.v[1593] = 0.0;

        s.b[1602] = ((p.p29 >= 1.0) && (p.p188 > 0.0));
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && s.b[1602]) {
            s.store_scalar(1533, p.p171);
            s.store_scalar(1534, p.p172);
            s.copy_ad(1535, 158);
            s.store_scalar(1532, p.p188);
        }

        s.b[1603] = ((s.v[69] == 0.0) && (p.p188 > 0.0));
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && s.b[1602]) && s.b[1603]) {
            if (p.p43 == 1.0) {
                s.store_scale(1520, 287, s.v[1530]);
            } else {
                s.store_scale(1520, 108, s.v[1530]);
            }
        }

        if (((p.p24 != 0.0) && s.b[1602]) && s.b[1603]) {
            s.store_mul_ad_product_rhs(1523, 1533, s.ad_value(1520), A::add(s.ad_value(1534), s.ad_value(1535)));
            s.store_mul(1524, 1532, 1520);
            s.copy_ad(1528, 161);
            s.store_sub_from_scalar(1525, 1.2, 1528);
            s.store_add_scaled_products_indices(267, 158, 1524, 1.0, 1525, 1523, (-1.0));
            s.store_mul_ad_product_rhs(1523, 1533, s.ad_value(1520), A::add_scaled_inputs3(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(157), -1.0));
            s.store_sub(1528, 162, 157);
            s.store_sub_from_scalar(1525, 1.2, 1528);
            s.store_add_scaled_products_left_left_ad(268, A::sub(s.ad_value(158), s.ad_value(157)), 1524, 1.0, 1523, 1525, (-1.0));
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_mul_sqrt_ad_rhs(1552, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));
            s.store_scalar(1536, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1537, ((1.0 + -1.0) / 2.0));
        }

        s.b[1604] = (p.p43 == 1.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1604]) {
            s.store_add_scaled_products_right_right_ad(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1549, 1547, 1546);
            s.store_sub(1551, 1548, 1546);
            s.store_neg(1550, 1546);
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
            s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1536] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1537] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {
            s.store_scalar(1544, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_neg(1519, 1544);
        }

        s.b[1605] = (s.v[1519] > s.v[141]);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1605]) {
            s.store_sub(1520, 1519, 141);
            s.store_sub(1521, 140, 141);
            s.store_div(44, 1520, 1521);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1529, 1521, 1.0, 1529);
            s.store_add(1526, 141, 1529);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1605])) {
            s.copy_ad(1526, 1519);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));
            s.store_scale(1553, 1552, s.v[1531]);
            s.store_square(1554, 1553);
            s.store_sub_from_scalar(1555, s.v[82], 1551);
            s.store_div_from_scalar(1519, s.v[69], 230);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1556, 2.0, 225, A::ln(s.ad_value(1519)));
            s.store_neg(1557, 1545);
        }

        s.b[1606] = (s.v[1555] < s.v[1557]);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {
            s.store_div_from_scalar_mul_ad(1520, 1.0, s.ad_value(225), s.ad_value(1552));
            s.store_scale(1529, 1520, s.v[1530]);
            s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);
            s.store_sub(1560, 237, 1556);
            s.store_mul_add_rhs(1528, 225, 1555, 1545);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1561, (7.0 * 1.414213562373095), 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);
            s.store_square(1562, 1561);
        }

        s.b[1607] = (s.v[1559] < (s.v[1562] * 1e-8));
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && s.b[1607]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && (!s.b[1607])) {
            s.store_sqrt_add(1563, 1559, 1562);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {
            s.store_powf(1565, 1564, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);
            s.store_div(1567, 1566, 1565);
            s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);
            s.store_add(1520, 1568, 1545);
            s.store_div(1521, 1520, 1560);
            s.store_sqrt_square_offset(1522, 1521, 1.0);
            s.store_sub_div_lhs_indices(1569, 1520, 1522, 1545);
            s.store_sub(1521, 1555, 1569);
            s.store_scale(459, 1521, s.v[1530]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_scalar(1567, 3.0);
            s.store_sub_div_lhs_indices(1570, 1567, 225, 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1608] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1608]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1609] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1609]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1610] = (s.v[1567] < 3.0);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1610]) {
            s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(1574, 1555, -1.0, 1545, -1.0, 1553, 1.0);
            s.store_add_scaled_inputs3(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1574), 1.0, s.ad_value(1571), 2.0), 1.0);
            s.store_div_ad(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), A::mul_scaled_lhs(s.ad_value(1571), 9.0, s.ad_value(1571)));
            s.store_sqrt_ad(1524, A::add_scaled_square_product(s.ad_value(1575), 1.0, A::square(s.ad_value(1576)), s.ad_value(1576), 1.0));
            s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);
            s.store_neg_ad(1578, A::powf(A::add(s.ad_value(1575), s.ad_value(1524)), 0.3333333333333333));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1610]) {
            s.store_add_scaled_inputs3_mixed_iia(1528, 1577, 1.0, 1578, 1.0, A::div_scaled_inputs(s.ad_value(1572), 1.0, s.ad_value(1571), 3.0), -1.0);
            s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1611] = (p.p41 > 0.0);
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            s.store_offset_add(1579, 1555, 1545, 0.1);
            s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1581, 1580, 1586);
            s.store_mul(1519, 226, 1554);
            s.store_mul(1582, 225, 1579);
            s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.store_offset_sub(44, 1582, 1583, (-1.0));
            s.store_scale(45, 1582, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1583, 1582, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1582, 1582, 1583);
            s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.copy_ad(1585, 1567);
            s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));
            s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1567, 1584, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_sub_div_lhs_indices(1569, 1567, 225, 1545);
            s.store_add_offset_lhs_ad_rhs(1520, 1567, (-1.0), A::exp_scaled_input(s.ad_value(1567), -1.0));
        }

        s.b[1612] = (s.v[1520] < (10.0 * 2.220446049250313e-16));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1612]) {
            s.store_scalar(1520, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {
            s.store_sqrt(1521, 1520);
            s.store_mul(458, 1552, 1521);
            s.store_scaled_sub(459, 1555, 1569, s.v[1530]);
        }

        s.b[1613] = (p.p41 == 1.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
            s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1595, 1580, 1586);
            s.store_scalar(1542, 0.0);
            s.store_scalar(1589, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign29750_loop_guard: usize = 0;
        while {
            let assign29750_cond_e42262: f64 = (2.0 * 20.0);
            let assign29750_cond_e42264: f64 = (assign29750_cond_e42262 + 1.0);
            let assign29750_cond_e42266: f64 = if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (s.v[167] <= assign29750_cond_e42264)) { 1.0 } else { 0.0 };
            assign29750_cond_e42266 != 0.0
        } {
            assign29750_loop_guard += 1;
            assert!(assign29750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
                s.store_scalar(1591, 0.0);
                s.store_mul_add_rhs(1567, 225, 1569, 1545);
            }
            s.b[1614] = (s.v[1567] < 5.0);
            s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1614]) {
                s.store_mul3_ad_middle(1587, A::square(s.ad_value(1567)), 1567, A::offset(A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1588, A::square(s.ad_value(1567)), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1589, 1595, 1587, 1587);
                s.store_mul_product3_rhs(1590, 1588, s.ad_value(1595), s.ad_value(225), s.ad_value(1587), 2.0);
                s.store_mul_offset_ad_rhs(1591, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1592, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1593, A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1594, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1592), s.ad_value(1591), 2.0), 1.0, 1590, 1.0, 1593, 2.0);
            }
            s.b[1615] = (s.v[1567] < 80.0);
            s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && s.b[1615]) {
                s.store_exp(243, 1567);
                s.store_mul_offset_rhs(1589, 1595, 243, (-1.0));
                s.store_mul3_lhs(1590, 1595, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && (!s.b[1615])) {
                s.store_exp_mul(1596, 225, 1569);
                s.store_mul_sub_rhs(1589, 1580, 1596, 1586);
                s.store_mul3_lhs(1590, 1580, 225, 1596);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) {
                s.store_sqrt_add_ad(1593, A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589));
                s.store_scale_ad(1594, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
                s.store_add_scaled_inputs_product_indices(1597, 1555, 1.0, 1569, (-1.0), 1553, 1593, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1598, (-1.0), 1553, 1594, 1.0);
            }
            s.b[1616] = (s.v[1542] == 1.0);
            s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1616]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_div_scaled_inputs_indices(494, 1597, -1.0, 1598, 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_scaled_offset_ad(1599, {
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1617] = (((s.v[494]) as f64).abs() > s.v[1599]);
            s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1617]) {
                s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_add(1569, 1569, 494);
            }
            s.b[1618] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8));
            s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1618]) {
                s.store_scalar(1542, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1620] = (s.v[1567] < 5.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1620]) {
            s.store_offset_square(1600, 1591, (10.0 * 2.220446049250313e-16));
            s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1620])) {
            s.store_offset(1600, 1567, (-1.0));
            s.store_sqrt(1601, 1600);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
            s.store_mul(458, 1552, 1601);
            s.store_div_from_scalar_add_ad(1520, 1.0, s.ad_value(1593), s.ad_value(1601));
            s.store_mul3_lhs(460, 1552, 1589, 1520);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            if (p.p43 == 1.0) {
                s.store_mul(1523, 287, 1532);
            } else {
                s.store_mul(1523, 108, 1532);
            }
        }

        s.b[1622] = (((s.v[1538] != 0.0) && (p.p43 == 0.0)) || ((s.v[1536] != 0.0) && (p.p43 == 1.0)));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1622]) {
            s.store_mul(455, 1523, 459);
            s.store_mul(457, 1523, 458);
        }

        s.b[1623] = (((s.v[1539] != 0.0) && (p.p43 == 0.0)) || ((s.v[1537] != 0.0) && (p.p43 == 1.0)));
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1623]) {
            s.store_mul(454, 1523, 459);
            s.store_mul(456, 1523, 458);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_scalar(1536, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1537, ((1.0 + 1.0) / 2.0));
        }

        s.b[1624] = (p.p43 == 1.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1624]) {
            s.store_add_scaled_products_right_right_ad(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1549, 1547, 1546);
            s.store_sub(1551, 1548, 1546);
            s.store_neg(1550, 1546);
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
            s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {
            s.store_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);
            s.store_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1536] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1537] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {
            s.store_scalar(1544, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_neg(1519, 1544);
        }

        s.b[1625] = (s.v[1519] > s.v[141]);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1625]) {
            s.store_sub(1520, 1519, 141);
            s.store_sub(1521, 140, 141);
            s.store_div(44, 1520, 1521);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1529, 1521, 1.0, 1529);
            s.store_add(1526, 141, 1529);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1625])) {
            s.copy_ad(1526, 1519);
        }

        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));
            s.store_scale(1553, 1552, s.v[1531]);
            s.store_square(1554, 1553);
            s.store_sub_from_scalar(1555, s.v[82], 1551);
            s.store_div_from_scalar(1519, s.v[69], 230);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1556, 2.0, 225, A::ln(s.ad_value(1519)));
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) {
            s.store_neg(1557, 1545);
        }

        s.b[1626] = (s.v[1555] < s.v[1557]);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {
            s.store_div_from_scalar_mul_ad(1520, 1.0, s.ad_value(225), s.ad_value(1552));
            s.store_scale(1529, 1520, s.v[1530]);
            s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);
            s.store_sub(1560, 237, 1556);
            s.store_mul_add_rhs(1528, 225, 1555, 1545);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1561, (7.0 * 1.414213562373095), 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);
            s.store_square(1562, 1561);
        }

        s.b[1627] = (s.v[1559] < (s.v[1562] * 1e-8));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && s.b[1627]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && (!s.b[1627])) {
            s.store_sqrt_add(1563, 1559, 1562);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {
            s.store_powf(1565, 1564, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);
            s.store_div(1567, 1566, 1565);
            s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);
            s.store_add(1520, 1568, 1545);
            s.store_div(1521, 1520, 1560);
            s.store_sqrt_square_offset(1522, 1521, 1.0);
            s.store_sub_div_lhs_indices(1569, 1520, 1522, 1545);
            s.store_sub(1521, 1555, 1569);
            s.store_scale(459, 1521, s.v[1530]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_scalar(1567, 3.0);
            s.store_sub_div_lhs_indices(1570, 1567, 225, 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1628] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1628]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
            s.store_exp_neg_input(1529, 1567);
            s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1629] = (s.v[1528] < (10.0 * 2.220446049250313e-16));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1629]) {
            s.store_scalar(1528, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_add_ad_rhs(1570, 1555, A::mul3_scaled_output(s.ad_value(1554), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1630] = (s.v[1567] < 3.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1630]) {
            s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(1574, 1555, -1.0, 1545, -1.0, 1553, 1.0);
            s.store_add_scaled_inputs3(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1574), 1.0, s.ad_value(1571), 2.0), 1.0);
            s.store_div_ad(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), A::mul_scaled_lhs(s.ad_value(1571), 9.0, s.ad_value(1571)));
            s.store_sqrt_ad(1524, A::add_scaled_square_product(s.ad_value(1575), 1.0, A::square(s.ad_value(1576)), s.ad_value(1576), 1.0));
            s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);
            s.store_neg_ad(1578, A::powf(A::add(s.ad_value(1575), s.ad_value(1524)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(1528, 1577, 1.0, 1578, 1.0, A::div_scaled_inputs(s.ad_value(1572), 1.0, s.ad_value(1571), 3.0), -1.0);
            s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);
            s.store_mul_add_rhs(1567, 225, 1570, 1545);
        }

        s.b[1631] = (p.p41 > 0.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            s.store_offset_add(1579, 1555, 1545, 0.1);
            s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1581, 1580, 1586);
            s.store_mul(1519, 226, 1554);
            s.store_mul(1582, 225, 1579);
            s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.store_offset_sub(44, 1582, 1583, (-1.0));
            s.store_scale(45, 1582, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1583, 1582, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1582, 1582, 1583);
            s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);
            s.copy_ad(1585, 1567);
            s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));
            s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1567, 1584, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_sub_div_lhs_indices(1569, 1567, 225, 1545);
            s.store_add_offset_lhs_ad_rhs(1520, 1567, (-1.0), A::exp_scaled_input(s.ad_value(1567), -1.0));
        }

        s.b[1632] = (s.v[1520] < (10.0 * 2.220446049250313e-16));
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1632]) {
            s.store_scalar(1520, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {
            s.store_sqrt(1521, 1520);
            s.store_mul(458, 1552, 1521);
            s.store_scaled_sub(459, 1555, 1569, s.v[1530]);
        }

        s.b[1633] = (p.p41 == 1.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
            s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));
            s.store_scale(1519, 230, 1.0 / (s.v[69]));
            s.store_square(1580, 1519);
            s.store_mul(1595, 1580, 1586);
            s.store_scalar(1542, 0.0);
            s.store_scalar(1589, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign31350_loop_guard: usize = 0;
        while {
            let assign31350_cond_e45498: f64 = (2.0 * 20.0);
            let assign31350_cond_e45500: f64 = (assign31350_cond_e45498 + 1.0);
            let assign31350_cond_e45502: f64 = if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (s.v[167] <= assign31350_cond_e45500)) { 1.0 } else { 0.0 };
            assign31350_cond_e45502 != 0.0
        } {
            assign31350_loop_guard += 1;
            assert!(assign31350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
                s.store_scalar(1591, 0.0);
                s.store_mul_add_rhs(1567, 225, 1569, 1545);
            }
            s.b[1634] = (s.v[1567] < 5.0);
            s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1634]) {
                s.store_mul3_ad_middle(1587, A::square(s.ad_value(1567)), 1567, A::offset(A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1588, A::square(s.ad_value(1567)), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1589, 1595, 1587, 1587);
                s.store_mul_product3_rhs(1590, 1588, s.ad_value(1595), s.ad_value(225), s.ad_value(1587), 2.0);
                s.store_mul_offset_ad_rhs(1591, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1592, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1593, A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1594, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1592), s.ad_value(1591), 2.0), 1.0, 1590, 1.0, 1593, 2.0);
            }
            s.b[1635] = (s.v[1567] < 80.0);
            s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) && s.b[1635]) {
                s.store_exp(243, 1567);
                s.store_mul_offset_rhs(1589, 1595, 243, (-1.0));
                s.store_mul3_lhs(1590, 1595, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) && (!s.b[1635])) {
                s.store_exp_mul(1596, 225, 1569);
                s.store_mul_sub_rhs(1589, 1580, 1596, 1586);
                s.store_mul3_lhs(1590, 1580, 225, 1596);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1634])) {
                s.store_sqrt_add_ad(1593, A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589));
                s.store_scale_ad(1594, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
                s.store_add_scaled_inputs_product_indices(1597, 1555, 1.0, 1569, (-1.0), 1553, 1593, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1598, (-1.0), 1553, 1594, 1.0);
            }
            s.b[1636] = (s.v[1542] == 1.0);
            s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1636]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {
                s.store_div_scaled_inputs_indices(494, 1597, -1.0, 1598, 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {
                s.store_scaled_offset_ad(1599, {
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1637] = (((s.v[494]) as f64).abs() > s.v[1599]);
            s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) && s.b[1637]) {
                s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) {
                s.store_add(1569, 1569, 494);
            }
            s.b[1638] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8));
            s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1636])) && s.b[1638]) {
                s.store_scalar(1542, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1640] = (s.v[1567] < 5.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && s.b[1640]) {
            s.store_offset_square(1600, 1591, (10.0 * 2.220446049250313e-16));
            s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) && (!s.b[1640])) {
            s.store_offset(1600, 1567, (-1.0));
            s.store_sqrt(1601, 1600);
        }

    }
}
