#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let mut assign56720_loop_guard: usize = 0;
        while {
            let assign56720_cond_e88274: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56720_cond_e88274 != 0.0
        } {
            assign56720_loop_guard += 1;
            assert!(assign56720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && (!s.b[2410])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);
            s.store_add_scaled_inputs3(2162, s.ad_value(972), 1.0, s.ad_value(972), (-0.5), s.ad_value(780), 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2409])) {
            s.copy_ad(2162, 349);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_add_ad_lhs(989, A::div(s.ad_value(989), s.ad_value(340)), 2162);
            s.store_mul_square_lhs(338, 2162, 2162);
            s.store_offset(334, 338, 0.0001);
            s.store_div(2163, 338, 334);
        }

        s.b[2415] = (p.p43 == (-1.0));
        s.v[2415] = if s.b[2415] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2415]) {
            s.store_scalar(2163, 0.0);
            s.copy_ad(989, 349);
        }

        s.b[2416] = (p.p43 == 2.0);
        s.v[2416] = if s.b[2416] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) && s.b[2416]) {
            s.copy_ad(989, 349);
            s.store_scalar(2162, 0.0);
            s.store_scalar(2163, 0.0);
            s.store_sub(335, 2142, 972);
            s.store_add_scaled_inputs3_offset(992, s.ad_value(335), 0.5, A::ln(A::cosh(s.ad_value(335))), 0.5, s.ad_value(972), 1.0, (((2.0) as f64).ln() * 0.5));
        }

        s.b[2417] = (p.p43 == 3.0);
        s.v[2417] = if s.b[2417] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) && (!s.b[2416])) && s.b[2417]) {
            s.store_add_ad_lhs(992, A::ln_one_plus_exp(A::sub(s.ad_value(2142), s.ad_value(972))), 972);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            s.store_div(335, 989, 992);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            s.store_mul(340, 338, 337);
            s.store_add_ad_lhs(989, A::div(s.ad_value(989), s.ad_value(340)), 2162);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_mul(2120, 990, 2129);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 2120, 343);
            s.store_offset_sqrt_ad(2164, A::offset(A::square(s.ad_value(989)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 2164, 688, 1.0);
            s.store_offset_mul(339, 2164, 689, 1.0);
        }

        s.b[2418] = param_given[408];
        s.v[2418] = if s.b[2418] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2418]) {
            s.store_div_scaled_value_by_product(2150, A::sub_from_scalar(p.p408, s.ad_value(2088)), 1.0, s.ad_value(965), s.ad_value(339), 100.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2418])) {
            s.store_div_scaled_inputs(2150, s.ad_value(2120), 9662367879.197212, s.ad_value(339), 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[2150] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 2150, p.p376);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add(s.ad_value(966), A::mul3_scaled_output(s.ad_value(968), s.ad_value(338), s.ad_value(252), 1e-10)), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div_scaled_value_offset_denominator(2111, s.ad_value(989), 1.0, s.ad_value(162), p.p401, 1.0);
            s.store_powf(781, 989, 2.0);
            s.store_scalar(782, ((0.01) as f64).powf(2.0));
            s.store_sub_ad(334, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div_scaled_value_offset_denominator(2165, s.ad_value(334), 1.0, s.ad_value(162), (-p.p402), 1.0);
            s.store_div_scaled_product_indices(335, 254, 2165, 1.0, 973, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_div(985, 254, 338);
            s.store_mul_offset_ad_rhs(2128, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2111), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2111), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p.p400), 1.0);
            s.store_scaled_mul(335, 990, 2128, 1.6021918e-19);
            s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p.p7);
            s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2111);
            s.store_mul3_affine_lhs(988, 336, 2151, p.p363, 0.0, 2163);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add_scaled_inputs3(135, s.ad_value(986), 1.0, s.ad_value(987), 1.0, s.ad_value(988), 1.0);
            s.copy_ad(790, 349);
        }

        s.b[2419] = (p.p283 != 0.0);
        s.v[2419] = if s.b[2419] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2419]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(2085), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2420] = (s.v[336] < 0.0);
        s.v[2420] = if s.b[2420] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2419]) && s.b[2420]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2419]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1435, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3(339, s.ad_value(2085), 1.0, s.ad_value(340), 1.0, s.ad_value(1434), -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1435), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2419])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2421] = (p.p287 != 0.0);
        s.v[2421] = if s.b[2421] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2421]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1435);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2421])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2422] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2422] = if s.b[2422] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2422]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.copy_ad(134, 135);
            s.store_add_scaled_inputs4(131, s.ad_value(2094), (-0.5), s.ad_value(2118), ((-1.0) * (-0.5)), s.ad_value(2095), (-0.5), s.ad_value(2119), (-(-0.5)));
            s.store_scaled_add(133, 2118, 2119, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 2118, 2119, (-0.5));
            s.store_neg(238, 2118);
            s.copy_ad(255, 2112);
        }

        s.b[2423] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2423] = if s.b[2423] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2423]) {
            s.store_scalar(78, 1.0);
        }

        s.b[2424] = (s.v[791] < s.v[86]);
        s.v[2424] = if s.b[2424] { 1.0 } else { 0.0 };

        if ((!s.b[1439]) && s.b[2424]) {
            s.store_scalar(347, (-1.0));
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 85, 1431);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2425] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2425] = if s.b[2425] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2424]) && s.b[2425]) {
            s.store_div_scaled_inputs(274, s.ad_value(277), 0.5, s.ad_value(278), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1439]) && s.b[2424]) && (!s.b[2425])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((!s.b[1439]) && s.b[2424]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if ((!s.b[1439]) && s.b[2424]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 1431, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 1431);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_ad_lhs(87, A::div(s.ad_value(335), s.ad_value(337)), 1431);
            s.copy_ad(91, 87);
            s.store_scalar(94, 0.0);
            s.store_sub(336, 85, 87);
            s.store_mul(131, 185, 336);
            s.store_scalar(133, 0.0);
            s.store_scalar(247, 0.0);
            s.store_scalar(169, 0.0);
            s.store_scalar(134, 0.0);
            s.store_scalar(127, 0.0);
            s.store_scalar(78, 1.0);
            s.store_scalar(946, 1.0);
        }

        s.b[2426] = (s.v[946] == 0.0);
        s.v[2426] = if s.b[2426] { 1.0 } else { 0.0 };

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1431))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_add_ad_rhs(89, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
        }

        s.b[2427] = (s.v[77] == 0.0);
        s.v[2427] = if s.b[2427] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2427]) {
            s.store_mul_sub_rhs(116, 154, 89, 1431);
        }

        s.b[2428] = (s.v[116] < 3.0);
        s.v[2428] = if s.b[2428] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && s.b[2428]) {
            s.store_mul_sub_rhs(333, 154, 85, 1431);
            s.store_div_from_scalar_ad(335, 1.0, A::mul_scaled_lhs(s.ad_value(154), (1.414213562373095 / 108.0), s.ad_value(212)));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && s.b[2428]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && s.b[2428]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 1431, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2429] = (s.v[791] <= s.v[118]);
        s.v[2429] = if s.b[2429] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && s.b[2429]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && (!s.b[2429])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 85, 85);
            s.store_add_ad_rhs(337, 154, A::div_from_scalar(2.0, s.ad_value(85)));
            s.store_div_ad_lhs(90, A::ln(s.ad_value(336)), 337);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && (!s.b[2429])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && (!s.b[2429])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(88, s.ad_value(90), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(332, 1431, (1e-12 / 2.0));
        }

        s.b[2430] = (s.v[88] < s.v[332]);
        s.v[2430] = if s.b[2430] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2430]) {
            s.copy_ad(88, 332);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.copy_ad(87, 88);
            s.copy_ad(92, 89);
            s.store_exp_mul(229, 154, 1431);
            s.store_mul(222, 210, 229);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign58510_loop_guard: usize = 0;
        while {
            let assign58510_cond_e90897: f64 = (s.v[421] + 1.0);
            let assign58510_cond_e90899: f64 = if (((!s.b[1439]) && s.b[2426]) && (s.v[97] <= assign58510_cond_e90897)) { 1.0 } else { 0.0 };
            assign58510_cond_e90899 != 0.0
        } {
            assign58510_loop_guard += 1;
            assert!(assign58510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_mul_sub_rhs(116, 154, 87, 1431);
            }
            s.b[2431] = (s.v[116] < 5.0);
            s.v[2431] = if s.b[2431] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2431]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(214, 222, 225, 225);
                s.store_mul_product3_rhs(215, 226, s.ad_value(222), s.ad_value(154), s.ad_value(225), 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_inputs2(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, s.ad_value(215), 1.0, s.ad_value(216), 2.0);
            }
            s.b[2432] = (s.v[116] < 60.0);
            s.v[2432] = if s.b[2432] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2431])) && s.b[2432]) {
                s.store_exp(227, 116);
                s.store_mul_offset_rhs(214, 222, 227, (-1.0));
                s.store_mul3_lhs(215, 222, 154, 227);
            }
            if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2431])) && (!s.b[2432])) {
                s.store_exp_mul(231, 154, 87);
                s.store_mul_sub_rhs(214, 210, 231, 229);
                s.store_mul3_lhs(215, 210, 154, 231);
            }
            if (((!s.b[1439]) && s.b[2426]) && (!s.b[2431])) {
                s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));
                s.store_div_scaled_inputs2(217, s.ad_value(154), 1.0, s.ad_value(215), 1.0, s.ad_value(216), 2.0);
            }
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));
                s.store_sub_from_scalar_scaled_mul(233, (-1.0), 212, 217, 1.0);
            }
            s.b[2433] = (s.v[79] == 1.0);
            s.v[2433] = if s.b[2433] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2433]) {
                s.store_scalar(944, 1.0);
            }
            s.b[2434] = (s.v[944] == 0.0);
            s.v[2434] = if s.b[2434] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2434]) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2434]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2435] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2435] = if s.b[2435] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2434]) && s.b[2435]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2434]) {
                s.store_add(87, 87, 236);
            }
            s.b[2436] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2436] = if s.b[2436] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2434]) && s.b[2436]) {
                s.store_scalar(79, 1.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && (s.v[944] != 0.0)) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_scalar(944, 0.0);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(97, 97, (-1.0));
        }

        s.b[2438] = (s.v[116] < 5.0);
        s.v[2438] = if s.b[2438] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2438]) {
            s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2438])) {
            s.store_scalar(347, 3.0);
            s.store_scalar(78, 0.0);
            s.store_offset(99, 116, (-1.0));
            s.store_sqrt(100, 99);
            s.store_mul(101, 99, 100);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(239, 209, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_offset_product3(238, s.ad_value(209), s.ad_value(214), s.ad_value(335), 1.0, 1e-25);
        }

        s.b[2439] = (s.v[116] < 5.0);
        s.v[2439] = if s.b[2439] { 1.0 } else { 0.0 };

        s.b[2440] = (s.v[116] < 3.0);
        s.v[2440] = if s.b[2440] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && s.b[2440]) {
            s.store_scalar(347, 1.0);
            s.store_scalar(78, 1.0);
            s.copy_ad(133, 238);
            s.copy_ad(131, 239);
            s.store_scalar(247, 0.5);
            s.store_scalar(169, 0.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && (!s.b[2440])) {
            s.store_scalar(347, 2.0);
            s.store_scalar(78, 0.0);
            s.store_scalar(335, (1.0 / (5.0 - 3.0)));
            s.store_mul_offset_rhs(332, 335, 116, (-3.0));
            s.store_mul3_ad_middle(207, A::square(s.ad_value(332)), 332, A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0));
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(127, 238, 186);
            s.copy_ad(349, 790);
            s.store_div_ad_rhs(336, 636, A::square(s.ad_value(185)));
            s.store_add_scaled_inputs3(334, s.ad_value(85), 1.0, s.ad_value(155), (-1.0), s.ad_value(1434), -1.0);
            s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);
            s.store_scaled_add(343, 335, 782, 0.5);
        }

        s.b[2441] = (s.v[343] < 0.0);
        s.v[2441] = if s.b[2441] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2441]) {
            s.store_scalar(343, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1439]) && s.b[2426]) && s.b[2441]) {
            s.store_scalar(332, 0.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(336), 1.0, s.ad_value(337)));
            s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);
            s.store_scaled_add(344, 344, 782, 0.5);
        }

        s.b[2442] = (s.v[344] < 0.0);
        s.v[2442] = if s.b[2442] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2442]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
            s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1431), s.ad_value(790))));
        }

        s.b[2443] = (s.v[790] < 0.0);
        s.v[2443] = if s.b[2443] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2443]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(91, 87);
            s.store_scalar(947, 1.0);
        }

        s.b[2444] = (s.v[947] == 0.0);
        s.v[2444] = if s.b[2444] { 1.0 } else { 0.0 };

        s.b[2445] = (s.v[77] == 0.0);
        s.v[2445] = if s.b[2445] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }

        s.b[2446] = (((1.0 + 0.3) * s.v[96]) > 0.03);
        s.v[2446] = if s.b[2446] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && s.b[2446]) {
            s.store_offset_sub_scaled_inputs(781, s.ad_value(96), (1.0 + 0.3), s.ad_value(790), 1.0, (-0.03));
            s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && s.b[2446]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && s.b[2446]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(95, s.ad_value(96), (1.0 + 0.3), s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && (!s.b[2446])) {
            s.store_scale(95, 96, (1.0 + 0.3));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }

        s.b[2447] = (s.v[95] < 0.0);
        s.v[2447] = if s.b[2447] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2447]) {
            s.store_scalar(95, 0.0);
        }

        s.b[2448] = (s.v[95] > s.v[790]);
        s.v[2448] = if s.b[2448] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && (!s.b[2447])) && s.b[2448]) {
            s.copy_ad(95, 790);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2444]) {
            s.copy_ad(94, 95);
            s.store_add(91, 87, 94);
            s.store_scalar(79, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && (s.v[947] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_scalar(98, 1.0);
        }

        let mut assign59350_loop_guard: usize = 0;
        while {
            let assign59350_cond_e92313: f64 = (40.0 + 1.0);
            let assign59350_cond_e92315: f64 = if (((!s.b[1439]) && s.b[2426]) && (s.v[98] <= assign59350_cond_e92313)) { 1.0 } else { 0.0 };
            assign59350_cond_e92315 != 0.0
        } {
            assign59350_loop_guard += 1;
            assert!(assign59350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_mul_sub_rhs(116, 154, 91, 1431);
            }
            s.b[2449] = (s.v[116] < 5.0);
            s.v[2449] = if s.b[2449] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2449]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul(222, 210, 230);
                s.store_mul3_lhs(218, 222, 225, 225);
                s.store_mul_product3_rhs(219, 226, s.ad_value(222), s.ad_value(154), s.ad_value(225), 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(220, 223, 218);
                s.store_div_scaled_inputs2(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, s.ad_value(219), 1.0, s.ad_value(220), 2.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && (!s.b[2449])) {
                s.store_mul_sub_rhs(117, 154, 91, 790);
                s.store_exp(228, 117);
                s.store_mul_sub_rhs(218, 210, 228, 230);
                s.store_mul3_lhs(219, 210, 154, 228);
                s.store_offset(102, 116, (-1.0));
                s.store_sqrt_add(220, 102, 218);
                s.store_div_scaled_inputs2(221, s.ad_value(154), 1.0, s.ad_value(219), 1.0, s.ad_value(220), 2.0);
            }
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));
                s.store_sub_from_scalar_scaled_mul(235, (-1.0), 212, 221, 1.0);
            }
            s.b[2450] = (s.v[79] == 1.0);
            s.v[2450] = if s.b[2450] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2450]) {
                s.store_scalar(945, 1.0);
            }
            s.b[2451] = (s.v[945] == 0.0);
            s.v[2451] = if s.b[2451] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2451]) {
                s.store_div_scaled_inputs(237, s.ad_value(234), -1.0, s.ad_value(235), 1.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2451]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2452] = (((s.v[237]) as f64).abs() > s.v[93]);
            s.v[2452] = if s.b[2452] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2451]) && s.b[2452]) {
                s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2451]) {
                s.store_add(91, 91, 237);
            }
            s.b[2453] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));
            s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2451]) && s.b[2453]) {
                s.store_scalar(79, 1.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && (s.v[945] != 0.0)) {
                s.store_scalar(98, (40.0 + 1.0));
            }
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_scalar(945, 0.0);
                s.store_offset(98, 98, 1.0);
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(98, 98, (-1.0));
        }

        s.b[2455] = (s.v[116] < 5.0);
        s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2455]) {
            s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2455])) {
            s.store_offset(102, 116, (-1.0));
            s.store_sqrt(103, 102);
            s.store_mul(104, 102, 103);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_sub(94, 91, 87);
            s.copy_ad(790, 349);
            s.store_div(335, 154, 99);
            s.store_mul(258, 335, 94);
            s.store_offset(259, 258, 1.0);
            s.store_sqrt(260, 259);
            s.store_mul(261, 260, 259);
            s.store_mul(262, 261, 259);
            s.store_div_from_scalar_offset_input(263, 1.0, 260, 1.0);
            s.store_div_from_scalar_offset_input(264, 1.0, 261, 1.0);
            s.store_div_from_scalar_offset_input(265, 1.0, 262, 1.0);
            s.store_div(266, 263, 100);
            s.store_offset_mul_offset_rhs(335, 258, 258, 3.0, 3.0);
            s.store_mul3_affine_lhs(267, 100, 264, 0.6666666666666667, 0.0, 335);
            s.store_offset_mul_offset_rhs_ad_rhs(335, 258, A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 5.0), 10.0), 10.0, 5.0);
            s.store_mul_product3_rhs(268, 335, A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), s.ad_value(101), s.ad_value(265), 1.0);
            s.store_sub_ad_lhs(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);
            s.store_add_scaled_inputs4(335, s.ad_value(85), 1.0, s.ad_value(155), 1.0, s.ad_value(87), (-(2.0 * 0.5)), s.ad_value(94), (-0.5));
            s.store_sub(336, 266, 267);
            s.store_mul(337, 154, 185);
            s.store_mul(338, 154, 209);
            s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);
            s.store_mul(248, 94, 250);
        }

        s.b[2456] = (s.v[347] == 1.0);
        s.v[2456] = if s.b[2456] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2456]) {
            s.store_scalar(948, 1.0);
        }

        s.b[2457] = (s.v[948] == 0.0);
        s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };

        s.b[2458] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2458] = if s.b[2458] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2459] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
            s.store_offset_add_scaled_inputs3_offset(781, s.ad_value(168), 1.0, s.ad_value(87), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
        }

    }

    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
    ) {
        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
            s.copy_ad(726, 770);
        }

        s.b[2460] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        s.b[2461] = (2.0 == 1.0);
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2462] = (2.0 == 2.0);
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) && s.b[2462]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2463] = (2.0 == 4.0);
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        if (((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) && (!s.b[2462])) && s.b[2463]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2464] = (2.0 == 8.0);
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        if ((((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) && (!s.b[2462])) && (!s.b[2463])) && s.b[2464]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign60000_loop_guard: usize = 0;
        while {
            let assign60000_cond_e93569: f64 = if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60000_cond_e93569 != 0.0
        } {
            assign60000_loop_guard += 1;
            assert!(assign60000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && (!s.b[2460])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset(168, s.ad_value(87), 1.0, s.ad_value(790), 1.0, s.ad_value(780), 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && (!s.b[2459])) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && (!s.b[2459])) {
            s.store_scalar(334, 1.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) {
            s.copy_ad(335, 684);
            s.store_sqrt_sub(342, 91, 1431);
            s.store_mul(171, 335, 342);
            s.store_div_scaled_inputs(343, s.ad_value(335), 0.5, s.ad_value(342), 1.0);
            s.store_div_from_scalar(334, 1.0, 171);
            s.store_mul(335, 238, 334);
            s.store_scale(336, 335, s.v[509]);
            s.store_scale(337, 334, s.v[509]);
            s.store_add_scaled_product_indices(339, 336, 1.0, 508, 166, 1.0);
            s.store_div_from_scalar(335, 1.0, 339);
            s.store_scale(338, 335, 1.034943e-10);
            s.store_scalar(335, (1.0 - s.v[507]));
            s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 87, s.v[507], 335, 91, 1.0);
        }

        s.b[2465] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
            s.store_offset_add_scaled_inputs3_offset(781, s.ad_value(168), 1.0, s.ad_value(87), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2466] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        s.b[2467] = (2.0 == 1.0);
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && s.b[2467]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2468] = (2.0 == 2.0);
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (!s.b[2467])) && s.b[2468]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2469] = (2.0 == 4.0);
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        if (((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (!s.b[2467])) && (!s.b[2468])) && s.b[2469]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2470] = (2.0 == 8.0);
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        if ((((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (!s.b[2467])) && (!s.b[2468])) && (!s.b[2469])) && s.b[2470]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign60480_loop_guard: usize = 0;
        while {
            let assign60480_cond_e94350: f64 = if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60480_cond_e94350 != 0.0
        } {
            assign60480_loop_guard += 1;
            assert!(assign60480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && (!s.b[2466])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset(168, s.ad_value(87), 1.0, s.ad_value(790), 1.0, s.ad_value(780), 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2465])) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2465])) {
            s.store_scalar(334, 1.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) {
            s.store_sub(340, 168, 91);
            s.store_mul(337, 154, 238);
            s.store_div_from_scalar(335, 1.0, 337);
            s.store_mul_ad_product_lhs(339, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), s.ad_value(250), 335);
            s.store_mul(336, 339, 154);
            s.store_scale(344, 166, 9662367879.197212);
            s.store_scalar(335, 100000.0);
            s.store_div_from_scalar(336, 1.0, 162);
            s.store_mul_ad_lhs(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);
            s.store_mul(337, 336, 338);
            s.store_mul(341, 345, 338);
            s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));
            s.store_mul3_affine_lhs(335, 344, 338, 8.0, 0.0, 338);
            s.store_scaled_mul(336, 345, 338, 2.0);
            s.store_mul3_lhs(342, 345, 338, 338);
            s.store_sqrt_square_add(343, 341, 342);
            s.store_scaled_sub(169, 343, 341, 0.5);
            s.copy_ad(335, 169);
            s.store_mul(169, 208, 335);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
            s.store_scale(169, 169, s.v[619]);
            s.store_add(335, 85, 155);
            s.store_add_scaled_product_indices(336, 269, (-1.0), 335, 267, 1.0);
            s.store_mul_ad_rhs(240, 209, A::add_scaled_products(s.ad_value(209), A::add_scaled_sub_value_product(1.5, A::offset(s.ad_value(99), 1.0), 1.0, s.ad_value(154), s.ad_value(94), (-0.5)), 1.0, s.ad_value(185), s.ad_value(336), 1.0));
            s.copy_ad(335, 154);
            s.store_div_scaled_product_indices(131, 335, 240, 1.0, 250, 1.0);
            s.store_scale(335, 212, 2.0);
            s.store_mul_sub_rhs(241, 335, 267, 100);
            s.store_scaled_sub(336, 267, 100, 2.0);
            s.store_add(126, 94, 241);
            s.store_div_from_scalar(335, 1.0, 127);
            s.store_mul(336, 126, 335);
            s.store_sub_from_scalar(337, 1.0, 336);
            s.store_sub_from_scalar(332, 1.0, 337);
            s.store_square(722, 332);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2471] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        s.b[2472] = (4.0 == 1.0);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && s.b[2472]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2473] = (4.0 == 2.0);
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (!s.b[2472])) && s.b[2473]) {
            s.store_scalar(720, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2474] = (4.0 == 4.0);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

        if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) && s.b[2474]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2475] = (4.0 == 8.0);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) && (!s.b[2474])) && s.b[2475]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign61180_loop_guard: usize = 0;
        while {
            let assign61180_cond_e95323: f64 = if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61180_cond_e95323 != 0.0
        } {
            assign61180_loop_guard += 1;
            assert!(assign61180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2471])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(338, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(125, 1.0, 333);
            s.store_offset_mul_offset_rhs(242, 125, 125, 1.0, 1.0);
        }

        s.b[2476] = (((1.0 + s.v[125]) < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
            s.store_sub_from_scalar_ad(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), A::offset(s.ad_value(125), 1.0));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2477] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        s.b[2478] = (2.0 == 1.0);
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && s.b[2478]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2479] = (2.0 == 2.0);
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (!s.b[2478])) && s.b[2479]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2480] = (2.0 == 4.0);
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (!s.b[2478])) && (!s.b[2479])) && s.b[2480]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2481] = (2.0 == 8.0);
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        if (((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (!s.b[2478])) && (!s.b[2479])) && (!s.b[2480])) && s.b[2481]) {
            s.store_scalar(720, 4.0);
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign61510_loop_guard: usize = 0;
        while {
            let assign61510_cond_e95778: f64 = if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61510_cond_e95778 != 0.0
        } {
            assign61510_loop_guard += 1;
            assert!(assign61510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && (!s.b[2477])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(243, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2476])) {
            s.store_offset(243, 125, 1.0);
            s.store_scalar(334, 1.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
            s.store_div_scaled_product_indices(335, 127, 242, 0.6666666666666667, 243, 1.0);
            s.store_mul(133, 335, 185);
            s.store_offset(244, 125, 0.5);
            s.store_mul(245, 243, 242);
            s.store_div_scaled_inputs(246, s.ad_value(244), 0.4, s.ad_value(245), 1.0);
            s.store_sub_from_scalar(247, 0.6, 246);
        }

        s.b[2482] = (s.v[247] > 0.5);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2482]) {
            s.store_scalar(247, 0.5);
        }

        s.b[2483] = (s.v[347] == 2.0);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) {
            s.copy_ad(335, 131);
            s.store_add_scaled_product_value_ad(131, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(239)), 1.0, 207, 131, 1.0);
        }

        s.b[2484] = (s.v[131] < 0.0);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) && s.b[2484]) {
            s.store_scalar(131, 0.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) {
            s.copy_ad(335, 133);
            s.store_add_scaled_product_value_ad(133, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(238)), 1.0, 207, 133, 1.0);
        }

        s.b[2485] = (s.v[133] < 0.0);
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) && s.b[2485]) {
            s.store_scalar(133, 0.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) {
            s.copy_ad(335, 247);
            s.store_add_scaled_product_value_ad(247, A::scale_offset(s.ad_value(207), (-0.5), 0.5), 1.0, 207, 247, 1.0);
            s.copy_ad(335, 169);
            s.store_mul(169, 207, 169);
        }

        if (((!s.b[1439]) && s.b[2426]) && (s.v[948] != 0.0)) {
            s.store_scalar(948, 0.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_sub(170, 162, 169);
        }

        s.b[2486] = (s.v[170] < 1e-9);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2486]) {
            s.store_scalar(170, 1e-9);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_scalar(335, (s.v[625] / 100.0));
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_mul_ad(338, A::sub(s.ad_value(91), s.ad_value(87)), s.ad_value(334), 1.0);
            s.store_add_scaled_products_indices(339, 335, 131, 1.0, 336, 133, 1.0);
            s.store_div(337, 339, 338);
            s.store_mul_scale_offset_rhs(251, 337, 1434, p.p166, 1.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(342, 339, 251);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_ad(341, s.ad_value(251), A::offset(s.ad_value(624), (-1.0)));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), s.v[474])), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_square(337, 335);
            s.store_mul_neg_lhs(338, 154, 337);
            s.store_mul(339, 338, 170);
            s.store_mul_offset_rhs(340, 338, 238, 1e-25);
            s.store_mul_ad_product_lhs(333, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), s.ad_value(250), 335);
            s.store_div_scaled_inputs(336, s.ad_value(257), 0.2, s.ad_value(254), 1.0);
            s.store_div_scaled_inputs(337, s.ad_value(336), -1.0, s.ad_value(254), 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2487] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2487]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2488] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2487])) && s.b[2488]) {
            s.copy_ad(337, 335);
        }

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2487])) && (!s.b[2488])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[2489] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2489]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2490] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2489])) && s.b[2490]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2489])) && (!s.b[2490])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2489])) && (!s.b[2490])) {
            s.store_mul(339, 338, 340);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(253, 254, 339);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_div_scaled_inputs(335, s.ad_value(115), -1.0, s.ad_value(170), 1.0);
            s.store_mul3_lhs(135, 115, 248, 253);
        }

    }

    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2491] = (p.p283 != 0.0);
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2491]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_scale(336, 336, 0.5);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(87), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2492] = (s.v[336] < 0.0);
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2491]) && s.b[2492]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2491]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1435, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3(339, s.ad_value(87), 1.0, s.ad_value(340), 1.0, s.ad_value(1434), -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1435), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2491])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2493] = (p.p287 != 0.0);
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2493]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1435);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2493])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2494] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2494]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_mul3_lhs(45, 115, 249, 253);
            s.store_add(135, 135, 45);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2494])) {
            s.store_scalar(45, 0.0);
        }

        s.b[2495] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        s.b[2496] = (p.p296 > 0.0);
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(336, s.ad_value(338), 1.0, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(341, s.ad_value(337), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && (!s.b[2496])) {
            s.copy_ad(341, 647);
        }

        s.b[2497] = (s.v[793] >= 0.0);
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2497]) {
            s.copy_ad(369, 793);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && (!s.b[2497])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2498] = (s.v[369] < (20.0 * 1e-12));
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2498]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && (!s.b[2498])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2495]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2495])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        s.b[2499] = (p.p27 != 0.0);
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_scale(335, 186, 1.034943e-10);
            s.copy_ad(336, 684);
            s.store_scalar(337, (s.v[628] - p.p139));
            s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));
            s.store_mul_ad_product_lhs(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), s.ad_value(336), 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs(340, s.ad_value(339), 0.5, s.ad_value(181), 1.0);
            s.store_mul_ad_product_lhs(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.ad_value(338), 181);
            s.store_mul_product3_rhs(342, 181, s.ad_value(335), s.ad_value(336), s.ad_value(338), (-2.0));
            s.store_scalar(338, s.v[496]);
            s.store_scalar(340, s.v[497]);
            s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1435, 1.0);
            s.store_mul(137, 121, 335);
            s.store_sub_from_scalar_scaled_input(335, s.v[498], 790, p.p213);
            s.store_add_scaled_inputs3_offset(138, s.ad_value(1436), 1.0, s.ad_value(335), 1.0, s.ad_value(137), 1.0, (-s.v[160]));
            s.store_mul3_lhs(141, 694, 186, 186);
            s.store_scaled_mul(142, 141, 154, 0.5);
            s.store_scaled_mul(143, 142, 154, 2.0);
            s.store_scale(345, 154, 0.25);
            s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);
            s.store_offset_sub(335, 1436, 344, (-0.005));
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_sqrt_ad(336, A::add_scaled_square_product(s.ad_value(335), 1.0, s.ad_value(334), s.ad_value(344), (4.0 * 0.005)));
            s.store_sub_ad_lhs(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1434);
            s.store_offset_mul(338, 154, 337, (-1.0));
            s.store_div_from_scalar(339, 4.0, 143);
            s.store_offset_mul(335, 338, 339, 1.0);
            s.store_mul(340, 154, 339);
            s.store_mul(341, 338, 339);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2500] = (s.v[335] < 0.0);
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2500]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(144, 335);
            s.store_mul_sub_from_scalar_rhs(334, 142, 1.0, 144);
            s.store_add(146, 138, 334);
            s.store_div_from_scalar_add_ad(334, 1.0, s.ad_value(154), A::div_scalar_offset_denominator(2.0, s.ad_value(138), 1e-25, 1.0));
            s.store_mul_ln_ad_lhs(147, A::mul(A::div_scalar_by_product(1.0, s.ad_value(140), s.ad_value(141), 1.0), A::square(s.ad_value(138))), 334);
            s.store_offset_sub(148, 147, 146, (-0.002));
            s.store_sqrt_add_scaled_square_input(334, 148, 1.0, 147, (4.0 * 0.002));
            s.store_add_scaled_inputs3(149, s.ad_value(147), 1.0, s.ad_value(148), (-0.5), s.ad_value(334), (-0.5));
            s.store_mul_exp_ad_rhs(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));
            s.store_add_ad_lhs(335, A::offset(A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1434))), (-1.0)), 334);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2501] = (s.v[335] < 0.0);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2501]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
        }

    }

    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_sqrt(150, 335);
            s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1434)), (-1.0));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2502] = (s.v[335] < 0.0);
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2502]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(151, 335);
            s.store_div_from_scalar(336, 0.5, 151);
            s.store_mul_sub_rhs(152, 139, 150, 151);
            s.store_sub(335, 146, 149);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2503] = (s.v[335] < 0.0);
        s.v[2503] = if s.b[2503] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2503]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
            s.store_div(332, 790, 335);
            s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));
            s.store_square(722, 332);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2504] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2504] = if s.b[2504] { 1.0 } else { 0.0 };

        s.b[2505] = (4.0 == 1.0);
        s.v[2505] = if s.b[2505] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && s.b[2505]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2506] = (4.0 == 2.0);
        s.v[2506] = if s.b[2506] { 1.0 } else { 0.0 };

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && s.b[2506]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2507] = (4.0 == 4.0);
        s.v[2507] = if s.b[2507] { 1.0 } else { 0.0 };

        if (((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && (!s.b[2506])) && s.b[2507]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2508] = (4.0 == 8.0);
        s.v[2508] = if s.b[2508] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && (!s.b[2506])) && (!s.b[2507])) && s.b[2508]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign64080_loop_guard: usize = 0;
        while {
            let assign64080_cond_e99003: f64 = if (((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign64080_cond_e99003 != 0.0
        } {
            assign64080_loop_guard += 1;
            assert!(assign64080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && (!s.b[2504])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);
            s.store_scale(145, 155, ((2.0 * s.v[495]) * p.p7));
            s.copy_ad(335, 170);
            s.store_div_scaled_product_left_ad(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);
            s.store_add(134, 134, 153);
        }

        s.b[2509] = (((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[963] == 0.0));
        s.v[2509] = if s.b[2509] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {
            s.store_square(317, 127);
            s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);
            s.store_sub(319, 317, 318);
            s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);
            s.store_scaled_add(317, 317, 782, 0.5);
        }

        s.b[2510] = (s.v[317] < 0.0);
        s.v[2510] = if s.b[2510] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2510]) {
            s.store_scalar(317, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {
            s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);
            s.store_scaled_add(319, 319, 782, 0.5);
        }

        s.b[2511] = (s.v[319] < 0.0);
        s.v[2511] = if s.b[2511] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2511]) {
            s.store_scalar(319, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {
            s.store_sub(320, 317, 319);
        }

        s.b[2512] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));
        s.v[2512] = if s.b[2512] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2512]) {
            s.store_scalar(321, 0.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && (!s.b[2512])) {
            s.store_scalar(321, 1.0);
        }

        if ((!s.b[1439]) && (s.v[946] != 0.0)) {
            s.store_scalar(946, 0.0);
        }

        s.b[2513] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));
        s.v[2513] = if s.b[2513] { 1.0 } else { 0.0 };

        if ((!s.b[1439]) && s.b[2513]) {
            s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);
            s.store_add_ad_lhs(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);
        }

        if ((!s.b[1439]) && (!s.b[2513])) {
            s.store_scalar(128, 0.0);
        }

        if (!s.b[1439]) {
            s.copy_ad(136, 134);
            s.store_scalar(46, 0.0);
        }

        s.b[2515] = ((p.p450 > 0.0) && (p.p454 > 0.0));
        s.v[2515] = if s.b[2515] { 1.0 } else { 0.0 };

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_scalar(2520, 1e-5);
            s.store_offset_add_scaled_inputs3_offset(2521, s.ad_value(791), 1.0, s.ad_value(120), 1.0, s.ad_value(182), -1.0, (-s.v[160]), (-p.p455));
            s.store_offset(2522, 118, p.p455);
            s.store_sqrt_offset_ad(781, A::mul(A::sub(s.ad_value(960), s.ad_value(1431)), A::sub(s.ad_value(960), s.ad_value(1431))), ((4.0 * 0.01) * 0.01));
            s.store_add_scaled_inputs3(2532, s.ad_value(960), 0.5, s.ad_value(1431), ((-1.0) * 0.5), s.ad_value(781), 0.5);
            s.store_sqrt_ad(2516, A::div_scaled_product_offset_denominator(s.ad_value(2532), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));
            s.store_mul(2518, 2516, 162);
            s.store_div_scaled_product_add_scaled_denominator_indices(993, 2518, 2518, (-0.25), 790, 1.0, 2518, 1.0, 1.0);
        }

        s.b[2534] = (p.p457 > 0.0);
        s.v[2534] = if s.b[2534] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2515]) && s.b[2534]) {
            s.store_scalar(2519, p.p457);
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.copy_ad(2535, 993);
            s.copy_ad(2536, 2522);
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2521), s.ad_value(2535))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.store_add_ad_rhs(89, 2521, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
            s.store_mul_sub_rhs(116, 154, 89, 2535);
        }

        s.b[2537] = (s.v[116] < 3.0);
        s.v[2537] = if s.b[2537] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {
            s.store_mul_sub_rhs(333, 154, 2521, 2535);
            s.store_div_from_scalar_ad(335, 1.0, A::mul_scaled_lhs(s.ad_value(154), (1.414213562373095 / 108.0), s.ad_value(212)));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 2535, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2538] = (s.v[791] <= s.v[2536]);
        s.v[2538] = if s.b[2538] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && s.b[2538]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 2521, 2521);
            s.store_add_ad_rhs(337, 154, A::div_from_scalar(2.0, s.ad_value(2521)));
            s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p.p456);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

    }

    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_add_scaled_inputs3(88, s.ad_value(90), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.store_offset(332, 2535, (1e-12 / 2.0));
        }

        s.b[2539] = (s.v[88] < s.v[332]);
        s.v[2539] = if s.b[2539] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2539]) {
            s.copy_ad(88, 332);
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.copy_ad(2519, 88);
        }

        s.b[2540] = (p.p451 == 1.0);
        s.v[2540] = if s.b[2540] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) {
            s.copy_ad(88, 2519);
            s.copy_ad(2541, 993);
            s.store_offset_add_scaled_inputs3_offset(86, s.ad_value(120), (-1.0), s.ad_value(182), 1.0, s.ad_value(2541), 1.0, s.v[160], p.p455);
        }

        s.b[2550] = (s.v[791] < s.v[86]);
        s.v[2550] = if s.b[2550] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
            s.store_scalar(347, (-1.0));
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 2521, 2541);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2551] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2551] = if s.b[2551] { 1.0 } else { 0.0 };

        if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) && s.b[2551]) {
            s.store_add_scaled_inputs3_offset(274, s.ad_value(278), 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, s.ad_value(339), 1.0, ((-7.0) * 1.414213562373095));
        }

        if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) && (!s.b[2551])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_add_ad_lhs(274, A::offset(s.ad_value(275), ((-7.0) * 1.414213562373095)), 339);
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 2541, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 2541);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_ad_lhs(2519, A::div(s.ad_value(335), s.ad_value(337)), 2541);
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
            s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2541), (-p.p456)));
            s.store_scalar(79, 0.0);
            s.copy_ad(2542, 88);
            s.store_mul3_affine_lhs(2543, 166, 2520, (0.5 * 9662367879.197212), 0.0, 2520);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2543)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(2544, A::ln(s.ad_value(335)), 2543);
            s.store_scalar(97, 1.0);
        }

        let mut assign65270_loop_guard: usize = 0;
        while {
            let assign65270_cond_e100796: f64 = (s.v[421] + 1.0);
            let assign65270_cond_e100798: f64 = if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (s.v[97] <= assign65270_cond_e100796)) { 1.0 } else { 0.0 };
            assign65270_cond_e100798 != 0.0
        } {
            assign65270_loop_guard += 1;
            assert!(assign65270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
                s.store_sub(2545, 2542, 2541);
                s.store_mul(116, 154, 2545);
                s.store_mul_sub_rhs(333, 2544, 2545, 2543);
            }
            s.b[2552] = (s.v[333] < 60.0);
            s.v[2552] = if s.b[2552] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2552]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 2544, -1.0, 2543);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(2547, A::ln(A::offset(s.ad_value(336), 1.0)), 2544);
                s.store_div_scaled_value_offset_denominator(2548, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2552])) {
                s.store_sub(2547, 2545, 2543);
                s.store_scalar(2548, 1.0);
            }
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
                s.store_mul(2546, 154, 2547);
            }
            s.b[2553] = (((s.v[116]) as f64).abs() < 1e-16);
            s.v[2553] = if s.b[2553] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2553]) {
                s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2548))), 1.0 / (2.0));
                s.store_mul(223, 116, 334);
                s.store_mul(2549, 154, 334);
            }
            s.b[2554] = (s.v[116] < 0.0);
            s.v[2554] = if s.b[2554] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2553]) && s.b[2554]) {
                s.store_neg(223, 223);
                s.store_neg(2549, 2549);
            }
            s.b[2555] = (((s.v[116]) as f64).abs() < 0.005);
            s.v[2555] = if s.b[2555] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2553])) && s.b[2555]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2546, 1.0, 2546, 1.0, 2546, 1.0, 2546, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 2546, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2546), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2546), 1.0, A::scale(s.ad_value(2546), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(223, 334, 336);
                s.store_div_scaled_product_right_ad(2549, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2548), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2553])) && (!s.b[2555])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 2546);
                s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2546), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
                s.store_div_scaled_product_right_ad(2549, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2548), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            s.b[2556] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));
            s.v[2556] = if s.b[2556] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2556]) {
                s.store_scalar(347, (-1.0));
            }
            s.b[2557] = (s.v[116] < 0.0);
            s.v[2557] = if s.b[2557] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2557]) {
                s.store_neg(216, 223);
                s.store_neg(217, 2549);
            }
            s.b[2558] = (s.v[116] < 1e-7);
            s.v[2558] = if s.b[2558] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2557])) && s.b[2558]) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 2549);
            }
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2557])) && (!s.b[2558])) {
                s.store_mul_offset_rhs(117, 154, 2542, (-p.p456));
                s.store_exp(228, 117);
                s.store_mul_ad_rhs(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 210, s.ad_value(154), A::sub(s.ad_value(228), s.ad_value(230)));
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(2549), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
                s.store_add_scaled_inputs_product_indices(232, 2542, 1.0, 2521, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2559] = (s.v[79] == 1.0);
            s.v[2559] = if s.b[2559] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2559]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2542]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2542))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2560] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2560] = if s.b[2560] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) && s.b[2560]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {
                s.store_add(2542, 2542, 236);
            }
            s.b[2561] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2561] = if s.b[2561] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) && s.b[2561]) {
                s.store_scalar(79, 1.0);
            }
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
            s.copy_ad(2519, 2542);
        }

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_mul_sub_scaled_inputs_rhs(339, 154, s.ad_value(2519), -1.0, s.ad_value(993), -1.0);
            s.store_abs(2531, 339);
            s.store_exp(340, 339);
            s.store_sub_ad_lhs(341, A::offset(s.ad_value(340), (-1.0)), 339);
        }

        s.b[2562] = (s.v[339] > 1e-7);
        s.v[2562] = if s.b[2562] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2515]) && s.b[2562]) {
            s.store_mul_scaled_sqrt_rhs(2533, 209, -1.0, 341);
        }

        s.b[2563] = (s.v[2531] > 1e-7);
        s.v[2563] = if s.b[2563] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2562])) && s.b[2563]) {
            s.store_mul_sqrt_rhs(2533, 209, 341);
        }

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2562])) && (!s.b[2563])) {
            s.store_mul_scaled_sqrt_ad_rhs(2533, 339, (-0.7071067811865475), A::offset(A::mul_scaled_lhs(s.ad_value(2531), 0.3333333333333333, A::scale_offset(s.ad_value(2531), 0.25, 1.0)), 1.0));
        }

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_sqrt_square_offset(781, 2533, ((4.0 * 1e-6) * 1e-6));
            s.store_scaled_add(2528, 2533, 781, 0.5);
            s.store_div_scaled_inputs(2529, s.ad_value(2528), 1.0, s.ad_value(586), 1.6021918e-19);
            s.store_offset(335, 2529, (-p.p452));
            s.store_scale(2530, 2529, 0.01);
            s.store_sqrt_ad(781, A::add_scaled_square_product(s.ad_value(335), 1.0, s.ad_value(2530), s.ad_value(2530), 4.0));
            s.store_scaled_add(336, 335, 781, 0.5);
            s.store_div_scaled_product_by_product(2527, s.ad_value(336), s.ad_value(336), 1.0, s.ad_value(2529), s.ad_value(2529), 1.0);
            s.store_add_scaled_product_left_ad(994, 993, 1.0, A::sub(s.ad_value(2519), s.ad_value(993)), 2527, 1.0);
            s.store_mul_sub_from_scalar_rhs_ad(333, A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1431), 1.0))), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790))));
            s.store_scalar(2523, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());
            s.store_mul_sqrt_rhs(2524, 2523, 155);
            s.store_mul_sub_rhs(2517, 154, 994, 993);
        }

        s.b[2564] = ((s.v[2517] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));
        s.v[2564] = if s.b[2564] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
            s.store_sub_scaled_inputs(781, 154, 0.2, 2517, 1.0);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2565] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2565] = if s.b[2565] { 1.0 } else { 0.0 };

        s.b[2566] = (1.0 == 1.0);
        s.v[2566] = if s.b[2566] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && s.b[2566]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2567] = (1.0 == 2.0);
        s.v[2567] = if s.b[2567] { 1.0 } else { 0.0 };

        if ((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && s.b[2567]) {
            s.store_scalar(720, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2568] = (1.0 == 4.0);
        s.v[2568] = if s.b[2568] { 1.0 } else { 0.0 };

        if (((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) && s.b[2568]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2569] = (1.0 == 8.0);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) && (!s.b[2568])) && s.b[2569]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign65750_loop_guard: usize = 0;
        while {
            let assign65750_cond_e102478: f64 = if (((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign65750_cond_e102478 != 0.0
        } {
            assign65750_loop_guard += 1;
            assert!(assign65750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && (!s.b[2565])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);
            s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);
        }

        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2564])) {
            s.copy_ad(335, 2517);
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_sqrt_offset_input(2525, 335, (10.0 * 2.220446049250313e-16));
            s.store_mul(2526, 2524, 2525);
            s.store_mul_scaled_ad_lhs(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2526, p.p454);
            s.store_scaled_mul(46, 995, 333, s.v[632]);
            s.store_add(134, 136, 46);
        }

        if (!s.b[1439]) {
            s.store_add(134, 136, 46);
            s.copy_ad(978, 133);
        }

        s.store_scale(335, 162, (-s.v[635]));

        s.store_mul(20, 335, 131);

        s.store_mul(132, 335, 133);

        s.store_mul(19, 132, 247);

        s.store_mul(979, 335, 978);

        s.store_scaled_sub(335, 790, 94, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p263)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(110, p.p263, 782);

        s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);

        s.b[2570] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if s.b[2570] {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2571] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        s.b[2572] = (2.0 == 1.0);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        if ((s.b[2570] && s.b[2571]) && s.b[2572]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2573] = (2.0 == 2.0);
        s.v[2573] = if s.b[2573] { 1.0 } else { 0.0 };

        if (((s.b[2570] && s.b[2571]) && (!s.b[2572])) && s.b[2573]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2574] = (2.0 == 4.0);
        s.v[2574] = if s.b[2574] { 1.0 } else { 0.0 };

        if ((((s.b[2570] && s.b[2571]) && (!s.b[2572])) && (!s.b[2573])) && s.b[2574]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2575] = (2.0 == 8.0);
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        if (((((s.b[2570] && s.b[2571]) && (!s.b[2572])) && (!s.b[2573])) && (!s.b[2574])) && s.b[2575]) {
            s.store_scalar(720, 4.0);
        }

        if (s.b[2570] && s.b[2571]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign66290_loop_guard: usize = 0;
        while {
            let assign66290_cond_e102986: f64 = if ((s.b[2570] && s.b[2571]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66290_cond_e102986 != 0.0
        } {
            assign66290_loop_guard += 1;
            assert!(assign66290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[2570] && s.b[2571]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (s.b[2570] && (!s.b[2571])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if s.b[2570] {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if s.b[2570] {
        }

        if (!s.b[2570]) {
        }

        if (!s.b[2570]) {
            s.store_scalar(334, 1.0);
        }

        s.store_add(109, 87, 110);

        s.store_add_scaled_product_left_ad(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p.p435);

        s.b[2576] = (p.p23 == 0.0);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        if s.b[2576] {
            s.store_scalar(280, 0.0);
            s.store_scalar(288, 0.0);
        }

        s.b[2577] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_mul(335, 659, 85);
            s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));
            s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1434, (-1.0));
            s.store_offset_mul(340, 338, 339, 1.0);
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.b[2578] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_sub_ad_lhs(781, A::offset(s.ad_value(341), 1e-6), 340);
            s.store_square(722, 781);
            s.store_square(723, 341);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2579] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        s.b[2580] = (4.0 == 1.0);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        if (((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && s.b[2580]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2581] = (4.0 == 2.0);
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        if ((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && s.b[2581]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2582] = (4.0 == 4.0);
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        if (((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && (!s.b[2581])) && s.b[2582]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2583] = (4.0 == 8.0);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        if ((((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2582])) && s.b[2583]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign66800_loop_guard: usize = 0;
        while {
            let assign66800_cond_e103497: f64 = if (((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66800_cond_e103497 != 0.0
        } {
            assign66800_loop_guard += 1;
            assert!(assign66800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && (!s.b[2579])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 341, 726);
            s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);
            s.store_sub_ad_lhs(340, A::offset(s.ad_value(341), 1e-6), 780);
        }

        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
        }

        if (((!s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
        }

        if (((!s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_sqrt(340, 340);
            s.store_add_ad_rhs(282, 335, A::mul_sub_from_scalar_rhs(s.ad_value(337), 1.0, s.ad_value(340)));
            s.store_div_from_scalar_offset_input(336, s.v[582], 661, s.v[582]);
            s.store_add_scaled_inputs_product_indices(283, 1435, s.v[483], 109, 1.0, 336, 282, (-1.0));
            s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.b[2584] = (s.v[283] < 0.0);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2577]) && s.b[2584]) {
            s.store_scalar(283, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[2576]) && s.b[2577]) && s.b[2584]) {
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_offset(283, 283, 1e-25);
            s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);
        }

        if ((!s.b[2576]) && s.b[2577]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_div(339, 662, 958);
            s.store_mul(340, 663, 958);
            s.store_exp_ad(336, A::div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));
            s.store_mul_product3_rhs(280, 336, s.ad_value(339), s.ad_value(283), s.ad_value(134), 1.0);
            s.store_mul3_lhs(288, 339, 283, 336);
        }

        if ((!s.b[2576]) && (!s.b[2577])) {
            s.store_scalar(280, 0.0);
        }

        s.b[2585] = (s.v[664] != 0.0);
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        if ((!s.b[2576]) && s.b[2585]) {
            s.copy_ad(334, 799);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(335, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2586] = (s.v[334] < 0.0);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2585]) && s.b[2586]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(335, 0.0);
        }

        if ((!s.b[2576]) && s.b[2585]) {
            s.store_sqrt_offset_input(335, 127, 1e-25);
            s.store_div_from_scalar_scaled_input(337, 1.0, 335, 2.0);
            s.store_sub_ad_rhs(338, 334, A::scale_offset(s.ad_value(791), ((p.p106) * (p.p105)), p.p105));
            s.store_sqrt_square_offset(782, 338, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 338, 782, 0.5, 0.5);
            s.store_scaled_add(338, 338, 782, 0.5);
        }

        s.b[2587] = (s.v[338] < 0.0);
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2585]) && s.b[2587]) {
            s.store_scalar(338, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2576]) && s.b[2585]) {
            s.store_offset(338, 338, 1e-25);
            s.store_mul_ad_product_rhs(344, 450, s.ad_value(451), A::exp(A::div_from_scalar((-1.0), s.ad_value(338))));
            s.store_mul_offset_ad_rhs(345, 344, A::div_from_scalar(1.0, s.ad_value(338)), 1.0);
            s.store_mul(337, 338, 344);
            s.store_sub(334, 334, 337);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2588] = (s.v[334] < 0.0);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2585]) && s.b[2588]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2576]) && s.b[2585]) {
            s.store_offset(334, 334, 1e-25);
            s.store_div_from_scalar_mul_ad(338, 1.0, s.ad_value(334), s.ad_value(335));
            s.store_scalar(341, (s.v[165] * s.v[554]));
            s.store_exp_mul_scaled_lhs_indices(336, 341, -1.0, 338);
            s.store_mul_product3_rhs(340, 338, s.ad_value(341), s.ad_value(336), s.ad_value(338), 1.0);
            s.store_mul_product3_rhs(281, 336, s.ad_value(664), s.ad_value(134), s.ad_value(334), 1.0);
        }

        s.b[2589] = (p.p45 == 0.0);
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

        if s.b[2589] {
            s.store_scalar(423, 0.0);
        }

        s.b[2590] = ((p.p45 * (s.v[796] - p.p446)) < 0.0);
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        if ((!s.b[2589]) && s.b[2590]) {
            s.copy_ad(426, 427);
        }

        if ((!s.b[2589]) && (!s.b[2590])) {
            s.store_add_scaled_inputs_ad_lhs(426, A::powf(A::offset(s.ad_value(796), (-p.p446)), 2.0), p.p445, 427, 1.0);
        }

        if (!s.b[2589]) {
            s.store_scaled_limited_exp_ad(423, A::mul(s.ad_value(154), A::sub(s.ad_value(793), s.ad_value(426))), p.p449);
        }

        s.b[2591] = (s.v[423] > 0.0);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        s.b[2592] = ((s.v[423] > (100000.0 - 50000.0)) && (50000.0 >= 0.0));
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if (s.b[2591] && s.b[2592]) {
            s.store_offset(781, 423, (((-100000.0)) + (50000.0)));
            s.store_square(722, 781);
            s.store_scalar(723, (50000.0 * 50000.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2593] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        s.b[2594] = (1.0 == 1.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        if (((s.b[2591] && s.b[2592]) && s.b[2593]) && s.b[2594]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2595] = (1.0 == 2.0);
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        if ((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && s.b[2595]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2596] = (1.0 == 4.0);
        s.v[2596] = if s.b[2596] { 1.0 } else { 0.0 };

        if (((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && (!s.b[2595])) && s.b[2596]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2597] = (1.0 == 8.0);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        if ((((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2596])) && s.b[2597]) {
            s.store_scalar(720, 4.0);
        }

        if ((s.b[2591] && s.b[2592]) && s.b[2593]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign67730_loop_guard: usize = 0;
        while {
            let assign67730_cond_e104434: f64 = if (((s.b[2591] && s.b[2592]) && s.b[2593]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign67730_cond_e104434 != 0.0
        } {
            assign67730_loop_guard += 1;
            assert!(assign67730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[2591] && s.b[2592]) && s.b[2593]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((s.b[2591] && s.b[2592]) && (!s.b[2593])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (s.b[2591] && s.b[2592]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 50000.0);
            s.store_div_scaled_product_indices(334, 725, 726, 50000.0, 770, 1.0);
            s.store_offset(336, 780, (100000.0 - 50000.0));
        }

        if (s.b[2591] && s.b[2592]) {
        }

        if (s.b[2591] && (!s.b[2592])) {
            s.copy_ad(336, 423);
            s.store_scalar(334, 1.0);
        }

        s.b[2598] = ((((s.v[280] + s.v[281]) > 0.0) && (s.v[523] != 0.0)) && (s.v[963] == 0.0));
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if s.b[2598] {
            s.store_offset_scaled(334, 120, s.v[524], 1.0);
            s.store_add(335, 280, 281);
            s.store_scaled_mul(111, 334, 335, s.v[523]);
            s.store_div_from_scalar(344, 1.0, 99);
            s.store_mul3_lhs(335, 154, 111, 344);
            s.store_square(345, 344);
            s.store_div_from_scalar(344, 1.0, 102);
            s.store_mul3_lhs(336, 154, 111, 344);
            s.store_square(345, 344);
            s.store_mul_ad_rhs(112, 209, A::add_scaled_products(s.ad_value(104), s.ad_value(336), 1.0, s.ad_value(101), s.ad_value(335), (-1.0)));
            s.store_mul_add_scaled_products_indices_rhs(113, 209, 103, 336, ((-1.0) * (0.5)), 100, 335, 0.5);
            s.store_add(114, 112, 113);
            s.store_mul3_lhs(400, 115, 114, 253);
            s.store_mul(287, 288, 400);
        }

        s.b[2599] = (p.p24 != 0.0);
        s.v[2599] = if s.b[2599] { 1.0 } else { 0.0 };

        s.b[2600] = (s.v[78] == 0.0);
        s.v[2600] = if s.b[2600] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2600]) {
            s.store_offset_add(191, 109, 1435, (-(10.0 * 2.220446049250313e-16)));
            s.store_sub_scaled_ad_lhs(335, A::add_scaled_product(A::offset(s.ad_value(1436), (-s.v[160])), 1.0, A::sub(s.ad_value(120), s.ad_value(182)), s.ad_value(162), s.v[560]), 191, s.v[515]);
            s.store_square(335, 335);
            s.store_scalar(337, (1.0 / s.v[187]));
            s.store_mul(336, 335, 337);
            s.store_scalar(337, (1.0 / s.v[561]));
            s.store_offset_mul(341, 255, 337, 1.0);
            s.store_mul(195, 336, 341);
            s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);
            s.store_scaled_add(195, 195, 782, 0.5);
        }

        s.b[2601] = (s.v[195] < 0.0);
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if ((s.b[2599] && s.b[2600]) && s.b[2601]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (s.b[2599] && s.b[2600]) {
            s.store_sqrt_square_offset(782, 1436, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(338, 1436, 782, 0.5, 0.5);
            s.store_scaled_add(337, 1436, 782, 0.5);
        }

        s.b[2602] = (s.v[337] < 0.0);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        if ((s.b[2599] && s.b[2600]) && s.b[2602]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if (s.b[2599] && s.b[2600]) {
            s.store_offset(337, 337, (-p.p262));
            s.store_scale(332, 337, 10.0);
            s.store_offset_square(336, 332, 1.0);
            s.store_sub_from_scalar_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(336)));
            s.store_mul(195, 195, 335);
            s.store_scale(334, 162, s.v[632]);
        }

    }

    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2599] && s.b[2600]) {
            s.store_div_from_scalar_offset_input(341, s.v[562], 334, s.v[562]);
            s.store_scalar(340, s.v[516]);
            s.store_div_add_scaled_inputs_rhs_indices(343, 340, 340, 1.0, 1435, 1.0);
            s.store_div_from_scalar_offset_input(338, 1.0, 195, 1e-25);
            s.store_scaled_mul(335, 193, 338, (-s.v[514]));
            s.store_scaled_mul(337, 338, 338, s.v[514]);
        }

        s.b[2603] = (s.v[335] < (-34.0));
        s.v[2603] = if s.b[2603] { 1.0 } else { 0.0 };

        if ((s.b[2599] && s.b[2600]) && (!s.b[2603])) {
            s.store_exp(336, 335);
            s.store_mul_scale_ad_lhs(337, A::div_from_scalar(s.v[513], s.ad_value(192)), 1.6021918e-19, 334);
            s.store_div_from_scalar(339, 1.0, 209);
            s.store_sqrt_ad(340, A::mul_offset_lhs(s.ad_value(978), (s.v[188] * 1e-12), s.ad_value(339)));
            s.store_mul3_lhs(338, 336, 337, 340);
            s.store_mul(339, 338, 195);
            s.store_mul(344, 339, 195);
        }

        if s.b[2599] {
            s.store_offset_scaled(334, 791, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, s.v[187]);
            s.store_scale(334, 791, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));
            s.store_mul(337, 791, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_sub(335, 791, 790);
            s.store_offset_scaled(334, 335, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, s.v[187]);
            s.store_scale(334, 335, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));
            s.store_mul(337, 335, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_scaled_offset_ad(195, A::neg(A::sub(s.ad_value(791), s.ad_value(792))), ((s.v[160]) + (p.p258)), 1.0 / (s.v[187]));
            s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);
            s.store_scaled_add(195, 195, 782, 0.5);
        }

        s.b[2606] = (s.v[195] < 0.0);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2606]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if s.b[2599] {
            s.store_offset(195, 195, 1e-25);
            s.store_div_from_scalar(335, (-s.v[520]), 195);
        }

        s.b[2607] = (s.v[335] < (-34.0));
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (s.b[2599] && (!s.b[2607])) {
            s.store_exp(336, 335);
            s.store_mul_div_from_scalar_ad_lhs(337, s.v[520], A::square(s.ad_value(195)), 336);
            s.store_scale(337, 162, (s.v[519] * s.v[632]));
        }

        if s.b[2599] {
            s.copy_ad(285, 677);
            s.store_mul(286, 393, 285);
            s.store_scaled_offset_ad(336, A::add_scaled_inputs4(s.ad_value(1434), s.v[493], s.ad_value(1436), (-1.0), s.ad_value(122), 1.0, s.ad_value(174), 1.0), (-s.v[492]), (-1.0 / (s.v[187])));
            s.store_square(334, 336);
            s.store_scale(335, 286, s.v[491]);
            s.store_div_scaled_inputs(337, s.ad_value(335), -1.0, s.ad_value(336), 1.0);
        }

        s.b[2608] = (s.v[337] < (-34.0));
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2608]) {
            s.store_scalar(339, 0.0);
        }

        if (s.b[2599] && (!s.b[2608])) {
            s.store_exp(339, 337);
        }

        if s.b[2599] {
            s.store_div_from_scalar(338, (((1.6021918e-19 * s.v[490]) * s.v[632]) * s.v[582]), 285);
        }

        s.b[2610] = (p.p25 != 0.0);
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if s.b[2610] {
            s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(790), 1.0, A::scale(s.ad_value(790), 100.0)), (-1e-5));
            s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 790, (4.0 * 1e-5));
            s.store_add_scaled_inputs3(196, s.ad_value(790), 1.0, s.ad_value(335), (-0.5), s.ad_value(336), (-0.5));
        }

        s.b[2611] = (p.p25 == 0.0);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

        if (!s.b[2611]) {
            s.store_add_scaled_inputs4_offset(335, s.ad_value(196), p.p242, s.ad_value(791), (-1.0), s.ad_value(122), p.p244, s.ad_value(174), p.p244, (p.p243 * p.p242));
            s.store_scalar(336, (1.0 / s.v[187]));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(197, 194, 782, 0.5);
        }

        s.b[2612] = (s.v[197] < 0.0);
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

        if ((!s.b[2611]) && s.b[2612]) {
            s.store_scalar(197, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2611]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 197, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2613] = (s.v[334] < (-34.0));
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        if ((!s.b[2611]) && (!s.b[2613])) {
            s.store_exp(335, 334);
            s.store_scale_ad(336, A::div_from_scalar(s.v[511], s.ad_value(192)), (1.6021918e-19 * s.v[632]));
        }

        if (!s.b[2611]) {
            s.store_sub(205, 790, 792);
        }

        s.b[2614] = (s.v[205] > 0.0);
        s.v[2614] = if s.b[2614] { 1.0 } else { 0.0 };

        if ((!s.b[2611]) && s.b[2614]) {
            s.store_square(336, 205);
            s.store_mul(338, 336, 205);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
        }

        s.b[2615] = (p.p25 == 0.0);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if (!s.b[2615]) {
            s.store_add_scaled_inputs3(335, A::add_scaled_inputs3_offset(s.ad_value(196), (-p.p242), s.ad_value(791), -1.0, s.ad_value(196), 1.0, ((p.p243) * (p.p242))), 1.0, s.ad_value(122), p.p244, s.ad_value(174), p.p244);
            s.store_scalar(336, (1.0 / s.v[187]));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(198, 194, 782, 0.5);
        }

        s.b[2616] = (s.v[198] < 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        if ((!s.b[2615]) && s.b[2616]) {
            s.store_scalar(198, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2615]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 198, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2617] = (s.v[334] < (-34.0));
        s.v[2617] = if s.b[2617] { 1.0 } else { 0.0 };

        if ((!s.b[2615]) && (!s.b[2617])) {
            s.store_exp(335, 334);
            s.store_div_from_scalar(337, 1.0, 192);
            s.store_scale(336, 337, (s.v[511] * (1.6021918e-19 * s.v[632])));
        }

        if (!s.b[2615]) {
            s.store_neg(206, 792);
        }

        s.b[2618] = (s.v[206] > 0.0);
        s.v[2618] = if s.b[2618] { 1.0 } else { 0.0 };

        if ((!s.b[2615]) && s.b[2618]) {
            s.store_square(336, 206);
            s.store_mul(338, 336, 206);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
        }

        s.v[2619] = 0.0;

        s.v[2622] = 0.0;

        s.v[2621] = 0.0;

        s.v[406] = 0.0;

        s.v[2621] = 0.0;

        s.b[2623] = (1.0 == 1.0);
        s.v[2623] = if s.b[2623] { 1.0 } else { 0.0 };

        s.b[2624] = (1.0 == 2.0);
        s.v[2624] = if s.b[2624] { 1.0 } else { 0.0 };

        s.b[2625] = (1.0 == 3.0);
        s.v[2625] = if s.b[2625] { 1.0 } else { 0.0 };

        s.b[2626] = (1.0 == 4.0);
        s.v[2626] = if s.b[2626] { 1.0 } else { 0.0 };

        s.b[2627] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2627] = if s.b[2627] { 1.0 } else { 0.0 };

        if (s.b[2623] && s.b[2627]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2619, 1.0);
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2628] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2628] = if s.b[2628] { 1.0 } else { 0.0 };

        if ((s.b[2624] && (!s.b[2623])) && s.b[2628]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2629] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2629] = if s.b[2629] { 1.0 } else { 0.0 };

        if ((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2622, 1.0);
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2630] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2630] = if s.b[2630] { 1.0 } else { 0.0 };

        if (((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2631] = (p.p113 > 0.0);
        s.v[2631] = if s.b[2631] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[2632] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2632] = if s.b[2632] { 1.0 } else { 0.0 };

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && s.b[2632]) {
        }

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && (!s.b[2632])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && (!s.b[2632])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2633] = (s.v[336] < 0.0);
        s.v[2633] = if s.b[2633] { 1.0 } else { 0.0 };

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && s.b[2633]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2634] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2634] = if s.b[2634] { 1.0 } else { 0.0 };

        if ((s.b[2626] && (!((s.b[2623] || s.b[2624]) || s.b[2625]))) && s.b[2634]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(2642, 0.4);
            s.store_scalar(2643, 0.0);
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
            s.store_scalar(79, (-1.0));
            s.store_scalar(2644, 0.0);
            s.store_scalar(2645, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2640, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2640), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2641, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2647] = (s.v[2642] > (s.v[2641] * 0.5));
        s.v[2647] = if s.b[2647] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2647]) {
            s.store_scale(2642, 2641, 0.5);
        }

        s.b[2648] = param_given[338];
        s.v[2648] = if s.b[2648] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2648]) {
            s.store_scalar(2641, p.p338);
        }

        s.b[2649] = param_given[339];
        s.v[2649] = if s.b[2649] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2649]) {
            s.store_scalar(2642, p.p339);
        }

        s.b[2650] = param_given[338];
        s.v[2650] = if s.b[2650] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2649])) && s.b[2650]) {
            s.store_scale(2642, 2641, 0.5);
        }

        s.b[2651] = (s.v[2642] > (s.v[2641] * 0.5));
        s.v[2651] = if s.b[2651] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2651]) {
            s.store_scale(2642, 2641, 0.5);
        }

        s.b[2652] = (p.p38 == 1.0);
        s.v[2652] = if s.b[2652] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2652]) {
            s.store_neg(334, 396);
        }

        s.b[2653] = (s.v[334] > s.v[2642]);
        s.v[2653] = if s.b[2653] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2652]) && s.b[2653]) {
            s.store_sub(335, 334, 2642);
            s.store_sub(336, 2641, 2642);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2642, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[2652]) && (!s.b[2653])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[2652]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2652])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
            s.store_scalar(2636, 0.0);
            s.store_scale(2637, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2654] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2654] = if s.b[2654] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2654]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2654])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign70730_loop_guard: usize = 0;
        while {
            let assign70730_cond_e107147: f64 = if (((s.v[2621] != 0.0) && (!s.b[2654])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign70730_cond_e107147 != 0.0
        } {
            assign70730_loop_guard += 1;
            assert!(assign70730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[2654])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2654])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2655] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2655] = if s.b[2655] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
            s.store_add_scaled_inputs3(781, s.ad_value(402), 1.0, s.ad_value(397), 1.0, s.ad_value(335), 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2656] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2656] = if s.b[2656] { 1.0 } else { 0.0 };

        s.b[2657] = (1.0 == 1.0);
        s.v[2657] = if s.b[2657] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && s.b[2657]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2658] = (1.0 == 2.0);
        s.v[2658] = if s.b[2658] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && s.b[2658]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2659] = (1.0 == 4.0);
        s.v[2659] = if s.b[2659] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && (!s.b[2658])) && s.b[2659]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2660] = (1.0 == 8.0);
        s.v[2660] = if s.b[2660] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2660]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign71060_loop_guard: usize = 0;
        while {
            let assign71060_cond_e107497: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign71060_cond_e107497 != 0.0
        } {
            assign71060_loop_guard += 1;
            assert!(assign71060_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && (!s.b[2656])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2655])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.b[2661] = (s.v[402] < s.v[403]);
        s.v[2661] = if s.b[2661] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2661]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2662] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2662] = if s.b[2662] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2661]) && s.b[2662]) {
            s.store_div_scaled_inputs(274, s.ad_value(277), 0.5, s.ad_value(278), 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2661]) && (!s.b[2662])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[2661]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2644, 404);
        }

        s.b[2663] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2663] = if s.b[2663] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2663]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2663])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2664] = (s.v[116] >= 3.0);
        s.v[2664] = if s.b[2664] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2664]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2664])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2(437, s.ad_value(402), -1.0, s.ad_value(397), -1.0, s.ad_value(212), 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3(116, s.ad_value(439), 1.0, s.ad_value(438), 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2665] = (p.p33 > 0.0);
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2666] = (p.p33 == 2.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(447, s.ad_value(444), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && (!s.b[2666])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            s.copy_ad(445, 116);
        }

        s.b[2667] = (p.p33 == 2.0);
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        s.b[2668] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
            s.store_add_scaled_inputs3(781, s.ad_value(445), 1.0, s.ad_value(446), (-1.0), s.ad_value(446), 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        s.b[2670] = (2.0 == 1.0);
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && s.b[2670]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2671] = (2.0 == 2.0);
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && s.b[2671]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2672] = (2.0 == 4.0);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && (!s.b[2671])) && s.b[2672]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2673] = (2.0 == 8.0);
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && (!s.b[2671])) && (!s.b[2672])) && s.b[2673]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign72190_loop_guard: usize = 0;
        while {
            let assign72190_cond_e109054: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign72190_cond_e109054 != 0.0
        } {
            assign72190_loop_guard += 1;
            assert!(assign72190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && (!s.b[2669])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3(116, s.ad_value(446), 1.0, s.ad_value(446), (-0.2), s.ad_value(780), 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && (!s.b[2668])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && (!s.b[2667])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2674] = (p.p33 == 1.0);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2675] = (s.v[411] > 0.0);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2675]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) {
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2676] = (s.v[336] < 0.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) && s.b[2676]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2677] = (s.v[336] < 0.0);
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2677]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2637, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2678] = (s.v[333] < 60.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2678]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2678])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2679] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2679]) {
            s.store_offset(2643, 2643, 1.0);
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2680] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2680]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2680])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2681, 354, 2637);
        }

        s.b[2683] = (p.p33 == 2.0);
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        s.b[2684] = ((s.v[2681] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
            s.store_add_scaled_inputs3(781, s.ad_value(2681), 1.0, s.ad_value(386), (-1.0), s.ad_value(386), 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2685] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        s.b[2686] = (2.0 == 1.0);
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && s.b[2686]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2687] = (2.0 == 2.0);
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (!s.b[2686])) && s.b[2687]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2688] = (2.0 == 4.0);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (!s.b[2686])) && (!s.b[2687])) && s.b[2688]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2689] = (2.0 == 8.0);
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (!s.b[2686])) && (!s.b[2687])) && (!s.b[2688])) && s.b[2689]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign73000_loop_guard: usize = 0;
        while {
            let assign73000_cond_e110124: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73000_cond_e110124 != 0.0
        } {
            assign73000_loop_guard += 1;
            assert!(assign73000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && (!s.b[2685])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3(335, s.ad_value(386), 1.0, s.ad_value(386), (-0.1), s.ad_value(780), 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && (!s.b[2684])) {
            s.copy_ad(335, 2681);
            s.store_scalar(334, 1.0);
        }

        s.b[2690] = (s.v[334] < 1.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2690]) {
            s.store_offset(2643, 2643, 2.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2683])) {
            if (s.v[2681] <= s.v[386]) {
                s.copy_ad(335, 2681);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2691] = (s.v[2681] >= s.v[386]);
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2683])) && s.b[2691]) {
            s.store_offset(2643, 2643, 2.0);
        }

        s.b[2692] = (s.v[2643] >= 2.0);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) {
            s.copy_ad(2682, 404);
            s.store_mul(354, 335, 2637);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2693] = (p.p33 == 2.0);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        s.b[2694] = ((s.v[404] > (s.v[2682] - 0.1)) && (0.1 >= 0.0));
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
            s.store_offset_sub(781, 404, 2682, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2695] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        s.b[2696] = (2.0 == 1.0);
        s.v[2696] = if s.b[2696] { 1.0 } else { 0.0 };

        if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2697] = (2.0 == 2.0);
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        if ((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) && s.b[2697]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2698] = (2.0 == 4.0);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        if (((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) && (!s.b[2697])) && s.b[2698]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2699] = (2.0 == 8.0);
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if ((((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) && (!s.b[2697])) && (!s.b[2698])) && s.b[2699]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign73450_loop_guard: usize = 0;
        while {
            let assign73450_cond_e110732: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73450_cond_e110732 != 0.0
        } {
            assign73450_loop_guard += 1;
            assert!(assign73450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && (!s.b[2695])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_ad_lhs(404, A::offset(s.ad_value(2682), (-0.1)), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && (!s.b[2694])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && (!s.b[2694])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && (!s.b[2693])) {
            if (s.v[404] <= s.v[2682]) {
            } else {
                s.copy_ad(404, 2682);
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.copy_ad(2644, 404);
        }

        s.b[2700] = (p.p33 == 1.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2637)), s.ad_value(155)), 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2701] = (s.v[411] > 0.0);
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2701]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2701])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2702] = (s.v[336] < 0.0);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2701])) && s.b[2702]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2701])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2703] = (s.v[336] < 0.0);
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2703]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2637, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

        let mut assign73810_loop_guard: usize = 0;
        while {
            let assign73810_cond_e111256: f64 = (s.v[421] + 1.0);
            let assign73810_cond_e111258: f64 = if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (s.v[97] <= assign73810_cond_e111256)) { 1.0 } else { 0.0 };
            assign73810_cond_e111258 != 0.0
        } {
            assign73810_loop_guard += 1;
            assert!(assign73810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2705] = (s.v[333] < 60.0);
            s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2705]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2705])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2706] = (s.v[116] < 0.0);
            s.v[2706] = if s.b[2706] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2706]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2707] = (s.v[116] < 1e-6);
            s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && s.b[2707]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2708] = (s.v[338] > 0.0);
            s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && s.b[2707]) && s.b[2708]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && s.b[2707]) && (!s.b[2708])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && (!s.b[2707])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4(338, s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0));
            }
            s.b[2709] = (s.v[338] > 0.0);
            s.v[2709] = if s.b[2709] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && (!s.b[2707])) && s.b[2709]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && (!s.b[2707])) && (!s.b[2709])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2710] = (s.v[116] < 0.0);
            s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2710]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2711] = (s.v[116] < 60.0);
            s.v[2711] = if s.b[2711] { 1.0 } else { 0.0 };
            s.b[2712] = (s.v[116] < 5e-5);
            s.v[2712] = if s.b[2712] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && s.b[2711]) && s.b[2712]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && s.b[2711]) && (!s.b[2712])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && (!s.b[2711])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2713] = (s.v[214] > 0.0);
            s.v[2713] = if s.b[2713] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && s.b[2713]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && (!s.b[2713])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2714] = (s.v[79] == 1.0);
            s.v[2714] = if s.b[2714] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2714]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2715] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2715] = if s.b[2715] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) && s.b[2715]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) {
                s.store_add(404, 404, 236);
            }
            s.b[2716] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2716] = if s.b[2716] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) && s.b[2716]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.store_mul(2635, 982, 223);
            s.store_mul(2636, 2637, 2635);
            s.store_offset_div(100, 2636, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2718] = (p.p33 == 4.0);
        s.v[2718] = if s.b[2718] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2644);
            s.store_scalar(79, 0.0);
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2637)), s.ad_value(155)), 2.0);
        }

        s.b[2719] = (s.v[411] > 0.0);
        s.v[2719] = if s.b[2719] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2719]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2719])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2720] = (s.v[336] < 0.0);
        s.v[2720] = if s.b[2720] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2719])) && s.b[2720]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2719])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2721] = (s.v[336] < 0.0);
        s.v[2721] = if s.b[2721] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2721]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2637, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign74200_loop_guard: usize = 0;
        while {
            let assign74200_cond_e112911: f64 = (s.v[421] + 1.0);
            let assign74200_cond_e112913: f64 = if (((s.v[2621] != 0.0) && s.b[2718]) && (s.v[97] <= assign74200_cond_e112911)) { 1.0 } else { 0.0 };
            assign74200_cond_e112913 != 0.0
        } {
            assign74200_loop_guard += 1;
            assert!(assign74200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2723] = (s.v[333] < 60.0);
            s.v[2723] = if s.b[2723] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2723]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2723])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2724] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2724] = if s.b[2724] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2724]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2645, 334, 336);
                s.store_mul_add_scaled_product_rhs(2646, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2724])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4(2645, s.ad_value(116), 1.0, s.ad_value(415), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0));
                s.store_mul_sub_ad_rhs(2646, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2725] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2725] = if s.b[2725] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2725]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2726] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2726] = if s.b[2726] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2725])) && s.b[2726]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2725])) && (!s.b[2726])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2727] = (s.v[214] > 0.0);
            s.v[2727] = if s.b[2727] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2727]) {
                s.store_sqrt_add(216, 2645, 214);
                s.store_div_scaled_inputs2(217, s.ad_value(2646), 0.5, s.ad_value(215), 0.5, s.ad_value(216), 1.0);
            }
            s.b[2728] = (s.v[2645] > 0.0);
            s.v[2728] = if s.b[2728] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2727])) && s.b[2728]) {
                s.store_sqrt(216, 2645);
                s.store_div_scaled_inputs(217, s.ad_value(2646), 0.5, s.ad_value(216), 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2727])) && (!s.b[2728])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2729] = (s.v[79] > 0.0);
            s.v[2729] = if s.b[2729] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2729]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2730] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2730] = if s.b[2730] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) && s.b[2730]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) {
                s.store_add(404, 404, 236);
            }
            s.b[2731] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2731] = if s.b[2731] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) && s.b[2731]) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            if (s.v[2645] >= 0.0) {
                s.store_scaled_sqrt(223, 2645, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.store_mul(2635, 982, 223);
            s.store_mul(2636, 2637, 2635);
            s.store_offset_div(100, 2636, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2733] = (s.v[407] < 0.0);
        s.v[2733] = if s.b[2733] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2733]) {
            s.store_neg(407, 407);
        }

        s.b[2734] = (p.p55 == 0.0);
        s.v[2734] = if s.b[2734] { 1.0 } else { 0.0 };

        s.b[2735] = (p.p50 == 0.0);
        s.v[2735] = if s.b[2735] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) && s.b[2735]) {
            s.store_neg(2638, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) && (!s.b[2735])) {
            s.copy_ad(2638, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(2638), p.p137, A::offset(s.ad_value(2638), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2638), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2638), p.p137), 782, 0.5);
        }

        s.b[2736] = (s.v[336] < 0.0);
        s.v[2736] = if s.b[2736] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) && s.b[2736]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3(781, s.ad_value(407), 1.0, s.ad_value(600), (-1.0), s.ad_value(407), (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(603, s.ad_value(407), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2737] = (1.0 == 1.0);
        s.v[2737] = if s.b[2737] { 1.0 } else { 0.0 };

        s.b[2738] = (1.0 == 2.0);
        s.v[2738] = if s.b[2738] { 1.0 } else { 0.0 };

        s.b[2739] = (1.0 == 3.0);
        s.v[2739] = if s.b[2739] { 1.0 } else { 0.0 };

        s.b[2740] = (1.0 == 4.0);
        s.v[2740] = if s.b[2740] { 1.0 } else { 0.0 };

        s.b[2741] = (p.p55 == 1.0);
        s.v[2741] = if s.b[2741] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2737]) && s.b[2741]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && s.b[2737]) && (!s.b[2741])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[2737]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2738] && (!s.b[2737]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2742] = (p.p55 == 1.0);
        s.v[2742] = if s.b[2742] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) && s.b[2742]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) && (!s.b[2742])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) {
            s.copy_ad(697, 404);
        }

        s.b[2743] = (p.p430 == 0.0);
        s.v[2743] = if s.b[2743] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) && s.b[2743]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[2740] && (!((s.b[2737] || s.b[2738]) || s.b[2739])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2621] = 0.0;

        s.b[2744] = (2.0 == 1.0);
        s.v[2744] = if s.b[2744] { 1.0 } else { 0.0 };

        s.b[2745] = (2.0 == 2.0);
        s.v[2745] = if s.b[2745] { 1.0 } else { 0.0 };

        s.b[2746] = (2.0 == 3.0);
        s.v[2746] = if s.b[2746] { 1.0 } else { 0.0 };

        s.b[2747] = (2.0 == 4.0);
        s.v[2747] = if s.b[2747] { 1.0 } else { 0.0 };

        s.b[2748] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2748] = if s.b[2748] { 1.0 } else { 0.0 };

        if (s.b[2744] && s.b[2748]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2619, 1.0);
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2749] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2749] = if s.b[2749] { 1.0 } else { 0.0 };

        if ((s.b[2745] && (!s.b[2744])) && s.b[2749]) {
            s.store_scalar(2621, 1.0);
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2750] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2750] = if s.b[2750] { 1.0 } else { 0.0 };

        if ((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) {
            s.store_scalar(2621, 1.0);
            s.store_scalar(2622, 1.0);
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2751] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2751] = if s.b[2751] { 1.0 } else { 0.0 };

        if (((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) {
            s.store_neg(407, 407);
        }

    }
}
