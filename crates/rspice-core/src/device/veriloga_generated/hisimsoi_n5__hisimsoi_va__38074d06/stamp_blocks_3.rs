#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[735] && (!s.b[927])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }

        s.b[1037] = (s.v[430] == 0.0);
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if ((s.b[735] && (!s.b[927])) && s.b[1037]) {
            s.copy_ad(352, 346);
            s.copy_ad(353, 347);
            s.copy_ad(354, 348);
        }

        if (s.b[735] && (!s.b[927])) {
            s.copy_ad(162, 352);
            s.copy_ad(157, 453);
        }

        s.b[1038] = (s.v[349] < 0.0);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if ((s.b[735] && (!s.b[927])) && s.b[1038]) {
            s.store_scalar(145, 1.0);
        }

        if (s.b[735] && (!s.b[927])) {
            s.copy_ad(374, 349);
            s.copy_ad(375, 352);
            s.store_sub(164, 375, 374);
            s.copy_ad(373, 351);
            s.store_scale(400, 401, 9662367879.197212);
            s.store_add_scaled_inputs3_mixed_iia(246, 358, 1.0, 355, (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);
        }

        s.b[1039] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if ((s.b[735] && (!s.b[927])) && s.b[1039]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[735] && (!s.b[927])) {
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_sub(411, 352, 349);
            s.store_offset(411, 411, 5e-12);
            s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);
            s.store_div_scaled_inputs2_mixed_aai(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), 410, 1.0);
        }

        s.b[1040] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if ((s.b[735] && (!s.b[927])) && s.b[1040]) {
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

        s.b[1041] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        s.b[1042] = (2.0 == 1.0);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if ((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && s.b[1042]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1043] = (2.0 == 2.0);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1044] = (2.0 == 4.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if ((((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (!s.b[1042])) && (!s.b[1043])) && s.b[1044]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1045] = (2.0 == 8.0);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (((((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (!s.b[1042])) && (!s.b[1043])) && (!s.b[1044])) && s.b[1045]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13400_loop_guard: usize = 0;
        while {
            let assign13400_cond_e19027: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13400_cond_e19027 != 0.0
        } {
            assign13400_loop_guard += 1;
            assert!(assign13400_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[735] && (!s.b[927])) && s.b[1040]) && (!s.b[1041])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[735] && (!s.b[927])) && s.b[1040]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[735] && (!s.b[927])) && (!s.b[1040])) {
            s.store_neg(328, 409);
        }

        if (s.b[735] && (!s.b[927])) {
            s.store_neg(409, 328);
        }

        s.b[1046] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if ((s.b[735] && (!s.b[927])) && s.b[1046]) {
            s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));
        }

        if (s.b[735] && (!s.b[927])) {
            s.store_sub(414, 355, 358);
        }

        s.b[1047] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if ((s.b[735] && (!s.b[927])) && s.b[1047]) {
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

        s.b[1048] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        s.b[1049] = (2.0 == 1.0);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if ((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && s.b[1049]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1050] = (2.0 == 2.0);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (!s.b[1049])) && s.b[1050]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1051] = (2.0 == 4.0);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if ((((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (!s.b[1049])) && (!s.b[1050])) && s.b[1051]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1052] = (2.0 == 8.0);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (((((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (!s.b[1049])) && (!s.b[1050])) && (!s.b[1051])) && s.b[1052]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13760_loop_guard: usize = 0;
        while {
            let assign13760_cond_e19453: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13760_cond_e19453 != 0.0
        } {
            assign13760_loop_guard += 1;
            assert!(assign13760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[735] && (!s.b[927])) && s.b[1047]) && (!s.b[1048])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[735] && (!s.b[927])) && s.b[1047]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[735] && (!s.b[927])) && (!s.b[1047])) {
        }

        if (s.b[735] && (!s.b[927])) {
            s.store_offset_div_scaled_inputs_mixed_ia(412, 414, (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);
            s.store_mul_ad_product_lhs(328, A::square(s.ad_value(411)), s.ad_value(411), 411);
            s.store_mul(415, 412, 411);
            s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);
        }

        s.b[1053] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if ((s.b[735] && (!s.b[927])) && s.b[1053]) {
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

        s.b[1054] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        s.b[1055] = (2.0 == 1.0);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if ((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && s.b[1055]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1056] = (2.0 == 2.0);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (!s.b[1055])) && s.b[1056]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1057] = (2.0 == 4.0);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if ((((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (!s.b[1055])) && (!s.b[1056])) && s.b[1057]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1058] = (2.0 == 8.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (((((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (!s.b[1055])) && (!s.b[1056])) && (!s.b[1057])) && s.b[1058]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign14120_loop_guard: usize = 0;
        while {
            let assign14120_cond_e19882: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign14120_cond_e19882 != 0.0
        } {
            assign14120_loop_guard += 1;
            assert!(assign14120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[735] && (!s.b[927])) && s.b[1053]) && (!s.b[1054])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[735] && (!s.b[927])) && s.b[1053]) {
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
        if ((s.b[735] && (!s.b[927])) && s.b[1053]) {
            s.store_sub_from_scalar(413, 1e-5, 43);
        }

        if ((s.b[735] && (!s.b[927])) && (!s.b[1053])) {
        }

        if (s.b[735] && (!s.b[927])) {
            s.copy_ad(190, 413);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if (s.b[735] && (!s.b[927])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[735] && (!s.b[927])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        if (!s.b[735]) {
            s.copy_ad(515, 154);
        }

        s.b[1065] = (s.v[416] < p.p237);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if ((!s.b[735]) && s.b[1065]) {
            s.store_scalar(339, 1.0);
        }

        if ((!s.b[735]) && (!s.b[1065])) {
            s.store_scalar(339, 2.0);
        }

        if (!s.b[735]) {
            s.store_add_scaled_inputs3_offset_indices(160, 185, (-1.0), 320, 1.0, 515, 1.0, s.v[123]);
        }

        s.b[1066] = (s.v[158] < s.v[160]);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if ((!s.b[735]) && s.b[1066]) {
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

        s.b[1067] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (((!s.b[735]) && s.b[1066]) && s.b[1067]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((!s.b[735]) && s.b[1066]) && (!s.b[1067])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);
        }

        if ((!s.b[735]) && s.b[1066]) {
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

        s.b[1068] = (s.v[144] >= 1.0);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (((!s.b[735]) && (!s.b[1066])) && s.b[1068]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(378, s.v[619]);
        }

        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {
            s.store_add_ad_rhs(376, 159, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
            s.store_mul_sub_rhs(181, 225, 376, 515);
        }

        s.b[1069] = (s.v[181] < 3.0);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && s.b[1069]) {
            s.store_mul_sub_rhs(337, 225, 159, 515);
            s.store_div_from_scalar_scaled_mul(328, 1.0, 225, 240, (1.414213562373095 / 108.0));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 515, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1070] = (s.v[158] <= s.v[182]);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && s.b[1070]) {
            s.copy_ad(378, 376);
        }

        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && (!s.b[1070])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul3_lhs(329, 328, 159, 159);
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, s.ad_value(159)));
            s.store_div_ln_lhs(377, 329, 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && (!s.b[1070])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && (!s.b[1070])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {
            s.store_offset(336, 515, (5e-12 / 2.0));
        }

        s.b[1071] = (s.v[378] < s.v[336]);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && s.b[1071]) {
            s.copy_ad(378, 336);
        }

        if ((!s.b[735]) && (!s.b[1066])) {
            s.copy_ad(161, 378);
            s.copy_ad(163, 376);
        }

        s.b[1072] = ((p.p25 == 1.0) && (p.p26 == 2.0));
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if ((!s.b[735]) && s.b[1072]) {
            s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
        }

        if ((!s.b[735]) && (!s.b[1072])) {
            s.store_scalar(393, 0.0);
        }

        if (!s.b[735]) {
            s.store_exp_mul(486, 225, 515);
            s.store_mul(487, 379, 486);
            s.store_scalar(430, 0.0);
            s.copy_ad(349, 161);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ln_lhs(420, 328, 419);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign15050_loop_guard: usize = 0;
        while {
            let assign15050_cond_e21050: f64 = (s.v[57] + 1.0);
            let assign15050_cond_e21052: f64 = if ((!s.b[735]) && (s.v[167] <= assign15050_cond_e21050)) { 1.0 } else { 0.0 };
            assign15050_cond_e21052 != 0.0
        } {
            assign15050_loop_guard += 1;
            assert!(assign15050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[735]) {
                s.store_sub(417, 349, 515);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1073] = (s.v[337] < 80.0);
            s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };
            if ((!s.b[735]) && s.b[1073]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ln_offset_lhs(422, 329, 1.0, 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if ((!s.b[735]) && (!s.b[1073])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if (!s.b[735]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1074] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };
            if ((!s.b[735]) && s.b[1074]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1075] = (s.v[181] < 0.0);
            s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && s.b[1074]) && s.b[1075]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1076] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && (!s.b[1074])) && s.b[1076]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if (((!s.b[735]) && (!s.b[1074])) && (!s.b[1076])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1077] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };
            if ((!s.b[735]) && s.b[1077]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1078] = (s.v[338] == (-1.0));
            s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };
            if ((!s.b[735]) && s.b[1078]) {
                s.store_scalar(401, 0.0);
            }
            if ((!s.b[735]) && (!s.b[1078])) {
                s.store_mul(401, 444, 242);
            }
            s.b[1079] = (s.v[401] < (p.p237 * 1.01));
            s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };
            if ((!s.b[735]) && s.b[1079]) {
                s.store_scalar(339, 1.0);
            }
            if ((!s.b[735]) && (!s.b[1079])) {
                s.store_scalar(339, 2.0);
            }
            if (!s.b[735]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1080] = (s.v[181] < 0.0);
            s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };
            if ((!s.b[735]) && s.b[1080]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1081] = (s.v[181] < 1e-7);
            s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && (!s.b[1080])) && s.b[1081]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            s.b[1082] = (s.v[181] < 80.0);
            s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };
            if ((((!s.b[735]) && (!s.b[1080])) && (!s.b[1081])) && s.b[1082]) {
                s.store_exp(243, 181);
                s.store_mul_sub_ad_rhs(488, 487, s.ad_value(243), A::offset(s.ad_value(181), 1.0));
                s.store_mul_ad_product_rhs(489, 487, s.ad_value(225), A::offset(s.ad_value(243), (-1.0)));
            }
            if ((((!s.b[735]) && (!s.b[1080])) && (!s.b[1081])) && (!s.b[1082])) {
                s.store_exp_mul(485, 225, 349);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(485), s.ad_value(486)));
            }
            if (((!s.b[735]) && (!s.b[1080])) && (!s.b[1081])) {
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if (!s.b[735]) {
                s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1083] = (s.v[430] == 1.0);
            s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };
            if ((!s.b[735]) && s.b[1083]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((!s.b[735]) && (!s.b[1083])) {
                s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);
            }
            if ((!s.b[735]) && (!s.b[1083])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1084] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && (!s.b[1083])) && s.b[1084]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((!s.b[735]) && (!s.b[1083])) {
                s.store_add(349, 349, 494);
            }
            s.b[1085] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && (!s.b[1083])) && s.b[1085]) {
                s.store_scalar(430, 1.0);
            }
            if (!s.b[735]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (!s.b[735]) {
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

        s.b[1086] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((!s.b[735]) && s.b[1086]) {
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

        s.b[1087] = (s.v[612] == 0.0);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if ((!s.b[735]) && s.b[1087]) {
            s.copy_ad(453, 157);
            s.store_scalar(1094, 1e-50);
            s.store_div_square_rhs(1089, 545, 323);
            s.store_offset_mul_ad(1091, A::div_from_scalar(2.0, s.ad_value(1089)), A::sub(s.ad_value(159), s.ad_value(1094)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1089), 1.0);
        }

        s.b[1095] = ((s.v[1091] < s.v[332]) && (s.v[332] >= 0.0));
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (((!s.b[735]) && s.b[1087]) && s.b[1095]) {
            s.store_sub(44, 332, 1091);
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

        s.b[1096] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        s.b[1097] = (4.0 == 1.0);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && s.b[1097]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1098] = (4.0 == 2.0);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if ((((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (!s.b[1097])) && s.b[1098]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1099] = (4.0 == 4.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (((((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (!s.b[1097])) && (!s.b[1098])) && s.b[1099]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1100] = (4.0 == 8.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if ((((((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (!s.b[1097])) && (!s.b[1098])) && (!s.b[1099])) && s.b[1100]) {
            s.store_scalar(55, 4.0);
        }

        if ((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e22465: f64 = if (((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign15770_cond_e22465 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((!s.b[735]) && s.b[1087]) && s.b[1095]) && (!s.b[1096])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((!s.b[735]) && s.b[1087]) && s.b[1095]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(1091, 332, 43);
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1095])) {
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_sqrt(1090, 1091);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[735]) && s.b[1087]) {
            s.store_add_ad_rhs(1094, 159, A::mul_sub_from_scalar_rhs(s.ad_value(1089), 1.0, s.ad_value(1090)));
            s.store_sqrt_square_offset(44, 1094, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1094, 1094, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1101] = (s.v[1094] < 0.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if (((!s.b[735]) && s.b[1087]) && s.b[1101]) {
            s.store_scalar(1094, 0.0);
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_div(1088, 157, 1094);
            s.store_pow_offset_rhs(1089, 1088, 138, (-1.0));
            s.store_mul(1093, 1089, 1088);
            s.store_offset(1090, 1093, 1.0);
            s.store_pow_ad(1091, s.ad_value(1090), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));
            s.store_mul(1092, 1091, 1090);
            s.store_div(452, 157, 1092);
            s.copy_ad(157, 452);
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));
        }

        s.b[1102] = (s.v[157] <= 0.0);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if (((!s.b[735]) && s.b[1087]) && s.b[1102]) {
            s.store_scalar(164, 0.0);
            s.copy_ad(162, 161);
            s.store_scalar(430, 0.0);
        }

        s.b[1103] = (s.v[144] >= 1.0);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1103]) {
            s.store_scalar(352, s.v[622]);
            s.store_sub_from_scalar(165, s.v[622], 161);
        }

        s.b[1104] = (s.v[144] == 0.0);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            if ((s.v[163] - s.v[161]) >= 0.0) {
                s.store_sub(166, 163, 161);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            s.store_offset_sub_scaled_inputs(44, s.ad_value(166), (1.0 + 0.3), s.ad_value(157), 1.0, (-0.03));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));
        }

        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[1105] = (s.v[165] < 0.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1105]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1106] = (s.v[165] > s.v[157]);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if (((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && (!s.b[1105])) && s.b[1106]) {
            s.copy_ad(165, 157);
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1102])) {
            s.copy_ad(164, 165);
            s.store_add(162, 161, 164);
            s.store_scalar(430, 0.0);
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.copy_ad(352, 162);
            s.store_scalar(168, 1.0);
        }

        let mut assign16230_loop_guard: usize = 0;
        while {
            let assign16230_cond_e22998: f64 = (s.v[58] + 1.0);
            let assign16230_cond_e23000: f64 = if (((!s.b[735]) && s.b[1087]) && (s.v[168] <= assign16230_cond_e22998)) { 1.0 } else { 0.0 };
            assign16230_cond_e23000 != 0.0
        } {
            assign16230_loop_guard += 1;
            assert!(assign16230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[735]) && s.b[1087]) {
                s.store_sub(418, 352, 515);
                s.store_mul(181, 225, 418);
                s.store_mul_sub_rhs(337, 420, 418, 419);
            }
            s.b[1107] = (s.v[337] < 80.0);
            s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && s.b[1087]) && s.b[1107]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ln_offset_lhs(422, 329, 1.0, 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1107])) {
                s.store_sub(422, 418, 419);
                s.store_scalar(423, 1.0);
            }
            if ((!s.b[735]) && s.b[1087]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1108] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && s.b[1087]) && s.b[1108]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1109] = (s.v[181] < 0.0);
            s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };
            if ((((!s.b[735]) && s.b[1087]) && s.b[1108]) && s.b[1109]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1110] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1108])) && s.b[1110]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1108])) && (!s.b[1110])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1111] = (s.v[338] == (-1.0));
            s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && s.b[1087]) && s.b[1111]) {
                s.store_scalar(401, 0.0);
            }
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1111])) {
                s.store_mul(401, 444, 242);
            }
            if ((!s.b[735]) && s.b[1087]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1112] = (s.v[181] < 0.0);
            s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && s.b[1087]) && s.b[1112]) {
                s.store_neg(499, 242);
                s.store_neg(500, 443);
            }
            s.b[1113] = (s.v[181] < 1e-7);
            s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1112])) && s.b[1113]) {
                s.copy_ad(499, 242);
                s.copy_ad(500, 443);
            }
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1112])) && (!s.b[1113])) {
                s.store_mul_sub_rhs(501, 225, 352, 157);
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(497, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(498, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(499, 242, 497);
                s.store_div_scaled_add_product(500, s.ad_value(498), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(499), 1.0);
            }
            if ((!s.b[735]) && s.b[1087]) {
                s.store_add_scaled_inputs_products_indices(503, 352, 1.0, 159, (-1.0), 240, 499, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(504, 240, 500, 1.0);
            }
            s.b[1114] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));
            s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };
            if (((!s.b[735]) && s.b[1087]) && s.b[1114]) {
                s.store_scalar(168, (s.v[58] + 1.0));
            }
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1114])) {
                s.store_div_scaled_inputs_indices(495, 503, -1.0, 504, 1.0);
            }
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1114])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1115] = (((s.v[495]) as f64).abs() > s.v[496]);
            s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1114])) && s.b[1115]) {
                s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1114])) {
                s.store_add(352, 352, 495);
            }
            s.b[1116] = ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8));
            s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1114])) && s.b[1116]) {
                s.store_scalar(430, 1.0);
            }
            if ((!s.b[735]) && s.b[1087]) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((!s.b[735]) && s.b[1087]) {
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
            s.store_sub(427, 355, 358);
            s.store_square(428, 238);
        }

        s.b[1117] = (s.v[339] <= 1.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (((!s.b[735]) && s.b[1087]) && s.b[1117]) {
            s.store_add_scaled_inputs3_mixed_aia(246, A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), 1.0, 427, (-1.0), A::div_scaled_product(A::square(s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), -1.0);
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1117])) {
            s.store_mul(246, 164, 511);
        }

        s.b[1118] = ((s.v[84] >= 1.0) && (s.v[246] < 0.0));
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if (((!s.b[735]) && s.b[1087]) && s.b[1118]) {
            s.store_scalar(246, 0.0);
        }

        s.b[1119] = (s.v[339] <= 1.0);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        s.b[1120] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && s.b[1119]) && s.b[1120]) {
            s.store_add_scaled_product_mixed_aia(437, A::div_scaled_product(A::mul3(A::add_scaled_inputs_product(s.ad_value(425), 1.0, s.ad_value(424), (-2.0), A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div_scaled_product(s.ad_value(424), s.ad_value(424), 2.0, s.ad_value(428), 1.0)), A::div_scaled_product(s.ad_value(426), s.ad_value(426), 0.1, s.ad_value(428), 1.0)), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), 1.0, 424, A::sub(A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), s.ad_value(427)), 1.0);
            s.store_div(437, 437, 246);
        }

        if ((((!s.b[735]) && s.b[1087]) && s.b[1119]) && (!s.b[1120])) {
            s.copy_ad(437, 424);
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1119])) {
            s.store_scaled_add(437, 359, 356, 0.5);
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_scale(328, 240, 2.0);
            s.store_mul_sub_rhs(512, 328, 510, 170);
            s.store_add(191, 164, 512);
            s.store_div_from_scalar(328, 1.0, 192);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[735]) && s.b[1087]) {
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

        s.b[1121] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        s.b[1122] = (4.0 == 1.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && s.b[1121]) && s.b[1122]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1123] = (4.0 == 2.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && s.b[1123]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1124] = (4.0 == 4.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if ((((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && (!s.b[1123])) && s.b[1124]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1125] = (4.0 == 8.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (((((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && (!s.b[1123])) && (!s.b[1124])) && s.b[1125]) {
            s.store_scalar(55, 4.0);
        }

        if (((!s.b[735]) && s.b[1087]) && s.b[1121]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign16990_loop_guard: usize = 0;
        while {
            let assign16990_cond_e24542: f64 = if ((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign16990_cond_e24542 != 0.0
        } {
            assign16990_loop_guard += 1;
            assert!(assign16990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[735]) && s.b[1087]) && s.b[1121]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1121])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(337, 336, 53, 1.0);
            s.store_sub_from_scalar(190, 1.0, 337);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if ((!s.b[735]) && s.b[1087]) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[735]) && s.b[1087]) {
            s.store_div_scaled_product_indices(328, 192, 478, 0.6666666666666667, 479, 1.0);
        }

        s.b[1126] = (s.v[339] <= 1.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        s.b[1127] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if ((((!s.b[735]) && s.b[1087]) && s.b[1126]) && s.b[1127]) {
            s.store_sub_ad(436, A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));
            s.store_div(436, 436, 246);
        }

        if ((((!s.b[735]) && s.b[1087]) && s.b[1126]) && (!s.b[1127])) {
            s.copy_ad(436, 425);
        }

        if (((!s.b[735]) && s.b[1087]) && (!s.b[1126])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.b[1131] = (s.v[612] == 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if s.b[1131] {
            s.store_offset(480, 190, 0.5);
            s.store_mul(481, 479, 478);
            s.store_div_scaled_inputs_indices(482, 480, 0.4, 481, 1.0);
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.b[1132] = (s.v[438] > (0.5 + 1e-8));
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1132]) {
            s.store_scalar(438, 0.5);
        }

        if s.b[1131] {
            s.copy_ad(439, 438);
            s.store_scalar(438, 0.5);
        }

        s.b[1134] = (s.v[145] == 0.0);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        s.b[1150] = ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16)));
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1134]) && s.b[1150]) {
            s.store_scalar(316, 0.0);
            s.copy_ad(314, 162);
        }

        s.b[1151] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && s.b[1150]) && s.b[1151]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_scalar(1149, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_div_from_scalar(1135, 1.0, 1149);
            s.store_mul(1136, 244, 1135);
            s.store_scale(1137, 1136, p.p191);
            s.store_add_scaled_product_indices(1140, 1137, 1.0, 80, 229, 1.0);
            s.store_div_from_scalar(1136, 1.0, 1140);
            s.store_scale(1139, 1136, 1.034943e-10);
            s.store_scalar(1136, (1.0 - p.p189));
            s.store_add_scaled_inputs_product_indices(314, 157, p.p189, 161, p.p189, 1136, 162, 1.0);
        }

        s.b[1152] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1152]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_sub(1142, 314, 162);
            s.store_sqrt_square_offset(44, 1142, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1141, 1142, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1153] = (s.v[1141] < 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1153]) {
            s.store_scalar(1141, 0.0);
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_mul(1138, 225, 244);
            s.store_div_from_scalar(1136, 1.0, 1138);
            s.store_mul(1140, 246, 1136);
        }

        s.b[1154] = (s.v[1140] < s.v[227]);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1154]) {
            s.copy_ad(1140, 227);
        }

        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {
            s.store_scale(1146, 229, 9662367879.197212);
            s.store_scalar(1136, (100000.0 * 10000.0));
            s.store_scalar(1137, (1.0 / s.v[97]));
            s.store_mul_ad_lhs(1148, A::add_scaled_inputs_product(s.ad_value(1140), 2.0, A::mul3_scaled_output(s.ad_value(1146), s.ad_value(1141), s.ad_value(1139), 2.0), 1.0, s.ad_value(1136), s.ad_value(1139), 1.0), 1137);
            s.store_mul(1143, 1148, 1139);
            s.store_add_scaled_product_indices(1147, 1136, 4.0, 1146, 1141, (2.0 * 4.0));
            s.store_mul3_lhs(1144, 1147, 1139, 1139);
            s.store_sqrt_square_add(1145, 1143, 1144);
            s.store_mul_sub_scaled_inputs_rhs(316, 326, s.ad_value(1145), 0.5, s.ad_value(1143), 0.5);
        }

        if (s.b[1131] && s.b[1134]) {
            s.store_scale(316, 316, s.v[127]);
        }

        if s.b[1131] {
            s.store_sub_from_scalar(441, s.v[97], 316);
        }

        s.b[1155] = (s.v[441] < 1e-9);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1155]) {
            s.store_scalar(441, 1e-9);
        }

        if s.b[1131] {
            s.store_scale(328, 108, (-s.v[98]));
            s.store_mul(196, 328, 437);
            s.store_mul(197, 328, 436);
            s.store_mul(198, 197, 438);
        }

        s.b[1156] = (p.p43 == 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1156]) {
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_mul_scale_ad_lhs(392, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 108);
        }

        if s.b[1131] {
            s.store_scaled_sub(1157, 157, 164, 0.5);
            s.store_scale(44, 1157, (2.0 * 1.0 / (p.p227)));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.b[1158] = (s.v[177] < (10.0 * 2.220446049250313e-16));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1158]) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if s.b[1131] {
            s.store_add(176, 161, 177);
            s.store_scalar(1168, (1.034943e-10 / 100.0));
            s.store_scale(1169, 437, 0.0001);
            s.store_scale(1170, 436, 0.0001);
            s.store_div_from_scalar(1159, p.p92, 1168);
            s.store_div_from_scalar(1160, p.p93, 1168);
            s.store_scalar(1161, p.p94);
            s.store_offset_mul_ad(1162, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1161), 1.0);
            s.store_add_scaled_products_indices(1163, 1159, 1169, 1.0, 1160, 1170, 1.0);
            s.store_div(1164, 1163, 1162);
            s.copy_ad(248, 1164);
            s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));
            s.store_offset_add_scaled_inputs_indices(1161, 248, 0.5, 44, 0.5, (1e-10 * 3000.0));
        }

        s.b[1171] = (s.v[1161] < 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1171]) {
            s.store_scalar(1161, 0.0);
        }

        if s.b[1131] {
            s.store_powf(1163, 1161, (p.p97 - 1.0));
            s.store_mul(1165, 1163, 1161);
            s.store_powf(1166, 1161, (s.v[111] - 1.0));
            s.store_mul(1167, 1166, 1161);
            s.store_scale(249, 1170, 6.241449993689894e18);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1131] {
            s.store_add_scaled_ad_lhs(1159, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p.p96 * 1e-11), p.p95)), 1.0, s.ad_value(543), s.ad_value(1165), 1.0), 1167, 1.0 / (p.p106));
            s.store_div_from_scalar(251, 1.0, 1159);
            s.store_scale(251, 251, 0.0001);
            s.store_mul3_lhs(1172, 225, 244, 441);
            s.store_sqrt_square_offset(44, 1172, ((4.0 * 1e-50) * 1e-50));
            s.store_offset_add_scaled_inputs_indices(1172, 1172, 0.5, 44, 0.5, (1e-10 * 1e-50));
        }

        s.b[1180] = (s.v[1172] < 0.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1180]) {
            s.store_scalar(1172, 0.0);
        }

        if s.b[1131] {
            s.store_div_from_scalar(1173, 1.0, 1172);
            s.store_mul(1174, 246, 1173);
            s.store_div_scaled_inputs_indices(1172, 253, 0.2, 251, 1.0);
            s.store_sqrt_square_sum(252, 1174, 1172);
            s.store_mul(1175, 251, 252);
            s.store_div(1173, 1175, 253);
        }

        s.b[1181] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1181]) {
            s.store_scalar(1176, 1.0);
        }

        s.b[1182] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if ((s.b[1131] && (!s.b[1181])) && s.b[1182]) {
            s.copy_ad(1176, 1173);
        }

        if ((s.b[1131] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_powf(1176, 1173, (p.p113 - 1.0));
        }

        if s.b[1131] {
            s.store_mul(1172, 1173, 1176);
            s.store_offset(1177, 1172, 1.0);
        }

        s.b[1183] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1183]) {
            s.store_div_from_scalar(1178, 1.0, 1177);
        }

        s.b[1184] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1131] && (!s.b[1183])) && s.b[1184]) {
            s.store_div_from_scalar_sqrt_ad(1178, 1.0, s.ad_value(1177));
        }

        if ((s.b[1131] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_powf(1179, 1177, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1178, 1177, 1179);
        }

        if s.b[1131] {
            s.store_mul(250, 251, 1178);
            s.store_div_scaled_product_denominator_ad(264, 107, 227, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0);
            s.store_mul3_lhs(200, 264, 246, 250);
            s.store_scalar(201, 0.0);
        }

        s.b[1194] = ((p.p281 > 0.0) && (p.p244 != 0.0));
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1194]) {
            s.store_scaled_sub(1185, 157, 164, 0.5);
            s.store_scale(44, 1185, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(1191, 0.01, 45);
            s.store_sub_from_scalar_ad(1185, 1.1, A::add(s.ad_value(161), s.ad_value(1191)));
            s.store_sqrt_square_offset(44, 1185, ((4.0 * 0.05) * 0.05));
            s.store_offset_add_scaled_inputs_indices(1193, 1185, 0.5, 44, 0.5, (1e-10 * 0.05));
        }

        s.b[1195] = (s.v[1193] < 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1194]) && s.b[1195]) {
            s.store_scalar(1193, 0.0);
        }

        if (s.b[1131] && s.b[1194]) {
            s.store_scale(1186, 225, s.v[116]);
            s.store_mul(1187, 323, 1186);
            s.store_powf(1186, 1193, p.p245);
            s.store_mul(1188, 1187, 1186);
            s.store_offset_scaled(1189, 173, p.p246, 1.0);
            s.store_scalar(1186, s.v[117]);
        }

        s.b[1196] = ((s.v[56] < 3.0) || (p.p43 == 1.0));
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1194]) && s.b[1196]) {
            s.store_add_scaled_inputs3_indices(1190, 161, 1.0, 1191, 1.0, 172, -1.0);
        }

        if ((s.b[1131] && s.b[1194]) && (!s.b[1196])) {
            s.store_add_scaled_inputs3_indices(1190, 161, 1.0, 1191, 1.0, 350, -1.0);
        }

        if (s.b[1131] && s.b[1194]) {
            s.store_add_ad_rhs(1189, 1189, A::mul3(s.ad_value(173), s.ad_value(1186), s.ad_value(1190)));
            s.store_mul(1191, 1188, 1189);
            s.copy_ad(1188, 1191);
        }

        if (s.b[1131] && (!s.b[1194])) {
            s.store_scalar(1188, 0.0);
        }

        s.b[1197] = (p.p248 != 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1197]) {
            s.store_scale(1185, 225, s.v[118]);
            s.store_mul(1193, 323, 1185);
            s.store_mul(1192, 1193, 173);
        }

        if (s.b[1131] && (!s.b[1197])) {
            s.store_scalar(1192, 0.0);
        }

        s.b[1198] = ((s.v[1188] + s.v[1192]) > 0.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1198]) {
            s.store_mul_add_rhs(247, 164, 1188, 1192);
            s.store_mul3_lhs(201, 264, 247, 250);
        }

        if s.b[1131] {
            s.store_add(199, 200, 201);
            s.copy_ad(203, 201);
        }

        s.b[1208] = (p.p33 != 0.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1208]) {
            s.copy_ad(1201, 549);
            s.store_scalar(1202, (s.v[124] - p.p71));
            s.store_div_from_scalar_square_ad(1203, 1.0, s.ad_value(1202));
            s.store_mul_ad_product_lhs(1204, A::mul_sub_from_scalar_lhs_scaled_output(p.p69, s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), s.ad_value(1201), 1203);
            s.store_mul(186, 1204, 235);
            s.store_offset_scaled(1200, 173, p.p155, p.p154);
            s.store_mul(206, 186, 1200);
            s.store_sub_from_scalar_scaled_input(1199, p.p156, 157, p.p157);
            s.store_add_scaled_inputs3_offset_indices(207, 174, 1.0, 1199, 1.0, 206, 1.0, (-s.v[123]));
            s.store_mul3_lhs(210, 205, 324, 324);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_offset_sub_ad(1205, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p.p156)))), s.ad_value(206), 1e-50);
            s.store_offset_sub(1199, 174, 1205, (-0.005));
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_scalar(327, (if (s.v[1205] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_add_scaled_square_product(1201, 1199, 1.0, 327, 1205, (4.0 * 0.005));
            s.store_sub_ad_lhs(1202, A::add_scaled_inputs4_offset(s.ad_value(1205), 1.0, s.ad_value(1199), 0.5, s.ad_value(1201), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p.p156))), 514);
            s.store_offset_mul(1203, 225, 1202, (-1.0));
            s.store_div_from_scalar(1204, 4.0, 212);
            s.store_offset_mul(1200, 1203, 1204, 1.0);
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1209] = (s.v[1199] < 0.0);
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1209]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_offset_input(213, 1199, 1e-50);
            s.store_add_ad_rhs(215, 207, A::mul_sub_from_scalar_rhs(s.ad_value(211), 1.0, s.ad_value(213)));
            s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));
            s.store_mul_ln_ad_lhs(216, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))), 327);
            s.store_div_scaled_value_offset_denominator(1202, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);
            s.store_offset_sub(217, 216, 215, (-0.002));
            s.store_sqrt_add_scaled_square_input(327, 217, 1.0, 216, (4.0 * 0.002));
            s.store_add_scaled_inputs3_indices(218, 216, 1.0, 217, (-0.5), 327, (-0.5));
            s.store_div_from_scalar(1199, 1.0, 327);
            s.store_mul_exp_ad_rhs(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));
            s.store_add_offset_ad_lhs(1200, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0), 327);
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1210] = (s.v[1199] < 0.0);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1210]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_offset_input(219, 1199, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(1200, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1211] = (s.v[1199] < 0.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1211]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_sqrt_offset_input(220, 1199, (10.0 * 2.220446049250313e-16));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_sub(1200, 215, 218);
            s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1212] = (s.v[1199] < 0.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1208]) && s.b[1212]) {
            s.store_scalar(1199, 0.0);
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_div_scaled_value_offset_denominator(1206, s.ad_value(157), 1.0, s.ad_value(1199), (10.0 * 2.220446049250313e-16), 1.0);
            s.store_square(49, 1206);
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
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1131] && s.b[1208]) {
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1213] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        s.b[1214] = (4.0 == 1.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (((s.b[1131] && s.b[1208]) && s.b[1213]) && s.b[1214]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1215] = (4.0 == 2.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && s.b[1215]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1216] = (4.0 == 4.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && s.b[1216]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1217] = (4.0 == 8.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1216])) && s.b[1217]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1131] && s.b[1208]) && s.b[1213]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19470_loop_guard: usize = 0;
        while {
            let assign19470_cond_e26967: f64 = if (((s.b[1131] && s.b[1208]) && s.b[1213]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19470_cond_e26967 != 0.0
        } {
            assign19470_loop_guard += 1;
            assert!(assign19470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1131] && s.b[1208]) && s.b[1213]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1131] && s.b[1208]) && (!s.b[1213])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (s.b[1131] && s.b[1208]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(1207, 1206, 53, 1.0);
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
            s.store_div_scaled_product_left_ad(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1207, 1.0, 441, 1.0);
            s.store_add(199, 199, 222);
        }

        s.b[1218] = ((p.p30 != 0.0) && (p.p32 != 0.0));
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1218]) {
            s.store_square(294, 192);
            s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);
            s.store_sub(296, 294, 295);
            s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(294, 294, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1219] = (s.v[294] < 0.0);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1218]) && s.b[1219]) {
            s.store_scalar(294, 0.0);
        }

        if (s.b[1131] && s.b[1218]) {
            s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(296, 296, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1220] = (s.v[296] < 0.0);
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1218]) && s.b[1220]) {
            s.store_scalar(296, 0.0);
        }

        if (s.b[1131] && s.b[1218]) {
            s.store_sub(297, 294, 296);
        }

        s.b[1221] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if ((s.b[1131] && s.b[1218]) && s.b[1221]) {
            s.store_scalar(146, 0.0);
        }

        if ((s.b[1131] && s.b[1218]) && (!s.b[1221])) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.b[1222] = ((p.p281 > 0.0) && (p.p285 > 0.0));
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        if s.b[1222] {
            s.store_scalar(1229, s.v[99]);
            s.store_scalar(1233, p.p237);
            s.store_offset_add_scaled_inputs3_offset_indices(1234, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]), (-p.p286));
            s.store_offset(1235, 182, p.p286);
            s.store_scalar(1237, p.p285);
            s.store_scalar(1236, p.p283);
            s.store_scalar(1227, s.v[70]);
            s.store_mul_ln_ad_rhs(1228, 227, A::div_scaled_product_by_product(s.ad_value(1227), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));
        }

        if s.b[1222] {
            if (p.p43 == 1.0) {
                s.copy_ad(1225, 435);
            } else {
                s.copy_ad(1225, 350);
            }
        }

        if s.b[1222] {
            s.store_sqrt_ad(1230, A::div_scaled_product3(A::sub(s.ad_value(1228), s.ad_value(1225)), s.ad_value(536), s.ad_value(1227), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1227)), 1.0));
            s.store_mul(1224, 1230, 1229);
            s.store_div_scaled_product_add_scaled_denominator_indices(1223, 1224, 1224, (-0.25), 157, 1.0, 1224, 1.0, 1.0);
            s.copy_ad(1249, 1223);
            s.copy_ad(1250, 1235);
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(1234), s.ad_value(1249))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if s.b[1222] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if s.b[1222] {
            s.store_add_ad_rhs(376, 1234, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
        }

        s.b[1251] = (s.v[158] < ((s.v[123] + s.v[1250]) * 0.5));
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1251]) {
            s.store_scalar(144, 0.0);
        }

        s.b[1252] = ((s.v[144] == 0.0) || (1.0 != 0.0));
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1252]) {
            s.store_mul_sub_rhs(181, 225, 376, 1249);
        }

        s.b[1253] = (s.v[181] < 3.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((s.b[1222] && s.b[1252]) && s.b[1253]) {
            s.store_mul_sub_rhs(337, 225, 1234, 1249);
            s.store_div_from_scalar_scaled_mul(328, 1.0, 225, 240, (1.414213562373095 / 108.0));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 1249, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1254] = ((s.v[158] - s.v[383]) <= s.v[1250]);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        s.b[1255] = (p.p43 == 0.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && s.b[1252]) && (!s.b[1253])) && s.b[1254]) && s.b[1255]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1233, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1234, 331, 323);
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && s.b[1254]) {
            s.copy_ad(378, 376);
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1234), s.ad_value(383)), A::sub(s.ad_value(1234), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        s.b[1256] = (p.p43 == 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        s.b[1257] = ((s.v[158] - s.v[383]) <= s.v[1250]);
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((s.b[1222] && s.b[1252]) && s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1233, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1234, 331, 323);
            s.copy_ad(378, 376);
        }

        if (((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1233, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1234, 331, 323);
            s.copy_ad(378, 376);
        }

        s.b[1258] = ((s.v[1234] - s.v[383]) > 0.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1234), s.ad_value(383)), A::sub(s.ad_value(1234), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
        }

        s.b[1259] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
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
            s.store_mul(52, 52, 50);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1260] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        s.b[1261] = (2.0 == 1.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && s.b[1261]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1262] = (2.0 == 2.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if ((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && s.b[1262]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1263] = (2.0 == 4.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) && s.b[1263]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1264] = (2.0 == 8.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) && (!s.b[1263])) && s.b[1264]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20700_loop_guard: usize = 0;
        while {
            let assign20700_cond_e28539: f64 = if (((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20700_cond_e28539 != 0.0
        } {
            assign20700_loop_guard += 1;
            assert!(assign20700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && (!s.b[1260])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && (!s.b[1259])) {
            s.copy_ad(378, 376);
        }

        if s.b[1222] {
            s.store_offset(336, 1249, (5e-12 / 2.0));
        }

        s.b[1265] = (s.v[378] < s.v[336]);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1265]) {
            s.copy_ad(378, 336);
        }

        if s.b[1222] {
            s.copy_ad(1232, 378);
            s.copy_ad(163, 376);
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            if ((s.v[376] - s.v[1232]) >= 0.0) {
                s.store_sub(166, 376, 1232);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p.p287)) + ((-0.03))));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[1266] = (s.v[165] < 0.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((s.b[1222] && (0.0 != 0.0)) && s.b[1266]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1267] = (s.v[165] > s.v[157]);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if (((s.b[1222] && (0.0 != 0.0)) && (!s.b[1266])) && s.b[1267]) {
            s.copy_ad(165, 157);
        }

        if (s.b[1222] && (0.0 != 0.0)) {
            s.store_add(163, 1232, 165);
        }

        s.b[1268] = (p.p282 == 1.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1268]) {
            s.copy_ad(378, 1232);
            s.copy_ad(1269, 1223);
            s.store_offset_add_scaled_inputs3_offset_indices(160, 185, (-1.0), 320, 1.0, 1269, 1.0, s.v[123], p.p286);
        }

        s.b[1271] = (s.v[158] < s.v[160]);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((s.b[1222] && s.b[1268]) && s.b[1271]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));
            s.store_mul_sub_rhs(336, 225, 1234, 1269);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1272] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (((s.b[1222] && s.b[1268]) && s.b[1271]) && s.b[1272]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((s.b[1222] && s.b[1268]) && s.b[1271]) && (!s.b[1272])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);
        }

        if ((s.b[1222] && s.b[1268]) && s.b[1271]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 1269, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 1269);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_div_lhs_indices(1232, 328, 330, 1269);
        }

        if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
            s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1269), (-p.p287)));
            s.store_scalar(430, 0.0);
            s.copy_ad(1270, 378);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ln_lhs(420, 328, 419);
            s.store_scalar(167, 1.0);
        }

        let mut assign21300_loop_guard: usize = 0;
        while {
            let assign21300_cond_e29269: f64 = (s.v[57] + 1.0);
            let assign21300_cond_e29271: f64 = if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (s.v[167] <= assign21300_cond_e29269)) { 1.0 } else { 0.0 };
            assign21300_cond_e29271 != 0.0
        } {
            assign21300_loop_guard += 1;
            assert!(assign21300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_sub(417, 1270, 1269);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1273] = (s.v[337] < 80.0);
            s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1273]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ln_offset_lhs(422, 329, 1.0, 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1273])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_mul(421, 225, 422);
            }
            s.b[1274] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1274]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1275] = (s.v[181] < 0.0);
            s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1274]) && s.b[1275]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1276] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1274])) && s.b[1276]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1274])) && (!s.b[1276])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1277] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1277]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1278] = (s.v[181] < 0.0);
            s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1278]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1279] = (s.v[181] < 1e-7);
            s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && s.b[1279]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && (!s.b[1279])) {
                s.store_mul_offset_rhs(501, 225, 1270, (-p.p287));
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_add_scaled_inputs_product_indices(492, 1270, 1.0, 1234, (-1.0), 240, 490, 1.0);
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1280] = (s.v[430] == 1.0);
            s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1280]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {
                s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1270]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1270))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1281] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) && s.b[1281]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {
                s.store_add(1270, 1270, 494);
            }
            s.b[1282] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) && s.b[1282]) {
                s.store_scalar(430, 1.0);
            }
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
            s.copy_ad(1232, 1270);
        }

        if s.b[1222] {
            s.store_mul_sub_scaled_inputs_rhs(332, 225, s.ad_value(1232), -1.0, s.ad_value(1223), -1.0);
        }

        if s.b[1222] {
            s.store_scalar(1247, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[1222] {
            s.store_mul(1248, 1247, 332);
            s.store_exp(333, 332);
            s.store_sub_offset_lhs(334, 333, (-1.0), 332);
        }

        s.b[1283] = (s.v[332] > 1e-7);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1283]) {
            s.store_mul_scaled_sqrt_rhs(437, 238, -1.0, 334);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1284] = (s.v[1248] > 1e-7);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if ((s.b[1222] && (!s.b[1283])) && s.b[1284]) {
            s.store_mul_sqrt_rhs(437, 238, 334);
        }

        if ((s.b[1222] && (!s.b[1283])) && (!s.b[1284])) {
            s.store_mul_ad_affine_product_rhs(437, 1247, s.ad_value(1248), A::sqrt_scaled_lhs_product_offset(s.ad_value(1248), 0.3333333333333333, A::scale_offset(s.ad_value(1248), 0.25, 1.0), 1.0), (-0.7071067811865475), 0.0);
        }

        if s.b[1222] {
            s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_add_scaled_inputs_indices(1244, 437, 0.5, 44, 0.5, (1e-10 * 1e-6));
        }

        s.b[1285] = (s.v[1244] < 0.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1285]) {
            s.store_scalar(1244, 0.0);
        }

        if s.b[1222] {
            s.store_div_scaled_inputs_indices(1245, 1244, 1.0, 536, 1.6021918e-19);
            s.store_sub(328, 1245, 1236);
            s.store_scale(1246, 1245, 0.01);
            s.store_sqrt_add_scaled_square_product(44, 328, 1.0, 1246, 1246, 4.0);
            s.store_add_scaled_inputs3_indices(329, 328, 0.5, 44, 0.5, 1246, 1e-10);
        }

        s.b[1286] = (s.v[329] < 0.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1286]) {
            s.store_scalar(329, 0.0);
        }

        if s.b[1222] {
            s.store_div_scaled_product_by_product(1243, s.ad_value(329), s.ad_value(329), 1.0, s.ad_value(1245), s.ad_value(1245), 1.0);
            s.store_add_scaled_product_left_ad(1226, 1223, 1.0, A::sub(s.ad_value(1232), s.ad_value(1223)), 1243, 1.0);
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1226))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1226), s.ad_value(157)))));
            s.store_sqrt_scaled_input(1239, 1227, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_mul_sqrt_rhs(1240, 1239, 227);
            s.store_mul_sub_rhs(1231, 225, 1226, 1223);
        }

        s.b[1287] = ((s.v[1231] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1287]) {
            s.store_sub_scaled_inputs(44, 225, 0.2, 1231, 1.0);
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

        s.b[1288] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        s.b[1289] = (1.0 == 1.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (((s.b[1222] && s.b[1287]) && s.b[1288]) && s.b[1289]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1290] = (1.0 == 2.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && s.b[1290]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1291] = (1.0 == 4.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && s.b[1291]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1292] = (1.0 == 8.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if ((((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && (!s.b[1291])) && s.b[1292]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21830_loop_guard: usize = 0;
        while {
            let assign21830_cond_e30586: f64 = if (((s.b[1222] && s.b[1287]) && s.b[1288]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21830_cond_e30586 != 0.0
        } {
            assign21830_loop_guard += 1;
            assert!(assign21830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1222] && s.b[1287]) && (!s.b[1288])) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if (s.b[1222] && s.b[1287]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);
            s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);
        }

        if (s.b[1222] && (!s.b[1287])) {
            s.copy_ad(328, 1231);
        }

        if s.b[1222] {
            s.store_sqrt_offset_input(1241, 328, (10.0 * 2.220446049250313e-16));
            s.store_mul(1242, 1240, 1241);
            s.store_mul_div_scaled_inputs_indices(1238, 1242, 227, 2.0, 1229, 1.0);
            s.store_mul_product3_indices(204, 337, 1238, 1237, 107, 1.0);
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.b[1293] = ((p.p43 == 1.0) || (p.p45 == 1.0));
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        s.b[1306] = ((s.v[145] == 1.0) || (p.p25 == 0.0));
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (s.b[1293] && s.b[1306]) {
            s.store_scalar(263, 0.0);
        }

        s.b[1307] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((s.b[1293] && (!s.b[1306])) && s.b[1307]) {
            s.store_scalar(263, 0.0);
        }

        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {
            s.store_offset_add_scaled_inputs3_offset_indices(445, 174, 1.0, 185, 1.0, 320, -1.0, (-s.v[136]), p.p48);
        }

        s.b[1308] = (p.p44 <= 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {
            s.copy_ad(1294, 445);
            s.store_square(1301, 323);
            s.copy_ad(1302, 545);
            s.store_div(1296, 1302, 1301);
            s.store_div_from_scalar(1303, 2.0, 1302);
            s.store_mul(1297, 1303, 1301);
            s.store_add_scaled_inputs_product_indices(1298, 1294, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1298, 1298, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1300, 1297, 1298, 1.0);
            s.store_sqrt_square_offset(44, 1300, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1299, 1300, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1309] = (s.v[1299] < 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1309]) {
            s.store_scalar(1299, 0.0);
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {
            s.store_offset(1299, 1299, 1e-50);
            s.store_sqrt(1299, 1299);
            s.store_add_scaled_product_value_ad(1304, A::mul_sub_from_scalar_rhs(s.ad_value(1296), 1.0, s.ad_value(1299)), 1.0, 1294, 137, 1.0);
            s.store_add_scaled_inputs3_mixed_iia(1305, 173, p.p122, 176, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1304)), -1.0);
            s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1305, 1305, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1310] = (s.v[1305] < 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1310]) {
            s.store_scalar(1305, 0.0);
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {
            s.store_mul(1294, 134, 445);
            s.store_div_square_rhs(1296, 545, 323);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1297, 2.0, 545, A::square(s.ad_value(323)));
            s.store_add_scaled_inputs_product_indices(1298, 1294, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1298, 1298, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1299, 1297, 1298, 1.0);
            s.store_scaled_offset(1301, 1297, 1.0, 2.0);
        }

        s.b[1311] = ((s.v[1299] < (1e-50 + s.v[1301])) && (s.v[1301] >= 0.0));
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
            s.store_sub_offset_lhs(44, 1301, 1e-50, 1299);
            s.store_square(49, 44);
            s.store_square(50, 1301);
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

        s.b[1312] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        s.b[1313] = (4.0 == 1.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && s.b[1313]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1314] = (4.0 == 2.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && s.b[1314]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1315] = (4.0 == 4.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && s.b[1315]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1316] = (4.0 == 8.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && (!s.b[1315])) && s.b[1316]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22620_loop_guard: usize = 0;
        while {
            let assign22620_cond_e31705: f64 = if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22620_cond_e31705 != 0.0
        } {
            assign22620_loop_guard += 1;
            assert!(assign22620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && (!s.b[1312])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 1301, 53);
            s.store_sub_offset_lhs(1299, 1301, 1e-50, 43);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && (!s.b[1311])) {
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {
            if (s.v[1299] <= 0.0) {
                s.store_scalar(1299, 0.0);
            } else {
                s.store_sqrt(1299, 1299);
            }
        }

        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {
            s.store_add_ad_rhs(1304, 1294, A::mul_sub_from_scalar_rhs(s.ad_value(1296), 1.0, s.ad_value(1299)));
            s.store_div_from_scalar_offset_input(1295, s.v[100], 131, s.v[100]);
            s.store_add_scaled_inputs_product_indices(1305, 173, p.p122, 176, 1.0, 1295, 1304, (-1.0));
            s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1305, 1305, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1317] = (s.v[1305] < 0.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1317]) {
            s.store_scalar(1305, 0.0);
        }

        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {
            s.store_offset(1305, 1305, 1e-50);
            s.store_ad_value(1295, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1305), 1.0));
            s.store_mul_product3_indices(263, 1295, 132, 1305, 199, 1.0);
        }

        s.b[1318] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if s.b[1318] {
            s.store_scale(1322, 227, 0.0);
            s.store_add_scaled_inputs3_indices(44, 231, 1.0, 1322, (-1.0), 231, (-0.01));
            s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));
        }

        if s.b[1318] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1318] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(1322, 231, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sqrt_mul_scaled_lhs(1323, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);
            s.store_mul_sub_rhs(1324, 225, 176, 1322);
        }

        if s.b[1318] {
            if (s.v[1324] > 0.0) {
                s.store_sqrt(1324, 1324);
            } else {
                s.store_scaled_sqrt_scaled_input(1324, 1324, -1.0, -1.0);
            }
        }

        if s.b[1318] {
            s.store_sqrt_mul(1325, 225, 176);
            s.store_mul_sub_scaled_inputs_rhs(1326, 1323, s.ad_value(1324), -1.0, s.ad_value(1325), -1.0);
            s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1326), (-(p.p47 * 0.01)));
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if s.b[1318] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1318] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p.p47);
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
        }

        s.b[1340] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        s.b[1341] = (s.v[56] < 3.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (s.b[1340] && s.b[1341]) {
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
        }

        if (s.b[1340] && (!s.b[1341])) {
            if (p.p43 == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }

        if (s.b[1340] && (!s.b[1341])) {
            if (p.p43 == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }

        if s.b[1340] {
            s.store_offset_scaled(1327, 185, p.p147, 1.0);
            s.store_scaled_mul(1328, 1327, 263, p.p146);
            s.store_offset_mul_ad(1329, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));
            s.store_sqrt_square_offset(44, 1329, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1329, 1329, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1342] = (s.v[1329] < 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (s.b[1340] && s.b[1342]) {
            s.store_scalar(1329, 0.0);
        }

        if s.b[1340] {
            s.store_sqrt(1330, 1329);
            s.store_mul(1331, 1329, 1330);
            s.store_offset_mul_ad(1332, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));
            s.store_sqrt_square_offset(44, 1332, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1332, 1332, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1343] = (s.v[1332] < 0.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if (s.b[1340] && s.b[1343]) {
            s.store_scalar(1332, 0.0);
        }

        if s.b[1340] {
            s.store_sqrt(1333, 1332);
            s.store_mul(1334, 1332, 1333);
            s.store_div_from_scalar(1335, 1.0, 1329);
            s.store_mul3_lhs(328, 225, 1328, 1335);
            s.store_div_from_scalar(1335, 1.0, 1332);
            s.store_mul3_lhs(1336, 225, 1328, 1335);
            s.store_mul_ad_rhs(1337, 238, A::add_scaled_products(s.ad_value(1334), s.ad_value(1336), 1.0, s.ad_value(1331), s.ad_value(328), (-1.0)));
            s.store_mul_add_scaled_products_indices_rhs(1338, 238, 1333, 1336, ((-1.0) * (0.5)), 1330, 328, 0.5);
            s.store_add(1339, 1337, 1338);
            s.store_mul3_lhs(265, 264, 1339, 250);
        }

        s.v[1357] = (s.v[88] * 100.0);

        s.store_scale(1358, 323, 0.0001);

        s.v[1359] = (s.v[97] * 100.0);

        s.store_scale(1360, 107, 100.0);

        s.store_scale(1361, 252, 0.01);

        s.store_scale(1362, 436, 0.0001);

        s.store_scale(1363, 238, 0.0001);

        s.b[1364] = (p.p27 == 0.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        s.b[1365] = (s.v[145] == 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && s.b[1365]) {
            s.store_offset_add(1356, 176, 173, (-(10.0 * 2.220446049250313e-16)));
            s.store_add_scaled_inputs4_offset_indices(1346, 174, 1.0, 185, (p.p216 * s.v[1359]), 320, (-(p.p216 * s.v[1359])), 1356, (-p.p215), (-s.v[123]));
            s.store_scalar(1348, (1.0 / s.v[1357]));
            s.store_mul(1347, 1346, 1348);
            s.store_scalar(1348, (1.0 / p.p217));
            s.store_offset_mul(1352, 1361, 1348, 1.0);
            s.store_mul(1355, 1347, 1352);
            s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1355, 1355, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1366] = (s.v[1355] < 0.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if (((!s.b[1364]) && s.b[1365]) && s.b[1366]) {
            s.store_scalar(1355, 0.0);
        }

        if ((!s.b[1364]) && s.b[1365]) {
            s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1348, 174, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1367] = (s.v[1348] < 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if (((!s.b[1364]) && s.b[1365]) && s.b[1367]) {
            s.store_scalar(1348, 0.0);
        }

        if ((!s.b[1364]) && s.b[1365]) {
            s.store_offset(1348, 1348, (-p.p226));
            s.store_scale(1344, 1348, 10.0);
            s.store_offset_square(1347, 1344, 1.0);
            s.store_sub_from_scalar_ad(1346, 1.0, A::div_from_scalar(1.0, s.ad_value(1347)));
            s.store_mul(1355, 1355, 1346);
            s.store_scale(1345, 1360, s.v[1359]);
            s.store_div_from_scalar_offset_input(1352, p.p219, 1345, p.p219);
            s.store_scalar(1351, p.p218);
            s.store_div_from_scalar_offset_input(1349, 1.0, 1355, 1e-50);
            s.store_scaled_mul(1346, 303, 1349, (-p.p214));
        }

        s.b[1368] = (s.v[1346] < (-34.0));
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((!s.b[1364]) && s.b[1365]) && (!s.b[1368])) {
            s.store_exp(1347, 1346);
            s.store_mul_scale_ad_lhs(1348, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 1345);
            s.store_div_from_scalar(1350, 1.0, 1363);
            s.store_sqrt_mul_ad(1351, A::add_scaled_inputs(s.ad_value(1362), 1.0, s.ad_value(1358), 1e-12), s.ad_value(1350));
            s.store_mul3_lhs(1349, 1347, 1348, 1351);
        }

        if (!s.b[1364]) {
            s.store_offset_scaled(1345, 158, (-p.p221), p.p222);
            s.store_exp_scaled_input(1347, 1345, s.v[1357]);
            s.store_scale(1345, 158, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));
            s.store_mul(1348, 158, 1345);
            s.store_scale(1349, 1360, (p.p220 / 1000000.0));
            s.store_sub(1346, 158, 157);
            s.store_offset_scaled(1345, 1346, (-p.p221), p.p222);
            s.store_exp_scaled_input(1347, 1345, s.v[1357]);
            s.store_scale(1345, 1346, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));
            s.store_mul(1348, 1346, 1345);
            s.store_scale(1349, 1360, (p.p220 / 1000000.0));
            s.store_offset_scaled_sub(1355, 513, 158, 1.0 / (s.v[1357]), ((((s.v[123]) + (p.p225))) * (1.0 / (s.v[1357]))));
            s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1355, 1355, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1371] = (s.v[1355] < 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && s.b[1371]) {
            s.store_scalar(1355, 0.0);
        }

        if (!s.b[1364]) {
            s.store_offset(1355, 1355, 1e-50);
            s.store_div_from_scalar(1346, (-p.p224), 1355);
        }

        s.b[1372] = (s.v[1346] < (-34.0));
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if ((!s.b[1364]) && (!s.b[1372])) {
            s.store_exp(1347, 1346);
            s.store_scale(1348, 1360, (p.p223 * s.v[1359]));
        }

        s.b[1380] = (p.p28 == 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if (!s.b[1380]) {
            s.store_add_scaled_inputs4_offset_indices(1373, 157, p.p209, 158, (-1.0), 187, p.p211, 319, p.p211, (p.p210 * p.p209));
            s.store_scalar(1374, (1.0 / s.v[88]));
            s.store_mul(1375, 1373, 1374);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1380]) {
            s.store_sqrt_square_offset(44, 1375, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(304, 1375, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1381] = (s.v[304] < 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && s.b[1381]) {
            s.store_scalar(304, 0.0);
        }

        if (!s.b[1380]) {
            s.store_div_from_scalar_offset_input(1376, 1.0, 304, 1e-50);
            s.store_scaled_mul(1377, 303, 1376, (-p.p208));
        }

        s.b[1382] = (s.v[1377] < (-34.0));
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && (!s.b[1382])) {
            s.store_exp(1373, 1377);
            s.store_mul_scale_ad_lhs(1374, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 107);
        }

        if (!s.b[1380]) {
            s.store_sub(1379, 157, 513);
        }

        s.b[1383] = (s.v[1379] > 0.0);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && s.b[1383]) {
            s.store_square(1374, 1379);
            s.store_mul(331, 1374, 1379);
            s.store_offset(1377, 331, p.p212);
        }

        s.b[1391] = (p.p28 == 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if (!s.b[1391]) {
            s.store_add_scaled_inputs3_mixed_aii(1384, A::add_scaled_inputs3_offset(s.ad_value(157), (-p.p209), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p.p210) * (p.p209))), 1.0, 187, p.p211, 319, p.p211);
            s.store_scalar(1385, (1.0 / s.v[88]));
            s.store_mul(1386, 1384, 1385);
            s.store_sqrt_square_offset(44, 1386, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(305, 1386, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1392] = (s.v[305] < 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if ((!s.b[1391]) && s.b[1392]) {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[1391]) {
            s.store_div_from_scalar_offset_input(1387, 1.0, 305, 1e-50);
            s.store_scaled_mul(1388, 303, 1387, (-p.p208));
        }

        s.b[1393] = (s.v[1388] < (-34.0));
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if ((!s.b[1391]) && (!s.b[1393])) {
            s.store_exp(1384, 1388);
            s.store_div_from_scalar(1387, 1.0, 302);
            s.store_scaled_mul(1385, 1387, 107, (p.p207 * 1.6021918e-19));
        }

        if (!s.b[1391]) {
            s.store_neg(1390, 513);
        }

        s.b[1394] = (s.v[1390] > 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if ((!s.b[1391]) && s.b[1394]) {
            s.store_square(1385, 1390);
            s.store_mul(331, 1385, 1390);
            s.store_offset(1388, 331, p.p212);
        }

        s.b[1395] = (p.p43 == 1.0);
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if s.b[1395] {
            s.store_scalar(1405, s.v[91]);
            s.store_div_from_scalar(1406, 1.0, 1405);
            s.store_scalar(1462, 0.0);
            s.store_scalar(1464, 0.0);
            s.store_scalar(1466, 0.0);
            s.store_neg(1398, 534);
            s.store_mul(1399, 1398, 436);
            s.store_add_scaled_product_indices(331, 1399, 1.0, 1398, 437, 1.0);
            s.store_mul(470, 1399, 438);
            s.store_sub(469, 1399, 470);
            s.store_mul(468, 331, 438);
            s.store_sub(467, 331, 468);
        }

        if (s.b[1395] && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
            s.store_scalar(528, 0.0);
        }

        s.b[1475] = (1.0 == 1.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        s.b[1476] = (1.0 == 2.0);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1475]) {
            s.store_scale(522, 533, 0.5);
            s.store_scalar(523, p.p292);
            s.store_scalar(528, s.v[525]);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && (s.b[1476] && (!s.b[1475]))) {
            s.store_scale(522, 534, 0.5);
            s.store_scalar(523, p.p68);
            s.store_scalar(528, s.v[524]);
            s.store_scalar(528, 1.0);
        }

        s.b[1477] = (s.v[528] == 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_mul_sqrt_ad_rhs(1425, 238, A::div(s.ad_value(521), s.ad_value(536)));
            s.store_scalar(1407, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1408, ((1.0 + -1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1418, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1419, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1420, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1421, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1422, 1419, 1418);
            s.store_neg(1423, 1418);
            s.store_add_scaled_products_indices(1409, 1407, 461, 1.0, 1408, 462, 1.0);
            s.store_add_scaled_products_indices(1410, 1407, 462, 1.0, 1408, 461, 1.0);
            s.store_add_scaled_products_indices(1424, 1409, 1420, 1.0, 1410, 1421, 1.0);
            s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1396, 1416);
        }

        s.b[1478] = (s.v[1396] > s.v[141]);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1478]) {
            s.store_sub(1397, 1396, 141);
            s.store_sub(1398, 140, 141);
            s.store_div(44, 1397, 1398);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1404, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1404, 1398, 1.0, 1404);
            s.store_add(1401, 141, 1404);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1478])) {
            s.copy_ad(1401, 1396);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));
            s.store_mul(1426, 1425, 1406);
            s.store_square(1427, 1426);
            s.store_sub(1428, 1424, 523);
            s.store_div(1396, 521, 230);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1429, 2.0, 225, A::ln(s.ad_value(1396)));
            s.store_neg(1430, 1417);
        }

        s.b[1479] = (s.v[1428] < s.v[1430]);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) {
            s.store_div_from_scalar_mul_ad(1397, 1.0, s.ad_value(225), s.ad_value(1425));
            s.store_mul(1404, 1397, 1405);
            s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);
            s.store_sub(1433, 237, 1429);
            s.store_mul_add_rhs(1403, 225, 1428, 1417);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1434, (7.0 * 1.414213562373095), 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);
            s.store_square(1435, 1434);
        }

        s.b[1480] = (s.v[1432] < (s.v[1435] * 1e-8));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) && s.b[1480]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1437, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) && (!s.b[1480])) {
            s.store_sqrt_add(1436, 1432, 1435);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, 1404, 1403, (-2.0), 9.0);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) {
            s.store_powf(1438, 1437, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1439, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, 1438, 2.0, 1438, 1438, 1.414213562373095);
            s.store_div(1440, 1439, 1438);
            s.store_add_scaled_product_indices(1441, 1417, (-1.0), 1440, 227, 1.0);
            s.store_add(1397, 1441, 1417);
            s.store_div(1398, 1397, 1433);
            s.store_sqrt_square_offset(1399, 1398, 1.0);
            s.store_sub_div_lhs_indices(1442, 1397, 1399, 1417);
            s.store_sub(1398, 1428, 1442);
            s.store_mul(459, 1405, 1398);
            s.copy_ad(458, 459);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_scalar(1440, 3.0);
            s.store_sub_div_lhs_indices(1443, 1440, 225, 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1481] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1481]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1482] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1482]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        s.b[1483] = (s.v[1440] < 3.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1483]) {
            s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1483]) {
            s.store_div_scaled_inputs2_indices(1447, 1428, -1.0, 1417, -1.0, 1426, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1448, A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), 1447, 1.0, 1444, 2.0, 1.0);
            s.store_div_ad(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), A::mul_scaled_lhs(s.ad_value(1444), 9.0, s.ad_value(1444)));
            s.store_sqrt_add_scaled_square_cube_product(1400, 1448, 1.0, 1449, 1.0);
            s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);
            s.store_neg_powf_add_input(1451, 1448, 1400, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(1403, 1450, 1.0, 1451, 1.0, 1445, 1.0, 1444, 3.0, -1.0);
            s.store_add_scaled_product_indices(1443, 1417, (-1.0), 1403, 227, 1.0);
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_offset_add(1452, 1428, 1417, 0.1);
            s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);
            s.store_div(1396, 230, 521);
            s.store_square(1453, 1396);
            s.store_mul(1454, 1453, 1459);
            s.store_mul(1396, 226, 1427);
            s.store_mul(1455, 225, 1452);
            s.store_add_scaled_inputs_product_mixed_aaii(1456, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);
            s.store_offset_sub(44, 1455, 1456, (-1.0));
            s.store_scale(45, 1455, 4.0);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1456, 1455, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1455, 1455, 1456);
            s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1457, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);
            s.copy_ad(1458, 1440);
            s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));
            s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1440, 1457, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub_div_lhs_indices(1442, 1440, 225, 1417);
            s.store_add_offset_lhs_ad_rhs(1397, 1440, (-1.0), A::exp_scaled_input(s.ad_value(1440), -1.0));
        }

        s.b[1484] = (s.v[1397] < (10.0 * 2.220446049250313e-16));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1484]) {
            s.store_scalar(1397, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            s.store_sqrt(1398, 1397);
            s.store_mul(458, 1425, 1398);
            s.store_mul_sub_rhs(459, 1405, 1428, 1442);
        }

        s.b[1485] = (p.p42 == 1.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
            s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));
            s.store_div(1396, 230, 521);
            s.store_square(1453, 1396);
            s.store_mul(1468, 1453, 1459);
            s.store_scalar(1413, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign26220_loop_guard: usize = 0;
        while {
            let assign26220_cond_e35796: f64 = (2.0 * 20.0);
            let assign26220_cond_e35798: f64 = (assign26220_cond_e35796 + 1.0);
            let assign26220_cond_e35800: f64 = if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (s.v[167] <= assign26220_cond_e35798)) { 1.0 } else { 0.0 };
            assign26220_cond_e35800 != 0.0
        } {
            assign26220_loop_guard += 1;
            assert!(assign26220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
                s.store_scalar(1464, 0.0);
                s.store_mul_add_rhs(1440, 225, 1442, 1417);
            }
            s.b[1486] = (s.v[1440] < 5.0);
            s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1486]) {
                s.store_mul3_ad_middle(1460, A::square(s.ad_value(1440)), 1440, A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1461, A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1462, 1468, 1460, 1460);
                s.store_mul_product3_indices(1463, 1461, 1468, 225, 1460, 2.0);
                s.store_mul_offset_ad_rhs(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1465, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1467, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), 1.0, 1463, 1.0, 1466, 2.0);
            }
            s.b[1487] = (s.v[1440] < 80.0);
            s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && s.b[1487]) {
                s.store_exp(243, 1440);
                s.store_mul_offset_rhs(1462, 1468, 243, (-1.0));
                s.store_mul3_lhs(1463, 1468, 225, 243);
            }
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && (!s.b[1487])) {
                s.store_exp_mul(1469, 225, 1442);
                s.store_mul_sub_rhs(1462, 1453, 1469, 1459);
                s.store_mul3_lhs(1463, 1453, 225, 1469);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) {
                s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));
                s.store_scale_ad(1467, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1463), 1.0, s.ad_value(1466), 1.0), 0.5);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
                s.store_add_scaled_inputs_product_indices(1470, 1428, 1.0, 1442, (-1.0), 1426, 1466, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1471, (-1.0), 1426, 1467, 1.0);
            }
            s.b[1488] = (s.v[1413] == 1.0);
            s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1488]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_div_scaled_inputs_indices(494, 1470, -1.0, 1471, 1.0);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1489] = (((s.v[494]) as f64).abs() > s.v[1472]);
            s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1489]) {
                s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_add(1442, 1442, 494);
            }
            s.b[1490] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));
            s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1490]) {
                s.store_scalar(1413, 1.0);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1492] = (s.v[1440] < 5.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1492]) {
            s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));
            s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1492])) {
            s.store_offset(1473, 1440, (-1.0));
            s.store_sqrt(1474, 1473);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
            s.store_mul(458, 1425, 1474);
            s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));
            s.store_mul3_lhs(460, 1425, 1462, 1397);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1494] = (1.0 == 1.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        s.b[1495] = (1.0 == 2.0);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_scalar(1407, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1408, ((1.0 + 1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1418, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1419, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1420, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1421, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1422, 1419, 1418);
            s.store_neg(1423, 1418);
            s.store_add_scaled_products_indices(1409, 1407, 461, 1.0, 1408, 462, 1.0);
            s.store_add_scaled_products_indices(1410, 1407, 462, 1.0, 1408, 461, 1.0);
            s.store_add_scaled_products_indices(1424, 1409, 1420, 1.0, 1410, 1421, 1.0);
            s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1396, 1416);
        }

        s.b[1496] = (s.v[1396] > s.v[141]);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1496]) {
            s.store_sub(1397, 1396, 141);
            s.store_sub(1398, 140, 141);
            s.store_div(44, 1397, 1398);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1404, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1404, 1398, 1.0, 1404);
            s.store_add(1401, 141, 1404);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1496])) {
            s.copy_ad(1401, 1396);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));
            s.store_mul(1426, 1425, 1406);
            s.store_square(1427, 1426);
            s.store_sub(1428, 1424, 523);
            s.store_div(1396, 521, 230);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1429, 2.0, 225, A::ln(s.ad_value(1396)));
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_neg(1430, 1417);
        }

        s.b[1497] = (s.v[1428] < s.v[1430]);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) {
            s.store_div_from_scalar_mul_ad(1397, 1.0, s.ad_value(225), s.ad_value(1425));
            s.store_mul(1404, 1397, 1405);
            s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);
            s.store_sub(1433, 237, 1429);
            s.store_mul_add_rhs(1403, 225, 1428, 1417);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1434, (7.0 * 1.414213562373095), 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);
            s.store_square(1435, 1434);
        }

        s.b[1498] = (s.v[1432] < (s.v[1435] * 1e-8));
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) && s.b[1498]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1437, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) && (!s.b[1498])) {
            s.store_sqrt_add(1436, 1432, 1435);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, 1404, 1403, (-2.0), 9.0);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) {
            s.store_powf(1438, 1437, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1439, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, 1438, 2.0, 1438, 1438, 1.414213562373095);
            s.store_div(1440, 1439, 1438);
            s.store_add_scaled_product_indices(1441, 1417, (-1.0), 1440, 227, 1.0);
            s.store_add(1397, 1441, 1417);
            s.store_div(1398, 1397, 1433);
            s.store_sqrt_square_offset(1399, 1398, 1.0);
            s.store_sub_div_lhs_indices(1442, 1397, 1399, 1417);
            s.store_sub(1398, 1428, 1442);
            s.store_mul(459, 1405, 1398);
            s.copy_ad(458, 459);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_scalar(1440, 3.0);
            s.store_sub_div_lhs_indices(1443, 1440, 225, 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1499] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1499]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
            s.store_exp_neg_input(1404, 1440);
            s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1500] = (s.v[1403] < (10.0 * 2.220446049250313e-16));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1500]) {
            s.store_scalar(1403, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_add_ad_rhs(1443, 1428, A::mul3_scaled_output(s.ad_value(1427), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        s.b[1501] = (s.v[1440] < 3.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1501]) {
            s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(1447, 1428, -1.0, 1417, -1.0, 1426, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1448, A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), 1447, 1.0, 1444, 2.0, 1.0);
            s.store_div_ad(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), A::mul_scaled_lhs(s.ad_value(1444), 9.0, s.ad_value(1444)));
            s.store_sqrt_add_scaled_square_cube_product(1400, 1448, 1.0, 1449, 1.0);
            s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);
            s.store_neg_powf_add_input(1451, 1448, 1400, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(1403, 1450, 1.0, 1451, 1.0, 1445, 1.0, 1444, 3.0, -1.0);
            s.store_add_scaled_product_indices(1443, 1417, (-1.0), 1403, 227, 1.0);
            s.store_mul_add_rhs(1440, 225, 1443, 1417);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_offset_add(1452, 1428, 1417, 0.1);
            s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);
            s.store_div(1396, 230, 521);
            s.store_square(1453, 1396);
            s.store_mul(1454, 1453, 1459);
            s.store_mul(1396, 226, 1427);
            s.store_mul(1455, 225, 1452);
            s.store_add_scaled_inputs_product_mixed_aaii(1456, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);
            s.store_offset_sub(44, 1455, 1456, (-1.0));
            s.store_scale(45, 1455, 4.0);
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1456, 1455, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1455, 1455, 1456);
            s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1457, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);
            s.copy_ad(1458, 1440);
            s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));
            s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1440, 1457, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub_div_lhs_indices(1442, 1440, 225, 1417);
            s.store_add_offset_lhs_ad_rhs(1397, 1440, (-1.0), A::exp_scaled_input(s.ad_value(1440), -1.0));
        }

        s.b[1502] = (s.v[1397] < (10.0 * 2.220446049250313e-16));
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1502]) {
            s.store_scalar(1397, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            s.store_sqrt(1398, 1397);
            s.store_mul(458, 1425, 1398);
            s.store_mul_sub_rhs(459, 1405, 1428, 1442);
        }

        s.b[1503] = (p.p42 == 1.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
            s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));
            s.store_div(1396, 230, 521);
            s.store_square(1453, 1396);
            s.store_mul(1468, 1453, 1459);
            s.store_scalar(1413, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign27770_loop_guard: usize = 0;
        while {
            let assign27770_cond_e38739: f64 = (2.0 * 20.0);
            let assign27770_cond_e38741: f64 = (assign27770_cond_e38739 + 1.0);
            let assign27770_cond_e38743: f64 = if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (s.v[167] <= assign27770_cond_e38741)) { 1.0 } else { 0.0 };
            assign27770_cond_e38743 != 0.0
        } {
            assign27770_loop_guard += 1;
            assert!(assign27770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
                s.store_scalar(1464, 0.0);
                s.store_mul_add_rhs(1440, 225, 1442, 1417);
            }
            s.b[1504] = (s.v[1440] < 5.0);
            s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1504]) {
                s.store_mul3_ad_middle(1460, A::square(s.ad_value(1440)), 1440, A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1461, A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1462, 1468, 1460, 1460);
                s.store_mul_product3_indices(1463, 1461, 1468, 225, 1460, 2.0);
                s.store_mul_offset_ad_rhs(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1465, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1467, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), 1.0, 1463, 1.0, 1466, 2.0);
            }
            s.b[1505] = (s.v[1440] < 80.0);
            s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && s.b[1505]) {
                s.store_exp(243, 1440);
                s.store_mul_offset_rhs(1462, 1468, 243, (-1.0));
                s.store_mul3_lhs(1463, 1468, 225, 243);
            }
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && (!s.b[1505])) {
                s.store_exp_mul(1469, 225, 1442);
                s.store_mul_sub_rhs(1462, 1453, 1469, 1459);
                s.store_mul3_lhs(1463, 1453, 225, 1469);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) {
                s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));
                s.store_scale_ad(1467, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1463), 1.0, s.ad_value(1466), 1.0), 0.5);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
                s.store_add_scaled_inputs_product_indices(1470, 1428, 1.0, 1442, (-1.0), 1426, 1466, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1471, (-1.0), 1426, 1467, 1.0);
            }
            s.b[1506] = (s.v[1413] == 1.0);
            s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1506]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_div_scaled_inputs_indices(494, 1470, -1.0, 1471, 1.0);
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1507] = (((s.v[494]) as f64).abs() > s.v[1472]);
            s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1507]) {
                s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_add(1442, 1442, 494);
            }
            s.b[1508] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));
            s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1508]) {
                s.store_scalar(1413, 1.0);
            }
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1510] = (s.v[1440] < 5.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1510]) {
            s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));
            s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1510])) {
            s.store_offset(1473, 1440, (-1.0));
            s.store_sqrt(1474, 1473);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
            s.store_mul(458, 1425, 1474);
            s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));
            s.store_mul3_lhs(460, 1425, 1462, 1397);
        }

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
            s.store_add(459, 458, 460);
        }

        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1512] = (1.0 == 1.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        s.b[1513] = (1.0 == 2.0);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1407] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1408] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        s.v[317] = p.p189;

        s.b[1516] = (s.v[145] != 0.0);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if s.b[1516] {
            s.store_add(1515, 157, 161);
            s.store_add_scaled_inputs(314, 1515, s.v[317], 162, (1.0 - s.v[317]));
        }

        s.b[1517] = (p.p64 != 0.0);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1517]) {
            s.store_scalar(315, 0.0);
        }

        s.b[1518] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1518]) {
            s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[1519] = (p.p64 != 0.0);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        s.b[1520] = (s.v[246] < 1e-15);
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if (((!s.b[1516]) && s.b[1519]) && s.b[1520]) {
            s.store_scalar(315, 0.0);
        }

        if (((!s.b[1516]) && s.b[1519]) && (!s.b[1520])) {
            s.store_scale(1514, 227, 1.0 / (s.v[97]));
            s.store_div_from_scalar(1515, 1.0, 244);
            s.store_mul3_lhs(315, 246, 1514, 1515);
        }

        s.v[1532] = s.v[91];

        s.v[1533] = (1.0 / s.v[1532]);

        s.v[1553] = 0.0;

        s.v[1593] = 0.0;

        s.v[1591] = 0.0;

        s.v[1595] = 0.0;

        s.b[1604] = ((p.p29 >= 1.0) && (p.p188 > 0.0));
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && s.b[1604]) {
            s.store_scalar(1535, p.p171);
            s.store_scalar(1536, p.p172);
            s.copy_ad(1537, 158);
            s.store_scalar(1534, p.p188);
        }

        s.b[1605] = ((s.v[69] == 0.0) && (p.p188 > 0.0));
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {
            if (p.p43 == 1.0) {
                s.store_scale(1522, 287, s.v[1532]);
            } else {
                s.store_scale(1522, 108, s.v[1532]);
            }
        }

        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {
            s.store_mul_ad_product_rhs(1525, 1535, s.ad_value(1522), A::add(s.ad_value(1536), s.ad_value(1537)));
            s.store_mul(1526, 1534, 1522);
            s.copy_ad(1530, 161);
            s.store_sub_from_scalar(1527, 1.2, 1530);
            s.store_add_scaled_products_indices(267, 158, 1526, 1.0, 1527, 1525, (-1.0));
            s.store_mul_ad_product_rhs(1525, 1535, s.ad_value(1522), A::add_scaled_inputs3(s.ad_value(1536), 1.0, s.ad_value(1537), 1.0, s.ad_value(157), -1.0));
            s.store_sub(1530, 162, 157);
            s.store_sub_from_scalar(1527, 1.2, 1530);
            s.store_add_scaled_products_left_left_ad(268, A::sub(s.ad_value(158), s.ad_value(157)), 1526, 1.0, 1525, 1527, (-1.0));
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_mul_sqrt_ad_rhs(1554, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));
            s.store_scalar(1538, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1539, ((1.0 + -1.0) / 2.0));
        }

        s.b[1606] = (p.p43 == 1.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {
            s.store_add_scaled_products_right_right_ad(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1551, 1549, 1548);
            s.store_sub(1553, 1550, 1548);
            s.store_neg(1552, 1548);
            s.store_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);
            s.store_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);
            s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);
            s.store_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1538] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1553, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1539] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1553, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_scalar(1546, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_neg(1521, 1546);
        }

        s.b[1607] = (s.v[1521] > s.v[141]);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1607]) {
            s.store_sub(1522, 1521, 141);
            s.store_sub(1523, 140, 141);
            s.store_div(44, 1522, 1523);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1531, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1531, 1523, 1.0, 1531);
            s.store_add(1528, 141, 1531);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1607])) {
            s.copy_ad(1528, 1521);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));
            s.store_scale(1555, 1554, s.v[1533]);
            s.store_square(1556, 1555);
            s.store_sub_from_scalar(1557, s.v[82], 1553);
            s.store_div_from_scalar(1521, s.v[69], 230);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1558, 2.0, 225, A::ln(s.ad_value(1521)));
            s.store_neg(1559, 1547);
        }

        s.b[1608] = (s.v[1557] < s.v[1559]);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {
            s.store_div_from_scalar_mul_ad(1522, 1.0, s.ad_value(225), s.ad_value(1554));
            s.store_scale(1531, 1522, s.v[1532]);
            s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);
            s.store_sub(1562, 237, 1558);
            s.store_mul_add_rhs(1530, 225, 1557, 1547);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1563, (7.0 * 1.414213562373095), 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);
            s.store_square(1564, 1563);
        }

        s.b[1609] = (s.v[1561] < (s.v[1564] * 1e-8));
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && s.b[1609]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1566, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && (!s.b[1609])) {
            s.store_sqrt_add(1565, 1561, 1564);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, 1531, 1530, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {
            s.store_powf(1567, 1566, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1568, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, 1567, 2.0, 1567, 1567, 1.414213562373095);
            s.store_div(1569, 1568, 1567);
            s.store_add_scaled_product_indices(1570, 1547, (-1.0), 1569, 227, 1.0);
            s.store_add(1522, 1570, 1547);
            s.store_div(1523, 1522, 1562);
            s.store_sqrt_square_offset(1524, 1523, 1.0);
            s.store_sub_div_lhs_indices(1571, 1522, 1524, 1547);
            s.store_sub(1523, 1557, 1571);
            s.store_scale(459, 1523, s.v[1532]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_scalar(1569, 3.0);
            s.store_sub_div_lhs_indices(1572, 1569, 225, 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1610] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1610]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1611] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1611]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1612] = (s.v[1569] < 3.0);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1612]) {
            s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(1576, 1557, -1.0, 1547, -1.0, 1555, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1577, A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), 1576, 1.0, 1573, 2.0, 1.0);
            s.store_div_ad(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), A::mul_scaled_lhs(s.ad_value(1573), 9.0, s.ad_value(1573)));
            s.store_sqrt_add_scaled_square_cube_product(1526, 1577, 1.0, 1578, 1.0);
            s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);
            s.store_neg_powf_add_input(1580, 1577, 1526, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(1530, 1579, 1.0, 1580, 1.0, 1574, 1.0, 1573, 3.0, -1.0);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1612]) {
            s.store_add_scaled_product_indices(1572, 1547, (-1.0), 1530, 227, 1.0);
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1613] = (p.p41 > 0.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_offset_add(1581, 1557, 1547, 0.1);
            s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1583, 1582, 1588);
            s.store_mul(1521, 226, 1556);
            s.store_mul(1584, 225, 1581);
            s.store_add_scaled_inputs_product_mixed_aaii(1585, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);
            s.store_offset_sub(44, 1584, 1585, (-1.0));
            s.store_scale(45, 1584, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1585, 1584, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1584, 1584, 1585);
            s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1586, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);
            s.copy_ad(1587, 1569);
            s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));
            s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1569, 1586, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_sub_div_lhs_indices(1571, 1569, 225, 1547);
            s.store_add_offset_lhs_ad_rhs(1522, 1569, (-1.0), A::exp_scaled_input(s.ad_value(1569), -1.0));
        }

        s.b[1614] = (s.v[1522] < (10.0 * 2.220446049250313e-16));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1614]) {
            s.store_scalar(1522, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {
            s.store_sqrt(1523, 1522);
            s.store_mul(458, 1554, 1523);
            s.store_scaled_sub(459, 1557, 1571, s.v[1532]);
        }

        s.b[1615] = (p.p41 == 1.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
            s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1597, 1582, 1588);
            s.store_scalar(1544, 0.0);
            s.store_scalar(1591, 0.0);
            s.store_scalar(1595, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign29770_loop_guard: usize = 0;
        while {
            let assign29770_cond_e42272: f64 = (2.0 * 20.0);
            let assign29770_cond_e42274: f64 = (assign29770_cond_e42272 + 1.0);
            let assign29770_cond_e42276: f64 = if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (s.v[167] <= assign29770_cond_e42274)) { 1.0 } else { 0.0 };
            assign29770_cond_e42276 != 0.0
        } {
            assign29770_loop_guard += 1;
            assert!(assign29770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
                s.store_scalar(1593, 0.0);
                s.store_mul_add_rhs(1569, 225, 1571, 1547);
            }
            s.b[1616] = (s.v[1569] < 5.0);
            s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1616]) {
                s.store_mul3_ad_middle(1589, A::square(s.ad_value(1569)), 1569, A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1590, A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1591, 1597, 1589, 1589);
                s.store_mul_product3_indices(1592, 1590, 1597, 225, 1589, 2.0);
                s.store_mul_offset_ad_rhs(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1594, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1596, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), 1.0, 1592, 1.0, 1595, 2.0);
            }
            s.b[1617] = (s.v[1569] < 80.0);
            s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && s.b[1617]) {
                s.store_exp(243, 1569);
                s.store_mul_offset_rhs(1591, 1597, 243, (-1.0));
                s.store_mul3_lhs(1592, 1597, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && (!s.b[1617])) {
                s.store_exp_mul(1598, 225, 1571);
                s.store_mul_sub_rhs(1591, 1582, 1598, 1588);
                s.store_mul3_lhs(1592, 1582, 225, 1598);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) {
                s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));
                s.store_scale_ad(1596, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1592), 1.0, s.ad_value(1595), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
                s.store_add_scaled_inputs_product_indices(1599, 1557, 1.0, 1571, (-1.0), 1555, 1595, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1600, (-1.0), 1555, 1596, 1.0);
            }
            s.b[1618] = (s.v[1544] == 1.0);
            s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1618]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_div_scaled_inputs_indices(494, 1599, -1.0, 1600, 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1619] = (((s.v[494]) as f64).abs() > s.v[1601]);
            s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1619]) {
                s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_add(1571, 1571, 494);
            }
            s.b[1620] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));
            s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1620]) {
                s.store_scalar(1544, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1622] = (s.v[1569] < 5.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1622]) {
            s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));
            s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1622])) {
            s.store_offset(1602, 1569, (-1.0));
            s.store_sqrt(1603, 1602);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
            s.store_mul(458, 1554, 1603);
            s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));
            s.store_mul3_lhs(460, 1554, 1591, 1522);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            if (p.p43 == 1.0) {
                s.store_mul(1525, 287, 1534);
            } else {
                s.store_mul(1525, 108, 1534);
            }
        }

        s.b[1624] = (((s.v[1540] != 0.0) && (p.p43 == 0.0)) || ((s.v[1538] != 0.0) && (p.p43 == 1.0)));
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1624]) {
            s.store_mul(455, 1525, 459);
            s.store_mul(457, 1525, 458);
        }

        s.b[1625] = (((s.v[1541] != 0.0) && (p.p43 == 0.0)) || ((s.v[1539] != 0.0) && (p.p43 == 1.0)));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1625]) {
            s.store_mul(454, 1525, 459);
            s.store_mul(456, 1525, 458);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_scalar(1538, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1539, ((1.0 + 1.0) / 2.0));
        }

        s.b[1626] = (p.p43 == 1.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1626]) {
            s.store_add_scaled_products_right_right_ad(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1551, 1549, 1548);
            s.store_sub(1553, 1550, 1548);
            s.store_neg(1552, 1548);
            s.store_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);
            s.store_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);
            s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {
            s.store_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);
            s.store_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1538] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1553, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1539] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1553, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {
            s.store_scalar(1546, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_neg(1521, 1546);
        }

        s.b[1627] = (s.v[1521] > s.v[141]);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1627]) {
            s.store_sub(1522, 1521, 141);
            s.store_sub(1523, 140, 141);
            s.store_div(44, 1522, 1523);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1531, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1531, 1523, 1.0, 1531);
            s.store_add(1528, 141, 1531);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1627])) {
            s.copy_ad(1528, 1521);
        }

        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {
            s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));
            s.store_scale(1555, 1554, s.v[1533]);
            s.store_square(1556, 1555);
            s.store_sub_from_scalar(1557, s.v[82], 1553);
            s.store_div_from_scalar(1521, s.v[69], 230);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1558, 2.0, 225, A::ln(s.ad_value(1521)));
            s.store_neg(1559, 1547);
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1628] = (s.v[1557] < s.v[1559]);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {
            s.store_div_from_scalar_mul_ad(1522, 1.0, s.ad_value(225), s.ad_value(1554));
            s.store_scale(1531, 1522, s.v[1532]);
            s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);
            s.store_sub(1562, 237, 1558);
            s.store_mul_add_rhs(1530, 225, 1557, 1547);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1563, (7.0 * 1.414213562373095), 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);
            s.store_square(1564, 1563);
        }

        s.b[1629] = (s.v[1561] < (s.v[1564] * 1e-8));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && s.b[1629]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1566, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && (!s.b[1629])) {
            s.store_sqrt_add(1565, 1561, 1564);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, 1531, 1530, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {
            s.store_powf(1567, 1566, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1568, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, 1567, 2.0, 1567, 1567, 1.414213562373095);
            s.store_div(1569, 1568, 1567);
            s.store_add_scaled_product_indices(1570, 1547, (-1.0), 1569, 227, 1.0);
            s.store_add(1522, 1570, 1547);
            s.store_div(1523, 1522, 1562);
            s.store_sqrt_square_offset(1524, 1523, 1.0);
            s.store_sub_div_lhs_indices(1571, 1522, 1524, 1547);
            s.store_sub(1523, 1557, 1571);
            s.store_scale(459, 1523, s.v[1532]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_scalar(1569, 3.0);
            s.store_sub_div_lhs_indices(1572, 1569, 225, 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1630] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1630]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
            s.store_exp_neg_input(1531, 1569);
            s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1631] = (s.v[1530] < (10.0 * 2.220446049250313e-16));
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1631]) {
            s.store_scalar(1530, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_add_ad_rhs(1572, 1557, A::mul3_scaled_output(s.ad_value(1556), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1632] = (s.v[1569] < 3.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {
            s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(1576, 1557, -1.0, 1547, -1.0, 1555, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1577, A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), 1576, 1.0, 1573, 2.0, 1.0);
            s.store_div_ad(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), A::mul_scaled_lhs(s.ad_value(1573), 9.0, s.ad_value(1573)));
            s.store_sqrt_add_scaled_square_cube_product(1526, 1577, 1.0, 1578, 1.0);
            s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);
            s.store_neg_powf_add_input(1580, 1577, 1526, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(1530, 1579, 1.0, 1580, 1.0, 1574, 1.0, 1573, 3.0, -1.0);
            s.store_add_scaled_product_indices(1572, 1547, (-1.0), 1530, 227, 1.0);
            s.store_mul_add_rhs(1569, 225, 1572, 1547);
        }

        s.b[1633] = (p.p41 > 0.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_offset_add(1581, 1557, 1547, 0.1);
            s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1583, 1582, 1588);
            s.store_mul(1521, 226, 1556);
            s.store_mul(1584, 225, 1581);
            s.store_add_scaled_inputs_product_mixed_aaii(1585, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);
            s.store_offset_sub(44, 1584, 1585, (-1.0));
            s.store_scale(45, 1584, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1585, 1584, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_sub(1584, 1584, 1585);
            s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1586, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);
            s.copy_ad(1587, 1569);
            s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));
            s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(1569, 1586, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_sub_div_lhs_indices(1571, 1569, 225, 1547);
            s.store_add_offset_lhs_ad_rhs(1522, 1569, (-1.0), A::exp_scaled_input(s.ad_value(1569), -1.0));
        }

        s.b[1634] = (s.v[1522] < (10.0 * 2.220446049250313e-16));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1634]) {
            s.store_scalar(1522, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {
            s.store_sqrt(1523, 1522);
            s.store_mul(458, 1554, 1523);
            s.store_scaled_sub(459, 1557, 1571, s.v[1532]);
        }

        s.b[1635] = (p.p41 == 1.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
            s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));
            s.store_scale(1521, 230, 1.0 / (s.v[69]));
            s.store_square(1582, 1521);
            s.store_mul(1597, 1582, 1588);
            s.store_scalar(1544, 0.0);
            s.store_scalar(1591, 0.0);
            s.store_scalar(1595, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign31370_loop_guard: usize = 0;
        while {
            let assign31370_cond_e45508: f64 = (2.0 * 20.0);
            let assign31370_cond_e45510: f64 = (assign31370_cond_e45508 + 1.0);
            let assign31370_cond_e45512: f64 = if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (s.v[167] <= assign31370_cond_e45510)) { 1.0 } else { 0.0 };
            assign31370_cond_e45512 != 0.0
        } {
            assign31370_loop_guard += 1;
            assert!(assign31370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
                s.store_scalar(1593, 0.0);
                s.store_mul_add_rhs(1569, 225, 1571, 1547);
            }
            s.b[1636] = (s.v[1569] < 5.0);
            s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1636]) {
                s.store_mul3_ad_middle(1589, A::square(s.ad_value(1569)), 1569, A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1590, A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1591, 1597, 1589, 1589);
                s.store_mul_product3_indices(1592, 1590, 1597, 225, 1589, 2.0);
                s.store_mul_offset_ad_rhs(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1594, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(1596, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), 1.0, 1592, 1.0, 1595, 2.0);
            }
            s.b[1637] = (s.v[1569] < 80.0);
            s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && s.b[1637]) {
                s.store_exp(243, 1569);
                s.store_mul_offset_rhs(1591, 1597, 243, (-1.0));
                s.store_mul3_lhs(1592, 1597, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) && (!s.b[1637])) {
                s.store_exp_mul(1598, 225, 1571);
                s.store_mul_sub_rhs(1591, 1582, 1598, 1588);
                s.store_mul3_lhs(1592, 1582, 225, 1598);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1636])) {
                s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));
                s.store_scale_ad(1596, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1592), 1.0, s.ad_value(1595), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
                s.store_add_scaled_inputs_product_indices(1599, 1557, 1.0, 1571, (-1.0), 1555, 1595, (-1.0));
                s.store_sub_from_scalar_scaled_mul(1600, (-1.0), 1555, 1596, 1.0);
            }
            s.b[1638] = (s.v[1544] == 1.0);
            s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1638]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_div_scaled_inputs_indices(494, 1599, -1.0, 1600, 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1639] = (((s.v[494]) as f64).abs() > s.v[1601]);
            s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1639]) {
                s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) {
                s.store_add(1571, 1571, 494);
            }
            s.b[1640] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));
            s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1638])) && s.b[1640]) {
                s.store_scalar(1544, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1642] = (s.v[1569] < 5.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && s.b[1642]) {
            s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));
            s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) && (!s.b[1642])) {
            s.store_offset(1602, 1569, (-1.0));
            s.store_sqrt(1603, 1602);
        }

        if (((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
            s.store_mul(458, 1554, 1603);
        }

    }
}
