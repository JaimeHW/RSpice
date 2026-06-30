#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            s.store_scalar(559, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
            s.store_scalar(562, 0.0);
            s.store_scalar(563, 0.0);
            s.store_scalar(564, 0.0);
            s.store_scalar(565, 0.0);
            s.store_scalar(566, 0.0);
            s.store_scalar(567, 0.0);
            s.store_scalar(568, 0.0);
            s.store_scalar(569, 0.0);
            s.store_scalar(570, 0.0);
            s.store_scalar(571, 0.0);
            s.store_scalar(572, 0.0);
            s.store_scalar(573, 0.0);
            s.store_scalar(574, 0.0);
            s.store_scalar(575, 0.0);
            s.store_scalar(576, 0.0);
            s.store_scalar(577, 0.0);
            s.store_scalar(578, 0.0);
            s.store_scalar(579, 0.0);
        }

        if s.b[467] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(576, 478, A::tanh_scaled_input(s.ad_value(478), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(576, 478, p.p53);
                } else {
                    s.store_scalar(576, 0.0);
                }
            }
        }

        if s.b[467] {
            s.store_sub(577, 477, 478);
            s.store_mul(511, 497, 485);
            s.store_add_scaled_product_value_ad(513, A::div_scaled_inputs(s.ad_value(493), 1.0, s.ad_value(485), 2.302585092994046), 1.0, 496, 576, 1.0);
            s.store_add_scaled_product_right_sub(514, 492, 1.0, 503, 483, 484, 1.0);
            s.store_pow_ad(532, A::div(s.ad_value(483), s.ad_value(484)), s.ad_value(505));
        }

        s.b[580] = (s.v[504] != 0.0);
        s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[580]) {
            s.store_div_ad_rhs(515, 576, A::pow(A::offset(A::pow(A::div(s.ad_value(576), s.ad_value(504)), s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.b[467] && (!s.b[580])) {
            s.store_scalar(515, 0.0);
        }

        if s.b[467] {
            s.store_mul_add_scaled_product_rhs(512, 576, s.ad_value(494), 1.0, s.ad_value(515), s.ad_value(495), (-1.0));
            s.store_sub(475, 514, 512);
            s.store_scaled_mul(517, 513, 485, 2.0);
            s.store_mul(518, 488, 517);
            s.store_sub_scaled_inputs(575, 475, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aii(574, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 575, (-1.0), 511, 1.0);
        }

        s.b[581] = (s.v[574] > 50.0);
        s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[581]) {
            s.store_scalar(533, 0.0);
        }

        s.b[582] = (s.v[574] < (-50.0));
        s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[581])) && s.b[582]) {
            s.store_scalar(533, 1.0);
        }

        if ((s.b[467] && (!s.b[581])) && (!s.b[582])) {
            s.store_div_from_scalar_offset_ad(533, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aai(534, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(533), (-(p.p51 * 0.1))), (-1.0), 517, 1.0);
        }

        s.b[583] = (s.v[534] > 50.0);
        s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[583]) {
            s.store_mul(535, 518, 534);
        }

        s.b[584] = (s.v[534] < (-50.0));
        s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[583])) && s.b[584]) {
            s.store_mul_exp_rhs(535, 518, 534);
        }

        if ((s.b[467] && (!s.b[583])) && (!s.b[584])) {
            s.store_mul_ln_one_plus_exp_rhs(535, 518, 534);
        }

        if s.b[467] {
            s.store_div_ad_rhs(521, 499, A::mul_offset_rhs(s.ad_value(532), A::div_scaled_product(s.ad_value(501), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(522, 498, A::div_scaled_offset_numerator(A::mul(s.ad_value(506), s.ad_value(484)), 1.0, 1.0, A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(507), s.ad_value(576), 1.0, s.ad_value(487), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(502), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(539, 522, 487, 1.0, 521, 1.0);
            s.store_add_scaled_product_right_ad(540, 539, (-1.0), 539, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(535), 2.0, s.ad_value(488), s.ad_value(539), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(541, A::mul_sub_from_scalar_rhs(s.ad_value(539), 1.0, s.ad_value(533)), 1.0, 517, 533, 1.0);
            s.store_add_scaled_product_value_ad(476, A::mul_sub_from_scalar_rhs(s.ad_value(540), 1.0, s.ad_value(533)), 1.0, 517, 533, 1.0);
        }

        if s.b[467] {
            s.store_div_from_scalar_pow_ad(542, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::div(s.ad_value(478), s.ad_value(476)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(476))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(478), s.ad_value(476))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(543, 478, 542);
        }

        if s.b[467] {
            s.store_div_from_scalar_pow_ad(544, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(545, 478, 544);
            s.store_div_scaled_inputs2_indices(574, 477, 1.0, 575, (-1.0), 511, 1.0);
        }

        s.b[585] = (s.v[574] > 50.0);
        s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[585]) {
            s.store_scalar(516, 0.0);
        }

        s.b[586] = (s.v[574] < (-50.0));
        s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[585])) && s.b[586]) {
            s.store_scalar(516, 1.0);
        }

        if ((s.b[467] && (!s.b[585])) && (!s.b[586])) {
            s.store_div_from_scalar_offset_ad(516, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(519, 577, 1.0, 545, (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(516), (-(p.p51 * 0.1))), -1.0, 517, 1.0);
        }

        s.b[587] = (s.v[519] > 50.0);
        s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[587]) {
            s.store_mul(520, 518, 519);
        }

        s.b[588] = (s.v[519] < (-50.0));
        s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[587])) && s.b[588]) {
            s.store_mul_exp_rhs(520, 518, 519);
        }

        if ((s.b[467] && (!s.b[587])) && (!s.b[588])) {
            s.store_mul_ln_one_plus_exp_rhs(520, 518, 519);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_indices(574, 577, 1.0, 575, (-1.0), 511, 1.0);
        }

        s.b[589] = (s.v[574] > 50.0);
        s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[589]) {
            s.store_scalar(546, 0.0);
        }

        s.b[590] = (s.v[574] < (-50.0));
        s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[589])) && s.b[590]) {
            s.store_scalar(546, 1.0);
        }

        if ((s.b[467] && (!s.b[589])) && (!s.b[590])) {
            s.store_div_from_scalar_offset_ad(546, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(547, 477, 1.0, 543, (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(546), (-(p.p51 * 0.1))), -1.0, 517, 1.0);
        }

        s.b[591] = (s.v[547] > 50.0);
        s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[591]) {
            s.store_mul(548, 518, 547);
        }

        s.b[592] = (s.v[547] < (-50.0));
        s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[591])) && s.b[592]) {
            s.store_mul_exp_rhs(548, 518, 547);
        }

        if ((s.b[467] && (!s.b[591])) && (!s.b[592])) {
            s.store_mul_ln_one_plus_exp_rhs(548, 518, 547);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_indices(549, 520, 1.0, 548, (-1.0), 488, 1.0);
            s.store_div(575, 549, 541);
            s.store_div_scaled_inputs_indices(524, 493, 1.0, 485, 2.302585092994046);
            s.store_scaled_mul(526, 524, 485, 2.0);
            s.store_mul(527, 488, 526);
            s.store_sub_scaled_inputs(579, 514, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aii(578, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 579, (-1.0), 511, 1.0);
        }

        s.b[593] = (s.v[578] > 50.0);
        s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[593]) {
            s.store_scalar(536, 0.0);
        }

        s.b[594] = (s.v[578] < (-50.0));
        s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[593])) && s.b[594]) {
            s.store_scalar(536, 1.0);
        }

        if ((s.b[467] && (!s.b[593])) && (!s.b[594])) {
            s.store_div_from_scalar_offset_ad(536, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aai(537, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(536), (-(p.p51 * 0.1))), (-1.0), 526, 1.0);
        }

        s.b[595] = (s.v[537] > 50.0);
        s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[595]) {
            s.store_mul(538, 527, 537);
        }

        s.b[596] = (s.v[537] < (-50.0));
        s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[595])) && s.b[596]) {
            s.store_mul_exp_rhs(538, 527, 537);
        }

        if ((s.b[467] && (!s.b[595])) && (!s.b[596])) {
            s.store_mul_ln_one_plus_exp_rhs(538, 527, 537);
        }

        if s.b[467] {
            s.store_div(530, 499, 532);
            s.store_mul_div_scaled_offset_numerator_rhs(531, 498, A::mul(s.ad_value(506), s.ad_value(484)), 1.0, 1.0, A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0), 1.0);
            s.store_div_scaled_product_indices(552, 531, 487, 1.0, 530, 1.0);
            s.store_add_scaled_product_right_ad(553, 552, (-1.0), 552, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(538), 2.0, s.ad_value(488), s.ad_value(552), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(554, A::mul_sub_from_scalar_rhs(s.ad_value(553), 1.0, s.ad_value(536)), 1.0, 526, 536, 1.0);
        }

        if s.b[467] {
            s.store_div_from_scalar_pow_ad(555, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::div(s.ad_value(478), s.ad_value(554)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(554))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(478), s.ad_value(554))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(556, 478, 555);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            s.store_div_from_scalar_pow_ad(557, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(558, 478, 557);
            s.store_div_scaled_inputs2_indices(578, 477, 1.0, 579, (-1.0), 511, 1.0);
        }

        s.b[597] = (s.v[578] > 50.0);
        s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[597]) {
            s.store_scalar(525, 0.0);
        }

        s.b[598] = (s.v[578] < (-50.0));
        s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[597])) && s.b[598]) {
            s.store_scalar(525, 1.0);
        }

        if ((s.b[467] && (!s.b[597])) && (!s.b[598])) {
            s.store_div_from_scalar_offset_ad(525, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(528, 577, 1.0, 558, (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(525), (-(p.p51 * 0.1))), -1.0, 526, 1.0);
        }

        s.b[599] = (s.v[528] > 50.0);
        s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[599]) {
            s.store_mul(529, 527, 528);
        }

        s.b[600] = (s.v[528] < (-50.0));
        s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[599])) && s.b[600]) {
            s.store_mul_exp_rhs(529, 527, 528);
        }

        if ((s.b[467] && (!s.b[599])) && (!s.b[600])) {
            s.store_mul_ln_one_plus_exp_rhs(529, 527, 528);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_indices(578, 577, 1.0, 579, (-1.0), 511, 1.0);
        }

        s.b[601] = (s.v[578] > 50.0);
        s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[601]) {
            s.store_scalar(559, 0.0);
        }

        s.b[602] = (s.v[578] < (-50.0));
        s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[601])) && s.b[602]) {
            s.store_scalar(559, 1.0);
        }

        if ((s.b[467] && (!s.b[601])) && (!s.b[602])) {
            s.store_div_from_scalar_offset_ad(559, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(560, 477, 1.0, 556, (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(559), (-(p.p51 * 0.1))), -1.0, 526, 1.0);
        }

        s.b[603] = (s.v[560] > 50.0);
        s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[603]) {
            s.store_mul(561, 527, 560);
        }

        s.b[604] = (s.v[560] < (-50.0));
        s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[603])) && s.b[604]) {
            s.store_mul_exp_rhs(561, 527, 560);
        }

        if ((s.b[467] && (!s.b[603])) && (!s.b[604])) {
            s.store_mul_ln_one_plus_exp_rhs(561, 527, 560);
        }

        if s.b[467] {
            s.store_offset_square(562, 529, 1e-38);
            s.store_offset_mul(563, 562, 529, 1e-57);
            s.store_offset_square(564, 561, 1e-38);
            s.store_offset_mul(565, 564, 561, 1e-57);
            s.store_offset_mul(566, 529, 561, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(567, 562, (2.0 / 3.0), 564, (2.0 / 3.0), 566, (2.0 / 3.0), A::offset(A::add(s.ad_value(529), s.ad_value(561)), 2e-19), 1.0);
            s.store_div_ad(568, A::add_scaled_inputs_products(s.ad_value(563), (2.0 * 2.0), s.ad_value(565), (3.0 * 2.0), s.ad_value(562), s.ad_value(561), (4.0 * 2.0), s.ad_value(564), s.ad_value(529), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(562), 15.0, s.ad_value(564), 15.0, s.ad_value(566), (2.0 * 15.0)));
            s.store_sub(569, 567, 568);
            s.copy_ad(570, 568);
            s.store_mul_product3_mixed_iaii(470, 510, A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), 509, 569, 1.0);
            s.store_mul_product3_mixed_iaii(471, 510, A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), 509, 570, 1.0);
        }

        s.b[605] = (s.v[479] == 1.0);
        s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[605]) {
            s.store_div_scaled_inputs3_indices(571, 480, 1.0, 514, -1.0, 511, (-(-(p.p51 * 0.5))), 526, 1.0);
        }

        s.b[606] = (s.v[571] > 50.0);
        s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });

        if ((s.b[467] && s.b[605]) && s.b[606]) {
            s.copy_ad(574, 571);
        }

        s.b[607] = (s.v[571] < (-50.0));
        s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && s.b[607]) {
            s.store_exp(574, 571);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && (!s.b[607])) {
            s.store_ln_one_plus_exp(574, 571);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs_mixed_ai(472, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(490), s.ad_value(526)), 574, 510);
            s.store_div_scaled_inputs3_indices(572, 481, 1.0, 514, -1.0, 511, (-(-(p.p51 * 0.5))), 526, 1.0);
        }

        s.b[608] = (s.v[572] > 50.0);
        s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });

        if ((s.b[467] && s.b[605]) && s.b[608]) {
            s.copy_ad(574, 572);
        }

        s.b[609] = (s.v[572] < (-50.0));
        s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && s.b[609]) {
            s.store_exp(574, 572);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && (!s.b[609])) {
            s.store_ln_one_plus_exp(574, 572);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs_mixed_ai(473, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(491), s.ad_value(526)), 574, 510);
        }

        if (s.b[467] && (!s.b[605])) {
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
        }

        s.b[610] = (s.v[482] == 1.0);
        s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[610]) {
            s.store_div_scaled_inputs3_indices(573, 477, 1.0, 514, -1.0, 511, (-(-(p.p51 * 0.5))), 526, 1.0);
        }

        s.b[611] = (s.v[573] > 50.0);
        s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });

        if ((s.b[467] && s.b[610]) && s.b[611]) {
            s.copy_ad(574, 573);
        }

        s.b[612] = (s.v[573] < (-50.0));
        s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && s.b[612]) {
            s.store_exp(574, 573);
        }

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && (!s.b[612])) {
            s.store_ln_one_plus_exp(574, 573);
        }

        if (s.b[467] && s.b[610]) {
            s.store_mul_ad_product_lhs_mixed_ai(474, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(489), s.ad_value(526)), 574, 510);
        }

        if (s.b[467] && (!s.b[610])) {
            s.store_scalar(474, 0.0);
        }

        if s.b[467] {
            s.copy_ad(203, 470);
            s.copy_ad(204, 471);
            s.copy_ad(205, 472);
            s.copy_ad(206, 473);
            s.copy_ad(207, 474);
        }

        s.b[613] = (p.p210 == 1.0);
        s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });

        s.store_scalar(197, 0.0);

        s.store_scalar(198, 0.0);

        s.store_scalar(199, 0.0);

        s.store_scalar(200, 0.0);

        s.store_scalar(201, 0.0);

        s.b[614] = (p.p189 > p.p354);
        s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });

        if s.b[614] {
            s.store_scalar(617, 0.0);
            s.store_scalar(618, 0.0);
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
            s.store_scalar(621, 0.0);
            s.store_scalar(622, 0.0);
            s.store_scalar(623, 0.0);
            s.copy_ad(624, 90);
            s.copy_ad(625, 91);
            s.store_scalar(626, p.p195);
            s.copy_ad(627, 92);
            s.copy_ad(628, 93);
            s.store_scalar(629, p.p193);
            s.copy_ad(630, 111);
            s.store_scalar(631, s.v[109]);
            s.copy_ad(632, 113);
            s.store_scalar(633, p.p0);
            s.store_scalar(634, p.p189);
            s.copy_ad(635, 35);
            s.store_scalar(636, p.p194);
            s.copy_ad(637, 36);
            s.copy_ad(638, 37);
            s.store_scalar(639, p.p190);
            s.store_scalar(640, p.p204);
            s.store_scalar(641, p.p203);
            s.store_scalar(642, 0.0);
            s.store_scalar(643, p.p205);
            s.store_scalar(644, p.p209);
            s.store_scalar(645, p.p200);
            s.store_scalar(646, p.p201);
            s.store_scalar(647, p.p202);
            s.store_scalar(648, p.p208);
            s.store_scalar(649, p.p207);
            s.store_scalar(650, p.p206);
            s.store_scalar(651, p.p39);
            s.store_scalar(652, p.p47);
            s.store_scalar(653, p.p45);
            s.store_scalar(654, p.p42);
            s.store_scalar(655, p.p2);
            s.store_scalar(656, p.p6);
            s.store_scalar(657, 1.0);
            s.store_scalar(658, 0.0);
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
            s.store_scalar(671, 0.0);
            s.store_scalar(672, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            s.store_scalar(675, 0.0);
            s.store_scalar(676, 0.0);
            s.store_scalar(677, 0.0);
            s.store_scalar(678, 0.0);
            s.store_scalar(679, 0.0);
            s.store_scalar(680, 0.0);
            s.store_scalar(681, 0.0);
            s.store_scalar(682, 0.0);
            s.store_scalar(683, 0.0);
            s.store_scalar(684, 0.0);
            s.store_scalar(685, 0.0);
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(688, 0.0);
            s.store_scalar(689, 0.0);
            s.store_scalar(690, 0.0);
            s.store_scalar(691, 0.0);
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
            s.store_scalar(694, 0.0);
            s.store_scalar(695, 0.0);
            s.store_scalar(696, 0.0);
            s.store_scalar(699, 0.0);
            s.store_scalar(700, 0.0);
            s.store_scalar(701, 0.0);
            s.store_scalar(702, 0.0);
            s.store_scalar(703, 0.0);
            s.store_scalar(704, 0.0);
            s.store_scalar(705, 0.0);
            s.store_scalar(706, 0.0);
            s.store_scalar(707, 0.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(709, 0.0);
            s.store_scalar(710, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(712, 0.0);
            s.store_scalar(713, 0.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(715, 0.0);
            s.store_scalar(716, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(718, 0.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(721, 0.0);
            s.store_scalar(722, 0.0);
            s.store_scalar(723, 0.0);
            s.store_scalar(724, 0.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(726, 0.0);
        }

        if s.b[614] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(723, 625, A::tanh_scaled_input(s.ad_value(625), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(723, 625, p.p53);
                } else {
                    s.store_scalar(723, 0.0);
                }
            }
        }

        if s.b[614] {
            s.store_sub(724, 624, 625);
            s.store_mul(658, 644, 632);
            s.store_add_scaled_product_value_ad(660, A::div_scaled_inputs(s.ad_value(640), 1.0, s.ad_value(632), 2.302585092994046), 1.0, 643, 723, 1.0);
            s.store_add_scaled_product_right_sub(661, 639, 1.0, 650, 630, 631, 1.0);
            s.store_pow_ad(679, A::div(s.ad_value(630), s.ad_value(631)), s.ad_value(652));
        }

        s.b[727] = (s.v[651] != 0.0);
        s.store_scalar(727, if s.b[727] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[727]) {
            s.store_div_ad_rhs(662, 723, A::pow(A::offset(A::pow(A::div(s.ad_value(723), s.ad_value(651)), s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.b[614] && (!s.b[727])) {
            s.store_scalar(662, 0.0);
        }

        if s.b[614] {
            s.store_mul_add_scaled_product_rhs(659, 723, s.ad_value(641), 1.0, s.ad_value(662), s.ad_value(642), (-1.0));
            s.store_sub(622, 661, 659);
            s.store_scaled_mul(664, 660, 632, 2.0);
            s.store_mul(665, 635, 664);
            s.store_sub_scaled_inputs(722, 622, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aii(721, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[728] = (s.v[721] > 50.0);
        s.store_scalar(728, if s.b[728] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[728]) {
            s.store_scalar(680, 0.0);
        }

        s.b[729] = (s.v[721] < (-50.0));
        s.store_scalar(729, if s.b[729] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[728])) && s.b[729]) {
            s.store_scalar(680, 1.0);
        }

        if ((s.b[614] && (!s.b[728])) && (!s.b[729])) {
            s.store_div_from_scalar_offset_ad(680, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aai(681, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(680), (-(p.p51 * 0.1))), (-1.0), 664, 1.0);
        }

        s.b[730] = (s.v[681] > 50.0);
        s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[730]) {
            s.store_mul(682, 665, 681);
        }

        s.b[731] = (s.v[681] < (-50.0));
        s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[730])) && s.b[731]) {
            s.store_mul_exp_rhs(682, 665, 681);
        }

        if ((s.b[614] && (!s.b[730])) && (!s.b[731])) {
            s.store_mul_ln_one_plus_exp_rhs(682, 665, 681);
        }

        if s.b[614] {
            s.store_div_ad_rhs(668, 646, A::mul_offset_rhs(s.ad_value(679), A::div_scaled_product(s.ad_value(648), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(669, 645, A::div_scaled_offset_numerator(A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(654), s.ad_value(723), 1.0, s.ad_value(634), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(649), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(686, 669, 634, 1.0, 668, 1.0);
            s.store_add_scaled_product_right_ad(687, 686, (-1.0), 686, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(682), 2.0, s.ad_value(635), s.ad_value(686), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(688, A::mul_sub_from_scalar_rhs(s.ad_value(686), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
            s.store_add_scaled_product_value_ad(623, A::mul_sub_from_scalar_rhs(s.ad_value(687), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(689, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::div(s.ad_value(625), s.ad_value(623)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(623))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(625), s.ad_value(623))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(690, 625, 689);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(691, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(692, 625, 691);
            s.store_div_scaled_inputs2_indices(721, 624, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[732] = (s.v[721] > 50.0);
        s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[732]) {
            s.store_scalar(663, 0.0);
        }

        s.b[733] = (s.v[721] < (-50.0));
        s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[732])) && s.b[733]) {
            s.store_scalar(663, 1.0);
        }

        if ((s.b[614] && (!s.b[732])) && (!s.b[733])) {
            s.store_div_from_scalar_offset_ad(663, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(666, 724, 1.0, 692, (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(663), (-(p.p51 * 0.1))), -1.0, 664, 1.0);
        }

        s.b[734] = (s.v[666] > 50.0);
        s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[734]) {
            s.store_mul(667, 665, 666);
        }

        s.b[735] = (s.v[666] < (-50.0));
        s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[734])) && s.b[735]) {
            s.store_mul_exp_rhs(667, 665, 666);
        }

        if ((s.b[614] && (!s.b[734])) && (!s.b[735])) {
            s.store_mul_ln_one_plus_exp_rhs(667, 665, 666);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(721, 724, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[736] = (s.v[721] > 50.0);
        s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[736]) {
            s.store_scalar(693, 0.0);
        }

        s.b[737] = (s.v[721] < (-50.0));
        s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[736])) && s.b[737]) {
            s.store_scalar(693, 1.0);
        }

        if ((s.b[614] && (!s.b[736])) && (!s.b[737])) {
            s.store_div_from_scalar_offset_ad(693, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(694, 624, 1.0, 690, (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(693), (-(p.p51 * 0.1))), -1.0, 664, 1.0);
        }

        s.b[738] = (s.v[694] > 50.0);
        s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[738]) {
            s.store_mul(695, 665, 694);
        }

        s.b[739] = (s.v[694] < (-50.0));
        s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[738])) && s.b[739]) {
            s.store_mul_exp_rhs(695, 665, 694);
        }

        if ((s.b[614] && (!s.b[738])) && (!s.b[739])) {
            s.store_mul_ln_one_plus_exp_rhs(695, 665, 694);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(696, 667, 1.0, 695, (-1.0), 635, 1.0);
            s.store_div(722, 696, 688);
            s.store_div_scaled_inputs_indices(671, 640, 1.0, 632, 2.302585092994046);
            s.store_scaled_mul(673, 671, 632, 2.0);
            s.store_mul(674, 635, 673);
            s.store_sub_scaled_inputs(726, 661, 1.0, 658, (p.p51 * 0.5));
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aii(725, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[740] = (s.v[725] > 50.0);
        s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[740]) {
            s.store_scalar(683, 0.0);
        }

        s.b[741] = (s.v[725] < (-50.0));
        s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[740])) && s.b[741]) {
            s.store_scalar(683, 1.0);
        }

        if ((s.b[614] && (!s.b[740])) && (!s.b[741])) {
            s.store_div_from_scalar_offset_ad(683, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aai(684, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(683), (-(p.p51 * 0.1))), (-1.0), 673, 1.0);
        }

        s.b[742] = (s.v[684] > 50.0);
        s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[742]) {
            s.store_mul(685, 674, 684);
        }

        s.b[743] = (s.v[684] < (-50.0));
        s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[742])) && s.b[743]) {
            s.store_mul_exp_rhs(685, 674, 684);
        }

        if ((s.b[614] && (!s.b[742])) && (!s.b[743])) {
            s.store_mul_ln_one_plus_exp_rhs(685, 674, 684);
        }

        if s.b[614] {
            s.store_div(677, 646, 679);
            s.store_mul_div_scaled_offset_numerator_rhs(678, 645, A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0);
            s.store_div_scaled_product_indices(699, 678, 634, 1.0, 677, 1.0);
            s.store_add_scaled_product_right_ad(700, 699, (-1.0), 699, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(685), 2.0, s.ad_value(635), s.ad_value(699), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(701, A::mul_sub_from_scalar_rhs(s.ad_value(700), 1.0, s.ad_value(683)), 1.0, 673, 683, 1.0);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(702, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::div(s.ad_value(625), s.ad_value(701)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(701))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(625), s.ad_value(701))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(703, 625, 702);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(704, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(705, 625, 704);
            s.store_div_scaled_inputs2_indices(725, 624, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[744] = (s.v[725] > 50.0);
        s.store_scalar(744, if s.b[744] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[744]) {
            s.store_scalar(672, 0.0);
        }

        s.b[745] = (s.v[725] < (-50.0));
        s.store_scalar(745, if s.b[745] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[744])) && s.b[745]) {
            s.store_scalar(672, 1.0);
        }

        if ((s.b[614] && (!s.b[744])) && (!s.b[745])) {
            s.store_div_from_scalar_offset_ad(672, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(675, 724, 1.0, 705, (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(672), (-(p.p51 * 0.1))), -1.0, 673, 1.0);
        }

        s.b[746] = (s.v[675] > 50.0);
        s.store_scalar(746, if s.b[746] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[746]) {
            s.store_mul(676, 674, 675);
        }

        s.b[747] = (s.v[675] < (-50.0));
        s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[746])) && s.b[747]) {
            s.store_mul_exp_rhs(676, 674, 675);
        }

        if ((s.b[614] && (!s.b[746])) && (!s.b[747])) {
            s.store_mul_ln_one_plus_exp_rhs(676, 674, 675);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(725, 724, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[748] = (s.v[725] > 50.0);
        s.store_scalar(748, if s.b[748] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[748]) {
            s.store_scalar(706, 0.0);
        }

        s.b[749] = (s.v[725] < (-50.0));
        s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[748])) && s.b[749]) {
            s.store_scalar(706, 1.0);
        }

        if ((s.b[614] && (!s.b[748])) && (!s.b[749])) {
            s.store_div_from_scalar_offset_ad(706, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(707, 624, 1.0, 703, (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(706), (-(p.p51 * 0.1))), -1.0, 673, 1.0);
        }

        s.b[750] = (s.v[707] > 50.0);
        s.store_scalar(750, if s.b[750] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[750]) {
            s.store_mul(708, 674, 707);
        }

        s.b[751] = (s.v[707] < (-50.0));
        s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[750])) && s.b[751]) {
            s.store_mul_exp_rhs(708, 674, 707);
        }

        if ((s.b[614] && (!s.b[750])) && (!s.b[751])) {
            s.store_mul_ln_one_plus_exp_rhs(708, 674, 707);
        }

        if s.b[614] {
            s.store_offset_square(709, 676, 1e-38);
            s.store_offset_mul(710, 709, 676, 1e-57);
            s.store_offset_square(711, 708, 1e-38);
            s.store_offset_mul(712, 711, 708, 1e-57);
            s.store_offset_mul(713, 676, 708, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(714, 709, (2.0 / 3.0), 711, (2.0 / 3.0), 713, (2.0 / 3.0), A::offset(A::add(s.ad_value(676), s.ad_value(708)), 2e-19), 1.0);
            s.store_div_ad(715, A::add_scaled_inputs_products(s.ad_value(710), (2.0 * 2.0), s.ad_value(712), (3.0 * 2.0), s.ad_value(709), s.ad_value(708), (4.0 * 2.0), s.ad_value(711), s.ad_value(676), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(709), 15.0, s.ad_value(711), 15.0, s.ad_value(713), (2.0 * 15.0)));
            s.store_sub(716, 714, 715);
            s.copy_ad(717, 715);
            s.store_mul_product3_mixed_iaii(617, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), 656, 716, 1.0);
            s.store_mul_product3_mixed_iaii(618, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), 656, 717, 1.0);
        }

        s.b[752] = (s.v[626] == 1.0);
        s.store_scalar(752, if s.b[752] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[752]) {
            s.store_div_scaled_inputs3_indices(718, 627, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[753] = (s.v[718] > 50.0);
        s.store_scalar(753, if s.b[753] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[752]) && s.b[753]) {
            s.copy_ad(721, 718);
        }

        s.b[754] = (s.v[718] < (-50.0));
        s.store_scalar(754, if s.b[754] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && s.b[754]) {
            s.store_exp(721, 718);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && (!s.b[754])) {
            s.store_ln_one_plus_exp(721, 718);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs_mixed_ai(619, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(637), s.ad_value(673)), 721, 657);
            s.store_div_scaled_inputs3_indices(719, 628, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[755] = (s.v[719] > 50.0);
        s.store_scalar(755, if s.b[755] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[752]) && s.b[755]) {
            s.copy_ad(721, 719);
        }

        s.b[756] = (s.v[719] < (-50.0));
        s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && s.b[756]) {
            s.store_exp(721, 719);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && (!s.b[756])) {
            s.store_ln_one_plus_exp(721, 719);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs_mixed_ai(620, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(638), s.ad_value(673)), 721, 657);
        }

        if (s.b[614] && (!s.b[752])) {
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
        }

        s.b[757] = (s.v[629] == 1.0);
        s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[757]) {
            s.store_div_scaled_inputs3_indices(720, 624, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[758] = (s.v[720] > 50.0);
        s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[757]) && s.b[758]) {
            s.copy_ad(721, 720);
        }

        s.b[759] = (s.v[720] < (-50.0));
        s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && s.b[759]) {
            s.store_exp(721, 720);
        }

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && (!s.b[759])) {
            s.store_ln_one_plus_exp(721, 720);
        }

        if (s.b[614] && s.b[757]) {
            s.store_mul_ad_product_lhs_mixed_ai(621, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(636), s.ad_value(673)), 721, 657);
        }

        if (s.b[614] && (!s.b[757])) {
            s.store_scalar(621, 0.0);
        }

        if s.b[614] {
            s.copy_ad(197, 617);
            s.copy_ad(198, 618);
            s.copy_ad(199, 619);
            s.copy_ad(200, 620);
            s.copy_ad(201, 621);
        }

        s.b[760] = (p.p188 == 1.0);
        s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });

        s.store_scalar(191, 0.0);

        s.store_scalar(192, 0.0);

        s.store_scalar(193, 0.0);

        s.store_scalar(194, 0.0);

        s.store_scalar(195, 0.0);

        s.b[761] = (p.p167 > p.p354);
        s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });

        if s.b[761] {
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
            s.store_scalar(768, 0.0);
            s.store_scalar(769, 0.0);
            s.store_scalar(770, 0.0);
            s.copy_ad(771, 84);
            s.copy_ad(772, 85);
            s.store_scalar(773, p.p173);
            s.copy_ad(774, 86);
            s.copy_ad(775, 87);
            s.store_scalar(776, p.p171);
            s.copy_ad(777, 111);
            s.store_scalar(778, s.v[109]);
            s.copy_ad(779, 113);
            s.store_scalar(780, p.p0);
            s.store_scalar(781, p.p167);
            s.copy_ad(782, 32);
            s.store_scalar(783, p.p172);
            s.copy_ad(784, 33);
            s.copy_ad(785, 34);
            s.store_scalar(786, p.p168);
            s.store_scalar(787, p.p182);
            s.store_scalar(788, p.p181);
            s.store_scalar(789, 0.0);
            s.store_scalar(790, p.p183);
            s.store_scalar(791, p.p187);
            s.store_scalar(792, p.p178);
            s.store_scalar(793, p.p179);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            s.store_scalar(794, p.p180);
            s.store_scalar(795, p.p186);
            s.store_scalar(796, p.p185);
            s.store_scalar(797, p.p184);
            s.store_scalar(798, p.p39);
            s.store_scalar(799, p.p47);
            s.store_scalar(800, p.p45);
            s.store_scalar(801, p.p42);
            s.store_scalar(802, p.p2);
            s.store_scalar(803, p.p6);
            s.store_scalar(804, 1.0);
            s.store_scalar(805, 0.0);
            s.store_scalar(806, 0.0);
            s.store_scalar(807, 0.0);
            s.store_scalar(808, 0.0);
            s.store_scalar(809, 0.0);
            s.store_scalar(810, 0.0);
            s.store_scalar(811, 0.0);
            s.store_scalar(812, 0.0);
            s.store_scalar(813, 0.0);
            s.store_scalar(814, 0.0);
            s.store_scalar(815, 0.0);
            s.store_scalar(816, 0.0);
            s.store_scalar(818, 0.0);
            s.store_scalar(819, 0.0);
            s.store_scalar(820, 0.0);
            s.store_scalar(821, 0.0);
            s.store_scalar(822, 0.0);
            s.store_scalar(823, 0.0);
            s.store_scalar(824, 0.0);
            s.store_scalar(825, 0.0);
            s.store_scalar(826, 0.0);
            s.store_scalar(827, 0.0);
            s.store_scalar(828, 0.0);
            s.store_scalar(829, 0.0);
            s.store_scalar(830, 0.0);
            s.store_scalar(831, 0.0);
            s.store_scalar(832, 0.0);
            s.store_scalar(833, 0.0);
            s.store_scalar(834, 0.0);
            s.store_scalar(835, 0.0);
            s.store_scalar(836, 0.0);
            s.store_scalar(837, 0.0);
            s.store_scalar(838, 0.0);
            s.store_scalar(839, 0.0);
            s.store_scalar(840, 0.0);
            s.store_scalar(841, 0.0);
            s.store_scalar(842, 0.0);
            s.store_scalar(843, 0.0);
            s.store_scalar(846, 0.0);
            s.store_scalar(847, 0.0);
            s.store_scalar(848, 0.0);
            s.store_scalar(849, 0.0);
            s.store_scalar(850, 0.0);
            s.store_scalar(851, 0.0);
            s.store_scalar(852, 0.0);
            s.store_scalar(853, 0.0);
            s.store_scalar(854, 0.0);
            s.store_scalar(855, 0.0);
            s.store_scalar(856, 0.0);
            s.store_scalar(857, 0.0);
            s.store_scalar(858, 0.0);
            s.store_scalar(859, 0.0);
            s.store_scalar(860, 0.0);
            s.store_scalar(861, 0.0);
            s.store_scalar(862, 0.0);
            s.store_scalar(863, 0.0);
            s.store_scalar(864, 0.0);
            s.store_scalar(865, 0.0);
            s.store_scalar(866, 0.0);
            s.store_scalar(867, 0.0);
            s.store_scalar(868, 0.0);
            s.store_scalar(869, 0.0);
            s.store_scalar(870, 0.0);
            s.store_scalar(871, 0.0);
            s.store_scalar(872, 0.0);
            s.store_scalar(873, 0.0);
        }

        if s.b[761] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(870, 772, A::tanh_scaled_input(s.ad_value(772), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(870, 772, p.p53);
                } else {
                    s.store_scalar(870, 0.0);
                }
            }
        }

        if s.b[761] {
            s.store_sub(871, 771, 772);
            s.store_mul(805, 791, 779);
            s.store_add_scaled_product_value_ad(807, A::div_scaled_inputs(s.ad_value(787), 1.0, s.ad_value(779), 2.302585092994046), 1.0, 790, 870, 1.0);
            s.store_add_scaled_product_right_sub(808, 786, 1.0, 797, 777, 778, 1.0);
            s.store_pow_ad(826, A::div(s.ad_value(777), s.ad_value(778)), s.ad_value(799));
        }

        s.b[874] = (s.v[798] != 0.0);
        s.store_scalar(874, if s.b[874] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[874]) {
            s.store_div_ad_rhs(809, 870, A::pow(A::offset(A::pow(A::div(s.ad_value(870), s.ad_value(798)), s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.b[761] && (!s.b[874])) {
            s.store_scalar(809, 0.0);
        }

        if s.b[761] {
            s.store_mul_add_scaled_product_rhs(806, 870, s.ad_value(788), 1.0, s.ad_value(809), s.ad_value(789), (-1.0));
            s.store_sub(769, 808, 806);
            s.store_scaled_mul(811, 807, 779, 2.0);
            s.store_mul(812, 782, 811);
            s.store_sub_scaled_inputs(869, 769, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aii(868, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 869, (-1.0), 805, 1.0);
        }

        s.b[875] = (s.v[868] > 50.0);
        s.store_scalar(875, if s.b[875] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[875]) {
            s.store_scalar(827, 0.0);
        }

        s.b[876] = (s.v[868] < (-50.0));
        s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[875])) && s.b[876]) {
            s.store_scalar(827, 1.0);
        }

        if ((s.b[761] && (!s.b[875])) && (!s.b[876])) {
            s.store_div_from_scalar_offset_ad(827, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aai(828, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(827), (-(p.p51 * 0.1))), (-1.0), 811, 1.0);
        }

        s.b[877] = (s.v[828] > 50.0);
        s.store_scalar(877, if s.b[877] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[877]) {
            s.store_mul(829, 812, 828);
        }

        s.b[878] = (s.v[828] < (-50.0));
        s.store_scalar(878, if s.b[878] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[877])) && s.b[878]) {
            s.store_mul_exp_rhs(829, 812, 828);
        }

        if ((s.b[761] && (!s.b[877])) && (!s.b[878])) {
            s.store_mul_ln_one_plus_exp_rhs(829, 812, 828);
        }

        if s.b[761] {
            s.store_div_ad_rhs(815, 793, A::mul_offset_rhs(s.ad_value(826), A::div_scaled_product(s.ad_value(795), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(816, 792, A::div_scaled_offset_numerator(A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(801), s.ad_value(870), 1.0, s.ad_value(781), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(796), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(833, 816, 781, 1.0, 815, 1.0);
            s.store_add_scaled_product_right_ad(834, 833, (-1.0), 833, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(829), 2.0, s.ad_value(782), s.ad_value(833), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(835, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(827)), 1.0, 811, 827, 1.0);
            s.store_add_scaled_product_value_ad(770, A::mul_sub_from_scalar_rhs(s.ad_value(834), 1.0, s.ad_value(827)), 1.0, 811, 827, 1.0);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(836, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::div(s.ad_value(772), s.ad_value(770)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(770))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(772), s.ad_value(770))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(837, 772, 836);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(838, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(839, 772, 838);
            s.store_div_scaled_inputs2_indices(868, 771, 1.0, 869, (-1.0), 805, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[879] = (s.v[868] > 50.0);
        s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[879]) {
            s.store_scalar(810, 0.0);
        }

        s.b[880] = (s.v[868] < (-50.0));
        s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[879])) && s.b[880]) {
            s.store_scalar(810, 1.0);
        }

        if ((s.b[761] && (!s.b[879])) && (!s.b[880])) {
            s.store_div_from_scalar_offset_ad(810, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(813, 871, 1.0, 839, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(810), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
        }

        s.b[881] = (s.v[813] > 50.0);
        s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[881]) {
            s.store_mul(814, 812, 813);
        }

        s.b[882] = (s.v[813] < (-50.0));
        s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[881])) && s.b[882]) {
            s.store_mul_exp_rhs(814, 812, 813);
        }

        if ((s.b[761] && (!s.b[881])) && (!s.b[882])) {
            s.store_mul_ln_one_plus_exp_rhs(814, 812, 813);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(868, 871, 1.0, 869, (-1.0), 805, 1.0);
        }

        s.b[883] = (s.v[868] > 50.0);
        s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[883]) {
            s.store_scalar(840, 0.0);
        }

        s.b[884] = (s.v[868] < (-50.0));
        s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[883])) && s.b[884]) {
            s.store_scalar(840, 1.0);
        }

        if ((s.b[761] && (!s.b[883])) && (!s.b[884])) {
            s.store_div_from_scalar_offset_ad(840, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(841, 771, 1.0, 837, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(840), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
        }

        s.b[885] = (s.v[841] > 50.0);
        s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[885]) {
            s.store_mul(842, 812, 841);
        }

        s.b[886] = (s.v[841] < (-50.0));
        s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[885])) && s.b[886]) {
            s.store_mul_exp_rhs(842, 812, 841);
        }

        if ((s.b[761] && (!s.b[885])) && (!s.b[886])) {
            s.store_mul_ln_one_plus_exp_rhs(842, 812, 841);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(843, 814, 1.0, 842, (-1.0), 782, 1.0);
            s.store_div(869, 843, 835);
            s.store_div_scaled_inputs_indices(818, 787, 1.0, 779, 2.302585092994046);
            s.store_scaled_mul(820, 818, 779, 2.0);
            s.store_mul(821, 782, 820);
            s.store_sub_scaled_inputs(873, 808, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aii(872, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[887] = (s.v[872] > 50.0);
        s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[887]) {
            s.store_scalar(830, 0.0);
        }

        s.b[888] = (s.v[872] < (-50.0));
        s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[887])) && s.b[888]) {
            s.store_scalar(830, 1.0);
        }

        if ((s.b[761] && (!s.b[887])) && (!s.b[888])) {
            s.store_div_from_scalar_offset_ad(830, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aai(831, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(830), (-(p.p51 * 0.1))), (-1.0), 820, 1.0);
        }

        s.b[889] = (s.v[831] > 50.0);
        s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[889]) {
            s.store_mul(832, 821, 831);
        }

        s.b[890] = (s.v[831] < (-50.0));
        s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[889])) && s.b[890]) {
            s.store_mul_exp_rhs(832, 821, 831);
        }

        if ((s.b[761] && (!s.b[889])) && (!s.b[890])) {
            s.store_mul_ln_one_plus_exp_rhs(832, 821, 831);
        }

        if s.b[761] {
            s.store_div(824, 793, 826);
            s.store_mul_div_scaled_offset_numerator_rhs(825, 792, A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0);
            s.store_div_scaled_product_indices(846, 825, 781, 1.0, 824, 1.0);
            s.store_add_scaled_product_right_ad(847, 846, (-1.0), 846, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(832), 2.0, s.ad_value(782), s.ad_value(846), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(848, A::mul_sub_from_scalar_rhs(s.ad_value(847), 1.0, s.ad_value(830)), 1.0, 820, 830, 1.0);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(849, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::div(s.ad_value(772), s.ad_value(848)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(848))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(772), s.ad_value(848))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(850, 772, 849);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(851, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(852, 772, 851);
            s.store_div_scaled_inputs2_indices(872, 771, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[891] = (s.v[872] > 50.0);
        s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[891]) {
            s.store_scalar(819, 0.0);
        }

        s.b[892] = (s.v[872] < (-50.0));
        s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[891])) && s.b[892]) {
            s.store_scalar(819, 1.0);
        }

        if ((s.b[761] && (!s.b[891])) && (!s.b[892])) {
            s.store_div_from_scalar_offset_ad(819, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(822, 871, 1.0, 852, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(819), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
        }

        s.b[893] = (s.v[822] > 50.0);
        s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[893]) {
            s.store_mul(823, 821, 822);
        }

        s.b[894] = (s.v[822] < (-50.0));
        s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[893])) && s.b[894]) {
            s.store_mul_exp_rhs(823, 821, 822);
        }

        if ((s.b[761] && (!s.b[893])) && (!s.b[894])) {
            s.store_mul_ln_one_plus_exp_rhs(823, 821, 822);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(872, 871, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[895] = (s.v[872] > 50.0);
        s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[895]) {
            s.store_scalar(853, 0.0);
        }

        s.b[896] = (s.v[872] < (-50.0));
        s.store_scalar(896, if s.b[896] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[895])) && s.b[896]) {
            s.store_scalar(853, 1.0);
        }

        if ((s.b[761] && (!s.b[895])) && (!s.b[896])) {
            s.store_div_from_scalar_offset_ad(853, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(854, 771, 1.0, 850, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(853), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
        }

        s.b[897] = (s.v[854] > 50.0);
        s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[897]) {
            s.store_mul(855, 821, 854);
        }

        s.b[898] = (s.v[854] < (-50.0));
        s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[897])) && s.b[898]) {
            s.store_mul_exp_rhs(855, 821, 854);
        }

        if ((s.b[761] && (!s.b[897])) && (!s.b[898])) {
            s.store_mul_ln_one_plus_exp_rhs(855, 821, 854);
        }

        if s.b[761] {
            s.store_offset_square(856, 823, 1e-38);
            s.store_offset_mul(857, 856, 823, 1e-57);
            s.store_offset_square(858, 855, 1e-38);
            s.store_offset_mul(859, 858, 855, 1e-57);
            s.store_offset_mul(860, 823, 855, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(861, 856, (2.0 / 3.0), 858, (2.0 / 3.0), 860, (2.0 / 3.0), A::offset(A::add(s.ad_value(823), s.ad_value(855)), 2e-19), 1.0);
            s.store_div_ad(862, A::add_scaled_inputs_products(s.ad_value(857), (2.0 * 2.0), s.ad_value(859), (3.0 * 2.0), s.ad_value(856), s.ad_value(855), (4.0 * 2.0), s.ad_value(858), s.ad_value(823), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(856), 15.0, s.ad_value(858), 15.0, s.ad_value(860), (2.0 * 15.0)));
            s.store_sub(863, 861, 862);
            s.copy_ad(864, 862);
            s.store_mul_product3_mixed_iaii(764, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), 803, 863, 1.0);
            s.store_mul_product3_mixed_iaii(765, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), 803, 864, 1.0);
        }

        s.b[899] = (s.v[773] == 1.0);
        s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[899]) {
            s.store_div_scaled_inputs3_indices(865, 774, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[900] = (s.v[865] > 50.0);
        s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[899]) && s.b[900]) {
            s.copy_ad(868, 865);
        }

        s.b[901] = (s.v[865] < (-50.0));
        s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && s.b[901]) {
            s.store_exp(868, 865);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && (!s.b[901])) {
            s.store_ln_one_plus_exp(868, 865);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs_mixed_ai(766, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(784), s.ad_value(820)), 868, 804);
            s.store_div_scaled_inputs3_indices(866, 775, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[902] = (s.v[866] > 50.0);
        s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[899]) && s.b[902]) {
            s.copy_ad(868, 866);
        }

        s.b[903] = (s.v[866] < (-50.0));
        s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && s.b[903]) {
            s.store_exp(868, 866);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && (!s.b[903])) {
            s.store_ln_one_plus_exp(868, 866);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs_mixed_ai(767, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(785), s.ad_value(820)), 868, 804);
        }

        if (s.b[761] && (!s.b[899])) {
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
        }

        s.b[904] = (s.v[776] == 1.0);
        s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[904]) {
            s.store_div_scaled_inputs3_indices(867, 771, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[905] = (s.v[867] > 50.0);
        s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[904]) && s.b[905]) {
            s.copy_ad(868, 867);
        }

        s.b[906] = (s.v[867] < (-50.0));
        s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && s.b[906]) {
            s.store_exp(868, 867);
        }

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && (!s.b[906])) {
            s.store_ln_one_plus_exp(868, 867);
        }

        if (s.b[761] && s.b[904]) {
            s.store_mul_ad_product_lhs_mixed_ai(768, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(783), s.ad_value(820)), 868, 804);
        }

        if (s.b[761] && (!s.b[904])) {
            s.store_scalar(768, 0.0);
        }

        if s.b[761] {
            s.copy_ad(191, 764);
            s.copy_ad(192, 765);
            s.copy_ad(193, 766);
            s.copy_ad(194, 767);
            s.copy_ad(195, 768);
        }

        s.b[907] = (p.p166 == 1.0);
        s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });

        s.store_scalar(167, 0.0);

        s.store_scalar(168, 0.0);

        s.store_scalar(169, 0.0);

        s.store_scalar(170, 0.0);

        s.store_scalar(171, 0.0);

        s.b[908] = (p.p79 > p.p354);
        s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });

        if s.b[908] {
            s.store_scalar(911, 0.0);
            s.store_scalar(912, 0.0);
            s.store_scalar(913, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[908] {
            s.store_scalar(914, 0.0);
            s.store_scalar(915, 0.0);
            s.store_scalar(916, 0.0);
            s.store_scalar(917, 0.0);
            s.copy_ad(918, 60);
            s.copy_ad(919, 61);
            s.store_scalar(920, p.p85);
            s.copy_ad(921, 62);
            s.copy_ad(922, 63);
            s.store_scalar(923, p.p83);
            s.copy_ad(924, 111);
            s.store_scalar(925, s.v[109]);
            s.copy_ad(926, 113);
            s.store_scalar(927, p.p0);
            s.store_scalar(928, p.p79);
            s.copy_ad(929, 20);
            s.store_scalar(930, p.p84);
            s.copy_ad(931, 21);
            s.copy_ad(932, 22);
            s.store_scalar(933, p.p80);
            s.store_scalar(934, p.p94);
            s.store_scalar(935, p.p93);
            s.store_scalar(936, 0.0);
            s.store_scalar(937, p.p95);
            s.store_scalar(938, p.p99);
            s.store_scalar(939, p.p90);
            s.store_scalar(940, p.p91);
            s.store_scalar(941, p.p92);
            s.store_scalar(942, p.p98);
            s.store_scalar(943, p.p97);
            s.store_scalar(944, p.p96);
            s.store_scalar(945, p.p39);
            s.store_scalar(946, p.p47);
            s.store_scalar(947, p.p45);
            s.store_scalar(948, p.p42);
            s.store_scalar(949, p.p2);
            s.store_scalar(950, p.p6);
            s.store_scalar(951, 1.0);
            s.store_scalar(952, 0.0);
            s.store_scalar(953, 0.0);
            s.store_scalar(954, 0.0);
            s.store_scalar(955, 0.0);
            s.store_scalar(956, 0.0);
            s.store_scalar(957, 0.0);
            s.store_scalar(958, 0.0);
            s.store_scalar(959, 0.0);
            s.store_scalar(960, 0.0);
            s.store_scalar(961, 0.0);
            s.store_scalar(962, 0.0);
            s.store_scalar(963, 0.0);
            s.store_scalar(965, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(970, 0.0);
            s.store_scalar(971, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
            s.store_scalar(976, 0.0);
            s.store_scalar(977, 0.0);
            s.store_scalar(978, 0.0);
            s.store_scalar(979, 0.0);
            s.store_scalar(980, 0.0);
            s.store_scalar(981, 0.0);
            s.store_scalar(982, 0.0);
            s.store_scalar(983, 0.0);
            s.store_scalar(984, 0.0);
            s.store_scalar(985, 0.0);
            s.store_scalar(986, 0.0);
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(989, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(993, 0.0);
            s.store_scalar(994, 0.0);
            s.store_scalar(995, 0.0);
            s.store_scalar(996, 0.0);
            s.store_scalar(997, 0.0);
            s.store_scalar(998, 0.0);
            s.store_scalar(999, 0.0);
            s.store_scalar(1000, 0.0);
            s.store_scalar(1001, 0.0);
            s.store_scalar(1002, 0.0);
            s.store_scalar(1003, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1010, 0.0);
            s.store_scalar(1011, 0.0);
            s.store_scalar(1012, 0.0);
            s.store_scalar(1013, 0.0);
            s.store_scalar(1014, 0.0);
            s.store_scalar(1015, 0.0);
            s.store_scalar(1016, 0.0);
            s.store_scalar(1017, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
        }

        if s.b[908] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1017, 919, A::tanh_scaled_input(s.ad_value(919), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1017, 919, p.p53);
                } else {
                    s.store_scalar(1017, 0.0);
                }
            }
        }

        if s.b[908] {
            s.store_sub(1018, 918, 919);
            s.store_mul(952, 938, 926);
            s.store_add_scaled_product_value_ad(954, A::div_scaled_inputs(s.ad_value(934), 1.0, s.ad_value(926), 2.302585092994046), 1.0, 937, 1017, 1.0);
            s.store_add_scaled_product_right_sub(955, 933, 1.0, 944, 924, 925, 1.0);
            s.store_pow_ad(973, A::div(s.ad_value(924), s.ad_value(925)), s.ad_value(946));
        }

        s.b[1021] = (s.v[945] != 0.0);
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1021]) {
            s.store_div_ad_rhs(956, 1017, A::pow(A::offset(A::pow(A::div(s.ad_value(1017), s.ad_value(945)), s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.b[908] && (!s.b[1021])) {
            s.store_scalar(956, 0.0);
        }

        if s.b[908] {
            s.store_mul_add_scaled_product_rhs(953, 1017, s.ad_value(935), 1.0, s.ad_value(956), s.ad_value(936), (-1.0));
            s.store_sub(916, 955, 953);
            s.store_scaled_mul(958, 954, 926, 2.0);
            s.store_mul(959, 929, 958);
            s.store_sub_scaled_inputs(1016, 916, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1015, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1022] = (s.v[1015] > 50.0);
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1022]) {
            s.store_scalar(974, 0.0);
        }

        s.b[1023] = (s.v[1015] < (-50.0));
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[908] && (!s.b[1022])) && s.b[1023]) {
            s.store_scalar(974, 1.0);
        }

        if ((s.b[908] && (!s.b[1022])) && (!s.b[1023])) {
            s.store_div_from_scalar_offset_ad(974, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aai(975, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(974), (-(p.p51 * 0.1))), (-1.0), 958, 1.0);
        }

        s.b[1024] = (s.v[975] > 50.0);
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1024]) {
            s.store_mul(976, 959, 975);
        }

        s.b[1025] = (s.v[975] < (-50.0));
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1024])) && s.b[1025]) {
            s.store_mul_exp_rhs(976, 959, 975);
        }

        if ((s.b[908] && (!s.b[1024])) && (!s.b[1025])) {
            s.store_mul_ln_one_plus_exp_rhs(976, 959, 975);
        }

        if s.b[908] {
            s.store_div_ad_rhs(962, 940, A::mul_offset_rhs(s.ad_value(973), A::div_scaled_product(s.ad_value(942), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(963, 939, A::div_scaled_offset_numerator(A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(948), s.ad_value(1017), 1.0, s.ad_value(928), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(943), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(980, 963, 928, 1.0, 962, 1.0);
            s.store_add_scaled_product_right_ad(981, 980, (-1.0), 980, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(976), 2.0, s.ad_value(929), s.ad_value(980), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(982, A::mul_sub_from_scalar_rhs(s.ad_value(980), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
            s.store_add_scaled_product_value_ad(917, A::mul_sub_from_scalar_rhs(s.ad_value(981), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(983, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::div(s.ad_value(919), s.ad_value(917)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(917))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(919), s.ad_value(917))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(984, 919, 983);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(985, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(986, 919, 985);
            s.store_div_scaled_inputs2_indices(1015, 918, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1026] = (s.v[1015] > 50.0);
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1026]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1027] = (s.v[1015] < (-50.0));
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1026])) && s.b[1027]) {
            s.store_scalar(957, 1.0);
        }

        if ((s.b[908] && (!s.b[1026])) && (!s.b[1027])) {
            s.store_div_from_scalar_offset_ad(957, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(960, 1018, 1.0, 986, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(957), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
        }

        s.b[1028] = (s.v[960] > 50.0);
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1028]) {
            s.store_mul(961, 959, 960);
        }

        s.b[1029] = (s.v[960] < (-50.0));
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1028])) && s.b[1029]) {
            s.store_mul_exp_rhs(961, 959, 960);
        }

        if ((s.b[908] && (!s.b[1028])) && (!s.b[1029])) {
            s.store_mul_ln_one_plus_exp_rhs(961, 959, 960);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1015, 1018, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1030] = (s.v[1015] > 50.0);
        s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1030]) {
            s.store_scalar(987, 0.0);
        }

        s.b[1031] = (s.v[1015] < (-50.0));
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1030])) && s.b[1031]) {
            s.store_scalar(987, 1.0);
        }

        if ((s.b[908] && (!s.b[1030])) && (!s.b[1031])) {
            s.store_div_from_scalar_offset_ad(987, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(988, 918, 1.0, 984, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(987), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
        }

        s.b[1032] = (s.v[988] > 50.0);
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1032]) {
            s.store_mul(989, 959, 988);
        }

        s.b[1033] = (s.v[988] < (-50.0));
        s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1032])) && s.b[1033]) {
            s.store_mul_exp_rhs(989, 959, 988);
        }

        if ((s.b[908] && (!s.b[1032])) && (!s.b[1033])) {
            s.store_mul_ln_one_plus_exp_rhs(989, 959, 988);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(990, 961, 1.0, 989, (-1.0), 929, 1.0);
            s.store_div(1016, 990, 982);
            s.store_div_scaled_inputs_indices(965, 934, 1.0, 926, 2.302585092994046);
            s.store_scaled_mul(967, 965, 926, 2.0);
            s.store_mul(968, 929, 967);
            s.store_sub_scaled_inputs(1020, 955, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1019, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1034] = (s.v[1019] > 50.0);
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1034]) {
            s.store_scalar(977, 0.0);
        }

        s.b[1035] = (s.v[1019] < (-50.0));
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1034])) && s.b[1035]) {
            s.store_scalar(977, 1.0);
        }

        if ((s.b[908] && (!s.b[1034])) && (!s.b[1035])) {
            s.store_div_from_scalar_offset_ad(977, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aai(978, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(977), (-(p.p51 * 0.1))), (-1.0), 967, 1.0);
        }

        s.b[1036] = (s.v[978] > 50.0);
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1036]) {
            s.store_mul(979, 968, 978);
        }

        s.b[1037] = (s.v[978] < (-50.0));
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1036])) && s.b[1037]) {
            s.store_mul_exp_rhs(979, 968, 978);
        }

        if ((s.b[908] && (!s.b[1036])) && (!s.b[1037])) {
            s.store_mul_ln_one_plus_exp_rhs(979, 968, 978);
        }

        if s.b[908] {
            s.store_div(971, 940, 973);
            s.store_mul_div_scaled_offset_numerator_rhs(972, 939, A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0);
            s.store_div_scaled_product_indices(993, 972, 928, 1.0, 971, 1.0);
            s.store_add_scaled_product_right_ad(994, 993, (-1.0), 993, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(979), 2.0, s.ad_value(929), s.ad_value(993), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(995, A::mul_sub_from_scalar_rhs(s.ad_value(994), 1.0, s.ad_value(977)), 1.0, 967, 977, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(996, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::div(s.ad_value(919), s.ad_value(995)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(995))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(919), s.ad_value(995))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(997, 919, 996);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(998, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(999, 919, 998);
            s.store_div_scaled_inputs2_indices(1019, 918, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1038] = (s.v[1019] > 50.0);
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1038]) {
            s.store_scalar(966, 0.0);
        }

        s.b[1039] = (s.v[1019] < (-50.0));
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1038])) && s.b[1039]) {
            s.store_scalar(966, 1.0);
        }

        if ((s.b[908] && (!s.b[1038])) && (!s.b[1039])) {
            s.store_div_from_scalar_offset_ad(966, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(969, 1018, 1.0, 999, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(966), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
        }

        s.b[1040] = (s.v[969] > 50.0);
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1040]) {
            s.store_mul(970, 968, 969);
        }

        s.b[1041] = (s.v[969] < (-50.0));
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1040])) && s.b[1041]) {
            s.store_mul_exp_rhs(970, 968, 969);
        }

        if ((s.b[908] && (!s.b[1040])) && (!s.b[1041])) {
            s.store_mul_ln_one_plus_exp_rhs(970, 968, 969);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1019, 1018, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1042] = (s.v[1019] > 50.0);
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1042]) {
            s.store_scalar(1000, 0.0);
        }

        s.b[1043] = (s.v[1019] < (-50.0));
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(1000, 1.0);
        }

        if ((s.b[908] && (!s.b[1042])) && (!s.b[1043])) {
            s.store_div_from_scalar_offset_ad(1000, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(1001, 918, 1.0, 997, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(1000), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
        }

        s.b[1044] = (s.v[1001] > 50.0);
        s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1044]) {
            s.store_mul(1002, 968, 1001);
        }

        s.b[1045] = (s.v[1001] < (-50.0));
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1044])) && s.b[1045]) {
            s.store_mul_exp_rhs(1002, 968, 1001);
        }

        if ((s.b[908] && (!s.b[1044])) && (!s.b[1045])) {
            s.store_mul_ln_one_plus_exp_rhs(1002, 968, 1001);
        }

        if s.b[908] {
            s.store_offset_square(1003, 970, 1e-38);
            s.store_offset_mul(1004, 1003, 970, 1e-57);
            s.store_offset_square(1005, 1002, 1e-38);
            s.store_offset_mul(1006, 1005, 1002, 1e-57);
            s.store_offset_mul(1007, 970, 1002, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1008, 1003, (2.0 / 3.0), 1005, (2.0 / 3.0), 1007, (2.0 / 3.0), A::offset(A::add(s.ad_value(970), s.ad_value(1002)), 2e-19), 1.0);
            s.store_div_ad(1009, A::add_scaled_inputs_products(s.ad_value(1004), (2.0 * 2.0), s.ad_value(1006), (3.0 * 2.0), s.ad_value(1003), s.ad_value(1002), (4.0 * 2.0), s.ad_value(1005), s.ad_value(970), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1003), 15.0, s.ad_value(1005), 15.0, s.ad_value(1007), (2.0 * 15.0)));
            s.store_sub(1010, 1008, 1009);
            s.copy_ad(1011, 1009);
            s.store_mul_product3_mixed_iaii(911, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), 950, 1010, 1.0);
            s.store_mul_product3_mixed_iaii(912, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), 950, 1011, 1.0);
        }

        s.b[1046] = (s.v[920] == 1.0);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1046]) {
            s.store_div_scaled_inputs3_indices(1012, 921, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

        s.b[1047] = (s.v[1012] > 50.0);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1046]) && s.b[1047]) {
            s.copy_ad(1015, 1012);
        }

        s.b[1048] = (s.v[1012] < (-50.0));
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
            s.store_exp(1015, 1012);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) {
            s.store_ln_one_plus_exp(1015, 1012);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs_mixed_ai(913, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(931), s.ad_value(967)), 1015, 951);
            s.store_div_scaled_inputs3_indices(1013, 922, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1049] = (s.v[1013] > 50.0);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1046]) && s.b[1049]) {
            s.copy_ad(1015, 1013);
        }

        s.b[1050] = (s.v[1013] < (-50.0));
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && s.b[1050]) {
            s.store_exp(1015, 1013);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && (!s.b[1050])) {
            s.store_ln_one_plus_exp(1015, 1013);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs_mixed_ai(914, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(932), s.ad_value(967)), 1015, 951);
        }

        if (s.b[908] && (!s.b[1046])) {
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
        }

        s.b[1051] = (s.v[923] == 1.0);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1051]) {
            s.store_div_scaled_inputs3_indices(1014, 918, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

        s.b[1052] = (s.v[1014] > 50.0);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1051]) && s.b[1052]) {
            s.copy_ad(1015, 1014);
        }

        s.b[1053] = (s.v[1014] < (-50.0));
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && s.b[1053]) {
            s.store_exp(1015, 1014);
        }

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && (!s.b[1053])) {
            s.store_ln_one_plus_exp(1015, 1014);
        }

        if (s.b[908] && s.b[1051]) {
            s.store_mul_ad_product_lhs_mixed_ai(915, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(930), s.ad_value(967)), 1015, 951);
        }

        if (s.b[908] && (!s.b[1051])) {
            s.store_scalar(915, 0.0);
        }

        if s.b[908] {
            s.copy_ad(167, 911);
            s.copy_ad(168, 912);
            s.copy_ad(169, 913);
            s.copy_ad(170, 914);
            s.copy_ad(171, 915);
        }

        s.b[1054] = (p.p78 == 1.0);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        s.store_scalar(173, 0.0);

        s.store_scalar(174, 0.0);

        s.store_scalar(175, 0.0);

        s.store_scalar(176, 0.0);

        s.store_scalar(177, 0.0);

        s.b[1055] = (p.p101 > p.p354);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if s.b[1055] {
            s.store_scalar(1058, 0.0);
            s.store_scalar(1059, 0.0);
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
            s.store_scalar(1062, 0.0);
            s.store_scalar(1063, 0.0);
            s.store_scalar(1064, 0.0);
            s.copy_ad(1065, 66);
            s.copy_ad(1066, 67);
            s.store_scalar(1067, p.p107);
            s.copy_ad(1068, 68);
            s.copy_ad(1069, 69);
            s.store_scalar(1070, p.p105);
            s.copy_ad(1071, 111);
            s.store_scalar(1072, s.v[109]);
            s.copy_ad(1073, 113);
            s.store_scalar(1074, p.p0);
            s.store_scalar(1075, p.p101);
            s.copy_ad(1076, 23);
            s.store_scalar(1077, p.p106);
            s.copy_ad(1078, 24);
            s.copy_ad(1079, 25);
            s.store_scalar(1080, p.p102);
            s.store_scalar(1081, p.p116);
            s.store_scalar(1082, p.p115);
            s.store_scalar(1083, 0.0);
            s.store_scalar(1084, p.p117);
            s.store_scalar(1085, p.p121);
            s.store_scalar(1086, p.p112);
            s.store_scalar(1087, p.p113);
            s.store_scalar(1088, p.p114);
            s.store_scalar(1089, p.p120);
            s.store_scalar(1090, p.p119);
            s.store_scalar(1091, p.p118);
            s.store_scalar(1092, p.p39);
            s.store_scalar(1093, p.p47);
            s.store_scalar(1094, p.p45);
            s.store_scalar(1095, p.p42);
            s.store_scalar(1096, p.p2);
            s.store_scalar(1097, p.p6);
            s.store_scalar(1098, 1.0);
            s.store_scalar(1099, 0.0);
            s.store_scalar(1100, 0.0);
            s.store_scalar(1101, 0.0);
            s.store_scalar(1102, 0.0);
            s.store_scalar(1103, 0.0);
            s.store_scalar(1104, 0.0);
            s.store_scalar(1105, 0.0);
            s.store_scalar(1106, 0.0);
            s.store_scalar(1107, 0.0);
            s.store_scalar(1108, 0.0);
            s.store_scalar(1109, 0.0);
            s.store_scalar(1110, 0.0);
            s.store_scalar(1112, 0.0);
            s.store_scalar(1113, 0.0);
            s.store_scalar(1114, 0.0);
            s.store_scalar(1115, 0.0);
            s.store_scalar(1116, 0.0);
            s.store_scalar(1117, 0.0);
            s.store_scalar(1118, 0.0);
            s.store_scalar(1119, 0.0);
            s.store_scalar(1120, 0.0);
            s.store_scalar(1121, 0.0);
            s.store_scalar(1122, 0.0);
            s.store_scalar(1123, 0.0);
            s.store_scalar(1124, 0.0);
            s.store_scalar(1125, 0.0);
            s.store_scalar(1126, 0.0);
            s.store_scalar(1127, 0.0);
            s.store_scalar(1128, 0.0);
            s.store_scalar(1129, 0.0);
            s.store_scalar(1130, 0.0);
            s.store_scalar(1131, 0.0);
            s.store_scalar(1132, 0.0);
            s.store_scalar(1133, 0.0);
            s.store_scalar(1134, 0.0);
            s.store_scalar(1135, 0.0);
            s.store_scalar(1136, 0.0);
            s.store_scalar(1137, 0.0);
            s.store_scalar(1140, 0.0);
            s.store_scalar(1141, 0.0);
            s.store_scalar(1142, 0.0);
            s.store_scalar(1143, 0.0);
            s.store_scalar(1144, 0.0);
            s.store_scalar(1145, 0.0);
            s.store_scalar(1146, 0.0);
            s.store_scalar(1147, 0.0);
            s.store_scalar(1148, 0.0);
            s.store_scalar(1149, 0.0);
            s.store_scalar(1150, 0.0);
            s.store_scalar(1151, 0.0);
            s.store_scalar(1152, 0.0);
            s.store_scalar(1153, 0.0);
            s.store_scalar(1154, 0.0);
            s.store_scalar(1155, 0.0);
            s.store_scalar(1156, 0.0);
            s.store_scalar(1157, 0.0);
            s.store_scalar(1158, 0.0);
            s.store_scalar(1159, 0.0);
            s.store_scalar(1160, 0.0);
            s.store_scalar(1161, 0.0);
            s.store_scalar(1162, 0.0);
            s.store_scalar(1163, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            s.store_scalar(1164, 0.0);
            s.store_scalar(1165, 0.0);
            s.store_scalar(1166, 0.0);
            s.store_scalar(1167, 0.0);
        }

        if s.b[1055] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1164, 1066, A::tanh_scaled_input(s.ad_value(1066), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1164, 1066, p.p53);
                } else {
                    s.store_scalar(1164, 0.0);
                }
            }
        }

        if s.b[1055] {
            s.store_sub(1165, 1065, 1066);
            s.store_mul(1099, 1085, 1073);
            s.store_add_scaled_product_value_ad(1101, A::div_scaled_inputs(s.ad_value(1081), 1.0, s.ad_value(1073), 2.302585092994046), 1.0, 1084, 1164, 1.0);
            s.store_add_scaled_product_right_sub(1102, 1080, 1.0, 1091, 1071, 1072, 1.0);
            s.store_pow_ad(1120, A::div(s.ad_value(1071), s.ad_value(1072)), s.ad_value(1093));
        }

        s.b[1168] = (s.v[1092] != 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1168]) {
            s.store_div_ad_rhs(1103, 1164, A::pow(A::offset(A::pow(A::div(s.ad_value(1164), s.ad_value(1092)), s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.b[1055] && (!s.b[1168])) {
            s.store_scalar(1103, 0.0);
        }

        if s.b[1055] {
            s.store_mul_add_scaled_product_rhs(1100, 1164, s.ad_value(1082), 1.0, s.ad_value(1103), s.ad_value(1083), (-1.0));
            s.store_sub(1063, 1102, 1100);
            s.store_scaled_mul(1105, 1101, 1073, 2.0);
            s.store_mul(1106, 1076, 1105);
            s.store_sub_scaled_inputs(1163, 1063, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1162, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1169] = (s.v[1162] > 50.0);
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1169]) {
            s.store_scalar(1121, 0.0);
        }

        s.b[1170] = (s.v[1162] < (-50.0));
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1169])) && s.b[1170]) {
            s.store_scalar(1121, 1.0);
        }

        if ((s.b[1055] && (!s.b[1169])) && (!s.b[1170])) {
            s.store_div_from_scalar_offset_ad(1121, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aai(1122, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1121), (-(p.p51 * 0.1))), (-1.0), 1105, 1.0);
        }

        s.b[1171] = (s.v[1122] > 50.0);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1171]) {
            s.store_mul(1123, 1106, 1122);
        }

        s.b[1172] = (s.v[1122] < (-50.0));
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1171])) && s.b[1172]) {
            s.store_mul_exp_rhs(1123, 1106, 1122);
        }

        if ((s.b[1055] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_mul_ln_one_plus_exp_rhs(1123, 1106, 1122);
        }

        if s.b[1055] {
            s.store_div_ad_rhs(1109, 1087, A::mul_offset_rhs(s.ad_value(1120), A::div_scaled_product(s.ad_value(1089), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1110, 1086, A::div_scaled_offset_numerator(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1095), s.ad_value(1164), 1.0, s.ad_value(1075), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1090), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1127, 1110, 1075, 1.0, 1109, 1.0);
            s.store_add_scaled_product_right_ad(1128, 1127, (-1.0), 1127, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1123), 2.0, s.ad_value(1076), s.ad_value(1127), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1129, A::mul_sub_from_scalar_rhs(s.ad_value(1127), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
            s.store_add_scaled_product_value_ad(1064, A::mul_sub_from_scalar_rhs(s.ad_value(1128), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1130, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::div(s.ad_value(1066), s.ad_value(1064)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1131, 1066, 1130);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1132, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1133, 1066, 1132);
            s.store_div_scaled_inputs2_indices(1162, 1065, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1173] = (s.v[1162] > 50.0);
        s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1173]) {
            s.store_scalar(1104, 0.0);
        }

        s.b[1174] = (s.v[1162] < (-50.0));
        s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1173])) && s.b[1174]) {
            s.store_scalar(1104, 1.0);
        }

        if ((s.b[1055] && (!s.b[1173])) && (!s.b[1174])) {
            s.store_div_from_scalar_offset_ad(1104, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1107, 1165, 1.0, 1133, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1104), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
        }

        s.b[1175] = (s.v[1107] > 50.0);
        s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1175]) {
            s.store_mul(1108, 1106, 1107);
        }

        s.b[1176] = (s.v[1107] < (-50.0));
        s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1175])) && s.b[1176]) {
            s.store_mul_exp_rhs(1108, 1106, 1107);
        }

        if ((s.b[1055] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_mul_ln_one_plus_exp_rhs(1108, 1106, 1107);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1162, 1165, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1177] = (s.v[1162] > 50.0);
        s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1177]) {
            s.store_scalar(1134, 0.0);
        }

        s.b[1178] = (s.v[1162] < (-50.0));
        s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1177])) && s.b[1178]) {
            s.store_scalar(1134, 1.0);
        }

        if ((s.b[1055] && (!s.b[1177])) && (!s.b[1178])) {
            s.store_div_from_scalar_offset_ad(1134, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1135, 1065, 1.0, 1131, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1134), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
        }

        s.b[1179] = (s.v[1135] > 50.0);
        s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1179]) {
            s.store_mul(1136, 1106, 1135);
        }

        s.b[1180] = (s.v[1135] < (-50.0));
        s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1179])) && s.b[1180]) {
            s.store_mul_exp_rhs(1136, 1106, 1135);
        }

        if ((s.b[1055] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_mul_ln_one_plus_exp_rhs(1136, 1106, 1135);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1137, 1108, 1.0, 1136, (-1.0), 1076, 1.0);
            s.store_div(1163, 1137, 1129);
            s.store_div_scaled_inputs_indices(1112, 1081, 1.0, 1073, 2.302585092994046);
            s.store_scaled_mul(1114, 1112, 1073, 2.0);
            s.store_mul(1115, 1076, 1114);
            s.store_sub_scaled_inputs(1167, 1102, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1166, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1181] = (s.v[1166] > 50.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1181]) {
            s.store_scalar(1124, 0.0);
        }

        s.b[1182] = (s.v[1166] < (-50.0));
        s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1181])) && s.b[1182]) {
            s.store_scalar(1124, 1.0);
        }

        if ((s.b[1055] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_div_from_scalar_offset_ad(1124, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aai(1125, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1124), (-(p.p51 * 0.1))), (-1.0), 1114, 1.0);
        }

        s.b[1183] = (s.v[1125] > 50.0);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1183]) {
            s.store_mul(1126, 1115, 1125);
        }

        s.b[1184] = (s.v[1125] < (-50.0));
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1183])) && s.b[1184]) {
            s.store_mul_exp_rhs(1126, 1115, 1125);
        }

        if ((s.b[1055] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_mul_ln_one_plus_exp_rhs(1126, 1115, 1125);
        }

        if s.b[1055] {
            s.store_div(1118, 1087, 1120);
            s.store_mul_div_scaled_offset_numerator_rhs(1119, 1086, A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1140, 1119, 1075, 1.0, 1118, 1.0);
            s.store_add_scaled_product_right_ad(1141, 1140, (-1.0), 1140, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1126), 2.0, s.ad_value(1076), s.ad_value(1140), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1142, A::mul_sub_from_scalar_rhs(s.ad_value(1141), 1.0, s.ad_value(1124)), 1.0, 1114, 1124, 1.0);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1143, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::div(s.ad_value(1066), s.ad_value(1142)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1144, 1066, 1143);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1145, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1146, 1066, 1145);
            s.store_div_scaled_inputs2_indices(1166, 1065, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1185] = (s.v[1166] > 50.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1185]) {
            s.store_scalar(1113, 0.0);
        }

        s.b[1186] = (s.v[1166] < (-50.0));
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1185])) && s.b[1186]) {
            s.store_scalar(1113, 1.0);
        }

        if ((s.b[1055] && (!s.b[1185])) && (!s.b[1186])) {
            s.store_div_from_scalar_offset_ad(1113, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1116, 1165, 1.0, 1146, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1113), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
        }

        s.b[1187] = (s.v[1116] > 50.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1187]) {
            s.store_mul(1117, 1115, 1116);
        }

        s.b[1188] = (s.v[1116] < (-50.0));
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1187])) && s.b[1188]) {
            s.store_mul_exp_rhs(1117, 1115, 1116);
        }

        if ((s.b[1055] && (!s.b[1187])) && (!s.b[1188])) {
            s.store_mul_ln_one_plus_exp_rhs(1117, 1115, 1116);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1166, 1165, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1189] = (s.v[1166] > 50.0);
        s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1189]) {
            s.store_scalar(1147, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1190] = (s.v[1166] < (-50.0));
        s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1189])) && s.b[1190]) {
            s.store_scalar(1147, 1.0);
        }

        if ((s.b[1055] && (!s.b[1189])) && (!s.b[1190])) {
            s.store_div_from_scalar_offset_ad(1147, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1148, 1065, 1.0, 1144, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1147), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
        }

        s.b[1191] = (s.v[1148] > 50.0);
        s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1191]) {
            s.store_mul(1149, 1115, 1148);
        }

        s.b[1192] = (s.v[1148] < (-50.0));
        s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1191])) && s.b[1192]) {
            s.store_mul_exp_rhs(1149, 1115, 1148);
        }

        if ((s.b[1055] && (!s.b[1191])) && (!s.b[1192])) {
            s.store_mul_ln_one_plus_exp_rhs(1149, 1115, 1148);
        }

        if s.b[1055] {
            s.store_offset_square(1150, 1117, 1e-38);
            s.store_offset_mul(1151, 1150, 1117, 1e-57);
            s.store_offset_square(1152, 1149, 1e-38);
            s.store_offset_mul(1153, 1152, 1149, 1e-57);
            s.store_offset_mul(1154, 1117, 1149, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1155, 1150, (2.0 / 3.0), 1152, (2.0 / 3.0), 1154, (2.0 / 3.0), A::offset(A::add(s.ad_value(1117), s.ad_value(1149)), 2e-19), 1.0);
            s.store_div_ad(1156, A::add_scaled_inputs_products(s.ad_value(1151), (2.0 * 2.0), s.ad_value(1153), (3.0 * 2.0), s.ad_value(1150), s.ad_value(1149), (4.0 * 2.0), s.ad_value(1152), s.ad_value(1117), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1150), 15.0, s.ad_value(1152), 15.0, s.ad_value(1154), (2.0 * 15.0)));
            s.store_sub(1157, 1155, 1156);
            s.copy_ad(1158, 1156);
            s.store_mul_product3_mixed_iaii(1058, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), 1097, 1157, 1.0);
            s.store_mul_product3_mixed_iaii(1059, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), 1097, 1158, 1.0);
        }

        s.b[1193] = (s.v[1067] == 1.0);
        s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1193]) {
            s.store_div_scaled_inputs3_indices(1159, 1068, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1194] = (s.v[1159] > 50.0);
        s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1193]) && s.b[1194]) {
            s.copy_ad(1162, 1159);
        }

        s.b[1195] = (s.v[1159] < (-50.0));
        s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && s.b[1195]) {
            s.store_exp(1162, 1159);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && (!s.b[1195])) {
            s.store_ln_one_plus_exp(1162, 1159);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs_mixed_ai(1060, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1078), s.ad_value(1114)), 1162, 1098);
            s.store_div_scaled_inputs3_indices(1160, 1069, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1196] = (s.v[1160] > 50.0);
        s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1193]) && s.b[1196]) {
            s.copy_ad(1162, 1160);
        }

        s.b[1197] = (s.v[1160] < (-50.0));
        s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && s.b[1197]) {
            s.store_exp(1162, 1160);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && (!s.b[1197])) {
            s.store_ln_one_plus_exp(1162, 1160);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs_mixed_ai(1061, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1079), s.ad_value(1114)), 1162, 1098);
        }

        if (s.b[1055] && (!s.b[1193])) {
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
        }

        s.b[1198] = (s.v[1070] == 1.0);
        s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1198]) {
            s.store_div_scaled_inputs3_indices(1161, 1065, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1199] = (s.v[1161] > 50.0);
        s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1198]) && s.b[1199]) {
            s.copy_ad(1162, 1161);
        }

        s.b[1200] = (s.v[1161] < (-50.0));
        s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && s.b[1200]) {
            s.store_exp(1162, 1161);
        }

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && (!s.b[1200])) {
            s.store_ln_one_plus_exp(1162, 1161);
        }

        if (s.b[1055] && s.b[1198]) {
            s.store_mul_ad_product_lhs_mixed_ai(1062, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1077), s.ad_value(1114)), 1162, 1098);
        }

        if (s.b[1055] && (!s.b[1198])) {
            s.store_scalar(1062, 0.0);
        }

        if s.b[1055] {
            s.copy_ad(173, 1058);
            s.copy_ad(174, 1059);
            s.copy_ad(175, 1060);
            s.copy_ad(176, 1061);
            s.copy_ad(177, 1062);
        }

        s.b[1201] = (p.p100 == 1.0);
        s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });

        s.store_scalar(179, 0.0);

        s.store_scalar(180, 0.0);

        s.store_scalar(181, 0.0);

        s.store_scalar(182, 0.0);

        s.store_scalar(183, 0.0);

        s.b[1202] = (p.p123 > p.p354);
        s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });

        if s.b[1202] {
            s.store_scalar(1205, 0.0);
            s.store_scalar(1206, 0.0);
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
            s.store_scalar(1209, 0.0);
            s.store_scalar(1210, 0.0);
            s.store_scalar(1211, 0.0);
            s.copy_ad(1212, 72);
            s.copy_ad(1213, 73);
            s.store_scalar(1214, p.p129);
            s.copy_ad(1215, 74);
            s.copy_ad(1216, 75);
            s.store_scalar(1217, p.p127);
            s.copy_ad(1218, 111);
            s.store_scalar(1219, s.v[109]);
            s.copy_ad(1220, 113);
            s.store_scalar(1221, p.p0);
            s.store_scalar(1222, p.p123);
            s.copy_ad(1223, 26);
            s.store_scalar(1224, p.p128);
            s.copy_ad(1225, 27);
            s.copy_ad(1226, 28);
            s.store_scalar(1227, p.p124);
            s.store_scalar(1228, p.p138);
            s.store_scalar(1229, p.p137);
            s.store_scalar(1230, 0.0);
            s.store_scalar(1231, p.p139);
            s.store_scalar(1232, p.p143);
            s.store_scalar(1233, p.p134);
            s.store_scalar(1234, p.p135);
            s.store_scalar(1235, p.p136);
            s.store_scalar(1236, p.p142);
            s.store_scalar(1237, p.p141);
            s.store_scalar(1238, p.p140);
            s.store_scalar(1239, p.p39);
            s.store_scalar(1240, p.p47);
            s.store_scalar(1241, p.p45);
            s.store_scalar(1242, p.p42);
            s.store_scalar(1243, p.p2);
            s.store_scalar(1244, p.p6);
            s.store_scalar(1245, 1.0);
            s.store_scalar(1246, 0.0);
            s.store_scalar(1247, 0.0);
            s.store_scalar(1248, 0.0);
            s.store_scalar(1249, 0.0);
            s.store_scalar(1250, 0.0);
            s.store_scalar(1251, 0.0);
            s.store_scalar(1252, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1254, 0.0);
            s.store_scalar(1255, 0.0);
            s.store_scalar(1256, 0.0);
            s.store_scalar(1257, 0.0);
            s.store_scalar(1259, 0.0);
            s.store_scalar(1260, 0.0);
            s.store_scalar(1261, 0.0);
            s.store_scalar(1262, 0.0);
            s.store_scalar(1263, 0.0);
            s.store_scalar(1264, 0.0);
            s.store_scalar(1265, 0.0);
            s.store_scalar(1266, 0.0);
            s.store_scalar(1267, 0.0);
            s.store_scalar(1268, 0.0);
            s.store_scalar(1269, 0.0);
            s.store_scalar(1270, 0.0);
            s.store_scalar(1271, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1273, 0.0);
            s.store_scalar(1274, 0.0);
            s.store_scalar(1275, 0.0);
            s.store_scalar(1276, 0.0);
            s.store_scalar(1277, 0.0);
            s.store_scalar(1278, 0.0);
            s.store_scalar(1279, 0.0);
            s.store_scalar(1280, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_scalar(1281, 0.0);
            s.store_scalar(1282, 0.0);
            s.store_scalar(1283, 0.0);
            s.store_scalar(1284, 0.0);
            s.store_scalar(1287, 0.0);
            s.store_scalar(1288, 0.0);
            s.store_scalar(1289, 0.0);
            s.store_scalar(1290, 0.0);
            s.store_scalar(1291, 0.0);
            s.store_scalar(1292, 0.0);
            s.store_scalar(1293, 0.0);
            s.store_scalar(1294, 0.0);
            s.store_scalar(1295, 0.0);
            s.store_scalar(1296, 0.0);
            s.store_scalar(1297, 0.0);
            s.store_scalar(1298, 0.0);
            s.store_scalar(1299, 0.0);
            s.store_scalar(1300, 0.0);
            s.store_scalar(1301, 0.0);
            s.store_scalar(1302, 0.0);
            s.store_scalar(1303, 0.0);
            s.store_scalar(1304, 0.0);
            s.store_scalar(1305, 0.0);
            s.store_scalar(1306, 0.0);
            s.store_scalar(1307, 0.0);
            s.store_scalar(1308, 0.0);
            s.store_scalar(1309, 0.0);
            s.store_scalar(1310, 0.0);
            s.store_scalar(1311, 0.0);
            s.store_scalar(1312, 0.0);
            s.store_scalar(1313, 0.0);
            s.store_scalar(1314, 0.0);
        }

        if s.b[1202] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1311, 1213, A::tanh_scaled_input(s.ad_value(1213), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1311, 1213, p.p53);
                } else {
                    s.store_scalar(1311, 0.0);
                }
            }
        }

        if s.b[1202] {
            s.store_sub(1312, 1212, 1213);
            s.store_mul(1246, 1232, 1220);
            s.store_add_scaled_product_value_ad(1248, A::div_scaled_inputs(s.ad_value(1228), 1.0, s.ad_value(1220), 2.302585092994046), 1.0, 1231, 1311, 1.0);
            s.store_add_scaled_product_right_sub(1249, 1227, 1.0, 1238, 1218, 1219, 1.0);
            s.store_pow_ad(1267, A::div(s.ad_value(1218), s.ad_value(1219)), s.ad_value(1240));
        }

        s.b[1315] = (s.v[1239] != 0.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1315]) {
            s.store_div_ad_rhs(1250, 1311, A::pow(A::offset(A::pow(A::div(s.ad_value(1311), s.ad_value(1239)), s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.b[1202] && (!s.b[1315])) {
            s.store_scalar(1250, 0.0);
        }

        if s.b[1202] {
            s.store_mul_add_scaled_product_rhs(1247, 1311, s.ad_value(1229), 1.0, s.ad_value(1250), s.ad_value(1230), (-1.0));
            s.store_sub(1210, 1249, 1247);
            s.store_scaled_mul(1252, 1248, 1220, 2.0);
            s.store_mul(1253, 1223, 1252);
            s.store_sub_scaled_inputs(1310, 1210, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1309, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1316] = (s.v[1309] > 50.0);
        s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1316]) {
            s.store_scalar(1268, 0.0);
        }

        s.b[1317] = (s.v[1309] < (-50.0));
        s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(1268, 1.0);
        }

        if ((s.b[1202] && (!s.b[1316])) && (!s.b[1317])) {
            s.store_div_from_scalar_offset_ad(1268, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aai(1269, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1268), (-(p.p51 * 0.1))), (-1.0), 1252, 1.0);
        }

        s.b[1318] = (s.v[1269] > 50.0);
        s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1318]) {
            s.store_mul(1270, 1253, 1269);
        }

        s.b[1319] = (s.v[1269] < (-50.0));
        s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1318])) && s.b[1319]) {
            s.store_mul_exp_rhs(1270, 1253, 1269);
        }

        if ((s.b[1202] && (!s.b[1318])) && (!s.b[1319])) {
            s.store_mul_ln_one_plus_exp_rhs(1270, 1253, 1269);
        }

        if s.b[1202] {
            s.store_div_ad_rhs(1256, 1234, A::mul_offset_rhs(s.ad_value(1267), A::div_scaled_product(s.ad_value(1236), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1257, 1233, A::div_scaled_offset_numerator(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1242), s.ad_value(1311), 1.0, s.ad_value(1222), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1237), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1274, 1257, 1222, 1.0, 1256, 1.0);
            s.store_add_scaled_product_right_ad(1275, 1274, (-1.0), 1274, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1270), 2.0, s.ad_value(1223), s.ad_value(1274), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1276, A::mul_sub_from_scalar_rhs(s.ad_value(1274), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
            s.store_add_scaled_product_value_ad(1211, A::mul_sub_from_scalar_rhs(s.ad_value(1275), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1277, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::div(s.ad_value(1213), s.ad_value(1211)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1278, 1213, 1277);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1279, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1280, 1213, 1279);
            s.store_div_scaled_inputs2_indices(1309, 1212, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1320] = (s.v[1309] > 50.0);
        s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1320]) {
            s.store_scalar(1251, 0.0);
        }

        s.b[1321] = (s.v[1309] < (-50.0));
        s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1320])) && s.b[1321]) {
            s.store_scalar(1251, 1.0);
        }

        if ((s.b[1202] && (!s.b[1320])) && (!s.b[1321])) {
            s.store_div_from_scalar_offset_ad(1251, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1254, 1312, 1.0, 1280, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1251), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1322] = (s.v[1254] > 50.0);
        s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1322]) {
            s.store_mul(1255, 1253, 1254);
        }

        s.b[1323] = (s.v[1254] < (-50.0));
        s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1322])) && s.b[1323]) {
            s.store_mul_exp_rhs(1255, 1253, 1254);
        }

        if ((s.b[1202] && (!s.b[1322])) && (!s.b[1323])) {
            s.store_mul_ln_one_plus_exp_rhs(1255, 1253, 1254);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1309, 1312, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1324] = (s.v[1309] > 50.0);
        s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1324]) {
            s.store_scalar(1281, 0.0);
        }

        s.b[1325] = (s.v[1309] < (-50.0));
        s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1324])) && s.b[1325]) {
            s.store_scalar(1281, 1.0);
        }

        if ((s.b[1202] && (!s.b[1324])) && (!s.b[1325])) {
            s.store_div_from_scalar_offset_ad(1281, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1282, 1212, 1.0, 1278, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1281), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1326] = (s.v[1282] > 50.0);
        s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1326]) {
            s.store_mul(1283, 1253, 1282);
        }

        s.b[1327] = (s.v[1282] < (-50.0));
        s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1326])) && s.b[1327]) {
            s.store_mul_exp_rhs(1283, 1253, 1282);
        }

        if ((s.b[1202] && (!s.b[1326])) && (!s.b[1327])) {
            s.store_mul_ln_one_plus_exp_rhs(1283, 1253, 1282);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1284, 1255, 1.0, 1283, (-1.0), 1223, 1.0);
            s.store_div(1310, 1284, 1276);
            s.store_div_scaled_inputs_indices(1259, 1228, 1.0, 1220, 2.302585092994046);
            s.store_scaled_mul(1261, 1259, 1220, 2.0);
            s.store_mul(1262, 1223, 1261);
            s.store_sub_scaled_inputs(1314, 1249, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1313, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1328] = (s.v[1313] > 50.0);
        s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1328]) {
            s.store_scalar(1271, 0.0);
        }

        s.b[1329] = (s.v[1313] < (-50.0));
        s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1328])) && s.b[1329]) {
            s.store_scalar(1271, 1.0);
        }

        if ((s.b[1202] && (!s.b[1328])) && (!s.b[1329])) {
            s.store_div_from_scalar_offset_ad(1271, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aai(1272, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1271), (-(p.p51 * 0.1))), (-1.0), 1261, 1.0);
        }

        s.b[1330] = (s.v[1272] > 50.0);
        s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1330]) {
            s.store_mul(1273, 1262, 1272);
        }

        s.b[1331] = (s.v[1272] < (-50.0));
        s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1330])) && s.b[1331]) {
            s.store_mul_exp_rhs(1273, 1262, 1272);
        }

        if ((s.b[1202] && (!s.b[1330])) && (!s.b[1331])) {
            s.store_mul_ln_one_plus_exp_rhs(1273, 1262, 1272);
        }

        if s.b[1202] {
            s.store_div(1265, 1234, 1267);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_mul_div_scaled_offset_numerator_rhs(1266, 1233, A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1287, 1266, 1222, 1.0, 1265, 1.0);
            s.store_add_scaled_product_right_ad(1288, 1287, (-1.0), 1287, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1273), 2.0, s.ad_value(1223), s.ad_value(1287), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1289, A::mul_sub_from_scalar_rhs(s.ad_value(1288), 1.0, s.ad_value(1271)), 1.0, 1261, 1271, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1290, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::div(s.ad_value(1213), s.ad_value(1289)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1291, 1213, 1290);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1292, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1293, 1213, 1292);
            s.store_div_scaled_inputs2_indices(1313, 1212, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1332] = (s.v[1313] > 50.0);
        s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1332]) {
            s.store_scalar(1260, 0.0);
        }

        s.b[1333] = (s.v[1313] < (-50.0));
        s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1332])) && s.b[1333]) {
            s.store_scalar(1260, 1.0);
        }

        if ((s.b[1202] && (!s.b[1332])) && (!s.b[1333])) {
            s.store_div_from_scalar_offset_ad(1260, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1263, 1312, 1.0, 1293, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1260), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1334] = (s.v[1263] > 50.0);
        s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1334]) {
            s.store_mul(1264, 1262, 1263);
        }

        s.b[1335] = (s.v[1263] < (-50.0));
        s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1334])) && s.b[1335]) {
            s.store_mul_exp_rhs(1264, 1262, 1263);
        }

        if ((s.b[1202] && (!s.b[1334])) && (!s.b[1335])) {
            s.store_mul_ln_one_plus_exp_rhs(1264, 1262, 1263);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1313, 1312, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1336] = (s.v[1313] > 50.0);
        s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1336]) {
            s.store_scalar(1294, 0.0);
        }

        s.b[1337] = (s.v[1313] < (-50.0));
        s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1336])) && s.b[1337]) {
            s.store_scalar(1294, 1.0);
        }

        if ((s.b[1202] && (!s.b[1336])) && (!s.b[1337])) {
            s.store_div_from_scalar_offset_ad(1294, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1295, 1212, 1.0, 1291, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1294), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1338] = (s.v[1295] > 50.0);
        s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1338]) {
            s.store_mul(1296, 1262, 1295);
        }

        s.b[1339] = (s.v[1295] < (-50.0));
        s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1338])) && s.b[1339]) {
            s.store_mul_exp_rhs(1296, 1262, 1295);
        }

        if ((s.b[1202] && (!s.b[1338])) && (!s.b[1339])) {
            s.store_mul_ln_one_plus_exp_rhs(1296, 1262, 1295);
        }

        if s.b[1202] {
            s.store_offset_square(1297, 1264, 1e-38);
            s.store_offset_mul(1298, 1297, 1264, 1e-57);
            s.store_offset_square(1299, 1296, 1e-38);
            s.store_offset_mul(1300, 1299, 1296, 1e-57);
            s.store_offset_mul(1301, 1264, 1296, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1302, 1297, (2.0 / 3.0), 1299, (2.0 / 3.0), 1301, (2.0 / 3.0), A::offset(A::add(s.ad_value(1264), s.ad_value(1296)), 2e-19), 1.0);
            s.store_div_ad(1303, A::add_scaled_inputs_products(s.ad_value(1298), (2.0 * 2.0), s.ad_value(1300), (3.0 * 2.0), s.ad_value(1297), s.ad_value(1296), (4.0 * 2.0), s.ad_value(1299), s.ad_value(1264), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1297), 15.0, s.ad_value(1299), 15.0, s.ad_value(1301), (2.0 * 15.0)));
            s.store_sub(1304, 1302, 1303);
            s.copy_ad(1305, 1303);
            s.store_mul_product3_mixed_iaii(1205, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), 1244, 1304, 1.0);
            s.store_mul_product3_mixed_iaii(1206, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), 1244, 1305, 1.0);
        }

        s.b[1340] = (s.v[1214] == 1.0);
        s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1340]) {
            s.store_div_scaled_inputs3_indices(1306, 1215, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1341] = (s.v[1306] > 50.0);
        s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1340]) && s.b[1341]) {
            s.copy_ad(1309, 1306);
        }

        s.b[1342] = (s.v[1306] < (-50.0));
        s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && s.b[1342]) {
            s.store_exp(1309, 1306);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && (!s.b[1342])) {
            s.store_ln_one_plus_exp(1309, 1306);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1225), s.ad_value(1261)), 1309, 1245);
            s.store_div_scaled_inputs3_indices(1307, 1216, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1343] = (s.v[1307] > 50.0);
        s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1340]) && s.b[1343]) {
            s.copy_ad(1309, 1307);
        }

        s.b[1344] = (s.v[1307] < (-50.0));
        s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && s.b[1344]) {
            s.store_exp(1309, 1307);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && (!s.b[1344])) {
            s.store_ln_one_plus_exp(1309, 1307);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs_mixed_ai(1208, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1226), s.ad_value(1261)), 1309, 1245);
        }

        if (s.b[1202] && (!s.b[1340])) {
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
        }

        s.b[1345] = (s.v[1217] == 1.0);
        s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1345]) {
            s.store_div_scaled_inputs3_indices(1308, 1212, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1346] = (s.v[1308] > 50.0);
        s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1345]) && s.b[1346]) {
            s.copy_ad(1309, 1308);
        }

        s.b[1347] = (s.v[1308] < (-50.0));
        s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && s.b[1347]) {
            s.store_exp(1309, 1308);
        }

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_ln_one_plus_exp(1309, 1308);
        }

        if (s.b[1202] && s.b[1345]) {
            s.store_mul_ad_product_lhs_mixed_ai(1209, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1224), s.ad_value(1261)), 1309, 1245);
        }

        if (s.b[1202] && (!s.b[1345])) {
            s.store_scalar(1209, 0.0);
        }

        if s.b[1202] {
            s.copy_ad(179, 1205);
            s.copy_ad(180, 1206);
            s.copy_ad(181, 1207);
            s.copy_ad(182, 1208);
            s.copy_ad(183, 1209);
        }

        s.b[1348] = (p.p122 == 1.0);
        s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });

        s.store_scalar(185, 0.0);

        s.store_scalar(186, 0.0);

        s.store_scalar(187, 0.0);

        s.store_scalar(188, 0.0);

        s.store_scalar(189, 0.0);

        s.b[1349] = (p.p145 > p.p354);
        s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });

        if s.b[1349] {
            s.store_scalar(1352, 0.0);
            s.store_scalar(1353, 0.0);
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1357, 0.0);
            s.store_scalar(1358, 0.0);
            s.copy_ad(1359, 78);
            s.copy_ad(1360, 79);
            s.store_scalar(1361, p.p151);
            s.copy_ad(1362, 80);
            s.copy_ad(1363, 81);
            s.store_scalar(1364, p.p149);
            s.copy_ad(1365, 111);
            s.store_scalar(1366, s.v[109]);
            s.copy_ad(1367, 113);
            s.store_scalar(1368, p.p0);
            s.store_scalar(1369, p.p145);
            s.copy_ad(1370, 29);
            s.store_scalar(1371, p.p150);
            s.copy_ad(1372, 30);
            s.copy_ad(1373, 31);
            s.store_scalar(1374, p.p146);
            s.store_scalar(1375, p.p160);
            s.store_scalar(1376, p.p159);
            s.store_scalar(1377, 0.0);
            s.store_scalar(1378, p.p161);
            s.store_scalar(1379, p.p165);
            s.store_scalar(1380, p.p156);
            s.store_scalar(1381, p.p157);
            s.store_scalar(1382, p.p158);
            s.store_scalar(1383, p.p164);
            s.store_scalar(1384, p.p163);
            s.store_scalar(1385, p.p162);
            s.store_scalar(1386, p.p39);
            s.store_scalar(1387, p.p47);
            s.store_scalar(1388, p.p45);
            s.store_scalar(1389, p.p42);
            s.store_scalar(1390, p.p2);
            s.store_scalar(1391, p.p6);
            s.store_scalar(1392, 1.0);
            s.store_scalar(1393, 0.0);
            s.store_scalar(1394, 0.0);
            s.store_scalar(1395, 0.0);
            s.store_scalar(1396, 0.0);
            s.store_scalar(1397, 0.0);
            s.store_scalar(1398, 0.0);
            s.store_scalar(1399, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1349] {
            s.store_scalar(1400, 0.0);
            s.store_scalar(1401, 0.0);
            s.store_scalar(1402, 0.0);
            s.store_scalar(1403, 0.0);
            s.store_scalar(1404, 0.0);
            s.store_scalar(1406, 0.0);
            s.store_scalar(1407, 0.0);
            s.store_scalar(1408, 0.0);
            s.store_scalar(1409, 0.0);
            s.store_scalar(1410, 0.0);
            s.store_scalar(1411, 0.0);
            s.store_scalar(1412, 0.0);
            s.store_scalar(1413, 0.0);
            s.store_scalar(1414, 0.0);
            s.store_scalar(1415, 0.0);
            s.store_scalar(1416, 0.0);
            s.store_scalar(1417, 0.0);
            s.store_scalar(1418, 0.0);
            s.store_scalar(1419, 0.0);
            s.store_scalar(1420, 0.0);
            s.store_scalar(1421, 0.0);
            s.store_scalar(1422, 0.0);
            s.store_scalar(1423, 0.0);
            s.store_scalar(1424, 0.0);
            s.store_scalar(1425, 0.0);
            s.store_scalar(1426, 0.0);
            s.store_scalar(1427, 0.0);
            s.store_scalar(1428, 0.0);
            s.store_scalar(1429, 0.0);
            s.store_scalar(1430, 0.0);
            s.store_scalar(1431, 0.0);
            s.store_scalar(1434, 0.0);
            s.store_scalar(1435, 0.0);
            s.store_scalar(1436, 0.0);
            s.store_scalar(1437, 0.0);
            s.store_scalar(1438, 0.0);
            s.store_scalar(1439, 0.0);
            s.store_scalar(1440, 0.0);
            s.store_scalar(1441, 0.0);
            s.store_scalar(1442, 0.0);
            s.store_scalar(1443, 0.0);
            s.store_scalar(1444, 0.0);
            s.store_scalar(1445, 0.0);
            s.store_scalar(1446, 0.0);
            s.store_scalar(1447, 0.0);
            s.store_scalar(1448, 0.0);
            s.store_scalar(1449, 0.0);
            s.store_scalar(1450, 0.0);
            s.store_scalar(1451, 0.0);
            s.store_scalar(1452, 0.0);
            s.store_scalar(1453, 0.0);
            s.store_scalar(1454, 0.0);
            s.store_scalar(1455, 0.0);
            s.store_scalar(1456, 0.0);
            s.store_scalar(1457, 0.0);
            s.store_scalar(1458, 0.0);
            s.store_scalar(1459, 0.0);
            s.store_scalar(1460, 0.0);
            s.store_scalar(1461, 0.0);
        }

        if s.b[1349] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1458, 1360, A::tanh_scaled_input(s.ad_value(1360), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1458, 1360, p.p53);
                } else {
                    s.store_scalar(1458, 0.0);
                }
            }
        }

        if s.b[1349] {
            s.store_sub(1459, 1359, 1360);
            s.store_mul(1393, 1379, 1367);
            s.store_add_scaled_product_value_ad(1395, A::div_scaled_inputs(s.ad_value(1375), 1.0, s.ad_value(1367), 2.302585092994046), 1.0, 1378, 1458, 1.0);
            s.store_add_scaled_product_right_sub(1396, 1374, 1.0, 1385, 1365, 1366, 1.0);
            s.store_pow_ad(1414, A::div(s.ad_value(1365), s.ad_value(1366)), s.ad_value(1387));
        }

        s.b[1462] = (s.v[1386] != 0.0);
        s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1462]) {
            s.store_div_ad_rhs(1397, 1458, A::pow(A::offset(A::pow(A::div(s.ad_value(1458), s.ad_value(1386)), s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.b[1349] && (!s.b[1462])) {
            s.store_scalar(1397, 0.0);
        }

        if s.b[1349] {
            s.store_mul_add_scaled_product_rhs(1394, 1458, s.ad_value(1376), 1.0, s.ad_value(1397), s.ad_value(1377), (-1.0));
            s.store_sub(1357, 1396, 1394);
            s.store_scaled_mul(1399, 1395, 1367, 2.0);
            s.store_mul(1400, 1370, 1399);
            s.store_sub_scaled_inputs(1457, 1357, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aii(1456, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1463] = (s.v[1456] > 50.0);
        s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1463]) {
            s.store_scalar(1415, 0.0);
        }

        s.b[1464] = (s.v[1456] < (-50.0));
        s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1463])) && s.b[1464]) {
            s.store_scalar(1415, 1.0);
        }

        if ((s.b[1349] && (!s.b[1463])) && (!s.b[1464])) {
            s.store_div_from_scalar_offset_ad(1415, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aai(1416, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1415), (-(p.p51 * 0.1))), (-1.0), 1399, 1.0);
        }

        s.b[1465] = (s.v[1416] > 50.0);
        s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1465]) {
            s.store_mul(1417, 1400, 1416);
        }

        s.b[1466] = (s.v[1416] < (-50.0));
        s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1465])) && s.b[1466]) {
            s.store_mul_exp_rhs(1417, 1400, 1416);
        }

        if ((s.b[1349] && (!s.b[1465])) && (!s.b[1466])) {
            s.store_mul_ln_one_plus_exp_rhs(1417, 1400, 1416);
        }

        if s.b[1349] {
            s.store_div_ad_rhs(1403, 1381, A::mul_offset_rhs(s.ad_value(1414), A::div_scaled_product(s.ad_value(1383), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1404, 1380, A::div_scaled_offset_numerator(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1389), s.ad_value(1458), 1.0, s.ad_value(1369), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1384), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1421, 1404, 1369, 1.0, 1403, 1.0);
            s.store_add_scaled_product_right_ad(1422, 1421, (-1.0), 1421, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1417), 2.0, s.ad_value(1370), s.ad_value(1421), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1423, A::mul_sub_from_scalar_rhs(s.ad_value(1421), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
            s.store_add_scaled_product_value_ad(1358, A::mul_sub_from_scalar_rhs(s.ad_value(1422), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1424, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::div(s.ad_value(1360), s.ad_value(1358)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1425, 1360, 1424);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1426, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1427, 1360, 1426);
            s.store_div_scaled_inputs2_indices(1456, 1359, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1467] = (s.v[1456] > 50.0);
        s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1467]) {
            s.store_scalar(1398, 0.0);
        }

        s.b[1468] = (s.v[1456] < (-50.0));
        s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1467])) && s.b[1468]) {
            s.store_scalar(1398, 1.0);
        }

        if ((s.b[1349] && (!s.b[1467])) && (!s.b[1468])) {
            s.store_div_from_scalar_offset_ad(1398, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1401, 1459, 1.0, 1427, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1398), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
        }

        s.b[1469] = (s.v[1401] > 50.0);
        s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1469]) {
            s.store_mul(1402, 1400, 1401);
        }

        s.b[1470] = (s.v[1401] < (-50.0));
        s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1469])) && s.b[1470]) {
            s.store_mul_exp_rhs(1402, 1400, 1401);
        }

        if ((s.b[1349] && (!s.b[1469])) && (!s.b[1470])) {
            s.store_mul_ln_one_plus_exp_rhs(1402, 1400, 1401);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1456, 1459, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1471] = (s.v[1456] > 50.0);
        s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1471]) {
            s.store_scalar(1428, 0.0);
        }

        s.b[1472] = (s.v[1456] < (-50.0));
        s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1471])) && s.b[1472]) {
            s.store_scalar(1428, 1.0);
        }

        if ((s.b[1349] && (!s.b[1471])) && (!s.b[1472])) {
            s.store_div_from_scalar_offset_ad(1428, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1429, 1359, 1.0, 1425, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1428), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
        }

        s.b[1473] = (s.v[1429] > 50.0);
        s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1473]) {
            s.store_mul(1430, 1400, 1429);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1474] = (s.v[1429] < (-50.0));
        s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1473])) && s.b[1474]) {
            s.store_mul_exp_rhs(1430, 1400, 1429);
        }

        if ((s.b[1349] && (!s.b[1473])) && (!s.b[1474])) {
            s.store_mul_ln_one_plus_exp_rhs(1430, 1400, 1429);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1431, 1402, 1.0, 1430, (-1.0), 1370, 1.0);
            s.store_div(1457, 1431, 1423);
            s.store_div_scaled_inputs_indices(1406, 1375, 1.0, 1367, 2.302585092994046);
            s.store_scaled_mul(1408, 1406, 1367, 2.0);
            s.store_mul(1409, 1370, 1408);
            s.store_sub_scaled_inputs(1461, 1396, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aii(1460, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1475] = (s.v[1460] > 50.0);
        s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1475]) {
            s.store_scalar(1418, 0.0);
        }

        s.b[1476] = (s.v[1460] < (-50.0));
        s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1475])) && s.b[1476]) {
            s.store_scalar(1418, 1.0);
        }

        if ((s.b[1349] && (!s.b[1475])) && (!s.b[1476])) {
            s.store_div_from_scalar_offset_ad(1418, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aai(1419, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1418), (-(p.p51 * 0.1))), (-1.0), 1408, 1.0);
        }

        s.b[1477] = (s.v[1419] > 50.0);
        s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1477]) {
            s.store_mul(1420, 1409, 1419);
        }

        s.b[1478] = (s.v[1419] < (-50.0));
        s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1477])) && s.b[1478]) {
            s.store_mul_exp_rhs(1420, 1409, 1419);
        }

        if ((s.b[1349] && (!s.b[1477])) && (!s.b[1478])) {
            s.store_mul_ln_one_plus_exp_rhs(1420, 1409, 1419);
        }

        if s.b[1349] {
            s.store_div(1412, 1381, 1414);
            s.store_mul_div_scaled_offset_numerator_rhs(1413, 1380, A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1434, 1413, 1369, 1.0, 1412, 1.0);
            s.store_add_scaled_product_right_ad(1435, 1434, (-1.0), 1434, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1420), 2.0, s.ad_value(1370), s.ad_value(1434), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1436, A::mul_sub_from_scalar_rhs(s.ad_value(1435), 1.0, s.ad_value(1418)), 1.0, 1408, 1418, 1.0);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1437, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::div(s.ad_value(1360), s.ad_value(1436)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1438, 1360, 1437);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1439, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1440, 1360, 1439);
            s.store_div_scaled_inputs2_indices(1460, 1359, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1479] = (s.v[1460] > 50.0);
        s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1479]) {
            s.store_scalar(1407, 0.0);
        }

        s.b[1480] = (s.v[1460] < (-50.0));
        s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1479])) && s.b[1480]) {
            s.store_scalar(1407, 1.0);
        }

        if ((s.b[1349] && (!s.b[1479])) && (!s.b[1480])) {
            s.store_div_from_scalar_offset_ad(1407, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1410, 1459, 1.0, 1440, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1407), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
        }

        s.b[1481] = (s.v[1410] > 50.0);
        s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1481]) {
            s.store_mul(1411, 1409, 1410);
        }

        s.b[1482] = (s.v[1410] < (-50.0));
        s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1481])) && s.b[1482]) {
            s.store_mul_exp_rhs(1411, 1409, 1410);
        }

        if ((s.b[1349] && (!s.b[1481])) && (!s.b[1482])) {
            s.store_mul_ln_one_plus_exp_rhs(1411, 1409, 1410);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1460, 1459, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1483] = (s.v[1460] > 50.0);
        s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1483]) {
            s.store_scalar(1441, 0.0);
        }

        s.b[1484] = (s.v[1460] < (-50.0));
        s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1483])) && s.b[1484]) {
            s.store_scalar(1441, 1.0);
        }

        if ((s.b[1349] && (!s.b[1483])) && (!s.b[1484])) {
            s.store_div_from_scalar_offset_ad(1441, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1442, 1359, 1.0, 1438, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1441), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
        }

        s.b[1485] = (s.v[1442] > 50.0);
        s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1485]) {
            s.store_mul(1443, 1409, 1442);
        }

        s.b[1486] = (s.v[1442] < (-50.0));
        s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1485])) && s.b[1486]) {
            s.store_mul_exp_rhs(1443, 1409, 1442);
        }

        if ((s.b[1349] && (!s.b[1485])) && (!s.b[1486])) {
            s.store_mul_ln_one_plus_exp_rhs(1443, 1409, 1442);
        }

        if s.b[1349] {
            s.store_offset_square(1444, 1411, 1e-38);
            s.store_offset_mul(1445, 1444, 1411, 1e-57);
            s.store_offset_square(1446, 1443, 1e-38);
            s.store_offset_mul(1447, 1446, 1443, 1e-57);
            s.store_offset_mul(1448, 1411, 1443, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1449, 1444, (2.0 / 3.0), 1446, (2.0 / 3.0), 1448, (2.0 / 3.0), A::offset(A::add(s.ad_value(1411), s.ad_value(1443)), 2e-19), 1.0);
            s.store_div_ad(1450, A::add_scaled_inputs_products(s.ad_value(1445), (2.0 * 2.0), s.ad_value(1447), (3.0 * 2.0), s.ad_value(1444), s.ad_value(1443), (4.0 * 2.0), s.ad_value(1446), s.ad_value(1411), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1444), 15.0, s.ad_value(1446), 15.0, s.ad_value(1448), (2.0 * 15.0)));
            s.store_sub(1451, 1449, 1450);
            s.copy_ad(1452, 1450);
            s.store_mul_product3_mixed_iaii(1352, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), 1391, 1451, 1.0);
            s.store_mul_product3_mixed_iaii(1353, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), 1391, 1452, 1.0);
        }

        s.b[1487] = (s.v[1361] == 1.0);
        s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1487]) {
            s.store_div_scaled_inputs3_indices(1453, 1362, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1488] = (s.v[1453] > 50.0);
        s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });

        if ((s.b[1349] && s.b[1487]) && s.b[1488]) {
            s.copy_ad(1456, 1453);
        }

        s.b[1489] = (s.v[1453] < (-50.0));
        s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {
            s.store_exp(1456, 1453);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_ln_one_plus_exp(1456, 1453);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs_mixed_ai(1354, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1372), s.ad_value(1408)), 1456, 1392);
            s.store_div_scaled_inputs3_indices(1454, 1363, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1490] = (s.v[1454] > 50.0);
        s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });

        if ((s.b[1349] && s.b[1487]) && s.b[1490]) {
            s.copy_ad(1456, 1454);
        }

        s.b[1491] = (s.v[1454] < (-50.0));
        s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {
            s.store_exp(1456, 1454);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && (!s.b[1491])) {
            s.store_ln_one_plus_exp(1456, 1454);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs_mixed_ai(1355, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1373), s.ad_value(1408)), 1456, 1392);
        }

        if (s.b[1349] && (!s.b[1487])) {
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
        }

        s.b[1492] = (s.v[1364] == 1.0);
        s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1492]) {
            s.store_div_scaled_inputs3_indices(1455, 1359, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1493] = (s.v[1455] > 50.0);
        s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });

        if ((s.b[1349] && s.b[1492]) && s.b[1493]) {
            s.copy_ad(1456, 1455);
        }

        s.b[1494] = (s.v[1455] < (-50.0));
        s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && s.b[1494]) {
            s.store_exp(1456, 1455);
        }

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && (!s.b[1494])) {
            s.store_ln_one_plus_exp(1456, 1455);
        }

        if (s.b[1349] && s.b[1492]) {
            s.store_mul_ad_product_lhs_mixed_ai(1356, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1371), s.ad_value(1408)), 1456, 1392);
        }

        if (s.b[1349] && (!s.b[1492])) {
            s.store_scalar(1356, 0.0);
        }

        if s.b[1349] {
            s.copy_ad(185, 1352);
            s.copy_ad(186, 1353);
            s.copy_ad(187, 1354);
            s.copy_ad(188, 1355);
            s.copy_ad(189, 1356);
        }

        s.b[1495] = (p.p144 == 1.0);
        s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });

        s.store_scalar(1788, 0.0);

        s.store_scalar(1789, 0.0);

        s.store_scalar(1790, 0.0);

        s.store_scalar(1791, 0.0);

        s.store_scalar(1795, 0.0);

        s.store_scalar(1796, 0.0);

        s.copy_ad(1797, 45);

        s.copy_ad(1798, 44);

        s.store_scalar(1799, 0.0);

        s.store_scalar(1800, 0.0);

        s.store_scalar(1801, 0.0);

        s.store_scalar(1802, 0.0);

        s.copy_ad(1803, 111);

        s.store_scalar(1804, s.v[109]);

        s.copy_ad(1805, 113);

        s.store_scalar(1806, p.p0);

        s.store_scalar(1807, p.p1);

        s.copy_ad(1808, 19);

        s.store_scalar(1812, p.p35);

        s.store_scalar(1813, p.p36);

        s.store_scalar(1814, p.p37);

        s.store_scalar(1815, p.p38);

        s.store_scalar(1816, p.p40);

        s.store_scalar(1817, p.p41);

        s.store_scalar(1818, p.p32);

        s.store_scalar(1819, p.p33);

        s.store_scalar(1820, p.p34);

        s.store_scalar(1821, p.p44);

        s.store_scalar(1822, p.p43);

        s.store_scalar(1823, p.p46);

        s.store_scalar(1824, p.p39);

        s.store_scalar(1825, p.p47);

        s.store_scalar(1826, p.p45);

        s.store_scalar(1827, p.p42);

        s.store_scalar(1828, p.p2);

        s.store_scalar(1829, p.p6);

        s.copy_ad(1830, 230);

        s.store_scalar(1831, 0.0);

        s.store_scalar(1832, 0.0);

        s.store_scalar(1833, 0.0);

        s.store_scalar(1834, 0.0);

        s.store_scalar(1835, 0.0);

        s.store_scalar(1836, 0.0);

        s.store_scalar(1837, 0.0);

        s.store_scalar(1838, 0.0);

        s.store_scalar(1839, 0.0);

        s.store_scalar(1840, 0.0);

        s.store_scalar(1841, 0.0);

        s.store_scalar(1842, 0.0);

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(1843, 0.0);

        s.store_scalar(1844, 0.0);

        s.store_scalar(1845, 0.0);

        s.store_scalar(1846, 0.0);

        s.store_scalar(1847, 0.0);

        s.store_scalar(1848, 0.0);

        s.store_scalar(1849, 0.0);

        s.store_scalar(1850, 0.0);

        s.store_scalar(1851, 0.0);

        s.store_scalar(1852, 0.0);

        s.store_scalar(1853, 0.0);

        s.store_scalar(1854, 0.0);

        s.store_scalar(1855, 0.0);

        s.store_scalar(1856, 0.0);

        s.store_scalar(1857, 0.0);

        s.store_scalar(1858, 0.0);

        s.store_scalar(1859, 0.0);

        s.store_scalar(1860, 0.0);

        s.store_scalar(1861, 0.0);

        s.store_scalar(1862, 0.0);

        s.store_scalar(1863, 0.0);

        s.store_scalar(1864, 0.0);

        s.store_scalar(1865, 0.0);

        s.store_scalar(1866, 0.0);

        s.store_scalar(1867, 0.0);

        s.store_scalar(1868, 0.0);

        s.store_scalar(1869, 0.0);

        s.store_scalar(1870, 0.0);

        s.store_scalar(1871, 0.0);

        s.store_scalar(1872, 0.0);

        s.store_scalar(1873, 0.0);

        s.store_scalar(1874, 0.0);

        s.store_scalar(1875, 0.0);

        s.store_scalar(1876, 0.0);

        s.store_scalar(1877, 0.0);

        s.store_scalar(1878, 0.0);

        s.store_scalar(1879, 0.0);

        s.store_scalar(1880, 0.0);

        s.store_scalar(1881, 0.0);

        s.store_scalar(1882, 0.0);

        s.store_scalar(1883, 0.0);

        s.store_scalar(1884, 0.0);

        s.store_scalar(1885, 0.0);

        s.store_scalar(1886, 0.0);

        s.store_scalar(1887, 0.0);

        s.store_scalar(1888, 0.0);

        s.store_scalar(1889, 0.0);

        s.store_scalar(1890, 0.0);

        s.store_scalar(1891, 0.0);

        s.store_scalar(1892, 0.0);

        s.store_scalar(1893, 0.0);

        s.store_scalar(1894, 0.0);

        s.store_scalar(1895, 0.0);

        s.store_scalar(1896, 0.0);

        s.store_scalar(1897, 0.0);

        s.store_scalar(1898, 0.0);

        s.store_scalar(1899, 0.0);

        if (p.p52 != 0.0) {
            s.store_mul_ad_rhs(1896, 1798, A::tanh_scaled_input(s.ad_value(1798), (0.001 / p.p53)));
        } else {
            if (p.p52 == 0.0) {
                s.store_sqrt_square_offset(1896, 1798, p.p53);
            } else {
                s.store_scalar(1896, 0.0);
            }
        }

        s.store_sub(1897, 1797, 1798);

        s.store_scale(1831, 1805, s.v[1817]);

        s.store_add_scaled_ad_lhs(1833, A::div_from_scalar(s.v[1813], A::scale(s.ad_value(1805), 2.302585092994046)), 1896, s.v[1816]);

        s.store_offset_scaled(1834, 1803, s.v[1823], (((((-s.v[1804])) * (s.v[1823]))) + (s.v[1812])));

        s.store_powf_scaled_input(1852, 1803, 1.0 / (s.v[1804]), s.v[1825]);

        s.b[1900] = (s.v[1824] != 0.0);
        s.store_scalar(1900, if s.b[1900] { 1.0 } else { 0.0 });

        if s.b[1900] {
            s.store_div_ad_rhs(1835, 1896, A::powf(A::offset(A::powf(A::scale(s.ad_value(1896), 1.0 / (s.v[1824])), s.v[1820]), 1.0), (1.0 / s.v[1820])));
        }

        if (!s.b[1900]) {
            s.store_scalar(1835, 0.0);
        }

        s.store_mul_sub_from_scalar_ad_lhs(1832, s.v[1814], A::scale(s.ad_value(1835), s.v[1815]), 1896);

        s.store_sub(1795, 1834, 1832);

        s.store_scaled_mul(1837, 1833, 1805, 2.0);

        s.store_mul(1838, 1808, 1837);

        s.store_sub_scaled_inputs(1895, 1795, 1.0, 1831, (p.p51 * 0.5));

        s.store_div_scaled_inputs2_mixed_aii(1894, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1901] = (s.v[1894] > 50.0);
        s.store_scalar(1901, if s.b[1901] { 1.0 } else { 0.0 });

        if s.b[1901] {
            s.store_scalar(1853, 0.0);
        }

        s.b[1902] = (s.v[1894] < (-50.0));
        s.store_scalar(1902, if s.b[1902] { 1.0 } else { 0.0 });

        if ((!s.b[1901]) && s.b[1902]) {
            s.store_scalar(1853, 1.0);
        }

        if ((!s.b[1901]) && (!s.b[1902])) {
            s.store_div_from_scalar_offset_ad(1853, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs2_mixed_aai(1854, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1853), (-(p.p51 * 0.1))), (-1.0), 1837, 1.0);

        s.b[1903] = (s.v[1854] > 50.0);
        s.store_scalar(1903, if s.b[1903] { 1.0 } else { 0.0 });

        if s.b[1903] {
            s.store_mul(1855, 1838, 1854);
        }

        s.b[1904] = (s.v[1854] < (-50.0));
        s.store_scalar(1904, if s.b[1904] { 1.0 } else { 0.0 });

        if ((!s.b[1903]) && s.b[1904]) {
            s.store_mul_exp_rhs(1855, 1838, 1854);
        }

        if ((!s.b[1903]) && (!s.b[1904])) {
            s.store_mul_ln_one_plus_exp_rhs(1855, 1838, 1854);
        }

        s.store_div_from_scalar_ad(1841, s.v[1819], A::mul_offset_rhs(s.ad_value(1852), A::div_scaled_inputs(s.ad_value(1855), s.v[1821], s.ad_value(1808), 1.0), 1.0));

        s.store_div_scaled_value_by_product(1842, A::scale_offset(s.ad_value(1896), (s.v[1827] * 1.0 / (s.v[1807])), 1.0), (s.v[1818] * (1.0 + (s.v[1826] * s.v[1804]))), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), A::offset(A::div_scaled_inputs(s.ad_value(1855), s.v[1822], s.ad_value(1808), 1.0), 1.0), 1.0);

        s.store_add_ad(1843, A::mul3_scaled_output(s.ad_value(1853), s.ad_value(1805), s.ad_value(1841), (2.0 * 1.0 / (s.v[1807]))), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1853), s.ad_value(1842)));

        s.store_div_scaled_inputs_indices(1859, 1842, s.v[1807], 1841, 1.0);

        s.store_add_scaled_product_right_ad(1860, 1859, (-1.0), 1859, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1855), 2.0, s.ad_value(1808), s.ad_value(1859), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1861, A::mul_sub_from_scalar_rhs(s.ad_value(1859), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

        s.store_add_scaled_product_value_ad(1796, A::mul_sub_from_scalar_rhs(s.ad_value(1860), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

        s.store_div_from_scalar_powf_ad(1862, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::div(s.ad_value(1798), s.ad_value(1796)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1863, 1798, 1862);

        s.store_div_from_scalar_powf_ad(1864, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0)), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1865, 1798, 1864);

        s.store_div_scaled_inputs2_indices(1894, 1797, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1905] = (s.v[1894] > 50.0);
        s.store_scalar(1905, if s.b[1905] { 1.0 } else { 0.0 });

        if s.b[1905] {
            s.store_scalar(1836, 0.0);
        }

        s.b[1906] = (s.v[1894] < (-50.0));
        s.store_scalar(1906, if s.b[1906] { 1.0 } else { 0.0 });

        if ((!s.b[1905]) && s.b[1906]) {
            s.store_scalar(1836, 1.0);
        }

        if ((!s.b[1905]) && (!s.b[1906])) {
            s.store_div_from_scalar_offset_ad(1836, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1839, 1897, 1.0, 1865, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1836), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

        s.b[1907] = (s.v[1839] > 50.0);
        s.store_scalar(1907, if s.b[1907] { 1.0 } else { 0.0 });

        if s.b[1907] {
            s.store_mul(1840, 1838, 1839);
        }

        s.b[1908] = (s.v[1839] < (-50.0));
        s.store_scalar(1908, if s.b[1908] { 1.0 } else { 0.0 });

        if ((!s.b[1907]) && s.b[1908]) {
            s.store_mul_exp_rhs(1840, 1838, 1839);
        }

        if ((!s.b[1907]) && (!s.b[1908])) {
            s.store_mul_ln_one_plus_exp_rhs(1840, 1838, 1839);
        }

        s.store_div_scaled_inputs2_indices(1894, 1897, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1909] = (s.v[1894] > 50.0);
        s.store_scalar(1909, if s.b[1909] { 1.0 } else { 0.0 });

        if s.b[1909] {
            s.store_scalar(1866, 0.0);
        }

        s.b[1910] = (s.v[1894] < (-50.0));
        s.store_scalar(1910, if s.b[1910] { 1.0 } else { 0.0 });

        if ((!s.b[1909]) && s.b[1910]) {
            s.store_scalar(1866, 1.0);
        }

        if ((!s.b[1909]) && (!s.b[1910])) {
            s.store_div_from_scalar_offset_ad(1866, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1867, 1797, 1.0, 1863, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1866), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

        s.b[1911] = (s.v[1867] > 50.0);
        s.store_scalar(1911, if s.b[1911] { 1.0 } else { 0.0 });

        if s.b[1911] {
            s.store_mul(1868, 1838, 1867);
        }

        s.b[1912] = (s.v[1867] < (-50.0));
        s.store_scalar(1912, if s.b[1912] { 1.0 } else { 0.0 });

        if ((!s.b[1911]) && s.b[1912]) {
            s.store_mul_exp_rhs(1868, 1838, 1867);
        }

        if ((!s.b[1911]) && (!s.b[1912])) {
            s.store_mul_ln_one_plus_exp_rhs(1868, 1838, 1867);
        }

        s.store_div_scaled_inputs2_indices(1869, 1840, 1.0, 1868, (-1.0), 1808, 1.0);

        s.store_div(1895, 1869, 1861);

        s.store_div_ad_rhs(1870, 1895, A::powf(A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::mul(s.ad_value(1895), A::tanh_scaled_input(s.ad_value(1895), (0.001 / p.p53)))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::sqrt_square_offset(s.ad_value(1895), p.p53)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul(1871, 1843, 1870);

        s.store_mul_ad_affine_product_lhs(1789, A::add(s.ad_value(1840), s.ad_value(1868)), s.ad_value(1871), (((s.v[1829] * s.v[1806]) * s.v[1828]) * 0.5), 0.0, 1830);

        s.store_div_from_scalar_scaled_input(1844, s.v[1813], 1805, 2.302585092994046);

        s.store_scaled_mul(1846, 1844, 1805, 2.0);

        s.store_mul(1847, 1808, 1846);

        s.store_sub_scaled_inputs(1899, 1834, 1.0, 1831, (p.p51 * 0.5));

        s.store_div_scaled_inputs2_mixed_aii(1898, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1913] = (s.v[1898] > 50.0);
        s.store_scalar(1913, if s.b[1913] { 1.0 } else { 0.0 });

        if s.b[1913] {
            s.store_scalar(1856, 0.0);
        }

        s.b[1914] = (s.v[1898] < (-50.0));
        s.store_scalar(1914, if s.b[1914] { 1.0 } else { 0.0 });

        if ((!s.b[1913]) && s.b[1914]) {
            s.store_scalar(1856, 1.0);
        }

        if ((!s.b[1913]) && (!s.b[1914])) {
            s.store_div_from_scalar_offset_ad(1856, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs2_mixed_aai(1857, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1856), (-(p.p51 * 0.1))), (-1.0), 1846, 1.0);

        s.b[1915] = (s.v[1857] > 50.0);
        s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });

        if s.b[1915] {
            s.store_mul(1858, 1847, 1857);
        }

        s.b[1916] = (s.v[1857] < (-50.0));
        s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });

        if ((!s.b[1915]) && s.b[1916]) {
            s.store_mul_exp_rhs(1858, 1847, 1857);
        }

        if ((!s.b[1915]) && (!s.b[1916])) {
            s.store_mul_ln_one_plus_exp_rhs(1858, 1847, 1857);
        }

        s.store_div_from_scalar(1850, s.v[1819], 1852);

        s.store_scaled_div_from_scalar_ad(1851, (1.0 + (s.v[1826] * s.v[1804])), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), s.v[1818]);

        s.store_div_scaled_inputs_indices(1872, 1851, s.v[1807], 1850, 1.0);

        s.store_add_scaled_product_right_ad(1873, 1872, (-1.0), 1872, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1858), 2.0, s.ad_value(1808), s.ad_value(1872), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1874, A::mul_sub_from_scalar_rhs(s.ad_value(1873), 1.0, s.ad_value(1856)), 1.0, 1846, 1856, 1.0);

        s.store_div_from_scalar_powf_ad(1875, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::div(s.ad_value(1798), s.ad_value(1874)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1876, 1798, 1875);

        s.store_div_from_scalar_powf_ad(1877, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0)), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1878, 1798, 1877);

        s.store_div_scaled_inputs2_indices(1898, 1797, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1917] = (s.v[1898] > 50.0);
        s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });

        if s.b[1917] {
            s.store_scalar(1845, 0.0);
        }

        s.b[1918] = (s.v[1898] < (-50.0));
        s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });

        if ((!s.b[1917]) && s.b[1918]) {
            s.store_scalar(1845, 1.0);
        }

        if ((!s.b[1917]) && (!s.b[1918])) {
            s.store_div_from_scalar_offset_ad(1845, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1848, 1897, 1.0, 1878, (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1845), (-(p.p51 * 0.1))), -1.0, 1846, 1.0);

        s.b[1919] = (s.v[1848] > 50.0);
        s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });

        if s.b[1919] {
            s.store_mul(1849, 1847, 1848);
        }

    }
}
