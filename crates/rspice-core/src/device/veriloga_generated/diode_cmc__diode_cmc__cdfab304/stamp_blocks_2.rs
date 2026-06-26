#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[486])) && s.b[488]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[489] = (s.v[9] == 0.5);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && s.b[489]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && (!s.b[489])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {
            s.store_add(442, 440, 441);
        }

        s.b[490] = (s.v[9] == 0.5);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && s.b[490]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && (!s.b[490])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_lhs(444, s.ad_value(98), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[491] = (s.v[23] == 0.0);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[491]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[122] * s.v[152]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[492] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[492]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[492])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[149]), s.ad_value(447), s.ad_value(450), s.v[149], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[493] = (s.v[457] > 0.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[493]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[493])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[494] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[494]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[494])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[495] = (s.v[457] > 0.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[495]) {
            s.copy_ad(458, 421);
        }

        s.b[496] = (s.v[456] > (-230.25850929940458));
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) && s.b[496]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) && (!s.b[496])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[149] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);
        }

        s.b[497] = (s.v[29] == 0.0);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[497]) {
            s.store_scalar(460, 0.0);
        }

        s.b[498] = (s.v[9] == 0.5);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && s.b[498]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[498])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[143]), ((s.v[6]) * (s.v[143]))), s.v[9]);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[497])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), s.ad_value(436), 1.0);
        }

        s.b[499] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && s.b[499]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));
        }

        s.b[500] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[499])) && s.b[500]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[499])) && (!s.b[500])) {
            let assign6990_ad_e7691: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign6990_ad_e7691, 1e100);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[497])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(190), s.ad_value(461), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[501] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[501]) {
            s.store_scalar(462, 1.0);
        }

        s.b[502] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        s.b[503] = (s.v[41] == 4.0);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) && s.b[503]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) && (!s.b[503])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[501])) && (!s.b[502])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[486])) {
            s.store_mul_ad_lhs(268, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(291, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[504] = (s.v[257] == 0.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[504]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[505] = (s.v[123] == 0.5);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[505]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[505])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[504])) {
            s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(190), s.ad_value(428)), 1.0);
            s.store_mul(437, 102, 371);
        }

        s.b[506] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[506]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[507] = (s.v[10] == 0.5);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && s.b[507]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && (!s.b[507])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {
            s.store_add(442, 440, 441);
        }

        s.b[508] = (s.v[10] == 0.5);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && s.b[508]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && (!s.b[508])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_lhs(444, s.ad_value(99), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[509] = (s.v[24] == 0.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[509]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[123] * s.v[153]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[510] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[510]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[510])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[150]), s.ad_value(447), s.ad_value(450), s.v[150], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[511] = (s.v[457] > 0.0);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[511]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[511])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[512] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[512]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[512])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[513] = (s.v[457] > 0.0);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[513]) {
            s.copy_ad(458, 421);
        }

        s.b[514] = (s.v[456] > (-230.25850929940458));
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) && s.b[514]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) && (!s.b[514])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[150] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);
        }

        s.b[515] = (s.v[30] == 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[515]) {
            s.store_scalar(460, 0.0);
        }

        s.b[516] = (s.v[10] == 0.5);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && s.b[516]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[516])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[515])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), s.ad_value(436), 1.0);
        }

        s.b[517] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && s.b[517]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));
        }

        s.b[518] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[517])) && s.b[518]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[517])) && (!s.b[518])) {
            let assign7800_ad_e8847: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign7800_ad_e8847, 1e100);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[515])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(190), s.ad_value(461), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[519] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[519]) {
            s.store_scalar(462, 1.0);
        }

        s.b[520] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        s.b[521] = (s.v[42] == 4.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) && s.b[521]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) && (!s.b[521])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[519])) && (!s.b[520])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[504])) {
            s.store_mul_ad_lhs(270, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(292, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[522] = (s.v[258] == 0.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[522]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[523] = (s.v[124] == 0.5);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[523]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[523])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[522])) {
            s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(190), s.ad_value(428)), 1.0);
            s.store_mul(437, 103, 372);
        }

        s.b[524] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[524]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[525] = (s.v[11] == 0.5);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && s.b[525]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && (!s.b[525])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {
            s.store_add(442, 440, 441);
        }

        s.b[526] = (s.v[11] == 0.5);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && s.b[526]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && (!s.b[526])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_lhs(444, s.ad_value(100), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[527] = (s.v[25] == 0.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[527]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[124] * s.v[154]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[528] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[528]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[528])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[151]), s.ad_value(447), s.ad_value(450), s.v[151], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[529] = (s.v[457] > 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[529]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[529])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[530] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[530]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[530])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[531] = (s.v[457] > 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[531]) {
            s.copy_ad(458, 421);
        }

        s.b[532] = (s.v[456] > (-230.25850929940458));
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) && s.b[532]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) && (!s.b[532])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[151] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);
        }

        s.b[533] = (s.v[31] == 0.0);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[533]) {
            s.store_scalar(460, 0.0);
        }

        s.b[534] = (s.v[11] == 0.5);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && s.b[534]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[534])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[145]), ((s.v[8]) * (s.v[145]))), s.v[11]);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[533])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), s.ad_value(436), 1.0);
        }

        s.b[535] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && s.b[535]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));
        }

        s.b[536] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[535])) && s.b[536]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[535])) && (!s.b[536])) {
            let assign8610_ad_e10003: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign8610_ad_e10003, 1e100);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[533])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(190), s.ad_value(461), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[537] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[537]) {
            s.store_scalar(462, 1.0);
        }

        s.b[538] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        s.b[539] = (s.v[43] == 4.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) && s.b[539]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) && (!s.b[539])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[537])) && (!s.b[538])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[522])) {
            s.store_mul_ad_lhs(272, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(293, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        if s.b[418] {
            s.store_add_scaled_inputs3(180, s.ad_value(268), s.v[256], s.ad_value(270), s.v[257], s.ad_value(272), s.v[258]);
        }

        s.b[540] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[540]) {
            s.store_scaled_mul(422, 265, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_scaled_product_indices(424, 191, 1.0, 265, 423, 1.0);
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_div_scaled_product_denominator_ad(428, 191, 266, 2.0, A::add(s.ad_value(425), s.ad_value(427)), 1.0);
        }

        s.b[541] = (s.v[191] < s.v[262]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        s.b[542] = ((((0.5 * (s.v[191] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[542]) {
            s.store_exp_scaled_input(430, 191, (s.v[85] * 0.5));
        }

        s.b[543] = ((0.5 * (s.v[191] * s.v[85])) < (-230.25850929940458));
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[542])) && s.b[543]) {
            s.store_div_from_scalar_offset_ad(430, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[542])) && (!s.b[543])) {
            s.store_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(191), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(191), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(191), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[544] = (s.v[62] < p.p85);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_offset_scaled_sub(360, 191, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            let assign9180_ad_e10912: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign9180_ad_e10912, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && (!s.b[546])) {
            let assign9190_ad_e10988: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(370, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign9190_ad_e10988, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[547] = (s.v[64] < p.p85);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_offset_scaled_sub(360, 191, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[547])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[548] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[548]) {
            s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[549] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[548])) && s.b[549]) {
            let assign9500_ad_e11513: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, assign9500_ad_e11513, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[548])) && (!s.b[549])) {
            let assign9510_ad_e11589: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(371, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign9510_ad_e11589, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[550] = (s.v[63] < p.p85);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_offset_scaled_sub(360, 191, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[550])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[551] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[551]) {
            s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[552] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[551])) && s.b[552]) {
            let assign9820_ad_e12114: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, assign9820_ad_e12114, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[551])) && (!s.b[552])) {
            let assign9830_ad_e12190: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(372, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign9830_ad_e12190, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(191), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[553] = (s.v[62] < p.p85);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
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
            let assign10190_ad_e12822: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign10190_ad_e12822, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && (!s.b[555])) {
            let assign10200_ad_e12899: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(281, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign10200_ad_e12899, 1.0)), 1.0, 1e100);
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
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[556])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[557] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[557]) {
            s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[558] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[557])) && s.b[558]) {
            let assign10570_ad_e13557: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, assign10570_ad_e13557, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[557])) && (!s.b[558])) {
            let assign10580_ad_e13634: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(282, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign10580_ad_e13634, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(371, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[559] = (s.v[63] < p.p85);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[560] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[560]) {
            s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[561] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[560])) && s.b[561]) {
            let assign10950_ad_e14292: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, assign10950_ad_e14292, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[560])) && (!s.b[561])) {
            let assign10960_ad_e14369: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(283, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign10960_ad_e14369, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(372, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 283);
        }

        if (s.b[418] && s.b[540]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[562] = (s.v[191] > 0.0);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[540]) && s.b[562]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[540]) && (!s.b[562])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 191);
        }

        if (s.b[418] && s.b[540]) {
            s.store_sub(432, 264, 431);
            s.store_add_scaled_inputs3(433, s.ad_value(191), 0.5, s.ad_value(432), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(432)), A::sub(s.ad_value(191), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), (-0.5));
            s.store_add_scaled_inputs3(434, s.ad_value(191), 0.5, s.ad_value(267), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(191), s.ad_value(267)), A::sub(s.ad_value(191), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0)), (-0.5));
            s.store_scaled_sub_ad_rhs(435, 191, A::sqrt(A::offset(A::mul(s.ad_value(191), s.ad_value(191)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[540])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[563] = (s.v[256] == 0.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[563]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[564] = (s.v[122] == 0.5);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[564]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[564])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[563])) {
            s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(191), s.ad_value(428)), 1.0);
            s.store_mul(437, 101, 370);
        }

        s.b[565] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[563])) && s.b[565]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[566] = (s.v[9] == 0.5);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && s.b[566]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && (!s.b[566])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {
            s.store_add(442, 440, 441);
        }

        s.b[567] = (s.v[9] == 0.5);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && s.b[567]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && (!s.b[567])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_lhs(444, s.ad_value(98), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[568] = (s.v[23] == 0.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[568]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[122] * s.v[152]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[569] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[569]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[569])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[149]), s.ad_value(447), s.ad_value(450), s.v[149], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[570] = (s.v[457] > 0.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[570]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[570])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[571] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[571]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[571])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[572] = (s.v[457] > 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[572]) {
            s.copy_ad(458, 421);
        }

        s.b[573] = (s.v[456] > (-230.25850929940458));
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) && s.b[573]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) && (!s.b[573])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[149] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);
        }

        s.b[574] = (s.v[29] == 0.0);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[574]) {
            s.store_scalar(460, 0.0);
        }

        s.b[575] = (s.v[9] == 0.5);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && s.b[575]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[575])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[143]), ((s.v[6]) * (s.v[143]))), s.v[9]);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[574])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), s.ad_value(436), 1.0);
        }

        s.b[576] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && s.b[576]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));
        }

        s.b[577] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[576])) && s.b[577]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[576])) && (!s.b[577])) {
            let assign11880_ad_e15649: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign11880_ad_e15649, 1e100);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[574])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(191), s.ad_value(461), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[578] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[578]) {
            s.store_scalar(462, 1.0);
        }

        s.b[579] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        s.b[580] = (s.v[41] == 4.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) && s.b[580]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) && (!s.b[580])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[578])) && (!s.b[579])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[563])) {
            s.store_mul_ad_lhs(268, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(291, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[581] = (s.v[257] == 0.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[581]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[582] = (s.v[123] == 0.5);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[582]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[582])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[581])) {
            s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(191), s.ad_value(428)), 1.0);
            s.store_mul(437, 102, 371);
        }

        s.b[583] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[583]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[584] = (s.v[10] == 0.5);
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && s.b[584]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && (!s.b[584])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {
            s.store_add(442, 440, 441);
        }

        s.b[585] = (s.v[10] == 0.5);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && s.b[585]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && (!s.b[585])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_lhs(444, s.ad_value(99), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[586] = (s.v[24] == 0.0);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[586]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[123] * s.v[153]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[587] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[587]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[587])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[150]), s.ad_value(447), s.ad_value(450), s.v[150], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[588] = (s.v[457] > 0.0);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[588]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[588])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[589] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[589]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[589])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[590] = (s.v[457] > 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[590]) {
            s.copy_ad(458, 421);
        }

        s.b[591] = (s.v[456] > (-230.25850929940458));
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) && s.b[591]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) && (!s.b[591])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[150] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);
        }

        s.b[592] = (s.v[30] == 0.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[592]) {
            s.store_scalar(460, 0.0);
        }

        s.b[593] = (s.v[10] == 0.5);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && s.b[593]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[593])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[592])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), s.ad_value(436), 1.0);
        }

        s.b[594] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && s.b[594]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));
        }

        s.b[595] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[594])) && s.b[595]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[594])) && (!s.b[595])) {
            let assign12690_ad_e16805: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign12690_ad_e16805, 1e100);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[592])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(191), s.ad_value(461), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[596] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[596]) {
            s.store_scalar(462, 1.0);
        }

        s.b[597] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        s.b[598] = (s.v[42] == 4.0);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) && s.b[598]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) && (!s.b[598])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[596])) && (!s.b[597])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[581])) {
            s.store_mul_ad_lhs(270, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(292, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[599] = (s.v[258] == 0.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[599]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[600] = (s.v[124] == 0.5);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[600]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[600])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[599])) {
            s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(191), s.ad_value(428)), 1.0);
            s.store_mul(437, 103, 372);
        }

        s.b[601] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[601]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[602] = (s.v[11] == 0.5);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && s.b[602]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && (!s.b[602])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {
            s.store_add(442, 440, 441);
        }

        s.b[603] = (s.v[11] == 0.5);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && s.b[603]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && (!s.b[603])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_lhs(444, s.ad_value(100), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[604] = (s.v[25] == 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[604]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[124] * s.v[154]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[605] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[605]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[605])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[151]), s.ad_value(447), s.ad_value(450), s.v[151], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[606] = (s.v[457] > 0.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[606]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[606])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[607] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[607]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[607])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[608] = (s.v[457] > 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[608]) {
            s.copy_ad(458, 421);
        }

        s.b[609] = (s.v[456] > (-230.25850929940458));
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) && s.b[609]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) && (!s.b[609])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[151] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);
        }

        s.b[610] = (s.v[31] == 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[610]) {
            s.store_scalar(460, 0.0);
        }

        s.b[611] = (s.v[11] == 0.5);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && s.b[611]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[611])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[145]), ((s.v[8]) * (s.v[145]))), s.v[11]);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[610])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), s.ad_value(436), 1.0);
        }

        s.b[612] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && s.b[612]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));
        }

        s.b[613] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[612])) && s.b[613]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[612])) && (!s.b[613])) {
            let assign13500_ad_e17961: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign13500_ad_e17961, 1e100);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[610])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(191), s.ad_value(461), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[614] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[614]) {
            s.store_scalar(462, 1.0);
        }

        s.b[615] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        s.b[616] = (s.v[43] == 4.0);
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) && s.b[616]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) && (!s.b[616])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[614])) && (!s.b[615])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[599])) {
            s.store_mul_ad_lhs(272, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(293, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        if s.b[418] {
            s.store_add_scaled_inputs3(181, s.ad_value(268), s.v[256], s.ad_value(270), s.v[257], s.ad_value(272), s.v[258]);
        }

        s.b[617] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[617]) {
            s.store_scaled_mul(422, 265, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_scaled_product_indices(424, 192, 1.0, 265, 423, 1.0);
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_div_scaled_product_denominator_ad(428, 192, 266, 2.0, A::add(s.ad_value(425), s.ad_value(427)), 1.0);
        }

        s.b[618] = (s.v[192] < s.v[262]);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        s.b[619] = ((((0.5 * (s.v[192] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[619]) {
            s.store_exp_scaled_input(430, 192, (s.v[85] * 0.5));
        }

        s.b[620] = ((0.5 * (s.v[192] * s.v[85])) < (-230.25850929940458));
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[619])) && s.b[620]) {
            s.store_div_from_scalar_offset_ad(430, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[619])) && (!s.b[620])) {
            s.store_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(192), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(192), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(192), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[621] = (s.v[62] < p.p85);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_offset_scaled_sub(360, 192, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            let assign14070_ad_e18870: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign14070_ad_e18870, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && (!s.b[623])) {
            let assign14080_ad_e18946: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(370, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign14080_ad_e18946, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[624] = (s.v[64] < p.p85);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_offset_scaled_sub(360, 192, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[624])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[625] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[625]) {
            s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[626] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[625])) && s.b[626]) {
            let assign14390_ad_e19471: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, assign14390_ad_e19471, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[625])) && (!s.b[626])) {
            let assign14400_ad_e19547: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(371, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign14400_ad_e19547, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[627] = (s.v[63] < p.p85);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_offset_scaled_sub(360, 192, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[627])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[628] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[628]) {
            s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[629] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[628])) && s.b[629]) {
            let assign14710_ad_e20072: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, assign14710_ad_e20072, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[628])) && (!s.b[629])) {
            let assign14720_ad_e20148: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(372, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign14720_ad_e20148, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(192), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[630] = (s.v[62] < p.p85);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
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
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
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
            let assign15080_ad_e20780: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign15080_ad_e20780, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && (!s.b[632])) {
            let assign15090_ad_e20857: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(281, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign15090_ad_e20857, 1.0)), 1.0, 1e100);
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
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[633])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[634] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[634]) {
            s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[635] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[634])) && s.b[635]) {
            let assign15460_ad_e21515: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, assign15460_ad_e21515, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[634])) && (!s.b[635])) {
            let assign15470_ad_e21592: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(282, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign15470_ad_e21592, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(371, A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[636] = (s.v[63] < p.p85);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[636])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[637] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[637]) {
            s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[638] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[637])) && s.b[638]) {
            let assign15840_ad_e22250: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, assign15840_ad_e22250, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[637])) && (!s.b[638])) {
            let assign15850_ad_e22327: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(283, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign15850_ad_e22327, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(372, A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0, 283);
        }

        if (s.b[418] && s.b[617]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[639] = (s.v[192] > 0.0);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[617]) && s.b[639]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[617]) && (!s.b[639])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 192);
        }

        if (s.b[418] && s.b[617]) {
            s.store_sub(432, 264, 431);
            s.store_add_scaled_inputs3(433, s.ad_value(192), 0.5, s.ad_value(432), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(432)), A::sub(s.ad_value(192), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), (-0.5));
            s.store_add_scaled_inputs3(434, s.ad_value(192), 0.5, s.ad_value(267), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(192), s.ad_value(267)), A::sub(s.ad_value(192), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0)), (-0.5));
            s.store_scaled_sub_ad_rhs(435, 192, A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(192)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[617])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[640] = (s.v[256] == 0.0);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[640]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[641] = (s.v[122] == 0.5);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[641]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[641])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[640])) {
            s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(192), s.ad_value(428)), 1.0);
            s.store_mul(437, 101, 370);
        }

        s.b[642] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[640])) && s.b[642]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[642])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[643] = (s.v[9] == 0.5);
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && s.b[643]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && (!s.b[643])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[642])) {
            s.store_add(442, 440, 441);
        }

        s.b[644] = (s.v[9] == 0.5);
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && s.b[644]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && (!s.b[644])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[642])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_lhs(444, s.ad_value(98), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[645] = (s.v[23] == 0.0);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[645]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[122] * s.v[152]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[646] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[646]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[646])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[149]), s.ad_value(447), s.ad_value(450), s.v[149], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[647] = (s.v[457] > 0.0);
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[647]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[647])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[648] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[648]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[648])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[649] = (s.v[457] > 0.0);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[649]) {
            s.copy_ad(458, 421);
        }

        s.b[650] = (s.v[456] > (-230.25850929940458));
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[649])) && s.b[650]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[649])) && (!s.b[650])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[649])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[149] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);
        }

        s.b[651] = (s.v[29] == 0.0);
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[651]) {
            s.store_scalar(460, 0.0);
        }

        s.b[652] = (s.v[9] == 0.5);
        s.v[652] = if s.b[652] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[651])) && s.b[652]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[651])) && (!s.b[652])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[143]), ((s.v[6]) * (s.v[143]))), s.v[9]);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[651])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), s.ad_value(436), 1.0);
        }

        s.b[653] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[651])) && s.b[653]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));
        }

        s.b[654] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[640])) && (!s.b[651])) && (!s.b[653])) && s.b[654]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[640])) && (!s.b[651])) && (!s.b[653])) && (!s.b[654])) {
            let assign16770_ad_e23607: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign16770_ad_e23607, 1e100);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[651])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(192), s.ad_value(461), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[655] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[655]) {
            s.store_scalar(462, 1.0);
        }

        s.b[656] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        s.b[657] = (s.v[41] == 4.0);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[640])) && (!s.b[655])) && s.b[656]) && s.b[657]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[640])) && (!s.b[655])) && s.b[656]) && (!s.b[657])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[655])) && s.b[656]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[655])) && (!s.b[656])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[640])) {
            s.store_mul_ad_lhs(268, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(291, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[658] = (s.v[257] == 0.0);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[658]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[659] = (s.v[123] == 0.5);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[659]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[659])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[658])) {
            s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(192), s.ad_value(428)), 1.0);
            s.store_mul(437, 102, 371);
        }

        s.b[660] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[660]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[660])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[661] = (s.v[10] == 0.5);
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && s.b[661]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && (!s.b[661])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[660])) {
            s.store_add(442, 440, 441);
        }

        s.b[662] = (s.v[10] == 0.5);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && s.b[662]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && (!s.b[662])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[660])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_lhs(444, s.ad_value(99), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[663] = (s.v[24] == 0.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[663]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[123] * s.v[153]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[664] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[664]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[664])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[150]), s.ad_value(447), s.ad_value(450), s.v[150], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[665] = (s.v[457] > 0.0);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[665]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[665])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[666] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[666]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[666])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[667] = (s.v[457] > 0.0);
        s.v[667] = if s.b[667] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[667]) {
            s.copy_ad(458, 421);
        }

        s.b[668] = (s.v[456] > (-230.25850929940458));
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[667])) && s.b[668]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[667])) && (!s.b[668])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[667])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[150] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);
        }

        s.b[669] = (s.v[30] == 0.0);
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[669]) {
            s.store_scalar(460, 0.0);
        }

        s.b[670] = (s.v[10] == 0.5);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[669])) && s.b[670]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[658])) && (!s.b[669])) && (!s.b[670])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[669])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), s.ad_value(436), 1.0);
        }

        s.b[671] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[669])) && s.b[671]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));
        }

        s.b[672] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[658])) && (!s.b[669])) && (!s.b[671])) && s.b[672]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[658])) && (!s.b[669])) && (!s.b[671])) && (!s.b[672])) {
            let assign17580_ad_e24763: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign17580_ad_e24763, 1e100);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[669])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(192), s.ad_value(461), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[673] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[673] = if s.b[673] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[673]) {
            s.store_scalar(462, 1.0);
        }

        s.b[674] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[674] = if s.b[674] { 1.0 } else { 0.0 };

        s.b[675] = (s.v[42] == 4.0);
        s.v[675] = if s.b[675] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[658])) && (!s.b[673])) && s.b[674]) && s.b[675]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[658])) && (!s.b[673])) && s.b[674]) && (!s.b[675])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[673])) && s.b[674]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[673])) && (!s.b[674])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[658])) {
            s.store_mul_ad_lhs(270, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(292, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[676] = (s.v[258] == 0.0);
        s.v[676] = if s.b[676] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[676]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[677] = (s.v[124] == 0.5);
        s.v[677] = if s.b[677] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[677]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[677])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[676])) {
            s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(192), s.ad_value(428)), 1.0);
            s.store_mul(437, 103, 372);
        }

        s.b[678] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[678] = if s.b[678] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[678]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[678])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[679] = (s.v[11] == 0.5);
        s.v[679] = if s.b[679] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && s.b[679]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && (!s.b[679])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[678])) {
            s.store_add(442, 440, 441);
        }

        s.b[680] = (s.v[11] == 0.5);
        s.v[680] = if s.b[680] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && s.b[680]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && (!s.b[680])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[678])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_lhs(444, s.ad_value(100), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[681] = (s.v[25] == 0.0);
        s.v[681] = if s.b[681] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[681]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[124] * s.v[154]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[682] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[682] = if s.b[682] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[682]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[682])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[151]), s.ad_value(447), s.ad_value(450), s.v[151], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[683] = (s.v[457] > 0.0);
        s.v[683] = if s.b[683] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[683]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[683])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[684] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[684] = if s.b[684] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[684]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[684])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[685] = (s.v[457] > 0.0);
        s.v[685] = if s.b[685] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[685]) {
            s.copy_ad(458, 421);
        }

        s.b[686] = (s.v[456] > (-230.25850929940458));
        s.v[686] = if s.b[686] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[685])) && s.b[686]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[685])) && (!s.b[686])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[685])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[151] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);
        }

        s.b[687] = (s.v[31] == 0.0);
        s.v[687] = if s.b[687] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[687]) {
            s.store_scalar(460, 0.0);
        }

        s.b[688] = (s.v[11] == 0.5);
        s.v[688] = if s.b[688] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[687])) && s.b[688]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[687])) && (!s.b[688])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[145]), ((s.v[8]) * (s.v[145]))), s.v[11]);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[687])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), s.ad_value(436), 1.0);
        }

        s.b[689] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[689] = if s.b[689] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[687])) && s.b[689]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));
        }

        s.b[690] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[690] = if s.b[690] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[676])) && (!s.b[687])) && (!s.b[689])) && s.b[690]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[676])) && (!s.b[687])) && (!s.b[689])) && (!s.b[690])) {
            let assign18390_ad_e25919: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign18390_ad_e25919, 1e100);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[687])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(192), s.ad_value(461), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[691] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[691] = if s.b[691] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[691]) {
            s.store_scalar(462, 1.0);
        }

        s.b[692] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[692] = if s.b[692] { 1.0 } else { 0.0 };

        s.b[693] = (s.v[43] == 4.0);
        s.v[693] = if s.b[693] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[676])) && (!s.b[691])) && s.b[692]) && s.b[693]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[676])) && (!s.b[691])) && s.b[692]) && (!s.b[693])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[691])) && s.b[692]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[691])) && (!s.b[692])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[676])) {
            s.store_mul_ad_lhs(272, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(293, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        if s.b[418] {
            s.store_add_scaled_inputs3(182, s.ad_value(268), s.v[256], s.ad_value(270), s.v[257], s.ad_value(272), s.v[258]);
        }

        s.b[694] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[694] = if s.b[694] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[694]) {
            s.store_scaled_mul(422, 265, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_scaled_product_indices(424, 193, 1.0, 265, 423, 1.0);
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_div_scaled_product_denominator_ad(428, 193, 266, 2.0, A::add(s.ad_value(425), s.ad_value(427)), 1.0);
        }

        s.b[695] = (s.v[193] < s.v[262]);
        s.v[695] = if s.b[695] { 1.0 } else { 0.0 };

        s.b[696] = ((((0.5 * (s.v[193] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[696] = if s.b[696] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[696]) {
            s.store_exp_scaled_input(430, 193, (s.v[85] * 0.5));
        }

        s.b[697] = ((0.5 * (s.v[193] * s.v[85])) < (-230.25850929940458));
        s.v[697] = if s.b[697] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[696])) && s.b[697]) {
            s.store_div_from_scalar_offset_ad(430, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[696])) && (!s.b[697])) {
            s.store_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(193), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(193), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(193), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[698] = (s.v[62] < p.p85);
        s.v[698] = if s.b[698] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_offset_scaled_sub(360, 193, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            let assign18960_ad_e26828: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign18960_ad_e26828, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && (!s.b[700])) {
            let assign18970_ad_e26904: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(370, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign18970_ad_e26904, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[701] = (s.v[64] < p.p85);
        s.v[701] = if s.b[701] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_offset_scaled_sub(360, 193, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[701])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[702] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[702] = if s.b[702] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[702]) {
            s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[703] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[703] = if s.b[703] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[702])) && s.b[703]) {
            let assign19280_ad_e27429: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, assign19280_ad_e27429, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[702])) && (!s.b[703])) {
            let assign19290_ad_e27505: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(371, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign19290_ad_e27505, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[704] = (s.v[63] < p.p85);
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_offset_scaled_sub(360, 193, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[704])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[705] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[705] = if s.b[705] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[705]) {
            s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[706] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[706] = if s.b[706] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[705])) && s.b[706]) {
            let assign19600_ad_e28030: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, assign19600_ad_e28030, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[705])) && (!s.b[706])) {
            let assign19610_ad_e28106: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(372, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign19610_ad_e28106, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(193), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[707] = (s.v[62] < p.p85);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
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
            let assign19970_ad_e28738: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign19970_ad_e28738, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && (!s.b[709])) {
            let assign19980_ad_e28815: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(281, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign19980_ad_e28815, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[710] = (s.v[64] < p.p85);
        s.v[710] = if s.b[710] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[710])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[711] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[711] = if s.b[711] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[711]) {
            s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[712] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[712] = if s.b[712] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[711])) && s.b[712]) {
            let assign20350_ad_e29473: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, assign20350_ad_e29473, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[711])) && (!s.b[712])) {
            let assign20360_ad_e29550: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(282, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign20360_ad_e29550, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(371, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[713] = (s.v[63] < p.p85);
        s.v[713] = if s.b[713] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[713])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[714] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[714] = if s.b[714] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[714]) {
            s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[715] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[715] = if s.b[715] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[714])) && s.b[715]) {
            let assign20730_ad_e30208: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, assign20730_ad_e30208, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[714])) && (!s.b[715])) {
            let assign20740_ad_e30285: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(283, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign20740_ad_e30285, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(372, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 283);
        }

        if (s.b[418] && s.b[694]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[716] = (s.v[193] > 0.0);
        s.v[716] = if s.b[716] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[694]) && s.b[716]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[694]) && (!s.b[716])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 193);
        }

        if (s.b[418] && s.b[694]) {
            s.store_sub(432, 264, 431);
            s.store_add_scaled_inputs3(433, s.ad_value(193), 0.5, s.ad_value(432), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(432)), A::sub(s.ad_value(193), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), (-0.5));
            s.store_add_scaled_inputs3(434, s.ad_value(193), 0.5, s.ad_value(267), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(193), s.ad_value(267)), A::sub(s.ad_value(193), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0)), (-0.5));
            s.store_scaled_sub_ad_rhs(435, 193, A::sqrt(A::offset(A::mul(s.ad_value(193), s.ad_value(193)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[694])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[717] = (s.v[256] == 0.0);
        s.v[717] = if s.b[717] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[717]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[718] = (s.v[122] == 0.5);
        s.v[718] = if s.b[718] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[717])) && s.b[718]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[718])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[717])) {
            s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(193), s.ad_value(428)), 1.0);
            s.store_mul(437, 101, 370);
        }

        s.b[719] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[719] = if s.b[719] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[717])) && s.b[719]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[719])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[720] = (s.v[9] == 0.5);
        s.v[720] = if s.b[720] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && s.b[720]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && (!s.b[720])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[719])) {
            s.store_add(442, 440, 441);
        }

        s.b[721] = (s.v[9] == 0.5);
        s.v[721] = if s.b[721] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && s.b[721]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[719])) && (!s.b[721])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[719])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_lhs(444, s.ad_value(98), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[722] = (s.v[23] == 0.0);
        s.v[722] = if s.b[722] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[717])) && s.b[722]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[122] * s.v[152]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[723] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[723] = if s.b[723] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[723]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[723])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[149]), s.ad_value(447), s.ad_value(450), s.v[149], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[724] = (s.v[457] > 0.0);
        s.v[724] = if s.b[724] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[724]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[724])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[725] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[725] = if s.b[725] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[725]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[725])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[726] = (s.v[457] > 0.0);
        s.v[726] = if s.b[726] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && s.b[726]) {
            s.copy_ad(458, 421);
        }

        s.b[727] = (s.v[456] > (-230.25850929940458));
        s.v[727] = if s.b[727] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[726])) && s.b[727]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[726])) && (!s.b[727])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[722])) && (!s.b[726])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[722])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[149] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);
        }

        s.b[728] = (s.v[29] == 0.0);
        s.v[728] = if s.b[728] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[717])) && s.b[728]) {
            s.store_scalar(460, 0.0);
        }

        s.b[729] = (s.v[9] == 0.5);
        s.v[729] = if s.b[729] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[728])) && s.b[729]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[728])) && (!s.b[729])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[143]), ((s.v[6]) * (s.v[143]))), s.v[9]);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[728])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), s.ad_value(436), 1.0);
        }

        s.b[730] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[730] = if s.b[730] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[717])) && (!s.b[728])) && s.b[730]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));
        }

        s.b[731] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[731] = if s.b[731] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[717])) && (!s.b[728])) && (!s.b[730])) && s.b[731]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[717])) && (!s.b[728])) && (!s.b[730])) && (!s.b[731])) {
            let assign21660_ad_e31565: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign21660_ad_e31565, 1e100);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[728])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(193), s.ad_value(461), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[732] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[732] = if s.b[732] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[717])) && s.b[732]) {
            s.store_scalar(462, 1.0);
        }

        s.b[733] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[733] = if s.b[733] { 1.0 } else { 0.0 };

        s.b[734] = (s.v[41] == 4.0);
        s.v[734] = if s.b[734] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[717])) && (!s.b[732])) && s.b[733]) && s.b[734]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[717])) && (!s.b[732])) && s.b[733]) && (!s.b[734])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[732])) && s.b[733]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[732])) && (!s.b[733])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[717])) {
            s.store_mul_ad_lhs(268, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(291, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[735] = (s.v[257] == 0.0);
        s.v[735] = if s.b[735] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[735]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[736] = (s.v[123] == 0.5);
        s.v[736] = if s.b[736] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[735])) && s.b[736]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[736])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[735])) {
            s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(193), s.ad_value(428)), 1.0);
            s.store_mul(437, 102, 371);
        }

        s.b[737] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[737] = if s.b[737] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[735])) && s.b[737]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[737])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[738] = (s.v[10] == 0.5);
        s.v[738] = if s.b[738] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && s.b[738]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && (!s.b[738])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[737])) {
            s.store_add(442, 440, 441);
        }

        s.b[739] = (s.v[10] == 0.5);
        s.v[739] = if s.b[739] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && s.b[739]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[737])) && (!s.b[739])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[737])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_lhs(444, s.ad_value(99), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[740] = (s.v[24] == 0.0);
        s.v[740] = if s.b[740] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[735])) && s.b[740]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[123] * s.v[153]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[741] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[741] = if s.b[741] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[741]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[741])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[150]), s.ad_value(447), s.ad_value(450), s.v[150], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[742] = (s.v[457] > 0.0);
        s.v[742] = if s.b[742] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[742]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[742])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[743] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[743] = if s.b[743] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[743]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[743])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[744] = (s.v[457] > 0.0);
        s.v[744] = if s.b[744] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && s.b[744]) {
            s.copy_ad(458, 421);
        }

        s.b[745] = (s.v[456] > (-230.25850929940458));
        s.v[745] = if s.b[745] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[744])) && s.b[745]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[744])) && (!s.b[745])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[740])) && (!s.b[744])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[740])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[150] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);
        }

        s.b[746] = (s.v[30] == 0.0);
        s.v[746] = if s.b[746] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[735])) && s.b[746]) {
            s.store_scalar(460, 0.0);
        }

        s.b[747] = (s.v[10] == 0.5);
        s.v[747] = if s.b[747] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[746])) && s.b[747]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[735])) && (!s.b[746])) && (!s.b[747])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[746])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), s.ad_value(436), 1.0);
        }

        s.b[748] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[748] = if s.b[748] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[735])) && (!s.b[746])) && s.b[748]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));
        }

        s.b[749] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[749] = if s.b[749] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[735])) && (!s.b[746])) && (!s.b[748])) && s.b[749]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[735])) && (!s.b[746])) && (!s.b[748])) && (!s.b[749])) {
            let assign22470_ad_e32721: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign22470_ad_e32721, 1e100);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[746])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(193), s.ad_value(461), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[750] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[750] = if s.b[750] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[735])) && s.b[750]) {
            s.store_scalar(462, 1.0);
        }

        s.b[751] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[751] = if s.b[751] { 1.0 } else { 0.0 };

        s.b[752] = (s.v[42] == 4.0);
        s.v[752] = if s.b[752] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[735])) && (!s.b[750])) && s.b[751]) && s.b[752]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[735])) && (!s.b[750])) && s.b[751]) && (!s.b[752])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[750])) && s.b[751]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[750])) && (!s.b[751])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[735])) {
            s.store_mul_ad_lhs(270, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(292, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        s.b[753] = (s.v[258] == 0.0);
        s.v[753] = if s.b[753] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[753]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[754] = (s.v[124] == 0.5);
        s.v[754] = if s.b[754] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[753])) && s.b[754]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[754])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[753])) {
            s.store_add_scaled_product_mixed_aia(273, A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, 136, A::sub(s.ad_value(193), s.ad_value(428)), 1.0);
            s.store_mul(437, 103, 372);
        }

        s.b[755] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[755] = if s.b[755] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[753])) && s.b[755]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[755])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[756] = (s.v[11] == 0.5);
        s.v[756] = if s.b[756] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && s.b[756]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && (!s.b[756])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[755])) {
            s.store_add(442, 440, 441);
        }

        s.b[757] = (s.v[11] == 0.5);
        s.v[757] = if s.b[757] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && s.b[757]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[755])) && (!s.b[757])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[755])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_lhs(444, s.ad_value(100), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[758] = (s.v[25] == 0.0);
        s.v[758] = if s.b[758] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[753])) && s.b[758]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {
            s.store_div_scaled_inputs(446, s.ad_value(443), (s.v[124] * s.v[154]), s.ad_value(439), 1.0);
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[759] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[759] = if s.b[759] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[759]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[759])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {
            s.store_div_scaled_product_denominator_ad(453, 442, 452, 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0);
            s.store_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);
            s.store_add_scaled_value_products(456, s.ad_value(449), (-s.v[151]), s.ad_value(447), s.ad_value(450), s.v[151], s.ad_value(446), s.ad_value(451), 0.5);
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[760] = (s.v[457] > 0.0);
        s.v[760] = if s.b[760] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[760]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[760])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[761] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[761] = if s.b[761] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[761]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[761])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[762] = (s.v[457] > 0.0);
        s.v[762] = if s.b[762] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && s.b[762]) {
            s.copy_ad(458, 421);
        }

        s.b[763] = (s.v[456] > (-230.25850929940458));
        s.v[763] = if s.b[763] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[762])) && s.b[763]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[762])) && (!s.b[763])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[758])) && (!s.b[762])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[758])) {
            s.store_div_scaled_inputs(459, s.ad_value(458), (s.v[151] * (1.772453850905516 * 0.5)), s.ad_value(454), 1.0);
            s.store_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);
        }

        s.b[764] = (s.v[31] == 0.0);
        s.v[764] = if s.b[764] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[753])) && s.b[764]) {
            s.store_scalar(460, 0.0);
        }

        s.b[765] = (s.v[11] == 0.5);
        s.v[765] = if s.b[765] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[764])) && s.b[765]) {
            s.store_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[764])) && (!s.b[765])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[145]), ((s.v[8]) * (s.v[145]))), s.v[11]);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[764])) {
            s.store_div_scaled_offset_numerator(461, s.ad_value(434), ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), s.ad_value(436), 1.0);
        }

        s.b[766] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[766] = if s.b[766] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[753])) && (!s.b[764])) && s.b[766]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));
        }

        s.b[767] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[767] = if s.b[767] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[753])) && (!s.b[764])) && (!s.b[766])) && s.b[767]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[753])) && (!s.b[764])) && (!s.b[766])) && (!s.b[767])) {
            let assign23280_ad_e33877: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign23280_ad_e33877, 1e100);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[764])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(193), s.ad_value(461), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[768] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[768] = if s.b[768] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[753])) && s.b[768]) {
            s.store_scalar(462, 1.0);
        }

        s.b[769] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[769] = if s.b[769] { 1.0 } else { 0.0 };

        s.b[770] = (s.v[43] == 4.0);
        s.v[770] = if s.b[770] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[753])) && (!s.b[768])) && s.b[769]) && s.b[770]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[753])) && (!s.b[768])) && s.b[769]) && (!s.b[770])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[768])) && s.b[769]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[768])) && (!s.b[769])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[753])) {
            s.store_mul_ad_lhs(272, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_add_scaled_inputs3_offset_rhs(293, 462, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0, 0.0);
        }

        if s.b[418] {
            s.store_add_scaled_inputs3(183, s.ad_value(268), s.v[256], s.ad_value(270), s.v[257], s.ad_value(272), s.v[258]);
        }

        s.b[771] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[771] = if s.b[771] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[771]) {
            s.store_scaled_mul(422, 265, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_scaled_product_indices(424, 194, 1.0, 265, 423, 1.0);
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_div_scaled_product_denominator_ad(428, 194, 266, 2.0, A::add(s.ad_value(425), s.ad_value(427)), 1.0);
        }

        s.b[772] = (s.v[194] < s.v[262]);
        s.v[772] = if s.b[772] { 1.0 } else { 0.0 };

        s.b[773] = ((((0.5 * (s.v[194] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[773] = if s.b[773] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[773]) {
            s.store_exp_scaled_input(430, 194, (s.v[85] * 0.5));
        }

        s.b[774] = ((0.5 * (s.v[194] * s.v[85])) < (-230.25850929940458));
        s.v[774] = if s.b[774] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[773])) && s.b[774]) {
            s.store_div_from_scalar_offset_ad(430, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(194), (s.v[85] * 0.5)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(194), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[773])) && (!s.b[774])) {
            s.store_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(194), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(194), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(194), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[775] = (s.v[62] < p.p85);
        s.v[775] = if s.b[775] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_offset_scaled_sub(360, 194, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            let assign23850_ad_e34786: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign23850_ad_e34786, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && (!s.b[777])) {
            let assign23860_ad_e34862: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(370, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign23860_ad_e34862, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[778] = (s.v[64] < p.p85);
        s.v[778] = if s.b[778] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_offset_scaled_sub(360, 194, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[778])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[779] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[779] = if s.b[779] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[779]) {
            s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[780] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[780] = if s.b[780] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[779])) && s.b[780]) {
            let assign24170_ad_e35387: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, assign24170_ad_e35387, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[779])) && (!s.b[780])) {
            let assign24180_ad_e35463: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(371, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign24180_ad_e35463, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[781] = (s.v[63] < p.p85);
        s.v[781] = if s.b[781] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_offset_scaled_sub(360, 194, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[781])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[782] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[782] = if s.b[782] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[782]) {
            s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[783] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[783] = if s.b[783] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[782])) && s.b[783]) {
            let assign24490_ad_e35988: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, assign24490_ad_e35988, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[782])) && (!s.b[783])) {
            let assign24500_ad_e36064: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(372, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign24500_ad_e36064, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(194), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[784] = (s.v[62] < p.p85);
        s.v[784] = if s.b[784] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
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
            let assign24860_ad_e36696: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign24860_ad_e36696, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && (!s.b[786])) {
            let assign24870_ad_e36773: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(281, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign24870_ad_e36773, 1.0)), 1.0, 1e100);
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
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[787])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[788] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[788] = if s.b[788] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[788]) {
            s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[789] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[789] = if s.b[789] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[788])) && s.b[789]) {
            let assign25240_ad_e37431: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, assign25240_ad_e37431, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[788])) && (!s.b[789])) {
            let assign25250_ad_e37508: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(282, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign25250_ad_e37508, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(371, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[790] = (s.v[63] < p.p85);
        s.v[790] = if s.b[790] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
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
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(359, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
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
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
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
            s.store_offset_add_scaled_inputs(350, s.ad_value(314), 0.5, s.ad_value(315), 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[790])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[791] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[791] = if s.b[791] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[791]) {
            s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[792] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[792] = if s.b[792] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[791])) && s.b[792]) {
            let assign25620_ad_e38166: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, assign25620_ad_e38166, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[791])) && (!s.b[792])) {
            let assign25630_ad_e38243: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(283, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign25630_ad_e38243, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(372, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 283);
        }

        if (s.b[418] && s.b[771]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[793] = (s.v[194] > 0.0);
        s.v[793] = if s.b[793] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[771]) && s.b[793]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[771]) && (!s.b[793])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 194);
        }

        if (s.b[418] && s.b[771]) {
            s.store_sub(432, 264, 431);
            s.store_add_scaled_inputs3(433, s.ad_value(194), 0.5, s.ad_value(432), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(194), s.ad_value(432)), A::sub(s.ad_value(194), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), (-0.5));
            s.store_add_scaled_inputs3(434, s.ad_value(194), 0.5, s.ad_value(267), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(194), s.ad_value(267)), A::sub(s.ad_value(194), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0)), (-0.5));
            s.store_scaled_sub_ad_rhs(435, 194, A::sqrt(A::offset(A::mul(s.ad_value(194), s.ad_value(194)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[771])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[794] = (s.v[256] == 0.0);
        s.v[794] = if s.b[794] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[794]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[795] = (s.v[122] == 0.5);
        s.v[795] = if s.b[795] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[794])) && s.b[795]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[795])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[794])) {
            s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(194), s.ad_value(428)), 1.0);
            s.store_mul(437, 101, 370);
        }

        s.b[796] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[796] = if s.b[796] { 1.0 } else { 0.0 };

    }
}
