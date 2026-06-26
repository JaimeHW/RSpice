#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
            s.store_ad_value(461, A::div_scaled_offset_numerator(s.ad_value(434), ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), s.ad_value(436), 1.0));
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

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[812])) && (!s.b[823])) && (!s.b[824])) {
            s.store_powf_ad(436, A::scale_offset(s.ad_value(434), (-s.v[144]), ((s.v[7]) * (s.v[144]))), s.v[10]);
        }

        if ((s.b[418] && (!s.b[812])) && (!s.b[823])) {
            s.store_ad_value(461, A::div_scaled_offset_numerator(s.ad_value(434), ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), s.ad_value(436), 1.0));
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
            s.store_ad_value(461, A::div_scaled_offset_numerator(s.ad_value(434), ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), s.ad_value(436), 1.0));
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
            s.store_ad_value(302, A::div_scaled_value_offset_denominator(s.ad_value(188), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(193), s.v[85], s.ad_value(303))), (-1.0), 1.0));
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
            s.store_ad_value(304, A::div_scaled_value_offset_denominator(s.ad_value(187), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(192), (-s.v[85]), s.ad_value(305))), (-1.0), 1.0));
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

    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[857] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[857] = if s.b[857] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[857]) {
            s.store_ln_ad(294, A::div_scalar_offset_denominator((0.5 * s.v[2]), s.ad_value(300), 1e-21, 1.0));
            s.store_ln_ad(296, A::div_scalar_offset_denominator((0.5 * s.v[2]), s.ad_value(302), 1e-21, 1.0));
            s.store_ln_ad(298, A::div_scalar_offset_denominator((0.5 * s.v[2]), A::abs(s.ad_value(304)), 1e-21, 1.0));
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
                    A::div_scalar_offset_denominator(1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0, 1.0)
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
                    A::div_scalar_offset_denominator(1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0, 1.0)
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
                    A::div_scalar_offset_denominator(1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0, 1.0)
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

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
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
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((!s.b[858]) && s.b[866]) && (!s.b[867])) && s.b[882]) {
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
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
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
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
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
        }

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[858]) && (!s.b[889])) && (!s.b[894])) {
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
            s.store_ad_value(254, A::div_scaled_offset_numerator(s.ad_value(227), ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), s.ad_value(229), 1.0));
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
            s.store_ad_value(254, A::div_scaled_offset_numerator(s.ad_value(227), ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), s.ad_value(229), 1.0));
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
        }

    }

    pub(super) fn stamp_reactive_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[858]) && (!s.b[925])) && (!s.b[927])) {
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
            s.store_ad_value(254, A::div_scaled_offset_numerator(s.ad_value(227), ((-s.v[142]) * s.v[127]), (((s.v[8]) * (s.v[142])) * s.v[127]), s.ad_value(229), 1.0));
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
            s.store_ad_value(354, A::exp_scaled_input(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), 1.0, A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), (-1.0), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), 1.0), s.v[85]));
        }

        s.b[949] = ((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && s.b[949]) {
            let assign34480_ad_e51899: A = A::scale_offset(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-0.3333333333333333), (((-230.25850929940458)) * (0.3333333333333333)));
            let assign34480_ad_e51902: A = A::mul_sub_from_scalar_lhs_scaled_output((-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(assign34480_ad_e51899, 1.0), 0.5);
            let assign34480_ad_e51904: A = A::mul_sub_from_scalar_lhs((-230.25850929940458), A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), A::offset(assign34480_ad_e51902, 1.0));
            s.store_div_from_scalar_offset_ad(354, 1e-100, assign34480_ad_e51904, 1.0);
        }

        if (((s.b[945] && s.b[947]) && (!s.b[948])) && (!s.b[949])) {
            let assign34490_ad_e51994: A = A::scale_offset(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)));
            let assign34490_ad_e51996: A = A::mul_offset_lhs_scaled_output(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), assign34490_ad_e51994, 0.5);
            let assign34490_ad_e51998: A = A::mul_offset_lhs(A::add_scaled_inputs3(A::div(s.ad_value(277), s.ad_value(352)), s.v[85], A::div_scaled_inputs2(s.ad_value(348), 1.0, s.ad_value(347), (-1.0), s.ad_value(352), 1.0), ((-1.0) * s.v[85]), A::div_scaled_product(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), A::offset(assign34490_ad_e51996, 1.0));
            s.store_scaled_offset_ad(354, assign34490_ad_e51998, 1.0, 1e100);
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
        }

    }

    pub(super) fn stamp_reactive_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[945] && s.b[953]) {
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

        s.b[958] = ((p.p84 > 0.0) && (p.p92 > 0.0));
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        s.b[959] = ((p.p84 > 0.0) && (p.p95 > 0.0));
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let eq0_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (eq0_value),
        );
        let eq1_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (eq1_value),
        );
        let eq2_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(2),
            Some(1),
            multiplicity * (eq2_value),
        );
        let eq3_value: f64 = s.v[274];
        let eq3_node_derivatives: [f64; 6] = [s.dn[274][0], s.dn[274][1], s.dn[274][2], s.dn[274][3], s.dn[274][4], s.dn[274][5]];
        let eq3_branch_derivatives: [f64; 4] = [s.db[274][0], s.db[274][1], s.db[274][2], s.db[274][3]];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_e122: f64 = 0.0;
        let eq4_e124: f64 = (eq4_e122 * (nv0 - nv2));
        let eq4_e124_d_n2: f64 = (-eq4_e122);
        let eq4_value: f64 = eq4_e124;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq4_value),
            0,
            multiplicity * (eq4_e122),
            2,
            multiplicity * (eq4_e124_d_n2),
        );
        let (eq5_e130, eq5_e130_d_n0, eq5_e130_d_n1, eq5_e130_d_n2, eq5_e130_d_n3, eq5_e130_d_n4, eq5_e130_d_n5, eq5_e130_d_b0, eq5_e130_d_b1, eq5_e130_d_b2, eq5_e130_d_b3,) = {
    if s.b[957] {
        let eq5_e128: f64 = (s.v[284] / s.v[171]);
        let eq5_e128_d_n0: f64 = (((s.dn[284][0] * s.v[171]) - (s.v[284] * s.dn[171][0])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n1: f64 = (((s.dn[284][1] * s.v[171]) - (s.v[284] * s.dn[171][1])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n2: f64 = (((s.dn[284][2] * s.v[171]) - (s.v[284] * s.dn[171][2])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n3: f64 = (((s.dn[284][3] * s.v[171]) - (s.v[284] * s.dn[171][3])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n4: f64 = (((s.dn[284][4] * s.v[171]) - (s.v[284] * s.dn[171][4])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n5: f64 = (((s.dn[284][5] * s.v[171]) - (s.v[284] * s.dn[171][5])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_b0: f64 = (((s.db[284][0] * s.v[171]) - (s.v[284] * s.db[171][0])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_b1: f64 = (((s.db[284][1] * s.v[171]) - (s.v[284] * s.db[171][1])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_b2: f64 = (((s.db[284][2] * s.v[171]) - (s.v[284] * s.db[171][2])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_b3: f64 = (((s.db[284][3] * s.v[171]) - (s.v[284] * s.db[171][3])) / (s.v[171] * s.v[171]));
        (eq5_e128, eq5_e128_d_n0, eq5_e128_d_n1, eq5_e128_d_n2, eq5_e128_d_n3, eq5_e128_d_n4, eq5_e128_d_n5, eq5_e128_d_b0, eq5_e128_d_b1, eq5_e128_d_b2, eq5_e128_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e130;
        let eq5_node_derivatives: [f64; 6] = [eq5_e130_d_n0, eq5_e130_d_n1, eq5_e130_d_n2, eq5_e130_d_n3, eq5_e130_d_n4, eq5_e130_d_n5];
        let eq5_branch_derivatives: [f64; 4] = [eq5_e130_d_b0, eq5_e130_d_b1, eq5_e130_d_b2, eq5_e130_d_b3];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(1),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e135,) = {
    if (!s.b[957]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e135;
        stamper.stamp_potential_const_local(
            0,
            eq6_value,
        );
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3,) = {
    if s.b[958] {
        let eq7_e140: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[336]);
        let eq7_e140_d_n0: f64 = (s.dn[336][0] * ddt_scale);
        let eq7_e140_d_n1: f64 = (s.dn[336][1] * ddt_scale);
        let eq7_e140_d_n2: f64 = (s.dn[336][2] * ddt_scale);
        let eq7_e140_d_n3: f64 = (s.dn[336][3] * ddt_scale);
        let eq7_e140_d_n4: f64 = (s.dn[336][4] * ddt_scale);
        let eq7_e140_d_n5: f64 = (s.dn[336][5] * ddt_scale);
        let eq7_e140_d_b0: f64 = (s.db[336][0] * ddt_scale);
        let eq7_e140_d_b1: f64 = (s.db[336][1] * ddt_scale);
        let eq7_e140_d_b2: f64 = (s.db[336][2] * ddt_scale);
        let eq7_e140_d_b3: f64 = (s.db[336][3] * ddt_scale);
        let eq7_e141: f64 = (s.v[338] + eq7_e140);
        let eq7_e141_d_n0: f64 = (s.dn[338][0] + eq7_e140_d_n0);
        let eq7_e141_d_n1: f64 = (s.dn[338][1] + eq7_e140_d_n1);
        let eq7_e141_d_n2: f64 = (s.dn[338][2] + eq7_e140_d_n2);
        let eq7_e141_d_n3: f64 = (s.dn[338][3] + eq7_e140_d_n3);
        let eq7_e141_d_n4: f64 = (s.dn[338][4] + eq7_e140_d_n4);
        let eq7_e141_d_n5: f64 = (s.dn[338][5] + eq7_e140_d_n5);
        let eq7_e141_d_b0: f64 = (s.db[338][0] + eq7_e140_d_b0);
        let eq7_e141_d_b1: f64 = (s.db[338][1] + eq7_e140_d_b1);
        let eq7_e141_d_b2: f64 = (s.db[338][2] + eq7_e140_d_b2);
        let eq7_e141_d_b3: f64 = (s.db[338][3] + eq7_e140_d_b3);
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);
        let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);
        let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);
        let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e144;
        let eq7_node_derivatives: [f64; 6] = [eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5];
        let eq7_branch_derivatives: [f64; 4] = [eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3,) = {
    if s.b[958] {
        let eq8_e149: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[337]);
        let eq8_e149_d_n0: f64 = (s.dn[337][0] * ddt_scale);
        let eq8_e149_d_n1: f64 = (s.dn[337][1] * ddt_scale);
        let eq8_e149_d_n2: f64 = (s.dn[337][2] * ddt_scale);
        let eq8_e149_d_n3: f64 = (s.dn[337][3] * ddt_scale);
        let eq8_e149_d_n4: f64 = (s.dn[337][4] * ddt_scale);
        let eq8_e149_d_n5: f64 = (s.dn[337][5] * ddt_scale);
        let eq8_e149_d_b0: f64 = (s.db[337][0] * ddt_scale);
        let eq8_e149_d_b1: f64 = (s.db[337][1] * ddt_scale);
        let eq8_e149_d_b2: f64 = (s.db[337][2] * ddt_scale);
        let eq8_e149_d_b3: f64 = (s.db[337][3] * ddt_scale);
        let eq8_e150: f64 = (s.v[339] + eq8_e149);
        let eq8_e150_d_n0: f64 = (s.dn[339][0] + eq8_e149_d_n0);
        let eq8_e150_d_n1: f64 = (s.dn[339][1] + eq8_e149_d_n1);
        let eq8_e150_d_n2: f64 = (s.dn[339][2] + eq8_e149_d_n2);
        let eq8_e150_d_n3: f64 = (s.dn[339][3] + eq8_e149_d_n3);
        let eq8_e150_d_n4: f64 = (s.dn[339][4] + eq8_e149_d_n4);
        let eq8_e150_d_n5: f64 = (s.dn[339][5] + eq8_e149_d_n5);
        let eq8_e150_d_b0: f64 = (s.db[339][0] + eq8_e149_d_b0);
        let eq8_e150_d_b1: f64 = (s.db[339][1] + eq8_e149_d_b1);
        let eq8_e150_d_b2: f64 = (s.db[339][2] + eq8_e149_d_b2);
        let eq8_e150_d_b3: f64 = (s.db[339][3] + eq8_e149_d_b3);
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);
        let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);
        let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);
        let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e153;
        let eq8_node_derivatives: [f64; 6] = [eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5];
        let eq8_branch_derivatives: [f64; 4] = [eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e158,) = {
    if (!s.b[958]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e158;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
        let (eq10_e163,) = {
    if (!s.b[958]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e163;
        stamper.stamp_potential_const_local(
            2,
            eq10_value,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3,) = {
    if s.b[959] {
        let eq11_e168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[344]);
        let eq11_e168_d_n0: f64 = (s.dn[344][0] * ddt_scale);
        let eq11_e168_d_n1: f64 = (s.dn[344][1] * ddt_scale);
        let eq11_e168_d_n2: f64 = (s.dn[344][2] * ddt_scale);
        let eq11_e168_d_n3: f64 = (s.dn[344][3] * ddt_scale);
        let eq11_e168_d_n4: f64 = (s.dn[344][4] * ddt_scale);
        let eq11_e168_d_n5: f64 = (s.dn[344][5] * ddt_scale);
        let eq11_e168_d_b0: f64 = (s.db[344][0] * ddt_scale);
        let eq11_e168_d_b1: f64 = (s.db[344][1] * ddt_scale);
        let eq11_e168_d_b2: f64 = (s.db[344][2] * ddt_scale);
        let eq11_e168_d_b3: f64 = (s.db[344][3] * ddt_scale);
        let eq11_e169: f64 = (s.v[345] + eq11_e168);
        let eq11_e169_d_n0: f64 = (s.dn[345][0] + eq11_e168_d_n0);
        let eq11_e169_d_n1: f64 = (s.dn[345][1] + eq11_e168_d_n1);
        let eq11_e169_d_n2: f64 = (s.dn[345][2] + eq11_e168_d_n2);
        let eq11_e169_d_n3: f64 = (s.dn[345][3] + eq11_e168_d_n3);
        let eq11_e169_d_n4: f64 = (s.dn[345][4] + eq11_e168_d_n4);
        let eq11_e169_d_n5: f64 = (s.dn[345][5] + eq11_e168_d_n5);
        let eq11_e169_d_b0: f64 = (s.db[345][0] + eq11_e168_d_b0);
        let eq11_e169_d_b1: f64 = (s.db[345][1] + eq11_e168_d_b1);
        let eq11_e169_d_b2: f64 = (s.db[345][2] + eq11_e168_d_b2);
        let eq11_e169_d_b3: f64 = (s.db[345][3] + eq11_e168_d_b3);
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);
        let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);
        let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);
        let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e172;
        let eq11_node_derivatives: [f64; 6] = [eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5];
        let eq11_branch_derivatives: [f64; 4] = [eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e177,) = {
    if (!s.b[959]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e177;
        stamper.stamp_potential_const_local(
            3,
            eq12_value,
        );
        let eq13_e179: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[275]);
        let eq13_e179_d_n0: f64 = (s.dn[275][0] * ddt_scale);
        let eq13_e179_d_n1: f64 = (s.dn[275][1] * ddt_scale);
        let eq13_e179_d_n2: f64 = (s.dn[275][2] * ddt_scale);
        let eq13_e179_d_n3: f64 = (s.dn[275][3] * ddt_scale);
        let eq13_e179_d_n4: f64 = (s.dn[275][4] * ddt_scale);
        let eq13_e179_d_n5: f64 = (s.dn[275][5] * ddt_scale);
        let eq13_e179_d_b0: f64 = (s.db[275][0] * ddt_scale);
        let eq13_e179_d_b1: f64 = (s.db[275][1] * ddt_scale);
        let eq13_e179_d_b2: f64 = (s.db[275][2] * ddt_scale);
        let eq13_e179_d_b3: f64 = (s.db[275][3] * ddt_scale);
        let eq13_value: f64 = eq13_e179;
        let eq13_node_derivatives: [f64; 6] = [eq13_e179_d_n0, eq13_e179_d_n1, eq13_e179_d_n2, eq13_e179_d_n3, eq13_e179_d_n4, eq13_e179_d_n5];
        let eq13_branch_derivatives: [f64; 4] = [eq13_e179_d_b0, eq13_e179_d_b1, eq13_e179_d_b2, eq13_e179_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e183: f64 = (s.v[274] - s.v[290]);
        let eq14_e183_d_n0: f64 = (s.dn[274][0] - s.dn[290][0]);
        let eq14_e183_d_n1: f64 = (s.dn[274][1] - s.dn[290][1]);
        let eq14_e183_d_n2: f64 = (s.dn[274][2] - s.dn[290][2]);
        let eq14_e183_d_n3: f64 = (s.dn[274][3] - s.dn[290][3]);
        let eq14_e183_d_n4: f64 = (s.dn[274][4] - s.dn[290][4]);
        let eq14_e183_d_n5: f64 = (s.dn[274][5] - s.dn[290][5]);
        let eq14_e183_d_b0: f64 = (s.db[274][0] - s.db[290][0]);
        let eq14_e183_d_b1: f64 = (s.db[274][1] - s.db[290][1]);
        let eq14_e183_d_b2: f64 = (s.db[274][2] - s.db[290][2]);
        let eq14_e183_d_b3: f64 = (s.db[274][3] - s.db[290][3]);
        let eq14_e184: f64 = (s.v[55] * eq14_e183);
        let eq14_e184_d_n0: f64 = ((s.dn[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_n0));
        let eq14_e184_d_n1: f64 = ((s.dn[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_n1));
        let eq14_e184_d_n2: f64 = ((s.dn[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_n2));
        let eq14_e184_d_n3: f64 = ((s.dn[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_n3));
        let eq14_e184_d_n4: f64 = ((s.dn[55][4] * eq14_e183) + (s.v[55] * eq14_e183_d_n4));
        let eq14_e184_d_n5: f64 = ((s.dn[55][5] * eq14_e183) + (s.v[55] * eq14_e183_d_n5));
        let eq14_e184_d_b0: f64 = ((s.db[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_b0));
        let eq14_e184_d_b1: f64 = ((s.db[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_b1));
        let eq14_e184_d_b2: f64 = ((s.db[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_b2));
        let eq14_e184_d_b3: f64 = ((s.db[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_b3));
        let eq14_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq14_e184);
        let eq14_e185_d_n0: f64 = (eq14_e184_d_n0 * ddt_scale);
        let eq14_e185_d_n1: f64 = (eq14_e184_d_n1 * ddt_scale);
        let eq14_e185_d_n2: f64 = (eq14_e184_d_n2 * ddt_scale);
        let eq14_e185_d_n3: f64 = (eq14_e184_d_n3 * ddt_scale);
        let eq14_e185_d_n4: f64 = (eq14_e184_d_n4 * ddt_scale);
        let eq14_e185_d_n5: f64 = (eq14_e184_d_n5 * ddt_scale);
        let eq14_e185_d_b0: f64 = (eq14_e184_d_b0 * ddt_scale);
        let eq14_e185_d_b1: f64 = (eq14_e184_d_b1 * ddt_scale);
        let eq14_e185_d_b2: f64 = (eq14_e184_d_b2 * ddt_scale);
        let eq14_e185_d_b3: f64 = (eq14_e184_d_b3 * ddt_scale);
        let eq14_value: f64 = eq14_e185;
        let eq14_node_derivatives: [f64; 6] = [eq14_e185_d_n0, eq14_e185_d_n1, eq14_e185_d_n2, eq14_e185_d_n3, eq14_e185_d_n4, eq14_e185_d_n5];
        let eq14_branch_derivatives: [f64; 4] = [eq14_e185_d_b0, eq14_e185_d_b1, eq14_e185_d_b2, eq14_e185_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3, eq7_e144_q, eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5, eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3,) = {
    if s.b[958] {
        let eq7_e140_q: f64 = s.v[336];
        let eq7_e141: f64 = (s.v[338] + s.v[336]);
        let eq7_e141_d_n0: f64 = (s.dn[338][0] + s.dn[336][0]);
        let eq7_e141_d_n1: f64 = (s.dn[338][1] + s.dn[336][1]);
        let eq7_e141_d_n2: f64 = (s.dn[338][2] + s.dn[336][2]);
        let eq7_e141_d_n3: f64 = (s.dn[338][3] + s.dn[336][3]);
        let eq7_e141_d_n4: f64 = (s.dn[338][4] + s.dn[336][4]);
        let eq7_e141_d_n5: f64 = (s.dn[338][5] + s.dn[336][5]);
        let eq7_e141_d_b0: f64 = (s.db[338][0] + s.db[336][0]);
        let eq7_e141_d_b1: f64 = (s.db[338][1] + s.db[336][1]);
        let eq7_e141_d_b2: f64 = (s.db[338][2] + s.db[336][2]);
        let eq7_e141_d_b3: f64 = (s.db[338][3] + s.db[336][3]);
        let eq7_e141_q: f64 = eq7_e140_q;
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);
        let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);
        let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);
        let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);
        let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);
        let eq7_e142_q_d_n0: f64 = (1e-12 * s.dn[336][0]);
        let eq7_e142_q_d_n1: f64 = (1e-12 * s.dn[336][1]);
        let eq7_e142_q_d_n2: f64 = (1e-12 * s.dn[336][2]);
        let eq7_e142_q_d_n3: f64 = (1e-12 * s.dn[336][3]);
        let eq7_e142_q_d_n4: f64 = (1e-12 * s.dn[336][4]);
        let eq7_e142_q_d_n5: f64 = (1e-12 * s.dn[336][5]);
        let eq7_e142_q_d_b0: f64 = (1e-12 * s.db[336][0]);
        let eq7_e142_q_d_b1: f64 = (1e-12 * s.db[336][1]);
        let eq7_e142_q_d_b2: f64 = (1e-12 * s.db[336][2]);
        let eq7_e142_q_d_b3: f64 = (1e-12 * s.db[336][3]);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3, eq7_e142_q, eq7_e142_q_d_n0, eq7_e142_q_d_n1, eq7_e142_q_d_n2, eq7_e142_q_d_n3, eq7_e142_q_d_n4, eq7_e142_q_d_n5, eq7_e142_q_d_b0, eq7_e142_q_d_b1, eq7_e142_q_d_b2, eq7_e142_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 6] = [eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5];
        let eq7_reactive_branch_derivatives: [f64; 4] = [eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3, eq8_e153_q, eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5, eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3,) = {
    if s.b[958] {
        let eq8_e149_q: f64 = s.v[337];
        let eq8_e150: f64 = (s.v[339] + s.v[337]);
        let eq8_e150_d_n0: f64 = (s.dn[339][0] + s.dn[337][0]);
        let eq8_e150_d_n1: f64 = (s.dn[339][1] + s.dn[337][1]);
        let eq8_e150_d_n2: f64 = (s.dn[339][2] + s.dn[337][2]);
        let eq8_e150_d_n3: f64 = (s.dn[339][3] + s.dn[337][3]);
        let eq8_e150_d_n4: f64 = (s.dn[339][4] + s.dn[337][4]);
        let eq8_e150_d_n5: f64 = (s.dn[339][5] + s.dn[337][5]);
        let eq8_e150_d_b0: f64 = (s.db[339][0] + s.db[337][0]);
        let eq8_e150_d_b1: f64 = (s.db[339][1] + s.db[337][1]);
        let eq8_e150_d_b2: f64 = (s.db[339][2] + s.db[337][2]);
        let eq8_e150_d_b3: f64 = (s.db[339][3] + s.db[337][3]);
        let eq8_e150_q: f64 = eq8_e149_q;
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);
        let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);
        let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);
        let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);
        let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);
        let eq8_e151_q_d_n0: f64 = (1e-12 * s.dn[337][0]);
        let eq8_e151_q_d_n1: f64 = (1e-12 * s.dn[337][1]);
        let eq8_e151_q_d_n2: f64 = (1e-12 * s.dn[337][2]);
        let eq8_e151_q_d_n3: f64 = (1e-12 * s.dn[337][3]);
        let eq8_e151_q_d_n4: f64 = (1e-12 * s.dn[337][4]);
        let eq8_e151_q_d_n5: f64 = (1e-12 * s.dn[337][5]);
        let eq8_e151_q_d_b0: f64 = (1e-12 * s.db[337][0]);
        let eq8_e151_q_d_b1: f64 = (1e-12 * s.db[337][1]);
        let eq8_e151_q_d_b2: f64 = (1e-12 * s.db[337][2]);
        let eq8_e151_q_d_b3: f64 = (1e-12 * s.db[337][3]);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3, eq8_e151_q, eq8_e151_q_d_n0, eq8_e151_q_d_n1, eq8_e151_q_d_n2, eq8_e151_q_d_n3, eq8_e151_q_d_n4, eq8_e151_q_d_n5, eq8_e151_q_d_b0, eq8_e151_q_d_b1, eq8_e151_q_d_b2, eq8_e151_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 6] = [eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5];
        let eq8_reactive_branch_derivatives: [f64; 4] = [eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3, eq11_e172_q, eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5, eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3,) = {
    if s.b[959] {
        let eq11_e168_q: f64 = s.v[344];
        let eq11_e169: f64 = (s.v[345] + s.v[344]);
        let eq11_e169_d_n0: f64 = (s.dn[345][0] + s.dn[344][0]);
        let eq11_e169_d_n1: f64 = (s.dn[345][1] + s.dn[344][1]);
        let eq11_e169_d_n2: f64 = (s.dn[345][2] + s.dn[344][2]);
        let eq11_e169_d_n3: f64 = (s.dn[345][3] + s.dn[344][3]);
        let eq11_e169_d_n4: f64 = (s.dn[345][4] + s.dn[344][4]);
        let eq11_e169_d_n5: f64 = (s.dn[345][5] + s.dn[344][5]);
        let eq11_e169_d_b0: f64 = (s.db[345][0] + s.db[344][0]);
        let eq11_e169_d_b1: f64 = (s.db[345][1] + s.db[344][1]);
        let eq11_e169_d_b2: f64 = (s.db[345][2] + s.db[344][2]);
        let eq11_e169_d_b3: f64 = (s.db[345][3] + s.db[344][3]);
        let eq11_e169_q: f64 = eq11_e168_q;
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);
        let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);
        let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);
        let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);
        let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);
        let eq11_e170_q_d_n0: f64 = (1e-13 * s.dn[344][0]);
        let eq11_e170_q_d_n1: f64 = (1e-13 * s.dn[344][1]);
        let eq11_e170_q_d_n2: f64 = (1e-13 * s.dn[344][2]);
        let eq11_e170_q_d_n3: f64 = (1e-13 * s.dn[344][3]);
        let eq11_e170_q_d_n4: f64 = (1e-13 * s.dn[344][4]);
        let eq11_e170_q_d_n5: f64 = (1e-13 * s.dn[344][5]);
        let eq11_e170_q_d_b0: f64 = (1e-13 * s.db[344][0]);
        let eq11_e170_q_d_b1: f64 = (1e-13 * s.db[344][1]);
        let eq11_e170_q_d_b2: f64 = (1e-13 * s.db[344][2]);
        let eq11_e170_q_d_b3: f64 = (1e-13 * s.db[344][3]);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3, eq11_e170_q, eq11_e170_q_d_n0, eq11_e170_q_d_n1, eq11_e170_q_d_n2, eq11_e170_q_d_n3, eq11_e170_q_d_n4, eq11_e170_q_d_n5, eq11_e170_q_d_b0, eq11_e170_q_d_b1, eq11_e170_q_d_b2, eq11_e170_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 6] = [eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5];
        let eq11_reactive_branch_derivatives: [f64; 4] = [eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e179_q: f64 = s.v[275];
        let eq13_reactive_node_derivatives: [f64; 6] = [s.dn[275][0], s.dn[275][1], s.dn[275][2], s.dn[275][3], s.dn[275][4], s.dn[275][5]];
        let eq13_reactive_branch_derivatives: [f64; 4] = [s.db[275][0], s.db[275][1], s.db[275][2], s.db[275][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e183: f64 = (s.v[274] - s.v[290]);
        let eq14_e183_d_n0: f64 = (s.dn[274][0] - s.dn[290][0]);
        let eq14_e183_d_n1: f64 = (s.dn[274][1] - s.dn[290][1]);
        let eq14_e183_d_n2: f64 = (s.dn[274][2] - s.dn[290][2]);
        let eq14_e183_d_n3: f64 = (s.dn[274][3] - s.dn[290][3]);
        let eq14_e183_d_n4: f64 = (s.dn[274][4] - s.dn[290][4]);
        let eq14_e183_d_n5: f64 = (s.dn[274][5] - s.dn[290][5]);
        let eq14_e183_d_b0: f64 = (s.db[274][0] - s.db[290][0]);
        let eq14_e183_d_b1: f64 = (s.db[274][1] - s.db[290][1]);
        let eq14_e183_d_b2: f64 = (s.db[274][2] - s.db[290][2]);
        let eq14_e183_d_b3: f64 = (s.db[274][3] - s.db[290][3]);
        let eq14_e184: f64 = (s.v[55] * eq14_e183);
        let eq14_e184_d_n0: f64 = ((s.dn[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_n0));
        let eq14_e184_d_n1: f64 = ((s.dn[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_n1));
        let eq14_e184_d_n2: f64 = ((s.dn[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_n2));
        let eq14_e184_d_n3: f64 = ((s.dn[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_n3));
        let eq14_e184_d_n4: f64 = ((s.dn[55][4] * eq14_e183) + (s.v[55] * eq14_e183_d_n4));
        let eq14_e184_d_n5: f64 = ((s.dn[55][5] * eq14_e183) + (s.v[55] * eq14_e183_d_n5));
        let eq14_e184_d_b0: f64 = ((s.db[55][0] * eq14_e183) + (s.v[55] * eq14_e183_d_b0));
        let eq14_e184_d_b1: f64 = ((s.db[55][1] * eq14_e183) + (s.v[55] * eq14_e183_d_b1));
        let eq14_e184_d_b2: f64 = ((s.db[55][2] * eq14_e183) + (s.v[55] * eq14_e183_d_b2));
        let eq14_e184_d_b3: f64 = ((s.db[55][3] * eq14_e183) + (s.v[55] * eq14_e183_d_b3));
        let eq14_e185_q: f64 = eq14_e184;
        let eq14_reactive_node_derivatives: [f64; 6] = [eq14_e184_d_n0, eq14_e184_d_n1, eq14_e184_d_n2, eq14_e184_d_n3, eq14_e184_d_n4, eq14_e184_d_n5];
        let eq14_reactive_branch_derivatives: [f64; 4] = [eq14_e184_d_b0, eq14_e184_d_b1, eq14_e184_d_b2, eq14_e184_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
