#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[737] && (!s.b[929])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }

        s.b[1039] = (s.v[430] == 0.0);
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1039]) {
            s.copy_ad(352, 346);
            s.copy_ad(353, 347);
            s.copy_ad(354, 348);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(162, 352);
            s.copy_ad(157, 453);
        }

        s.b[1040] = (s.v[349] < 0.0);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1040]) {
            s.store_scalar(145, 1.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(374, 349);
            s.copy_ad(375, 352);
            s.store_sub(164, 375, 374);
            s.copy_ad(373, 351);
            s.store_scale(400, 401, 9662367879.197212);
            s.store_add_scaled_inputs3(246, s.ad_value(358), 1.0, s.ad_value(355), (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);
        }

        s.b[1041] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1041]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_sub(411, 352, 349);
            s.store_offset(411, 411, 5e-12);
            s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);
            s.store_div_scaled_inputs2(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), s.ad_value(410), 1.0);
        }

        s.b[1042] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
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

        s.b[1043] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        s.b[1044] = (2.0 == 1.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && s.b[1044]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1045] = (2.0 == 2.0);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && s.b[1045]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1046] = (2.0 == 4.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if ((((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && (!s.b[1045])) && s.b[1046]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1047] = (2.0 == 8.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if (((((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && (!s.b[1045])) && (!s.b[1046])) && s.b[1047]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13420_loop_guard: usize = 0;
        while {
            let assign13420_cond_e19042: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13420_cond_e19042 != 0.0
        } {
            assign13420_loop_guard += 1;
            assert!(assign13420_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1042]) && (!s.b[1043])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1042])) {
            s.store_neg(328, 409);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_neg(409, 328);
        }

        s.b[1048] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1048]) {
            s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_neg_ad(414, A::sub(s.ad_value(358), s.ad_value(355)));
        }

        s.b[1049] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
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

        s.b[1050] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        s.b[1051] = (2.0 == 1.0);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && s.b[1051]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1052] = (2.0 == 2.0);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && s.b[1052]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1053] = (2.0 == 4.0);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if ((((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && (!s.b[1052])) && s.b[1053]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1054] = (2.0 == 8.0);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (((((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && (!s.b[1052])) && (!s.b[1053])) && s.b[1054]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13780_loop_guard: usize = 0;
        while {
            let assign13780_cond_e19468: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13780_cond_e19468 != 0.0
        } {
            assign13780_loop_guard += 1;
            assert!(assign13780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1049]) && (!s.b[1050])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1049])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_offset_div_scaled_inputs(412, s.ad_value(414), (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);
            s.store_mul_ad_product_lhs(328, A::square(s.ad_value(411)), s.ad_value(411), 411);
            s.store_mul(415, 412, 411);
            s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);
        }

        s.b[1055] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
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

        s.b[1056] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        s.b[1057] = (2.0 == 1.0);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && s.b[1057]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1058] = (2.0 == 2.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && s.b[1058]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1059] = (2.0 == 4.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if ((((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && (!s.b[1058])) && s.b[1059]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1060] = (2.0 == 8.0);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (((((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && (!s.b[1058])) && (!s.b[1059])) && s.b[1060]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign14140_loop_guard: usize = 0;
        while {
            let assign14140_cond_e19897: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign14140_cond_e19897 != 0.0
        } {
            assign14140_loop_guard += 1;
            assert!(assign14140_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1055]) && (!s.b[1056])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
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
        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
            s.store_sub_from_scalar(413, 1e-5, 43);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1055])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(190, 413);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if (s.b[737] && (!s.b[929])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        if (!s.b[737]) {
            s.copy_ad(515, 154);
        }

        s.b[1067] = (s.v[416] < p.p237);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if ((!s.b[737]) && s.b[1067]) {
            s.store_scalar(339, 1.0);
        }

        if ((!s.b[737]) && (!s.b[1067])) {
            s.store_scalar(339, 2.0);
        }

        if (!s.b[737]) {
            s.store_add_scaled_inputs3_offset(160, s.ad_value(185), (-1.0), s.ad_value(320), 1.0, s.ad_value(515), 1.0, s.v[123]);
        }

        s.b[1068] = (s.v[158] < s.v[160]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if ((!s.b[737]) && s.b[1068]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ad_rhs(254, 227, 2.0, A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
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

        s.b[1069] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1068]) && s.b[1069]) {
            s.store_add_scaled_inputs3_offset(257, s.ad_value(261), 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, s.ad_value(332), 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((!s.b[737]) && s.b[1068]) && (!s.b[1069])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if ((!s.b[737]) && s.b[1068]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 515, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 515);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_ad_lhs(161, A::div(s.ad_value(328), s.ad_value(330)), 515);
        }

        s.b[1070] = (s.v[144] >= 1.0);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && (!s.b[1068])) && s.b[1070]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(378, s.v[619]);
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_add_ad_rhs(376, 159, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
            s.store_mul_sub_rhs(181, 225, 376, 515);
        }

        s.b[1071] = (s.v[181] < 3.0);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && s.b[1071]) {
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

        s.b[1072] = (s.v[158] <= s.v[182]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && s.b[1072]) {
            s.copy_ad(378, 376);
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul3_lhs(329, 328, 159, 159);
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, s.ad_value(159)));
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(378, s.ad_value(377), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_offset(336, 515, (5e-12 / 2.0));
        }

        s.b[1073] = (s.v[378] < s.v[336]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && s.b[1073]) {
            s.copy_ad(378, 336);
        }

        if ((!s.b[737]) && (!s.b[1068])) {
            s.copy_ad(161, 378);
            s.copy_ad(163, 376);
        }

        s.b[1074] = ((p.p25 == 1.0) && (p.p26 == 2.0));
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if ((!s.b[737]) && s.b[1074]) {
            s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
        }

        if ((!s.b[737]) && (!s.b[1074])) {
            s.store_scalar(393, 0.0);
        }

        if (!s.b[737]) {
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
        let mut assign15070_loop_guard: usize = 0;
        while {
            let assign15070_cond_e21065: f64 = (s.v[57] + 1.0);
            let assign15070_cond_e21067: f64 = if ((!s.b[737]) && (s.v[167] <= assign15070_cond_e21065)) { 1.0 } else { 0.0 };
            assign15070_cond_e21067 != 0.0
        } {
            assign15070_loop_guard += 1;
            assert!(assign15070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[737]) {
                s.store_sub(417, 349, 515);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1075] = (s.v[337] < 80.0);
            s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };
            if ((!s.b[737]) && s.b[1075]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if ((!s.b[737]) && (!s.b[1075])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if (!s.b[737]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1076] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };
            if ((!s.b[737]) && s.b[1076]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1077] = (s.v[181] < 0.0);
            s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1076]) && s.b[1077]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1078] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && (!s.b[1076])) && s.b[1078]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if (((!s.b[737]) && (!s.b[1076])) && (!s.b[1078])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1079] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };
            if ((!s.b[737]) && s.b[1079]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1080] = (s.v[338] == (-1.0));
            s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };
            if ((!s.b[737]) && s.b[1080]) {
                s.store_scalar(401, 0.0);
            }
            if ((!s.b[737]) && (!s.b[1080])) {
                s.store_mul(401, 444, 242);
            }
            s.b[1081] = (s.v[401] < (p.p237 * 1.01));
            s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };
            if ((!s.b[737]) && s.b[1081]) {
                s.store_scalar(339, 1.0);
            }
            if ((!s.b[737]) && (!s.b[1081])) {
                s.store_scalar(339, 2.0);
            }
            if (!s.b[737]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1082] = (s.v[181] < 0.0);
            s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };
            if ((!s.b[737]) && s.b[1082]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1083] = (s.v[181] < 1e-7);
            s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && (!s.b[1082])) && s.b[1083]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            s.b[1084] = (s.v[181] < 80.0);
            s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) && s.b[1084]) {
                s.store_exp(243, 181);
                s.store_mul_sub_ad_rhs(488, 487, s.ad_value(243), A::offset(s.ad_value(181), 1.0));
                s.store_mul_ad_product_rhs(489, 487, s.ad_value(225), A::offset(s.ad_value(243), (-1.0)));
            }
            if ((((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) && (!s.b[1084])) {
                s.store_exp_mul(485, 225, 349);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(485), s.ad_value(486)));
            }
            if (((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) {
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if (!s.b[737]) {
                s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1085] = (s.v[430] == 1.0);
            s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };
            if ((!s.b[737]) && s.b[1085]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((!s.b[737]) && (!s.b[1085])) {
                s.store_div_scaled_inputs(494, s.ad_value(492), -1.0, s.ad_value(493), 1.0);
            }
            if ((!s.b[737]) && (!s.b[1085])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1086] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && (!s.b[1085])) && s.b[1086]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((!s.b[737]) && (!s.b[1085])) {
                s.store_add(349, 349, 494);
            }
            s.b[1087] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && (!s.b[1085])) && s.b[1087]) {
                s.store_scalar(430, 1.0);
            }
            if (!s.b[737]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (!s.b[737]) {
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

        s.b[1088] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if ((!s.b[737]) && s.b[1088]) {
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

        s.b[1089] = (s.v[612] == 0.0);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if ((!s.b[737]) && s.b[1089]) {
            s.copy_ad(453, 157);
            s.store_scalar(1096, 1e-50);
            s.store_div_ad_rhs(1091, 545, A::square(s.ad_value(323)));
            s.store_offset_mul_ad(1093, A::div_from_scalar(2.0, s.ad_value(1091)), A::sub(s.ad_value(159), s.ad_value(1096)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1091), 1.0);
        }

        s.b[1097] = ((s.v[1093] < s.v[332]) && (s.v[332] >= 0.0));
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1097]) {
            s.store_sub(44, 332, 1093);
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

        s.b[1098] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        s.b[1099] = (4.0 == 1.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && s.b[1099]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1100] = (4.0 == 2.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if ((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && s.b[1100]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1101] = (4.0 == 4.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if (((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && (!s.b[1100])) && s.b[1101]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1102] = (4.0 == 8.0);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if ((((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && (!s.b[1100])) && (!s.b[1101])) && s.b[1102]) {
            s.store_scalar(55, 4.0);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign15790_loop_guard: usize = 0;
        while {
            let assign15790_cond_e22480: f64 = if (((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign15790_cond_e22480 != 0.0
        } {
            assign15790_loop_guard += 1;
            assert!(assign15790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && (!s.b[1098])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((!s.b[737]) && s.b[1089]) && s.b[1097]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(1093, 332, 43);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1097])) {
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_sqrt(1092, 1093);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[737]) && s.b[1089]) {
            s.store_add_ad_rhs(1096, 159, A::mul_sub_from_scalar_rhs(s.ad_value(1091), 1.0, s.ad_value(1092)));
            s.store_sqrt_square_offset(44, 1096, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1096, 1096, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1103] = (s.v[1096] < 0.0);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1103]) {
            s.store_scalar(1096, 0.0);
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_div(1090, 157, 1096);
            s.store_pow_ad(1091, s.ad_value(1090), A::offset(s.ad_value(138), (-1.0)));
            s.store_mul(1095, 1091, 1090);
            s.store_offset(1092, 1095, 1.0);
            s.store_pow_ad(1093, s.ad_value(1092), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));
            s.store_mul(1094, 1093, 1092);
            s.store_div(452, 157, 1094);
            s.copy_ad(157, 452);
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));
        }

        s.b[1104] = (s.v[157] <= 0.0);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1104]) {
            s.store_scalar(164, 0.0);
            s.copy_ad(162, 161);
            s.store_scalar(430, 0.0);
        }

        s.b[1105] = (s.v[144] >= 1.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1105]) {
            s.store_scalar(352, s.v[622]);
            s.store_sub_from_scalar(165, s.v[622], 161);
        }

        s.b[1106] = (s.v[144] == 0.0);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            if ((s.v[163] - s.v[161]) >= 0.0) {
                s.store_sub(166, 163, 161);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            s.store_offset_sub_scaled_inputs(44, s.ad_value(166), (1.0 + 0.3), s.ad_value(157), 1.0, (-0.03));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(165, s.ad_value(166), (1.0 + 0.3), s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[1107] = (s.v[165] < 0.0);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1107]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1108] = (s.v[165] > s.v[157]);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if (((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && (!s.b[1107])) && s.b[1108]) {
            s.copy_ad(165, 157);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1104])) {
            s.copy_ad(164, 165);
            s.store_add(162, 161, 164);
            s.store_scalar(430, 0.0);
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.copy_ad(352, 162);
            s.store_scalar(168, 1.0);
        }

        let mut assign16250_loop_guard: usize = 0;
        while {
            let assign16250_cond_e23013: f64 = (s.v[58] + 1.0);
            let assign16250_cond_e23015: f64 = if (((!s.b[737]) && s.b[1089]) && (s.v[168] <= assign16250_cond_e23013)) { 1.0 } else { 0.0 };
            assign16250_cond_e23015 != 0.0
        } {
            assign16250_loop_guard += 1;
            assert!(assign16250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[737]) && s.b[1089]) {
                s.store_sub(418, 352, 515);
                s.store_mul(181, 225, 418);
                s.store_mul_sub_rhs(337, 420, 418, 419);
            }
            s.b[1109] = (s.v[337] < 80.0);
            s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1109]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1109])) {
                s.store_sub(422, 418, 419);
                s.store_scalar(423, 1.0);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1110] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1110]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1111] = (s.v[181] < 0.0);
            s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && s.b[1110]) && s.b[1111]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1112] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1110])) && s.b[1112]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1110])) && (!s.b[1112])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1113] = (s.v[338] == (-1.0));
            s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1113]) {
                s.store_scalar(401, 0.0);
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1113])) {
                s.store_mul(401, 444, 242);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1114] = (s.v[181] < 0.0);
            s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1114]) {
                s.store_neg(499, 242);
                s.store_neg(500, 443);
            }
            s.b[1115] = (s.v[181] < 1e-7);
            s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1114])) && s.b[1115]) {
                s.copy_ad(499, 242);
                s.copy_ad(500, 443);
            }
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1114])) && (!s.b[1115])) {
                s.store_mul_sub_rhs(501, 225, 352, 157);
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(497, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(498, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(499, 242, 497);
                s.store_div_scaled_add_product(500, s.ad_value(498), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(499), 1.0);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_add_scaled_inputs_products_indices(503, 352, 1.0, 159, (-1.0), 240, 499, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(504, 240, 500, 1.0);
            }
            s.b[1116] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));
            s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1116]) {
                s.store_scalar(168, (s.v[58] + 1.0));
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {
                s.store_div_scaled_inputs(495, s.ad_value(503), -1.0, s.ad_value(504), 1.0);
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1117] = (((s.v[495]) as f64).abs() > s.v[496]);
            s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1116])) && s.b[1117]) {
                s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {
                s.store_add(352, 352, 495);
            }
            s.b[1118] = ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8));
            s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1116])) && s.b[1118]) {
                s.store_scalar(430, 1.0);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((!s.b[737]) && s.b[1089]) {
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
            s.store_add_scaled_inputs4(328, s.ad_value(159), 1.0, s.ad_value(227), 1.0, s.ad_value(161), (-(2.0 * 0.5)), s.ad_value(164), (-0.5));
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

        s.b[1119] = (s.v[339] <= 1.0);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1119]) {
            s.store_add_scaled_inputs3(246, A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(427), (-1.0), A::div_scaled_product(A::square(s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), -1.0);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1119])) {
            s.store_mul(246, 164, 511);
        }

        s.b[1120] = ((s.v[84] >= 1.0) && (s.v[246] < 0.0));
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1120]) {
            s.store_scalar(246, 0.0);
        }

        s.b[1121] = (s.v[339] <= 1.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        s.b[1122] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && s.b[1122]) {
            let assign16620_ad_e24190: A = A::mul(A::mul3(A::add_scaled_inputs_product(s.ad_value(425), 1.0, s.ad_value(424), (-2.0), A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div_scaled_product(s.ad_value(424), s.ad_value(424), 2.0, s.ad_value(428), 1.0)), A::div_scaled_product(s.ad_value(426), s.ad_value(426), 0.1, s.ad_value(428), 1.0)), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426));
            s.store_add_scaled_product_mixed_aia(437, A::div(assign16620_ad_e24190, s.ad_value(428)), 0.16666666666666666, 424, A::sub(A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), s.ad_value(427)), 1.0);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && s.b[1122]) {
            s.store_div(437, 437, 246);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && (!s.b[1122])) {
            s.copy_ad(437, 424);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1121])) {
            s.store_scaled_add(437, 359, 356, 0.5);
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_scale(328, 240, 2.0);
            s.store_mul_sub_rhs(512, 328, 510, 170);
            s.store_add(191, 164, 512);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[737]) && s.b[1089]) {
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

        s.b[1123] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        s.b[1124] = (4.0 == 1.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && s.b[1123]) && s.b[1124]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1125] = (4.0 == 2.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && s.b[1125]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1126] = (4.0 == 4.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if ((((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && (!s.b[1125])) && s.b[1126]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1127] = (4.0 == 8.0);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if (((((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && (!s.b[1125])) && (!s.b[1126])) && s.b[1127]) {
            s.store_scalar(55, 4.0);
        }

        if (((!s.b[737]) && s.b[1089]) && s.b[1123]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign17010_loop_guard: usize = 0;
        while {
            let assign17010_cond_e24557: f64 = if ((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign17010_cond_e24557 != 0.0
        } {
            assign17010_loop_guard += 1;
            assert!(assign17010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[737]) && s.b[1089]) && s.b[1123]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1123])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(337, 336, 53, 1.0);
            s.store_sub_from_scalar(190, 1.0, 337);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if ((!s.b[737]) && s.b[1089]) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_div_scaled_product_indices(328, 192, 478, 0.6666666666666667, 479, 1.0);
        }

        s.b[1128] = (s.v[339] <= 1.0);
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        s.b[1129] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && s.b[1128]) && s.b[1129]) {
            s.store_sub_ad(436, A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));
            s.store_div(436, 436, 246);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1128]) && (!s.b[1129])) {
            s.copy_ad(436, 425);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1128])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.b[1133] = (s.v[612] == 0.0);
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if s.b[1133] {
            s.store_offset(480, 190, 0.5);
            s.store_mul(481, 479, 478);
            s.store_div_scaled_inputs(482, s.ad_value(480), 0.4, s.ad_value(481), 1.0);
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.b[1134] = (s.v[438] > (0.5 + 1e-8));
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1134]) {
            s.store_scalar(438, 0.5);
        }

        if s.b[1133] {
            s.copy_ad(439, 438);
            s.store_scalar(438, 0.5);
        }

        s.b[1136] = (s.v[145] == 0.0);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        s.b[1152] = ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16)));
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1136]) && s.b[1152]) {
            s.store_scalar(316, 0.0);
            s.copy_ad(314, 162);
        }

        s.b[1153] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && s.b[1152]) && s.b[1153]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_scalar(1151, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_div_from_scalar(1137, 1.0, 1151);
            s.store_mul(1138, 244, 1137);
            s.store_scale(1139, 1138, p.p191);
            s.store_add_scaled_product_indices(1142, 1139, 1.0, 80, 229, 1.0);
            s.store_div_from_scalar(1138, 1.0, 1142);
            s.store_scale(1141, 1138, 1.034943e-10);
            s.store_scalar(1138, (1.0 - p.p189));
            s.store_add_scaled_inputs_product_indices(314, 157, p.p189, 161, p.p189, 1138, 162, 1.0);
        }

        s.b[1154] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1154]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_sub(1144, 314, 162);
            s.store_sqrt_square_offset(44, 1144, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1143, 1144, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1155] = (s.v[1143] < 0.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1155]) {
            s.store_scalar(1143, 0.0);
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_mul(1140, 225, 244);
            s.store_div_from_scalar(1138, 1.0, 1140);
            s.store_mul(1142, 246, 1138);
        }

        s.b[1156] = (s.v[1142] < s.v[227]);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1156]) {
            s.copy_ad(1142, 227);
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_scale(1148, 229, 9662367879.197212);
            s.store_scalar(1138, (100000.0 * 10000.0));
            s.store_scalar(1139, (1.0 / s.v[97]));
            s.store_mul_ad_lhs(1150, A::add_scaled_inputs_product(s.ad_value(1142), 2.0, A::mul3_scaled_output(s.ad_value(1148), s.ad_value(1143), s.ad_value(1141), 2.0), 1.0, s.ad_value(1138), s.ad_value(1141), 1.0), 1139);
            s.store_mul(1145, 1150, 1141);
            s.store_add_scaled_product_indices(1149, 1138, 4.0, 1148, 1143, (2.0 * 4.0));
            s.store_mul3_lhs(1146, 1149, 1141, 1141);
            s.store_sqrt_square_add(1147, 1145, 1146);
            s.store_mul_scale_ad_rhs(316, 326, A::sub(s.ad_value(1147), s.ad_value(1145)), 0.5);
        }

        if (s.b[1133] && s.b[1136]) {
            s.store_scale(316, 316, s.v[127]);
        }

        if s.b[1133] {
            s.store_sub_from_scalar(441, s.v[97], 316);
        }

        s.b[1157] = (s.v[441] < 1e-9);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1157]) {
            s.store_scalar(441, 1e-9);
        }

        if s.b[1133] {
            s.store_scale(328, 108, (-s.v[98]));
            s.store_mul(196, 328, 437);
            s.store_mul(197, 328, 436);
            s.store_mul(198, 197, 438);
        }

        s.b[1158] = (p.p43 == 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1158]) {
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_mul_scale_ad_lhs(392, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 108);
        }

        if s.b[1133] {
            s.store_scaled_sub(1159, 157, 164, 0.5);
            s.store_scale(44, 1159, (2.0 * 1.0 / (p.p227)));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.b[1160] = (s.v[177] < (10.0 * 2.220446049250313e-16));
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1160]) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if s.b[1133] {
            s.store_add(176, 161, 177);
            s.store_scalar(1170, (1.034943e-10 / 100.0));
            s.store_scale(1171, 437, 0.0001);
            s.store_scale(1172, 436, 0.0001);
            s.store_div_from_scalar(1161, p.p92, 1170);
            s.store_div_from_scalar(1162, p.p93, 1170);
            s.store_scalar(1163, p.p94);
            s.store_offset_mul_ad(1164, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1163), 1.0);
            s.store_add_scaled_products_indices(1165, 1161, 1171, 1.0, 1162, 1172, 1.0);
            s.store_div(1166, 1165, 1164);
            s.copy_ad(248, 1166);
            s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));
            s.store_offset_add_scaled_inputs_indices(1163, 248, 0.5, 44, 0.5, (1e-10 * 3000.0));
        }

        s.b[1173] = (s.v[1163] < 0.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1173]) {
            s.store_scalar(1163, 0.0);
        }

        if s.b[1133] {
            s.store_powf(1165, 1163, (p.p97 - 1.0));
            s.store_mul(1167, 1165, 1163);
            s.store_powf(1168, 1163, (s.v[111] - 1.0));
            s.store_mul(1169, 1168, 1163);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1133] {
            s.store_scale(249, 1172, 6.241449993689894e18);
            s.store_add_scaled_ad_lhs(1161, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p.p96 * 1e-11), p.p95)), 1.0, s.ad_value(543), s.ad_value(1167), 1.0), 1169, 1.0 / (p.p106));
            s.store_div_from_scalar(251, 1.0, 1161);
            s.store_scale(251, 251, 0.0001);
            s.store_mul3_lhs(1174, 225, 244, 441);
            s.store_sqrt_square_offset(44, 1174, ((4.0 * 1e-50) * 1e-50));
            s.store_offset_add_scaled_inputs_indices(1174, 1174, 0.5, 44, 0.5, (1e-10 * 1e-50));
        }

        s.b[1182] = (s.v[1174] < 0.0);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1182]) {
            s.store_scalar(1174, 0.0);
        }

        if s.b[1133] {
            s.store_div_from_scalar(1175, 1.0, 1174);
            s.store_mul(1176, 246, 1175);
            s.store_div_scaled_inputs(1174, s.ad_value(253), 0.2, s.ad_value(251), 1.0);
            s.store_sqrt_square_sum(252, 1176, 1174);
            s.store_mul(1177, 251, 252);
            s.store_div(1175, 1177, 253);
        }

        s.b[1183] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1183]) {
            s.store_scalar(1178, 1.0);
        }

        s.b[1184] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1133] && (!s.b[1183])) && s.b[1184]) {
            s.copy_ad(1178, 1175);
        }

        if ((s.b[1133] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_powf(1178, 1175, (p.p113 - 1.0));
        }

        if s.b[1133] {
            s.store_mul(1174, 1175, 1178);
            s.store_offset(1179, 1174, 1.0);
        }

        s.b[1185] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1185]) {
            s.store_div_from_scalar(1180, 1.0, 1179);
        }

        s.b[1186] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if ((s.b[1133] && (!s.b[1185])) && s.b[1186]) {
            s.store_div_from_scalar_sqrt_ad(1180, 1.0, s.ad_value(1179));
        }

        if ((s.b[1133] && (!s.b[1185])) && (!s.b[1186])) {
            s.store_powf(1181, 1179, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1180, 1179, 1181);
        }

        if s.b[1133] {
            s.store_mul(250, 251, 1180);
            s.store_div_scaled_product_denominator_ad(264, 107, 227, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0);
            s.store_mul3_lhs(200, 264, 246, 250);
            s.store_scalar(201, 0.0);
        }

        s.b[1196] = ((p.p281 > 0.0) && (p.p244 != 0.0));
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1196]) {
            s.store_scaled_sub(1187, 157, 164, 0.5);
            s.store_scale(44, 1187, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(1193, 0.01, 45);
            s.store_sub_from_scalar_ad(1187, 1.1, A::add(s.ad_value(161), s.ad_value(1193)));
            s.store_sqrt_square_offset(44, 1187, ((4.0 * 0.05) * 0.05));
            s.store_offset_add_scaled_inputs_indices(1195, 1187, 0.5, 44, 0.5, (1e-10 * 0.05));
        }

        s.b[1197] = (s.v[1195] < 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1196]) && s.b[1197]) {
            s.store_scalar(1195, 0.0);
        }

        if (s.b[1133] && s.b[1196]) {
            s.store_scale(1188, 225, s.v[116]);
            s.store_mul(1189, 323, 1188);
            s.store_powf(1188, 1195, p.p245);
            s.store_mul(1190, 1189, 1188);
            s.store_offset_scaled(1191, 173, p.p246, 1.0);
            s.store_scalar(1188, s.v[117]);
        }

        s.b[1198] = ((s.v[56] < 3.0) || (p.p43 == 1.0));
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1196]) && s.b[1198]) {
            s.store_add_scaled_inputs3(1192, s.ad_value(161), 1.0, s.ad_value(1193), 1.0, s.ad_value(172), -1.0);
        }

        if ((s.b[1133] && s.b[1196]) && (!s.b[1198])) {
            s.store_add_scaled_inputs3(1192, s.ad_value(161), 1.0, s.ad_value(1193), 1.0, s.ad_value(350), -1.0);
        }

        if (s.b[1133] && s.b[1196]) {
            s.store_add_ad_rhs(1191, 1191, A::mul3(s.ad_value(173), s.ad_value(1188), s.ad_value(1192)));
            s.store_mul(1193, 1190, 1191);
            s.copy_ad(1190, 1193);
        }

        if (s.b[1133] && (!s.b[1196])) {
            s.store_scalar(1190, 0.0);
        }

        s.b[1199] = (p.p248 != 0.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1199]) {
            s.store_scale(1187, 225, s.v[118]);
            s.store_mul(1195, 323, 1187);
            s.store_mul(1194, 1195, 173);
        }

        if (s.b[1133] && (!s.b[1199])) {
            s.store_scalar(1194, 0.0);
        }

        s.b[1200] = ((s.v[1190] + s.v[1194]) > 0.0);
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1200]) {
            s.store_mul_add_rhs(247, 164, 1190, 1194);
            s.store_mul3_lhs(201, 264, 247, 250);
        }

        if s.b[1133] {
            s.store_add(199, 200, 201);
            s.copy_ad(203, 201);
        }

        s.b[1210] = (p.p33 != 0.0);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1210]) {
            s.copy_ad(1203, 549);
            s.store_scalar(1204, (s.v[124] - p.p71));
            s.store_div_from_scalar_square_ad(1205, 1.0, s.ad_value(1204));
            s.store_mul_ad_product_lhs(1206, A::mul_sub_from_scalar_lhs_scaled_output(p.p69, s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), s.ad_value(1203), 1205);
            s.store_mul(186, 1206, 235);
            s.store_offset_scaled(1202, 173, p.p155, p.p154);
            s.store_mul(206, 186, 1202);
            s.store_sub_from_scalar_scaled_input(1201, p.p156, 157, p.p157);
            s.store_add_scaled_inputs3_offset(207, s.ad_value(174), 1.0, s.ad_value(1201), 1.0, s.ad_value(206), 1.0, (-s.v[123]));
            s.store_mul3_lhs(210, 205, 324, 324);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_offset_sub_ad(1207, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p.p156)))), s.ad_value(206), 1e-50);
            s.store_offset_sub(1201, 174, 1207, (-0.005));
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_scalar(327, (if (s.v[1207] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_ad(1203, A::add_scaled_square_product(s.ad_value(1201), 1.0, s.ad_value(327), s.ad_value(1207), (4.0 * 0.005)));
            s.store_sub_ad_lhs(1204, A::add_scaled_inputs4_offset(s.ad_value(1207), 1.0, s.ad_value(1201), 0.5, s.ad_value(1203), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p.p156))), 514);
            s.store_offset_mul(1205, 225, 1204, (-1.0));
            s.store_div_from_scalar(1206, 4.0, 212);
            s.store_offset_mul(1202, 1205, 1206, 1.0);
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1211] = (s.v[1201] < 0.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1211]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_offset_input(213, 1201, 1e-50);
            s.store_add_ad_rhs(215, 207, A::mul_sub_from_scalar_rhs(s.ad_value(211), 1.0, s.ad_value(213)));
            s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));
            s.store_mul_ln_ad_lhs(216, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))), 327);
            s.store_div_scaled_value_offset_denominator(1204, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);
            s.store_offset_sub(217, 216, 215, (-0.002));
            s.store_sqrt_add_scaled_square_input(327, 217, 1.0, 216, (4.0 * 0.002));
            s.store_add_scaled_inputs3(218, s.ad_value(216), 1.0, s.ad_value(217), (-0.5), s.ad_value(327), (-0.5));
            s.store_div_from_scalar(1201, 1.0, 327);
            s.store_mul_exp_ad_rhs(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));
            s.store_add_ad_lhs(1202, A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0)), 327);
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1212] = (s.v[1201] < 0.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1212]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_offset_input(219, 1201, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(1202, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1213] = (s.v[1201] < 0.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1213]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_offset_input(220, 1201, (10.0 * 2.220446049250313e-16));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_sub(1202, 215, 218);
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1214] = (s.v[1201] < 0.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1214]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_div_scaled_value_offset_denominator(1208, s.ad_value(157), 1.0, s.ad_value(1201), (10.0 * 2.220446049250313e-16), 1.0);
            s.store_square(49, 1208);
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
        if (s.b[1133] && s.b[1210]) {
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1215] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        s.b[1216] = (4.0 == 1.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1210]) && s.b[1215]) && s.b[1216]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1217] = (4.0 == 2.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && s.b[1217]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1218] = (4.0 == 4.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if (((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && s.b[1218]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1219] = (4.0 == 8.0);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && (!s.b[1218])) && s.b[1219]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1133] && s.b[1210]) && s.b[1215]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19490_loop_guard: usize = 0;
        while {
            let assign19490_cond_e26982: f64 = if (((s.b[1133] && s.b[1210]) && s.b[1215]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19490_cond_e26982 != 0.0
        } {
            assign19490_loop_guard += 1;
            assert!(assign19490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1133] && s.b[1210]) && s.b[1215]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1133] && s.b[1210]) && (!s.b[1215])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(1209, 1208, 53, 1.0);
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
            s.store_div_scaled_product_left_ad(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1209, 1.0, 441, 1.0);
            s.store_add(199, 199, 222);
        }

        s.b[1220] = ((p.p30 != 0.0) && (p.p32 != 0.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1220]) {
            s.store_square(294, 192);
            s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);
            s.store_sub(296, 294, 295);
            s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(294, 294, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1221] = (s.v[294] < 0.0);
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1220]) && s.b[1221]) {
            s.store_scalar(294, 0.0);
        }

        if (s.b[1133] && s.b[1220]) {
            s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(296, 296, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1222] = (s.v[296] < 0.0);
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1220]) && s.b[1222]) {
            s.store_scalar(296, 0.0);
        }

        if (s.b[1133] && s.b[1220]) {
            s.store_sub(297, 294, 296);
        }

        s.b[1223] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1220]) && s.b[1223]) {
            s.store_scalar(146, 0.0);
        }

        if ((s.b[1133] && s.b[1220]) && (!s.b[1223])) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.b[1224] = ((p.p281 > 0.0) && (p.p285 > 0.0));
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if s.b[1224] {
            s.store_scalar(1231, s.v[99]);
            s.store_scalar(1235, p.p237);
            s.store_offset_add_scaled_inputs3_offset(1236, s.ad_value(158), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[123]), (-p.p286));
            s.store_offset(1237, 182, p.p286);
            s.store_scalar(1239, p.p285);
            s.store_scalar(1238, p.p283);
            s.store_scalar(1229, s.v[70]);
            s.store_mul_ln_ad_rhs(1230, 227, A::div_scaled_product_by_product(s.ad_value(1229), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));
        }

        if s.b[1224] {
            if (p.p43 == 1.0) {
                s.copy_ad(1227, 435);
            } else {
                s.copy_ad(1227, 350);
            }
        }

        if s.b[1224] {
            s.store_sqrt_ad(1232, A::div_scaled_product3(A::sub(s.ad_value(1230), s.ad_value(1227)), s.ad_value(536), s.ad_value(1229), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1229)), 1.0));
            s.store_mul(1226, 1232, 1231);
            s.store_div_scaled_product_add_scaled_denominator_indices(1225, 1226, 1226, (-0.25), 157, 1.0, 1226, 1.0, 1.0);
            s.copy_ad(1251, 1225);
            s.copy_ad(1252, 1237);
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(1236), s.ad_value(1251))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if s.b[1224] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if s.b[1224] {
            s.store_add_ad_rhs(376, 1236, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
        }

        s.b[1253] = (s.v[158] < ((s.v[123] + s.v[1252]) * 0.5));
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1253]) {
            s.store_scalar(144, 0.0);
        }

        s.b[1254] = ((s.v[144] == 0.0) || (1.0 != 0.0));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1254]) {
            s.store_mul_sub_rhs(181, 225, 376, 1251);
        }

        s.b[1255] = (s.v[181] < 3.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if ((s.b[1224] && s.b[1254]) && s.b[1255]) {
            s.store_mul_sub_rhs(337, 225, 1236, 1251);
            s.store_div_from_scalar_ad(328, 1.0, A::mul_scaled_lhs(s.ad_value(225), (1.414213562373095 / 108.0), s.ad_value(240)));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 1251, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1256] = ((s.v[158] - s.v[383]) <= s.v[1252]);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        s.b[1257] = (p.p43 == 0.0);
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if ((((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1235, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1236, 331, 323);
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) {
            s.copy_ad(378, 376);
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1236), s.ad_value(383)), A::sub(s.ad_value(1236), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(378, s.ad_value(377), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        s.b[1258] = (p.p43 == 0.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        s.b[1259] = ((s.v[158] - s.v[383]) <= s.v[1252]);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && s.b[1259]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1235, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1236, 331, 323);
            s.copy_ad(378, 376);
        }

        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1235, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 1236, 331, 323);
            s.copy_ad(378, 376);
        }

        s.b[1260] = ((s.v[1236] - s.v[383]) > 0.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if ((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1236), s.ad_value(383)), A::sub(s.ad_value(1236), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
        }

        s.b[1261] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
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
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1262] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        s.b[1263] = (2.0 == 1.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && s.b[1263]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1264] = (2.0 == 2.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && s.b[1264]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1265] = (2.0 == 4.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && s.b[1265]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1266] = (2.0 == 8.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && (!s.b[1265])) && s.b[1266]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20720_loop_guard: usize = 0;
        while {
            let assign20720_cond_e28554: f64 = if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20720_cond_e28554 != 0.0
        } {
            assign20720_loop_guard += 1;
            assert!(assign20720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && (!s.b[1262])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && (!s.b[1261])) {
            s.copy_ad(378, 376);
        }

        if s.b[1224] {
            s.store_offset(336, 1251, (5e-12 / 2.0));
        }

        s.b[1267] = (s.v[378] < s.v[336]);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1267]) {
            s.copy_ad(378, 336);
        }

        if s.b[1224] {
            s.copy_ad(1234, 378);
            s.copy_ad(163, 376);
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            if ((s.v[376] - s.v[1234]) >= 0.0) {
                s.store_sub(166, 376, 1234);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p.p287)) + ((-0.03))));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(165, s.ad_value(166), (1.0 + 0.3), s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[1268] = (s.v[165] < 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if ((s.b[1224] && (0.0 != 0.0)) && s.b[1268]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1269] = (s.v[165] > s.v[157]);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if (((s.b[1224] && (0.0 != 0.0)) && (!s.b[1268])) && s.b[1269]) {
            s.copy_ad(165, 157);
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_add(163, 1234, 165);
        }

        s.b[1270] = (p.p282 == 1.0);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1270]) {
            s.copy_ad(378, 1234);
            s.copy_ad(1271, 1225);
            s.store_offset_add_scaled_inputs3_offset(160, s.ad_value(185), (-1.0), s.ad_value(320), 1.0, s.ad_value(1271), 1.0, s.v[123], p.p286);
        }

        s.b[1273] = (s.v[158] < s.v[160]);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ad_rhs(254, 227, 2.0, A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
            s.store_mul_sub_rhs(336, 225, 1236, 1271);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1274] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && s.b[1274]) {
            s.store_add_scaled_inputs3_offset(257, s.ad_value(261), 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, s.ad_value(332), 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && (!s.b[1274])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 1271, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 1271);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_ad_lhs(1234, A::div(s.ad_value(328), s.ad_value(330)), 1271);
        }

        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
            s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1271), (-p.p287)));
            s.store_scalar(430, 0.0);
            s.copy_ad(1272, 378);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_ad(327, A::mul_scaled_lhs(s.ad_value(225), 2.0, s.ad_value(419)));
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
            s.store_scalar(167, 1.0);
        }

        let mut assign21320_loop_guard: usize = 0;
        while {
            let assign21320_cond_e29284: f64 = (s.v[57] + 1.0);
            let assign21320_cond_e29286: f64 = if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (s.v[167] <= assign21320_cond_e29284)) { 1.0 } else { 0.0 };
            assign21320_cond_e29286 != 0.0
        } {
            assign21320_loop_guard += 1;
            assert!(assign21320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_sub(417, 1272, 1271);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1275] = (s.v[337] < 80.0);
            s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1275]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1275])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_mul(421, 225, 422);
            }
            s.b[1276] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1277] = (s.v[181] < 0.0);
            s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) && s.b[1277]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1278] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && s.b[1278]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && (!s.b[1278])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1279] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1279]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1280] = (s.v[181] < 0.0);
            s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1280]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1281] = (s.v[181] < 1e-7);
            s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && s.b[1281]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && (!s.b[1281])) {
                s.store_mul_offset_rhs(501, 225, 1272, (-p.p287));
                s.store_exp(502, 501);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_add_scaled_inputs_product_indices(492, 1272, 1.0, 1236, (-1.0), 240, 490, 1.0);
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1282] = (s.v[430] == 1.0);
            s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1282]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_div_scaled_inputs(494, s.ad_value(492), -1.0, s.ad_value(493), 1.0);
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1272]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1272))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1283] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1283]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_add(1272, 1272, 494);
            }
            s.b[1284] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1284]) {
                s.store_scalar(430, 1.0);
            }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
            s.copy_ad(1234, 1272);
        }

        if s.b[1224] {
            s.store_mul_scaled_ad_rhs(332, 225, -1.0, A::sub(s.ad_value(1234), s.ad_value(1225)));
        }

        if s.b[1224] {
            s.store_scalar(1249, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[1224] {
            s.store_mul(1250, 1249, 332);
            s.store_exp(333, 332);
            s.store_sub_ad_lhs(334, A::offset(s.ad_value(333), (-1.0)), 332);
        }

        s.b[1285] = (s.v[332] > 1e-7);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1224] && s.b[1285]) {
            s.store_mul_scaled_ad_rhs(437, 238, -1.0, A::sqrt(s.ad_value(334)));
        }

        s.b[1286] = (s.v[1250] > 1e-7);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if ((s.b[1224] && (!s.b[1285])) && s.b[1286]) {
            s.store_mul_sqrt_rhs(437, 238, 334);
        }

        if ((s.b[1224] && (!s.b[1285])) && (!s.b[1286])) {
            s.store_mul_ad_affine_product_rhs(437, 1249, s.ad_value(1250), A::sqrt(A::offset(A::mul_scaled_lhs(s.ad_value(1250), 0.3333333333333333, A::scale_offset(s.ad_value(1250), 0.25, 1.0)), 1.0)), (-0.7071067811865475), 0.0);
        }

        if s.b[1224] {
            s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_add_scaled_inputs_indices(1246, 437, 0.5, 44, 0.5, (1e-10 * 1e-6));
        }

        s.b[1287] = (s.v[1246] < 0.0);
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1287]) {
            s.store_scalar(1246, 0.0);
        }

        if s.b[1224] {
            s.store_div_scaled_inputs(1247, s.ad_value(1246), 1.0, s.ad_value(536), 1.6021918e-19);
            s.store_sub(328, 1247, 1238);
            s.store_scale(1248, 1247, 0.01);
            s.store_sqrt_ad(44, A::add_scaled_square_product(s.ad_value(328), 1.0, s.ad_value(1248), s.ad_value(1248), 4.0));
            s.store_add_scaled_inputs3(329, s.ad_value(328), 0.5, s.ad_value(44), 0.5, s.ad_value(1248), 1e-10);
        }

        s.b[1288] = (s.v[329] < 0.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1288]) {
            s.store_scalar(329, 0.0);
        }

        if s.b[1224] {
            s.store_div_scaled_product_by_product(1245, s.ad_value(329), s.ad_value(329), 1.0, s.ad_value(1247), s.ad_value(1247), 1.0);
            s.store_add_scaled_product_left_ad(1228, 1225, 1.0, A::sub(s.ad_value(1234), s.ad_value(1225)), 1245, 1.0);
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1228))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1228), s.ad_value(157)))));
            s.store_sqrt_scaled_input(1241, 1229, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_mul_sqrt_rhs(1242, 1241, 227);
            s.store_mul_sub_rhs(1233, 225, 1228, 1225);
        }

        s.b[1289] = ((s.v[1233] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1289]) {
            s.store_sub_scaled_inputs(44, 225, 0.2, 1233, 1.0);
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

        s.b[1290] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        s.b[1291] = (1.0 == 1.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((s.b[1224] && s.b[1289]) && s.b[1290]) && s.b[1291]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1292] = (1.0 == 2.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if ((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && s.b[1292]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1293] = (1.0 == 4.0);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if (((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1294] = (1.0 == 8.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21850_loop_guard: usize = 0;
        while {
            let assign21850_cond_e30601: f64 = if (((s.b[1224] && s.b[1289]) && s.b[1290]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21850_cond_e30601 != 0.0
        } {
            assign21850_loop_guard += 1;
            assert!(assign21850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1224] && s.b[1289]) && (!s.b[1290])) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if (s.b[1224] && s.b[1289]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);
            s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);
        }

        if (s.b[1224] && (!s.b[1289])) {
            s.copy_ad(328, 1233);
        }

        if s.b[1224] {
            s.store_sqrt_offset_input(1243, 328, (10.0 * 2.220446049250313e-16));
            s.store_mul(1244, 1242, 1243);
            s.store_mul_div_scaled_inputs_rhs(1240, 1244, s.ad_value(227), 2.0, s.ad_value(1231), 1.0);
            s.store_mul_product3_rhs(204, 337, s.ad_value(1240), s.ad_value(1239), s.ad_value(107), 1.0);
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.b[1295] = ((p.p43 == 1.0) || (p.p45 == 1.0));
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        s.b[1308] = ((s.v[145] == 1.0) || (p.p25 == 0.0));
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (s.b[1295] && s.b[1308]) {
            s.store_scalar(263, 0.0);
        }

        s.b[1309] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((s.b[1295] && (!s.b[1308])) && s.b[1309]) {
            s.store_scalar(263, 0.0);
        }

        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {
            s.store_offset_add_scaled_inputs3_offset(445, s.ad_value(174), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[136]), p.p48);
        }

        s.b[1310] = (p.p44 <= 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {
            s.copy_ad(1296, 445);
            s.store_square(1303, 323);
            s.copy_ad(1304, 545);
            s.store_div(1298, 1304, 1303);
            s.store_div_from_scalar(1305, 2.0, 1304);
            s.store_mul(1299, 1305, 1303);
            s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1302, 1299, 1300, 1.0);
            s.store_sqrt_square_offset(44, 1302, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1301, 1302, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1311] = (s.v[1301] < 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1311]) {
            s.store_scalar(1301, 0.0);
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {
            s.store_offset(1301, 1301, 1e-50);
            s.store_sqrt(1301, 1301);
            s.store_add_scaled_product_value_ad(1306, A::mul_sub_from_scalar_rhs(s.ad_value(1298), 1.0, s.ad_value(1301)), 1.0, 1296, 137, 1.0);
            s.store_add_scaled_inputs3(1307, s.ad_value(173), p.p122, s.ad_value(176), 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1306)), -1.0);
            s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1312] = (s.v[1307] < 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1312]) {
            s.store_scalar(1307, 0.0);
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            s.store_mul(1296, 134, 445);
            s.store_div_ad_rhs(1298, 545, A::square(s.ad_value(323)));
            s.store_mul_ad(1299, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
            s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));
            s.store_offset_mul(1301, 1299, 1300, 1.0);
            s.store_scaled_offset(1303, 1299, 1.0, 2.0);
        }

        s.b[1313] = ((s.v[1301] < (1e-50 + s.v[1303])) && (s.v[1303] >= 0.0));
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(1303), 1e-50), 1301);
            s.store_square(49, 44);
            s.store_square(50, 1303);
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

        s.b[1314] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        s.b[1315] = (4.0 == 1.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && s.b[1315]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1316] = (4.0 == 2.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && s.b[1316]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1317] = (4.0 == 4.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if ((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1318] = (4.0 == 8.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if (((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) && s.b[1318]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22640_loop_guard: usize = 0;
        while {
            let assign22640_cond_e31720: f64 = if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22640_cond_e31720 != 0.0
        } {
            assign22640_loop_guard += 1;
            assert!(assign22640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && (!s.b[1314])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 1303, 53);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
            s.store_sub_ad_lhs(1301, A::offset(s.ad_value(1303), 1e-50), 43);
        }

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && (!s.b[1313])) {
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            if (s.v[1301] <= 0.0) {
                s.store_scalar(1301, 0.0);
            } else {
                s.store_sqrt(1301, 1301);
            }
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            s.store_add_ad_rhs(1306, 1296, A::mul_sub_from_scalar_rhs(s.ad_value(1298), 1.0, s.ad_value(1301)));
            s.store_div_from_scalar_offset_input(1297, s.v[100], 131, s.v[100]);
            s.store_add_scaled_inputs_product_indices(1307, 173, p.p122, 176, 1.0, 1297, 1306, (-1.0));
            s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1319] = (s.v[1307] < 0.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1319]) {
            s.store_scalar(1307, 0.0);
        }

        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {
            s.store_offset(1307, 1307, 1e-50);
            s.store_exp_ad(1297, A::div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1307), 1.0));
            s.store_mul_product3_rhs(263, 1297, s.ad_value(132), s.ad_value(1307), s.ad_value(199), 1.0);
        }

        s.b[1320] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if s.b[1320] {
            s.store_scale(1324, 227, 0.0);
            s.store_add_scaled_inputs3(44, s.ad_value(231), 1.0, s.ad_value(1324), (-1.0), s.ad_value(231), (-0.01));
            s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));
        }

        if s.b[1320] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1320] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(1324, s.ad_value(231), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sqrt_ad(1325, A::mul_scaled_lhs(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(227)));
            s.store_mul_sub_rhs(1326, 225, 176, 1324);
        }

        if s.b[1320] {
            if (s.v[1326] > 0.0) {
                s.store_sqrt(1326, 1326);
            } else {
                s.store_neg_ad(1326, A::sqrt_scaled_input(s.ad_value(1326), -1.0));
            }
        }

        if s.b[1320] {
            s.store_sqrt_mul(1327, 225, 176);
            s.store_mul_scaled_ad_rhs(1328, 1325, -1.0, A::sub(s.ad_value(1326), s.ad_value(1327)));
            s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1328), (-(p.p47 * 0.01)));
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if s.b[1320] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[1320] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p.p47);
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
        }

        s.b[1342] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        s.b[1343] = (s.v[56] < 3.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if (s.b[1342] && s.b[1343]) {
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
        }

        if (s.b[1342] && (!s.b[1343])) {
            if (p.p43 == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }

        if (s.b[1342] && (!s.b[1343])) {
            if (p.p43 == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }

        if s.b[1342] {
            s.store_offset_scaled(1329, 185, p.p147, 1.0);
            s.store_scaled_mul(1330, 1329, 263, p.p146);
            s.store_offset_mul_ad(1331, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));
            s.store_sqrt_square_offset(44, 1331, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1331, 1331, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1344] = (s.v[1331] < 0.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (s.b[1342] && s.b[1344]) {
            s.store_scalar(1331, 0.0);
        }

        if s.b[1342] {
            s.store_sqrt(1332, 1331);
            s.store_mul(1333, 1331, 1332);
            s.store_offset_mul_ad(1334, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));
            s.store_sqrt_square_offset(44, 1334, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(1334, 1334, 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1345] = (s.v[1334] < 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if (s.b[1342] && s.b[1345]) {
            s.store_scalar(1334, 0.0);
        }

        if s.b[1342] {
            s.store_sqrt(1335, 1334);
            s.store_mul(1336, 1334, 1335);
            s.store_div_from_scalar(1337, 1.0, 1331);
            s.store_mul3_lhs(328, 225, 1330, 1337);
            s.store_div_from_scalar(1337, 1.0, 1334);
            s.store_mul3_lhs(1338, 225, 1330, 1337);
            s.store_mul_ad_rhs(1339, 238, A::add_scaled_products(s.ad_value(1336), s.ad_value(1338), 1.0, s.ad_value(1333), s.ad_value(328), (-1.0)));
            s.store_mul_scaled_ad_rhs(1340, 238, 0.5, A::add_scaled_products(s.ad_value(1335), s.ad_value(1338), -1.0, s.ad_value(1332), s.ad_value(328), 1.0));
            s.store_add(1341, 1339, 1340);
            s.store_mul3_lhs(265, 264, 1341, 250);
        }

        s.v[1359] = (s.v[88] * 100.0);

        s.store_scale(1360, 323, 0.0001);

        s.v[1361] = (s.v[97] * 100.0);

        s.store_scale(1362, 107, 100.0);

        s.store_scale(1363, 252, 0.01);

        s.store_scale(1364, 436, 0.0001);

        s.store_scale(1365, 238, 0.0001);

        s.b[1366] = (p.p27 == 0.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        s.b[1367] = (s.v[145] == 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_offset_add(1358, 176, 173, (-(10.0 * 2.220446049250313e-16)));
            s.store_add_scaled_inputs4_offset(1348, s.ad_value(174), 1.0, s.ad_value(185), (p.p216 * s.v[1361]), s.ad_value(320), (-(p.p216 * s.v[1361])), s.ad_value(1358), (-p.p215), (-s.v[123]));
            s.store_scalar(1350, (1.0 / s.v[1359]));
            s.store_mul(1349, 1348, 1350);
            s.store_scalar(1350, (1.0 / p.p217));
            s.store_offset_mul(1354, 1363, 1350, 1.0);
            s.store_mul(1357, 1349, 1354);
            s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1368] = (s.v[1357] < 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((!s.b[1366]) && s.b[1367]) && s.b[1368]) {
            s.store_scalar(1357, 0.0);
        }

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(1350, 174, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1369] = (s.v[1350] < 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if (((!s.b[1366]) && s.b[1367]) && s.b[1369]) {
            s.store_scalar(1350, 0.0);
        }

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_offset(1350, 1350, (-p.p226));
            s.store_scale(1346, 1350, 10.0);
            s.store_offset_square(1349, 1346, 1.0);
            s.store_sub_from_scalar_ad(1348, 1.0, A::div_from_scalar(1.0, s.ad_value(1349)));
            s.store_mul(1357, 1357, 1348);
            s.store_scale(1347, 1362, s.v[1361]);
            s.store_div_from_scalar_offset_input(1354, p.p219, 1347, p.p219);
            s.store_scalar(1353, p.p218);
            s.store_div_from_scalar_offset_input(1351, 1.0, 1357, 1e-50);
            s.store_scaled_mul(1348, 303, 1351, (-p.p214));
        }

        s.b[1370] = (s.v[1348] < (-34.0));
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if (((!s.b[1366]) && s.b[1367]) && (!s.b[1370])) {
            s.store_exp(1349, 1348);
            s.store_mul_scale_ad_lhs(1350, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 1347);
            s.store_div_from_scalar(1352, 1.0, 1365);
            s.store_sqrt_mul_ad(1353, A::add_scaled_inputs(s.ad_value(1364), 1.0, s.ad_value(1360), 1e-12), s.ad_value(1352));
            s.store_mul3_lhs(1351, 1349, 1350, 1353);
        }

        if (!s.b[1366]) {
            s.store_offset_scaled(1347, 158, (-p.p221), p.p222);
            s.store_exp_scaled_input(1349, 1347, s.v[1359]);
            s.store_scale(1347, 158, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));
            s.store_mul(1350, 158, 1347);
            s.store_scale(1351, 1362, (p.p220 / 1000000.0));
            s.store_sub(1348, 158, 157);
            s.store_offset_scaled(1347, 1348, (-p.p221), p.p222);
            s.store_exp_scaled_input(1349, 1347, s.v[1359]);
            s.store_scale(1347, 1348, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));
            s.store_mul(1350, 1348, 1347);
            s.store_scale(1351, 1362, (p.p220 / 1000000.0));
            s.store_offset_scaled_sub(1357, 513, 158, 1.0 / (s.v[1359]), ((((s.v[123]) + (p.p225))) * (1.0 / (s.v[1359]))));
            s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1373] = (s.v[1357] < 0.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && s.b[1373]) {
            s.store_scalar(1357, 0.0);
        }

        if (!s.b[1366]) {
            s.store_offset(1357, 1357, 1e-50);
            s.store_div_from_scalar(1348, (-p.p224), 1357);
        }

        s.b[1374] = (s.v[1348] < (-34.0));
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && (!s.b[1374])) {
            s.store_exp(1349, 1348);
            s.store_scale(1350, 1362, (p.p223 * s.v[1361]));
        }

        s.b[1382] = (p.p28 == 0.0);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if (!s.b[1382]) {
            s.store_add_scaled_inputs4_offset(1375, s.ad_value(157), p.p209, s.ad_value(158), (-1.0), s.ad_value(187), p.p211, s.ad_value(319), p.p211, (p.p210 * p.p209));
            s.store_scalar(1376, (1.0 / s.v[88]));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1382]) {
            s.store_mul(1377, 1375, 1376);
            s.store_sqrt_square_offset(44, 1377, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(304, 1377, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1383] = (s.v[304] < 0.0);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((!s.b[1382]) && s.b[1383]) {
            s.store_scalar(304, 0.0);
        }

        if (!s.b[1382]) {
            s.store_div_from_scalar_offset_input(1378, 1.0, 304, 1e-50);
            s.store_scaled_mul(1379, 303, 1378, (-p.p208));
        }

        s.b[1384] = (s.v[1379] < (-34.0));
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if ((!s.b[1382]) && (!s.b[1384])) {
            s.store_exp(1375, 1379);
            s.store_mul_scale_ad_lhs(1376, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 107);
        }

        if (!s.b[1382]) {
            s.store_sub(1381, 157, 513);
        }

        s.b[1385] = (s.v[1381] > 0.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if ((!s.b[1382]) && s.b[1385]) {
            s.store_square(1376, 1381);
            s.store_mul(331, 1376, 1381);
            s.store_offset(1379, 331, p.p212);
        }

        s.b[1393] = (p.p28 == 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if (!s.b[1393]) {
            s.store_add_scaled_inputs3(1386, A::add_scaled_inputs3_offset(s.ad_value(157), (-p.p209), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p.p210) * (p.p209))), 1.0, s.ad_value(187), p.p211, s.ad_value(319), p.p211);
            s.store_scalar(1387, (1.0 / s.v[88]));
            s.store_mul(1388, 1386, 1387);
            s.store_sqrt_square_offset(44, 1388, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(305, 1388, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1394] = (s.v[305] < 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if ((!s.b[1393]) && s.b[1394]) {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[1393]) {
            s.store_div_from_scalar_offset_input(1389, 1.0, 305, 1e-50);
            s.store_scaled_mul(1390, 303, 1389, (-p.p208));
        }

        s.b[1395] = (s.v[1390] < (-34.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if ((!s.b[1393]) && (!s.b[1395])) {
            s.store_exp(1386, 1390);
            s.store_div_from_scalar(1389, 1.0, 302);
            s.store_scaled_mul(1387, 1389, 107, (p.p207 * 1.6021918e-19));
        }

        if (!s.b[1393]) {
            s.store_neg(1392, 513);
        }

        s.b[1396] = (s.v[1392] > 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if ((!s.b[1393]) && s.b[1396]) {
            s.store_square(1387, 1392);
            s.store_mul(331, 1387, 1392);
            s.store_offset(1390, 331, p.p212);
        }

        s.b[1397] = (p.p43 == 1.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if s.b[1397] {
            s.store_scalar(1407, s.v[91]);
            s.store_div_from_scalar(1408, 1.0, 1407);
            s.store_scalar(1464, 0.0);
            s.store_scalar(1466, 0.0);
            s.store_scalar(1468, 0.0);
            s.store_neg(1400, 534);
            s.store_mul(1401, 1400, 436);
            s.store_add_scaled_product_indices(331, 1401, 1.0, 1400, 437, 1.0);
            s.store_mul(470, 1401, 438);
            s.store_sub(469, 1401, 470);
            s.store_mul(468, 331, 438);
            s.store_sub(467, 331, 468);
        }

        if (s.b[1397] && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
            s.store_scalar(528, 0.0);
        }

        s.b[1477] = (1.0 == 1.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        s.b[1478] = (1.0 == 2.0);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_scale(522, 533, 0.5);
            s.store_scalar(523, p.p292);
            s.store_scalar(528, s.v[525]);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && (s.b[1478] && (!s.b[1477]))) {
            s.store_scale(522, 534, 0.5);
            s.store_scalar(523, p.p68);
            s.store_scalar(528, s.v[524]);
            s.store_scalar(528, 1.0);
        }

        s.b[1479] = (s.v[528] == 0.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_mul_sqrt_ad_rhs(1427, 238, A::div(s.ad_value(521), s.ad_value(536)));
            s.store_scalar(1409, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1410, ((1.0 + -1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1420, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1421, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1422, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1423, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1424, 1421, 1420);
            s.store_neg(1425, 1420);
            s.store_add_scaled_products_indices(1411, 1409, 461, 1.0, 1410, 462, 1.0);
            s.store_add_scaled_products_indices(1412, 1409, 462, 1.0, 1410, 461, 1.0);
            s.store_add_scaled_products_indices(1426, 1411, 1422, 1.0, 1412, 1423, 1.0);
            s.store_offset_ad(1418, A::add_scaled_products(s.ad_value(1411), s.ad_value(1425), 1.0, s.ad_value(1412), s.ad_value(1424), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1398, 1418);
        }

        s.b[1480] = (s.v[1398] > s.v[141]);
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1480]) {
            s.store_sub(1399, 1398, 141);
            s.store_sub(1400, 140, 141);
            s.store_div(44, 1399, 1400);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1406, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1406, 1400, 1.0, 1406);
            s.store_add(1403, 141, 1406);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1480])) {
            s.copy_ad(1403, 1398);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));
            s.store_mul(1428, 1427, 1408);
            s.store_square(1429, 1428);
            s.store_sub(1430, 1426, 523);
            s.store_div(1398, 521, 230);
            s.store_mul_ad(1431, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1398)));
            s.store_neg(1432, 1419);
        }

        s.b[1481] = (s.v[1430] < s.v[1432]);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) {
            s.store_div_from_scalar_mul_ad(1399, 1.0, s.ad_value(225), s.ad_value(1427));
            s.store_mul(1406, 1399, 1407);
            s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1434, 1433, 1433, 8.0, 0.0, 1433);
            s.store_sub(1435, 237, 1431);
            s.store_mul_add_rhs(1405, 225, 1430, 1419);
            s.store_sub_from_scalar_ad(1436, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1406), 9.0, A::offset(s.ad_value(1405), (-2.0))));
            s.store_square(1437, 1436);
        }

        s.b[1482] = (s.v[1434] < (s.v[1437] * 1e-8));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) && s.b[1482]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1439, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1434), 0.5, s.ad_value(1436), 1.0), 1.0, 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) && (!s.b[1482])) {
            s.store_sqrt_add(1438, 1434, 1437);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), 1.0, 1406, 1405, (-2.0), 9.0);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) {
            s.store_powf(1440, 1439, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1441, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), 1.0, 1440, 2.0, 1440, 1440, 1.414213562373095);
            s.store_div(1442, 1441, 1440);
            s.store_add_scaled_product_indices(1443, 1419, (-1.0), 1442, 227, 1.0);
            s.store_add(1399, 1443, 1419);
            s.store_div(1400, 1399, 1435);
            s.store_sqrt_square_offset(1401, 1400, 1.0);
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1399), s.ad_value(1401)), 1419);
            s.store_sub(1400, 1430, 1444);
            s.store_mul(459, 1407, 1400);
            s.copy_ad(458, 459);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_scalar(1442, 3.0);
            s.store_sub_ad_lhs(1445, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_scaled_inputs2(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, s.ad_value(1406), 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1483] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1483]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_add_ad_rhs(1445, 1430, A::mul3_scaled_output(s.ad_value(1429), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_scaled_inputs2(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, s.ad_value(1406), 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1484] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1484]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_add_ad_rhs(1445, 1430, A::mul3_scaled_output(s.ad_value(1429), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        s.b[1485] = (s.v[1442] < 3.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1485]) {
            s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1485]) {
            s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1449, s.ad_value(1430), -1.0, s.ad_value(1419), -1.0, s.ad_value(1428), 1.0);
            s.store_add_scaled_inputs3(1450, A::div_scaled_product(A::square(s.ad_value(1447)), s.ad_value(1447), 1.0, A::mul3_scaled_output(s.ad_value(1446), s.ad_value(1446), s.ad_value(1446), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1447), s.ad_value(1448), 1.0, s.ad_value(1446), s.ad_value(1446), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1449), 1.0, s.ad_value(1446), 2.0), 1.0);
            s.store_div_ad(1451, A::add_scaled_square_product(s.ad_value(1447), (-1.0), s.ad_value(1446), s.ad_value(1448), 3.0), A::mul_scaled_lhs(s.ad_value(1446), 9.0, s.ad_value(1446)));
            s.store_sqrt_ad(1402, A::add_scaled_square_product(s.ad_value(1450), 1.0, A::square(s.ad_value(1451)), s.ad_value(1451), 1.0));
            s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);
            s.store_neg_ad(1453, A::powf(A::add(s.ad_value(1450), s.ad_value(1402)), 0.3333333333333333));
            s.store_add_scaled_inputs3(1405, s.ad_value(1452), 1.0, s.ad_value(1453), 1.0, A::div_scaled_inputs(s.ad_value(1447), 1.0, s.ad_value(1446), 3.0), -1.0);
            s.store_add_scaled_product_indices(1445, 1419, (-1.0), 1405, 227, 1.0);
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_offset_add(1454, 1430, 1419, 0.1);
            s.store_offset_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0), 1e-50);
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1456, 1455, 1461);
            s.store_mul(1398, 226, 1429);
            s.store_mul(1457, 225, 1454);
            s.store_add_scaled_inputs_product_mixed_aaii(1458, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);
            s.store_offset_sub(44, 1457, 1458, (-1.0));
            s.store_scale(45, 1457, 4.0);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1458, s.ad_value(1457), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1457, 1457, 1458);
            s.store_add_scaled_inputs(1457, 1457, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1459, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);
            s.copy_ad(1460, 1442);
            s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));
            s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1442, s.ad_value(1459), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_add_ad(1399, A::offset(s.ad_value(1442), (-1.0)), A::exp_scaled_input(s.ad_value(1442), -1.0));
        }

        s.b[1486] = (s.v[1399] < (10.0 * 2.220446049250313e-16));
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1486]) {
            s.store_scalar(1399, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_sqrt(1400, 1399);
            s.store_mul(458, 1427, 1400);
            s.store_mul_sub_rhs(459, 1407, 1430, 1444);
        }

        s.b[1487] = (p.p42 == 1.0);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
            s.store_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0));
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1470, 1455, 1461);
            s.store_scalar(1415, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign26240_loop_guard: usize = 0;
        while {
            let assign26240_cond_e35811: f64 = (2.0 * 20.0);
            let assign26240_cond_e35813: f64 = (assign26240_cond_e35811 + 1.0);
            let assign26240_cond_e35815: f64 = if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (s.v[167] <= assign26240_cond_e35813)) { 1.0 } else { 0.0 };
            assign26240_cond_e35815 != 0.0
        } {
            assign26240_loop_guard += 1;
            assert!(assign26240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
                s.store_scalar(1466, 0.0);
                s.store_mul_add_rhs(1442, 225, 1444, 1419);
            }
            s.b[1488] = (s.v[1442] < 5.0);
            s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1488]) {
                s.store_mul3_ad_middle(1462, A::square(s.ad_value(1442)), 1442, A::offset(A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1463, A::square(s.ad_value(1442)), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1464, 1470, 1462, 1462);
                s.store_mul_product3_rhs(1465, 1463, s.ad_value(1470), s.ad_value(225), s.ad_value(1462), 2.0);
                s.store_mul_offset_ad_rhs(1466, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1467, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);
                s.store_div_scaled_inputs2(1469, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1467), s.ad_value(1466), 2.0), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 2.0);
            }
            s.b[1489] = (s.v[1442] < 80.0);
            s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {
                s.store_exp(243, 1442);
                s.store_mul_offset_rhs(1464, 1470, 243, (-1.0));
                s.store_mul3_lhs(1465, 1470, 225, 243);
            }
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {
                s.store_exp_mul(1471, 225, 1444);
                s.store_mul_sub_rhs(1464, 1455, 1471, 1461);
                s.store_mul3_lhs(1465, 1455, 225, 1471);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) {
                s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));
                s.store_scale_ad(1469, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 1.0), 0.5);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
                s.store_add_scaled_inputs_product_indices(1472, 1430, 1.0, 1444, (-1.0), 1428, 1468, (-1.0));
                s.store_sub_from_scalar_ad(1473, (-1.0), A::mul(s.ad_value(1428), s.ad_value(1469)));
            }
            s.b[1490] = (s.v[1415] == 1.0);
            s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1490]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_div_scaled_inputs(494, s.ad_value(1472), -1.0, s.ad_value(1473), 1.0);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1491] = (((s.v[494]) as f64).abs() > s.v[1474]);
            s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {
                s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_add(1444, 1444, 494);
            }
            s.b[1492] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));
            s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1492]) {
                s.store_scalar(1415, 1.0);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1494] = (s.v[1442] < 5.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1494]) {
            s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));
            s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1494])) {
            s.store_offset(1475, 1442, (-1.0));
            s.store_sqrt(1476, 1475);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
            s.store_mul(458, 1427, 1476);
            s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));
            s.store_mul3_lhs(460, 1427, 1464, 1399);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1496] = (1.0 == 1.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        s.b[1497] = (1.0 == 2.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_scalar(1409, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1410, ((1.0 + 1.0) / 2.0));
            s.store_add_scaled_products_right_right_ad(1420, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1421, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1422, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_right_right_ad(1423, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1424, 1421, 1420);
            s.store_neg(1425, 1420);
            s.store_add_scaled_products_indices(1411, 1409, 461, 1.0, 1410, 462, 1.0);
            s.store_add_scaled_products_indices(1412, 1409, 462, 1.0, 1410, 461, 1.0);
            s.store_add_scaled_products_indices(1426, 1411, 1422, 1.0, 1412, 1423, 1.0);
            s.store_offset_ad(1418, A::add_scaled_products(s.ad_value(1411), s.ad_value(1425), 1.0, s.ad_value(1412), s.ad_value(1424), 1.0), (10.0 * 2.220446049250313e-16));
            s.store_neg(1398, 1418);
        }

        s.b[1498] = (s.v[1398] > s.v[141]);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1498]) {
            s.store_sub(1399, 1398, 141);
            s.store_sub(1400, 140, 141);
            s.store_div(44, 1399, 1400);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1406, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1406, 1400, 1.0, 1406);
            s.store_add(1403, 141, 1406);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1498])) {
            s.copy_ad(1403, 1398);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));
            s.store_mul(1428, 1427, 1408);
            s.store_square(1429, 1428);
            s.store_sub(1430, 1426, 523);
            s.store_div(1398, 521, 230);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_mul_ad(1431, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1398)));
            s.store_neg(1432, 1419);
        }

        s.b[1499] = (s.v[1430] < s.v[1432]);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) {
            s.store_div_from_scalar_mul_ad(1399, 1.0, s.ad_value(225), s.ad_value(1427));
            s.store_mul(1406, 1399, 1407);
            s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1434, 1433, 1433, 8.0, 0.0, 1433);
            s.store_sub(1435, 237, 1431);
            s.store_mul_add_rhs(1405, 225, 1430, 1419);
            s.store_sub_from_scalar_ad(1436, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1406), 9.0, A::offset(s.ad_value(1405), (-2.0))));
            s.store_square(1437, 1436);
        }

        s.b[1500] = (s.v[1434] < (s.v[1437] * 1e-8));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) && s.b[1500]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1439, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1434), 0.5, s.ad_value(1436), 1.0), 1.0, 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) && (!s.b[1500])) {
            s.store_sqrt_add(1438, 1434, 1437);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), 1.0, 1406, 1405, (-2.0), 9.0);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) {
            s.store_powf(1440, 1439, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1441, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), 1.0, 1440, 2.0, 1440, 1440, 1.414213562373095);
            s.store_div(1442, 1441, 1440);
            s.store_add_scaled_product_indices(1443, 1419, (-1.0), 1442, 227, 1.0);
            s.store_add(1399, 1443, 1419);
            s.store_div(1400, 1399, 1435);
            s.store_sqrt_square_offset(1401, 1400, 1.0);
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1399), s.ad_value(1401)), 1419);
            s.store_sub(1400, 1430, 1444);
            s.store_mul(459, 1407, 1400);
            s.copy_ad(458, 459);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_scalar(1442, 3.0);
            s.store_sub_ad_lhs(1445, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_scaled_inputs2(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, s.ad_value(1406), 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1501] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1501]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_add_ad_rhs(1445, 1430, A::mul3_scaled_output(s.ad_value(1429), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_scaled_inputs2(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, s.ad_value(1406), 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1502] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1502]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_add_ad_rhs(1445, 1430, A::mul3_scaled_output(s.ad_value(1429), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        s.b[1503] = (s.v[1442] < 3.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1503]) {
            s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1449, s.ad_value(1430), -1.0, s.ad_value(1419), -1.0, s.ad_value(1428), 1.0);
            s.store_add_scaled_inputs3(1450, A::div_scaled_product(A::square(s.ad_value(1447)), s.ad_value(1447), 1.0, A::mul3_scaled_output(s.ad_value(1446), s.ad_value(1446), s.ad_value(1446), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1447), s.ad_value(1448), 1.0, s.ad_value(1446), s.ad_value(1446), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1449), 1.0, s.ad_value(1446), 2.0), 1.0);
            s.store_div_ad(1451, A::add_scaled_square_product(s.ad_value(1447), (-1.0), s.ad_value(1446), s.ad_value(1448), 3.0), A::mul_scaled_lhs(s.ad_value(1446), 9.0, s.ad_value(1446)));
            s.store_sqrt_ad(1402, A::add_scaled_square_product(s.ad_value(1450), 1.0, A::square(s.ad_value(1451)), s.ad_value(1451), 1.0));
            s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);
            s.store_neg_ad(1453, A::powf(A::add(s.ad_value(1450), s.ad_value(1402)), 0.3333333333333333));
            s.store_add_scaled_inputs3(1405, s.ad_value(1452), 1.0, s.ad_value(1453), 1.0, A::div_scaled_inputs(s.ad_value(1447), 1.0, s.ad_value(1446), 3.0), -1.0);
            s.store_add_scaled_product_indices(1445, 1419, (-1.0), 1405, 227, 1.0);
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_offset_add(1454, 1430, 1419, 0.1);
            s.store_offset_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0), 1e-50);
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1456, 1455, 1461);
            s.store_mul(1398, 226, 1429);
            s.store_mul(1457, 225, 1454);
            s.store_add_scaled_inputs_product_mixed_aaii(1458, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);
            s.store_offset_sub(44, 1457, 1458, (-1.0));
            s.store_scale(45, 1457, 4.0);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1458, s.ad_value(1457), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1457, 1457, 1458);
            s.store_add_scaled_inputs(1457, 1457, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1459, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);
            s.copy_ad(1460, 1442);
            s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));
            s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1442, s.ad_value(1459), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_add_ad(1399, A::offset(s.ad_value(1442), (-1.0)), A::exp_scaled_input(s.ad_value(1442), -1.0));
        }

        s.b[1504] = (s.v[1399] < (10.0 * 2.220446049250313e-16));
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1504]) {
            s.store_scalar(1399, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_sqrt(1400, 1399);
            s.store_mul(458, 1427, 1400);
            s.store_mul_sub_rhs(459, 1407, 1430, 1444);
        }

        s.b[1505] = (p.p42 == 1.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
            s.store_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0));
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1470, 1455, 1461);
            s.store_scalar(1415, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign27790_loop_guard: usize = 0;
        while {
            let assign27790_cond_e38754: f64 = (2.0 * 20.0);
            let assign27790_cond_e38756: f64 = (assign27790_cond_e38754 + 1.0);
            let assign27790_cond_e38758: f64 = if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (s.v[167] <= assign27790_cond_e38756)) { 1.0 } else { 0.0 };
            assign27790_cond_e38758 != 0.0
        } {
            assign27790_loop_guard += 1;
            assert!(assign27790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
                s.store_scalar(1466, 0.0);
                s.store_mul_add_rhs(1442, 225, 1444, 1419);
            }
            s.b[1506] = (s.v[1442] < 5.0);
            s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1506]) {
                s.store_mul3_ad_middle(1462, A::square(s.ad_value(1442)), 1442, A::offset(A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1463, A::square(s.ad_value(1442)), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1464, 1470, 1462, 1462);
                s.store_mul_product3_rhs(1465, 1463, s.ad_value(1470), s.ad_value(225), s.ad_value(1462), 2.0);
                s.store_mul_offset_ad_rhs(1466, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1467, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);
                s.store_div_scaled_inputs2(1469, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1467), s.ad_value(1466), 2.0), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 2.0);
            }
            s.b[1507] = (s.v[1442] < 80.0);
            s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && s.b[1507]) {
                s.store_exp(243, 1442);
                s.store_mul_offset_rhs(1464, 1470, 243, (-1.0));
                s.store_mul3_lhs(1465, 1470, 225, 243);
            }
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && (!s.b[1507])) {
                s.store_exp_mul(1471, 225, 1444);
                s.store_mul_sub_rhs(1464, 1455, 1471, 1461);
                s.store_mul3_lhs(1465, 1455, 225, 1471);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) {
                s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));
                s.store_scale_ad(1469, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 1.0), 0.5);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
                s.store_add_scaled_inputs_product_indices(1472, 1430, 1.0, 1444, (-1.0), 1428, 1468, (-1.0));
                s.store_sub_from_scalar_ad(1473, (-1.0), A::mul(s.ad_value(1428), s.ad_value(1469)));
            }
            s.b[1508] = (s.v[1415] == 1.0);
            s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1508]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_div_scaled_inputs(494, s.ad_value(1472), -1.0, s.ad_value(1473), 1.0);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1509] = (((s.v[494]) as f64).abs() > s.v[1474]);
            s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1509]) {
                s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_add(1444, 1444, 494);
            }
            s.b[1510] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));
            s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1510]) {
                s.store_scalar(1415, 1.0);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1512] = (s.v[1442] < 5.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1512]) {
            s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));
            s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1512])) {
            s.store_offset(1475, 1442, (-1.0));
            s.store_sqrt(1476, 1475);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
            s.store_mul(458, 1427, 1476);
            s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
            s.store_mul3_lhs(460, 1427, 1464, 1399);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1514] = (1.0 == 1.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        s.b[1515] = (1.0 == 2.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        s.v[317] = p.p189;

        s.b[1518] = (s.v[145] != 0.0);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if s.b[1518] {
            s.store_add(1517, 157, 161);
            s.store_add_scaled_inputs(314, 1517, s.v[317], 162, (1.0 - s.v[317]));
        }

        s.b[1519] = (p.p64 != 0.0);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if (s.b[1518] && s.b[1519]) {
            s.store_scalar(315, 0.0);
        }

        s.b[1520] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if (s.b[1518] && s.b[1520]) {
            s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[1521] = (p.p64 != 0.0);
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        s.b[1522] = (s.v[246] < 1e-15);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        if (((!s.b[1518]) && s.b[1521]) && s.b[1522]) {
            s.store_scalar(315, 0.0);
        }

        if (((!s.b[1518]) && s.b[1521]) && (!s.b[1522])) {
            s.store_scale(1516, 227, 1.0 / (s.v[97]));
            s.store_div_from_scalar(1517, 1.0, 244);
            s.store_mul3_lhs(315, 246, 1516, 1517);
        }

        s.v[1534] = s.v[91];

        s.v[1535] = (1.0 / s.v[1534]);

        s.v[1555] = 0.0;

        s.v[1595] = 0.0;

        s.v[1593] = 0.0;

        s.v[1597] = 0.0;

        s.b[1606] = ((p.p29 >= 1.0) && (p.p188 > 0.0));
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && s.b[1606]) {
            s.store_scalar(1537, p.p171);
            s.store_scalar(1538, p.p172);
            s.copy_ad(1539, 158);
            s.store_scalar(1536, p.p188);
        }

        s.b[1607] = ((s.v[69] == 0.0) && (p.p188 > 0.0));
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && s.b[1606]) && s.b[1607]) {
            if (p.p43 == 1.0) {
                s.store_scale(1524, 287, s.v[1534]);
            } else {
                s.store_scale(1524, 108, s.v[1534]);
            }
        }

        if (((p.p24 != 0.0) && s.b[1606]) && s.b[1607]) {
            s.store_mul_ad_product_rhs(1527, 1537, s.ad_value(1524), A::add(s.ad_value(1538), s.ad_value(1539)));
            s.store_mul(1528, 1536, 1524);
            s.copy_ad(1532, 161);
            s.store_sub_from_scalar(1529, 1.2, 1532);
            s.store_add_scaled_products_indices(267, 158, 1528, 1.0, 1529, 1527, (-1.0));
            s.store_mul_ad_product_rhs(1527, 1537, s.ad_value(1524), A::add_scaled_inputs3(s.ad_value(1538), 1.0, s.ad_value(1539), 1.0, s.ad_value(157), -1.0));
            s.store_sub(1532, 162, 157);
            s.store_sub_from_scalar(1529, 1.2, 1532);
            s.store_add_scaled_products_left_left_ad(268, A::sub(s.ad_value(158), s.ad_value(157)), 1528, 1.0, 1527, 1529, (-1.0));
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_mul_sqrt_ad_rhs(1556, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));
            s.store_scalar(1540, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1541, ((1.0 + -1.0) / 2.0));
        }

        s.b[1608] = (p.p43 == 1.0);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {
            s.store_add_scaled_products_right_right_ad(1550, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1551, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1552, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1553, 1551, 1550);
            s.store_sub(1555, 1552, 1550);
            s.store_neg(1554, 1550);
            s.store_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);
            s.store_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);
            s.store_offset_ad(1548, A::add_scaled_products(s.ad_value(1542), s.ad_value(1554), 1.0, s.ad_value(1543), s.ad_value(1553), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {
            s.store_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);
            s.store_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1540] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1555, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1541] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1555, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {
            s.store_scalar(1548, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_neg(1523, 1548);
        }

        s.b[1609] = (s.v[1523] > s.v[141]);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1609]) {
            s.store_sub(1524, 1523, 141);
            s.store_sub(1525, 140, 141);
            s.store_div(44, 1524, 1525);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1533, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1533, 1525, 1.0, 1533);
            s.store_add(1530, 141, 1533);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1609])) {
            s.copy_ad(1530, 1523);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_offset_scaled(1549, 1530, -1.0, (-1e-12));
            s.store_scale(1557, 1556, s.v[1535]);
            s.store_square(1558, 1557);
            s.store_sub_from_scalar(1559, s.v[82], 1555);
            s.store_div_from_scalar(1523, s.v[69], 230);
            s.store_mul_ad(1560, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1523)));
            s.store_neg(1561, 1549);
        }

        s.b[1610] = (s.v[1559] < s.v[1561]);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {
            s.store_div_from_scalar_mul_ad(1524, 1.0, s.ad_value(225), s.ad_value(1556));
            s.store_scale(1533, 1524, s.v[1534]);
            s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1563, 1562, 1562, 8.0, 0.0, 1562);
            s.store_sub(1564, 237, 1560);
            s.store_mul_add_rhs(1532, 225, 1559, 1549);
            s.store_sub_from_scalar_ad(1565, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1533), 9.0, A::offset(s.ad_value(1532), (-2.0))));
            s.store_square(1566, 1565);
        }

        s.b[1611] = (s.v[1563] < (s.v[1566] * 1e-8));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && s.b[1611]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1568, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1563), 0.5, s.ad_value(1565), 1.0), 1.0, 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && (!s.b[1611])) {
            s.store_sqrt_add(1567, 1563, 1566);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), 1.0, 1533, 1532, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {
            s.store_powf(1569, 1568, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1570, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), 1.0, 1569, 2.0, 1569, 1569, 1.414213562373095);
            s.store_div(1571, 1570, 1569);
            s.store_add_scaled_product_indices(1572, 1549, (-1.0), 1571, 227, 1.0);
            s.store_add(1524, 1572, 1549);
            s.store_div(1525, 1524, 1564);
            s.store_sqrt_square_offset(1526, 1525, 1.0);
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1524), s.ad_value(1526)), 1549);
            s.store_sub(1525, 1559, 1573);
            s.store_scale(459, 1525, s.v[1534]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_scalar(1571, 3.0);
            s.store_sub_ad_lhs(1574, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_scaled_inputs2(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, s.ad_value(1533), 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1612] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1612]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_add_ad_rhs(1574, 1559, A::mul3_scaled_output(s.ad_value(1558), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_scaled_inputs2(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, s.ad_value(1533), 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1613] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1613]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_add_ad_rhs(1574, 1559, A::mul3_scaled_output(s.ad_value(1558), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1614] = (s.v[1571] < 3.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1614]) {
            s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1577, 1.0, A::mul(s.ad_value(225), s.ad_value(1557)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1578, s.ad_value(1559), -1.0, s.ad_value(1549), -1.0, s.ad_value(1557), 1.0);
            s.store_add_scaled_inputs3(1579, A::div_scaled_product(A::square(s.ad_value(1576)), s.ad_value(1576), 1.0, A::mul3_scaled_output(s.ad_value(1575), s.ad_value(1575), s.ad_value(1575), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1576), s.ad_value(1577), 1.0, s.ad_value(1575), s.ad_value(1575), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1578), 1.0, s.ad_value(1575), 2.0), 1.0);
            s.store_div_ad(1580, A::add_scaled_square_product(s.ad_value(1576), (-1.0), s.ad_value(1575), s.ad_value(1577), 3.0), A::mul_scaled_lhs(s.ad_value(1575), 9.0, s.ad_value(1575)));
            s.store_sqrt_ad(1528, A::add_scaled_square_product(s.ad_value(1579), 1.0, A::square(s.ad_value(1580)), s.ad_value(1580), 1.0));
            s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);
            s.store_neg_ad(1582, A::powf(A::add(s.ad_value(1579), s.ad_value(1528)), 0.3333333333333333));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1614]) {
            s.store_add_scaled_inputs3(1532, s.ad_value(1581), 1.0, s.ad_value(1582), 1.0, A::div_scaled_inputs(s.ad_value(1576), 1.0, s.ad_value(1575), 3.0), -1.0);
            s.store_add_scaled_product_indices(1574, 1549, (-1.0), 1532, 227, 1.0);
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1615] = (p.p41 > 0.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_offset_add(1583, 1559, 1549, 0.1);
            s.store_offset_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0), 1e-50);
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1585, 1584, 1590);
            s.store_mul(1523, 226, 1558);
            s.store_mul(1586, 225, 1583);
            s.store_add_scaled_inputs_product_mixed_aaii(1587, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);
            s.store_offset_sub(44, 1586, 1587, (-1.0));
            s.store_scale(45, 1586, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1587, s.ad_value(1586), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1586, 1586, 1587);
            s.store_add_scaled_inputs(1586, 1586, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1588, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);
            s.copy_ad(1589, 1571);
            s.store_offset_sub(44, 1588, 1589, (-(0.0008 * 75.0)));
            s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1571, s.ad_value(1588), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_add_ad(1524, A::offset(s.ad_value(1571), (-1.0)), A::exp_scaled_input(s.ad_value(1571), -1.0));
        }

        s.b[1616] = (s.v[1524] < (10.0 * 2.220446049250313e-16));
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1616]) {
            s.store_scalar(1524, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_sqrt(1525, 1524);
            s.store_mul(458, 1556, 1525);
            s.store_scaled_sub(459, 1559, 1573, s.v[1534]);
        }

        s.b[1617] = (p.p41 == 1.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
            s.store_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0));
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1599, 1584, 1590);
            s.store_scalar(1546, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(1597, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign29790_loop_guard: usize = 0;
        while {
            let assign29790_cond_e42287: f64 = (2.0 * 20.0);
            let assign29790_cond_e42289: f64 = (assign29790_cond_e42287 + 1.0);
            let assign29790_cond_e42291: f64 = if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (s.v[167] <= assign29790_cond_e42289)) { 1.0 } else { 0.0 };
            assign29790_cond_e42291 != 0.0
        } {
            assign29790_loop_guard += 1;
            assert!(assign29790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
                s.store_scalar(1595, 0.0);
                s.store_mul_add_rhs(1571, 225, 1573, 1549);
            }
            s.b[1618] = (s.v[1571] < 5.0);
            s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1618]) {
                s.store_mul3_ad_middle(1591, A::square(s.ad_value(1571)), 1571, A::offset(A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1592, A::square(s.ad_value(1571)), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1593, 1599, 1591, 1591);
                s.store_mul_product3_rhs(1594, 1592, s.ad_value(1599), s.ad_value(225), s.ad_value(1591), 2.0);
                s.store_mul_offset_ad_rhs(1595, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1596, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1597, A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50);
                s.store_div_scaled_inputs2(1598, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1596), s.ad_value(1595), 2.0), 1.0, s.ad_value(1594), 1.0, s.ad_value(1597), 2.0);
            }
            s.b[1619] = (s.v[1571] < 80.0);
            s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) && s.b[1619]) {
                s.store_exp(243, 1571);
                s.store_mul_offset_rhs(1593, 1599, 243, (-1.0));
                s.store_mul3_lhs(1594, 1599, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) && (!s.b[1619])) {
                s.store_exp_mul(1600, 225, 1573);
                s.store_mul_sub_rhs(1593, 1584, 1600, 1590);
                s.store_mul3_lhs(1594, 1584, 225, 1600);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) {
                s.store_sqrt_add_ad(1597, A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593));
                s.store_scale_ad(1598, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1594), 1.0, s.ad_value(1597), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
                s.store_add_scaled_inputs_product_indices(1601, 1559, 1.0, 1573, (-1.0), 1557, 1597, (-1.0));
                s.store_sub_from_scalar_ad(1602, (-1.0), A::mul(s.ad_value(1557), s.ad_value(1598)));
            }
            s.b[1620] = (s.v[1546] == 1.0);
            s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1620]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {
                s.store_div_scaled_inputs(494, s.ad_value(1601), -1.0, s.ad_value(1602), 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {
                s.store_scaled_offset_ad(1603, {
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1621] = (((s.v[494]) as f64).abs() > s.v[1603]);
            s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) && s.b[1621]) {
                s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {
                s.store_add(1573, 1573, 494);
            }
            s.b[1622] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8));
            s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) && s.b[1622]) {
                s.store_scalar(1546, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1624] = (s.v[1571] < 5.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1624]) {
            s.store_offset_square(1604, 1595, (10.0 * 2.220446049250313e-16));
            s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1624])) {
            s.store_offset(1604, 1571, (-1.0));
            s.store_sqrt(1605, 1604);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
            s.store_mul(458, 1556, 1605);
            s.store_div_from_scalar_add_ad(1524, 1.0, s.ad_value(1597), s.ad_value(1605));
            s.store_mul3_lhs(460, 1556, 1593, 1524);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            if (p.p43 == 1.0) {
                s.store_mul(1527, 287, 1536);
            } else {
                s.store_mul(1527, 108, 1536);
            }
        }

        s.b[1626] = (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0)));
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1626]) {
            s.store_mul(455, 1527, 459);
            s.store_mul(457, 1527, 458);
        }

        s.b[1627] = (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0)));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1627]) {
            s.store_mul(454, 1527, 459);
            s.store_mul(456, 1527, 458);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_scalar(1540, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1541, ((1.0 + 1.0) / 2.0));
        }

        s.b[1628] = (p.p43 == 1.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1628]) {
            s.store_add_scaled_products_right_right_ad(1550, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);
            s.store_add_scaled_products_indices(1551, 461, 157, 1.0, 462, 157, -1.0);
            s.store_add_scaled_products_right_right_ad(1552, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
            s.store_sub(1553, 1551, 1550);
            s.store_sub(1555, 1552, 1550);
            s.store_neg(1554, 1550);
            s.store_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);
            s.store_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);
            s.store_offset_ad(1548, A::add_scaled_products(s.ad_value(1542), s.ad_value(1554), 1.0, s.ad_value(1543), s.ad_value(1553), 1.0), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) {
            s.store_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);
            s.store_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) && (s.v[1540] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1555, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) && (s.v[1541] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(1555, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) {
            s.store_scalar(1548, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_neg(1523, 1548);
        }

        s.b[1629] = (s.v[1523] > s.v[141]);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1629]) {
            s.store_sub(1524, 1523, 141);
            s.store_sub(1525, 140, 141);
            s.store_div(44, 1524, 1525);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_ad(1533, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));
            s.store_mul_sub_from_scalar_rhs(1533, 1525, 1.0, 1533);
            s.store_add(1530, 141, 1533);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1629])) {
            s.copy_ad(1530, 1523);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_offset_scaled(1549, 1530, -1.0, (-1e-12));
            s.store_scale(1557, 1556, s.v[1535]);
            s.store_square(1558, 1557);
            s.store_sub_from_scalar(1559, s.v[82], 1555);
            s.store_div_from_scalar(1523, s.v[69], 230);
            s.store_mul_ad(1560, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1523)));
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_neg(1561, 1549);
        }

        s.b[1630] = (s.v[1559] < s.v[1561]);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) {
            s.store_div_from_scalar_mul_ad(1524, 1.0, s.ad_value(225), s.ad_value(1556));
            s.store_scale(1533, 1524, s.v[1534]);
            s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(1563, 1562, 1562, 8.0, 0.0, 1562);
            s.store_sub(1564, 237, 1560);
            s.store_mul_add_rhs(1532, 225, 1559, 1549);
            s.store_sub_from_scalar_ad(1565, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(1533), 9.0, A::offset(s.ad_value(1532), (-2.0))));
            s.store_square(1566, 1565);
        }

        s.b[1631] = (s.v[1563] < (s.v[1566] * 1e-8));
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) && s.b[1631]) {
            s.store_add_scaled_inputs_product_mixed_aaia(1568, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1563), 0.5, s.ad_value(1565), 1.0), 1.0, 1533, A::offset(s.ad_value(1532), (-2.0)), 9.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) && (!s.b[1631])) {
            s.store_sqrt_add(1567, 1563, 1566);
            s.store_add_scaled_offset_product_rhs_mixed_aii(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), 1.0, 1533, 1532, (-2.0), 9.0);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) {
            s.store_powf(1569, 1568, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(1570, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), 1.0, 1569, 2.0, 1569, 1569, 1.414213562373095);
            s.store_div(1571, 1570, 1569);
            s.store_add_scaled_product_indices(1572, 1549, (-1.0), 1571, 227, 1.0);
            s.store_add(1524, 1572, 1549);
            s.store_div(1525, 1524, 1564);
            s.store_sqrt_square_offset(1526, 1525, 1.0);
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1524), s.ad_value(1526)), 1549);
            s.store_sub(1525, 1559, 1573);
            s.store_scale(459, 1525, s.v[1534]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_scalar(1571, 3.0);
            s.store_sub_ad_lhs(1574, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_scaled_inputs2(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, s.ad_value(1533), 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1632] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1632]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_add_ad_rhs(1574, 1559, A::mul3_scaled_output(s.ad_value(1558), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_scaled_inputs2(1532, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), 4.0, s.ad_value(1533), 4.0, A::mul(s.ad_value(1558), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[1633] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1633]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_add_ad_rhs(1574, 1559, A::mul3_scaled_output(s.ad_value(1558), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532))), 1.0 / (2.0)));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1634] = (s.v[1571] < 3.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1634]) {
            s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1577, 1.0, A::mul(s.ad_value(225), s.ad_value(1557)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(1578, s.ad_value(1559), -1.0, s.ad_value(1549), -1.0, s.ad_value(1557), 1.0);
            s.store_add_scaled_inputs3(1579, A::div_scaled_product(A::square(s.ad_value(1576)), s.ad_value(1576), 1.0, A::mul3_scaled_output(s.ad_value(1575), s.ad_value(1575), s.ad_value(1575), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1576), s.ad_value(1577), 1.0, s.ad_value(1575), s.ad_value(1575), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(1578), 1.0, s.ad_value(1575), 2.0), 1.0);
            s.store_div_ad(1580, A::add_scaled_square_product(s.ad_value(1576), (-1.0), s.ad_value(1575), s.ad_value(1577), 3.0), A::mul_scaled_lhs(s.ad_value(1575), 9.0, s.ad_value(1575)));
            s.store_sqrt_ad(1528, A::add_scaled_square_product(s.ad_value(1579), 1.0, A::square(s.ad_value(1580)), s.ad_value(1580), 1.0));
            s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);
            s.store_neg_ad(1582, A::powf(A::add(s.ad_value(1579), s.ad_value(1528)), 0.3333333333333333));
            s.store_add_scaled_inputs3(1532, s.ad_value(1581), 1.0, s.ad_value(1582), 1.0, A::div_scaled_inputs(s.ad_value(1576), 1.0, s.ad_value(1575), 3.0), -1.0);
            s.store_add_scaled_product_indices(1574, 1549, (-1.0), 1532, 227, 1.0);
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1635] = (p.p41 > 0.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_offset_add(1583, 1559, 1549, 0.1);
            s.store_offset_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0), 1e-50);
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1585, 1584, 1590);
            s.store_mul(1523, 226, 1558);
            s.store_mul(1586, 225, 1583);
            s.store_add_scaled_inputs_product_mixed_aaii(1587, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);
            s.store_offset_sub(44, 1586, 1587, (-1.0));
            s.store_scale(45, 1586, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1587, s.ad_value(1586), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_sub(1586, 1586, 1587);
            s.store_add_scaled_inputs(1586, 1586, 1.0, 225, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(1588, A::ln(A::add_scaled_square_product(s.ad_value(1586), 1.0, s.ad_value(1585), s.ad_value(1523), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1584), s.ad_value(1523))), (-1.0), 225, 1549, 1.0);
            s.copy_ad(1589, 1571);
            s.store_offset_sub(44, 1588, 1589, (-(0.0008 * 75.0)));
            s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_offset_scaled_ad(1525, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(1571, s.ad_value(1588), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_add_ad(1524, A::offset(s.ad_value(1571), (-1.0)), A::exp_scaled_input(s.ad_value(1571), -1.0));
        }

        s.b[1636] = (s.v[1524] < (10.0 * 2.220446049250313e-16));
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1636]) {
            s.store_scalar(1524, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_sqrt(1525, 1524);
            s.store_mul(458, 1556, 1525);
            s.store_scaled_sub(459, 1559, 1573, s.v[1534]);
        }

        s.b[1637] = (p.p41 == 1.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
            s.store_exp_ad(1590, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1549), -1.0));
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1599, 1584, 1590);
            s.store_scalar(1546, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(1597, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign31390_loop_guard: usize = 0;
        while {
            let assign31390_cond_e45523: f64 = (2.0 * 20.0);
            let assign31390_cond_e45525: f64 = (assign31390_cond_e45523 + 1.0);
            let assign31390_cond_e45527: f64 = if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (s.v[167] <= assign31390_cond_e45525)) { 1.0 } else { 0.0 };
            assign31390_cond_e45527 != 0.0
        } {
            assign31390_loop_guard += 1;
            assert!(assign31390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
                s.store_scalar(1595, 0.0);
                s.store_mul_add_rhs(1571, 225, 1573, 1549);
            }
            s.b[1638] = (s.v[1571] < 5.0);
            s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1638]) {
                s.store_mul3_ad_middle(1591, A::square(s.ad_value(1571)), 1571, A::offset(A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(1592, A::square(s.ad_value(1571)), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(1593, 1599, 1591, 1591);
                s.store_mul_product3_rhs(1594, 1592, s.ad_value(1599), s.ad_value(225), s.ad_value(1591), 2.0);
                s.store_mul_offset_ad_rhs(1595, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(1596, 1571, A::mul_offset_rhs(s.ad_value(1571), A::mul(s.ad_value(1571), A::scale_offset(s.ad_value(1571), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(1597, A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50);
                s.store_div_scaled_inputs2(1598, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1596), s.ad_value(1595), 2.0), 1.0, s.ad_value(1594), 1.0, s.ad_value(1597), 2.0);
            }
            s.b[1639] = (s.v[1571] < 80.0);
            s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) && s.b[1639]) {
                s.store_exp(243, 1571);
                s.store_mul_offset_rhs(1593, 1599, 243, (-1.0));
                s.store_mul3_lhs(1594, 1599, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) && (!s.b[1639])) {
                s.store_exp_mul(1600, 225, 1573);
                s.store_mul_sub_rhs(1593, 1584, 1600, 1590);
                s.store_mul3_lhs(1594, 1584, 225, 1600);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) {
                s.store_sqrt_add_ad(1597, A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593));
                s.store_scale_ad(1598, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1594), 1.0, s.ad_value(1597), 1.0), 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
                s.store_add_scaled_inputs_product_indices(1601, 1559, 1.0, 1573, (-1.0), 1557, 1597, (-1.0));
                s.store_sub_from_scalar_ad(1602, (-1.0), A::mul(s.ad_value(1557), s.ad_value(1598)));
            }
            s.b[1640] = (s.v[1546] == 1.0);
            s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1640]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {
                s.store_div_scaled_inputs(494, s.ad_value(1601), -1.0, s.ad_value(1602), 1.0);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {
                s.store_scaled_offset_ad(1603, {
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1641] = (((s.v[494]) as f64).abs() > s.v[1603]);
            s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) && s.b[1641]) {
                s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {
                s.store_add(1573, 1573, 494);
            }
            s.b[1642] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8));
            s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) && s.b[1642]) {
                s.store_scalar(1546, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1644] = (s.v[1571] < 5.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1644]) {
            s.store_offset_square(1604, 1595, (10.0 * 2.220446049250313e-16));
            s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1644])) {
            s.store_offset(1604, 1571, (-1.0));
            s.store_sqrt(1605, 1604);
        }

    }
}
