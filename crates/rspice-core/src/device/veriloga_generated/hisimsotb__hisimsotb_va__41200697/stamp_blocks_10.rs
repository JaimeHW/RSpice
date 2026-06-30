#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(433, 434, 1.0, 638, (-0.5), 639, (-0.5));
            s.store_sub(434, 434, 433);
            s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_sub_div_lhs_indices(320, 432, 120, 416);
            s.copy_ad(431, 77);
            s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(77, 432, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_sub_div_lhs_indices(410, 77, 120, 416);
            s.store_add_offset_lhs_ad_rhs(279, 77, (-1.0), A::exp_scaled_input(s.ad_value(77), -1.0));
        }

        s.b[1003] = (s.v[279] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(1003, if s.b[1003] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[997])) && s.b[1003]) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_mul_sqrt_rhs(407, 437, 279);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.b[1004] = (p.p30 == 1.0);
        s.store_scalar(1004, if s.b[1004] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
            s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(204, 429, 203);
            s.store_scalar(379, 0.0);
            s.store_scalar(62, 1.0);
        }

        let mut assign20620_loop_guard: usize = 0;
        while {
            let assign20620_cond_e25604: f64 = (40.0 + 1.0);
            let assign20620_cond_e25606: f64 = if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (s.v[62] <= assign20620_cond_e25604)) { 1.0 } else { 0.0 };
            assign20620_cond_e25606 != 0.0
        } {
            assign20620_loop_guard += 1;
            assert!(assign20620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_mul_add_rhs(77, 120, 410, 416);
            }
            s.b[1005] = (s.v[77] < 5.0);
            s.store_scalar(1005, if s.b[1005] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1005]) {
                s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(207, 204, 205, 205);
                s.store_mul_product3_indices(208, 206, 204, 120, 205, 2.0);
                s.store_mul_offset_ad_rhs(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(148, 77, A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, 208, 1.0, 209, 2.0);
            }
            s.b[1006] = (s.v[77] < 80.0);
            s.store_scalar(1006, if s.b[1006] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && s.b[1006]) {
                s.store_exp(147, 77);
                s.store_mul_offset_rhs(207, 204, 147, (-1.0));
                s.store_mul3_lhs(208, 204, 120, 147);
            }
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && (!s.b[1006])) {
                s.store_exp_mul(202, 120, 410);
                s.store_mul_sub_rhs(207, 429, 202, 203);
                s.store_mul3_lhs(208, 429, 120, 202);
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) {
                s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));
                s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);
            }
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));
                s.store_sub_from_scalar_scaled_mul(212, (-1.0), 144, 210, 1.0);
            }
            s.b[1007] = (s.v[379] == 1.0);
            s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1007]) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_div_scaled_inputs_indices(213, 211, -1.0, 212, 1.0);
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1008] = (((s.v[213]) as f64).abs() > s.v[214]);
            s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1008]) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_add(410, 410, 213);
            }
            s.b[1009] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));
            s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1009]) {
                s.store_scalar(379, 1.0);
            }
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[1011] = (s.v[77] < 5.0);
        s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });

        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1011]) {
            s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1011])) {
            s.store_offset(64, 77, (-1.0));
            s.store_sqrt(65, 64);
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
            s.store_mul(407, 437, 65);
            s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));
            s.store_mul3_lhs(409, 437, 207, 279);
            s.store_add(408, 407, 409);
        }

        if s.b[978] {
            s.store_sub(409, 408, 407);
            s.store_scale(282, 195, s.v[513]);
        }

        if (s.b[978] && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
            s.store_mul(406, 282, 407);
        }

        if (s.b[978] && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
            s.store_mul(405, 282, 407);
        }

        if s.b[978] {
            s.store_add_scaled_inputs(194, 413, s.v[519], 412, s.v[518]);
        }

        if (s.b[978] && (s.v[194] != 0.0)) {
            s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);
            s.store_scale(198, 198, (-s.v[513]));
            s.store_offset_ad(197, A::mul_scaled_lhs(s.ad_value(198), -1.0, A::sub(s.ad_value(52), s.ad_value(51))), s.v[197]);
        }

        if s.b[978] {
            s.store_add_scaled_inputs(194, 412, s.v[519], 413, s.v[518]);
        }

        if (s.b[978] && (s.v[194] != 0.0)) {
            s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);
            s.store_scale(199, 199, (-s.v[513]));
            s.store_offset_scaled_mul(196, 199, 52, -1.0, s.v[196]);
        }

        s.b[1013] = (((s.v[575] == 1.0) && (!s.b[518])) || ((s.v[575] != 1.0) && (!s.b[519])));
        s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });

        s.b[1014] = (p.p175 > 0.0);
        s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });

        if (((!s.b[978]) && s.b[1013]) && s.b[1014]) {
            s.store_scalar(198, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if (((!s.b[978]) && s.b[1013]) && (!s.b[1014])) {
            s.store_scalar(198, 0.0);
        }

        if ((!s.b[978]) && (!s.b[1013])) {
            s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);
            s.store_scale(198, 198, (-s.v[513]));
        }

        if (!s.b[978]) {
            s.store_mul_sub_scaled_inputs_rhs(197, 198, s.ad_value(52), -1.0, s.ad_value(51), -1.0);
        }

        s.b[1015] = (((s.v[575] == 1.0) && (!s.b[519])) || ((s.v[575] != 1.0) && (!s.b[518])));
        s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });

        if ((!s.b[978]) && s.b[1015]) {
            s.store_scalar(199, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if ((!s.b[978]) && (!s.b[1015])) {
            s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);
            s.store_scale(199, 199, (-s.v[513]));
        }

        if (!s.b[978]) {
            s.store_mul_neg_lhs(196, 199, 52);
        }

        s.b[1016] = (s.v[34] == 0.0);
        s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });

        if ((s.v[38] != 0.0) && s.b[1016]) {
            s.store_scaled_mul(279, 386, 386, (p.p223 * p.p224));
            s.store_offset_ad(280, A::add_scaled_products(s.ad_value(158), s.ad_value(86), p.p223, s.ad_value(386), s.ad_value(386), p.p224), 1e-50);
            s.store_div(221, 279, 280);
        }

        if ((s.v[38] != 0.0) && (!s.b[1016])) {
            s.store_scalar(221, (p.p223 + 1e-50));
        }

        if (s.v[38] != 0.0) {
            s.store_scale(222, 270, (p.p225 * 0.0001));
        }

        s.b[1017] = ((p.p21 != 0.0) && (s.v[34] == 0.0));
        s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });

        if s.b[1017] {
            s.store_scalar(223, s.v[617]);
            s.store_scalar(225, s.v[619]);
            s.store_scale(279, 149, 6.241449993689894e18);
            s.store_mul_scaled_ad_lhs(280, A::add_scaled_inputs3(s.ad_value(270), 1.0, A::div(s.ad_value(149), A::sub(s.ad_value(56), s.ad_value(50))), 1.0, s.ad_value(225), 1.0), 122, 6.241449993689894e18);
            s.store_sub_ad_lhs(281, A::div_scaled_inputs(s.ad_value(91), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[513])), s.ad_value(386), 1.0), 279);
        }

        s.b[1018] = ((((s.v[281] - s.v[279])) as f64).abs() > (10.0 * 2.220446049250313e-16));
        s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });

        if (s.b[1017] && s.b[1018]) {
            s.store_add_scaled_value_products(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::sub(s.ad_value(281), s.ad_value(279)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(281), 1.0, s.ad_value(280), 1.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), s.ad_value(158), 1.0);
        }

        if (s.b[1017] && (!s.b[1018])) {
            s.store_add_scaled_inputs_product_mixed_aaai(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);
        }

        s.b[1019] = ((p.p23 != 0.0) && (s.v[34] == 0.0));
        s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });

        if s.b[1019] {
            s.store_div_scaled_inputs2_indices(227, 260, 1.0, 56, (-1.0), 386, 1.0);
            s.store_scaled_mul(289, 159, 227, 1.0 / ((10000000.0 * 0.01)));
        }

        s.b[1020] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1020]) {
            s.store_scalar(285, 1.0);
        }

        s.b[1021] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {
            s.copy_ad(285, 289);
        }

        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {
            s.store_powf(285, 289, (p.p114 - 1.0));
        }

        if s.b[1019] {
            s.store_offset_mul(287, 289, 285, 1.0);
            s.store_powf(288, 287, (((-1.0) / p.p114) - 1.0));
            s.store_mul3_lhs(230, 159, 287, 288);
            s.store_scaled_add(228, 158, 230, 0.5);
            s.store_square(278, 85);
            s.store_div_scaled_product_by_product(229, A::mul3_scaled_output(s.ad_value(270), s.ad_value(86), s.ad_value(158), s.v[466]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 3.0, 1.0), 1.0, s.ad_value(278), 6.0), s.ad_value(230), s.ad_value(230)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 4.0, 3.0), 1.0, s.ad_value(278), 3.0), s.ad_value(230), s.ad_value(158)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(85), 3.0, 6.0), s.ad_value(278)), s.ad_value(158), s.ad_value(158)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(386), A::offset(s.ad_value(85), 1.0), s.ad_value(228), 15.0), s.ad_value(228), 1.0);
        }

        if (!s.b[1019]) {
            s.store_scalar(229, 0.0);
        }

        s.b[1022] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

        if s.b[1022] {
            s.store_sqrt(235, 233);
            s.store_add(280, 86, 235);
            s.store_square(281, 231);
            s.store_square(282, 233);
            s.store_scaled_mul(283, 231, 233, 42.0);
            s.store_add_scaled_inputs3_indices(283, 283, 1.0, 281, 4.0, 282, 4.0);
            s.store_add_product3_rhs_mixed_iia(283, 283, 235, 86, A::add(s.ad_value(231), s.ad_value(233)), 20.0);
            s.store_square(288, 280);
            s.store_div_ad_rhs(236, 283, A::mul(A::square(s.ad_value(288)), s.ad_value(280)));
            s.store_mul_ad_product_lhs_mixed_ai(237, A::div_from_scalar(s.v[466], s.ad_value(386)), 158, 270);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1022] {
            s.store_add_ad_lhs(285, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 4.0), 233);
        }

        s.store_add(94, 94, 193);

        if s.b[517] {
            s.store_scalar(200, ((-p.p172) * s.v[277]));
            s.store_mul_sub_rhs(201, 200, 42, 40);
        }

        if (!s.b[517]) {
            s.store_scalar(200, 0.0);
            s.store_scalar(201, 0.0);
        }

        s.store_scalar(215, 0.0);

        s.store_scaled_sub(216, 42, 41, s.v[215]);

        s.store_scale(217, 42, s.v[215]);

        s.store_add(197, 197, 216);

        s.store_add(196, 196, 217);

        s.store_scale(0, 94, s.v[394]);

        s.store_scale(279, 123, (-s.v[513]));

        s.store_scaled_add(280, 523, 576, (-0.5));

        s.store_scaled_add(281, 531, 585, (-0.5));

        s.store_scaled_mul(444, 279, 40, (0.1 * s.v[294]));

        s.store_mul_sub_scaled_inputs_rhs(443, 279, s.ad_value(40), (0.1 * s.v[294]), s.ad_value(41), (0.1 * s.v[294]));

        s.store_mul(441, 279, 280);

        s.store_mul(442, 279, 281);

        if (p.p303 != 0.0) {
            s.store_scalar(336, 0.0);
            s.copy_ad(92, 91);
        }

        if (p.p303 == 0.0) {
            s.store_add_scaled_inputs3_indices(92, 91, 1.0, 441, 1.0, 442, 1.0);
        }

        s.store_scale(93, 92, s.v[385]);

        if (s.v[38] != 0.0) {
            s.store_scalar(15, 0.0);
            s.store_scalar(14, 0.0);
            s.store_scalar(492, 0.0);
            s.store_scale(556, 336, s.v[394]);
            s.store_scale(555, 92, s.v[394]);
        }

        if (s.v[38] == 0.0) {
            s.store_sub_scaled_inputs(14, 336, (-s.v[394]), 92, s.v[394]);
            s.store_scaled_add(15, 93, 443, s.v[394]);
            s.store_add_scaled_inputs3_indices(16, 92, s.v[394], 93, ((-1.0) * s.v[394]), 444, s.v[394]);
        }

        s.b[1023] = (p.p45 == 0.0);
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

        if s.b[1023] {
            s.store_scalar(219, 0.0);
        }

        if (!s.b[1023]) {
            s.store_add_scaled_product_indices(218, 56, 1.0, 261, 123, 1.0);
        }

        s.b[1024] = (s.v[218] > s.v[260]);
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if ((!s.b[1023]) && s.b[1024]) {
            s.copy_ad(218, 260);
        }

        if (!s.b[1023]) {
            s.store_add_scaled_inputs3_indices(279, 51, s.v[264], 56, s.v[264], 218, (1.0 - s.v[264]));
            s.store_sqrt_div_from_scalar_ad(288, (2.0 * 1.034943e-10), s.ad_value(126));
            s.store_scale(281, 288, 1.3);
            s.store_scale(280, 281, (1.034943e-10 * s.v[513]));
            s.store_mul_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(56), 1.0 / (p.p45), s.ad_value(51), 1.0 / (p.p45), s.ad_value(279), (-1.0 / (p.p45)), s.ad_value(261), -1.0), 280);
        }

        s.b[1025] = (p.p46 != 0.0);
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if s.b[1025] {
            s.store_add_scaled_inputs(219, 219, 1.0, 50, s.v[490]);
        }

        s.b[1026] = (p.p14 == 1.0);
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if s.b[1026] {
            s.store_add_ad_rhs(14, 14, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(197), 1.0, s.ad_value(196), 1.0, s.ad_value(201), -1.0, s.ad_value(219), -1.0), s.ad_value(398)), s.v[394], s.ad_value(397), s.v[394]));
            s.store_add_scaled_inputs4_indices(15, 15, 1.0, 219, s.v[394], 197, ((-1.0) * s.v[394]), 405, s.v[394]);
            s.store_add_scaled_inputs3_indices(16, 16, 1.0, 406, s.v[394], 196, (-s.v[394]));
        }

        s.store_scale(494, 185, s.v[394]);

        s.b[1027] = (s.v[575] == 1.0);
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if (!s.b[1027]) {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        s.b[1028] = (s.v[575] == 1.0);
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if s.b[1028] {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        s.store_scale(573, 374, (4.0 * 1.3806226e-23));

        s.store_scale(564, 229, s.v[394]);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        if (s.v[575] > 0.0) {
            s.copy_ad(493, 19);
        } else {
            s.copy_ad(493, 18);
        }

        s.b[1029] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if s.b[1029] {
            s.store_scaled_mul(278, 270, 123, (1e-6 * s.v[513]));
            s.store_scale(288, 493, 1.0 / (s.v[394]));
            s.store_div_scaled_product3_indices(241, 122, 288, 288, (0.1185185185185185 * 1.6021918e-19), 237, 1.0);
        }

        s.b[1030] = ((s.v[234] > (10.0 * 2.220446049250313e-16)) && (s.v[51] > (10.0 * 2.220446049250313e-16)));
        s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });

        if (s.b[1029] && s.b[1030]) {
            s.store_div(242, 159, 158);
            s.store_div_scaled_inputs2_mixed_aii(243, A::div(s.ad_value(159), s.ad_value(230)), 1.0, 242, (-1.0), 51, 1.0);
            s.store_add_ad_rhs(244, 242, A::div_scaled_product(s.ad_value(243), A::add(A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 1.0), s.ad_value(233)), 0.6666666666666667, A::add(s.ad_value(86), s.ad_value(235)), 1.0));
        }

        if (s.b[1029] && (!s.b[1030])) {
            s.store_div(244, 159, 230);
        }

        if s.b[1029] {
            s.store_mul3_affine_lhs(495, 241, 236, s.v[394], 0.0, 244);
        }

        if s.b[1029] {
            if (s.v[495] < 0.0) {
                s.store_scalar(495, 0.0);
            } else {
            }
        }

        if s.b[1029] {
            if ((-s.v[288]) > s.v[278]) {
            } else {
                s.store_scalar(495, 0.0);
            }
        }

        if (!s.b[1029]) {
            s.store_scalar(495, 0.0);
        }

        s.store_mul(608, 573, 564);

        if ((s.v[608] > 0.0) && (s.v[495] > 0.0)) {
            s.store_sqrt_div(610, 495, 608);
        } else {
            s.store_scalar(610, 0.0);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(611, 610, (1.0 - s.v[385]));
        } else {
            s.store_scale(611, 610, s.v[385]);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(612, 610, s.v[385]);
        } else {
            s.store_scale(612, 610, (1.0 - s.v[385]));
        }

        s.b[1031] = (p.p312 == 1.0);
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if s.b[1031] {
            s.store_scalar(1035, p.p317);
            s.store_scalar(1036, p.p319);
            s.store_scalar(1037, p.p324);
            s.store_scalar(1041, p.p311);
            s.store_scaled_voltage(1039, ctx, nodes, Some(12), Some(2), p.p33);
            s.store_scale(1035, 1035, 0.0001);
            s.store_scale(1036, 1036, 0.01);
            s.store_scale(1040, 374, 1.0 / (s.v[445]));
            s.store_powf(279, 1040, p.p320);
            s.store_div(1043, 1035, 279);
            s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1040), 0.4, 1.8), 1.0, s.ad_value(1040), s.ad_value(1040), 0.1), A::scale_offset(s.ad_value(1040), (-p.p321), p.p321));
            s.store_div(1044, 1036, 278);
            s.store_add_ad_rhs(1037, 1037, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));
            s.store_scalar(1032, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
            s.store_scalar(1034, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
            s.store_scalar(1033, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
            s.store_mul(1043, 1043, 1032);
            s.store_offset_product3(1044, s.ad_value(1044), s.ad_value(1033), s.ad_value(1034), 1.0, 1e-50);
            s.store_div(1045, 1039, 1041);
            s.store_mul(1046, 1043, 1045);
        }

        s.b[1051] = (s.v[1039] >= 0.0);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[1031] && s.b[1051]) {
            s.store_div(279, 1046, 1044);
        }

        if (s.b[1031] && (!s.b[1051])) {
            s.store_div_scaled_inputs_indices(279, 1046, -1.0, 1044, 1.0);
        }

        s.b[1052] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if (s.b[1031] && s.b[1052]) {
            s.store_scalar(281, 1.0);
        }

        s.b[1053] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if ((s.b[1031] && (!s.b[1052])) && s.b[1053]) {
            s.copy_ad(281, 279);
        }

        if ((s.b[1031] && (!s.b[1052])) && (!s.b[1053])) {
            s.store_pow_offset_rhs(281, 279, 1037, (-1.0));
        }

        if s.b[1031] {
            s.store_mul(280, 279, 281);
            s.store_offset(282, 280, 1.0);
        }

        s.b[1054] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        if (s.b[1031] && s.b[1054]) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[1055] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if ((s.b[1031] && (!s.b[1054])) && s.b[1055]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((s.b[1031] && (!s.b[1054])) && (!s.b[1055])) {
            s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1037)), (-1.0)));
            s.store_mul(283, 282, 284);
        }

        if s.b[1031] {
            s.store_div_from_scalar(279, 1.6021918e-19, 1041);
        }

        s.b[1058] = (p.p313 == 1.0);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if s.b[1058] {
            s.store_scalar(1062, p.p316);
            s.store_scalar(1063, p.p318);
            s.store_scalar(1064, p.p323);
            s.store_scalar(1068, p.p310);
            s.store_scaled_voltage(1066, ctx, nodes, Some(0), Some(11), p.p33);
            s.store_scale(1062, 1062, 0.0001);
            s.store_scale(1063, 1063, 0.01);
            s.store_scale(1067, 374, 1.0 / (s.v[445]));
            s.store_powf(279, 1067, p.p320);
            s.store_div(1070, 1062, 279);
            s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1067), 0.4, 1.8), 1.0, s.ad_value(1067), s.ad_value(1067), 0.1), A::scale_offset(s.ad_value(1067), (-p.p321), p.p321));
            s.store_div(1071, 1063, 278);
            s.store_add_ad_rhs(1064, 1064, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));
            s.store_scalar(1059, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
            s.store_scalar(1061, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
            s.store_scalar(1060, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
            s.store_mul(1070, 1070, 1059);
            s.store_offset_product3(1071, s.ad_value(1071), s.ad_value(1060), s.ad_value(1061), 1.0, 1e-50);
            s.store_div(1072, 1066, 1068);
            s.store_mul(1073, 1070, 1072);
        }

        s.b[1078] = (s.v[1066] >= 0.0);
        s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });

        if (s.b[1058] && s.b[1078]) {
            s.store_div(279, 1073, 1071);
        }

        if (s.b[1058] && (!s.b[1078])) {
            s.store_div_scaled_inputs_indices(279, 1073, -1.0, 1071, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1079] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });

        if (s.b[1058] && s.b[1079]) {
            s.store_scalar(281, 1.0);
        }

        s.b[1080] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });

        if ((s.b[1058] && (!s.b[1079])) && s.b[1080]) {
            s.copy_ad(281, 279);
        }

        if ((s.b[1058] && (!s.b[1079])) && (!s.b[1080])) {
            s.store_pow_offset_rhs(281, 279, 1064, (-1.0));
        }

        if s.b[1058] {
            s.store_mul(280, 279, 281);
            s.store_offset(282, 280, 1.0);
        }

        s.b[1081] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        if (s.b[1058] && s.b[1081]) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[1082] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });

        if ((s.b[1058] && (!s.b[1081])) && s.b[1082]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((s.b[1058] && (!s.b[1081])) && (!s.b[1082])) {
            s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1064)), (-1.0)));
            s.store_mul(283, 282, 284);
        }

        if s.b[1058] {
            s.store_div_from_scalar(279, 1.6021918e-19, 1068);
        }

        s.b[1085] = (s.v[221] < 1e-18);
        s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });

        if ((s.v[38] != 0.0) && s.b[1085]) {
            s.store_scalar(221, 1e-18);
        }

        s.b[1086] = (s.v[222] < 1e-18);
        s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });

        if ((s.v[38] != 0.0) && s.b[1086]) {
            s.store_scalar(222, 1e-18);
        }

        if (s.v[38] != 0.0) {
            s.store_div_scaled_inputs2_indices(549, 551, 1.0, 555, (-1.0), 221, 1.0);
            s.store_div_scaled_inputs2_indices(550, 548, 1.0, 556, (-1.0), 222, 1.0);
            s.store_sub_scaled_inputs(554, 551, -1.0, 548, 1.0);
            s.store_scale(552, 551, s.v[385]);
            s.store_scale(553, 551, (1.0 - s.v[385]));
        }

        if (s.v[38] == 0.0) {
            s.store_scalar(549, 0.0);
            s.store_scalar(550, 0.0);
            s.store_scalar(552, 0.0);
            s.store_scalar(553, 0.0);
            s.store_scalar(554, 0.0);
            s.store_scalar(548, 0.0);
        }

        s.b[1087] = (s.v[575] == 1.0);
        s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });

        if s.b[1087] {
            s.copy_ad(94, 0);
            s.copy_ad(185, 494);
            s.copy_ad(561, 14);
            s.copy_ad(93, 15);
            s.store_add_scaled_inputs3_indices(492, 14, (-1.0), 15, (-1.0), 16, (-1.0));
            s.copy_ad(90, 492);
        }

        if (!s.b[1087]) {
            s.store_neg(94, 0);
            s.store_scalar(185, 0.0);
            s.copy_ad(561, 14);
            s.copy_ad(93, 16);
            s.store_add_scaled_inputs3_indices(492, 14, (-1.0), 15, (-1.0), 16, (-1.0));
            s.copy_ad(90, 492);
            s.copy_ad(16, 15);
            s.copy_ad(15, 93);
        }

        if ((!s.b[1087]) && (s.v[38] != 0.0)) {
            s.copy_ad(279, 552);
            s.copy_ad(552, 553);
            s.copy_ad(553, 279);
        }

        s.b[1088] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });

        if s.b[1088] {
            s.store_mul(547, 0, 51);
            s.store_scalar(516, s.v[468]);
            s.store_scalar(557, (1.0 / s.v[467]));
        }

        if (!s.b[1088]) {
            s.store_scalar(547, 0.0);
            s.store_scalar(516, 0.0);
            s.store_scalar(557, 0.0);
        }

        s.copy_ad(0, 94);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        s.b[1094] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });

        s.b[1095] = (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0));
        s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_ids: f64,
        var_ids_db0: f64,
        var_ids_db1: f64,
        var_ids_db2: f64,
        var_ids_db3: f64,
        var_ids_db4: f64,
        var_ids_db5: f64,
        var_ids_db6: f64,
        var_ids_db7: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn2: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_igb: f64,
        var_igb_db0: f64,
        var_igb_db1: f64,
        var_igb_db2: f64,
        var_igb_db3: f64,
        var_igb_db4: f64,
        var_igb_db5: f64,
        var_igb_db6: f64,
        var_igb_db7: f64,
        var_igb_dn0: f64,
        var_igb_dn1: f64,
        var_igb_dn10: f64,
        var_igb_dn11: f64,
        var_igb_dn12: f64,
        var_igb_dn2: f64,
        var_igb_dn3: f64,
        var_igb_dn4: f64,
        var_igb_dn5: f64,
        var_igb_dn6: f64,
        var_igb_dn7: f64,
        var_igb_dn8: f64,
        var_igb_dn9: f64,
        var_igd: f64,
        var_igd_db0: f64,
        var_igd_db1: f64,
        var_igd_db2: f64,
        var_igd_db3: f64,
        var_igd_db4: f64,
        var_igd_db5: f64,
        var_igd_db6: f64,
        var_igd_db7: f64,
        var_igd_dn0: f64,
        var_igd_dn1: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn12: f64,
        var_igd_dn2: f64,
        var_igd_dn3: f64,
        var_igd_dn4: f64,
        var_igd_dn5: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igd_dn8: f64,
        var_igd_dn9: f64,
        var_igidl: f64,
        var_igidl_db0: f64,
        var_igidl_db1: f64,
        var_igidl_db2: f64,
        var_igidl_db3: f64,
        var_igidl_db4: f64,
        var_igidl_db5: f64,
        var_igidl_db6: f64,
        var_igidl_db7: f64,
        var_igidl_dn0: f64,
        var_igidl_dn1: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn2: f64,
        var_igidl_dn3: f64,
        var_igidl_dn4: f64,
        var_igidl_dn5: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igidl_dn8: f64,
        var_igidl_dn9: f64,
        var_igisl: f64,
        var_igisl_db0: f64,
        var_igisl_db1: f64,
        var_igisl_db2: f64,
        var_igisl_db3: f64,
        var_igisl_db4: f64,
        var_igisl_db5: f64,
        var_igisl_db6: f64,
        var_igisl_db7: f64,
        var_igisl_dn0: f64,
        var_igisl_dn1: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn2: f64,
        var_igisl_dn3: f64,
        var_igisl_dn4: f64,
        var_igisl_dn5: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_igisl_dn8: f64,
        var_igisl_dn9: f64,
        var_igs: f64,
        var_igs_db0: f64,
        var_igs_db1: f64,
        var_igs_db2: f64,
        var_igs_db3: f64,
        var_igs_db4: f64,
        var_igs_db5: f64,
        var_igs_db6: f64,
        var_igs_db7: f64,
        var_igs_dn0: f64,
        var_igs_dn1: f64,
        var_igs_dn10: f64,
        var_igs_dn11: f64,
        var_igs_dn12: f64,
        var_igs_dn2: f64,
        var_igs_dn3: f64,
        var_igs_dn4: f64,
        var_igs_dn5: f64,
        var_igs_dn6: f64,
        var_igs_dn7: f64,
        var_igs_dn8: f64,
        var_igs_dn9: f64,
        var_isub: f64,
        var_isub_db0: f64,
        var_isub_db1: f64,
        var_isub_db2: f64,
        var_isub_db3: f64,
        var_isub_db4: f64,
        var_isub_db5: f64,
        var_isub_db6: f64,
        var_isub_db7: f64,
        var_isub_dn0: f64,
        var_isub_dn1: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn2: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_isubs: f64,
        var_isubs_db0: f64,
        var_isubs_db1: f64,
        var_isubs_db2: f64,
        var_isubs_db3: f64,
        var_isubs_db4: f64,
        var_isubs_db5: f64,
        var_isubs_db6: f64,
        var_isubs_db7: f64,
        var_isubs_dn0: f64,
        var_isubs_dn1: f64,
        var_isubs_dn10: f64,
        var_isubs_dn11: f64,
        var_isubs_dn12: f64,
        var_isubs_dn2: f64,
        var_isubs_dn3: f64,
        var_isubs_dn4: f64,
        var_isubs_dn5: f64,
        var_isubs_dn6: f64,
        var_isubs_dn7: f64,
        var_isubs_dn8: f64,
        var_isubs_dn9: f64,
        var_qb: f64,
        var_qb_db0: f64,
        var_qb_db1: f64,
        var_qb_db2: f64,
        var_qb_db3: f64,
        var_qb_db4: f64,
        var_qb_db5: f64,
        var_qb_db6: f64,
        var_qb_db7: f64,
        var_qb_dn0: f64,
        var_qb_dn1: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn2: f64,
        var_qb_dn3: f64,
        var_qb_dn4: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qb_nqs: f64,
        var_qb_nqs_db0: f64,
        var_qb_nqs_db1: f64,
        var_qb_nqs_db2: f64,
        var_qb_nqs_db3: f64,
        var_qb_nqs_db4: f64,
        var_qb_nqs_db5: f64,
        var_qb_nqs_db6: f64,
        var_qb_nqs_db7: f64,
        var_qb_nqs_dn0: f64,
        var_qb_nqs_dn1: f64,
        var_qb_nqs_dn10: f64,
        var_qb_nqs_dn11: f64,
        var_qb_nqs_dn12: f64,
        var_qb_nqs_dn2: f64,
        var_qb_nqs_dn3: f64,
        var_qb_nqs_dn4: f64,
        var_qb_nqs_dn5: f64,
        var_qb_nqs_dn6: f64,
        var_qb_nqs_dn7: f64,
        var_qb_nqs_dn8: f64,
        var_qb_nqs_dn9: f64,
        var_qd: f64,
        var_qd_db0: f64,
        var_qd_db1: f64,
        var_qd_db2: f64,
        var_qd_db3: f64,
        var_qd_db4: f64,
        var_qd_db5: f64,
        var_qd_db6: f64,
        var_qd_db7: f64,
        var_qd_dn0: f64,
        var_qd_dn1: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn12: f64,
        var_qd_dn2: f64,
        var_qd_dn3: f64,
        var_qd_dn4: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qd_nqs: f64,
        var_qd_nqs_db0: f64,
        var_qd_nqs_db1: f64,
        var_qd_nqs_db2: f64,
        var_qd_nqs_db3: f64,
        var_qd_nqs_db4: f64,
        var_qd_nqs_db5: f64,
        var_qd_nqs_db6: f64,
        var_qd_nqs_db7: f64,
        var_qd_nqs_dn0: f64,
        var_qd_nqs_dn1: f64,
        var_qd_nqs_dn10: f64,
        var_qd_nqs_dn11: f64,
        var_qd_nqs_dn12: f64,
        var_qd_nqs_dn2: f64,
        var_qd_nqs_dn3: f64,
        var_qd_nqs_dn4: f64,
        var_qd_nqs_dn5: f64,
        var_qd_nqs_dn6: f64,
        var_qd_nqs_dn7: f64,
        var_qd_nqs_dn8: f64,
        var_qd_nqs_dn9: f64,
        var_qg: f64,
        var_qg_db0: f64,
        var_qg_db1: f64,
        var_qg_db2: f64,
        var_qg_db3: f64,
        var_qg_db4: f64,
        var_qg_db5: f64,
        var_qg_db6: f64,
        var_qg_db7: f64,
        var_qg_dn0: f64,
        var_qg_dn1: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn12: f64,
        var_qg_dn2: f64,
        var_qg_dn3: f64,
        var_qg_dn4: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
        var_qg_nqs: f64,
        var_qg_nqs_db0: f64,
        var_qg_nqs_db1: f64,
        var_qg_nqs_db2: f64,
        var_qg_nqs_db3: f64,
        var_qg_nqs_db4: f64,
        var_qg_nqs_db5: f64,
        var_qg_nqs_db6: f64,
        var_qg_nqs_db7: f64,
        var_qg_nqs_dn0: f64,
        var_qg_nqs_dn1: f64,
        var_qg_nqs_dn10: f64,
        var_qg_nqs_dn11: f64,
        var_qg_nqs_dn12: f64,
        var_qg_nqs_dn2: f64,
        var_qg_nqs_dn3: f64,
        var_qg_nqs_dn4: f64,
        var_qg_nqs_dn5: f64,
        var_qg_nqs_dn6: f64,
        var_qg_nqs_dn7: f64,
        var_qg_nqs_dn8: f64,
        var_qg_nqs_dn9: f64,
        var_rdd: f64,
        var_rdd_db0: f64,
        var_rdd_db1: f64,
        var_rdd_db2: f64,
        var_rdd_db3: f64,
        var_rdd_db4: f64,
        var_rdd_db5: f64,
        var_rdd_db6: f64,
        var_rdd_db7: f64,
        var_rdd_dn0: f64,
        var_rdd_dn1: f64,
        var_rdd_dn10: f64,
        var_rdd_dn11: f64,
        var_rdd_dn12: f64,
        var_rdd_dn2: f64,
        var_rdd_dn3: f64,
        var_rdd_dn4: f64,
        var_rdd_dn5: f64,
        var_rdd_dn6: f64,
        var_rdd_dn7: f64,
        var_rdd_dn8: f64,
        var_rdd_dn9: f64,
        var_rsd: f64,
        var_rsd_db0: f64,
        var_rsd_db1: f64,
        var_rsd_db2: f64,
        var_rsd_db3: f64,
        var_rsd_db4: f64,
        var_rsd_db5: f64,
        var_rsd_db6: f64,
        var_rsd_db7: f64,
        var_rsd_dn0: f64,
        var_rsd_dn1: f64,
        var_rsd_dn10: f64,
        var_rsd_dn11: f64,
        var_rsd_dn12: f64,
        var_rsd_dn2: f64,
        var_rsd_dn3: f64,
        var_rsd_dn4: f64,
        var_rsd_dn5: f64,
        var_rsd_dn6: f64,
        var_rsd_dn7: f64,
        var_rsd_dn8: f64,
        var_rsd_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq0_e342: f64 = (p.p33 * var_ids);
        let eq0_e342_d_n0: f64 = (p.p33 * var_ids_dn0);
        let eq0_e342_d_n1: f64 = (p.p33 * var_ids_dn1);
        let eq0_e342_d_n2: f64 = (p.p33 * var_ids_dn2);
        let eq0_e342_d_n3: f64 = (p.p33 * var_ids_dn3);
        let eq0_e342_d_n4: f64 = (p.p33 * var_ids_dn4);
        let eq0_e342_d_n5: f64 = (p.p33 * var_ids_dn5);
        let eq0_e342_d_n6: f64 = (p.p33 * var_ids_dn6);
        let eq0_e342_d_n7: f64 = (p.p33 * var_ids_dn7);
        let eq0_e342_d_n8: f64 = (p.p33 * var_ids_dn8);
        let eq0_e342_d_n9: f64 = (p.p33 * var_ids_dn9);
        let eq0_e342_d_n10: f64 = (p.p33 * var_ids_dn10);
        let eq0_e342_d_n11: f64 = (p.p33 * var_ids_dn11);
        let eq0_e342_d_n12: f64 = (p.p33 * var_ids_dn12);
        let eq0_e342_d_b0: f64 = (p.p33 * var_ids_db0);
        let eq0_e342_d_b1: f64 = (p.p33 * var_ids_db1);
        let eq0_e342_d_b2: f64 = (p.p33 * var_ids_db2);
        let eq0_e342_d_b3: f64 = (p.p33 * var_ids_db3);
        let eq0_e342_d_b4: f64 = (p.p33 * var_ids_db4);
        let eq0_e342_d_b5: f64 = (p.p33 * var_ids_db5);
        let eq0_e342_d_b6: f64 = (p.p33 * var_ids_db6);
        let eq0_e342_d_b7: f64 = (p.p33 * var_ids_db7);
        let eq0_value: f64 = eq0_e342;
        let eq0_node_derivatives: [f64; 13] = [eq0_e342_d_n0, eq0_e342_d_n1, eq0_e342_d_n2, eq0_e342_d_n3, eq0_e342_d_n4, eq0_e342_d_n5, eq0_e342_d_n6, eq0_e342_d_n7, eq0_e342_d_n8, eq0_e342_d_n9, eq0_e342_d_n10, eq0_e342_d_n11, eq0_e342_d_n12];
        let eq0_branch_derivatives: [f64; 8] = [eq0_e342_d_b0, eq0_e342_d_b1, eq0_e342_d_b2, eq0_e342_d_b3, eq0_e342_d_b4, eq0_e342_d_b5, eq0_e342_d_b6, eq0_e342_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e346: f64 = (var_igidl + var_isub);
        let eq1_e346_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq1_e346_d_n1: f64 = (var_igidl_dn1 + var_isub_dn1);
        let eq1_e346_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq1_e346_d_n3: f64 = (var_igidl_dn3 + var_isub_dn3);
        let eq1_e346_d_n4: f64 = (var_igidl_dn4 + var_isub_dn4);
        let eq1_e346_d_n5: f64 = (var_igidl_dn5 + var_isub_dn5);
        let eq1_e346_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq1_e346_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq1_e346_d_n8: f64 = (var_igidl_dn8 + var_isub_dn8);
        let eq1_e346_d_n9: f64 = (var_igidl_dn9 + var_isub_dn9);
        let eq1_e346_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq1_e346_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq1_e346_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq1_e346_d_b0: f64 = (var_igidl_db0 + var_isub_db0);
        let eq1_e346_d_b1: f64 = (var_igidl_db1 + var_isub_db1);
        let eq1_e346_d_b2: f64 = (var_igidl_db2 + var_isub_db2);
        let eq1_e346_d_b3: f64 = (var_igidl_db3 + var_isub_db3);
        let eq1_e346_d_b4: f64 = (var_igidl_db4 + var_isub_db4);
        let eq1_e346_d_b5: f64 = (var_igidl_db5 + var_isub_db5);
        let eq1_e346_d_b6: f64 = (var_igidl_db6 + var_isub_db6);
        let eq1_e346_d_b7: f64 = (var_igidl_db7 + var_isub_db7);
        let eq1_e347: f64 = (p.p33 * eq1_e346);
        let eq1_e347_d_n0: f64 = (p.p33 * eq1_e346_d_n0);
        let eq1_e347_d_n1: f64 = (p.p33 * eq1_e346_d_n1);
        let eq1_e347_d_n2: f64 = (p.p33 * eq1_e346_d_n2);
        let eq1_e347_d_n3: f64 = (p.p33 * eq1_e346_d_n3);
        let eq1_e347_d_n4: f64 = (p.p33 * eq1_e346_d_n4);
        let eq1_e347_d_n5: f64 = (p.p33 * eq1_e346_d_n5);
        let eq1_e347_d_n6: f64 = (p.p33 * eq1_e346_d_n6);
        let eq1_e347_d_n7: f64 = (p.p33 * eq1_e346_d_n7);
        let eq1_e347_d_n8: f64 = (p.p33 * eq1_e346_d_n8);
        let eq1_e347_d_n9: f64 = (p.p33 * eq1_e346_d_n9);
        let eq1_e347_d_n10: f64 = (p.p33 * eq1_e346_d_n10);
        let eq1_e347_d_n11: f64 = (p.p33 * eq1_e346_d_n11);
        let eq1_e347_d_n12: f64 = (p.p33 * eq1_e346_d_n12);
        let eq1_e347_d_b0: f64 = (p.p33 * eq1_e346_d_b0);
        let eq1_e347_d_b1: f64 = (p.p33 * eq1_e346_d_b1);
        let eq1_e347_d_b2: f64 = (p.p33 * eq1_e346_d_b2);
        let eq1_e347_d_b3: f64 = (p.p33 * eq1_e346_d_b3);
        let eq1_e347_d_b4: f64 = (p.p33 * eq1_e346_d_b4);
        let eq1_e347_d_b5: f64 = (p.p33 * eq1_e346_d_b5);
        let eq1_e347_d_b6: f64 = (p.p33 * eq1_e346_d_b6);
        let eq1_e347_d_b7: f64 = (p.p33 * eq1_e346_d_b7);
        let eq1_value: f64 = eq1_e347;
        let eq1_node_derivatives: [f64; 13] = [eq1_e347_d_n0, eq1_e347_d_n1, eq1_e347_d_n2, eq1_e347_d_n3, eq1_e347_d_n4, eq1_e347_d_n5, eq1_e347_d_n6, eq1_e347_d_n7, eq1_e347_d_n8, eq1_e347_d_n9, eq1_e347_d_n10, eq1_e347_d_n11, eq1_e347_d_n12];
        let eq1_branch_derivatives: [f64; 8] = [eq1_e347_d_b0, eq1_e347_d_b1, eq1_e347_d_b2, eq1_e347_d_b3, eq1_e347_d_b4, eq1_e347_d_b5, eq1_e347_d_b6, eq1_e347_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e351: f64 = (var_igisl + var_isubs);
        let eq2_e351_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq2_e351_d_n1: f64 = (var_igisl_dn1 + var_isubs_dn1);
        let eq2_e351_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq2_e351_d_n3: f64 = (var_igisl_dn3 + var_isubs_dn3);
        let eq2_e351_d_n4: f64 = (var_igisl_dn4 + var_isubs_dn4);
        let eq2_e351_d_n5: f64 = (var_igisl_dn5 + var_isubs_dn5);
        let eq2_e351_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq2_e351_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq2_e351_d_n8: f64 = (var_igisl_dn8 + var_isubs_dn8);
        let eq2_e351_d_n9: f64 = (var_igisl_dn9 + var_isubs_dn9);
        let eq2_e351_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq2_e351_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq2_e351_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq2_e351_d_b0: f64 = (var_igisl_db0 + var_isubs_db0);
        let eq2_e351_d_b1: f64 = (var_igisl_db1 + var_isubs_db1);
        let eq2_e351_d_b2: f64 = (var_igisl_db2 + var_isubs_db2);
        let eq2_e351_d_b3: f64 = (var_igisl_db3 + var_isubs_db3);
        let eq2_e351_d_b4: f64 = (var_igisl_db4 + var_isubs_db4);
        let eq2_e351_d_b5: f64 = (var_igisl_db5 + var_isubs_db5);
        let eq2_e351_d_b6: f64 = (var_igisl_db6 + var_isubs_db6);
        let eq2_e351_d_b7: f64 = (var_igisl_db7 + var_isubs_db7);
        let eq2_e352: f64 = (p.p33 * eq2_e351);
        let eq2_e352_d_n0: f64 = (p.p33 * eq2_e351_d_n0);
        let eq2_e352_d_n1: f64 = (p.p33 * eq2_e351_d_n1);
        let eq2_e352_d_n2: f64 = (p.p33 * eq2_e351_d_n2);
        let eq2_e352_d_n3: f64 = (p.p33 * eq2_e351_d_n3);
        let eq2_e352_d_n4: f64 = (p.p33 * eq2_e351_d_n4);
        let eq2_e352_d_n5: f64 = (p.p33 * eq2_e351_d_n5);
        let eq2_e352_d_n6: f64 = (p.p33 * eq2_e351_d_n6);
        let eq2_e352_d_n7: f64 = (p.p33 * eq2_e351_d_n7);
        let eq2_e352_d_n8: f64 = (p.p33 * eq2_e351_d_n8);
        let eq2_e352_d_n9: f64 = (p.p33 * eq2_e351_d_n9);
        let eq2_e352_d_n10: f64 = (p.p33 * eq2_e351_d_n10);
        let eq2_e352_d_n11: f64 = (p.p33 * eq2_e351_d_n11);
        let eq2_e352_d_n12: f64 = (p.p33 * eq2_e351_d_n12);
        let eq2_e352_d_b0: f64 = (p.p33 * eq2_e351_d_b0);
        let eq2_e352_d_b1: f64 = (p.p33 * eq2_e351_d_b1);
        let eq2_e352_d_b2: f64 = (p.p33 * eq2_e351_d_b2);
        let eq2_e352_d_b3: f64 = (p.p33 * eq2_e351_d_b3);
        let eq2_e352_d_b4: f64 = (p.p33 * eq2_e351_d_b4);
        let eq2_e352_d_b5: f64 = (p.p33 * eq2_e351_d_b5);
        let eq2_e352_d_b6: f64 = (p.p33 * eq2_e351_d_b6);
        let eq2_e352_d_b7: f64 = (p.p33 * eq2_e351_d_b7);
        let eq2_value: f64 = eq2_e352;
        let eq2_node_derivatives: [f64; 13] = [eq2_e352_d_n0, eq2_e352_d_n1, eq2_e352_d_n2, eq2_e352_d_n3, eq2_e352_d_n4, eq2_e352_d_n5, eq2_e352_d_n6, eq2_e352_d_n7, eq2_e352_d_n8, eq2_e352_d_n9, eq2_e352_d_n10, eq2_e352_d_n11, eq2_e352_d_n12];
        let eq2_branch_derivatives: [f64; 8] = [eq2_e352_d_b0, eq2_e352_d_b1, eq2_e352_d_b2, eq2_e352_d_b3, eq2_e352_d_b4, eq2_e352_d_b5, eq2_e352_d_b6, eq2_e352_d_b7];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(11),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e355: f64 = (p.p33 * var_igs);
        let eq3_e355_d_n0: f64 = (p.p33 * var_igs_dn0);
        let eq3_e355_d_n1: f64 = (p.p33 * var_igs_dn1);
        let eq3_e355_d_n2: f64 = (p.p33 * var_igs_dn2);
        let eq3_e355_d_n3: f64 = (p.p33 * var_igs_dn3);
        let eq3_e355_d_n4: f64 = (p.p33 * var_igs_dn4);
        let eq3_e355_d_n5: f64 = (p.p33 * var_igs_dn5);
        let eq3_e355_d_n6: f64 = (p.p33 * var_igs_dn6);
        let eq3_e355_d_n7: f64 = (p.p33 * var_igs_dn7);
        let eq3_e355_d_n8: f64 = (p.p33 * var_igs_dn8);
        let eq3_e355_d_n9: f64 = (p.p33 * var_igs_dn9);
        let eq3_e355_d_n10: f64 = (p.p33 * var_igs_dn10);
        let eq3_e355_d_n11: f64 = (p.p33 * var_igs_dn11);
        let eq3_e355_d_n12: f64 = (p.p33 * var_igs_dn12);
        let eq3_e355_d_b0: f64 = (p.p33 * var_igs_db0);
        let eq3_e355_d_b1: f64 = (p.p33 * var_igs_db1);
        let eq3_e355_d_b2: f64 = (p.p33 * var_igs_db2);
        let eq3_e355_d_b3: f64 = (p.p33 * var_igs_db3);
        let eq3_e355_d_b4: f64 = (p.p33 * var_igs_db4);
        let eq3_e355_d_b5: f64 = (p.p33 * var_igs_db5);
        let eq3_e355_d_b6: f64 = (p.p33 * var_igs_db6);
        let eq3_e355_d_b7: f64 = (p.p33 * var_igs_db7);
        let eq3_value: f64 = eq3_e355;
        let eq3_node_derivatives: [f64; 13] = [eq3_e355_d_n0, eq3_e355_d_n1, eq3_e355_d_n2, eq3_e355_d_n3, eq3_e355_d_n4, eq3_e355_d_n5, eq3_e355_d_n6, eq3_e355_d_n7, eq3_e355_d_n8, eq3_e355_d_n9, eq3_e355_d_n10, eq3_e355_d_n11, eq3_e355_d_n12];
        let eq3_branch_derivatives: [f64; 8] = [eq3_e355_d_b0, eq3_e355_d_b1, eq3_e355_d_b2, eq3_e355_d_b3, eq3_e355_d_b4, eq3_e355_d_b5, eq3_e355_d_b6, eq3_e355_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_e358: f64 = (p.p33 * var_igd);
        let eq4_e358_d_n0: f64 = (p.p33 * var_igd_dn0);
        let eq4_e358_d_n1: f64 = (p.p33 * var_igd_dn1);
        let eq4_e358_d_n2: f64 = (p.p33 * var_igd_dn2);
        let eq4_e358_d_n3: f64 = (p.p33 * var_igd_dn3);
        let eq4_e358_d_n4: f64 = (p.p33 * var_igd_dn4);
        let eq4_e358_d_n5: f64 = (p.p33 * var_igd_dn5);
        let eq4_e358_d_n6: f64 = (p.p33 * var_igd_dn6);
        let eq4_e358_d_n7: f64 = (p.p33 * var_igd_dn7);
        let eq4_e358_d_n8: f64 = (p.p33 * var_igd_dn8);
        let eq4_e358_d_n9: f64 = (p.p33 * var_igd_dn9);
        let eq4_e358_d_n10: f64 = (p.p33 * var_igd_dn10);
        let eq4_e358_d_n11: f64 = (p.p33 * var_igd_dn11);
        let eq4_e358_d_n12: f64 = (p.p33 * var_igd_dn12);
        let eq4_e358_d_b0: f64 = (p.p33 * var_igd_db0);
        let eq4_e358_d_b1: f64 = (p.p33 * var_igd_db1);
        let eq4_e358_d_b2: f64 = (p.p33 * var_igd_db2);
        let eq4_e358_d_b3: f64 = (p.p33 * var_igd_db3);
        let eq4_e358_d_b4: f64 = (p.p33 * var_igd_db4);
        let eq4_e358_d_b5: f64 = (p.p33 * var_igd_db5);
        let eq4_e358_d_b6: f64 = (p.p33 * var_igd_db6);
        let eq4_e358_d_b7: f64 = (p.p33 * var_igd_db7);
        let eq4_value: f64 = eq4_e358;
        let eq4_node_derivatives: [f64; 13] = [eq4_e358_d_n0, eq4_e358_d_n1, eq4_e358_d_n2, eq4_e358_d_n3, eq4_e358_d_n4, eq4_e358_d_n5, eq4_e358_d_n6, eq4_e358_d_n7, eq4_e358_d_n8, eq4_e358_d_n9, eq4_e358_d_n10, eq4_e358_d_n11, eq4_e358_d_n12];
        let eq4_branch_derivatives: [f64; 8] = [eq4_e358_d_b0, eq4_e358_d_b1, eq4_e358_d_b2, eq4_e358_d_b3, eq4_e358_d_b4, eq4_e358_d_b5, eq4_e358_d_b6, eq4_e358_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_e361: f64 = (p.p33 * var_igb);
        let eq5_e361_d_n0: f64 = (p.p33 * var_igb_dn0);
        let eq5_e361_d_n1: f64 = (p.p33 * var_igb_dn1);
        let eq5_e361_d_n2: f64 = (p.p33 * var_igb_dn2);
        let eq5_e361_d_n3: f64 = (p.p33 * var_igb_dn3);
        let eq5_e361_d_n4: f64 = (p.p33 * var_igb_dn4);
        let eq5_e361_d_n5: f64 = (p.p33 * var_igb_dn5);
        let eq5_e361_d_n6: f64 = (p.p33 * var_igb_dn6);
        let eq5_e361_d_n7: f64 = (p.p33 * var_igb_dn7);
        let eq5_e361_d_n8: f64 = (p.p33 * var_igb_dn8);
        let eq5_e361_d_n9: f64 = (p.p33 * var_igb_dn9);
        let eq5_e361_d_n10: f64 = (p.p33 * var_igb_dn10);
        let eq5_e361_d_n11: f64 = (p.p33 * var_igb_dn11);
        let eq5_e361_d_n12: f64 = (p.p33 * var_igb_dn12);
        let eq5_e361_d_b0: f64 = (p.p33 * var_igb_db0);
        let eq5_e361_d_b1: f64 = (p.p33 * var_igb_db1);
        let eq5_e361_d_b2: f64 = (p.p33 * var_igb_db2);
        let eq5_e361_d_b3: f64 = (p.p33 * var_igb_db3);
        let eq5_e361_d_b4: f64 = (p.p33 * var_igb_db4);
        let eq5_e361_d_b5: f64 = (p.p33 * var_igb_db5);
        let eq5_e361_d_b6: f64 = (p.p33 * var_igb_db6);
        let eq5_e361_d_b7: f64 = (p.p33 * var_igb_db7);
        let eq5_value: f64 = eq5_e361;
        let eq5_node_derivatives: [f64; 13] = [eq5_e361_d_n0, eq5_e361_d_n1, eq5_e361_d_n2, eq5_e361_d_n3, eq5_e361_d_n4, eq5_e361_d_n5, eq5_e361_d_n6, eq5_e361_d_n7, eq5_e361_d_n8, eq5_e361_d_n9, eq5_e361_d_n10, eq5_e361_d_n11, eq5_e361_d_n12];
        let eq5_branch_derivatives: [f64; 8] = [eq5_e361_d_b0, eq5_e361_d_b1, eq5_e361_d_b2, eq5_e361_d_b3, eq5_e361_d_b4, eq5_e361_d_b5, eq5_e361_d_b6, eq5_e361_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e367, eq6_e367_d_n0, eq6_e367_d_n1, eq6_e367_d_n2, eq6_e367_d_n3, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n7, eq6_e367_d_n8, eq6_e367_d_n9, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12, eq6_e367_d_b0, eq6_e367_d_b1, eq6_e367_d_b2, eq6_e367_d_b3, eq6_e367_d_b4, eq6_e367_d_b5, eq6_e367_d_b6, eq6_e367_d_b7,) = {
    if (p.p312 != 0.0) {
        let eq6_e365: f64 = ((nv12 - nv2) / var_rsd);
        let eq6_e365_d_n0: f64 = (-(((nv12 - nv2) * var_rsd_dn0) / (var_rsd * var_rsd)));
        let eq6_e365_d_n1: f64 = (-(((nv12 - nv2) * var_rsd_dn1) / (var_rsd * var_rsd)));
        let eq6_e365_d_n2: f64 = (((-var_rsd) - ((nv12 - nv2) * var_rsd_dn2)) / (var_rsd * var_rsd));
        let eq6_e365_d_n3: f64 = (-(((nv12 - nv2) * var_rsd_dn3) / (var_rsd * var_rsd)));
        let eq6_e365_d_n4: f64 = (-(((nv12 - nv2) * var_rsd_dn4) / (var_rsd * var_rsd)));
        let eq6_e365_d_n5: f64 = (-(((nv12 - nv2) * var_rsd_dn5) / (var_rsd * var_rsd)));
        let eq6_e365_d_n6: f64 = (-(((nv12 - nv2) * var_rsd_dn6) / (var_rsd * var_rsd)));
        let eq6_e365_d_n7: f64 = (-(((nv12 - nv2) * var_rsd_dn7) / (var_rsd * var_rsd)));
        let eq6_e365_d_n8: f64 = (-(((nv12 - nv2) * var_rsd_dn8) / (var_rsd * var_rsd)));
        let eq6_e365_d_n9: f64 = (-(((nv12 - nv2) * var_rsd_dn9) / (var_rsd * var_rsd)));
        let eq6_e365_d_n10: f64 = (-(((nv12 - nv2) * var_rsd_dn10) / (var_rsd * var_rsd)));
        let eq6_e365_d_n11: f64 = (-(((nv12 - nv2) * var_rsd_dn11) / (var_rsd * var_rsd)));
        let eq6_e365_d_n12: f64 = ((var_rsd - ((nv12 - nv2) * var_rsd_dn12)) / (var_rsd * var_rsd));
        let eq6_e365_d_b0: f64 = (-(((nv12 - nv2) * var_rsd_db0) / (var_rsd * var_rsd)));
        let eq6_e365_d_b1: f64 = (-(((nv12 - nv2) * var_rsd_db1) / (var_rsd * var_rsd)));
        let eq6_e365_d_b2: f64 = (-(((nv12 - nv2) * var_rsd_db2) / (var_rsd * var_rsd)));
        let eq6_e365_d_b3: f64 = (-(((nv12 - nv2) * var_rsd_db3) / (var_rsd * var_rsd)));
        let eq6_e365_d_b4: f64 = (-(((nv12 - nv2) * var_rsd_db4) / (var_rsd * var_rsd)));
        let eq6_e365_d_b5: f64 = (-(((nv12 - nv2) * var_rsd_db5) / (var_rsd * var_rsd)));
        let eq6_e365_d_b6: f64 = (-(((nv12 - nv2) * var_rsd_db6) / (var_rsd * var_rsd)));
        let eq6_e365_d_b7: f64 = (-(((nv12 - nv2) * var_rsd_db7) / (var_rsd * var_rsd)));
        (eq6_e365, eq6_e365_d_n0, eq6_e365_d_n1, eq6_e365_d_n2, eq6_e365_d_n3, eq6_e365_d_n4, eq6_e365_d_n5, eq6_e365_d_n6, eq6_e365_d_n7, eq6_e365_d_n8, eq6_e365_d_n9, eq6_e365_d_n10, eq6_e365_d_n11, eq6_e365_d_n12, eq6_e365_d_b0, eq6_e365_d_b1, eq6_e365_d_b2, eq6_e365_d_b3, eq6_e365_d_b4, eq6_e365_d_b5, eq6_e365_d_b6, eq6_e365_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e367;
        let eq6_node_derivatives: [f64; 13] = [eq6_e367_d_n0, eq6_e367_d_n1, eq6_e367_d_n2, eq6_e367_d_n3, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n7, eq6_e367_d_n8, eq6_e367_d_n9, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12];
        let eq6_branch_derivatives: [f64; 8] = [eq6_e367_d_b0, eq6_e367_d_b1, eq6_e367_d_b2, eq6_e367_d_b3, eq6_e367_d_b4, eq6_e367_d_b5, eq6_e367_d_b6, eq6_e367_d_b7];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(2),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq8_e378, eq8_e378_d_n0, eq8_e378_d_n1, eq8_e378_d_n2, eq8_e378_d_n3, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n7, eq8_e378_d_n8, eq8_e378_d_n9, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12, eq8_e378_d_b0, eq8_e378_d_b1, eq8_e378_d_b2, eq8_e378_d_b3, eq8_e378_d_b4, eq8_e378_d_b5, eq8_e378_d_b6, eq8_e378_d_b7,) = {
    if (p.p313 != 0.0) {
        let eq8_e376: f64 = ((nv0 - nv11) / var_rdd);
        let eq8_e376_d_n0: f64 = ((var_rdd - ((nv0 - nv11) * var_rdd_dn0)) / (var_rdd * var_rdd));
        let eq8_e376_d_n1: f64 = (-(((nv0 - nv11) * var_rdd_dn1) / (var_rdd * var_rdd)));
        let eq8_e376_d_n2: f64 = (-(((nv0 - nv11) * var_rdd_dn2) / (var_rdd * var_rdd)));
        let eq8_e376_d_n3: f64 = (-(((nv0 - nv11) * var_rdd_dn3) / (var_rdd * var_rdd)));
        let eq8_e376_d_n4: f64 = (-(((nv0 - nv11) * var_rdd_dn4) / (var_rdd * var_rdd)));
        let eq8_e376_d_n5: f64 = (-(((nv0 - nv11) * var_rdd_dn5) / (var_rdd * var_rdd)));
        let eq8_e376_d_n6: f64 = (-(((nv0 - nv11) * var_rdd_dn6) / (var_rdd * var_rdd)));
        let eq8_e376_d_n7: f64 = (-(((nv0 - nv11) * var_rdd_dn7) / (var_rdd * var_rdd)));
        let eq8_e376_d_n8: f64 = (-(((nv0 - nv11) * var_rdd_dn8) / (var_rdd * var_rdd)));
        let eq8_e376_d_n9: f64 = (-(((nv0 - nv11) * var_rdd_dn9) / (var_rdd * var_rdd)));
        let eq8_e376_d_n10: f64 = (-(((nv0 - nv11) * var_rdd_dn10) / (var_rdd * var_rdd)));
        let eq8_e376_d_n11: f64 = (((-var_rdd) - ((nv0 - nv11) * var_rdd_dn11)) / (var_rdd * var_rdd));
        let eq8_e376_d_n12: f64 = (-(((nv0 - nv11) * var_rdd_dn12) / (var_rdd * var_rdd)));
        let eq8_e376_d_b0: f64 = (-(((nv0 - nv11) * var_rdd_db0) / (var_rdd * var_rdd)));
        let eq8_e376_d_b1: f64 = (-(((nv0 - nv11) * var_rdd_db1) / (var_rdd * var_rdd)));
        let eq8_e376_d_b2: f64 = (-(((nv0 - nv11) * var_rdd_db2) / (var_rdd * var_rdd)));
        let eq8_e376_d_b3: f64 = (-(((nv0 - nv11) * var_rdd_db3) / (var_rdd * var_rdd)));
        let eq8_e376_d_b4: f64 = (-(((nv0 - nv11) * var_rdd_db4) / (var_rdd * var_rdd)));
        let eq8_e376_d_b5: f64 = (-(((nv0 - nv11) * var_rdd_db5) / (var_rdd * var_rdd)));
        let eq8_e376_d_b6: f64 = (-(((nv0 - nv11) * var_rdd_db6) / (var_rdd * var_rdd)));
        let eq8_e376_d_b7: f64 = (-(((nv0 - nv11) * var_rdd_db7) / (var_rdd * var_rdd)));
        (eq8_e376, eq8_e376_d_n0, eq8_e376_d_n1, eq8_e376_d_n2, eq8_e376_d_n3, eq8_e376_d_n4, eq8_e376_d_n5, eq8_e376_d_n6, eq8_e376_d_n7, eq8_e376_d_n8, eq8_e376_d_n9, eq8_e376_d_n10, eq8_e376_d_n11, eq8_e376_d_n12, eq8_e376_d_b0, eq8_e376_d_b1, eq8_e376_d_b2, eq8_e376_d_b3, eq8_e376_d_b4, eq8_e376_d_b5, eq8_e376_d_b6, eq8_e376_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e378;
        let eq8_node_derivatives: [f64; 13] = [eq8_e378_d_n0, eq8_e378_d_n1, eq8_e378_d_n2, eq8_e378_d_n3, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n7, eq8_e378_d_n8, eq8_e378_d_n9, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12];
        let eq8_branch_derivatives: [f64; 8] = [eq8_e378_d_b0, eq8_e378_d_b1, eq8_e378_d_b2, eq8_e378_d_b3, eq8_e378_d_b4, eq8_e378_d_b5, eq8_e378_d_b6, eq8_e378_d_b7];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(11),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq10_e387: f64 = (var_qg + var_qg_nqs);
        let eq10_e387_d_n0: f64 = (var_qg_dn0 + var_qg_nqs_dn0);
        let eq10_e387_d_n1: f64 = (var_qg_dn1 + var_qg_nqs_dn1);
        let eq10_e387_d_n2: f64 = (var_qg_dn2 + var_qg_nqs_dn2);
        let eq10_e387_d_n3: f64 = (var_qg_dn3 + var_qg_nqs_dn3);
        let eq10_e387_d_n4: f64 = (var_qg_dn4 + var_qg_nqs_dn4);
        let eq10_e387_d_n5: f64 = (var_qg_dn5 + var_qg_nqs_dn5);
        let eq10_e387_d_n6: f64 = (var_qg_dn6 + var_qg_nqs_dn6);
        let eq10_e387_d_n7: f64 = (var_qg_dn7 + var_qg_nqs_dn7);
        let eq10_e387_d_n8: f64 = (var_qg_dn8 + var_qg_nqs_dn8);
        let eq10_e387_d_n9: f64 = (var_qg_dn9 + var_qg_nqs_dn9);
        let eq10_e387_d_n10: f64 = (var_qg_dn10 + var_qg_nqs_dn10);
        let eq10_e387_d_n11: f64 = (var_qg_dn11 + var_qg_nqs_dn11);
        let eq10_e387_d_n12: f64 = (var_qg_dn12 + var_qg_nqs_dn12);
        let eq10_e387_d_b0: f64 = (var_qg_db0 + var_qg_nqs_db0);
        let eq10_e387_d_b1: f64 = (var_qg_db1 + var_qg_nqs_db1);
        let eq10_e387_d_b2: f64 = (var_qg_db2 + var_qg_nqs_db2);
        let eq10_e387_d_b3: f64 = (var_qg_db3 + var_qg_nqs_db3);
        let eq10_e387_d_b4: f64 = (var_qg_db4 + var_qg_nqs_db4);
        let eq10_e387_d_b5: f64 = (var_qg_db5 + var_qg_nqs_db5);
        let eq10_e387_d_b6: f64 = (var_qg_db6 + var_qg_nqs_db6);
        let eq10_e387_d_b7: f64 = (var_qg_db7 + var_qg_nqs_db7);
        let eq10_e388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq10_e387);
        let eq10_e389: f64 = (p.p33 * eq10_e388);
        let eq10_e389_d_n0: f64 = (p.p33 * (eq10_e387_d_n0 * ddt_scale));
        let eq10_e389_d_n1: f64 = (p.p33 * (eq10_e387_d_n1 * ddt_scale));
        let eq10_e389_d_n2: f64 = (p.p33 * (eq10_e387_d_n2 * ddt_scale));
        let eq10_e389_d_n3: f64 = (p.p33 * (eq10_e387_d_n3 * ddt_scale));
        let eq10_e389_d_n4: f64 = (p.p33 * (eq10_e387_d_n4 * ddt_scale));
        let eq10_e389_d_n5: f64 = (p.p33 * (eq10_e387_d_n5 * ddt_scale));
        let eq10_e389_d_n6: f64 = (p.p33 * (eq10_e387_d_n6 * ddt_scale));
        let eq10_e389_d_n7: f64 = (p.p33 * (eq10_e387_d_n7 * ddt_scale));
        let eq10_e389_d_n8: f64 = (p.p33 * (eq10_e387_d_n8 * ddt_scale));
        let eq10_e389_d_n9: f64 = (p.p33 * (eq10_e387_d_n9 * ddt_scale));
        let eq10_e389_d_n10: f64 = (p.p33 * (eq10_e387_d_n10 * ddt_scale));
        let eq10_e389_d_n11: f64 = (p.p33 * (eq10_e387_d_n11 * ddt_scale));
        let eq10_e389_d_n12: f64 = (p.p33 * (eq10_e387_d_n12 * ddt_scale));
        let eq10_e389_d_b0: f64 = (p.p33 * (eq10_e387_d_b0 * ddt_scale));
        let eq10_e389_d_b1: f64 = (p.p33 * (eq10_e387_d_b1 * ddt_scale));
        let eq10_e389_d_b2: f64 = (p.p33 * (eq10_e387_d_b2 * ddt_scale));
        let eq10_e389_d_b3: f64 = (p.p33 * (eq10_e387_d_b3 * ddt_scale));
        let eq10_e389_d_b4: f64 = (p.p33 * (eq10_e387_d_b4 * ddt_scale));
        let eq10_e389_d_b5: f64 = (p.p33 * (eq10_e387_d_b5 * ddt_scale));
        let eq10_e389_d_b6: f64 = (p.p33 * (eq10_e387_d_b6 * ddt_scale));
        let eq10_e389_d_b7: f64 = (p.p33 * (eq10_e387_d_b7 * ddt_scale));
        let eq10_value: f64 = eq10_e389;
        let eq10_node_derivatives: [f64; 13] = [eq10_e389_d_n0, eq10_e389_d_n1, eq10_e389_d_n2, eq10_e389_d_n3, eq10_e389_d_n4, eq10_e389_d_n5, eq10_e389_d_n6, eq10_e389_d_n7, eq10_e389_d_n8, eq10_e389_d_n9, eq10_e389_d_n10, eq10_e389_d_n11, eq10_e389_d_n12];
        let eq10_branch_derivatives: [f64; 8] = [eq10_e389_d_b0, eq10_e389_d_b1, eq10_e389_d_b2, eq10_e389_d_b3, eq10_e389_d_b4, eq10_e389_d_b5, eq10_e389_d_b6, eq10_e389_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e393: f64 = (var_qd + var_qd_nqs);
        let eq11_e393_d_n0: f64 = (var_qd_dn0 + var_qd_nqs_dn0);
        let eq11_e393_d_n1: f64 = (var_qd_dn1 + var_qd_nqs_dn1);
        let eq11_e393_d_n2: f64 = (var_qd_dn2 + var_qd_nqs_dn2);
        let eq11_e393_d_n3: f64 = (var_qd_dn3 + var_qd_nqs_dn3);
        let eq11_e393_d_n4: f64 = (var_qd_dn4 + var_qd_nqs_dn4);
        let eq11_e393_d_n5: f64 = (var_qd_dn5 + var_qd_nqs_dn5);
        let eq11_e393_d_n6: f64 = (var_qd_dn6 + var_qd_nqs_dn6);
        let eq11_e393_d_n7: f64 = (var_qd_dn7 + var_qd_nqs_dn7);
        let eq11_e393_d_n8: f64 = (var_qd_dn8 + var_qd_nqs_dn8);
        let eq11_e393_d_n9: f64 = (var_qd_dn9 + var_qd_nqs_dn9);
        let eq11_e393_d_n10: f64 = (var_qd_dn10 + var_qd_nqs_dn10);
        let eq11_e393_d_n11: f64 = (var_qd_dn11 + var_qd_nqs_dn11);
        let eq11_e393_d_n12: f64 = (var_qd_dn12 + var_qd_nqs_dn12);
        let eq11_e393_d_b0: f64 = (var_qd_db0 + var_qd_nqs_db0);
        let eq11_e393_d_b1: f64 = (var_qd_db1 + var_qd_nqs_db1);
        let eq11_e393_d_b2: f64 = (var_qd_db2 + var_qd_nqs_db2);
        let eq11_e393_d_b3: f64 = (var_qd_db3 + var_qd_nqs_db3);
        let eq11_e393_d_b4: f64 = (var_qd_db4 + var_qd_nqs_db4);
        let eq11_e393_d_b5: f64 = (var_qd_db5 + var_qd_nqs_db5);
        let eq11_e393_d_b6: f64 = (var_qd_db6 + var_qd_nqs_db6);
        let eq11_e393_d_b7: f64 = (var_qd_db7 + var_qd_nqs_db7);
        let eq11_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e393);
        let eq11_e395: f64 = (p.p33 * eq11_e394);
        let eq11_e395_d_n0: f64 = (p.p33 * (eq11_e393_d_n0 * ddt_scale));
        let eq11_e395_d_n1: f64 = (p.p33 * (eq11_e393_d_n1 * ddt_scale));
        let eq11_e395_d_n2: f64 = (p.p33 * (eq11_e393_d_n2 * ddt_scale));
        let eq11_e395_d_n3: f64 = (p.p33 * (eq11_e393_d_n3 * ddt_scale));
        let eq11_e395_d_n4: f64 = (p.p33 * (eq11_e393_d_n4 * ddt_scale));
        let eq11_e395_d_n5: f64 = (p.p33 * (eq11_e393_d_n5 * ddt_scale));
        let eq11_e395_d_n6: f64 = (p.p33 * (eq11_e393_d_n6 * ddt_scale));
        let eq11_e395_d_n7: f64 = (p.p33 * (eq11_e393_d_n7 * ddt_scale));
        let eq11_e395_d_n8: f64 = (p.p33 * (eq11_e393_d_n8 * ddt_scale));
        let eq11_e395_d_n9: f64 = (p.p33 * (eq11_e393_d_n9 * ddt_scale));
        let eq11_e395_d_n10: f64 = (p.p33 * (eq11_e393_d_n10 * ddt_scale));
        let eq11_e395_d_n11: f64 = (p.p33 * (eq11_e393_d_n11 * ddt_scale));
        let eq11_e395_d_n12: f64 = (p.p33 * (eq11_e393_d_n12 * ddt_scale));
        let eq11_e395_d_b0: f64 = (p.p33 * (eq11_e393_d_b0 * ddt_scale));
        let eq11_e395_d_b1: f64 = (p.p33 * (eq11_e393_d_b1 * ddt_scale));
        let eq11_e395_d_b2: f64 = (p.p33 * (eq11_e393_d_b2 * ddt_scale));
        let eq11_e395_d_b3: f64 = (p.p33 * (eq11_e393_d_b3 * ddt_scale));
        let eq11_e395_d_b4: f64 = (p.p33 * (eq11_e393_d_b4 * ddt_scale));
        let eq11_e395_d_b5: f64 = (p.p33 * (eq11_e393_d_b5 * ddt_scale));
        let eq11_e395_d_b6: f64 = (p.p33 * (eq11_e393_d_b6 * ddt_scale));
        let eq11_e395_d_b7: f64 = (p.p33 * (eq11_e393_d_b7 * ddt_scale));
        let eq11_value: f64 = eq11_e395;
        let eq11_node_derivatives: [f64; 13] = [eq11_e395_d_n0, eq11_e395_d_n1, eq11_e395_d_n2, eq11_e395_d_n3, eq11_e395_d_n4, eq11_e395_d_n5, eq11_e395_d_n6, eq11_e395_d_n7, eq11_e395_d_n8, eq11_e395_d_n9, eq11_e395_d_n10, eq11_e395_d_n11, eq11_e395_d_n12];
        let eq11_branch_derivatives: [f64; 8] = [eq11_e395_d_b0, eq11_e395_d_b1, eq11_e395_d_b2, eq11_e395_d_b3, eq11_e395_d_b4, eq11_e395_d_b5, eq11_e395_d_b6, eq11_e395_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e399: f64 = (var_qb + var_qb_nqs);
        let eq12_e399_d_n0: f64 = (var_qb_dn0 + var_qb_nqs_dn0);
        let eq12_e399_d_n1: f64 = (var_qb_dn1 + var_qb_nqs_dn1);
        let eq12_e399_d_n2: f64 = (var_qb_dn2 + var_qb_nqs_dn2);
        let eq12_e399_d_n3: f64 = (var_qb_dn3 + var_qb_nqs_dn3);
        let eq12_e399_d_n4: f64 = (var_qb_dn4 + var_qb_nqs_dn4);
        let eq12_e399_d_n5: f64 = (var_qb_dn5 + var_qb_nqs_dn5);
        let eq12_e399_d_n6: f64 = (var_qb_dn6 + var_qb_nqs_dn6);
        let eq12_e399_d_n7: f64 = (var_qb_dn7 + var_qb_nqs_dn7);
        let eq12_e399_d_n8: f64 = (var_qb_dn8 + var_qb_nqs_dn8);
        let eq12_e399_d_n9: f64 = (var_qb_dn9 + var_qb_nqs_dn9);
        let eq12_e399_d_n10: f64 = (var_qb_dn10 + var_qb_nqs_dn10);
        let eq12_e399_d_n11: f64 = (var_qb_dn11 + var_qb_nqs_dn11);
        let eq12_e399_d_n12: f64 = (var_qb_dn12 + var_qb_nqs_dn12);
        let eq12_e399_d_b0: f64 = (var_qb_db0 + var_qb_nqs_db0);
        let eq12_e399_d_b1: f64 = (var_qb_db1 + var_qb_nqs_db1);
        let eq12_e399_d_b2: f64 = (var_qb_db2 + var_qb_nqs_db2);
        let eq12_e399_d_b3: f64 = (var_qb_db3 + var_qb_nqs_db3);
        let eq12_e399_d_b4: f64 = (var_qb_db4 + var_qb_nqs_db4);
        let eq12_e399_d_b5: f64 = (var_qb_db5 + var_qb_nqs_db5);
        let eq12_e399_d_b6: f64 = (var_qb_db6 + var_qb_nqs_db6);
        let eq12_e399_d_b7: f64 = (var_qb_db7 + var_qb_nqs_db7);
        let eq12_e400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e399);
        let eq12_e401: f64 = (p.p33 * eq12_e400);
        let eq12_e401_d_n0: f64 = (p.p33 * (eq12_e399_d_n0 * ddt_scale));
        let eq12_e401_d_n1: f64 = (p.p33 * (eq12_e399_d_n1 * ddt_scale));
        let eq12_e401_d_n2: f64 = (p.p33 * (eq12_e399_d_n2 * ddt_scale));
        let eq12_e401_d_n3: f64 = (p.p33 * (eq12_e399_d_n3 * ddt_scale));
        let eq12_e401_d_n4: f64 = (p.p33 * (eq12_e399_d_n4 * ddt_scale));
        let eq12_e401_d_n5: f64 = (p.p33 * (eq12_e399_d_n5 * ddt_scale));
        let eq12_e401_d_n6: f64 = (p.p33 * (eq12_e399_d_n6 * ddt_scale));
        let eq12_e401_d_n7: f64 = (p.p33 * (eq12_e399_d_n7 * ddt_scale));
        let eq12_e401_d_n8: f64 = (p.p33 * (eq12_e399_d_n8 * ddt_scale));
        let eq12_e401_d_n9: f64 = (p.p33 * (eq12_e399_d_n9 * ddt_scale));
        let eq12_e401_d_n10: f64 = (p.p33 * (eq12_e399_d_n10 * ddt_scale));
        let eq12_e401_d_n11: f64 = (p.p33 * (eq12_e399_d_n11 * ddt_scale));
        let eq12_e401_d_n12: f64 = (p.p33 * (eq12_e399_d_n12 * ddt_scale));
        let eq12_e401_d_b0: f64 = (p.p33 * (eq12_e399_d_b0 * ddt_scale));
        let eq12_e401_d_b1: f64 = (p.p33 * (eq12_e399_d_b1 * ddt_scale));
        let eq12_e401_d_b2: f64 = (p.p33 * (eq12_e399_d_b2 * ddt_scale));
        let eq12_e401_d_b3: f64 = (p.p33 * (eq12_e399_d_b3 * ddt_scale));
        let eq12_e401_d_b4: f64 = (p.p33 * (eq12_e399_d_b4 * ddt_scale));
        let eq12_e401_d_b5: f64 = (p.p33 * (eq12_e399_d_b5 * ddt_scale));
        let eq12_e401_d_b6: f64 = (p.p33 * (eq12_e399_d_b6 * ddt_scale));
        let eq12_e401_d_b7: f64 = (p.p33 * (eq12_e399_d_b7 * ddt_scale));
        let eq12_value: f64 = eq12_e401;
        let eq12_node_derivatives: [f64; 13] = [eq12_e401_d_n0, eq12_e401_d_n1, eq12_e401_d_n2, eq12_e401_d_n3, eq12_e401_d_n4, eq12_e401_d_n5, eq12_e401_d_n6, eq12_e401_d_n7, eq12_e401_d_n8, eq12_e401_d_n9, eq12_e401_d_n10, eq12_e401_d_n11, eq12_e401_d_n12];
        let eq12_branch_derivatives: [f64; 8] = [eq12_e401_d_b0, eq12_e401_d_b1, eq12_e401_d_b2, eq12_e401_d_b3, eq12_e401_d_b4, eq12_e401_d_b5, eq12_e401_d_b6, eq12_e401_d_b7];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_ci: f64,
        var_ci_db0: f64,
        var_ci_db1: f64,
        var_ci_db2: f64,
        var_ci_db3: f64,
        var_ci_db4: f64,
        var_ci_db5: f64,
        var_ci_db6: f64,
        var_ci_db7: f64,
        var_ci_dn0: f64,
        var_ci_dn1: f64,
        var_ci_dn10: f64,
        var_ci_dn11: f64,
        var_ci_dn12: f64,
        var_ci_dn2: f64,
        var_ci_dn3: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_ci_dn8: f64,
        var_ci_dn9: f64,
        var_cthe: f64,
        var_cthe_db0: f64,
        var_cthe_db1: f64,
        var_cthe_db2: f64,
        var_cthe_db3: f64,
        var_cthe_db4: f64,
        var_cthe_db5: f64,
        var_cthe_db6: f64,
        var_cthe_db7: f64,
        var_cthe_dn0: f64,
        var_cthe_dn1: f64,
        var_cthe_dn10: f64,
        var_cthe_dn11: f64,
        var_cthe_dn12: f64,
        var_cthe_dn2: f64,
        var_cthe_dn3: f64,
        var_cthe_dn4: f64,
        var_cthe_dn5: f64,
        var_cthe_dn6: f64,
        var_cthe_dn7: f64,
        var_cthe_dn8: f64,
        var_cthe_dn9: f64,
        var_gth: f64,
        var_gth_db0: f64,
        var_gth_db1: f64,
        var_gth_db2: f64,
        var_gth_db3: f64,
        var_gth_db4: f64,
        var_gth_db5: f64,
        var_gth_db6: f64,
        var_gth_db7: f64,
        var_gth_dn0: f64,
        var_gth_dn1: f64,
        var_gth_dn10: f64,
        var_gth_dn11: f64,
        var_gth_dn12: f64,
        var_gth_dn2: f64,
        var_gth_dn3: f64,
        var_gth_dn4: f64,
        var_gth_dn5: f64,
        var_gth_dn6: f64,
        var_gth_dn7: f64,
        var_gth_dn8: f64,
        var_gth_dn9: f64,
        var_guard443: f64,
        var_guard444: f64,
        var_iqb_nqs: f64,
        var_iqb_nqs_db0: f64,
        var_iqb_nqs_db1: f64,
        var_iqb_nqs_db2: f64,
        var_iqb_nqs_db3: f64,
        var_iqb_nqs_db4: f64,
        var_iqb_nqs_db5: f64,
        var_iqb_nqs_db6: f64,
        var_iqb_nqs_db7: f64,
        var_iqb_nqs_dn0: f64,
        var_iqb_nqs_dn1: f64,
        var_iqb_nqs_dn10: f64,
        var_iqb_nqs_dn11: f64,
        var_iqb_nqs_dn12: f64,
        var_iqb_nqs_dn2: f64,
        var_iqb_nqs_dn3: f64,
        var_iqb_nqs_dn4: f64,
        var_iqb_nqs_dn5: f64,
        var_iqb_nqs_dn6: f64,
        var_iqb_nqs_dn7: f64,
        var_iqb_nqs_dn8: f64,
        var_iqb_nqs_dn9: f64,
        var_iqh_nqs: f64,
        var_iqh_nqs_db0: f64,
        var_iqh_nqs_db1: f64,
        var_iqh_nqs_db2: f64,
        var_iqh_nqs_db3: f64,
        var_iqh_nqs_db4: f64,
        var_iqh_nqs_db5: f64,
        var_iqh_nqs_db6: f64,
        var_iqh_nqs_db7: f64,
        var_iqh_nqs_dn0: f64,
        var_iqh_nqs_dn1: f64,
        var_iqh_nqs_dn10: f64,
        var_iqh_nqs_dn11: f64,
        var_iqh_nqs_dn12: f64,
        var_iqh_nqs_dn2: f64,
        var_iqh_nqs_dn3: f64,
        var_iqh_nqs_dn4: f64,
        var_iqh_nqs_dn5: f64,
        var_iqh_nqs_dn6: f64,
        var_iqh_nqs_dn7: f64,
        var_iqh_nqs_dn8: f64,
        var_iqh_nqs_dn9: f64,
        var_iqi_nqs: f64,
        var_iqi_nqs_db0: f64,
        var_iqi_nqs_db1: f64,
        var_iqi_nqs_db2: f64,
        var_iqi_nqs_db3: f64,
        var_iqi_nqs_db4: f64,
        var_iqi_nqs_db5: f64,
        var_iqi_nqs_db6: f64,
        var_iqi_nqs_db7: f64,
        var_iqi_nqs_dn0: f64,
        var_iqi_nqs_dn1: f64,
        var_iqi_nqs_dn10: f64,
        var_iqi_nqs_dn11: f64,
        var_iqi_nqs_dn12: f64,
        var_iqi_nqs_dn2: f64,
        var_iqi_nqs_dn3: f64,
        var_iqi_nqs_dn4: f64,
        var_iqi_nqs_dn5: f64,
        var_iqi_nqs_dn6: f64,
        var_iqi_nqs_dn7: f64,
        var_iqi_nqs_dn8: f64,
        var_iqi_nqs_dn9: f64,
        var_rpower: f64,
        var_rpower_db0: f64,
        var_rpower_db1: f64,
        var_rpower_db2: f64,
        var_rpower_db3: f64,
        var_rpower_db4: f64,
        var_rpower_db5: f64,
        var_rpower_db6: f64,
        var_rpower_db7: f64,
        var_rpower_dn0: f64,
        var_rpower_dn1: f64,
        var_rpower_dn10: f64,
        var_rpower_dn11: f64,
        var_rpower_dn12: f64,
        var_rpower_dn2: f64,
        var_rpower_dn3: f64,
        var_rpower_dn4: f64,
        var_rpower_dn5: f64,
        var_rpower_dn6: f64,
        var_rpower_dn7: f64,
        var_rpower_dn8: f64,
        var_rpower_dn9: f64,
        var_sigrat_d: f64,
        var_sigrat_d_db0: f64,
        var_sigrat_d_db1: f64,
        var_sigrat_d_db2: f64,
        var_sigrat_d_db3: f64,
        var_sigrat_d_db4: f64,
        var_sigrat_d_db5: f64,
        var_sigrat_d_db6: f64,
        var_sigrat_d_db7: f64,
        var_sigrat_d_dn0: f64,
        var_sigrat_d_dn1: f64,
        var_sigrat_d_dn10: f64,
        var_sigrat_d_dn11: f64,
        var_sigrat_d_dn12: f64,
        var_sigrat_d_dn2: f64,
        var_sigrat_d_dn3: f64,
        var_sigrat_d_dn4: f64,
        var_sigrat_d_dn5: f64,
        var_sigrat_d_dn6: f64,
        var_sigrat_d_dn7: f64,
        var_sigrat_d_dn8: f64,
        var_sigrat_d_dn9: f64,
        var_sigrat_s: f64,
        var_sigrat_s_db0: f64,
        var_sigrat_s_db1: f64,
        var_sigrat_s_db2: f64,
        var_sigrat_s_db3: f64,
        var_sigrat_s_db4: f64,
        var_sigrat_s_db5: f64,
        var_sigrat_s_db6: f64,
        var_sigrat_s_db7: f64,
        var_sigrat_s_dn0: f64,
        var_sigrat_s_dn1: f64,
        var_sigrat_s_dn10: f64,
        var_sigrat_s_dn11: f64,
        var_sigrat_s_dn12: f64,
        var_sigrat_s_dn2: f64,
        var_sigrat_s_dn3: f64,
        var_sigrat_s_dn4: f64,
        var_sigrat_s_dn5: f64,
        var_sigrat_s_dn6: f64,
        var_sigrat_s_dn7: f64,
        var_sigrat_s_dn8: f64,
        var_sigrat_s_dn9: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq17_e427: f64 = (var_ci * (nv7 - 0.0));
        let eq17_e427_d_n0: f64 = (var_ci_dn0 * (nv7 - 0.0));
        let eq17_e427_d_n1: f64 = (var_ci_dn1 * (nv7 - 0.0));
        let eq17_e427_d_n2: f64 = (var_ci_dn2 * (nv7 - 0.0));
        let eq17_e427_d_n3: f64 = (var_ci_dn3 * (nv7 - 0.0));
        let eq17_e427_d_n4: f64 = (var_ci_dn4 * (nv7 - 0.0));
        let eq17_e427_d_n5: f64 = (var_ci_dn5 * (nv7 - 0.0));
        let eq17_e427_d_n6: f64 = (var_ci_dn6 * (nv7 - 0.0));
        let eq17_e427_d_n7: f64 = ((var_ci_dn7 * (nv7 - 0.0)) + var_ci);
        let eq17_e427_d_n8: f64 = (var_ci_dn8 * (nv7 - 0.0));
        let eq17_e427_d_n9: f64 = (var_ci_dn9 * (nv7 - 0.0));
        let eq17_e427_d_n10: f64 = (var_ci_dn10 * (nv7 - 0.0));
        let eq17_e427_d_n11: f64 = (var_ci_dn11 * (nv7 - 0.0));
        let eq17_e427_d_n12: f64 = (var_ci_dn12 * (nv7 - 0.0));
        let eq17_e427_d_b0: f64 = (var_ci_db0 * (nv7 - 0.0));
        let eq17_e427_d_b1: f64 = (var_ci_db1 * (nv7 - 0.0));
        let eq17_e427_d_b2: f64 = (var_ci_db2 * (nv7 - 0.0));
        let eq17_e427_d_b3: f64 = (var_ci_db3 * (nv7 - 0.0));
        let eq17_e427_d_b4: f64 = (var_ci_db4 * (nv7 - 0.0));
        let eq17_e427_d_b5: f64 = (var_ci_db5 * (nv7 - 0.0));
        let eq17_e427_d_b6: f64 = (var_ci_db6 * (nv7 - 0.0));
        let eq17_e427_d_b7: f64 = (var_ci_db7 * (nv7 - 0.0));
        let eq17_value: f64 = eq17_e427;
        let eq17_node_derivatives: [f64; 13] = [eq17_e427_d_n0, eq17_e427_d_n1, eq17_e427_d_n2, eq17_e427_d_n3, eq17_e427_d_n4, eq17_e427_d_n5, eq17_e427_d_n6, eq17_e427_d_n7, eq17_e427_d_n8, eq17_e427_d_n9, eq17_e427_d_n10, eq17_e427_d_n11, eq17_e427_d_n12];
        let eq17_branch_derivatives: [f64; 8] = [eq17_e427_d_b0, eq17_e427_d_b1, eq17_e427_d_b2, eq17_e427_d_b3, eq17_e427_d_b4, eq17_e427_d_b5, eq17_e427_d_b6, eq17_e427_d_b7];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * var_sigrat_s);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * var_sigrat_s_dn0);
        let eq18_e430_d_n1: f64 = ((nv7 - 0.0) * var_sigrat_s_dn1);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * var_sigrat_s_dn2);
        let eq18_e430_d_n3: f64 = ((nv7 - 0.0) * var_sigrat_s_dn3);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * var_sigrat_s_dn4);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * var_sigrat_s_dn5);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * var_sigrat_s_dn6);
        let eq18_e430_d_n7: f64 = (var_sigrat_s + ((nv7 - 0.0) * var_sigrat_s_dn7));
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * var_sigrat_s_dn8);
        let eq18_e430_d_n9: f64 = ((nv7 - 0.0) * var_sigrat_s_dn9);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * var_sigrat_s_dn10);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * var_sigrat_s_dn11);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * var_sigrat_s_dn12);
        let eq18_e430_d_b0: f64 = ((nv7 - 0.0) * var_sigrat_s_db0);
        let eq18_e430_d_b1: f64 = ((nv7 - 0.0) * var_sigrat_s_db1);
        let eq18_e430_d_b2: f64 = ((nv7 - 0.0) * var_sigrat_s_db2);
        let eq18_e430_d_b3: f64 = ((nv7 - 0.0) * var_sigrat_s_db3);
        let eq18_e430_d_b4: f64 = ((nv7 - 0.0) * var_sigrat_s_db4);
        let eq18_e430_d_b5: f64 = ((nv7 - 0.0) * var_sigrat_s_db5);
        let eq18_e430_d_b6: f64 = ((nv7 - 0.0) * var_sigrat_s_db6);
        let eq18_e430_d_b7: f64 = ((nv7 - 0.0) * var_sigrat_s_db7);
        let eq18_e431: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e430);
        let eq18_value: f64 = eq18_e431;
        let eq18_node_derivatives: [f64; 13] = [(eq18_e430_d_n0 * ddt_scale), (eq18_e430_d_n1 * ddt_scale), (eq18_e430_d_n2 * ddt_scale), (eq18_e430_d_n3 * ddt_scale), (eq18_e430_d_n4 * ddt_scale), (eq18_e430_d_n5 * ddt_scale), (eq18_e430_d_n6 * ddt_scale), (eq18_e430_d_n7 * ddt_scale), (eq18_e430_d_n8 * ddt_scale), (eq18_e430_d_n9 * ddt_scale), (eq18_e430_d_n10 * ddt_scale), (eq18_e430_d_n11 * ddt_scale), (eq18_e430_d_n12 * ddt_scale)];
        let eq18_branch_derivatives: [f64; 8] = [(eq18_e430_d_b0 * ddt_scale), (eq18_e430_d_b1 * ddt_scale), (eq18_e430_d_b2 * ddt_scale), (eq18_e430_d_b3 * ddt_scale), (eq18_e430_d_b4 * ddt_scale), (eq18_e430_d_b5 * ddt_scale), (eq18_e430_d_b6 * ddt_scale), (eq18_e430_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(12),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * var_sigrat_d);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * var_sigrat_d_dn0);
        let eq19_e434_d_n1: f64 = ((nv7 - 0.0) * var_sigrat_d_dn1);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * var_sigrat_d_dn2);
        let eq19_e434_d_n3: f64 = ((nv7 - 0.0) * var_sigrat_d_dn3);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * var_sigrat_d_dn4);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * var_sigrat_d_dn5);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * var_sigrat_d_dn6);
        let eq19_e434_d_n7: f64 = (var_sigrat_d + ((nv7 - 0.0) * var_sigrat_d_dn7));
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * var_sigrat_d_dn8);
        let eq19_e434_d_n9: f64 = ((nv7 - 0.0) * var_sigrat_d_dn9);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * var_sigrat_d_dn10);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * var_sigrat_d_dn11);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * var_sigrat_d_dn12);
        let eq19_e434_d_b0: f64 = ((nv7 - 0.0) * var_sigrat_d_db0);
        let eq19_e434_d_b1: f64 = ((nv7 - 0.0) * var_sigrat_d_db1);
        let eq19_e434_d_b2: f64 = ((nv7 - 0.0) * var_sigrat_d_db2);
        let eq19_e434_d_b3: f64 = ((nv7 - 0.0) * var_sigrat_d_db3);
        let eq19_e434_d_b4: f64 = ((nv7 - 0.0) * var_sigrat_d_db4);
        let eq19_e434_d_b5: f64 = ((nv7 - 0.0) * var_sigrat_d_db5);
        let eq19_e434_d_b6: f64 = ((nv7 - 0.0) * var_sigrat_d_db6);
        let eq19_e434_d_b7: f64 = ((nv7 - 0.0) * var_sigrat_d_db7);
        let eq19_e435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e434);
        let eq19_value: f64 = eq19_e435;
        let eq19_node_derivatives: [f64; 13] = [(eq19_e434_d_n0 * ddt_scale), (eq19_e434_d_n1 * ddt_scale), (eq19_e434_d_n2 * ddt_scale), (eq19_e434_d_n3 * ddt_scale), (eq19_e434_d_n4 * ddt_scale), (eq19_e434_d_n5 * ddt_scale), (eq19_e434_d_n6 * ddt_scale), (eq19_e434_d_n7 * ddt_scale), (eq19_e434_d_n8 * ddt_scale), (eq19_e434_d_n9 * ddt_scale), (eq19_e434_d_n10 * ddt_scale), (eq19_e434_d_n11 * ddt_scale), (eq19_e434_d_n12 * ddt_scale)];
        let eq19_branch_derivatives: [f64; 8] = [(eq19_e434_d_b0 * ddt_scale), (eq19_e434_d_b1 * ddt_scale), (eq19_e434_d_b2 * ddt_scale), (eq19_e434_d_b3 * ddt_scale), (eq19_e434_d_b4 * ddt_scale), (eq19_e434_d_b5 * ddt_scale), (eq19_e434_d_b6 * ddt_scale), (eq19_e434_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_d_b4, eq28_e498_d_b5, eq28_e498_d_b6, eq28_e498_d_b7,) = {
    if (var_guard443 != 0.0) {
        let eq28_e487: f64 = (-var_rpower);
        let eq28_e490: f64 = (var_cthe * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (var_cthe_dn0 * (nv4 - 0.0));
        let eq28_e490_d_n1: f64 = (var_cthe_dn1 * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (var_cthe_dn2 * (nv4 - 0.0));
        let eq28_e490_d_n3: f64 = (var_cthe_dn3 * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((var_cthe_dn4 * (nv4 - 0.0)) + var_cthe);
        let eq28_e490_d_n5: f64 = (var_cthe_dn5 * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (var_cthe_dn6 * (nv4 - 0.0));
        let eq28_e490_d_n7: f64 = (var_cthe_dn7 * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (var_cthe_dn8 * (nv4 - 0.0));
        let eq28_e490_d_n9: f64 = (var_cthe_dn9 * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (var_cthe_dn10 * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (var_cthe_dn11 * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (var_cthe_dn12 * (nv4 - 0.0));
        let eq28_e490_d_b0: f64 = (var_cthe_db0 * (nv4 - 0.0));
        let eq28_e490_d_b1: f64 = (var_cthe_db1 * (nv4 - 0.0));
        let eq28_e490_d_b2: f64 = (var_cthe_db2 * (nv4 - 0.0));
        let eq28_e490_d_b3: f64 = (var_cthe_db3 * (nv4 - 0.0));
        let eq28_e490_d_b4: f64 = (var_cthe_db4 * (nv4 - 0.0));
        let eq28_e490_d_b5: f64 = (var_cthe_db5 * (nv4 - 0.0));
        let eq28_e490_d_b6: f64 = (var_cthe_db6 * (nv4 - 0.0));
        let eq28_e490_d_b7: f64 = (var_cthe_db7 * (nv4 - 0.0));
        let eq28_e491: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e490);
        let eq28_e492: f64 = (eq28_e487 + eq28_e491);
        let eq28_e492_d_n0: f64 = ((-var_rpower_dn0) + (eq28_e490_d_n0 * ddt_scale));
        let eq28_e492_d_n1: f64 = ((-var_rpower_dn1) + (eq28_e490_d_n1 * ddt_scale));
        let eq28_e492_d_n2: f64 = ((-var_rpower_dn2) + (eq28_e490_d_n2 * ddt_scale));
        let eq28_e492_d_n3: f64 = ((-var_rpower_dn3) + (eq28_e490_d_n3 * ddt_scale));
        let eq28_e492_d_n4: f64 = ((-var_rpower_dn4) + (eq28_e490_d_n4 * ddt_scale));
        let eq28_e492_d_n5: f64 = ((-var_rpower_dn5) + (eq28_e490_d_n5 * ddt_scale));
        let eq28_e492_d_n6: f64 = ((-var_rpower_dn6) + (eq28_e490_d_n6 * ddt_scale));
        let eq28_e492_d_n7: f64 = ((-var_rpower_dn7) + (eq28_e490_d_n7 * ddt_scale));
        let eq28_e492_d_n8: f64 = ((-var_rpower_dn8) + (eq28_e490_d_n8 * ddt_scale));
        let eq28_e492_d_n9: f64 = ((-var_rpower_dn9) + (eq28_e490_d_n9 * ddt_scale));
        let eq28_e492_d_n10: f64 = ((-var_rpower_dn10) + (eq28_e490_d_n10 * ddt_scale));
        let eq28_e492_d_n11: f64 = ((-var_rpower_dn11) + (eq28_e490_d_n11 * ddt_scale));
        let eq28_e492_d_n12: f64 = ((-var_rpower_dn12) + (eq28_e490_d_n12 * ddt_scale));
        let eq28_e492_d_b0: f64 = ((-var_rpower_db0) + (eq28_e490_d_b0 * ddt_scale));
        let eq28_e492_d_b1: f64 = ((-var_rpower_db1) + (eq28_e490_d_b1 * ddt_scale));
        let eq28_e492_d_b2: f64 = ((-var_rpower_db2) + (eq28_e490_d_b2 * ddt_scale));
        let eq28_e492_d_b3: f64 = ((-var_rpower_db3) + (eq28_e490_d_b3 * ddt_scale));
        let eq28_e492_d_b4: f64 = ((-var_rpower_db4) + (eq28_e490_d_b4 * ddt_scale));
        let eq28_e492_d_b5: f64 = ((-var_rpower_db5) + (eq28_e490_d_b5 * ddt_scale));
        let eq28_e492_d_b6: f64 = ((-var_rpower_db6) + (eq28_e490_d_b6 * ddt_scale));
        let eq28_e492_d_b7: f64 = ((-var_rpower_db7) + (eq28_e490_d_b7 * ddt_scale));
        let eq28_e495: f64 = ((nv4 - 0.0) * var_gth);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * var_gth_dn0);
        let eq28_e495_d_n1: f64 = ((nv4 - 0.0) * var_gth_dn1);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * var_gth_dn2);
        let eq28_e495_d_n3: f64 = ((nv4 - 0.0) * var_gth_dn3);
        let eq28_e495_d_n4: f64 = (var_gth + ((nv4 - 0.0) * var_gth_dn4));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * var_gth_dn5);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * var_gth_dn6);
        let eq28_e495_d_n7: f64 = ((nv4 - 0.0) * var_gth_dn7);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * var_gth_dn8);
        let eq28_e495_d_n9: f64 = ((nv4 - 0.0) * var_gth_dn9);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * var_gth_dn10);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * var_gth_dn11);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * var_gth_dn12);
        let eq28_e495_d_b0: f64 = ((nv4 - 0.0) * var_gth_db0);
        let eq28_e495_d_b1: f64 = ((nv4 - 0.0) * var_gth_db1);
        let eq28_e495_d_b2: f64 = ((nv4 - 0.0) * var_gth_db2);
        let eq28_e495_d_b3: f64 = ((nv4 - 0.0) * var_gth_db3);
        let eq28_e495_d_b4: f64 = ((nv4 - 0.0) * var_gth_db4);
        let eq28_e495_d_b5: f64 = ((nv4 - 0.0) * var_gth_db5);
        let eq28_e495_d_b6: f64 = ((nv4 - 0.0) * var_gth_db6);
        let eq28_e495_d_b7: f64 = ((nv4 - 0.0) * var_gth_db7);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n1: f64 = (eq28_e492_d_n1 + eq28_e495_d_n1);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n3: f64 = (eq28_e492_d_n3 + eq28_e495_d_n3);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n7: f64 = (eq28_e492_d_n7 + eq28_e495_d_n7);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n9: f64 = (eq28_e492_d_n9 + eq28_e495_d_n9);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_d_b0: f64 = (eq28_e492_d_b0 + eq28_e495_d_b0);
        let eq28_e496_d_b1: f64 = (eq28_e492_d_b1 + eq28_e495_d_b1);
        let eq28_e496_d_b2: f64 = (eq28_e492_d_b2 + eq28_e495_d_b2);
        let eq28_e496_d_b3: f64 = (eq28_e492_d_b3 + eq28_e495_d_b3);
        let eq28_e496_d_b4: f64 = (eq28_e492_d_b4 + eq28_e495_d_b4);
        let eq28_e496_d_b5: f64 = (eq28_e492_d_b5 + eq28_e495_d_b5);
        let eq28_e496_d_b6: f64 = (eq28_e492_d_b6 + eq28_e495_d_b6);
        let eq28_e496_d_b7: f64 = (eq28_e492_d_b7 + eq28_e495_d_b7);
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3, eq28_e496_d_b4, eq28_e496_d_b5, eq28_e496_d_b6, eq28_e496_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e498;
        let eq28_node_derivatives: [f64; 13] = [eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12];
        let eq28_branch_derivatives: [f64; 8] = [eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_d_b4, eq28_e498_d_b5, eq28_e498_d_b6, eq28_e498_d_b7];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_d_b4, eq30_e512_d_b5, eq30_e512_d_b6, eq30_e512_d_b7,) = {
    if (var_guard444 != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq30_e508);
        let eq30_e510: f64 = (var_iqh_nqs + eq30_e509);
        let eq30_e510_d_n10: f64 = (var_iqh_nqs_dn10 + (1e-9 * ddt_scale));
        (eq30_e510, var_iqh_nqs_dn0, var_iqh_nqs_dn1, var_iqh_nqs_dn2, var_iqh_nqs_dn3, var_iqh_nqs_dn4, var_iqh_nqs_dn5, var_iqh_nqs_dn6, var_iqh_nqs_dn7, var_iqh_nqs_dn8, var_iqh_nqs_dn9, eq30_e510_d_n10, var_iqh_nqs_dn11, var_iqh_nqs_dn12, var_iqh_nqs_db0, var_iqh_nqs_db1, var_iqh_nqs_db2, var_iqh_nqs_db3, var_iqh_nqs_db4, var_iqh_nqs_db5, var_iqh_nqs_db6, var_iqh_nqs_db7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e512;
        let eq30_node_derivatives: [f64; 13] = [eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12];
        let eq30_branch_derivatives: [f64; 8] = [eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_d_b4, eq30_e512_d_b5, eq30_e512_d_b6, eq30_e512_d_b7];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_d_b4, eq32_e526_d_b5, eq32_e526_d_b6, eq32_e526_d_b7,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq32_e522);
        let eq32_e524: f64 = (var_iqi_nqs + eq32_e523);
        let eq32_e524_d_n8: f64 = (var_iqi_nqs_dn8 + (1e-9 * ddt_scale));
        (eq32_e524, var_iqi_nqs_dn0, var_iqi_nqs_dn1, var_iqi_nqs_dn2, var_iqi_nqs_dn3, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn7, eq32_e524_d_n8, var_iqi_nqs_dn9, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_db0, var_iqi_nqs_db1, var_iqi_nqs_db2, var_iqi_nqs_db3, var_iqi_nqs_db4, var_iqi_nqs_db5, var_iqi_nqs_db6, var_iqi_nqs_db7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e526;
        let eq32_node_derivatives: [f64; 13] = [eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12];
        let eq32_branch_derivatives: [f64; 8] = [eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_d_b4, eq32_e526_d_b5, eq32_e526_d_b6, eq32_e526_d_b7];
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_d_b4, eq33_e535_d_b5, eq33_e535_d_b6, eq33_e535_d_b7,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq33_e531);
        let eq33_e533: f64 = (var_iqb_nqs + eq33_e532);
        let eq33_e533_d_n9: f64 = (var_iqb_nqs_dn9 + (1e-9 * ddt_scale));
        (eq33_e533, var_iqb_nqs_dn0, var_iqb_nqs_dn1, var_iqb_nqs_dn2, var_iqb_nqs_dn3, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn8, eq33_e533_d_n9, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_db0, var_iqb_nqs_db1, var_iqb_nqs_db2, var_iqb_nqs_db3, var_iqb_nqs_db4, var_iqb_nqs_db5, var_iqb_nqs_db6, var_iqb_nqs_db7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e535;
        let eq33_node_derivatives: [f64; 13] = [eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12];
        let eq33_branch_derivatives: [f64; 8] = [eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_d_b4, eq33_e535_d_b5, eq33_e535_d_b6, eq33_e535_d_b7];
        stamper.stamp_current_dense_local(
            Some(9),
            None,
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e387: f64 = (s.v[561] + s.v[554]);
        let eq10_e387_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);
        let eq10_e387_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);
        let eq10_e387_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);
        let eq10_e387_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);
        let eq10_e387_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);
        let eq10_e387_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);
        let eq10_e387_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);
        let eq10_e387_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);
        let eq10_e387_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);
        let eq10_e387_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);
        let eq10_e387_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);
        let eq10_e387_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);
        let eq10_e387_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);
        let eq10_e387_d_b0: f64 = (s.db[561][0] + s.db[554][0]);
        let eq10_e387_d_b1: f64 = (s.db[561][1] + s.db[554][1]);
        let eq10_e387_d_b2: f64 = (s.db[561][2] + s.db[554][2]);
        let eq10_e387_d_b3: f64 = (s.db[561][3] + s.db[554][3]);
        let eq10_e387_d_b4: f64 = (s.db[561][4] + s.db[554][4]);
        let eq10_e387_d_b5: f64 = (s.db[561][5] + s.db[554][5]);
        let eq10_e387_d_b6: f64 = (s.db[561][6] + s.db[554][6]);
        let eq10_e387_d_b7: f64 = (s.db[561][7] + s.db[554][7]);
        let eq10_e388_q: f64 = eq10_e387;
        let eq10_e389: f64 = (p.p33 * eq10_e387);
        let eq10_e389_d_n0: f64 = (p.p33 * eq10_e387_d_n0);
        let eq10_e389_d_n1: f64 = (p.p33 * eq10_e387_d_n1);
        let eq10_e389_d_n2: f64 = (p.p33 * eq10_e387_d_n2);
        let eq10_e389_d_n3: f64 = (p.p33 * eq10_e387_d_n3);
        let eq10_e389_d_n4: f64 = (p.p33 * eq10_e387_d_n4);
        let eq10_e389_d_n5: f64 = (p.p33 * eq10_e387_d_n5);
        let eq10_e389_d_n6: f64 = (p.p33 * eq10_e387_d_n6);
        let eq10_e389_d_n7: f64 = (p.p33 * eq10_e387_d_n7);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * eq10_e387_d_n9);
        let eq10_e389_d_n10: f64 = (p.p33 * eq10_e387_d_n10);
        let eq10_e389_d_n11: f64 = (p.p33 * eq10_e387_d_n11);
        let eq10_e389_d_n12: f64 = (p.p33 * eq10_e387_d_n12);
        let eq10_e389_d_b0: f64 = (p.p33 * eq10_e387_d_b0);
        let eq10_e389_d_b1: f64 = (p.p33 * eq10_e387_d_b1);
        let eq10_e389_d_b2: f64 = (p.p33 * eq10_e387_d_b2);
        let eq10_e389_d_b3: f64 = (p.p33 * eq10_e387_d_b3);
        let eq10_e389_d_b4: f64 = (p.p33 * eq10_e387_d_b4);
        let eq10_e389_d_b5: f64 = (p.p33 * eq10_e387_d_b5);
        let eq10_e389_d_b6: f64 = (p.p33 * eq10_e387_d_b6);
        let eq10_e389_d_b7: f64 = (p.p33 * eq10_e387_d_b7);
        let eq10_e389_q: f64 = (p.p33 * eq10_e388_q);
        let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e389_d_n0, eq10_e389_d_n1, eq10_e389_d_n2, eq10_e389_d_n3, eq10_e389_d_n4, eq10_e389_d_n5, eq10_e389_d_n6, eq10_e389_d_n7, eq10_e389_d_n8, eq10_e389_d_n9, eq10_e389_d_n10, eq10_e389_d_n11, eq10_e389_d_n12];
        let eq10_reactive_branch_derivatives: [f64; 8] = [eq10_e389_d_b0, eq10_e389_d_b1, eq10_e389_d_b2, eq10_e389_d_b3, eq10_e389_d_b4, eq10_e389_d_b5, eq10_e389_d_b6, eq10_e389_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e393: f64 = (s.v[93] + s.v[552]);
        let eq11_e393_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);
        let eq11_e393_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);
        let eq11_e393_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);
        let eq11_e393_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);
        let eq11_e393_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);
        let eq11_e393_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);
        let eq11_e393_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);
        let eq11_e393_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);
        let eq11_e393_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);
        let eq11_e393_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);
        let eq11_e393_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);
        let eq11_e393_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);
        let eq11_e393_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);
        let eq11_e393_d_b0: f64 = (s.db[93][0] + s.db[552][0]);
        let eq11_e393_d_b1: f64 = (s.db[93][1] + s.db[552][1]);
        let eq11_e393_d_b2: f64 = (s.db[93][2] + s.db[552][2]);
        let eq11_e393_d_b3: f64 = (s.db[93][3] + s.db[552][3]);
        let eq11_e393_d_b4: f64 = (s.db[93][4] + s.db[552][4]);
        let eq11_e393_d_b5: f64 = (s.db[93][5] + s.db[552][5]);
        let eq11_e393_d_b6: f64 = (s.db[93][6] + s.db[552][6]);
        let eq11_e393_d_b7: f64 = (s.db[93][7] + s.db[552][7]);
        let eq11_e394_q: f64 = eq11_e393;
        let eq11_e395: f64 = (p.p33 * eq11_e393);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_d_n1: f64 = (p.p33 * eq11_e393_d_n1);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_d_n3: f64 = (p.p33 * eq11_e393_d_n3);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_d_n7: f64 = (p.p33 * eq11_e393_d_n7);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_d_n9: f64 = (p.p33 * eq11_e393_d_n9);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_d_b0: f64 = (p.p33 * eq11_e393_d_b0);
        let eq11_e395_d_b1: f64 = (p.p33 * eq11_e393_d_b1);
        let eq11_e395_d_b2: f64 = (p.p33 * eq11_e393_d_b2);
        let eq11_e395_d_b3: f64 = (p.p33 * eq11_e393_d_b3);
        let eq11_e395_d_b4: f64 = (p.p33 * eq11_e393_d_b4);
        let eq11_e395_d_b5: f64 = (p.p33 * eq11_e393_d_b5);
        let eq11_e395_d_b6: f64 = (p.p33 * eq11_e393_d_b6);
        let eq11_e395_d_b7: f64 = (p.p33 * eq11_e393_d_b7);
        let eq11_e395_q: f64 = (p.p33 * eq11_e394_q);
        let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e395_d_n0, eq11_e395_d_n1, eq11_e395_d_n2, eq11_e395_d_n3, eq11_e395_d_n4, eq11_e395_d_n5, eq11_e395_d_n6, eq11_e395_d_n7, eq11_e395_d_n8, eq11_e395_d_n9, eq11_e395_d_n10, eq11_e395_d_n11, eq11_e395_d_n12];
        let eq11_reactive_branch_derivatives: [f64; 8] = [eq11_e395_d_b0, eq11_e395_d_b1, eq11_e395_d_b2, eq11_e395_d_b3, eq11_e395_d_b4, eq11_e395_d_b5, eq11_e395_d_b6, eq11_e395_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e399: f64 = (s.v[90] + s.v[548]);
        let eq12_e399_d_n0: f64 = (s.dn[90][0] + s.dn[548][0]);
        let eq12_e399_d_n1: f64 = (s.dn[90][1] + s.dn[548][1]);
        let eq12_e399_d_n2: f64 = (s.dn[90][2] + s.dn[548][2]);
        let eq12_e399_d_n3: f64 = (s.dn[90][3] + s.dn[548][3]);
        let eq12_e399_d_n4: f64 = (s.dn[90][4] + s.dn[548][4]);
        let eq12_e399_d_n5: f64 = (s.dn[90][5] + s.dn[548][5]);
        let eq12_e399_d_n6: f64 = (s.dn[90][6] + s.dn[548][6]);
        let eq12_e399_d_n7: f64 = (s.dn[90][7] + s.dn[548][7]);
        let eq12_e399_d_n8: f64 = (s.dn[90][8] + s.dn[548][8]);
        let eq12_e399_d_n9: f64 = (s.dn[90][9] + s.dn[548][9]);
        let eq12_e399_d_n10: f64 = (s.dn[90][10] + s.dn[548][10]);
        let eq12_e399_d_n11: f64 = (s.dn[90][11] + s.dn[548][11]);
        let eq12_e399_d_n12: f64 = (s.dn[90][12] + s.dn[548][12]);
        let eq12_e399_d_b0: f64 = (s.db[90][0] + s.db[548][0]);
        let eq12_e399_d_b1: f64 = (s.db[90][1] + s.db[548][1]);
        let eq12_e399_d_b2: f64 = (s.db[90][2] + s.db[548][2]);
        let eq12_e399_d_b3: f64 = (s.db[90][3] + s.db[548][3]);
        let eq12_e399_d_b4: f64 = (s.db[90][4] + s.db[548][4]);
        let eq12_e399_d_b5: f64 = (s.db[90][5] + s.db[548][5]);
        let eq12_e399_d_b6: f64 = (s.db[90][6] + s.db[548][6]);
        let eq12_e399_d_b7: f64 = (s.db[90][7] + s.db[548][7]);
        let eq12_e400_q: f64 = eq12_e399;
        let eq12_e401: f64 = (p.p33 * eq12_e399);
        let eq12_e401_d_n0: f64 = (p.p33 * eq12_e399_d_n0);
        let eq12_e401_d_n1: f64 = (p.p33 * eq12_e399_d_n1);
        let eq12_e401_d_n2: f64 = (p.p33 * eq12_e399_d_n2);
        let eq12_e401_d_n3: f64 = (p.p33 * eq12_e399_d_n3);
        let eq12_e401_d_n4: f64 = (p.p33 * eq12_e399_d_n4);
        let eq12_e401_d_n5: f64 = (p.p33 * eq12_e399_d_n5);
        let eq12_e401_d_n6: f64 = (p.p33 * eq12_e399_d_n6);
        let eq12_e401_d_n7: f64 = (p.p33 * eq12_e399_d_n7);
        let eq12_e401_d_n8: f64 = (p.p33 * eq12_e399_d_n8);
        let eq12_e401_d_n9: f64 = (p.p33 * eq12_e399_d_n9);
        let eq12_e401_d_n10: f64 = (p.p33 * eq12_e399_d_n10);
        let eq12_e401_d_n11: f64 = (p.p33 * eq12_e399_d_n11);
        let eq12_e401_d_n12: f64 = (p.p33 * eq12_e399_d_n12);
        let eq12_e401_d_b0: f64 = (p.p33 * eq12_e399_d_b0);
        let eq12_e401_d_b1: f64 = (p.p33 * eq12_e399_d_b1);
        let eq12_e401_d_b2: f64 = (p.p33 * eq12_e399_d_b2);
        let eq12_e401_d_b3: f64 = (p.p33 * eq12_e399_d_b3);
        let eq12_e401_d_b4: f64 = (p.p33 * eq12_e399_d_b4);
        let eq12_e401_d_b5: f64 = (p.p33 * eq12_e399_d_b5);
        let eq12_e401_d_b6: f64 = (p.p33 * eq12_e399_d_b6);
        let eq12_e401_d_b7: f64 = (p.p33 * eq12_e399_d_b7);
        let eq12_e401_q: f64 = (p.p33 * eq12_e400_q);
        let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e401_d_n0, eq12_e401_d_n1, eq12_e401_d_n2, eq12_e401_d_n3, eq12_e401_d_n4, eq12_e401_d_n5, eq12_e401_d_n6, eq12_e401_d_n7, eq12_e401_d_n8, eq12_e401_d_n9, eq12_e401_d_n10, eq12_e401_d_n11, eq12_e401_d_n12];
        let eq12_reactive_branch_derivatives: [f64; 8] = [eq12_e401_d_b0, eq12_e401_d_b1, eq12_e401_d_b2, eq12_e401_d_b3, eq12_e401_d_b4, eq12_e401_d_b5, eq12_e401_d_b6, eq12_e401_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * s.v[611]);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * s.dn[611][0]);
        let eq18_e430_d_n1: f64 = ((nv7 - 0.0) * s.dn[611][1]);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * s.dn[611][2]);
        let eq18_e430_d_n3: f64 = ((nv7 - 0.0) * s.dn[611][3]);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * s.dn[611][4]);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * s.dn[611][5]);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * s.dn[611][6]);
        let eq18_e430_d_n7: f64 = (s.v[611] + ((nv7 - 0.0) * s.dn[611][7]));
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * s.dn[611][8]);
        let eq18_e430_d_n9: f64 = ((nv7 - 0.0) * s.dn[611][9]);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * s.dn[611][10]);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * s.dn[611][11]);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * s.dn[611][12]);
        let eq18_e430_d_b0: f64 = ((nv7 - 0.0) * s.db[611][0]);
        let eq18_e430_d_b1: f64 = ((nv7 - 0.0) * s.db[611][1]);
        let eq18_e430_d_b2: f64 = ((nv7 - 0.0) * s.db[611][2]);
        let eq18_e430_d_b3: f64 = ((nv7 - 0.0) * s.db[611][3]);
        let eq18_e430_d_b4: f64 = ((nv7 - 0.0) * s.db[611][4]);
        let eq18_e430_d_b5: f64 = ((nv7 - 0.0) * s.db[611][5]);
        let eq18_e430_d_b6: f64 = ((nv7 - 0.0) * s.db[611][6]);
        let eq18_e430_d_b7: f64 = ((nv7 - 0.0) * s.db[611][7]);
        let eq18_e431_q: f64 = eq18_e430;
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e430_d_n0, eq18_e430_d_n1, eq18_e430_d_n2, eq18_e430_d_n3, eq18_e430_d_n4, eq18_e430_d_n5, eq18_e430_d_n6, eq18_e430_d_n7, eq18_e430_d_n8, eq18_e430_d_n9, eq18_e430_d_n10, eq18_e430_d_n11, eq18_e430_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 8] = [eq18_e430_d_b0, eq18_e430_d_b1, eq18_e430_d_b2, eq18_e430_d_b3, eq18_e430_d_b4, eq18_e430_d_b5, eq18_e430_d_b6, eq18_e430_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * s.v[612]);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * s.dn[612][0]);
        let eq19_e434_d_n1: f64 = ((nv7 - 0.0) * s.dn[612][1]);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * s.dn[612][2]);
        let eq19_e434_d_n3: f64 = ((nv7 - 0.0) * s.dn[612][3]);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * s.dn[612][4]);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * s.dn[612][5]);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * s.dn[612][6]);
        let eq19_e434_d_n7: f64 = (s.v[612] + ((nv7 - 0.0) * s.dn[612][7]));
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * s.dn[612][8]);
        let eq19_e434_d_n9: f64 = ((nv7 - 0.0) * s.dn[612][9]);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * s.dn[612][10]);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * s.dn[612][11]);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * s.dn[612][12]);
        let eq19_e434_d_b0: f64 = ((nv7 - 0.0) * s.db[612][0]);
        let eq19_e434_d_b1: f64 = ((nv7 - 0.0) * s.db[612][1]);
        let eq19_e434_d_b2: f64 = ((nv7 - 0.0) * s.db[612][2]);
        let eq19_e434_d_b3: f64 = ((nv7 - 0.0) * s.db[612][3]);
        let eq19_e434_d_b4: f64 = ((nv7 - 0.0) * s.db[612][4]);
        let eq19_e434_d_b5: f64 = ((nv7 - 0.0) * s.db[612][5]);
        let eq19_e434_d_b6: f64 = ((nv7 - 0.0) * s.db[612][6]);
        let eq19_e434_d_b7: f64 = ((nv7 - 0.0) * s.db[612][7]);
        let eq19_e435_q: f64 = eq19_e434;
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e434_d_n0, eq19_e434_d_n1, eq19_e434_d_n2, eq19_e434_d_n3, eq19_e434_d_n4, eq19_e434_d_n5, eq19_e434_d_n6, eq19_e434_d_n7, eq19_e434_d_n8, eq19_e434_d_n9, eq19_e434_d_n10, eq19_e434_d_n11, eq19_e434_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 8] = [eq19_e434_d_b0, eq19_e434_d_b1, eq19_e434_d_b2, eq19_e434_d_b3, eq19_e434_d_b4, eq19_e434_d_b5, eq19_e434_d_b6, eq19_e434_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3, eq28_e498_d_b4, eq28_e498_d_b5, eq28_e498_d_b6, eq28_e498_d_b7, eq28_e498_q, eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12, eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3, eq28_e498_q_d_b4, eq28_e498_q_d_b5, eq28_e498_q_d_b6, eq28_e498_q_d_b7,) = {
    if s.b[1094] {
        let eq28_e487: f64 = (-s.v[547]);
        let eq28_e490: f64 = (s.v[516] * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));
        let eq28_e490_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));
        let eq28_e490_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);
        let eq28_e490_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));
        let eq28_e490_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));
        let eq28_e490_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));
        let eq28_e490_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));
        let eq28_e490_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));
        let eq28_e490_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));
        let eq28_e490_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));
        let eq28_e490_d_b4: f64 = (s.db[516][4] * (nv4 - 0.0));
        let eq28_e490_d_b5: f64 = (s.db[516][5] * (nv4 - 0.0));
        let eq28_e490_d_b6: f64 = (s.db[516][6] * (nv4 - 0.0));
        let eq28_e490_d_b7: f64 = (s.db[516][7] * (nv4 - 0.0));
        let eq28_e491_q: f64 = eq28_e490;
        let eq28_e492: f64 = (eq28_e487 + eq28_e490);
        let eq28_e492_d_n0: f64 = ((-s.dn[547][0]) + eq28_e490_d_n0);
        let eq28_e492_d_n1: f64 = ((-s.dn[547][1]) + eq28_e490_d_n1);
        let eq28_e492_d_n2: f64 = ((-s.dn[547][2]) + eq28_e490_d_n2);
        let eq28_e492_d_n3: f64 = ((-s.dn[547][3]) + eq28_e490_d_n3);
        let eq28_e492_d_n4: f64 = ((-s.dn[547][4]) + eq28_e490_d_n4);
        let eq28_e492_d_n5: f64 = ((-s.dn[547][5]) + eq28_e490_d_n5);
        let eq28_e492_d_n6: f64 = ((-s.dn[547][6]) + eq28_e490_d_n6);
        let eq28_e492_d_n7: f64 = ((-s.dn[547][7]) + eq28_e490_d_n7);
        let eq28_e492_d_n8: f64 = ((-s.dn[547][8]) + eq28_e490_d_n8);
        let eq28_e492_d_n9: f64 = ((-s.dn[547][9]) + eq28_e490_d_n9);
        let eq28_e492_d_n10: f64 = ((-s.dn[547][10]) + eq28_e490_d_n10);
        let eq28_e492_d_n11: f64 = ((-s.dn[547][11]) + eq28_e490_d_n11);
        let eq28_e492_d_n12: f64 = ((-s.dn[547][12]) + eq28_e490_d_n12);
        let eq28_e492_d_b0: f64 = ((-s.db[547][0]) + eq28_e490_d_b0);
        let eq28_e492_d_b1: f64 = ((-s.db[547][1]) + eq28_e490_d_b1);
        let eq28_e492_d_b2: f64 = ((-s.db[547][2]) + eq28_e490_d_b2);
        let eq28_e492_d_b3: f64 = ((-s.db[547][3]) + eq28_e490_d_b3);
        let eq28_e492_d_b4: f64 = ((-s.db[547][4]) + eq28_e490_d_b4);
        let eq28_e492_d_b5: f64 = ((-s.db[547][5]) + eq28_e490_d_b5);
        let eq28_e492_d_b6: f64 = ((-s.db[547][6]) + eq28_e490_d_b6);
        let eq28_e492_d_b7: f64 = ((-s.db[547][7]) + eq28_e490_d_b7);
        let eq28_e492_q: f64 = eq28_e491_q;
        let eq28_e495: f64 = ((nv4 - 0.0) * s.v[557]);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);
        let eq28_e495_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);
        let eq28_e495_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);
        let eq28_e495_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);
        let eq28_e495_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);
        let eq28_e495_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);
        let eq28_e495_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);
        let eq28_e495_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);
        let eq28_e495_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);
        let eq28_e495_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);
        let eq28_e495_d_b4: f64 = ((nv4 - 0.0) * s.db[557][4]);
        let eq28_e495_d_b5: f64 = ((nv4 - 0.0) * s.db[557][5]);
        let eq28_e495_d_b6: f64 = ((nv4 - 0.0) * s.db[557][6]);
        let eq28_e495_d_b7: f64 = ((nv4 - 0.0) * s.db[557][7]);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n1: f64 = (eq28_e492_d_n1 + eq28_e495_d_n1);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n3: f64 = (eq28_e492_d_n3 + eq28_e495_d_n3);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n7: f64 = (eq28_e492_d_n7 + eq28_e495_d_n7);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n9: f64 = (eq28_e492_d_n9 + eq28_e495_d_n9);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_d_b0: f64 = (eq28_e492_d_b0 + eq28_e495_d_b0);
        let eq28_e496_d_b1: f64 = (eq28_e492_d_b1 + eq28_e495_d_b1);
        let eq28_e496_d_b2: f64 = (eq28_e492_d_b2 + eq28_e495_d_b2);
        let eq28_e496_d_b3: f64 = (eq28_e492_d_b3 + eq28_e495_d_b3);
        let eq28_e496_d_b4: f64 = (eq28_e492_d_b4 + eq28_e495_d_b4);
        let eq28_e496_d_b5: f64 = (eq28_e492_d_b5 + eq28_e495_d_b5);
        let eq28_e496_d_b6: f64 = (eq28_e492_d_b6 + eq28_e495_d_b6);
        let eq28_e496_d_b7: f64 = (eq28_e492_d_b7 + eq28_e495_d_b7);
        let eq28_e496_q: f64 = eq28_e492_q;
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3, eq28_e496_d_b4, eq28_e496_d_b5, eq28_e496_d_b6, eq28_e496_d_b7, eq28_e496_q, eq28_e490_d_n0, eq28_e490_d_n1, eq28_e490_d_n2, eq28_e490_d_n3, eq28_e490_d_n4, eq28_e490_d_n5, eq28_e490_d_n6, eq28_e490_d_n7, eq28_e490_d_n8, eq28_e490_d_n9, eq28_e490_d_n10, eq28_e490_d_n11, eq28_e490_d_n12, eq28_e490_d_b0, eq28_e490_d_b1, eq28_e490_d_b2, eq28_e490_d_b3, eq28_e490_d_b4, eq28_e490_d_b5, eq28_e490_d_b6, eq28_e490_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e498_q_d_n0, eq28_e498_q_d_n1, eq28_e498_q_d_n2, eq28_e498_q_d_n3, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n7, eq28_e498_q_d_n8, eq28_e498_q_d_n9, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 8] = [eq28_e498_q_d_b0, eq28_e498_q_d_b1, eq28_e498_q_d_b2, eq28_e498_q_d_b3, eq28_e498_q_d_b4, eq28_e498_q_d_b5, eq28_e498_q_d_b6, eq28_e498_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3, eq30_e512_d_b4, eq30_e512_d_b5, eq30_e512_d_b6, eq30_e512_d_b7, eq30_e512_q, eq30_e512_q_d_n10,) = {
    if s.b[1095] {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509_q: f64 = eq30_e508;
        let eq30_e510: f64 = (s.v[558] + eq30_e508);
        let eq30_e510_d_n10: f64 = (s.dn[558][10] + 1e-9);
        let eq30_e510_q: f64 = eq30_e509_q;
        (eq30_e510, s.dn[558][0], s.dn[558][1], s.dn[558][2], s.dn[558][3], s.dn[558][4], s.dn[558][5], s.dn[558][6], s.dn[558][7], s.dn[558][8], s.dn[558][9], eq30_e510_d_n10, s.dn[558][11], s.dn[558][12], s.db[558][0], s.db[558][1], s.db[558][2], s.db[558][3], s.db[558][4], s.db[558][5], s.db[558][6], s.db[558][7], eq30_e510_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e512_q_d_n10),
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3, eq32_e526_d_b4, eq32_e526_d_b5, eq32_e526_d_b6, eq32_e526_d_b7, eq32_e526_q, eq32_e526_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523_q: f64 = eq32_e522;
        let eq32_e524: f64 = (s.v[549] + eq32_e522);
        let eq32_e524_d_n8: f64 = (s.dn[549][8] + 1e-9);
        let eq32_e524_q: f64 = eq32_e523_q;
        (eq32_e524, s.dn[549][0], s.dn[549][1], s.dn[549][2], s.dn[549][3], s.dn[549][4], s.dn[549][5], s.dn[549][6], s.dn[549][7], eq32_e524_d_n8, s.dn[549][9], s.dn[549][10], s.dn[549][11], s.dn[549][12], s.db[549][0], s.db[549][1], s.db[549][2], s.db[549][3], s.db[549][4], s.db[549][5], s.db[549][6], s.db[549][7], eq32_e524_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * (eq32_e526_q_d_n8),
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3, eq33_e535_d_b4, eq33_e535_d_b5, eq33_e535_d_b6, eq33_e535_d_b7, eq33_e535_q, eq33_e535_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532_q: f64 = eq33_e531;
        let eq33_e533: f64 = (s.v[550] + eq33_e531);
        let eq33_e533_d_n9: f64 = (s.dn[550][9] + 1e-9);
        let eq33_e533_q: f64 = eq33_e532_q;
        (eq33_e533, s.dn[550][0], s.dn[550][1], s.dn[550][2], s.dn[550][3], s.dn[550][4], s.dn[550][5], s.dn[550][6], s.dn[550][7], s.dn[550][8], eq33_e533_d_n9, s.dn[550][10], s.dn[550][11], s.dn[550][12], s.db[550][0], s.db[550][1], s.db[550][2], s.db[550][3], s.db[550][4], s.db[550][5], s.db[550][6], s.db[550][7], eq33_e533_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (eq33_e535_q_d_n9),
        );
    }
}
