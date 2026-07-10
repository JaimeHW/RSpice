#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
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
        if (s.b[908] && s.b[909]) {s.store_add_scaled_inputs3_indices(638, 174, 1.0, 173, (-1.0), 173, 0.05);s.store_square(642, 638);s.store_scaled_mul(643, 173, 173, (0.05 * 0.05));s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[910] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });s.b[911] = (2.0 == 1.0);s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });
        if (((s.b[908] && s.b[909]) && s.b[910]) && s.b[911]) {s.store_scalar(648, 1.0);}
        s.b[912] = (2.0 == 2.0);s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
        if ((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && s.b[912]) {s.store_scalar(648, 2.0);}
        s.b[913] = (2.0 == 4.0);s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
        if (((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {s.store_scalar(648, 3.0);}
        s.b[914] = (2.0 == 8.0);s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
        if ((((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && (!s.b[913])) && s.b[914]) {s.store_scalar(648, 4.0);}
        if ((s.b[908] && s.b[909]) && s.b[910]) {s.store_scalar(647, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((s.b[908] && s.b[909]) && s.b[910]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[908] && s.b[909]) && s.b[910]) {s.store_sqrt(646, 646);s.store_primal_offset(647, 647, 1.0);}
        }
        if ((s.b[908] && s.b[909]) && (!s.b[910])) {s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));}
        if (s.b[908] && s.b[909]) {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_mul3_affine_lhs(637, 638, 173, 0.05, 0.0, 646);s.store_div_scaled_product3_mixed_iiia(278, 173, 645, 646, 0.05, A::offset(s.ad_value(220), 1e-50), 1.0);s.store_add_scaled_inputs3_indices(174, 173, 1.0, 173, (-0.05), 637, 1.0);}
        if (s.b[908] && s.b[909]) {
        }
        if (s.b[908] && (!s.b[909])) {
        }
        if (s.b[908] && (!s.b[909])) {s.store_scalar(278, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if s.b[915] {s.store_sqrt(278, 278);s.store_add_scaled_inputs3_indices(111, 109, 1.0, 110, (-0.5), 278, (-0.5));s.store_div_from_scalar(279, 1.0, 278);s.store_mul_exp_mixed_ia(278, 101, A::mul(s.ad_value(120), s.ad_value(111)));s.store_add_offset_lhs_mixed_ai(279, A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0), 278);s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[918] = (s.v[279] < 0.0);s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[918]) {s.store_scalar(279, 0.0);s.store_scalar(278, 0.0);}
        if s.b[915] {s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));s.store_sqrt(113, 279);s.store_offset_mul_ad(279, s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70)), (-1.0));s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[919] = (s.v[279] < 0.0);s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[919]) {s.store_scalar(279, 0.0);s.store_scalar(278, 0.0);}
        if s.b[915] {s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));s.store_sqrt(114, 279);s.store_mul_sub_rhs(115, 100, 113, 114);s.store_sub(279, 107, 111);s.store_sqrt_square_offset(639, 279, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.1));}
        s.b[920] = (s.v[279] < 0.0);s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
        if (s.b[915] && s.b[920]) {s.store_scalar(279, 0.0);s.store_scalar(280, 0.0);}
        if s.b[915] {s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));s.store_div(290, 51, 279);s.store_square(642, 290);s.store_scalar(643, 1.0);s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[921] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });s.b[922] = (4.0 == 1.0);s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
        if ((s.b[915] && s.b[921]) && s.b[922]) {s.store_scalar(648, 1.0);}
        s.b[923] = (4.0 == 2.0);s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });
        if (((s.b[915] && s.b[921]) && (!s.b[922])) && s.b[923]) {s.store_scalar(648, 2.0);}
        s.b[924] = (4.0 == 4.0);s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
        if ((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && s.b[924]) {s.store_scalar(648, 3.0);}
        s.b[925] = (4.0 == 8.0);s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if (((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && (!s.b[924])) && s.b[925]) {s.store_scalar(648, 4.0);}
        if (s.b[915] && s.b[921]) {s.store_scalar(647, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((s.b[915] && s.b[921]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[915] && s.b[921]) {s.store_sqrt(646, 646);s.store_primal_offset(647, 647, 1.0);}
        }
        if (s.b[915] && (!s.b[921])) {s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));}
        if s.b[915] {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_scaled_mul(291, 290, 646, 1.0);s.store_div_scaled_product_offset_denominator_indices(280, 645, 646, 1.0, 220, 1e-50, 1.0);s.store_scale(106, 122, ((2.0 * s.v[453]) * p.p5));s.copy_ad(279, 386);s.store_div_scaled_product_mixed_aii(116, A::mul3(s.ad_value(106), s.ad_value(158), s.ad_value(115)), 291, 1.0, 279, 1.0);s.store_add(94, 94, 116);}
        s.b[926] = ((p.p20 != 0.0) && (p.p23 != 0.0));s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });
        if s.b[926] {s.store_square(231, 86);s.store_mul3_affine_lhs(232, 122, 271, 2.0, 0.0, 151);s.store_sub(233, 231, 232);s.store_sqrt_square_offset(639, 231, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(278, 231, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(231, 231, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[927] = (s.v[231] < 0.0);s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });
        if (s.b[926] && s.b[927]) {s.store_scalar(231, 0.0);s.store_scalar(278, 0.0);}
        if s.b[926] {s.store_sqrt_square_offset(639, 233, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(278, 233, 639, 0.5, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[926] {s.store_offset_add_scaled_inputs_indices(233, 233, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[928] = (s.v[233] < 0.0);s.store_scalar(928, if s.b[928] { 1.0 } else { 0.0 });
        if (s.b[926] && s.b[928]) {s.store_scalar(233, 0.0);s.store_scalar(278, 0.0);}
        if s.b[926] {s.store_sub(234, 231, 233);}
        s.b[929] = ((s.v[149] < (10.0 * 2.220446049250313e-16)) || (s.v[234] < (10.0 * 2.220446049250313e-16)));s.store_scalar(929, if s.b[929] { 1.0 } else { 0.0 });
        if (s.b[926] && s.b[929]) {s.store_scalar(35, 0.0);}
        if (s.b[926] && (!s.b[929])) {s.store_scalar(35, 1.0);}
        s.b[930] = (s.v[185] > 0.0);s.store_scalar(930, if s.b[930] { 1.0 } else { 0.0 });
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
        if s.b[933] {s.store_sqrt(67, 69);s.store_mul(68, 69, 67);s.store_div_scaled_product_indices(279, 120, 188, 1.0, 64, 1.0);s.store_div_scaled_product_indices(280, 120, 188, 1.0, 69, 1.0);s.store_mul_mixed_ia(190, 141, A::add_scaled_products(s.ad_value(68), s.ad_value(280), 1.0, s.ad_value(66), s.ad_value(279), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(191, 141, 67, 280, ((-1.0) * (0.5)), 65, 279, 0.5);s.store_add(192, 190, 191);s.store_mul3_lhs(193, 189, 192, 158);}
        s.store_scalar(949, (s.v[272] * 100.0));s.store_scale(951, 123, 100.0);s.store_scalar(952, (s.v[466] * 100.0));s.store_scale(953, 160, 0.01);s.b[956] = (p.p17 == 0.0);s.store_scalar(956, if s.b[956] { 1.0 } else { 0.0 });
        if s.b[956] {s.store_scalar(256, 0.0);}
        s.b[957] = (s.v[34] == 0.0);s.store_scalar(957, if s.b[957] { 1.0 } else { 0.0 });
        if ((!s.b[956]) && s.b[957]) {s.store_offset_add(948, 74, 71, (-(10.0 * 2.220446049250313e-16)));s.store_add_scaled_inputs4_mixed_iiai(938, 72, 1.0, 138, (-p.p256), A::div_scaled_inputs3(s.ad_value(50), (-p.p258), s.ad_value(80), p.p206, s.ad_value(267), (-p.p206), s.ad_value(951), 1.0), 1.0, 948, (-p.p205));s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[956]) && s.b[957]) {s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[958] = (s.v[947] < 0.0);s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && s.b[957]) && s.b[958]) {s.store_scalar(947, 0.0);s.store_scalar(942, 0.0);}
        if ((!s.b[956]) && s.b[957]) {s.store_sqrt_square_offset(639, 72, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(941, 72, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(940, 72, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[959] = (s.v[940] < 0.0);s.store_scalar(959, if s.b[959] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && s.b[957]) && s.b[959]) {s.store_scalar(940, 0.0);s.store_scalar(941, 0.0);}
        if ((!s.b[956]) && s.b[957]) {s.store_scaled_offset(936, 940, (-p.p216), 10.0);s.store_sub_from_scalar_ad(938, 1.0, A::div_scalar_offset_denominator(1.0, A::square(s.ad_value(936)), 1.0, 1.0));s.store_mul(947, 947, 938);s.store_scale(937, 951, s.v[952]);s.store_div_from_scalar_offset_input(944, p.p209, 937, p.p209);s.store_div_from_scalar_offset_square(941, 1.0, 947, 1e-50);s.store_scaled_mul(938, 246, 941, (-p.p204));}
        s.b[960] = (s.v[938] < (-34.0));s.store_scalar(960, if s.b[960] { 1.0 } else { 0.0 });
        if (((!s.b[956]) && s.b[957]) && (!s.b[960])) {s.store_mul_scale_offset_mixed_ia(940, 937, A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19, 0.0);}
        if (!s.b[956]) {s.store_offset_scaled(937, 52, (-p.p211), p.p212);s.store_exp_scaled_input(939, 937, s.v[949]);s.store_scale(938, 52, p.p260);s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));s.store_mul_square_lhs(940, 938, 937);s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));s.store_sub(942, 52, 51);s.store_offset_scaled(937, 942, (-p.p211), p.p212);s.store_exp_scaled_input(939, 937, s.v[949]);s.store_scale(938, 942, p.p260);s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));s.store_mul_square_lhs(940, 938, 937);s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p261, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p215, 1.0 / (s.v[949]));s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[963] = (s.v[947] < 0.0);s.store_scalar(963, if s.b[963] { 1.0 } else { 0.0 });
        if ((!s.b[956]) && s.b[963]) {s.store_scalar(947, 0.0);s.store_scalar(942, 0.0);}
        if (!s.b[956]) {s.store_offset(947, 947, 1e-50);s.store_div_from_scalar_powf_ad(938, (-p.p214), s.ad_value(947), p.p263);}
        s.b[964] = (s.v[938] < (-34.0));s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });
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
        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {s.store_exp(939, 938);s.store_scalar(940, (s.v[375] + p.p272));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {s.store_sub_scaled_inputs_mixed_ai(638, A::offset(s.ad_value(940), (-p.p273)), 1.0, 940, 0.001);s.store_scale(639, 940, (0.001 * (4.0 * p.p273)));}
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
        if ((!s.b[956]) && (!s.b[964])) {s.store_sqrt_square_add(639, 638, 639);}
        if (!s.b[956]) {s.store_scalar(256, 0.5);}
        s.b[968] = (p.p18 == 0.0);s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
        if (!s.b[968]) {s.store_add_scaled_inputs4_offset_indices(279, 51, p.p198, 52, (-1.0), 82, (-p.p200), 266, (-p.p200), (p.p199 * p.p198));s.store_scale(247, 279, 1.0 / (p.p228));s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(248, 247, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[969] = (s.v[248] < 0.0);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
        if ((!s.b[968]) && s.b[969]) {s.store_scalar(248, 0.0);s.store_scalar(283, 0.0);}
        if (!s.b[968]) {s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(248), 1e-50, 1.0);}
        s.b[970] = (s.v[278] < (-34.0));s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
        if ((!s.b[968]) && (!s.b[970])) {s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));}
        s.b[971] = (p.p18 == 0.0);s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
        if (!s.b[971]) {s.store_add_scaled_inputs3_mixed_aii(279, A::add_scaled_inputs3_offset(s.ad_value(51), (-p.p198), s.ad_value(52), -1.0, s.ad_value(51), 1.0, ((p.p199) * (p.p198))), 1.0, 82, (-p.p200), 266, (-p.p200));s.store_scale(247, 279, 1.0 / (p.p228));s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(249, 247, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[972] = (s.v[249] < 0.0);s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });
        if ((!s.b[971]) && s.b[972]) {s.store_scalar(249, 0.0);s.store_scalar(283, 0.0);}
        if (!s.b[971]) {s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(249), 1e-50, 1.0);}
        s.b[973] = (s.v[278] < (-34.0));s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });
        if ((!s.b[971]) && (!s.b[973])) {s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));}
        s.store_scalar(264, p.p176);s.store_scalar(261, 0.0);s.b[974] = (s.v[34] != 0.0);s.store_scalar(974, if s.b[974] { 1.0 } else { 0.0 });
        if s.b[974] {s.store_add(280, 51, 56);s.store_add_scaled_inputs(260, 280, s.v[264], 57, (1.0 - s.v[264]));}
        s.b[975] = (s.v[260] > ((s.v[56] + s.v[51]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(975, if s.b[975] { 1.0 } else { 0.0 });
        if (s.b[974] && s.b[975]) {s.store_offset_add(260, 56, 51, (-(10.0 * 2.220446049250313e-16)));}
        s.b[976] = (p.p45 != 0.0);s.store_scalar(976, if s.b[976] { 1.0 } else { 0.0 });s.b[977] = (s.v[151] > 1e-15);s.store_scalar(977, if s.b[977] { 1.0 } else { 0.0 });
        if (((!s.b[974]) && s.b[976]) && s.b[977]) {s.store_div_scaled_product_by_product_indices(261, 151, 122, 1.0, 123, 149, 1.0);}
        s.store_scalar(435, s.v[273]);s.store_scalar(436, (1.0 / s.v[435]));s.b[978] = (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (s.v[624] > 0.0));s.store_scalar(978, if s.b[978] { 1.0 } else { 0.0 });
        if s.b[978] {s.store_scalar(195, p.p175);s.store_mul_sqrt_mixed_ia(437, 141, A::div_from_scalar(s.v[624], s.ad_value(457)));s.store_scalar(399, ((1.0 - -1.0) / 2.0));s.store_scalar(400, ((1.0 + -1.0) / 2.0));s.store_primal_add_scaled_products_indices(402, 399, 412, 1.0, 400, 413, 1.0);s.store_primal_add_scaled_products_indices(403, 399, 413, 1.0, 400, 412, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[978] && (s.v[399] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if (s.b[978] && (s.v[400] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if s.b[978] {s.store_scalar(415, 0.0);s.store_neg(278, 415);}
        s.b[979] = (s.v[278] > s.v[31]);s.store_scalar(979, if s.b[979] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[979]) {s.store_sub(279, 278, 31);s.store_sub_from_scalar(280, s.v[30], 31);s.store_div(638, 279, 280);s.store_square(639, 638);s.store_mul(640, 639, 638);s.store_square(641, 639);s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);s.store_mul_scale_offset_indices(291, 280, 291, -1.0, 1.0);s.store_neg(387, 387);s.store_add(288, 31, 291);}
        if (s.b[978] && (!s.b[979])) {s.copy_ad(288, 278);}
        if s.b[978] {s.store_offset_scaled(416, 288, -1.0, (-1e-12));s.store_scale(144, 437, s.v[436]);s.store_square(145, 144);s.store_sub_from_scalar(404, p.p39, 414);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(417, 2.0, 120, A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));s.store_neg(419, 416);}
        s.b[980] = (s.v[404] < s.v[419]);s.store_scalar(980, if s.b[980] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[980]) {s.store_div_scalar_by_product_indices(291, s.v[435], 120, 437, 1.0);s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);s.store_sub(176, 137, 417);s.store_mul_add_rhs(290, 120, 404, 416);s.store_sub_from_scalar_scaled_mul_mixed_ia(183, (7.0 * 1.414213562373095), 291, A::offset(s.ad_value(290), (-2.0)), 9.0);s.store_square(181, 183);}
        s.b[981] = (s.v[182] < (s.v[181] * 1e-8));s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });
        if ((s.b[978] && s.b[980]) && s.b[981]) {s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);}
        if ((s.b[978] && s.b[980]) && (!s.b[981])) {s.store_sqrt_add(180, 182, 181);s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);}
        if (s.b[978] && s.b[980]) {s.store_powf(178, 179, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);s.store_div(77, 177, 178);s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);s.store_add(279, 259, 416);s.store_div(280, 279, 176);s.store_sub_div_lhs_mixed_ia(410, 279, A::sqrt_square_offset(s.ad_value(280), 1.0), 416);s.store_scaled_sub(408, 404, 410, s.v[435]);s.copy_ad(407, 408);}
        if (s.b[978] && (!s.b[980])) {s.store_scalar(77, 3.0);s.store_sub_div_lhs_indices(319, 77, 120, 416);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[978] && (!s.b[980])) {s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[982] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(982, if s.b[982] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[982]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[980])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[983] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[983]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[980])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[984] = (s.v[77] < 3.0);s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[984]) {s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(425, 404, -1.0, 416, -1.0, 144, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), 425, 1.0, 421, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), 1.0, 421, 421, 9.0);s.store_sqrt_add_scaled_square_cube_product(283, 426, 1.0, 424, 1.0);s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);s.store_neg_powf_add_input(428, 426, 283, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(290, 427, 1.0, 428, 1.0, 422, 1.0, 421, 3.0, -1.0);s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[985] = (p.p30 > 0.0);s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[980])) && s.b[985]) {s.store_offset_add(420, 404, 416, 0.1);s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);s.store_mul(430, 429, 203);s.store_mul(278, 121, 145);s.store_mul(434, 120, 420);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[980])) && s.b[985]) {s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);s.store_offset_sub(638, 434, 433, (-1.0));s.store_scale(639, 434, 4.0);}
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
        if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);s.store_mul(204, 429, 203);s.store_scalar(379, 0.0);s.store_scalar(62, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
    ) {
        let mut t6: usize = 0;
        while {
            let t4: f64 = (40.0 + 1.0);let t5: f64 = if (((s.b[978] && (!s.b[980])) && s.b[987]) && (s.v[62] <= t4)) { 1.0 } else { 0.0 };
            t5 != 0.0
        } {
            t6 += 1;assert!(t6 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
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
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[992]) {s.store_scalar(379, 1.0);}
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[994] = (s.v[77] < 5.0);s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[994]) {s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));}
        if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[994])) {s.store_offset(64, 77, (-1.0));s.store_sqrt(65, 64);}
        if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_mul(407, 437, 65);s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_mul3_lhs(409, 437, 207, 279);s.store_add(408, 407, 409);}
        if s.b[978] {s.store_sub(409, 408, 407);s.store_scale(282, 195, s.v[513]);}
        if (s.b[978] && (s.v[402] != 0.0)) {s.store_mul(398, 282, 408);s.store_mul(406, 282, 407);}
        if (s.b[978] && (s.v[403] != 0.0)) {s.store_mul(397, 282, 408);s.store_mul(405, 282, 407);}
        if s.b[978] {s.store_scalar(399, ((1.0 - 1.0) / 2.0));s.store_scalar(400, ((1.0 + 1.0) / 2.0));s.store_primal_add_scaled_products_indices(402, 399, 412, 1.0, 400, 413, 1.0);s.store_primal_add_scaled_products_indices(403, 399, 413, 1.0, 400, 412, 1.0);}
        if (s.b[978] && (s.v[399] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if (s.b[978] && (s.v[400] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if s.b[978] {s.store_scalar(415, 0.0);s.store_neg(278, 415);}
        s.b[996] = (s.v[278] > s.v[31]);s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[996]) {s.store_sub(279, 278, 31);s.store_sub_from_scalar(280, s.v[30], 31);s.store_div(638, 279, 280);s.store_square(639, 638);s.store_mul(640, 639, 638);s.store_square(641, 639);s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);s.store_mul_scale_offset_indices(291, 280, 291, -1.0, 1.0);s.store_neg(387, 387);s.store_add(288, 31, 291);}
        if (s.b[978] && (!s.b[996])) {s.copy_ad(288, 278);}
        if s.b[978] {s.store_offset_scaled(416, 288, -1.0, (-1e-12));s.store_scale(144, 437, s.v[436]);s.store_square(145, 144);s.store_sub_from_scalar(404, p.p39, 414);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(417, 2.0, 120, A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));s.store_neg(419, 416);}
        s.b[997] = (s.v[404] < s.v[419]);s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[997]) {s.store_div_scalar_by_product_indices(291, s.v[435], 120, 437, 1.0);s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);s.store_sub(176, 137, 417);s.store_mul_add_rhs(290, 120, 404, 416);s.store_sub_from_scalar_scaled_mul_mixed_ia(183, (7.0 * 1.414213562373095), 291, A::offset(s.ad_value(290), (-2.0)), 9.0);s.store_square(181, 183);}
        s.b[998] = (s.v[182] < (s.v[181] * 1e-8));s.store_scalar(998, if s.b[998] { 1.0 } else { 0.0 });
        if ((s.b[978] && s.b[997]) && s.b[998]) {s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);}
        if ((s.b[978] && s.b[997]) && (!s.b[998])) {s.store_sqrt_add(180, 182, 181);s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);}
        if (s.b[978] && s.b[997]) {s.store_powf(178, 179, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);s.store_div(77, 177, 178);s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);s.store_add(279, 259, 416);s.store_div(280, 279, 176);s.store_sub_div_lhs_mixed_ia(410, 279, A::sqrt_square_offset(s.ad_value(280), 1.0), 416);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[978] && s.b[997]) {s.store_scaled_sub(408, 404, 410, s.v[435]);s.copy_ad(407, 408);}
        if (s.b[978] && (!s.b[997])) {s.store_scalar(77, 3.0);s.store_sub_div_lhs_indices(319, 77, 120, 416);s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[999] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[999]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[997])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[1000] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1000]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[997])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[1001] = (s.v[77] < 3.0);s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1001]) {s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(425, 404, -1.0, 416, -1.0, 144, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), 425, 1.0, 421, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), 1.0, 421, 421, 9.0);s.store_sqrt_add_scaled_square_cube_product(283, 426, 1.0, 424, 1.0);s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);s.store_neg_powf_add_input(428, 426, 283, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(290, 427, 1.0, 428, 1.0, 422, 1.0, 421, 3.0, -1.0);s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[1002] = (p.p30 > 0.0);s.store_scalar(1002, if s.b[1002] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {s.store_offset_add(420, 404, 416, 0.1);s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {s.store_mul(430, 429, 203);s.store_mul(278, 121, 145);s.store_mul(434, 120, 420);s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);s.store_offset_sub(638, 434, 433, (-1.0));s.store_scale(639, 434, 4.0);}
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(433, 434, 1.0, 638, (-0.5), 639, (-0.5));s.store_sub(434, 434, 433);s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);s.store_sub_div_lhs_indices(320, 432, 120, 416);s.copy_ad(431, 77);s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));}
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(77, 432, 1.0, 638, (-0.5), 639, (-0.5));}
        if (s.b[978] && (!s.b[997])) {s.store_sub_div_lhs_indices(410, 77, 120, 416);s.store_add_offset_lhs_mixed_ia(279, 77, (-1.0), A::exp_scaled_input(s.ad_value(77), -1.0));}
        s.b[1003] = (s.v[279] < (10.0 * 2.220446049250313e-16));s.store_scalar(1003, if s.b[1003] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1003]) {s.store_scalar(279, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[997])) {s.store_mul_sqrt_rhs(407, 437, 279);s.store_scaled_sub(408, 404, 410, s.v[435]);}
        s.b[1004] = (p.p30 == 1.0);s.store_scalar(1004, if s.b[1004] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);s.store_mul(204, 429, 203);s.store_scalar(379, 0.0);s.store_scalar(62, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
    ) {
        let mut t9: usize = 0;
        while {
            let t7: f64 = (40.0 + 1.0);let t8: f64 = if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (s.v[62] <= t7)) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_mul_add_rhs(77, 120, 410, 416);}
            s.b[1005] = (s.v[77] < 5.0);s.store_scalar(1005, if s.b[1005] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1005]) {s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(207, 204, 205, 205);s.store_mul_product3_indices(208, 206, 204, 120, 205, 2.0);s.store_mul_scale_offset_mixed_ia(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(148, 77, A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, 208, 1.0, 209, 2.0);}
            s.b[1006] = (s.v[77] < 80.0);s.store_scalar(1006, if s.b[1006] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && s.b[1006]) {s.store_exp(147, 77);s.store_mul_scale_offset_indices(207, 204, 147, 1.0, (-1.0));s.store_mul3_lhs(208, 204, 120, 147);}
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && (!s.b[1006])) {s.store_exp_mul(202, 120, 410);s.store_mul_sub_rhs(207, 429, 202, 203);s.store_mul3_lhs(208, 429, 120, 202);}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) {s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);}
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));s.store_sub_from_scalar_scaled_mul(212, (-1.0), 144, 210, 1.0);}
            s.b[1007] = (s.v[379] == 1.0);s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1007]) {s.store_scalar(62, (40.0 + 1.0));}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {s.store_div_scaled_inputs_indices(213, 211, -1.0, 212, 1.0);}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1008] = (((s.v[213]) as f64).abs() > s.v[214]);s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1008]) {s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {s.store_add(410, 410, 213);}
            s.b[1009] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1009]) {s.store_scalar(379, 1.0);}
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[1011] = (s.v[77] < 5.0);s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1011]) {s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));}
        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1011])) {s.store_offset(64, 77, (-1.0));s.store_sqrt(65, 64);}
        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_mul(407, 437, 65);s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_mul3_lhs(409, 437, 207, 279);s.store_add(408, 407, 409);}
        if s.b[978] {s.store_sub(409, 408, 407);s.store_scale(282, 195, s.v[513]);}
        if (s.b[978] && (s.v[402] != 0.0)) {s.store_mul(398, 282, 408);s.store_mul(406, 282, 407);}
        if (s.b[978] && (s.v[403] != 0.0)) {s.store_mul(397, 282, 408);s.store_mul(405, 282, 407);}
        if s.b[978] {s.store_primal_add_scaled_inputs(194, 413, s.v[519], 412, s.v[518]);}
        if (s.b[978] && (s.v[194] != 0.0)) {s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);s.store_scale(198, 198, (-s.v[513]));s.store_offset_ad(197, A::mul_scaled_lhs(s.ad_value(198), -1.0, A::sub(s.ad_value(52), s.ad_value(51))), s.v[197]);}
        if s.b[978] {s.store_primal_add_scaled_inputs(194, 412, s.v[519], 413, s.v[518]);}
        if (s.b[978] && (s.v[194] != 0.0)) {s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);s.store_scale(199, 199, (-s.v[513]));s.store_offset_scaled_mul(196, 199, 52, -1.0, s.v[196]);}
        s.b[1013] = (((s.v[575] == 1.0) && (!s.b[518])) || ((s.v[575] != 1.0) && (!s.b[519])));s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });s.b[1014] = (p.p175 > 0.0);s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
        if (((!s.b[978]) && s.b[1013]) && s.b[1014]) {s.store_scalar(198, (((-s.v[435]) * p.p175) * s.v[513]));}
        if (((!s.b[978]) && s.b[1013]) && (!s.b[1014])) {s.store_scalar(198, 0.0);}
        if ((!s.b[978]) && (!s.b[1013])) {s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);s.store_scale(198, 198, (-s.v[513]));}
        if (!s.b[978]) {s.store_mul_sub_scaled_inputs_rhs_indices(197, 198, 52, -1.0, 51, -1.0);}
        s.b[1015] = (((s.v[575] == 1.0) && (!s.b[519])) || ((s.v[575] != 1.0) && (!s.b[518])));s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });
        if ((!s.b[978]) && s.b[1015]) {s.store_scalar(199, (((-s.v[435]) * p.p175) * s.v[513]));}
        if ((!s.b[978]) && (!s.b[1015])) {s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);s.store_scale(199, 199, (-s.v[513]));}
        if (!s.b[978]) {s.store_mul_scale_offset_indices(196, 52, 199, -1.0, 0.0);}
        s.b[1016] = (s.v[34] == 0.0);s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
        if ((s.v[38] != 0.0) && s.b[1016]) {s.store_scaled_mul(279, 386, 386, (p.p223 * p.p224));s.store_offset_ad(280, A::add_scaled_products(s.ad_value(158), s.ad_value(86), p.p223, s.ad_value(386), s.ad_value(386), p.p224), 1e-50);s.store_div(221, 279, 280);}
        if ((s.v[38] != 0.0) && (!s.b[1016])) {s.store_scalar(221, (p.p223 + 1e-50));}
        if (s.v[38] != 0.0) {s.store_scale(222, 270, (p.p225 * 0.0001));}
        s.b[1017] = ((p.p21 != 0.0) && (s.v[34] == 0.0));s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
        if s.b[1017] {s.store_scalar(223, s.v[617]);s.store_scalar(225, s.v[619]);s.store_scale(279, 149, 6.241449993689894e18);s.store_mul_scale_offset_mixed_ai(280, A::add_scaled_inputs3(s.ad_value(270), 1.0, A::div(s.ad_value(149), A::sub(s.ad_value(56), s.ad_value(50))), 1.0, s.ad_value(225), 1.0), 122, 6.241449993689894e18, 0.0);s.store_sub_mixed_ai(281, A::div_scaled_inputs(s.ad_value(91), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[513])), s.ad_value(386), 1.0), 279);}
        s.b[1018] = ((((s.v[281] - s.v[279])) as f64).abs() > (10.0 * 2.220446049250313e-16));s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1017] && s.b[1018]) {s.store_add_scaled_value_products_mixed_aaaai(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::sub(s.ad_value(281), s.ad_value(279)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(281), 1.0, s.ad_value(280), 1.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);}
        if (s.b[1017] && (!s.b[1018])) {s.store_add_scaled_inputs_product_mixed_aaai(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);}
        s.b[1019] = ((p.p23 != 0.0) && (s.v[34] == 0.0));s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
        if s.b[1019] {s.store_div_scaled_inputs2_indices(227, 260, 1.0, 56, (-1.0), 386, 1.0);s.store_scaled_mul(289, 159, 227, 1.0 / ((10000000.0 * 0.01)));}
        s.b[1020] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1020]) {s.store_scalar(285, 1.0);}
        s.b[1021] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {s.copy_ad(285, 289);}
        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {s.store_powf(285, 289, (p.p114 - 1.0));}
        if s.b[1019] {s.store_offset_mul(287, 289, 285, 1.0);s.store_powf(288, 287, (((-1.0) / p.p114) - 1.0));s.store_mul3_lhs(230, 159, 287, 288);s.store_scaled_add(228, 158, 230, 0.5);s.store_square(278, 85);}
    }
}
