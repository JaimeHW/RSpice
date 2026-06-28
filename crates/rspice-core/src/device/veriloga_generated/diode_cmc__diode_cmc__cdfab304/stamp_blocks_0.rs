#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[0] = (8.8541878176e-12 * 11.8);

        s.v[1] = (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) });

        s.b[388] = ((!param_given[6]) && param_given[96]);
        s.v[388] = if s.b[388] { 1.0 } else { 0.0 };

        if s.b[388] {
            s.store_scalar(1, (if (p.p96 > (-250.0)) { p.p96 } else { (-250.0) }));
        }

        s.v[2] = (if (p.p5 > 1e-12) { p.p5 } else { 1e-12 });

        s.v[12] = p.p17;

        s.v[13] = p.p18;

        s.v[14] = p.p19;

        s.v[15] = (if (p.p20 > 0.0) { p.p20 } else { 0.0 });

        s.v[16] = (if (p.p21 > 0.0) { p.p21 } else { 0.0 });

        s.v[17] = (if (p.p22 > 0.0) { p.p22 } else { 0.0 });

        s.v[62] = (if (p.p63 > 0.1) { p.p63 } else { 0.1 });

        s.v[64] = (if (p.p64 > 0.1) { p.p64 } else { 0.1 });

        s.v[63] = (if (p.p65 > 0.1) { p.p65 } else { 0.1 });

        s.v[75] = (if (p.p76 > 0.1) { p.p76 } else { 0.1 });

        s.v[76] = (if (p.p77 > 0.0) { p.p77 } else { 0.0 });

        s.v[77] = (if (p.p78 > 0.0) { p.p78 } else { 0.0 });

        s.v[45] = 0.0;

        s.b[389] = (p.p81 > 0.5);
        s.v[389] = if s.b[389] { 1.0 } else { 0.0 };

        let (assign790_e598,) = {
    if s.b[389] {
        (1.0,)
    } else {
        (s.v[45],)
    }
};
        s.v[45] = assign790_e598;

        let (assign800_e603,) = {
    if (!s.b[389]) {
        (0.0,)
    } else {
        (s.v[45],)
    }
};
        s.v[45] = assign800_e603;

        s.v[46] = (if (p.p82 > 0.5) { p.p82 } else { 0.5 });

        s.store_offset(78, 1, 273.15);

        s.v[79] = ((ctx_temp + p.p102)).max((273.15 + (-250.0)));

        s.store_div_from_scalar(80, s.v[79], 78);

        s.v[81] = (1.3806505e-23 / 1.6021918e-19);

        s.store_scale(82, 78, s.v[81]);

        s.store_div_from_scalar(83, 1.0, 82);

        s.v[84] = (s.v[81] * s.v[79]);

        s.v[85] = (1.0 / s.v[84]);

        s.store_div_scaled_inputs(89, A::mul_scaled_lhs(s.ad_value(78), 0.000702, s.ad_value(78)), -1.0, A::offset(s.ad_value(78), 1108.0), 1.0);

        s.store_offset(92, 89, s.v[12]);

        s.store_offset(93, 89, s.v[13]);

        s.store_offset(94, 89, s.v[14]);

        s.v[90] = ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79]));

        s.v[95] = (s.v[12] + s.v[90]);

        s.v[96] = (s.v[13] + s.v[90]);

        s.v[97] = (s.v[14] + s.v[90]);

        s.store_mul_ad(176, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[62])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62]))));

        s.store_mul_ad(177, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[64])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64]))));

        s.store_mul_ad(178, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[63])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63]))));

        s.store_scaled_mul(101, 176, 176, s.v[15]);

        s.store_scaled_mul(102, 177, 177, s.v[16]);

        s.store_scaled_mul(103, 178, 178, s.v[17]);

        s.v[179] = (1.0 - (0.01 * s.v[77]));

        s.v[308] = (p.p87 * 1000000.0);

        s.v[310] = (p.p89 * 1000000.0);

        s.v[309] = (p.p88 * 1000000.0);

        s.v[307] = s.v[308];

        s.v[313] = s.v[62];

        s.v[311] = (1450.0 * 0.0001);

        s.v[312] = (500.0 * 0.0001);

        s.v[368] = 0.6;

        s.v[369] = 0.001;

        s.store_scale(318, 176, 1.45e16);

        s.store_scaled_square(319, 318, 1.0 / (s.v[307]));

        s.store_powf(316, 80, (-1.5));

        s.store_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));

        s.store_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));

        s.store_div_scaled_product_add_scaled_denominator_indices(322, 320, 321, 2.0, 320, 1.0, 321, 1.0, 1.0);

        s.store_powf(317, 80, p.p97);

        s.store_scale(324, 317, p.p93);

        s.store_sqrt_mul(323, 324, 322);

        s.store_scaled_ln_ad(347, A::div_from_scalar(s.v[307], s.ad_value(319)), (s.v[313] / s.v[85]));

        s.store_scaled_add_ad(348, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323)), (s.v[313] / s.v[85]));

        s.v[256] = (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]);

        s.v[257] = (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[258] = (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[281] = 0.0;

        s.b[393] = ((s.v[101] * s.v[256]) > 0.0);
        s.v[393] = if s.b[393] { 1.0 } else { 0.0 };

        if s.b[393] {
            s.store_scaled_ln_ad(168, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0), (s.v[84] * s.v[62]));
        }

        if (!s.b[393]) {
            s.store_scalar(168, 100000000.0);
        }

        s.b[394] = ((s.v[102] * s.v[257]) > 0.0);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if s.b[394] {
            s.store_scaled_ln_ad(169, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0), (s.v[84] * s.v[64]));
        }

        if (!s.b[394]) {
            s.store_scalar(169, 100000000.0);
        }

        s.b[395] = ((s.v[103] * s.v[258]) > 0.0);
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if s.b[395] {
            s.store_scaled_ln_ad(170, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0), (s.v[84] * s.v[63]));
        }

        if (!s.b[395]) {
            s.store_scalar(170, 100000000.0);
        }

        s.store_min3(262, 168, 169, 170);

        s.v[370] = 0.0;

        s.v[345] = 0.0;

        s.v[338] = 0.0;

        s.v[339] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

        s.v[344] = 0.0;

        s.v[333] = (1.6021918e-19 * s.v[256]);

        s.v[343] = ((((2.0 * s.v[0]) / (1.6021918e-19 * s.v[307]))) as f64).sqrt();

        s.v[314] = ((p.p94 - s.v[343]) - 1e-7);

        s.v[315] = ((4.0 * p.p94) * 1e-7);

        if (!(s.v[315] > 0.0)) {
            s.store_scalar(315, (-s.v[315]));
        }

        s.store_sqrt_offset_input(315, 315, (s.v[314] * s.v[314]));

        s.store_sub_from_scalar_ad(343, p.p94, A::scaled_offset(s.ad_value(315), s.v[314], 0.5));

        s.b[413] = (s.v[45] > 0.9);
        s.v[413] = if s.b[413] { 1.0 } else { 0.0 };

        s.b[414] = ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0)));
        s.v[414] = if s.b[414] { 1.0 } else { 0.0 };

        let (assign3210_e1950,) = {
    if (s.b[413] && s.b[414]) {
        (0.0,)
    } else {
        (s.v[45],)
    }
};
        s.v[45] = assign3210_e1950;

        s.b[418] = (s.v[45] == 1.0);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

        if s.b[418] {
            s.store_scalar(277, 0.0);
            s.store_scalar(205, 0.4);
            s.store_scalar(206, 0.65);
            s.store_scalar(207, 0.8);
            s.store_scale(190, 205, (-s.v[46]));
            s.store_scale(191, 206, (-s.v[46]));
            s.store_scale(192, 207, (-s.v[46]));
            s.store_scalar(193, 0.1);
            s.store_scalar(194, 0.2);
        }

        s.b[463] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        s.b[464] = (s.v[190] < s.v[262]);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[467] = (s.v[62] < p.p85);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(190), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[467])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[468] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[468]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[469] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && s.b[469]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && (!s.b[469])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[470] = (s.v[64] < p.p85);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(190), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[470])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[473] = (s.v[63] < p.p85);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(190), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[473])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[476] = (s.v[62] < p.p85);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[476])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[477] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[477]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[478] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && s.b[478]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && (!s.b[478])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[479] = (s.v[64] < p.p85);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[479])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[482] = (s.v[63] < p.p85);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[482])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[463]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[463])) {
            s.store_scalar(370, 0.0);
        }

        s.b[540] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        s.b[541] = (s.v[191] < s.v[262]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[544] = (s.v[62] < p.p85);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(191), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[544])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[545] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[545]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[546] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && s.b[546]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && (!s.b[546])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[547] = (s.v[64] < p.p85);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(191), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[547])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[550] = (s.v[63] < p.p85);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(191), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[550])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[553] = (s.v[62] < p.p85);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[553])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[554] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[554]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[555] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && s.b[555]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && (!s.b[555])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[556] = (s.v[64] < p.p85);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[556])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[559] = (s.v[63] < p.p85);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[540]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[540])) {
            s.store_scalar(370, 0.0);
        }

        s.b[617] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        s.b[618] = (s.v[192] < s.v[262]);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[621] = (s.v[62] < p.p85);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(192), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[621])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[622] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[622]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[623] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && s.b[623]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && (!s.b[623])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[624] = (s.v[64] < p.p85);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(192), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[624])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[627] = (s.v[63] < p.p85);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(192), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[627])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[630] = (s.v[62] < p.p85);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[630])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[631] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[631]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[632] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && s.b[632]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && (!s.b[632])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[633] = (s.v[64] < p.p85);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[633])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[636] = (s.v[63] < p.p85);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[636])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[617]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[617])) {
            s.store_scalar(370, 0.0);
        }

        s.b[694] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[694] = if s.b[694] { 1.0 } else { 0.0 };

        s.b[695] = (s.v[193] < s.v[262]);
        s.v[695] = if s.b[695] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[698] = (s.v[62] < p.p85);
        s.v[698] = if s.b[698] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(193), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[698])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[699] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[699] = if s.b[699] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[699]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[700] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[700] = if s.b[700] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && s.b[700]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && (!s.b[700])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[701] = (s.v[64] < p.p85);
        s.v[701] = if s.b[701] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(193), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[701])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[704] = (s.v[63] < p.p85);
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(193), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[704])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[707] = (s.v[62] < p.p85);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[707])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[708] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[708] = if s.b[708] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[708]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[709] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[709] = if s.b[709] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && s.b[709]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && (!s.b[709])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[710] = (s.v[64] < p.p85);
        s.v[710] = if s.b[710] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[710])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[713] = (s.v[63] < p.p85);
        s.v[713] = if s.b[713] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[713])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[694]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[694])) {
            s.store_scalar(370, 0.0);
        }

        s.b[771] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[771] = if s.b[771] { 1.0 } else { 0.0 };

        s.b[772] = (s.v[194] < s.v[262]);
        s.v[772] = if s.b[772] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[775] = (s.v[62] < p.p85);
        s.v[775] = if s.b[775] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(194), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[775])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[776] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[776] = if s.b[776] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[776]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[777] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[777] = if s.b[777] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && s.b[777]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && (!s.b[777])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[778] = (s.v[64] < p.p85);
        s.v[778] = if s.b[778] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(194), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[778])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[781] = (s.v[63] < p.p85);
        s.v[781] = if s.b[781] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(194), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[781])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[784] = (s.v[62] < p.p85);
        s.v[784] = if s.b[784] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[784])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[785] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[785] = if s.b[785] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[785]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[786] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[786] = if s.b[786] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && s.b[786]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && (!s.b[786])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[787] = (s.v[64] < p.p85);
        s.v[787] = if s.b[787] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

    }

    pub(super) fn stamp_transient_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[787])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[790] = (s.v[63] < p.p85);
        s.v[790] = if s.b[790] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[790])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[771]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[771])) {
            s.store_scalar(370, 0.0);
        }

        s.store_voltage(277, ctx, nodes, Some(0), Some(2));

        s.b[858] = (s.v[45] == 1.0);
        s.v[858] = if s.b[858] { 1.0 } else { 0.0 };

        s.b[866] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[866] = if s.b[866] { 1.0 } else { 0.0 };

        s.b[867] = (s.v[277] < s.v[262]);
        s.v[867] = if s.b[867] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[870] = (s.v[62] < p.p85);
        s.v[870] = if s.b[870] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(277), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[870])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[871] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[871] = if s.b[871] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[871]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[872] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[872] = if s.b[872] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && s.b[872]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && (!s.b[872])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[873] = (s.v[64] < p.p85);
        s.v[873] = if s.b[873] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(277), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[873])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[876] = (s.v[63] < p.p85);
        s.v[876] = if s.b[876] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(277), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[876])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[879] = (s.v[62] < p.p85);
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[879])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[880] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[880]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[881] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && s.b[881]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && (!s.b[881])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[882] = (s.v[64] < p.p85);
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[882])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[885] = (s.v[63] < p.p85);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
        }

    }

    pub(super) fn stamp_transient_block_11(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[885])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if ((!s.b[858]) && s.b[866]) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((!s.b[858]) && (!s.b[866])) {
            s.store_scalar(370, 0.0);
        }

        s.b[945] = (p.p84 > 0.0);
        s.v[945] = if s.b[945] { 1.0 } else { 0.0 };

        s.b[946] = (s.v[313] < p.p85);
        s.v[946] = if s.b[946] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[946]) {
            s.store_offset_sub_scaled_inputs(349, s.ad_value(277), p.p86, s.ad_value(348), p.p86, s.v[313]);
            s.store_sub_from_scalar_scaled_input(350, s.v[313], 348, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(349), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(351, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 351, (((-s.v[313])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(352, 314, 0.5, 315, 0.5, s.v[313]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[313])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[313]);
        }

        if (s.b[945] && (!s.b[946])) {
            s.store_scalar(352, s.v[313]);
            s.store_scalar(350, s.v[313]);
        }

        if s.b[945] {
            s.copy_ad(353, 370);
        }

        s.b[947] = ((s.v[277] - (s.v[348] - s.v[347])) > 0.0);
        s.v[947] = if s.b[947] { 1.0 } else { 0.0 };

        s.b[948] = ((((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[948] = if s.b[948] { 1.0 } else { 0.0 };

        if ((s.b[945] && s.b[947]) && s.b[948]) {
            s.store_exp_scaled_input_ad(354, A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), 1.0, A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), (-1.0), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), 1.0), s.v[85]);
        }

        s.b[949] = ((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && s.b[949]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(354, 1e-100, (-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && (!s.b[949])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(354, A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[945] && (!s.b[947])) {
            s.store_scalar(354, 1.0);
        }

        s.b[950] = ((p.p91 == 0.0) || (s.v[277] < s.v[347]));
        s.v[950] = if s.b[950] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[950]) {
            s.store_scale(357, 353, p.p90);
        }

        if (s.b[945] && (!s.b[950])) {
            s.store_mul_scaled_exp_ad_rhs(357, 353, p.p90, A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(347)), A::sub(s.ad_value(277), s.ad_value(347)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91)));
        }

        if s.b[945] {
            if (s.v[357] > p.p79) {
                s.store_scalar(357, p.p79);
            } else {
            }
        }

        if s.b[945] {
            s.store_mul(355, 319, 357);
            s.store_scaled_sub(331, 355, 319, (1.6021918e-19 * s.v[256]));
        }

        s.b[951] = (p.p92 > 0.0);
        s.v[951] = if s.b[951] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[951]) {
            s.store_scale(334, 331, (1e-23 / s.v[333]));
            s.store_voltage(336, ctx, nodes, Some(3), None);
            s.store_scaled_sub(338, 336, 334, 1.0 / (p.p92));
        }

        if (s.b[945] && (!s.b[951])) {
            s.copy_ad(334, 331);
        }

        s.b[952] = ((p.p91 == 0.0) || (s.v[277] < s.v[348]));
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[952]) {
            s.store_scale(358, 354, p.p90);
        }

        if (s.b[945] && (!s.b[952])) {
            s.store_mul_scaled_exp_ad_rhs(358, 354, p.p90, A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(348)), A::sub(s.ad_value(277), s.ad_value(348)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91)));
        }

        if s.b[945] {
            if (s.v[358] > p.p79) {
                s.store_scalar(358, p.p79);
            } else {
            }
        }

        if s.b[945] {
            s.store_mul(356, 319, 358);
            s.store_scaled_sub(332, 356, 319, (1.6021918e-19 * s.v[256]));
        }

        s.b[953] = (p.p92 > 0.0);
        s.v[953] = if s.b[953] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[953]) {
            s.store_scale(335, 332, (1e-23 / s.v[333]));
            s.store_voltage(337, ctx, nodes, Some(4), None);
            s.store_scaled_sub(339, 337, 335, 1.0 / (p.p92));
        }

        if (s.b[945] && (!s.b[953])) {
            s.copy_ad(335, 332);
        }

        if s.b[945] {
            s.store_sub_from_scalar(325, s.v[368], 277);
            s.store_sqrt_square_offset(315, 325, ((4.0 * s.v[369]) * s.v[369]));
            s.store_scaled_add(325, 325, 315, 0.5);
        }

        s.b[954] = (s.v[325] < 0.0);
        s.v[954] = if s.b[954] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[954]) {
            s.store_scalar(325, 0.0);
        }

        if s.b[945] {
            s.store_sqrt_scaled_input(326, 325, ((2.0 * s.v[0]) * 1.0 / ((1.6021918e-19 * s.v[307]))));
            s.store_offset_sub_from_scalar_ad(314, p.p94, s.ad_value(326), (-1e-7));
            s.store_scalar(315, ((4.0 * p.p94) * 1e-7));
        }

        if s.b[945] {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if s.b[945] {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(326, 314, (-0.5), 315, (-0.5), p.p94);
        }

        s.b[955] = (p.p95 > 0.0);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[955]) {
            s.store_mul_div_from_scalar_rhs(342, 326, 1.0, 343);
            s.store_voltage(344, ctx, nodes, Some(5), None);
            s.store_scaled_sub(345, 344, 342, 1.0 / (p.p95));
        }

        if (s.b[945] && (!s.b[955])) {
            s.copy_ad(342, 326);
        }

        s.b[958] = ((p.p84 > 0.0) && (p.p92 > 0.0));
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        s.b[959] = ((p.p84 > 0.0) && (p.p95 > 0.0));
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[0] = (8.8541878176e-12 * 11.8);

        s.v[1] = (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) });

        s.b[388] = ((!param_given[6]) && param_given[96]);
        s.v[388] = if s.b[388] { 1.0 } else { 0.0 };

        if s.b[388] {
            s.store_scalar(1, (if (p.p96 > (-250.0)) { p.p96 } else { (-250.0) }));
        }

        s.v[2] = (if (p.p5 > 1e-12) { p.p5 } else { 1e-12 });

        s.v[12] = p.p17;

        s.v[13] = p.p18;

        s.v[14] = p.p19;

        s.v[15] = (if (p.p20 > 0.0) { p.p20 } else { 0.0 });

        s.v[16] = (if (p.p21 > 0.0) { p.p21 } else { 0.0 });

        s.v[17] = (if (p.p22 > 0.0) { p.p22 } else { 0.0 });

        s.v[62] = (if (p.p63 > 0.1) { p.p63 } else { 0.1 });

        s.v[64] = (if (p.p64 > 0.1) { p.p64 } else { 0.1 });

        s.v[63] = (if (p.p65 > 0.1) { p.p65 } else { 0.1 });

        s.v[75] = (if (p.p76 > 0.1) { p.p76 } else { 0.1 });

        s.v[76] = (if (p.p77 > 0.0) { p.p77 } else { 0.0 });

        s.v[77] = (if (p.p78 > 0.0) { p.p78 } else { 0.0 });

        s.v[45] = 0.0;

        s.b[389] = (p.p81 > 0.5);
        s.v[389] = if s.b[389] { 1.0 } else { 0.0 };

        if s.b[389] {
            s.store_scalar(45, 1.0);
        }

        if (!s.b[389]) {
            s.store_scalar(45, 0.0);
        }

        s.v[46] = (if (p.p82 > 0.5) { p.p82 } else { 0.5 });

        s.store_offset(78, 1, 273.15);

        s.v[79] = ((ctx_temp + p.p102)).max((273.15 + (-250.0)));

        s.store_div_from_scalar(80, s.v[79], 78);

        s.v[81] = (1.3806505e-23 / 1.6021918e-19);

        s.store_scale(82, 78, s.v[81]);

        s.store_div_from_scalar(83, 1.0, 82);

        s.v[84] = (s.v[81] * s.v[79]);

        s.v[85] = (1.0 / s.v[84]);

        s.store_div_scaled_inputs(89, A::mul_scaled_lhs(s.ad_value(78), 0.000702, s.ad_value(78)), -1.0, A::offset(s.ad_value(78), 1108.0), 1.0);

        s.store_offset(92, 89, s.v[12]);

        s.store_offset(93, 89, s.v[13]);

        s.store_offset(94, 89, s.v[14]);

        s.v[90] = ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79]));

        s.v[95] = (s.v[12] + s.v[90]);

        s.v[96] = (s.v[13] + s.v[90]);

        s.v[97] = (s.v[14] + s.v[90]);

        s.store_mul_ad(176, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[62])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62]))));

        s.store_mul_ad(177, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[64])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64]))));

        s.store_mul_ad(178, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[63])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63]))));

        s.store_scaled_mul(101, 176, 176, s.v[15]);

        s.store_scaled_mul(102, 177, 177, s.v[16]);

        s.store_scaled_mul(103, 178, 178, s.v[17]);

        s.v[179] = (1.0 - (0.01 * s.v[77]));

        s.v[308] = (p.p87 * 1000000.0);

        s.v[310] = (p.p89 * 1000000.0);

        s.v[309] = (p.p88 * 1000000.0);

        s.v[307] = s.v[308];

        s.v[313] = s.v[62];

        s.v[311] = (1450.0 * 0.0001);

        s.v[312] = (500.0 * 0.0001);

        s.v[368] = 0.6;

        s.v[369] = 0.001;

        s.store_scale(318, 176, 1.45e16);

        s.store_scaled_square(319, 318, 1.0 / (s.v[307]));

        s.store_powf(316, 80, (-1.5));

        s.store_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));

        s.store_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));

        s.store_div_scaled_product_add_scaled_denominator_indices(322, 320, 321, 2.0, 320, 1.0, 321, 1.0, 1.0);

        s.store_powf(317, 80, p.p97);

        s.store_scale(324, 317, p.p93);

        s.store_sqrt_mul(323, 324, 322);

        s.store_scaled_ln_ad(347, A::div_from_scalar(s.v[307], s.ad_value(319)), (s.v[313] / s.v[85]));

        s.store_scaled_add_ad(348, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323)), (s.v[313] / s.v[85]));

        s.v[256] = (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]);

        s.v[257] = (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[258] = (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[281] = 0.0;

        s.b[393] = ((s.v[101] * s.v[256]) > 0.0);
        s.v[393] = if s.b[393] { 1.0 } else { 0.0 };

        if s.b[393] {
            s.store_scaled_ln_ad(168, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0), (s.v[84] * s.v[62]));
        }

        if (!s.b[393]) {
            s.store_scalar(168, 100000000.0);
        }

        s.b[394] = ((s.v[102] * s.v[257]) > 0.0);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if s.b[394] {
            s.store_scaled_ln_ad(169, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0), (s.v[84] * s.v[64]));
        }

        if (!s.b[394]) {
            s.store_scalar(169, 100000000.0);
        }

        s.b[395] = ((s.v[103] * s.v[258]) > 0.0);
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if s.b[395] {
            s.store_scaled_ln_ad(170, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0), (s.v[84] * s.v[63]));
        }

        if (!s.b[395]) {
            s.store_scalar(170, 100000000.0);
        }

        s.store_min3(262, 168, 169, 170);

        s.v[370] = 0.0;

        s.v[345] = 0.0;

        s.v[338] = 0.0;

        s.v[339] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

        s.v[344] = 0.0;

        s.v[333] = (1.6021918e-19 * s.v[256]);

        s.v[343] = ((((2.0 * s.v[0]) / (1.6021918e-19 * s.v[307]))) as f64).sqrt();

        s.v[314] = ((p.p94 - s.v[343]) - 1e-7);

        s.v[315] = ((4.0 * p.p94) * 1e-7);

        if (!(s.v[315] > 0.0)) {
            s.store_scalar(315, (-s.v[315]));
        }

        s.store_sqrt_offset_input(315, 315, (s.v[314] * s.v[314]));

        s.store_sub_from_scalar_ad(343, p.p94, A::scaled_offset(s.ad_value(315), s.v[314], 0.5));

        s.b[413] = (s.v[45] > 0.9);
        s.v[413] = if s.b[413] { 1.0 } else { 0.0 };

        s.b[414] = ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0)));
        s.v[414] = if s.b[414] { 1.0 } else { 0.0 };

        if (s.b[413] && s.b[414]) {
            s.store_scalar(45, 0.0);
        }

        s.b[418] = (s.v[45] == 1.0);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

        if s.b[418] {
            s.store_scalar(277, 0.0);
            s.store_scalar(205, 0.4);
            s.store_scalar(206, 0.65);
            s.store_scalar(207, 0.8);
            s.store_scale(190, 205, (-s.v[46]));
            s.store_scale(191, 206, (-s.v[46]));
            s.store_scale(192, 207, (-s.v[46]));
            s.store_scalar(193, 0.1);
            s.store_scalar(194, 0.2);
        }

        s.b[463] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        s.b[464] = (s.v[190] < s.v[262]);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[467] = (s.v[62] < p.p85);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(190), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[467])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[468] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[468]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[469] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && s.b[469]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && (!s.b[469])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[470] = (s.v[64] < p.p85);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(190), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[470])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[473] = (s.v[63] < p.p85);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(190), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[473])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[476] = (s.v[62] < p.p85);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[476])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[477] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[477]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[478] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && s.b[478]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && (!s.b[478])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[479] = (s.v[64] < p.p85);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[479])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[482] = (s.v[63] < p.p85);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[482])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[463]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[463])) {
            s.store_scalar(370, 0.0);
        }

        s.b[540] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        s.b[541] = (s.v[191] < s.v[262]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[544] = (s.v[62] < p.p85);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(191), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[544])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[545] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[545]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[546] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && s.b[546]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && (!s.b[546])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[547] = (s.v[64] < p.p85);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(191), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[547])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[550] = (s.v[63] < p.p85);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(191), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[550])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[553] = (s.v[62] < p.p85);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[553])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[554] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[554]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[555] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && s.b[555]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && (!s.b[555])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[556] = (s.v[64] < p.p85);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[556])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[559] = (s.v[63] < p.p85);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_offset_sub_scaled_inputs(360, s.ad_value(262), p.p86, s.ad_value(362), p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

    }
}
