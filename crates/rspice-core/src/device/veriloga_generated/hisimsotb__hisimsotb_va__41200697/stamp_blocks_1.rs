#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[725]) {s.store_offset_div_scaled_offset_numerator(290, A::mul(s.ad_value(120), A::sub(s.ad_value(76), s.ad_value(50))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        if (!s.b[725]) {
            if (s.v[290] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(290, (10.0 * 2.220446049250313e-16));
            }
        }
        if (!s.b[725]) {s.store_add_product3_rhs_mixed_iia(319, 76, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5);s.store_div_from_scalar(278, 1.0, 270);s.store_scalar(279, (p.p227 / 1.034943e-10));s.store_scalar(280, (1.0 / s.v[294]));s.store_div_from_scalar_ad(281, 1.0, A::add_scaled_inputs3(s.ad_value(278), 1.0, s.ad_value(279), 1.0, s.ad_value(280), 1.0));}
        s.b[743] = ((s.v[52] - s.v[327]) <= s.v[78]);s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[743]) {
            if (s.v[319] > 0.0) {
                s.store_sqrt_mul_scaled_lhs(283, 471, ((1.6021918e-19 * 2.0) * 1.034943e-10), 319);
            } else {
                s.store_scalar(283, 0.0);
            }
        }
        if ((!s.b[725]) && s.b[743]) {
            if (s.v[296] <= s.v[283]) {
                s.copy_ad(283, 296);
            } else {
            }
        }
        if ((!s.b[725]) && s.b[743]) {s.store_mul_mixed_ia(282, 281, A::add_scaled_inputs_product(s.ad_value(76), 1.0, s.ad_value(440), (-1.0), A::add_scaled_inputs(s.ad_value(280), 1.0, s.ad_value(279), 0.5), s.ad_value(283), -1.0));}
        if ((!s.b[725]) && (!s.b[743])) {s.store_mul_mixed_ia(282, 281, A::add_scaled_inputs_product(s.ad_value(76), 1.0, s.ad_value(440), (-1.0), A::add_scaled_inputs(s.ad_value(280), 1.0, s.ad_value(279), 0.5), s.ad_value(296), -1.0));}
        if (!s.b[725]) {s.store_sub_div_rhs_indices(319, 76, 282, 270);s.copy_ad(321, 319);}
        s.b[744] = ((s.v[52] - s.v[327]) > s.v[78]);s.store_scalar(744, if s.b[744] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[744]) {s.store_div_scalar_by_product_indices(279, 1.0, 142, 381, 1.0);s.store_mul_ad_product_rhs(280, 279, A::sub(s.ad_value(76), s.ad_value(327)), A::sub(s.ad_value(76), s.ad_value(327)));s.store_add_mixed_ia(281, 120, A::div_from_scalar(2.0, A::sub(s.ad_value(76), s.ad_value(327))));s.store_div_ln_lhs(320, 280, 281);}
        s.b[745] = ((s.v[319] > (s.v[320] - 0.15)) && (0.15 >= 0.0));s.store_scalar(745, if s.b[745] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && s.b[744]) && s.b[745]) {s.store_offset_sub(638, 319, 320, 0.15);s.store_square(642, 638);s.store_scalar(643, (0.15 * 0.15));s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);}
        let (tb,) = {
    if (((!s.b[725]) && s.b[744]) && s.b[745]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, tb);
        let (tc,) = {
    if (((!s.b[725]) && s.b[744]) && s.b[745]) {
        (0.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, tc);
        if (((!s.b[725]) && s.b[744]) && s.b[745]) {s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[746] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(746, if s.b[746] { 1.0 } else { 0.0 });s.b[747] = (1.0 == 1.0);s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (td,) = {
    if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && s.b[747]) {
        (1.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, td);s.b[748] = (1.0 == 2.0);s.store_scalar(748, if s.b[748] { 1.0 } else { 0.0 });
        let (te,) = {
    if ((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && s.b[748]) {
        (2.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, te);s.b[749] = (1.0 == 4.0);s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });
        let (tf,) = {
    if (((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && s.b[749]) {
        (3.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, tf);s.b[750] = (1.0 == 8.0);s.store_scalar(750, if s.b[750] { 1.0 } else { 0.0 });
        let (t10,) = {
    if ((((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && (!s.b[749])) && s.b[750]) {
        (4.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t10);
        let (t11,) = {
    if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, t11);let mut t15: usize = 0;
        while {
            let t14: f64 = if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;assert!(t15 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {s.store_sqrt(646, 646);}
            let (t13,) = {
    if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {
        let t12: f64 = (s.v[647] + 1.0);
        (t12,)
    } else {
        (s.v[647],)
    }
};
            s.store_scalar(647, t13);
        }
        if ((((!s.b[725]) && s.b[744]) && s.b[745]) && (!s.b[746])) {s.store_powf(646, 646, (1.0 / 2.0));}
        if (((!s.b[725]) && s.b[744]) && s.b[745]) {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_scaled_mul(637, 638, 646, 0.15);s.store_div_scaled_product_offset_denominator_indices(279, 645, 646, 0.15, 220, 1e-50, 1.0);s.store_add_offset_lhs(321, 320, (-0.15), 637);}
        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
        }
        if (((!s.b[725]) && s.b[744]) && (!s.b[745])) {s.copy_ad(321, 319);s.store_scalar(279, 1.0);}
        let (t1c,) = {
    if (!s.b[725]) {
        let (t1b,) = {
            if (s.v[321] > 0.0) {
                let t16: f64 = (2.0 * 1.034943e-10);let t17: f64 = (t16 / 1.6021918e-19);let t18: f64 = (t17 * s.v[321]);let t19: f64 = (t18 / s.v[471]);let t1a: f64 = (t19).sqrt();
                (t1a,)
            } else {
                (0.0,)
            }
        };
        (t1b,)
    } else {
        (s.v[345],)
    }
};
        s.store_scalar(345, t1c);s.b[751] = (s.v[345] < p.p227);s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });
        let (t1d,) = {
    if ((!s.b[725]) && s.b[751]) {
        (1.0,)
    } else {
        (s.v[39],)
    }
};
        s.store_scalar(39, t1d);
        let (t1e,) = {
    if ((!s.b[725]) && (!s.b[751])) {
        (2.0,)
    } else {
        (s.v[39],)
    }
};
        s.store_scalar(39, t1e);
        if (!s.b[725]) {s.copy_ad(305, 321);s.copy_ad(58, 319);s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));}
        s.b[752] = (s.v[39] == 1.0);s.store_scalar(752, if s.b[752] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[752]) {s.store_neg(279, 440);s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if ((!s.b[725]) && s.b[752]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[725]) && s.b[752]) {s.store_sqrt(280, 280);s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);s.store_scaled_sub(324, 281, 280, 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[753] = (s.v[324] < s.v[326]);s.store_scalar(753, if s.b[753] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && s.b[752]) && s.b[753]) {s.copy_ad(307, 324);}
        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        if ((!s.b[725]) && (!s.b[752])) {s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 305, (-(-1.0)), 296, (-(-(0.5 * (p.p227 * 9662367879.197212)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
    ) {
        if ((!s.b[725]) && (!s.b[752])) {s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if ((!s.b[725]) && (!s.b[752])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[725]) && (!s.b[752])) {s.store_sqrt(280, 280);s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);s.store_scaled_sub(324, 281, 280, 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[754] = (s.v[324] < s.v[326]);s.store_scalar(754, if s.b[754] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[752])) && s.b[754]) {s.copy_ad(307, 324);}
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        s.b[755] = ((s.v[39] == 1.0) && (0.0 != 0.0));s.store_scalar(755, if s.b[755] { 1.0 } else { 0.0 });
        let (t1f,) = {
    if ((!s.b[725]) && s.b[755]) {
        (1.0,)
    } else {
        (s.v[39],)
    }
};
        s.store_scalar(39, t1f);
        if ((!s.b[725]) && s.b[755]) {s.store_scalar(62, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        let mut t21: usize = 0;
        while {
            let t20: f64 = if (((!s.b[725]) && s.b[755]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;assert!(t21 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[755]) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[756] = (s.v[307] > 1e-8);s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[755]) && s.b[756]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[757] = (s.v[307] < (-1e-8));s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && s.b[757]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && (!s.b[757])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if ((!s.b[725]) && s.b[755]) {s.store_sub_mixed_ia(284, 307, A::div_scaled_inputs3(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(307), (-1.0), s.ad_value(440), -1.0, A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0));}
            s.b[758] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[755]) && s.b[758]) {s.copy_ad(285, 62);s.store_scalar(62, s.v[28]);}
            if ((!s.b[725]) && s.b[755]) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        if ((!s.b[725]) && s.b[755]) {s.store_add(307, 440, 307);s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));}
        let (t22,) = {
    if ((!s.b[725]) && (!s.b[755])) {
        (2.0,)
    } else {
        (s.v[39],)
    }
};
        s.store_scalar(39, t22);s.b[759] = (0.0 == 0.0);s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });
        let (t24,) = {
    if (((!s.b[725]) && (!s.b[755])) && s.b[759]) {
        let t23: f64 = (1e-12 * 100.0);
        (t23,)
    } else {
        (s.v[315],)
    }
};
        s.store_scalar(315, t24);
        if (((!s.b[725]) && (!s.b[755])) && s.b[759]) {s.copy_ad(56, 319);}
        let (t25,) = {
    if (((!s.b[725]) && (!s.b[755])) && (!s.b[759])) {
        (0.001,)
    } else {
        (s.v[315],)
    }
};
        s.store_scalar(315, t25);
        if (((!s.b[725]) && (!s.b[755])) && (!s.b[759])) {s.copy_ad(56, 305);}
        if ((!s.b[725]) && (!s.b[755])) {s.store_scalar(62, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t27: usize = 0;
        while {
            let t26: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[760] = (s.v[307] > 1e-8);s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[760]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[761] = (s.v[307] < (-1e-8));s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && s.b[761]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && (!s.b[761])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if ((!s.b[725]) && (!s.b[755])) {s.store_sub_div_rhs_ad(284, 307, A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));}
            s.b[762] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);s.store_scalar(762, if s.b[762] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[762]) {s.copy_ad(285, 62);s.store_scalar(62, s.v[28]);}
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[763] = (0.0 == 0.0);s.store_scalar(763, if s.b[763] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[755])) && s.b[763]) {s.copy_ad(316, 312);}
        s.b[764] = (1.0 == 0.0);s.store_scalar(764, if s.b[764] { 1.0 } else { 0.0 });
        let (t29,) = {
    if (((!s.b[725]) && (!s.b[755])) && s.b[764]) {
        let t28: f64 = (1e-12 * 100.0);
        (t28,)
    } else {
        (s.v[315],)
    }
};
        s.store_scalar(315, t29);
        if (((!s.b[725]) && (!s.b[755])) && s.b[764]) {s.copy_ad(56, 319);}
        let (t2a,) = {
    if (((!s.b[725]) && (!s.b[755])) && (!s.b[764])) {
        (0.001,)
    } else {
        (s.v[315],)
    }
};
        s.store_scalar(315, t2a);
        if (((!s.b[725]) && (!s.b[755])) && (!s.b[764])) {s.copy_ad(56, 305);}
        if ((!s.b[725]) && (!s.b[755])) {s.store_scalar(62, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t2c: usize = 0;
        while {
            let t2b: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;assert!(t2c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[765] = (s.v[307] > 1e-8);s.store_scalar(765, if s.b[765] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[765]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[766] = (s.v[307] < (-1e-8));s.store_scalar(766, if s.b[766] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && s.b[766]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && (!s.b[766])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if ((!s.b[725]) && (!s.b[755])) {s.store_sub_div_rhs_ad(284, 307, A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));}
            s.b[767] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);s.store_scalar(767, if s.b[767] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[767]) {s.copy_ad(285, 62);s.store_scalar(62, s.v[28]);}
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[768] = (1.0 == 0.0);s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[755])) && s.b[768]) {s.copy_ad(316, 312);}
        if ((!s.b[725]) && (!s.b[755])) {s.store_scalar(63, 0.0);}
        if (!s.b[725]) {s.store_offset_add(307, 440, 307, (-0.01));s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));}
        s.b[769] = ((s.v[306] > (s.v[305] - 0.15)) && (0.15 >= 0.0));s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[769]) {s.store_offset_sub(638, 306, 305, 0.15);s.store_square(642, 638);s.store_scalar(643, (0.15 * 0.15));s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);}
        let (t2d,) = {
    if ((!s.b[725]) && s.b[769]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, t2d);
        let (t2e,) = {
    if ((!s.b[725]) && s.b[769]) {
        (0.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t2e);
        if ((!s.b[725]) && s.b[769]) {s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[770] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(770, if s.b[770] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[771] = (1.0 == 1.0);s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });
        let (t2f,) = {
    if ((((!s.b[725]) && s.b[769]) && s.b[770]) && s.b[771]) {
        (1.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t2f);s.b[772] = (1.0 == 2.0);s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });
        let (t30,) = {
    if (((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && s.b[772]) {
        (2.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t30);s.b[773] = (1.0 == 4.0);s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });
        let (t31,) = {
    if ((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && s.b[773]) {
        (3.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t31);s.b[774] = (1.0 == 8.0);s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });
        let (t32,) = {
    if (((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && (!s.b[773])) && s.b[774]) {
        (4.0,)
    } else {
        (s.v[648],)
    }
};
        s.store_scalar(648, t32);
        let (t33,) = {
    if (((!s.b[725]) && s.b[769]) && s.b[770]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, t33);let mut t37: usize = 0;
        while {
            let t36: f64 = if ((((!s.b[725]) && s.b[769]) && s.b[770]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            t36 != 0.0
        } {
            t37 += 1;assert!(t37 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[769]) && s.b[770]) {s.store_sqrt(646, 646);}
            let (t35,) = {
    if (((!s.b[725]) && s.b[769]) && s.b[770]) {
        let t34: f64 = (s.v[647] + 1.0);
        (t34,)
    } else {
        (s.v[647],)
    }
};
            s.store_scalar(647, t35);
        }
        if (((!s.b[725]) && s.b[769]) && (!s.b[770])) {s.store_powf(646, 646, (1.0 / 2.0));}
        if ((!s.b[725]) && s.b[769]) {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_scaled_mul(637, 638, 646, 0.15);s.store_div_scaled_product_offset_denominator_indices(278, 645, 646, 0.15, 220, 1e-50, 1.0);s.store_add_offset_lhs(306, 305, (-0.15), 637);}
        if ((!s.b[725]) && s.b[769]) {
        }
        if ((!s.b[725]) && (!s.b[769])) {
        }
        if ((!s.b[725]) && (!s.b[769])) {s.store_scalar(278, 1.0);}
        if (!s.b[725]) {s.copy_ad(522, 306);}
        s.b[775] = ((p.p15 == 1.0) && (s.v[52] > (s.v[54] + 0.2)));s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });
        if s.b[775] {s.store_scalar(389, s.v[559]);s.store_add_scaled_inputs4_indices(388, 72, 1.0, 389, (-1.0), 80, 1.0, 267, -1.0);s.store_scalar(32, p.p136);s.copy_ad(99, 388);s.store_sqrt_div_scaled_inputs(100, 471, ((2.0 * 1.6021918e-19) * 1.034943e-10), 120, 1.0);s.store_div_scaled_product_by_product_indices(101, 127, 127, 1.0, 471, 471, 1.0);s.store_div_scaled_product_by_product_indices(102, 100, 100, 1.0, 270, 270, 1.0);s.store_scaled_mul(103, 102, 120, 0.5);s.store_scaled_mul(104, 103, 120, 2.0);s.store_sqrt_offset_ad(105, A::div_scaled_offset_numerator(A::mul(s.ad_value(120), s.ad_value(99)), 4.0, ((-1.0) * 4.0), s.ad_value(104), 1.0), 1.0);s.store_add_mul_sub_from_scalar_rhs_indices(107, 99, 103, 1.0, 105);s.store_div_scalar_by_product_indices(108, 1.0, 101, 102, 1.0);s.store_div_ad(109, A::ln(A::mul(s.ad_value(108), A::square(s.ad_value(99)))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(99))));s.store_add_scaled_inputs3_indices(110, 109, 1.0, 107, (-1.0), 32, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(111, 109, 1.0, 110, (-0.5), A::add_scaled_square_product(s.ad_value(110), 1.0, s.ad_value(32), s.ad_value(109), 4.0), (-0.5));s.store_exp_mul(112, 120, 111);s.store_add_scaled_product_mixed_aii(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), 1.0, 101, 112, 1.0);s.store_offset_mul(114, 120, 111, (-1.0));}
        s.b[776] = ((s.v[113] > 0.0) && (s.v[114] > 0.0));s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });
        if (s.b[775] && s.b[776]) {s.store_sqrt_ad(113, A::add_scaled_product(A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), 1.0, s.ad_value(101), s.ad_value(112), 1.0));s.store_sqrt_offset_ad(114, A::mul(s.ad_value(120), s.ad_value(111)), (-1.0));s.store_mul_sub_rhs(115, 100, 113, 114);s.store_div_from_scalar(106, (2.0 * s.v[124]), 120);s.store_scalar(158, (300.0 * 0.0001));s.store_scalar(262, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[775] && s.b[776]) {s.store_neg_ad(279, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, s.ad_value(71))), (-1.0)));s.store_div_scaled_product_mixed_aia(116, A::mul3(s.ad_value(106), s.ad_value(158), s.ad_value(115)), 279, 1.0, A::sub(s.ad_value(123), s.ad_value(262)), 1.0);s.copy_ad(338, 116);s.copy_ad(339, 111);s.store_offset_div_scaled_offset_numerator(290, A::mul(s.ad_value(120), s.ad_value(76)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[777] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[777]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[775] && s.b[776]) {s.store_add_product3_rhs_mixed_iia(319, 76, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5);s.copy_ad(58, 319);s.store_sub(61, 319, 339);}
        s.b[778] = (s.v[61] < 0.0);s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[778]) {s.store_scalar(61, 0.0);}
        if (s.b[775] && s.b[776]) {s.store_scale(283, 61, (1.0 + 0.3));s.store_offset_sub(284, 283, 71, (-0.03));s.store_sqrt_add_scaled_square_input(285, 284, 1.0, 283, (4.0 * 0.03));s.store_add_scaled_inputs3_indices(60, 283, 1.0, 284, (-0.5), 285, (-0.5));}
        s.b[779] = (s.v[60] > s.v[61]);s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[779]) {s.copy_ad(60, 61);}
        if (s.b[775] && s.b[776]) {s.copy_ad(392, 60);s.store_scalar(796, (s.v[272] * 100.0));s.store_scalar(797, (s.v[466] * 100.0));s.store_scale(798, 123, 100.0);}
        s.b[799] = (p.p26 == 0.0);s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[799]) {s.store_scalar(390, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_scalar(391, 4.12);s.store_scaled_mul(780, 797, 798, (p.p141 * 1.6021918e-19));s.store_div(781, 780, 245);s.store_div_scaled_inputs_mixed_ai(782, A::offset(A::add_scaled_inputs4(s.ad_value(70), p.p144, s.ad_value(82), 1.0, s.ad_value(266), 1.0, s.ad_value(137), 1.0), p.p143), -1.0, 796, 1.0);s.store_scalar(514, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t3a: usize = 0;
        while {
            let t38: f64 = (100.0 - 1.0);let t39: f64 = if (((s.b[775] && s.b[776]) && (!s.b[799])) && (s.v[514] <= t38)) { 1.0 } else { 0.0 };
            t39 != 0.0
        } {
            t3a += 1;assert!(t3a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.copy_ad(783, 514);s.store_scalar(784, 100.0);s.store_primal_div(785, 783, 784);s.store_add_scaled_inputs3_mixed_iia(786, 53, 1.0, 73, 1.0, A::add_scaled_product(s.ad_value(339), 1.0, s.ad_value(392), s.ad_value(785), 1.0), -1.0);s.store_sub_from_scalar_div_indices(787, 1.0, 786, 391);s.store_add_div_rhs_indices(790, 782, 786, 796);s.store_square(788, 790);s.store_sqrt_square_offset(639, 787, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(787, 787, 0.5, 639, 0.5, (1e-10 * 0.001));}
            s.b[800] = (s.v[787] < 0.0);s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[800]) {s.store_scalar(787, 0.0);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_offset_scaled_ad(789, A::mul(A::sqrt(s.ad_value(787)), s.ad_value(787)), (-p.p142), p.p142);s.store_div_scaled_inputs_indices(791, 789, -1.0, 790, 1.0);}
            s.b[801] = (s.v[791] < (-34.0));s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[801]) {s.store_scalar(792, 0.0);}
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[801])) {s.store_exp(792, 791);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.copy_ad(793, 781);s.store_mul3_affine_lhs(794, 793, 789, (0.25 * 7.38905609893065), 0.0, 789);}
            s.b[802] = (((2.0 * s.v[790]) + s.v[789]) < 0.0);s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[802]) {s.copy_ad(393, 794);}
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) {s.store_mul3_lhs(795, 780, 788, 792);}
            s.b[803] = ((s.v[795] < s.v[794]) || (s.v[790] < 0.0));s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && s.b[803]) {s.copy_ad(393, 794);}
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && (!s.b[803])) {s.copy_ad(393, 795);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_add(390, 390, 393);}
            s.b[804] = (s.v[393] < 1e-9);s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[804]) {s.store_scalar(514, 100.0);s.store_scalar(62, s.v[28]);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_primal_offset(514, 514, 1.0);}
        }
        s.b[805] = ((s.v[488] <= 0.0) || (s.v[162] <= 0.0));s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[805]) {s.store_scalar(185, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[805])) {s.copy_ad(279, 388);s.store_square(285, 270);s.store_mul_div_from_scalar_lhs_ad_indices(282, 2.0, 472, 285);s.store_add_scaled_inputs3_indices(283, 279, 1.0, 122, (-1.0), 70, (-s.v[486]));s.store_offset_mul(284, 282, 283, 1.0);s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[806] = (s.v[284] < 0.0);s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });
        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[806]) {s.store_scalar(284, 0.0);s.store_scalar(287, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[805])) {s.store_offset(284, 284, 1e-50);s.store_add_scaled_inputs_mixed_ia(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))), 1.0);s.store_add_scaled_inputs3_indices(187, 71, p.p123, 339, 1.0, 186, (-(s.v[487] * s.v[485])));s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(187, 187, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[807] = (s.v[187] < 0.0);s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[807]) {s.store_scalar(187, 0.0);s.store_scalar(287, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[805])) {s.store_offset(187, 187, 1e-50);s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));s.store_mul3_affine_lhs(185, 187, 338, s.v[488], 0.0, 280);}
        s.b[808] = (p.p16 == 1.0);s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[808]) {s.store_scaled_exp_scaled_input(279, 120, (-p.p140), ((1.6021918e-19 * p.p227) * s.v[466]));s.store_offset_scaled(280, 471, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(282, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 279, 280, 1.0);s.store_mul_add_lhs(520, 185, 390, 282);s.store_mul_scaled_ln_offset_rhs(283, 122, p.p139, 520, 1.0);s.store_sqrt_mul_scaled_lhs(284, 471, ((2.0 * 1.034943e-10) * 1.6021918e-19), 122);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[775] && s.b[776]) && s.b[808]) {s.store_sqrt_ad(285, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, A::sub(s.ad_value(339), s.ad_value(283)))), (-1.0)), 1.0, s.ad_value(120), A::sub(s.ad_value(339), s.ad_value(283)), 1.0));s.store_sqrt_ad(286, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, s.ad_value(339))), (-1.0)), 1.0, s.ad_value(120), s.ad_value(339), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(337, 284, 285, -1.0, 286, -1.0);}
        if (((s.b[775] && s.b[776]) && s.b[808]) && (p.p27 != 0.0)) {s.store_div_from_scalar_offset_input(342, p.p137, 185, p.p138);s.store_mul(341, 342, 270);s.copy_ad(340, 337);s.store_scaled_voltage(562, ctx, nodes, Some(10), None, 1e-9);s.copy_ad(337, 562);s.store_div_scaled_inputs2_indices(558, 562, 1.0, 340, (-1.0), 341, 1.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[808])) {s.store_scalar(337, 0.0);}
        if (s.b[775] && (!s.b[776])) {s.store_scalar(185, 0.0);s.store_scalar(337, 0.0);}
        if (!s.b[775]) {s.store_scalar(185, 0.0);s.store_scalar(337, 0.0);}
        s.copy_ad(299, 305);s.copy_ad(300, 306);s.store_sub(301, 307, 440);s.store_scalar(379, 0.0);s.store_scalar(606, 1.0);s.store_scalar(604, 0.0);s.store_scalar(605, 0.0);s.b[809] = (s.v[649] < 4.0);s.store_scalar(809, if s.b[809] { 1.0 } else { 0.0 });
        if s.b[809] {s.copy_ad(599, 296);s.store_neg(600, 599);s.store_div_scalar_by_product_mixed_ai(601, 0.004832, A::square(s.ad_value(296)), 296, 1.0);s.store_scale(603, 296, (-3.7477));s.store_scale(602, 296, 4.3495);}
        if (!s.b[809]) {s.store_scale(599, 296, 1.5);s.store_neg(600, 599);s.store_div_scalar_by_product_mixed_ai(601, 0.001765, A::square(s.ad_value(296)), 296, 1.0);s.store_scale(603, 296, (-4.8303));s.store_scale(602, 296, 5.9661);}
        s.copy_ad(306, 300);s.copy_ad(534, 300);s.copy_ad(522, 534);s.copy_ad(307, 301);s.store_scalar(62, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        let mut t4: usize = 0;
        while {
            let t3: f64 = if s.v[62] <= s.v[28] { 1.0 } else { 0.0 };
            t3 != 0.0
        } {
            t4 += 1;assert!(t4 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.copy_ad(279, 307);s.store_mul(297, 120, 279);s.store_exp_neg_input(278, 297);s.b[810] = (s.v[279] < (-1e-8));s.store_scalar(810, if s.b[810] { 1.0 } else { 0.0 });
            if s.b[810] {s.store_exp_mul(280, 120, 307);s.store_mul_sqrt_mixed_ia(312, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 312, 1.0);}
            s.b[811] = (s.v[279] > (1e-8 / 10.0));s.store_scalar(811, if s.b[811] { 1.0 } else { 0.0 });
            if ((!s.b[810]) && s.b[811]) {s.store_exp_mul(280, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(312, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 312, 1.0);}
            if ((!s.b[810]) && (!s.b[811])) {s.store_scaled_mul(312, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(343, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));}
            s.store_add_scaled_inputs4_indices(306, 307, 1.0, 312, (-1.0 / (s.v[294])), 50, 1.0, 298, 1.0);s.store_sub_from_scalar_scaled_input(583, 1.0, 343, 1.0 / (s.v[294]));s.store_sub(279, 305, 522);s.store_mul(297, 120, 279);s.b[812] = ((-s.v[297]) >= 80.0);s.store_scalar(812, if s.b[812] { 1.0 } else { 0.0 });
            if s.b[812] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[812]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[813] = (s.v[279] < (-1e-8));s.store_scalar(813, if s.b[813] { 1.0 } else { 0.0 });
            if s.b[813] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(523, 141, 280);s.store_div_scaled_product3_mixed_iiai(524, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(525, 524);s.store_scalar(311, 0.0);s.store_scalar(526, 0.0);s.store_scalar(527, 0.0);}
            s.b[814] = (s.v[279] > 1e-8);s.store_scalar(814, if s.b[814] { 1.0 } else { 0.0 });
            if ((!s.b[813]) && s.b[814]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(523, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(524, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(525, 524);s.store_exp(278, 297);s.store_exp_mul(281, 120, 522);s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(523), s.ad_value(523), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(523), s.ad_value(524), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(523), s.ad_value(525), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(311, 523, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(526, 524, (-1.0), 141, 537, -1.0);s.store_add_scaled_product_indices(527, 525, (-1.0), 141, 538, -1.0);}
            if ((!s.b[813]) && (!s.b[814])) {s.store_scaled_mul(523, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(524, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(525, 524);s.store_scalar(311, 0.0);s.store_scalar(526, 0.0);s.store_scalar(527, 0.0);}
            s.store_sub(279, 306, 522);s.store_mul(297, 120, 279);s.b[815] = ((-s.v[297]) >= 80.0);s.store_scalar(815, if s.b[815] { 1.0 } else { 0.0 });
            if s.b[815] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[815]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[816] = (s.v[279] < (-1e-8));s.store_scalar(816, if s.b[816] { 1.0 } else { 0.0 });
            if s.b[816] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(531, 141, 280);s.store_div_scaled_product3_mixed_iiai(532, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(533, 532);s.store_scalar(528, 0.0);s.store_scalar(529, 0.0);s.store_scalar(530, 0.0);}
            s.b[817] = (s.v[279] > 1e-8);s.store_scalar(817, if s.b[817] { 1.0 } else { 0.0 });
            if ((!s.b[816]) && s.b[817]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(531, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(532, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(533, 532);s.store_exp(278, 297);s.store_exp_mul(281, 120, 522);s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(531), s.ad_value(531), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(531), s.ad_value(532), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(531), s.ad_value(533), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(528, 531, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(529, 532, (-1.0), 141, 539, -1.0);s.store_add_scaled_product_indices(530, 533, (-1.0), 141, 538, -1.0);}
            if ((!s.b[816]) && (!s.b[817])) {s.store_scaled_mul(531, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(532, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(533, 532);s.store_scalar(528, 0.0);s.store_scalar(529, 0.0);s.store_scalar(530, 0.0);}
            s.b[818] = (s.v[379] == 1.0);s.store_scalar(818, if s.b[818] { 1.0 } else { 0.0 });
            if s.b[818] {s.store_scalar(574, s.v[62]);s.store_scalar(62, s.v[28]);}
            if (!s.b[818]) {s.store_add_scaled_inputs3_mixed_iia(346, 305, 1.0, 76, (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(312), 1.0, s.ad_value(311), 1.0, s.ad_value(523), 1.0, s.ad_value(528), 1.0), s.ad_value(531)), s.ad_value(337)), s.ad_value(270)), -1.0);s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(526), 1.0, s.ad_value(524), 1.0, s.ad_value(270), 1.0));s.store_div_scaled_inputs_mixed_ai(348, A::add_scaled_inputs4(s.ad_value(527), 1.0, s.ad_value(525), 1.0, s.ad_value(530), 1.0, s.ad_value(533), 1.0), -1.0, 270, 1.0);s.store_div_scaled_inputs_mixed_ai(349, A::add_scaled_product(s.ad_value(343), 1.0, A::add(s.ad_value(529), s.ad_value(532)), s.ad_value(583), 1.0), -1.0, 270, 1.0);}
            s.b[819] = (s.v[312] <= s.v[599]);s.store_scalar(819, if s.b[819] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[819]) {s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(312), 2.0, s.ad_value(296), 1.0));s.store_div_scaled_product_indices(604, 296, 343, 1.0, 279, 1.0);}
            s.b[820] = (s.v[312] <= s.v[603]);s.store_scalar(820, if s.b[820] { 1.0 } else { 0.0 });
            if (((!s.b[818]) && (!s.b[819])) && s.b[820]) {s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(602)));s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(312), 3.0, s.ad_value(602), (-3.0), s.ad_value(312), 1.0, s.ad_value(603), (-1.0)), 343);}
            if (((!s.b[818]) && (!s.b[819])) && (!s.b[820])) {s.store_scalar(279, 0.0);s.store_scalar(604, 0.0);}
            if (!s.b[818]) {s.store_div_scaled_inputs_indices(281, 316, (-s.v[650]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_neg_add(279, 296, 280);s.store_scalar(604, 0.0);s.store_scaled_add(350, 523, 279, 1.0 / (s.v[535]));s.store_scale(351, 524, 1.0 / (s.v[535]));s.store_scale(352, 525, 1.0 / (s.v[535]));s.store_scale(353, 604, 1.0 / (s.v[535]));s.store_div_scaled_inputs_indices(281, 316, (-s.v[651]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_scalar(605, 0.0);s.store_scaled_add(354, 531, 280, 1.0 / (s.v[535]));s.store_scale(355, 533, 1.0 / (s.v[535]));s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 532, 583, 1.0 / (s.v[535]));s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);}
            s.b[821] = (s.v[357] > 0.0);s.store_scalar(821, if s.b[821] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[821]) {s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);}
            if ((!s.b[818]) && (!s.b[821])) {s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));}
            if (!s.b[818]) {s.store_add_scaled_products_indices(359, 352, 356, 1.0, 353, 355, (-1.0));s.store_add_scaled_products_indices(360, 349, 355, 1.0, 348, 356, (-1.0));s.store_add_scaled_products_indices(361, 348, 353, 1.0, 349, 352, (-1.0));s.store_mul_scale_offset_indices(362, 356, 351, -1.0, 0.0);s.store_mul(363, 347, 356);s.store_add_scaled_products_indices(364, 349, 351, 1.0, 347, 353, (-1.0));s.store_mul(365, 351, 355);s.store_mul_scale_offset_indices(366, 355, 347, -1.0, 0.0);s.store_add_scaled_products_indices(367, 347, 352, 1.0, 348, 351, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(368, 358, 359, 346, -1.0, 360, 350, -1.0, 361, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(369, 358, 362, 346, -1.0, 363, 350, -1.0, 364, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(370, 358, 365, 346, -1.0, 366, 350, -1.0, 367, 354, -1.0);s.store_abs(279, 368);}
            s.b[822] = (s.v[279] < ((s.v[369]) as f64).abs());s.store_scalar(822, if s.b[822] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[822]) {s.store_abs(279, 369);}
            s.b[823] = (s.v[279] < ((s.v[370]) as f64).abs());s.store_scalar(823, if s.b[823] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[823]) {s.store_abs(279, 370);}
            if (!s.b[818]) {s.store_scalar(606, 1.0);}
            s.b[824] = (s.v[62] > 80.0);s.store_scalar(824, if s.b[824] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[824]) {s.store_scalar(606, 25.0);}
            s.b[825] = (s.v[62] > 40.0);s.store_scalar(825, if s.b[825] { 1.0 } else { 0.0 });
            if (((!s.b[818]) && (!s.b[824])) && s.b[825]) {s.store_scalar(606, 25.0);}
            s.b[826] = (s.v[62] > 20.0);s.store_scalar(826, if s.b[826] { 1.0 } else { 0.0 });
            if ((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && s.b[826]) {s.store_scalar(606, 25.0);}
            s.b[827] = (s.v[62] > 10.0);s.store_scalar(827, if s.b[827] { 1.0 } else { 0.0 });
            if (((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && (!s.b[826])) && s.b[827]) {s.store_scalar(606, 5.0);}
            s.b[828] = (s.v[279] > (0.1 / s.v[606]));s.store_scalar(828, if s.b[828] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[828]) {s.store_mul_mixed_ia(368, 368, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(369, 369, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(370, 370, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));}
            if (!s.b[818]) {s.store_add(305, 305, 368);s.store_add(522, 522, 369);s.store_add(307, 307, 370);}
            let (t1,) = {
    if (!s.b[818]) {
        let t0: f64 = (1e-12 * s.v[606]);
        (t0,)
    } else {
        (s.v[607],)
    }
};
            s.store_scalar(607, t1);s.b[829] = (s.v[279] < s.v[607]);s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });
            let (t2,) = {
    if ((!s.b[818]) && s.b[829]) {
        (1.0,)
    } else {
        (s.v[379],)
    }
};
            s.store_scalar(379, t2);s.store_primal_offset(62, 62, 1.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        s.b[830] = (s.v[574] > 0.0);s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });
        if s.b[830] {s.copy_ad(62, 574);s.store_scalar(574, 0.0);}
        s.b[831] = (s.v[62] > s.v[28]);s.store_scalar(831, if s.b[831] { 1.0 } else { 0.0 });
        if s.b[831] {s.copy_ad(305, 299);s.copy_ad(306, 300);s.copy_ad(307, 301);s.copy_ad(522, 534);}
        s.copy_ad(56, 305);s.store_neg(149, 311);s.b[833] = (s.v[149] <= 1e-50);s.store_scalar(833, if s.b[833] { 1.0 } else { 0.0 });
        if s.b[833] {s.store_scalar(149, 1e-50);}
        let (t5,) = {
    if s.b[833] {
        (1.0,)
    } else {
        (s.v[34],)
    }
};
        s.store_scalar(34, t5);s.store_neg(150, 528);s.b[834] = (s.v[150] <= 1e-50);s.store_scalar(834, if s.b[834] { 1.0 } else { 0.0 });
        if s.b[834] {s.store_scalar(150, 1e-50);}
        s.store_mul(86, 149, 271);s.copy_ad(396, 51);s.store_div_square_rhs(280, 472, 270);s.store_sub(278, 76, 122);s.store_offset_mul_ad(287, A::div_from_scalar(2.0, s.ad_value(280)), s.ad_value(278), 1.0);s.store_sqrt_square_offset(639, 287, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(284, 287, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(287, 287, 0.5, 639, 0.5, (1e-10 * 0.05));s.b[835] = (s.v[287] < 0.0);s.store_scalar(835, if s.b[835] { 1.0 } else { 0.0 });
        if s.b[835] {s.store_scalar(287, 0.0);s.store_scalar(284, 0.0);}
        s.store_sqrt(281, 287);s.store_add_mul_sub_from_scalar_rhs_indices(288, 76, 280, 1.0, 281);s.store_sqrt_square_offset(639, 288, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(278, 288, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(288, 288, 0.5, 639, 0.5, (1e-10 * 0.01));s.b[836] = (s.v[288] < 0.0);s.store_scalar(836, if s.b[836] { 1.0 } else { 0.0 });
        if s.b[836] {s.store_scalar(288, 0.0);s.store_scalar(278, 0.0);}
        s.copy_ad(89, 288);s.store_offset_div(279, 51, 89, 1e-50);s.store_powf(280, 279, (s.v[481] - 1.0));s.store_offset_mul(281, 280, 279, 1.0);s.store_powf(282, 281, ((1.0 / s.v[481]) - 1.0));s.store_mul(284, 282, 281);s.store_div(395, 51, 284);s.copy_ad(51, 395);s.b[837] = (s.v[51] < 0.0);s.store_scalar(837, if s.b[837] { 1.0 } else { 0.0 });
        if s.b[837] {s.copy_ad(57, 56);s.store_sub(59, 57, 56);s.copy_ad(308, 57);s.copy_ad(309, 306);s.copy_ad(584, 522);s.copy_ad(310, 307);}
        let (t6,) = {
    if s.b[837] {
        (1.0,)
    } else {
        (s.v[379],)
    }
};
        s.store_scalar(379, t6);s.b[838] = ((s.v[33] >= 1.0) || (s.v[86] < 1e-12));s.store_scalar(838, if s.b[838] { 1.0 } else { 0.0 });
        if ((!s.b[837]) && s.b[838]) {s.store_scalar(308, s.v[698]);s.store_scalar(309, s.v[699]);s.store_offset(310, 440, s.v[700]);}
        if ((!s.b[837]) && (!s.b[838])) {
            if ((s.v[58] - s.v[305]) >= 0.0) {
                s.store_sub(61, 58, 305);
            } else {
                s.store_scalar(61, 0.0);
            }
        }
        if ((!s.b[837]) && (!s.b[838])) {s.store_offset_sub_scaled_inputs_indices(638, 61, (1.0 + (0.3 * 0.5)), 51, 1.0, (-0.03));s.store_scale(639, 61, ((1.0 + (0.3 * 0.5)) * (4.0 * 0.03)));}
        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((!s.b[837]) && (!s.b[838])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(60, 61, (1.0 + (0.3 * 0.5)), 638, (-0.5), 639, (-0.5));}
        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[60] <= s.v[61]) {
            } else {
                s.copy_ad(60, 61);
            }
        }
        s.b[839] = (s.v[60] < 0.0);s.store_scalar(839, if s.b[839] { 1.0 } else { 0.0 });
        if (((!s.b[837]) && (!s.b[838])) && s.b[839]) {s.store_scalar(60, 0.0);}
        s.b[840] = (s.v[60] > s.v[51]);s.store_scalar(840, if s.b[840] { 1.0 } else { 0.0 });
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[839])) && s.b[840]) {s.copy_ad(60, 51);}
        if ((!s.b[837]) && (!s.b[838])) {s.copy_ad(59, 60);s.store_add(57, 305, 59);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        if ((!s.b[837]) && (!s.b[838])) {s.store_scalar(290, (1e-12 / 2.0));}
        s.b[841] = (s.v[57] < s.v[290]);s.store_scalar(841, if s.b[841] { 1.0 } else { 0.0 });
        if (((!s.b[837]) && (!s.b[838])) && s.b[841]) {s.copy_ad(57, 290);}
        if ((!s.b[837]) && (!s.b[838])) {s.copy_ad(308, 57);}
        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[292] == (-1.0)) {
                s.copy_ad(308, 305);
            } else {
                s.copy_ad(308, 57);
            }
        }
        if ((!s.b[837]) && (!s.b[838])) {s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));}
        s.b[842] = (s.v[308] < s.v[329]);s.store_scalar(842, if s.b[842] { 1.0 } else { 0.0 });
        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {s.store_neg(279, 440);s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {s.store_scaled_sub_ad(324, A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::sqrt(s.ad_value(280)), 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[843] = (s.v[324] < s.v[326]);s.store_scalar(843, if s.b[843] { 1.0 } else { 0.0 });
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && s.b[843]) {s.copy_ad(310, 324);}
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(310, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 308, (-(-1.0)), 296, (-(-(0.5 * s.v[536]))));s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {s.store_scaled_sub_ad(324, A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::sqrt(s.ad_value(280)), 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[844] = (s.v[324] < s.v[326]);s.store_scalar(844, if s.b[844] { 1.0 } else { 0.0 });
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && s.b[844]) {s.copy_ad(310, 324);}
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(310, 325, 1.0, 638, (-0.5), 639, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        s.b[845] = ((s.v[308] < s.v[329]) && (0.0 != 0.0));s.store_scalar(845, if s.b[845] { 1.0 } else { 0.0 });
        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_scalar(63, 0.0);}
        let mut t8: usize = 0;
        while {
            let t7: f64 = if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            t7 != 0.0
        } {
            t8 += 1;assert!(t8 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_mul(280, 120, 310);s.store_exp_neg_input(281, 280);}
            s.b[846] = (s.v[310] > 1e-8);s.store_scalar(846, if s.b[846] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[846]) {s.store_exp_mul(278, 120, 310);s.store_mul_scaled_sqrt_ad_rhs(282, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[847] = (s.v[310] < (-1e-8));s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && s.b[847]) {s.store_mul_sqrt_mixed_ia(282, 439, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && (!s.b[847])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 310);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));}
            s.b[848] = (s.v[284] < 0.0);s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[848]) {s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));s.store_scale(639, 296, (-(4.0 * 1e-9)));}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));s.store_mul3_lhs(285, 285, 283, 286);s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);s.store_sub_mixed_ia(284, 310, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(310), (-1.0), s.ad_value(440), -1.0, s.ad_value(332), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(333)), 1.0));}
            s.b[849] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[849]) {s.store_scalar(63, s.v[29]);}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.copy_ad(310, 284);s.copy_ad(314, 282);s.store_primal_offset(63, 63, 1.0);}
        }
        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_add(310, 440, 310);s.store_sub_scaled_inputs(309, 310, 1.0, 314, 1.0 / (s.v[294]));}
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_scalar(63, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        let mut ta: usize = 0;
        while {
            let t9: f64 = if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;assert!(ta <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.copy_ad(279, 439);s.store_mul(280, 120, 310);s.store_exp_neg_input(281, 280);}
            s.b[850] = (s.v[310] > 1e-8);s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[850]) {s.store_exp_mul(278, 120, 310);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[851] = (s.v[310] < (-1e-8));s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && s.b[851]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && (!s.b[851])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 310);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));}
            s.b[852] = (s.v[284] < 0.0);s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[852]) {s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));s.store_scale(639, 296, (-(4.0 * 1e-9)));}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));s.store_mul3_lhs(285, 285, 283, 286);s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);s.store_sub_div_rhs_ad(284, 310, A::add(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(308), 1.0, s.ad_value(310), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), s.v[536], s.ad_value(296), (0.5 * s.v[536]))), s.ad_value(440)), s.ad_value(332)), A::add_scaled_inputs3_offset(s.ad_value(283), 1.0 / (s.v[294]), s.ad_value(283), s.v[536], s.ad_value(333), 1.0, (-1.0)));}
            s.b[853] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[853]) {s.store_scalar(63, s.v[29]);}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.copy_ad(310, 284);s.copy_ad(314, 282);s.store_primal_offset(63, 63, 1.0);}
        }
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_add(310, 440, 310);s.store_sub_scaled_inputs(309, 310, 1.0, 314, 1.0 / (s.v[294]));}
        if ((!s.b[837]) && (!s.b[838])) {s.copy_ad(584, 309);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        s.b[854] = (s.v[86] < 1e-12);s.store_scalar(854, if s.b[854] { 1.0 } else { 0.0 });
        if s.b[854] {s.copy_ad(302, 305);s.copy_ad(303, 306);s.copy_ad(304, 307);s.copy_ad(581, 522);}
        if (!s.b[854]) {s.copy_ad(302, 308);s.copy_ad(303, 309);s.store_sub(304, 310, 440);}
        if (!s.b[854]) {
            if (s.v[303] < s.v[302]) {
                s.copy_ad(581, 303);
            } else {
                s.copy_ad(581, 302);
            }
        }
        s.b[379] = (s.v[292] < 0.0);s.store_scalar(379, if s.b[379] { 1.0 } else { 0.0 });s.copy_ad(308, 302);s.copy_ad(309, 303);s.copy_ad(310, 304);s.copy_ad(584, 581);s.store_scalar(63, 1.0);
    }
}
