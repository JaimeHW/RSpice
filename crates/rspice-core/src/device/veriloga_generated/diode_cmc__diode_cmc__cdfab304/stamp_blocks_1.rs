#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[418] && (!s.b[694])) {
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
            s.store_ad_value(269, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, s.ad_value(134), A::sub(s.ad_value(193), s.ad_value(428)), 1.0));
            s.store_mul(437, 101, 370);
        }

        s.b[719] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[719] = if s.b[719] { 1.0 } else { 0.0 };

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
            s.store_scaled_div(446, 443, 439, (s.v[122] * s.v[152]));
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
            s.store_ad_value(453, A::div_scaled_product(s.ad_value(442), s.ad_value(452), 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_ad_value(455, A::add_scaled_product(s.ad_value(449), (-1.0), s.ad_value(447), s.ad_value(450), 2.0));
            s.store_ad_value(456, A::add_scaled_value_products(s.ad_value(449), (-s.v[149]), s.ad_value(447), s.ad_value(450), s.v[149], s.ad_value(446), s.ad_value(451), 0.5));
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
            s.store_scaled_div(459, 458, 454, (s.v[149] * (1.772453850905516 * 0.5)));
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
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[717])) && (!s.b[728])) && (!s.b[729])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[143]), ((s.v[6]) * (s.v[143]))), s.v[9]);
        }

        if ((s.b[418] && (!s.b[717])) && (!s.b[728])) {
            s.store_scaled_div_ad_lhs(461, A::scale_offset(s.ad_value(434), (-s.v[140]), ((s.v[6]) * (s.v[140]))), 436, s.v[125]);
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
            s.store_mul_ad_lhs(291, A::add_scaled_inputs3(s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
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
            s.store_ad_value(271, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, s.ad_value(135), A::sub(s.ad_value(193), s.ad_value(428)), 1.0));
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
            s.store_scaled_div(446, 443, 439, (s.v[123] * s.v[153]));
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
            s.store_ad_value(453, A::div_scaled_product(s.ad_value(442), s.ad_value(452), 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_ad_value(455, A::add_scaled_product(s.ad_value(449), (-1.0), s.ad_value(447), s.ad_value(450), 2.0));
            s.store_ad_value(456, A::add_scaled_value_products(s.ad_value(449), (-s.v[150]), s.ad_value(447), s.ad_value(450), s.v[150], s.ad_value(446), s.ad_value(451), 0.5));
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

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_scaled_div(459, 458, 454, (s.v[150] * (1.772453850905516 * 0.5)));
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
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

        if (((s.b[418] && (!s.b[735])) && (!s.b[746])) && (!s.b[747])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if ((s.b[418] && (!s.b[735])) && (!s.b[746])) {
            s.store_scaled_div_ad_lhs(461, A::scale_offset(s.ad_value(434), (-s.v[141]), ((s.v[7]) * (s.v[141]))), 436, s.v[126]);
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
            s.store_mul_ad_lhs(292, A::add_scaled_inputs3(s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
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
            s.store_ad_value(273, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, s.ad_value(136), A::sub(s.ad_value(193), s.ad_value(428)), 1.0));
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
            s.store_scaled_div(446, 443, 439, (s.v[124] * s.v[154]));
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
            s.store_ad_value(453, A::div_scaled_product(s.ad_value(442), s.ad_value(452), 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_ad_value(455, A::add_scaled_product(s.ad_value(449), (-1.0), s.ad_value(447), s.ad_value(450), 2.0));
            s.store_ad_value(456, A::add_scaled_value_products(s.ad_value(449), (-s.v[151]), s.ad_value(447), s.ad_value(450), s.v[151], s.ad_value(446), s.ad_value(451), 0.5));
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
            s.store_scaled_div(459, 458, 454, (s.v[151] * (1.772453850905516 * 0.5)));
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
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[753])) && (!s.b[764])) && (!s.b[765])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[145]), ((s.v[8]) * (s.v[145]))), s.v[11]);
        }

        if ((s.b[418] && (!s.b[753])) && (!s.b[764])) {
            s.store_scaled_div_ad_lhs(461, A::scale_offset(s.ad_value(434), (-s.v[142]), ((s.v[8]) * (s.v[142]))), 436, s.v[127]);
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
            s.store_mul_ad_lhs(293, A::add_scaled_inputs3(s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
        }

        if s.b[418] {
            s.store_ad_value(183, A::add_scaled_inputs3(s.ad_value(268), s.v[256], s.ad_value(270), s.v[257], s.ad_value(272), s.v[258]));
        }

        s.b[771] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[771] = if s.b[771] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[771]) {
            s.store_scaled_mul(422, 265, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_ad_value(424, A::add_scaled_product(s.ad_value(194), 1.0, s.ad_value(265), s.ad_value(423), 1.0));
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_ad_value(428, A::div_scaled_product(s.ad_value(194), s.ad_value(266), 2.0, A::add(s.ad_value(425), s.ad_value(427)), 1.0));
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
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[775])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[776] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[776] = if s.b[776] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[776]) {
            s.store_ad_value(370, A::exp_scaled_input(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
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
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[778])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[779] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[779] = if s.b[779] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[779]) {
            s.store_ad_value(371, A::exp_scaled_input(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
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
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[781])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[782] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[782] = if s.b[782] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[782]) {
            s.store_ad_value(372, A::exp_scaled_input(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
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
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
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
            s.store_ad_value(281, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
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
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[787] = (s.v[64] < p.p85);
        s.v[787] = if s.b[787] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
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
            s.store_ad_value(282, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
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
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(371, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[790] = (s.v[63] < p.p85);
        s.v[790] = if s.b[790] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
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
            s.store_ad_value(283, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
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
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
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
            s.store_ad_value(433, A::add_scaled_inputs3(s.ad_value(194), 0.5, s.ad_value(432), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(194), s.ad_value(432)), A::sub(s.ad_value(194), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), (-0.5)));
            s.store_ad_value(434, A::add_scaled_inputs3(s.ad_value(194), 0.5, s.ad_value(267), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(194), s.ad_value(267)), A::sub(s.ad_value(194), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0)), (-0.5)));
            s.store_scaled_sub_ad_rhs(435, 194, A::sqrt(A::offset(A::mul(s.ad_value(194), s.ad_value(194)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[771])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(428, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[418] && (!s.b[771])) {
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
            s.store_ad_value(269, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, s.ad_value(134), A::sub(s.ad_value(194), s.ad_value(428)), 1.0));
            s.store_mul(437, 101, 370);
        }

        s.b[796] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[796] = if s.b[796] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[794])) && s.b[796]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[796])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[797] = (s.v[9] == 0.5);
        s.v[797] = if s.b[797] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && s.b[797]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && (!s.b[797])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[796])) {
            s.store_add(442, 440, 441);
        }

        s.b[798] = (s.v[9] == 0.5);
        s.v[798] = if s.b[798] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && s.b[798]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[796])) && (!s.b[798])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[796])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_lhs(444, s.ad_value(98), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[799] = (s.v[23] == 0.0);
        s.v[799] = if s.b[799] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[794])) && s.b[799]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {
            s.store_scaled_div(446, 443, 439, (s.v[122] * s.v[152]));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[800] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[800] = if s.b[800] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[800]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[800])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {
            s.store_ad_value(453, A::div_scaled_product(s.ad_value(442), s.ad_value(452), 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_ad_value(455, A::add_scaled_product(s.ad_value(449), (-1.0), s.ad_value(447), s.ad_value(450), 2.0));
            s.store_ad_value(456, A::add_scaled_value_products(s.ad_value(449), (-s.v[149]), s.ad_value(447), s.ad_value(450), s.v[149], s.ad_value(446), s.ad_value(451), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[801] = (s.v[457] > 0.0);
        s.v[801] = if s.b[801] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[801]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[801])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[802] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[802] = if s.b[802] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[802]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[802])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[803] = (s.v[457] > 0.0);
        s.v[803] = if s.b[803] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && s.b[803]) {
            s.copy_ad(458, 421);
        }

        s.b[804] = (s.v[456] > (-230.25850929940458));
        s.v[804] = if s.b[804] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[803])) && s.b[804]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[803])) && (!s.b[804])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[799])) && (!s.b[803])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[799])) {
            s.store_scaled_div(459, 458, 454, (s.v[149] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);
        }

        s.b[805] = (s.v[29] == 0.0);
        s.v[805] = if s.b[805] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[794])) && s.b[805]) {
            s.store_scalar(460, 0.0);
        }

        s.b[806] = (s.v[9] == 0.5);
        s.v[806] = if s.b[806] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[805])) && s.b[806]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[805])) && (!s.b[806])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[143]), ((s.v[6]) * (s.v[143]))), s.v[9]);
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[805])) {
            s.store_scaled_div_ad_lhs(461, A::scale_offset(s.ad_value(434), (-s.v[140]), ((s.v[6]) * (s.v[140]))), 436, s.v[125]);
        }

        s.b[807] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[794])) && (!s.b[805])) && s.b[807]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));
        }

        s.b[808] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[794])) && (!s.b[805])) && (!s.b[807])) && s.b[808]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[794])) && (!s.b[805])) && (!s.b[807])) && (!s.b[808])) {
            let assign26550_ad_e39523: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign26550_ad_e39523, 1e100);
        }

        if ((s.b[418] && (!s.b[794])) && (!s.b[805])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(194), s.ad_value(461), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[809] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[809] = if s.b[809] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[794])) && s.b[809]) {
            s.store_scalar(462, 1.0);
        }

        s.b[810] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[810] = if s.b[810] { 1.0 } else { 0.0 };

        s.b[811] = (s.v[41] == 4.0);
        s.v[811] = if s.b[811] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[794])) && (!s.b[809])) && s.b[810]) && s.b[811]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[794])) && (!s.b[809])) && s.b[810]) && (!s.b[811])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[809])) && s.b[810]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[794])) && (!s.b[809])) && (!s.b[810])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[794])) {
            s.store_mul_ad_lhs(268, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_ad_lhs(291, A::add_scaled_inputs3(s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
        }

        s.b[812] = (s.v[257] == 0.0);
        s.v[812] = if s.b[812] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[812]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[813] = (s.v[123] == 0.5);
        s.v[813] = if s.b[813] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[812])) && s.b[813]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[813])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[812])) {
            s.store_ad_value(271, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, s.ad_value(135), A::sub(s.ad_value(194), s.ad_value(428)), 1.0));
            s.store_mul(437, 102, 371);
        }

        s.b[814] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[814] = if s.b[814] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[812])) && s.b[814]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[814])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[815] = (s.v[10] == 0.5);
        s.v[815] = if s.b[815] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && s.b[815]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && (!s.b[815])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[814])) {
            s.store_add(442, 440, 441);
        }

        s.b[816] = (s.v[10] == 0.5);
        s.v[816] = if s.b[816] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && s.b[816]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[814])) && (!s.b[816])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[814])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_lhs(444, s.ad_value(99), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[817] = (s.v[24] == 0.0);
        s.v[817] = if s.b[817] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[812])) && s.b[817]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {
            s.store_scaled_div(446, 443, 439, (s.v[123] * s.v[153]));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[818] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[818] = if s.b[818] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[818]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[818])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {
            s.store_ad_value(453, A::div_scaled_product(s.ad_value(442), s.ad_value(452), 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_ad_value(455, A::add_scaled_product(s.ad_value(449), (-1.0), s.ad_value(447), s.ad_value(450), 2.0));
            s.store_ad_value(456, A::add_scaled_value_products(s.ad_value(449), (-s.v[150]), s.ad_value(447), s.ad_value(450), s.v[150], s.ad_value(446), s.ad_value(451), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[819] = (s.v[457] > 0.0);
        s.v[819] = if s.b[819] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[819]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[819])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[820] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[820] = if s.b[820] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[820]) {
            s.store_exp_sub(436, 456, 419);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[820])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[821] = (s.v[457] > 0.0);
        s.v[821] = if s.b[821] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && s.b[821]) {
            s.copy_ad(458, 421);
        }

        s.b[822] = (s.v[456] > (-230.25850929940458));
        s.v[822] = if s.b[822] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[821])) && s.b[822]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[821])) && (!s.b[822])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[817])) && (!s.b[821])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[817])) {
            s.store_scaled_div(459, 458, 454, (s.v[150] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);
        }

        s.b[823] = (s.v[30] == 0.0);
        s.v[823] = if s.b[823] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[812])) && s.b[823]) {
            s.store_scalar(460, 0.0);
        }

        s.b[824] = (s.v[10] == 0.5);
        s.v[824] = if s.b[824] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[823])) && s.b[824]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[823])) && (!s.b[824])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[823])) {
            s.store_scaled_div_ad_lhs(461, A::scale_offset(s.ad_value(434), (-s.v[141]), ((s.v[7]) * (s.v[141]))), 436, s.v[126]);
        }

        s.b[825] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[825] = if s.b[825] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[812])) && (!s.b[823])) && s.b[825]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));
        }

        s.b[826] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[826] = if s.b[826] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[812])) && (!s.b[823])) && (!s.b[825])) && s.b[826]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[812])) && (!s.b[823])) && (!s.b[825])) && (!s.b[826])) {
            let assign27360_ad_e40679: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign27360_ad_e40679, 1e100);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[823])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(194), s.ad_value(461), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[827] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[827] = if s.b[827] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[812])) && s.b[827]) {
            s.store_scalar(462, 1.0);
        }

        s.b[828] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[828] = if s.b[828] { 1.0 } else { 0.0 };

        s.b[829] = (s.v[42] == 4.0);
        s.v[829] = if s.b[829] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[812])) && (!s.b[827])) && s.b[828]) && s.b[829]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[812])) && (!s.b[827])) && s.b[828]) && (!s.b[829])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[827])) && s.b[828]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[812])) && (!s.b[827])) && (!s.b[828])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[812])) {
            s.store_mul_ad_lhs(270, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_ad_lhs(292, A::add_scaled_inputs3(s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
        }

        s.b[830] = (s.v[258] == 0.0);
        s.v[830] = if s.b[830] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[830]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[831] = (s.v[124] == 0.5);
        s.v[831] = if s.b[831] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[830])) && s.b[831]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[831])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[830])) {
            s.store_ad_value(273, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(436)), 1.0, s.ad_value(136), A::sub(s.ad_value(194), s.ad_value(428)), 1.0));
            s.store_mul(437, 103, 372);
        }

        s.b[832] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[832] = if s.b[832] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[830])) && s.b[832]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[832])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[833] = (s.v[11] == 0.5);
        s.v[833] = if s.b[833] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && s.b[833]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && (!s.b[833])) {
            s.store_scaled_add_ad_lhs(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[832])) {
            s.store_add(442, 440, 441);
        }

        s.b[834] = (s.v[11] == 0.5);
        s.v[834] = if s.b[834] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && s.b[834]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[832])) && (!s.b[834])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[832])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_lhs(444, s.ad_value(100), A::offset(s.ad_value(430), (-1.0)), 443);
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[835] = (s.v[25] == 0.0);
        s.v[835] = if s.b[835] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[830])) && s.b[835]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {
            s.store_scaled_div(446, 443, 439, (s.v[124] * s.v[154]));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_ad(449, A::div_scaled_product_offset_denominator(s.ad_value(448), s.ad_value(448), 1.0, A::square(s.ad_value(448)), 1.0, 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[836] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[836] = if s.b[836] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[836]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[836])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {
            s.store_ad_value(453, A::div_scaled_product(s.ad_value(442), s.ad_value(452), 1.0, A::add(s.ad_value(442), s.ad_value(452)), 1.0));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_ad_value(455, A::add_scaled_product(s.ad_value(449), (-1.0), s.ad_value(447), s.ad_value(450), 2.0));
            s.store_ad_value(456, A::add_scaled_value_products(s.ad_value(449), (-s.v[151]), s.ad_value(447), s.ad_value(450), s.v[151], s.ad_value(446), s.ad_value(451), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[837] = (s.v[457] > 0.0);
        s.v[837] = if s.b[837] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[837]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[837])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[838] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[838] = if s.b[838] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[838]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[838])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {
            s.store_mul_ad_lhs(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);
        }

        s.b[839] = (s.v[457] > 0.0);
        s.v[839] = if s.b[839] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && s.b[839]) {
            s.copy_ad(458, 421);
        }

        s.b[840] = (s.v[456] > (-230.25850929940458));
        s.v[840] = if s.b[840] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[839])) && s.b[840]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[839])) && (!s.b[840])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(456), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(456), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[835])) && (!s.b[839])) {
            s.store_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[835])) {
            s.store_scaled_div(459, 458, 454, (s.v[151] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(445, 444, 459, s.v[25], 0.0, 453);
        }

        s.b[841] = (s.v[31] == 0.0);
        s.v[841] = if s.b[841] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[830])) && s.b[841]) {
            s.store_scalar(460, 0.0);
        }

        s.b[842] = (s.v[11] == 0.5);
        s.v[842] = if s.b[842] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[841])) && s.b[842]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[841])) && (!s.b[842])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[145]), ((s.v[8]) * (s.v[145]))), s.v[11]);
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[841])) {
            s.store_scaled_div_ad_lhs(461, A::scale_offset(s.ad_value(434), (-s.v[142]), ((s.v[8]) * (s.v[142]))), 436, s.v[127]);
        }

        s.b[843] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[843] = if s.b[843] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[830])) && (!s.b[841])) && s.b[843]) {
            s.store_exp_ad(436, A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0));
        }

        s.b[844] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[844] = if s.b[844] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[830])) && (!s.b[841])) && (!s.b[843])) && s.b[844]) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && (!s.b[830])) && (!s.b[841])) && (!s.b[843])) && (!s.b[844])) {
            let assign28170_ad_e41835: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(461), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(436, assign28170_ad_e41835, 1e100);
        }

        if ((s.b[418] && (!s.b[830])) && (!s.b[841])) {
            s.store_mul_scaled_ad_lhs(460, A::mul3(s.ad_value(194), s.ad_value(461), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[845] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[845] = if s.b[845] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[830])) && s.b[845]) {
            s.store_scalar(462, 1.0);
        }

        s.b[846] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[846] = if s.b[846] { 1.0 } else { 0.0 };

        s.b[847] = (s.v[43] == 4.0);
        s.v[847] = if s.b[847] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[830])) && (!s.b[845])) && s.b[846]) && s.b[847]) {
            s.store_mul_ad(436, A::mul3(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[830])) && (!s.b[845])) && s.b[846]) && (!s.b[847])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[845])) && s.b[846]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[830])) && (!s.b[845])) && (!s.b[846])) {
            s.store_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[830])) {
            s.store_mul_ad_lhs(272, A::add_scaled_inputs4(s.ad_value(437), 1.0, s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
            s.store_mul_ad_lhs(293, A::add_scaled_inputs3(s.ad_value(438), 1.0, s.ad_value(445), 1.0, s.ad_value(460), 1.0), 462);
        }

        if s.b[418] {
            s.store_ad_value(184, A::add_scaled_inputs3(s.ad_value(268), s.v[256], s.ad_value(270), s.v[257], s.ad_value(272), s.v[258]));
            s.copy_ad(300, 289);
            s.store_ad_value(188, A::add_scaled_offset_product_rhs(s.ad_value(183), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(193), s.v[85], s.ad_value(301))), (-1.0), (-1.0)));
            s.store_ad_value(189, A::add_scaled_offset_product_rhs(s.ad_value(184), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(194), s.v[85], s.ad_value(301))), (-1.0), (-1.0)));
        }

        s.b[848] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[848] = if s.b[848] { 1.0 } else { 0.0 };

        s.b[849] = ((s.v[183] > 0.0) && (s.v[184] > 0.0));
        s.v[849] = if s.b[849] { 1.0 } else { 0.0 };

        s.b[850] = (((((s.v[188] / s.v[183]) > 0.001) || ((s.v[189] / s.v[184]) > 0.001)) && (s.v[188] > 0.0)) && (s.v[189] > 0.0));
        s.v[850] = if s.b[850] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[848]) && s.b[849]) && s.b[850]) {
            s.store_div(195, 188, 189);
            s.store_ad_value(303, A::div_scaled_inputs(A::ln(s.ad_value(195)), s.v[84], A::sub(s.ad_value(193), s.ad_value(194)), 1.0));
            s.store_div_ad_rhs(302, 188, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(193), s.v[85], s.ad_value(303))), (-1.0)));
        }

        if (s.b[418] && s.b[848]) {
            s.store_ad_value(185, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(180), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(190), s.v[85], s.ad_value(301))), (-1.0), (-1.0)), 1.0, s.ad_value(302), A::exp(A::mul_scaled_lhs(s.ad_value(190), s.v[85], s.ad_value(303))), (-1.0), (-1.0)));
            s.store_ad_value(186, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(181), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(191), s.v[85], s.ad_value(301))), (-1.0), (-1.0)), 1.0, s.ad_value(302), A::exp(A::mul_scaled_lhs(s.ad_value(191), s.v[85], s.ad_value(303))), (-1.0), (-1.0)));
            s.store_ad_value(187, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(182), 1.0, s.ad_value(300), A::exp(A::mul_scaled_lhs(s.ad_value(192), s.v[85], s.ad_value(301))), (-1.0), (-1.0)), 1.0, s.ad_value(302), A::exp(A::mul_scaled_lhs(s.ad_value(192), s.v[85], s.ad_value(303))), (-1.0), (-1.0)));
        }

        s.b[851] = (((s.v[180] < 0.0) && (s.v[181] < 0.0)) && (s.v[182] < 0.0));
        s.v[851] = if s.b[851] { 1.0 } else { 0.0 };

        s.b[852] = (((((((s.v[185] / s.v[180]) > 0.001) || ((s.v[186] / s.v[181]) > 0.001)) || ((s.v[187] / s.v[182]) > 0.001)) && (s.v[185] < 0.0)) && (s.v[186] < 0.0)) && (s.v[187] < 0.0));
        s.v[852] = if s.b[852] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) {
            s.store_div(195, 185, 186);
            s.store_ad_value(196, A::div_scaled_inputs(A::ln(s.ad_value(195)), (-s.v[84]), A::sub(s.ad_value(190), s.ad_value(191)), 1.0));
            s.store_div_ad_rhs(198, 191, A::sub(s.ad_value(191), s.ad_value(190)));
            s.store_scaled_mul_ad(199, A::offset(s.ad_value(195), (-1.0)), A::offset(A::pow(s.ad_value(195), s.ad_value(198)), (-1.0)), s.v[84]);
            s.store_div_ad_rhs(198, 190, A::sub(s.ad_value(190), s.ad_value(191)));
            s.store_sub_ad_lhs(200, A::add_scaled_products(A::pow(s.ad_value(195), s.ad_value(198)), A::sub(s.ad_value(191), s.ad_value(190)), 1.0, s.ad_value(195), s.ad_value(190), 1.0), 191);
            s.store_div(197, 199, 200);
        }

    }

    pub(super) fn stamp_transient_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) {
            s.store_add(305, 196, 197);
        }

        s.b[853] = (((((s.v[192] * s.v[85]) * s.v[305])) as f64).abs() < 1e-6);
        s.v[853] = if s.b[853] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) && s.b[853]) {
            s.store_scalar(306, 1.0);
            s.store_mul_ad_rhs(304, 187, A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(192)), 1.0, s.ad_value(305), (0.5 * s.v[85])));
            s.store_ad_value(305, A::div_scaled_product(s.ad_value(187), s.ad_value(305), ((-0.5) * s.v[85]), s.ad_value(192), 1.0));
        }

        if ((((s.b[418] && s.b[848]) && s.b[851]) && s.b[852]) && (!s.b[853])) {
            s.store_scalar(306, 0.0);
            s.store_ad_value(304, A::div_scaled_inputs(s.ad_value(187), -1.0, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(192), (-s.v[85]), s.ad_value(305))), (-1.0)), 1.0));
        }

        if s.b[418] {
            s.store_ad_value(208, A::add_scaled_inputs3(s.ad_value(128), (s.v[256] * s.v[47]), s.ad_value(129), (s.v[257] * s.v[47]), s.ad_value(130), (s.v[258] * s.v[47])));
        }

        s.b[854] = ((s.v[256] * s.v[128]) <= s.v[208]);
        s.v[854] = if s.b[854] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[854]) {
            s.store_scalar(259, 0.0);
        }

        s.b[855] = ((s.v[257] * s.v[129]) <= s.v[208]);
        s.v[855] = if s.b[855] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[855]) {
            s.store_scalar(260, 0.0);
        }

        s.b[856] = ((s.v[258] * s.v[130]) <= s.v[208]);
        s.v[856] = if s.b[856] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[856]) {
            s.store_scalar(261, 0.0);
        }

        s.b[857] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[857] = if s.b[857] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[857]) {
            s.store_ln_ad(294, A::div_from_scalar((0.5 * s.v[2]), A::offset(s.ad_value(300), 1e-21)));
            s.store_ln_ad(296, A::div_from_scalar((0.5 * s.v[2]), A::offset(s.ad_value(302), 1e-21)));
            s.store_ln_ad(298, A::div_from_scalar((0.5 * s.v[2]), A::offset(A::abs(s.ad_value(304)), 1e-21)));
        }

        if s.b[418] {
            s.store_min_with_scalar(294, 294, 230.25850929940458);
            s.store_exp(295, 294);
            s.store_min_with_scalar(296, 296, 230.25850929940458);
            s.store_exp(297, 296);
            s.store_min_with_scalar(298, 298, 230.25850929940458);
            s.store_exp(299, 298);
        }

        s.store_voltage(277, ctx, nodes, Some(0), Some(2));

        s.b[858] = (s.v[45] == 1.0);
        s.v[858] = if s.b[858] { 1.0 } else { 0.0 };

        if s.b[858] {
            s.store_scaled_mul(201, 277, 301, s.v[85]);
        }

        if s.b[858] {
            s.store_ad_value(202, {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[294]) {
                            A::mul_offset_rhs(s.ad_value(295), A::sub(s.ad_value(201), s.ad_value(294)), 1.0)
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            });
        }

        if s.b[858] {
            s.store_mul_offset_rhs(209, 300, 202, (-1.0));
            s.store_scaled_mul(201, 277, 303, s.v[85]);
        }

        if s.b[858] {
            s.store_ad_value(202, {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[296]) {
                            A::mul_offset_rhs(s.ad_value(297), A::sub(s.ad_value(201), s.ad_value(296)), 1.0)
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            });
        }

        if s.b[858] {
            s.store_mul_offset_rhs(210, 302, 202, (-1.0));
            s.store_scalar(211, 0.0);
        }

        s.b[859] = (s.v[306] > 0.0);
        s.v[859] = if s.b[859] { 1.0 } else { 0.0 };

        if (s.b[858] && s.b[859]) {
            s.store_mul_ad_rhs(211, 277, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(277), s.ad_value(305), 1.0));
        }

        if (s.b[858] && (!s.b[859])) {
            s.store_scaled_mul(201, 277, 305, (-s.v[85]));
        }

        if (s.b[858] && (!s.b[859])) {
            s.store_ad_value(202, {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[298]) {
                            A::mul_offset_rhs(s.ad_value(299), A::sub(s.ad_value(201), s.ad_value(298)), 1.0)
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            });
        }

        if (s.b[858] && (!s.b[859])) {
            s.store_mul_scaled_ad_rhs(211, 304, -1.0, A::offset(s.ad_value(202), (-1.0)));
        }

        if s.b[858] {
            s.store_ad_value(274, A::add_scaled_inputs3(s.ad_value(209), 1.0, s.ad_value(210), 1.0, s.ad_value(211), 1.0));
            s.store_add(290, 210, 211);
            s.store_scalar(268, 0.0);
            s.store_scalar(270, 0.0);
            s.store_scalar(272, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scaled_mul(215, 265, 265, 4.0);
            s.store_div(216, 265, 266);
            s.store_ad_value(217, A::add_scaled_product(s.ad_value(277), 1.0, s.ad_value(265), s.ad_value(216), 1.0));
            s.store_add(218, 266, 217);
            s.store_sub(219, 266, 217);
            s.store_sqrt_square_add(220, 219, 215);
            s.store_ad_value(204, A::div_scaled_product(s.ad_value(277), s.ad_value(266), 2.0, A::add(s.ad_value(218), s.ad_value(220)), 1.0));
        }

        s.b[860] = (s.v[259] > 0.5);
        s.v[860] = if s.b[860] { 1.0 } else { 0.0 };

        s.b[861] = (s.v[122] == 0.5);
        s.v[861] = if s.b[861] { 1.0 } else { 0.0 };

        if ((s.b[858] && s.b[860]) && s.b[861]) {
            s.store_sqrt_sub_from_scalar_ad(203, 1.0, A::mul(s.ad_value(204), s.ad_value(119)));
        }

        if ((s.b[858] && s.b[860]) && (!s.b[861])) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(119))), s.v[122]);
        }

        if (s.b[858] && s.b[860]) {
            s.store_ad_value(269, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(203)), 1.0, s.ad_value(134), A::sub(s.ad_value(277), s.ad_value(204)), 1.0));
        }

        if (s.b[858] && (!s.b[860])) {
            s.store_scalar(269, 0.0);
        }

        s.b[862] = (s.v[260] > 0.5);
        s.v[862] = if s.b[862] { 1.0 } else { 0.0 };

        s.b[863] = (s.v[123] == 0.5);
        s.v[863] = if s.b[863] { 1.0 } else { 0.0 };

        if ((s.b[858] && s.b[862]) && s.b[863]) {
            s.store_sqrt_sub_from_scalar_ad(203, 1.0, A::mul(s.ad_value(204), s.ad_value(120)));
        }

        if ((s.b[858] && s.b[862]) && (!s.b[863])) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(120))), s.v[123]);
        }

        if (s.b[858] && s.b[862]) {
            s.store_ad_value(271, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(203)), 1.0, s.ad_value(135), A::sub(s.ad_value(277), s.ad_value(204)), 1.0));
        }

        if (s.b[858] && (!s.b[862])) {
            s.store_scalar(271, 0.0);
        }

        s.b[864] = (s.v[261] > 0.5);
        s.v[864] = if s.b[864] { 1.0 } else { 0.0 };

        s.b[865] = (s.v[124] == 0.5);
        s.v[865] = if s.b[865] { 1.0 } else { 0.0 };

        if ((s.b[858] && s.b[864]) && s.b[865]) {
            s.store_sqrt_sub_from_scalar_ad(203, 1.0, A::mul(s.ad_value(204), s.ad_value(121)));
        }

        if ((s.b[858] && s.b[864]) && (!s.b[865])) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(121))), s.v[124]);
        }

        if (s.b[858] && s.b[864]) {
            s.store_ad_value(273, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(203)), 1.0, s.ad_value(136), A::sub(s.ad_value(277), s.ad_value(204)), 1.0));
        }

        if (s.b[858] && (!s.b[864])) {
            s.store_scalar(273, 0.0);
        }

        s.b[866] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[866] = if s.b[866] { 1.0 } else { 0.0 };

        if ((!s.b[858]) && s.b[866]) {
            s.store_scaled_mul(215, 265, 265, 4.0);
            s.store_div(216, 265, 266);
            s.store_ad_value(217, A::add_scaled_product(s.ad_value(277), 1.0, s.ad_value(265), s.ad_value(216), 1.0));
            s.store_add(218, 266, 217);
            s.store_sub(219, 266, 217);
            s.store_sqrt_square_add(220, 219, 215);
            s.store_ad_value(221, A::div_scaled_product(s.ad_value(277), s.ad_value(266), 2.0, A::add(s.ad_value(218), s.ad_value(220)), 1.0));
        }

        s.b[867] = (s.v[277] < s.v[262]);
        s.v[867] = if s.b[867] { 1.0 } else { 0.0 };

        s.b[868] = ((((0.5 * (s.v[277] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[868] = if s.b[868] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[868]) {
            s.store_exp_scaled_input(223, 277, (s.v[85] * 0.5));
        }

        s.b[869] = ((0.5 * (s.v[277] * s.v[85])) < (-230.25850929940458));
        s.v[869] = if s.b[869] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[868])) && s.b[869]) {
            s.store_div_from_scalar_offset_ad(223, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[868])) && (!s.b[869])) {
            s.store_scaled_offset_ad(223, A::mul_offset_rhs(A::scale_offset(s.ad_value(277), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(277), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(277), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[870] = (s.v[62] < p.p85);
        s.v[870] = if s.b[870] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_offset_scaled_sub(360, 277, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[870])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[871] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[871] = if s.b[871] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[871]) {
            s.store_ad_value(370, A::exp_scaled_input(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[872] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[872] = if s.b[872] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && s.b[872]) {
            let assign29660_ad_e43817: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign29660_ad_e43817, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && (!s.b[872])) {
            let assign29670_ad_e43894: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(370, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign29670_ad_e43894, 1.0)), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[873] = (s.v[64] < p.p85);
        s.v[873] = if s.b[873] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_offset_scaled_sub(360, 277, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[873])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[874] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[874] = if s.b[874] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[874]) {
            s.store_ad_value(371, A::exp_scaled_input(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[875] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[875] = if s.b[875] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[874])) && s.b[875]) {
            let assign29980_ad_e44447: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, assign29980_ad_e44447, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[874])) && (!s.b[875])) {
            let assign29990_ad_e44524: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(371, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign29990_ad_e44524, 1.0)), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[876] = (s.v[63] < p.p85);
        s.v[876] = if s.b[876] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_offset_scaled_sub(360, 277, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[876])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[877] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[877] = if s.b[877] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[877]) {
            s.store_ad_value(372, A::exp_scaled_input(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[878] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[878] = if s.b[878] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[877])) && s.b[878]) {
            let assign30300_ad_e45077: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, assign30300_ad_e45077, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[877])) && (!s.b[878])) {
            let assign30310_ad_e45154: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(372, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign30310_ad_e45154, 1.0)), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_sqrt_ad(223, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(277), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[879] = (s.v[62] < p.p85);
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[879]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
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
            s.store_ad_value(281, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[881] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && s.b[881]) {
            let assign30670_ad_e45819: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign30670_ad_e45819, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[880])) && (!s.b[881])) {
            let assign30680_ad_e45897: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(281, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign30680_ad_e45897, 1.0)), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[882] = (s.v[64] < p.p85);
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[882])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[883] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[883] = if s.b[883] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[883]) {
            s.store_ad_value(282, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[884] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[884] = if s.b[884] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[883])) && s.b[884]) {
            let assign31050_ad_e46589: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, assign31050_ad_e46589, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[883])) && (!s.b[884])) {
            let assign31060_ad_e46667: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(282, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign31060_ad_e46667, 1.0)), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(371, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[885] = (s.v[63] < p.p85);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[885]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[885])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[886] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[886]) {
            s.store_ad_value(283, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[887] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[886])) && s.b[887]) {
            let assign31430_ad_e47359: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, assign31430_ad_e47359, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && (!s.b[867])) && (!s.b[886])) && (!s.b[887])) {
            let assign31440_ad_e47437: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(283, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign31440_ad_e47437, 1.0)), 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[867])) {
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(372, A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0, 283);
        }

        if ((!s.b[858]) && s.b[866]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(222, 1.0, 223);
        }

        s.b[888] = (s.v[277] > 0.0);
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && s.b[866]) && s.b[888]) {
            s.store_scaled_ln_ad(224, A::add(A::offset(s.ad_value(222), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(222), 1.0, A::offset(s.ad_value(222), 3.0)))), (s.v[84] * 2.0));
        }

        if (((!s.b[858]) && s.b[866]) && (!s.b[888])) {
            s.store_sub_ad_lhs(224, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(223), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(223), 1.0, A::scale_offset(s.ad_value(223), 3.0, 1.0))))), (s.v[84] * 2.0)), 277);
        }

        if ((!s.b[858]) && s.b[866]) {
            s.store_sub(225, 264, 224);
            s.store_ad_value(226, A::add_scaled_inputs3(s.ad_value(277), 0.5, s.ad_value(225), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(225)), A::sub(s.ad_value(277), s.ad_value(225))), ((4.0 * s.v[84]) * s.v[84]))), (-0.5)));
            s.store_ad_value(227, A::add_scaled_inputs3(s.ad_value(277), 0.5, s.ad_value(267), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(277), s.ad_value(267)), A::sub(s.ad_value(277), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0)), (-0.5)));
            s.store_scaled_sub_ad_rhs(228, 277, A::sqrt(A::offset(A::mul(s.ad_value(277), s.ad_value(277)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if ((!s.b[858]) && (!s.b[866])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(224, 0.0);
            s.store_scalar(221, 0.0);
            s.store_scalar(223, 0.0);
            s.store_scalar(226, 0.0);
            s.store_scalar(227, 0.0);
            s.store_scalar(228, 0.0);
        }

        s.b[889] = (s.v[256] == 0.0);
        s.v[889] = if s.b[889] { 1.0 } else { 0.0 };

        if ((!s.b[858]) && s.b[889]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[890] = (s.v[122] == 0.5);
        s.v[890] = if s.b[890] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[889])) && s.b[890]) {
            s.store_sqrt_sub_from_scalar_ad(229, 1.0, A::mul(s.ad_value(221), s.ad_value(119)));
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[890])) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(119))), s.v[122]);
        }

        if ((!s.b[858]) && (!s.b[889])) {
            s.store_ad_value(269, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(229)), 1.0, s.ad_value(134), A::sub(s.ad_value(277), s.ad_value(221)), 1.0));
            s.store_mul(230, 101, 370);
        }

        s.b[891] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[891] = if s.b[891] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[889])) && s.b[891]) {
            s.store_scalar(232, 0.0);
            s.store_scalar(235, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(237, 0.0);
            s.store_scalar(231, 0.0);
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[891])) {
            s.store_sub(232, 107, 226);
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.b[892] = (s.v[9] == 0.5);
        s.v[892] = if s.b[892] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && s.b[892]) {
            s.store_scalar(234, 0.0);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && (!s.b[892])) {
            s.store_scaled_add_ad_lhs(234, A::div_scaled_product(A::square(s.ad_value(233)), A::ln(s.ad_value(233)), 1.0, A::sub_from_scalar(1.0, s.ad_value(233)), 1.0), 233, (1.0 - (2.0 * s.v[9])));
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[891])) {
            s.store_add(235, 233, 234);
        }

        s.b[893] = (s.v[9] == 0.5);
        s.v[893] = if s.b[893] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && s.b[893]) {
            s.store_sqrt_scaled_input(229, 232, s.v[143]);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[891])) && (!s.b[893])) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[143]), s.v[9]);
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[891])) {
            s.store_scale(236, 229, s.v[137]);
            s.store_mul_ad_product_lhs(237, s.ad_value(98), A::offset(s.ad_value(223), (-1.0)), 236);
            s.store_scaled_mul(231, 237, 235, s.v[20]);
        }

        s.b[894] = (s.v[23] == 0.0);
        s.v[894] = if s.b[894] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[889])) && s.b[894]) {
            s.store_scalar(238, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {
            s.store_scaled_div(239, 236, 232, (s.v[122] * s.v[152]));
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[149]), 239);
            s.store_square(241, 240);
            s.store_sqrt_ad(242, A::div_scaled_product_offset_denominator(s.ad_value(241), s.ad_value(241), 1.0, A::square(s.ad_value(241)), 1.0, 1.0));
            s.store_sqrt_abs_ad(243, s.ad_value(242));
            s.store_mul(244, 242, 243);
        }

        s.b[895] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[895]) {
            s.store_div_from_scalar_offset_ad(245, 1.0, A::mul(s.ad_value(239), s.ad_value(244)), 1.0);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[895])) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {
            s.store_ad_value(246, A::div_scaled_product(s.ad_value(235), s.ad_value(245), 1.0, A::add(s.ad_value(235), s.ad_value(245)), 1.0));
            s.store_sqrt_scaled_ad(247, A::div(s.ad_value(239), s.ad_value(243)), 0.375);
            s.store_ad_value(248, A::add_scaled_product(s.ad_value(242), (-1.0), s.ad_value(240), s.ad_value(243), 2.0));
            s.store_ad_value(249, A::add_scaled_value_products(s.ad_value(242), (-s.v[149]), s.ad_value(240), s.ad_value(243), s.v[149], s.ad_value(239), s.ad_value(244), 0.5));
            s.store_mul_offset_lhs(250, 248, (-1.0), 247);
            s.store_square(212, 250);
        }

        s.b[896] = (s.v[250] > 0.0);
        s.v[896] = if s.b[896] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[896]) {
            s.store_div_from_scalar_offset_scaled_input(213, 1.0, 250, s.v[86], 1.0);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[896])) {
            s.store_div_from_scalar_sub_from_scalar_ad(213, 1.0, 1.0, A::scale(s.ad_value(250), s.v[86]));
        }

        s.b[897] = (((-s.v[212]) + s.v[249]) > (-230.25850929940458));
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[897]) {
            s.store_exp_sub(229, 249, 212);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[897])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {
            s.store_mul_ad_lhs(214, A::add_scaled_inputs_product(s.ad_value(213), 0.29214664, A::square(s.ad_value(213)), s.v[87], A::square(s.ad_value(213)), s.ad_value(213), s.v[88]), 229);
        }

        s.b[898] = (s.v[250] > 0.0);
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && s.b[898]) {
            s.copy_ad(251, 214);
        }

        s.b[899] = (s.v[249] > (-230.25850929940458));
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[898])) && s.b[899]) {
            s.store_exp(229, 249);
        }

        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[898])) && (!s.b[899])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(249), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(249), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[894])) && (!s.b[898])) {
            s.store_sub_scaled_inputs(251, 229, 2.0, 214, 1.0);
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {
            s.store_scaled_div(252, 251, 247, (s.v[149] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(238, 237, 252, s.v[23], 0.0, 246);
        }

        s.b[900] = (s.v[29] == 0.0);
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[889])) && s.b[900]) {
            s.store_scalar(253, 0.0);
        }

        s.b[901] = (s.v[9] == 0.5);
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && s.b[901]) {
            s.store_sqrt_scaled_ad(229, A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[143]);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && (!s.b[901])) {
            s.store_powf_ad(229, A::scale_offset(s.ad_value(227), (-s.v[143]), ((s.v[6]) * (s.v[143]))), s.v[9]);
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[900])) {
            s.store_scaled_div_ad_lhs(254, A::scale_offset(s.ad_value(227), (-s.v[140]), ((s.v[6]) * (s.v[140]))), 229, s.v[125]);
        }

        s.b[902] = (((((-s.v[155]) / s.v[254])) as f64).abs() < 230.25850929940458);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && s.b[902]) {
            s.store_exp_ad(229, A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(254), 1.0));
        }

        s.b[903] = (((-s.v[155]) / s.v[254]) < (-230.25850929940458));
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && (!s.b[902])) && s.b[903]) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(254), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(254), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[900])) && (!s.b[902])) && (!s.b[903])) {
            let assign32360_ad_e48793: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(254), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(254), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(254), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(229, assign32360_ad_e48793, 1e100);
        }

        if (((!s.b[858]) && (!s.b[889])) && (!s.b[900])) {
            s.store_mul_scaled_ad_lhs(253, A::mul3(s.ad_value(277), s.ad_value(254), s.ad_value(254)), 229, s.v[29]);
        }

        s.b[904] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[904] = if s.b[904] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[889])) && s.b[904]) {
            s.store_scalar(255, 1.0);
        }

        s.b[905] = (s.v[228] > ((-s.v[158]) * s.v[38]));
        s.v[905] = if s.b[905] { 1.0 } else { 0.0 };

        s.b[906] = (s.v[41] == 4.0);
        s.v[906] = if s.b[906] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && s.b[905]) && s.b[906]) {
            s.store_mul_ad(229, A::mul3(A::abs(A::mul(s.ad_value(228), s.ad_value(162))), A::abs(A::mul(s.ad_value(228), s.ad_value(162))), A::abs(A::mul(s.ad_value(228), s.ad_value(162)))), A::abs(A::mul(s.ad_value(228), s.ad_value(162))));
        }

        if (((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && s.b[905]) && (!s.b[906])) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(162))), s.v[41]);
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && s.b[905]) {
            s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));
        }

        if ((((!s.b[858]) && (!s.b[889])) && (!s.b[904])) && (!s.b[905])) {
            s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);
        }

        if ((!s.b[858]) && (!s.b[889])) {
            s.store_mul_ad_lhs(268, A::add_scaled_inputs4(s.ad_value(230), 1.0, s.ad_value(231), 1.0, s.ad_value(238), 1.0, s.ad_value(253), 1.0), 255);
            s.store_mul_ad_lhs(291, A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(238), 1.0, s.ad_value(253), 1.0), 255);
        }

        s.b[907] = (s.v[257] == 0.0);
        s.v[907] = if s.b[907] { 1.0 } else { 0.0 };

        if ((!s.b[858]) && s.b[907]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[908] = (s.v[123] == 0.5);
        s.v[908] = if s.b[908] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[907])) && s.b[908]) {
            s.store_sqrt_sub_from_scalar_ad(229, 1.0, A::mul(s.ad_value(221), s.ad_value(120)));
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[908])) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(120))), s.v[123]);
        }

        if ((!s.b[858]) && (!s.b[907])) {
            s.store_ad_value(271, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(229)), 1.0, s.ad_value(135), A::sub(s.ad_value(277), s.ad_value(221)), 1.0));
            s.store_mul(230, 102, 371);
        }

        s.b[909] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[909] = if s.b[909] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[907])) && s.b[909]) {
            s.store_scalar(232, 0.0);
            s.store_scalar(235, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(237, 0.0);
            s.store_scalar(231, 0.0);
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[909])) {
            s.store_sub(232, 108, 226);
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.b[910] = (s.v[10] == 0.5);
        s.v[910] = if s.b[910] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && s.b[910]) {
            s.store_scalar(234, 0.0);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && (!s.b[910])) {
            s.store_scaled_add_ad_lhs(234, A::div_scaled_product(A::square(s.ad_value(233)), A::ln(s.ad_value(233)), 1.0, A::sub_from_scalar(1.0, s.ad_value(233)), 1.0), 233, (1.0 - (2.0 * s.v[10])));
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[909])) {
            s.store_add(235, 233, 234);
        }

        s.b[911] = (s.v[10] == 0.5);
        s.v[911] = if s.b[911] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && s.b[911]) {
            s.store_sqrt_scaled_input(229, 232, s.v[144]);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[909])) && (!s.b[911])) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[144]), s.v[10]);
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[909])) {
            s.store_scale(236, 229, s.v[138]);
            s.store_mul_ad_product_lhs(237, s.ad_value(99), A::offset(s.ad_value(223), (-1.0)), 236);
            s.store_scaled_mul(231, 237, 235, s.v[21]);
        }

        s.b[912] = (s.v[24] == 0.0);
        s.v[912] = if s.b[912] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[907])) && s.b[912]) {
            s.store_scalar(238, 0.0);
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {
            s.store_scaled_div(239, 236, 232, (s.v[123] * s.v[153]));
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[150]), 239);
            s.store_square(241, 240);
            s.store_sqrt_ad(242, A::div_scaled_product_offset_denominator(s.ad_value(241), s.ad_value(241), 1.0, A::square(s.ad_value(241)), 1.0, 1.0));
            s.store_sqrt_abs_ad(243, s.ad_value(242));
            s.store_mul(244, 242, 243);
        }

        s.b[913] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[913] = if s.b[913] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[913]) {
            s.store_div_from_scalar_offset_ad(245, 1.0, A::mul(s.ad_value(239), s.ad_value(244)), 1.0);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[913])) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {
            s.store_ad_value(246, A::div_scaled_product(s.ad_value(235), s.ad_value(245), 1.0, A::add(s.ad_value(235), s.ad_value(245)), 1.0));
            s.store_sqrt_scaled_ad(247, A::div(s.ad_value(239), s.ad_value(243)), 0.375);
            s.store_ad_value(248, A::add_scaled_product(s.ad_value(242), (-1.0), s.ad_value(240), s.ad_value(243), 2.0));
            s.store_ad_value(249, A::add_scaled_value_products(s.ad_value(242), (-s.v[150]), s.ad_value(240), s.ad_value(243), s.v[150], s.ad_value(239), s.ad_value(244), 0.5));
            s.store_mul_offset_lhs(250, 248, (-1.0), 247);
            s.store_square(212, 250);
        }

        s.b[914] = (s.v[250] > 0.0);
        s.v[914] = if s.b[914] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[914]) {
            s.store_div_from_scalar_offset_scaled_input(213, 1.0, 250, s.v[86], 1.0);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[914])) {
            s.store_div_from_scalar_sub_from_scalar_ad(213, 1.0, 1.0, A::scale(s.ad_value(250), s.v[86]));
        }

        s.b[915] = (((-s.v[212]) + s.v[249]) > (-230.25850929940458));
        s.v[915] = if s.b[915] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[915]) {
            s.store_exp_sub(229, 249, 212);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[915])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {
            s.store_mul_ad_lhs(214, A::add_scaled_inputs_product(s.ad_value(213), 0.29214664, A::square(s.ad_value(213)), s.v[87], A::square(s.ad_value(213)), s.ad_value(213), s.v[88]), 229);
        }

        s.b[916] = (s.v[250] > 0.0);
        s.v[916] = if s.b[916] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && s.b[916]) {
            s.copy_ad(251, 214);
        }

        s.b[917] = (s.v[249] > (-230.25850929940458));
        s.v[917] = if s.b[917] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[916])) && s.b[917]) {
            s.store_exp(229, 249);
        }

        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[916])) && (!s.b[917])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(249), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(249), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[912])) && (!s.b[916])) {
            s.store_sub_scaled_inputs(251, 229, 2.0, 214, 1.0);
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[912])) {
            s.store_scaled_div(252, 251, 247, (s.v[150] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(238, 237, 252, s.v[24], 0.0, 246);
        }

        s.b[918] = (s.v[30] == 0.0);
        s.v[918] = if s.b[918] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[907])) && s.b[918]) {
            s.store_scalar(253, 0.0);
        }

        s.b[919] = (s.v[10] == 0.5);
        s.v[919] = if s.b[919] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && s.b[919]) {
            s.store_sqrt_scaled_ad(229, A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[144]);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && (!s.b[919])) {
            s.store_powf_ad(229, A::scale_offset(s.ad_value(227), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[918])) {
            s.store_scaled_div_ad_lhs(254, A::scale_offset(s.ad_value(227), (-s.v[141]), ((s.v[7]) * (s.v[141]))), 229, s.v[126]);
        }

        s.b[920] = (((((-s.v[156]) / s.v[254])) as f64).abs() < 230.25850929940458);
        s.v[920] = if s.b[920] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && s.b[920]) {
            s.store_exp_ad(229, A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(254), 1.0));
        }

        s.b[921] = (((-s.v[156]) / s.v[254]) < (-230.25850929940458));
        s.v[921] = if s.b[921] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && (!s.b[920])) && s.b[921]) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(254), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(254), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[918])) && (!s.b[920])) && (!s.b[921])) {
            let assign33170_ad_e50012: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(254), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(254), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(254), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(229, assign33170_ad_e50012, 1e100);
        }

        if (((!s.b[858]) && (!s.b[907])) && (!s.b[918])) {
            s.store_mul_scaled_ad_lhs(253, A::mul3(s.ad_value(277), s.ad_value(254), s.ad_value(254)), 229, s.v[30]);
        }

        s.b[922] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[922] = if s.b[922] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[907])) && s.b[922]) {
            s.store_scalar(255, 1.0);
        }

        s.b[923] = (s.v[228] > ((-s.v[158]) * s.v[39]));
        s.v[923] = if s.b[923] { 1.0 } else { 0.0 };

        s.b[924] = (s.v[42] == 4.0);
        s.v[924] = if s.b[924] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) && s.b[924]) {
            s.store_mul_ad(229, A::mul3(A::abs(A::mul(s.ad_value(228), s.ad_value(163))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))), A::abs(A::mul(s.ad_value(228), s.ad_value(163)))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))));
        }

        if (((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) && (!s.b[924])) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(163))), s.v[42]);
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && s.b[923]) {
            s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));
        }

        if ((((!s.b[858]) && (!s.b[907])) && (!s.b[922])) && (!s.b[923])) {
            s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);
        }

        if ((!s.b[858]) && (!s.b[907])) {
            s.store_mul_ad_lhs(270, A::add_scaled_inputs4(s.ad_value(230), 1.0, s.ad_value(231), 1.0, s.ad_value(238), 1.0, s.ad_value(253), 1.0), 255);
            s.store_mul_ad_lhs(292, A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(238), 1.0, s.ad_value(253), 1.0), 255);
        }

        s.b[925] = (s.v[258] == 0.0);
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if ((!s.b[858]) && s.b[925]) {
            s.store_scalar(272, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[858]) && s.b[925]) {
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[926] = (s.v[124] == 0.5);
        s.v[926] = if s.b[926] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[925])) && s.b[926]) {
            s.store_sqrt_sub_from_scalar_ad(229, 1.0, A::mul(s.ad_value(221), s.ad_value(121)));
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[926])) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(121))), s.v[124]);
        }

        if ((!s.b[858]) && (!s.b[925])) {
            s.store_ad_value(273, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(133), 1.0, s.ad_value(229)), 1.0, s.ad_value(136), A::sub(s.ad_value(277), s.ad_value(221)), 1.0));
            s.store_mul(230, 103, 372);
        }

        s.b[927] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[925])) && s.b[927]) {
            s.store_scalar(232, 0.0);
            s.store_scalar(235, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(237, 0.0);
            s.store_scalar(231, 0.0);
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[927])) {
            s.store_sub(232, 109, 226);
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.b[928] = (s.v[11] == 0.5);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && s.b[928]) {
            s.store_scalar(234, 0.0);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && (!s.b[928])) {
            s.store_scaled_add_ad_lhs(234, A::div_scaled_product(A::square(s.ad_value(233)), A::ln(s.ad_value(233)), 1.0, A::sub_from_scalar(1.0, s.ad_value(233)), 1.0), 233, (1.0 - (2.0 * s.v[11])));
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[927])) {
            s.store_add(235, 233, 234);
        }

        s.b[929] = (s.v[11] == 0.5);
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && s.b[929]) {
            s.store_sqrt_scaled_input(229, 232, s.v[145]);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[927])) && (!s.b[929])) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[145]), s.v[11]);
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[927])) {
            s.store_scale(236, 229, s.v[139]);
            s.store_mul_ad_product_lhs(237, s.ad_value(100), A::offset(s.ad_value(223), (-1.0)), 236);
            s.store_scaled_mul(231, 237, 235, s.v[22]);
        }

        s.b[930] = (s.v[25] == 0.0);
        s.v[930] = if s.b[930] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[925])) && s.b[930]) {
            s.store_scalar(238, 0.0);
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {
            s.store_scaled_div(239, 236, 232, (s.v[124] * s.v[154]));
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[151]), 239);
            s.store_square(241, 240);
            s.store_sqrt_ad(242, A::div_scaled_product_offset_denominator(s.ad_value(241), s.ad_value(241), 1.0, A::square(s.ad_value(241)), 1.0, 1.0));
            s.store_sqrt_abs_ad(243, s.ad_value(242));
            s.store_mul(244, 242, 243);
        }

        s.b[931] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[931] = if s.b[931] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[931]) {
            s.store_div_from_scalar_offset_ad(245, 1.0, A::mul(s.ad_value(239), s.ad_value(244)), 1.0);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[931])) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {
            s.store_ad_value(246, A::div_scaled_product(s.ad_value(235), s.ad_value(245), 1.0, A::add(s.ad_value(235), s.ad_value(245)), 1.0));
            s.store_sqrt_scaled_ad(247, A::div(s.ad_value(239), s.ad_value(243)), 0.375);
            s.store_ad_value(248, A::add_scaled_product(s.ad_value(242), (-1.0), s.ad_value(240), s.ad_value(243), 2.0));
            s.store_ad_value(249, A::add_scaled_value_products(s.ad_value(242), (-s.v[151]), s.ad_value(240), s.ad_value(243), s.v[151], s.ad_value(239), s.ad_value(244), 0.5));
            s.store_mul_offset_lhs(250, 248, (-1.0), 247);
            s.store_square(212, 250);
        }

        s.b[932] = (s.v[250] > 0.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[932]) {
            s.store_div_from_scalar_offset_scaled_input(213, 1.0, 250, s.v[86], 1.0);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[932])) {
            s.store_div_from_scalar_sub_from_scalar_ad(213, 1.0, 1.0, A::scale(s.ad_value(250), s.v[86]));
        }

        s.b[933] = (((-s.v[212]) + s.v[249]) > (-230.25850929940458));
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[933]) {
            s.store_exp_sub(229, 249, 212);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[933])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {
            s.store_mul_ad_lhs(214, A::add_scaled_inputs_product(s.ad_value(213), 0.29214664, A::square(s.ad_value(213)), s.v[87], A::square(s.ad_value(213)), s.ad_value(213), s.v[88]), 229);
        }

        s.b[934] = (s.v[250] > 0.0);
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && s.b[934]) {
            s.copy_ad(251, 214);
        }

        s.b[935] = (s.v[249] > (-230.25850929940458));
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[934])) && s.b[935]) {
            s.store_exp(229, 249);
        }

        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[934])) && (!s.b[935])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(249), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(249), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[930])) && (!s.b[934])) {
            s.store_sub_scaled_inputs(251, 229, 2.0, 214, 1.0);
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[930])) {
            s.store_scaled_div(252, 251, 247, (s.v[151] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(238, 237, 252, s.v[25], 0.0, 246);
        }

        s.b[936] = (s.v[31] == 0.0);
        s.v[936] = if s.b[936] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[925])) && s.b[936]) {
            s.store_scalar(253, 0.0);
        }

        s.b[937] = (s.v[11] == 0.5);
        s.v[937] = if s.b[937] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && s.b[937]) {
            s.store_sqrt_scaled_ad(229, A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[145]);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && (!s.b[937])) {
            s.store_powf_ad(229, A::scale_offset(s.ad_value(227), (-s.v[145]), ((s.v[8]) * (s.v[145]))), s.v[11]);
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[936])) {
            s.store_scaled_div_ad_lhs(254, A::scale_offset(s.ad_value(227), (-s.v[142]), ((s.v[8]) * (s.v[142]))), 229, s.v[127]);
        }

        s.b[938] = (((((-s.v[157]) / s.v[254])) as f64).abs() < 230.25850929940458);
        s.v[938] = if s.b[938] { 1.0 } else { 0.0 };

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && s.b[938]) {
            s.store_exp_ad(229, A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(254), 1.0));
        }

        s.b[939] = (((-s.v[157]) / s.v[254]) < (-230.25850929940458));
        s.v[939] = if s.b[939] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && (!s.b[938])) && s.b[939]) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(254), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(254), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[936])) && (!s.b[938])) && (!s.b[939])) {
            let assign33980_ad_e51231: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(254), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(254), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(157), -1.0, s.ad_value(254), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(229, assign33980_ad_e51231, 1e100);
        }

        if (((!s.b[858]) && (!s.b[925])) && (!s.b[936])) {
            s.store_mul_scaled_ad_lhs(253, A::mul3(s.ad_value(277), s.ad_value(254), s.ad_value(254)), 229, s.v[31]);
        }

        s.b[940] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[940] = if s.b[940] { 1.0 } else { 0.0 };

        if (((!s.b[858]) && (!s.b[925])) && s.b[940]) {
            s.store_scalar(255, 1.0);
        }

        s.b[941] = (s.v[228] > ((-s.v[158]) * s.v[40]));
        s.v[941] = if s.b[941] { 1.0 } else { 0.0 };

        s.b[942] = (s.v[43] == 4.0);
        s.v[942] = if s.b[942] { 1.0 } else { 0.0 };

        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) && s.b[942]) {
            s.store_mul_ad(229, A::mul3(A::abs(A::mul(s.ad_value(228), s.ad_value(164))), A::abs(A::mul(s.ad_value(228), s.ad_value(164))), A::abs(A::mul(s.ad_value(228), s.ad_value(164)))), A::abs(A::mul(s.ad_value(228), s.ad_value(164))));
        }

        if (((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) && (!s.b[942])) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(164))), s.v[43]);
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && s.b[941]) {
            s.store_div_from_scalar_sub_from_scalar_ad(255, 1.0, 1.0, s.ad_value(229));
        }

        if ((((!s.b[858]) && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            s.store_offset_mul_ad(255, A::add_scaled_inputs(s.ad_value(228), 1.0, s.ad_value(40), s.v[158]), s.ad_value(167), s.v[161]);
        }

        if ((!s.b[858]) && (!s.b[925])) {
            s.store_mul_ad_lhs(272, A::add_scaled_inputs4(s.ad_value(230), 1.0, s.ad_value(231), 1.0, s.ad_value(238), 1.0, s.ad_value(253), 1.0), 255);
            s.store_mul_ad_lhs(293, A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(238), 1.0, s.ad_value(253), 1.0), 255);
        }

        if (!s.b[858]) {
            s.store_ad_value(274, A::add_scaled_inputs3(s.ad_value(268), s.v[256], s.ad_value(270), s.v[257], s.ad_value(272), s.v[258]));
            s.store_ad_value(290, A::add_scaled_inputs3(s.ad_value(291), s.v[256], s.ad_value(292), s.v[257], s.ad_value(293), s.v[258]));
        }

        s.store_ad_value(275, A::add_scaled_inputs3(s.ad_value(269), s.v[256], s.ad_value(271), s.v[257], s.ad_value(273), s.v[258]));

        s.store_voltage(284, ctx, nodes, Some(2), Some(1));

        s.b[945] = (p.p84 > 0.0);
        s.v[945] = if s.b[945] { 1.0 } else { 0.0 };

        s.b[946] = (s.v[313] < p.p85);
        s.v[946] = if s.b[946] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[946]) {
            s.store_offset_scaled_sub(349, 277, 348, p.p86, s.v[313]);
            s.store_sub_from_scalar_ad(350, s.v[313], A::scale(s.ad_value(348), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(349), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(351, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 351, (((-s.v[313])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(352, 314, 315, 0.5, s.v[313]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[313])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if (s.b[945] && s.b[946]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (s.b[945] && s.b[946]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[313]);
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
            s.store_ad_value(354, A::exp_scaled_input(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), 1.0, A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352)), (-1.0), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), 1.0), s.v[85]));
        }

        s.b[949] = ((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && s.b[949]) {
            let assign34480_ad_e51900: A = A::scale_offset(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352)), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-0.3333333333333333), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)));
            let assign34480_ad_e51903: A = A::offset(A::mul_sub_from_scalar_lhs_scaled_output((-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352)), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), assign34480_ad_e51900, 0.5), 1.0);
            let assign34480_ad_e51906: A = A::div_from_scalar(1e-100, A::offset(A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352)), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), assign34480_ad_e51903), 1.0));
            s.store_ad_value(354, assign34480_ad_e51906);
        }

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && (!s.b[949])) {
            let assign34490_ad_e51996: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352)), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352)), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            let assign34490_ad_e51999: A = A::offset(A::mul_offset_lhs(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352)), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign34490_ad_e51996, 1.0)), 1.0);
            s.store_scale_ad(354, assign34490_ad_e51999, 1e100);
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
            s.store_mul_scaled_ad_rhs(357, 353, p.p90, A::exp(A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(347)), A::sub(s.ad_value(277), s.ad_value(347)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91))));
        }

        if s.b[945] {
            s.store_ad_value(357, {
                if (s.v[357] > p.p79) {
                    A::constant(p.p79)
                } else {
                    s.ad_value(357)
                }
            });
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
        }

    }

    pub(super) fn stamp_transient_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[945] && s.b[951]) {
            s.store_scaled_sub(338, 336, 334, 1.0 / (p.p92));
            s.store_scale(340, 336, 1.0 / ((1e-23 / s.v[333])));
        }

        if (s.b[945] && (!s.b[951])) {
            s.copy_ad(334, 331);
            s.copy_ad(340, 334);
        }

        s.b[952] = ((p.p91 == 0.0) || (s.v[277] < s.v[348]));
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[952]) {
            s.store_scale(358, 354, p.p90);
        }

        if (s.b[945] && (!s.b[952])) {
            s.store_mul_scaled_ad_rhs(358, 354, p.p90, A::exp(A::mul3_scaled_output(A::sub(s.ad_value(277), s.ad_value(348)), A::sub(s.ad_value(277), s.ad_value(348)), A::exp_scaled_input(A::ln_scaled_input(s.ad_value(78), 1.0 / (s.v[79])), p.p98), (-p.p91))));
        }

        if s.b[945] {
            s.store_ad_value(358, {
                if (s.v[358] > p.p79) {
                    A::constant(p.p79)
                } else {
                    s.ad_value(358)
                }
            });
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
            s.store_scale(341, 337, 1.0 / ((1e-23 / s.v[333])));
        }

        if (s.b[945] && (!s.b[953])) {
            s.copy_ad(335, 332);
            s.copy_ad(341, 335);
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
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if s.b[945] {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(326, p.p94, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
        }

        s.b[955] = (p.p95 > 0.0);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        if (s.b[945] && s.b[955]) {
            s.store_mul_div_from_scalar_rhs(342, 326, 1.0, 343);
            s.store_voltage(344, ctx, nodes, Some(5), None);
            s.store_scaled_sub(345, 344, 342, 1.0 / (p.p95));
            s.store_div_ad_rhs(346, 344, A::div_from_scalar(1.0, s.ad_value(343)));
        }

        if (s.b[945] && (!s.b[955])) {
            s.copy_ad(342, 326);
            s.copy_ad(346, 342);
        }

        if s.b[945] {
            s.store_scalar(327, ((-((s.v[307] * s.v[256]) * 1.6021918e-19)) * p.p94));
            s.store_mul_ad_product_rhs(328, 323, s.ad_value(340), A::sub(A::exp(A::div_from_scalar((-p.p94), s.ad_value(323))), A::exp(A::div_scaled_inputs(s.ad_value(346), -1.0, s.ad_value(323), 1.0))));
            s.store_mul_ad_product_rhs(329, 323, s.ad_value(341), A::offset(A::exp(A::div_scaled_inputs(A::sub_from_scalar(p.p94, s.ad_value(346)), -1.0, s.ad_value(323), 1.0)), (-1.0)));
            s.store_neg_ad(330, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_add(275, 275, 330);
            s.store_scalar(55, 0.0);
        }

        if (!s.b[945]) {
            s.store_mul_sub_rhs(330, 55, 274, 290);
        }

        s.store_ad_value(276, A::add_scaled_inputs4(s.ad_value(274), (2.0 * 1.6021918e-19), s.ad_value(290), ((-1.0) * (2.0 * 1.6021918e-19)), s.ad_value(289), (2.0 * (2.0 * 1.6021918e-19)), A::abs(s.ad_value(290)), (2.0 * 1.6021918e-19)));

        s.store_scaled_powf_ad(286, A::abs(s.ad_value(274)), s.v[54], s.v[53]);

        s.b[956] = ((s.v[171] > 0.0) && (s.v[171] >= p.p4));
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        if s.b[956] {
            s.store_div_from_scalar(287, ((4.0 * 1.3806505e-23) * s.v[79]), 171);
        }

        if (!s.b[956]) {
            s.store_scalar(287, 0.0);
        }

        s.b[957] = ((s.v[171] > 0.0) && (s.v[171] >= p.p4));
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

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

        s.v[3] = (if (p.p8 > 1e-12) { p.p8 } else { 1e-12 });

        s.v[4] = (if (p.p9 > 1e-18) { p.p9 } else { 1e-18 });

        s.v[5] = (if (p.p10 > 1e-18) { p.p10 } else { 1e-18 });

        s.v[6] = (if (p.p11 > 0.05) { p.p11 } else { 0.05 });

        s.v[7] = (if (p.p12 > 0.05) { p.p12 } else { 0.05 });

        s.v[8] = (if (p.p13 > 0.05) { p.p13 } else { 0.05 });

        s.v[9] = (if (p.p14 > 0.05) { (if (p.p14 < 0.95) { p.p14 } else { 0.95 }) } else { 0.05 });

        s.v[10] = (if (p.p15 > 0.05) { (if (p.p15 < 0.95) { p.p15 } else { 0.95 }) } else { 0.05 });

        s.v[11] = (if (p.p16 > 0.05) { (if (p.p16 < 0.95) { p.p16 } else { 0.95 }) } else { 0.05 });

        s.v[12] = p.p17;

        s.v[13] = p.p18;

        s.v[14] = p.p19;

        s.v[15] = (if (p.p20 > 0.0) { p.p20 } else { 0.0 });

        s.v[16] = (if (p.p21 > 0.0) { p.p21 } else { 0.0 });

        s.v[17] = (if (p.p22 > 0.0) { p.p22 } else { 0.0 });

        s.v[20] = (if (p.p23 > 0.0) { p.p23 } else { 0.0 });

        s.v[21] = (if (p.p24 > 0.0) { p.p24 } else { 0.0 });

        s.v[22] = (if (p.p25 > 0.0) { p.p25 } else { 0.0 });

        s.v[18] = (if (p.p26 > 1e-9) { p.p26 } else { 1e-9 });

        s.v[19] = (if (p.p27 > 1e-9) { p.p27 } else { 1e-9 });

        s.v[23] = (if (p.p28 > 0.0) { p.p28 } else { 0.0 });

        s.v[24] = (if (p.p29 > 0.0) { p.p29 } else { 0.0 });

        s.v[25] = (if (p.p30 > 0.0) { p.p30 } else { 0.0 });

        s.v[26] = (if (p.p31 > 0.01) { p.p31 } else { 0.01 });

        s.v[27] = (if (p.p32 > 0.01) { p.p32 } else { 0.01 });

        s.v[28] = (if (p.p33 > 0.01) { p.p33 } else { 0.01 });

        s.v[29] = (if (p.p34 > 0.0) { p.p34 } else { 0.0 });

        s.v[30] = (if (p.p35 > 0.0) { p.p35 } else { 0.0 });

        s.v[31] = (if (p.p36 > 0.0) { p.p36 } else { 0.0 });

        s.v[32] = p.p37;

        s.v[33] = p.p38;

        s.v[34] = p.p39;

        s.v[35] = p.p40;

        s.v[36] = p.p41;

        s.v[37] = p.p42;

        s.v[38] = (if (p.p43 > 0.1) { p.p43 } else { 0.1 });

        s.v[39] = (if (p.p44 > 0.1) { p.p44 } else { 0.1 });

        s.v[40] = (if (p.p45 > 0.1) { p.p45 } else { 0.1 });

        s.v[41] = (if (p.p46 > 0.1) { p.p46 } else { 0.1 });

        s.v[42] = (if (p.p47 > 0.1) { p.p47 } else { 0.1 });

        s.v[43] = (if (p.p48 > 0.1) { p.p48 } else { 0.1 });

        s.v[44] = p.p7;

        s.v[55] = (if (p.p56 > 0.0) { p.p56 } else { 0.0 });

        s.v[56] = p.p57;

        s.v[57] = p.p58;

        s.v[58] = p.p59;

        s.v[59] = p.p60;

        s.v[60] = p.p61;

        s.v[61] = p.p62;

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

        s.v[47] = (if (p.p83 > 0.0) { p.p83 } else { 0.0 });

        s.store_offset(78, 1, 273.15);

        s.v[79] = ((ctx_temp + p.p102)).max((273.15 + (-250.0)));

        s.store_div_from_scalar(80, s.v[79], 78);

        s.v[81] = (1.3806505e-23 / 1.6021918e-19);

        s.store_scale(82, 78, s.v[81]);

        s.store_div_from_scalar(83, 1.0, 82);

        s.v[84] = (s.v[81] * s.v[79]);

        s.v[85] = (1.0 / s.v[84]);

        s.store_ad_value(89, A::div_scaled_inputs(A::mul_scaled_lhs(s.ad_value(78), 0.000702, s.ad_value(78)), -1.0, A::offset(s.ad_value(78), 1108.0), 1.0));

        s.store_offset(92, 89, s.v[12]);

        s.store_offset(93, 89, s.v[13]);

        s.store_offset(94, 89, s.v[14]);

        s.v[90] = ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79]));

        s.v[95] = (s.v[12] + s.v[90]);

        s.v[96] = (s.v[13] + s.v[90]);

        s.v[97] = (s.v[14] + s.v[90]);

        s.store_mul_ad(98, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), 0.5));

        s.store_mul_ad(99, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), 0.5));

        s.store_mul_ad(100, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), 0.5));

        s.store_mul_ad(176, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[62])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62]))));

        s.store_mul_ad(177, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[64])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64]))));

        s.store_mul_ad(178, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[63])), A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63]))));

        s.store_scaled_mul(101, 176, 176, s.v[15]);

        s.store_scaled_mul(102, 177, 177, s.v[16]);

        s.store_scaled_mul(103, 178, 178, s.v[17]);

        s.store_ad_value(104, A::sub_scaled_inputs(s.ad_value(80), s.v[6], A::ln(s.ad_value(98)), (2.0 * s.v[84])));

        s.store_ad_value(105, A::sub_scaled_inputs(s.ad_value(80), s.v[7], A::ln(s.ad_value(99)), (2.0 * s.v[84])));

        s.store_ad_value(106, A::sub_scaled_inputs(s.ad_value(80), s.v[8], A::ln(s.ad_value(100)), (2.0 * s.v[84])));

        s.store_ad_value(107, A::add_scaled_inputs(s.ad_value(104), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(104), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]));

        s.store_ad_value(108, A::add_scaled_inputs(s.ad_value(105), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(105), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]));

        s.store_ad_value(109, A::add_scaled_inputs(s.ad_value(106), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(106), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]));

        s.store_div_from_scalar(119, 1.0, 107);

        s.store_div_from_scalar(120, 1.0, 108);

        s.store_div_from_scalar(121, 1.0, 109);

        s.v[122] = (1.0 - s.v[9]);

        s.v[123] = (1.0 - s.v[10]);

        s.v[124] = (1.0 - s.v[11]);

        s.v[125] = (1.0 / s.v[122]);

        s.v[126] = (1.0 / s.v[123]);

        s.v[127] = (1.0 / s.v[124]);

        s.store_scaled_powf_ad(128, A::scale(s.ad_value(119), s.v[6]), s.v[9], s.v[3]);

        s.store_scaled_powf_ad(129, A::scale(s.ad_value(120), s.v[7]), s.v[10], s.v[4]);

        s.store_scaled_powf_ad(130, A::scale(s.ad_value(121), s.v[8]), s.v[11], s.v[5]);

        s.store_scaled_mul(131, 128, 107, s.v[125]);

        s.store_scaled_mul(132, 129, 108, s.v[126]);

        s.store_scaled_mul(133, 130, 109, s.v[127]);

        s.store_scale(134, 128, 2.0);

        s.store_scale(135, 129, 2.0);

        s.store_scale(136, 130, 2.0);

        s.v[137] = (s.v[0] / s.v[3]);

        s.v[138] = ((s.v[18] * s.v[0]) / s.v[4]);

        s.v[139] = ((s.v[19] * s.v[0]) / s.v[5]);

        s.v[140] = (1.0 / s.v[137]);

        s.v[141] = (1.0 / s.v[138]);

        s.v[142] = (1.0 / s.v[139]);

        s.v[143] = (1.0 / s.v[6]);

        s.v[144] = (1.0 / s.v[7]);

        s.v[145] = (1.0 / s.v[8]);

        s.v[86] = (1.772453850905516 * 0.29214664);

        s.v[87] = (((((-5.0) * 0.29214664) + 6.0) - ((s.v[86]) as f64).powf((-2.0))) / 3.0);

        s.v[88] = ((1.0 - 0.29214664) - s.v[87]);

        s.v[146] = ((0.5 * s.v[95])).max(s.v[84]);

        s.v[147] = ((0.5 * s.v[96])).max(s.v[84]);

        s.v[148] = ((0.5 * s.v[97])).max(s.v[84]);

        s.v[149] = (s.v[146] * s.v[85]);

        s.v[150] = (s.v[147] * s.v[85]);

        s.v[151] = (s.v[148] * s.v[85]);

        s.v[152] = (((((((32.0 * s.v[26]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[146] * s.v[146]) * s.v[146]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[153] = (((((((32.0 * s.v[27]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[147] * s.v[147]) * s.v[147]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[154] = (((((((32.0 * s.v[28]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[148] * s.v[148]) * s.v[148]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.store_offset_scaled(155, 78, (((-s.v[35])) * (s.v[32])), ((((((s.v[79]) * (s.v[35]))) + (1.0))) * (s.v[32])));

        s.store_offset_scaled(156, 78, (((-s.v[36])) * (s.v[33])), ((((((s.v[79]) * (s.v[36]))) + (1.0))) * (s.v[33])));

        s.store_offset_scaled(157, 78, (((-s.v[37])) * (s.v[34])), ((((((s.v[79]) * (s.v[37]))) + (1.0))) * (s.v[34])));

        if (!(s.v[155] > 0.0)) {
            s.store_scalar(155, 0.0);
        }

        if (!(s.v[156] > 0.0)) {
            s.store_scalar(156, 0.0);
        }

        if (!(s.v[157] > 0.0)) {
            s.store_scalar(157, 0.0);
        }

        s.v[158] = ((s.v[44] - 1.0) / s.v[44]);

        s.v[159] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[41])));

        s.v[160] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[42])));

        s.v[161] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[43])));

        s.store_scaled_offset_ad(38, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[57], s.v[56], 1.0), 1.0, s.v[38]);

        s.store_scaled_offset_ad(39, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[59], s.v[58], 1.0), 1.0, s.v[39]);

        s.store_scaled_offset_ad(40, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[61], s.v[60], 1.0), 1.0, s.v[40]);

        s.b[390] = (s.v[38] <= 0.1);
        s.v[390] = if s.b[390] { 1.0 } else { 0.0 };

        if s.b[390] {
            s.store_scalar(38, 0.1);
            s.store_scalar(162, 10.0);
        }

        if (!s.b[390]) {
            s.store_div_from_scalar(162, 1.0, 38);
        }

        s.b[391] = (s.v[39] <= 0.1);
        s.v[391] = if s.b[391] { 1.0 } else { 0.0 };

        if s.b[391] {
            s.store_scalar(39, 0.1);
            s.store_scalar(163, 10.0);
        }

        if (!s.b[391]) {
            s.store_div_from_scalar(163, 1.0, 39);
        }

        s.b[392] = (s.v[40] <= 0.1);
        s.v[392] = if s.b[392] { 1.0 } else { 0.0 };

        if s.b[392] {
            s.store_scalar(40, 0.1);
            s.store_scalar(164, 10.0);
        }

        if (!s.b[392]) {
            s.store_div_from_scalar(164, 1.0, 40);
        }

        s.v[179] = (1.0 - (0.01 * s.v[77]));

        s.store_scale(165, 162, ((-((s.v[159] * s.v[159]) * ((s.v[158]) as f64).powf((s.v[41] - 1.0)))) * s.v[41]));

        s.store_scale(166, 163, ((-((s.v[160] * s.v[160]) * ((s.v[158]) as f64).powf((s.v[42] - 1.0)))) * s.v[42]));

        s.store_scale(167, 164, ((-((s.v[161] * s.v[161]) * ((s.v[158]) as f64).powf((s.v[43] - 1.0)))) * s.v[43]));

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

        s.store_ad_value(322, A::div_scaled_product(s.ad_value(320), s.ad_value(321), 2.0, A::add(s.ad_value(320), s.ad_value(321)), 1.0));

        s.store_powf(317, 80, p.p97);

        s.store_scale(324, 317, p.p93);

        s.store_sqrt_mul(323, 324, 322);

        s.store_scaled_ln_ad(347, A::div_from_scalar(s.v[307], s.ad_value(319)), (s.v[313] / s.v[85]));

        s.store_scaled_add_ad(348, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323)), (s.v[313] / s.v[85]));

        s.v[256] = (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]);

        s.v[257] = (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[258] = (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[263] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

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

        s.b[396] = ((((s.v[262] * s.v[85])) as f64).abs() < 230.25850929940458);
        s.v[396] = if s.b[396] { 1.0 } else { 0.0 };

        if s.b[396] {
            s.store_exp_scaled_input(263, 262, s.v[85]);
        }

        s.b[397] = ((s.v[262] * s.v[85]) < (-230.25850929940458));
        s.v[397] = if s.b[397] { 1.0 } else { 0.0 };

        if ((!s.b[396]) && s.b[397]) {
            s.store_div_from_scalar_offset_ad(263, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(262), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(262), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((!s.b[396]) && (!s.b[397])) {
            s.store_scaled_offset_ad(263, A::mul_offset_rhs(A::scale_offset(s.ad_value(262), s.v[85], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(262), s.v[85], (-230.25850929940458)), A::scale_offset(s.ad_value(262), ((s.v[85]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        s.copy_ad(110, 107);

        s.copy_ad(111, 108);

        s.copy_ad(112, 109);

        s.v[113] = s.v[9];

        s.v[114] = s.v[10];

        s.v[115] = s.v[11];

        s.v[116] = s.v[6];

        s.v[117] = s.v[7];

        s.v[118] = s.v[8];

        s.b[398] = (s.v[256] == 0.0);
        s.v[398] = if s.b[398] { 1.0 } else { 0.0 };

        if s.b[398] {
            s.store_add(110, 108, 109);
            s.store_scalar(113, (0.9 * (s.v[10]).min(s.v[11])));
            s.store_scalar(116, (s.v[7] + s.v[8]));
        }

        s.b[399] = (s.v[257] == 0.0);
        s.v[399] = if s.b[399] { 1.0 } else { 0.0 };

        if s.b[399] {
            s.store_add(111, 107, 109);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[399] {
            s.store_scalar(114, (0.9 * (s.v[9]).min(s.v[11])));
            s.store_scalar(117, (s.v[6] + s.v[8]));
        }

        s.b[400] = (s.v[258] == 0.0);
        s.v[400] = if s.b[400] { 1.0 } else { 0.0 };

        if s.b[400] {
            s.store_add(112, 107, 108);
            s.store_scalar(115, (0.9 * (s.v[9]).min(s.v[10])));
            s.store_scalar(118, (s.v[6] + s.v[7]));
        }

        s.store_min3(264, 110, 111, 112);

        s.store_scale(265, 264, 0.1);

        s.store_max3(91, 113, 114, 115);

        s.store_mul_sub_from_scalar_ad_rhs(266, 264, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(91))));

        s.store_offset_min_ad(267, A::min(s.ad_value(116), s.ad_value(117)), s.ad_value(118), (-0.05));

        s.store_ad_value(289, A::add_scaled_inputs3(s.ad_value(101), s.v[256], s.ad_value(102), s.v[257], s.ad_value(103), s.v[258]));

        s.v[300] = 0.0;

        s.v[301] = 1.0;

        s.v[303] = 1.0;

        s.v[302] = 0.0;

        s.v[305] = 1.0;

        s.v[304] = 0.0;

        s.v[306] = 0.0;

        s.v[294] = 0.0;

        s.v[295] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[185] = 0.0;

        s.v[186] = 0.0;

        s.v[187] = 0.0;

        s.v[188] = 0.0;

        s.v[189] = 0.0;

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[200] = 0.0;

        s.v[208] = 0.0;

        s.v[259] = 1.0;

        s.v[260] = 1.0;

        s.v[261] = 1.0;

        s.v[195] = 0.0;

        s.v[203] = 0.0;

        s.v[204] = 0.0;

        s.v[370] = 0.0;

        s.v[372] = 0.0;

        s.v[371] = 0.0;

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

        s.b[415] = (s.v[256] > 0.0);
        s.v[415] = if s.b[415] { 1.0 } else { 0.0 };

        if ((s.b[413] && (!s.b[414])) && s.b[415]) {
            s.store_scalar(301, s.v[62]);
        }

        s.b[416] = (s.v[258] > 0.0);
        s.v[416] = if s.b[416] { 1.0 } else { 0.0 };

        if ((s.b[413] && (!s.b[414])) && s.b[416]) {
            s.store_scalar(301, s.v[63]);
        }

        s.b[417] = (s.v[257] > 0.0);
        s.v[417] = if s.b[417] { 1.0 } else { 0.0 };

        if ((s.b[413] && (!s.b[414])) && s.b[417]) {
            s.store_scalar(301, s.v[64]);
        }

        s.b[418] = (s.v[45] == 1.0);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

        if s.b[418] {
            s.store_scalar(419, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(421, 0.0);
            s.store_scalar(422, 0.0);
            s.store_scalar(423, 0.0);
            s.store_scalar(424, 0.0);
            s.store_scalar(425, 0.0);
            s.store_scalar(426, 0.0);
            s.store_scalar(427, 0.0);
            s.store_scalar(277, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(429, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(432, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
            s.store_scalar(436, 0.0);
            s.store_scalar(437, 0.0);
            s.store_scalar(438, 0.0);
            s.store_scalar(439, 0.0);
            s.store_scalar(440, 0.0);
            s.store_scalar(441, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(448, 0.0);
            s.store_scalar(449, 0.0);
            s.store_scalar(450, 0.0);
            s.store_scalar(451, 0.0);
            s.store_scalar(452, 0.0);
            s.store_scalar(453, 0.0);
            s.store_scalar(454, 0.0);
            s.store_scalar(455, 0.0);
            s.store_scalar(456, 0.0);
            s.store_scalar(457, 0.0);
            s.store_scalar(458, 0.0);
            s.store_scalar(459, 0.0);
            s.store_scalar(460, 0.0);
            s.store_scalar(461, 0.0);
            s.store_scalar(462, 0.0);
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

        if (s.b[418] && s.b[463]) {
            s.store_scaled_mul(422, 265, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_ad_value(424, A::add_scaled_product(s.ad_value(190), 1.0, s.ad_value(265), s.ad_value(423), 1.0));
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_ad_value(428, A::div_scaled_product(s.ad_value(190), s.ad_value(266), 2.0, A::add(s.ad_value(425), s.ad_value(427)), 1.0));
        }

        s.b[464] = (s.v[190] < s.v[262]);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        s.b[465] = ((((0.5 * (s.v[190] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[465]) {
            s.store_exp_scaled_input(430, 190, (s.v[85] * 0.5));
        }

        s.b[466] = ((0.5 * (s.v[190] * s.v[85])) < (-230.25850929940458));
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && s.b[466]) {
            s.store_div_from_scalar_offset_ad(430, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && (!s.b[466])) {
            s.store_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(190), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(190), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(190), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[467] = (s.v[62] < p.p85);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_offset_scaled_sub(360, 190, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[467])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[468] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[468]) {
            s.store_ad_value(370, A::exp_scaled_input(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[469] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && s.b[469]) {
            let assign4290_ad_e2954: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, assign4290_ad_e2954, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && (!s.b[469])) {
            let assign4300_ad_e3030: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(370, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign4300_ad_e3030, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[470] = (s.v[64] < p.p85);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_offset_scaled_sub(360, 190, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[470])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[471] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[471]) {
            s.store_ad_value(371, A::exp_scaled_input(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[472] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && s.b[472]) {
            let assign4610_ad_e3555: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, assign4610_ad_e3555, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && (!s.b[472])) {
            let assign4620_ad_e3631: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(371, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign4620_ad_e3631, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[473] = (s.v[63] < p.p85);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_offset_scaled_sub(360, 190, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[473])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[474] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[474]) {
            s.store_ad_value(372, A::exp_scaled_input(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[475] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && s.b[475]) {
            let assign4930_ad_e4156: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, assign4930_ad_e4156, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && (!s.b[475])) {
            let assign4940_ad_e4232: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(372, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign4940_ad_e4232, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(190), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[476] = (s.v[62] < p.p85);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
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
            s.store_ad_value(281, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[478] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && s.b[478]) {
            let assign5300_ad_e4864: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, assign5300_ad_e4864, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && (!s.b[478])) {
            let assign5310_ad_e4941: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(281, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign5310_ad_e4941, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[479] = (s.v[64] < p.p85);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[479])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[480] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[480]) {
            s.store_ad_value(282, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[481] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && s.b[481]) {
            let assign5680_ad_e5599: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, assign5680_ad_e5599, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && (!s.b[481])) {
            let assign5690_ad_e5676: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(282, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign5690_ad_e5676, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(371, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[482] = (s.v[63] < p.p85);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::add_scaled_inputs(s.ad_value(314), 0.5, s.ad_value(315), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[482])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[483] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[483]) {
            s.store_ad_value(283, A::exp_scaled_input(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]));
        }

        s.b[484] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && s.b[484]) {
            let assign6060_ad_e6334: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, assign6060_ad_e6334, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && (!s.b[484])) {
            let assign6070_ad_e6411: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::scale_offset(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5);
            s.store_scaled_offset_ad(283, A::mul_offset_lhs(A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign6070_ad_e6411, 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div(A::add_scaled_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0)), A::square(s.ad_value(359))), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(372, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 283);
        }

        if (s.b[418] && s.b[463]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[485] = (s.v[190] > 0.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[463]) && s.b[485]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[463]) && (!s.b[485])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 190);
        }

        if (s.b[418] && s.b[463]) {
            s.store_sub(432, 264, 431);
            s.store_ad_value(433, A::add_scaled_inputs3(s.ad_value(190), 0.5, s.ad_value(432), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(432)), A::sub(s.ad_value(190), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), (-0.5)));
            s.store_ad_value(434, A::add_scaled_inputs3(s.ad_value(190), 0.5, s.ad_value(267), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(190), s.ad_value(267)), A::sub(s.ad_value(190), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0)), (-0.5)));
            s.store_scaled_sub_ad_rhs(435, 190, A::sqrt(A::offset(A::mul(s.ad_value(190), s.ad_value(190)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[463])) {
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

        s.b[486] = (s.v[256] == 0.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[486]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[487] = (s.v[122] == 0.5);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[487]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[487])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[486])) {
            s.store_ad_value(269, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, s.ad_value(134), A::sub(s.ad_value(190), s.ad_value(428)), 1.0));
            s.store_mul(437, 101, 370);
        }

        s.b[488] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

    }
}
