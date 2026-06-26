#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1467] {
            s.store_scalar(12, (s.v[47] * p.p77));
        }

        s.b[1468] = (((s.v[559] <= 0.0) || (s.v[417] <= 0.0)) || (s.v[561] < 0.0));
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if (s.b[1467] && s.b[1468]) {
            s.store_scalar(18, 0.0);
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_ad_value(13, A::div_scaled_inputs3(s.ad_value(54), -1.0, s.ad_value(562), (-1.0), s.ad_value(63), 1.0, s.ad_value(12), 1.0));
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_ad_value(13, {
                if (!(s.v[13] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(13), 0.5, A::sqrt(A::offset(A::square(s.ad_value(13)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[13] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(13))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_ad_value(14, A::div_scaled_value_offset_denominator(s.ad_value(417), 1.0, s.ad_value(13), 0.001, 1.0));
        }

        s.b[1469] = (s.v[561] != 0.0);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if ((s.b[1467] && (!s.b[1468])) && s.b[1469]) {
            s.store_mul_square_lhs(15, 48, 48);
            s.store_offset_add_ad(16, s.ad_value(561), A::abs(s.ad_value(15)), 0.0001);
        }

        if ((s.b[1467] && (!s.b[1468])) && s.b[1469]) {
            let assign24440_ad_e33600: A = {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(15), s.ad_value(16)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(15), s.ad_value(16)), A::div(s.ad_value(15), s.ad_value(16))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(17, assign24440_ad_e33600, (-1e-6));
        }

        if ((s.b[1467] && (!s.b[1468])) && (!s.b[1469])) {
            s.store_scalar(17, 1.0);
        }

        if (s.b[1467] && (!s.b[1468])) {
            s.store_mul_ad_lhs(18, A::mul3_scaled_output(s.ad_value(559), s.ad_value(13), A::limited_exp_scaled_input(s.ad_value(14), -1.0), s.v[29]), 17);
        }

        if s.b[1467] {
            s.copy_ad(179, 18);
        }

        s.b[1470] = (((s.v[563] <= 0.0) || (s.v[418] <= 0.0)) || (s.v[565] < 0.0));
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if (s.b[1467] && s.b[1470]) {
            s.store_scalar(18, 0.0);
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_ad_value(13, A::div_scaled_inputs3(s.ad_value(52), -1.0, s.ad_value(566), (-1.0), s.ad_value(63), 1.0, s.ad_value(12), 1.0));
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_ad_value(13, {
                if (!(s.v[13] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(13), 0.5, A::sqrt(A::offset(A::square(s.ad_value(13)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[13] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(13))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_ad_value(14, A::div_scaled_value_offset_denominator(s.ad_value(418), 1.0, s.ad_value(13), 0.001, 1.0));
        }

        s.b[1471] = (s.v[565] != 0.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if ((s.b[1467] && (!s.b[1470])) && s.b[1471]) {
            s.store_mul_square_lhs(15, 50, 50);
            s.store_offset_add_ad(16, s.ad_value(565), A::abs(s.ad_value(15)), 0.0001);
        }

        if ((s.b[1467] && (!s.b[1470])) && s.b[1471]) {
            let assign24560_ad_e33803: A = {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(15), s.ad_value(16)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(15), s.ad_value(16)), A::div(s.ad_value(15), s.ad_value(16))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(17, assign24560_ad_e33803, (-1e-6));
        }

        if ((s.b[1467] && (!s.b[1470])) && (!s.b[1471])) {
            s.store_scalar(17, 1.0);
        }

        if (s.b[1467] && (!s.b[1470])) {
            s.store_mul_ad_lhs(18, A::mul3_scaled_output(s.ad_value(563), s.ad_value(13), A::limited_exp_scaled_input(s.ad_value(14), -1.0), s.v[29]), 17);
        }

        if s.b[1467] {
            s.copy_ad(180, 18);
        }

        s.store_scaled_mul(825, 187, 179, (p.p28 * p.p2));

        s.store_scaled_mul(826, 187, 180, (p.p28 * p.p2));

        s.store_div(12, 306, 343);

        s.store_offset_limited_exp(13, 12, (-1.0));

        s.store_ad_value(14, A::add_scaled_product(s.ad_value(346), 1.0, s.ad_value(345), A::sub(s.ad_value(306), s.ad_value(347)), 1.0));

        s.store_mul(15, 13, 14);

        s.store_ad_value(13, A::div_scaled_offset_numerator(s.ad_value(306), 1.0, p.p731, s.ad_value(343), 1.0));

        s.store_limited_exp_neg_input(14, 13);

        s.store_mul_ad_rhs(16, 341, A::add_scaled_inputs3_offset(A::limited_exp(s.ad_value(12)), 1.0, s.ad_value(351), 1.0, s.ad_value(14), (-p.p733), (-1.0)));

        s.store_ad_value(17, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(348), A::sub(s.ad_value(306), s.ad_value(350)), 1.0));

        s.b[1472] = (s.v[341] > 0.0);
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if s.b[1472] {
            s.store_ad_value(18, A::add_scaled_offset_product_rhs(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(306), 1.0, s.ad_value(347), (-1.0), s.ad_value(343), 1.0)), 1.0 / (2.0)), 1.0, s.ad_value(16), A::tanh(A::div_scaled_inputs2(s.ad_value(306), 1.0, s.ad_value(347), (-1.0), s.ad_value(343), 1.0)), 1.0, 1.0 / (2.0)));
            s.store_ad_value(303, A::add_scaled_offset_product_rhs(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(18), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(306), 1.0, s.ad_value(350), (-1.0), s.ad_value(343), 1.0)), 1.0 / (2.0)), 1.0, s.ad_value(17), A::tanh(A::div_scaled_inputs2(s.ad_value(306), 1.0, s.ad_value(350), (-1.0), s.ad_value(343), 1.0)), 1.0, 1.0 / (2.0)));
        }

        if (!s.b[1472]) {
            s.store_scalar(303, 0.0);
        }

        s.b[1473] = (s.v[441] > 0.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        s.b[1474] = ((p.p748 - s.v[306]) < (p.p748 * 0.001));
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if (s.b[1473] && s.b[1474]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(447), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_sub_ad_rhs(303, 303, A::mul3(s.ad_value(250), s.ad_value(441), s.ad_value(13)));
        }

        if (s.b[1473] && (!s.b[1474])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(447), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p748, A::sub_from_scalar(p.p748, s.ad_value(306)), 1.0), (-1.0));
            s.store_sub_ad_rhs(303, 303, A::mul3(s.ad_value(250), s.ad_value(441), s.ad_value(13)));
        }

        s.b[1475] = (s.v[443] > 0.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        s.b[1476] = ((p.p750 - s.v[306]) < (p.p750 * 0.001));
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if (s.b[1475] && s.b[1476]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(449), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_sub_ad_rhs(303, 303, A::mul3(s.ad_value(300), s.ad_value(443), s.ad_value(13)));
        }

        if (s.b[1475] && (!s.b[1476])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(449), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p750, A::sub_from_scalar(p.p750, s.ad_value(306)), 1.0), (-1.0));
            s.store_sub_ad_rhs(303, 303, A::mul3(s.ad_value(300), s.ad_value(443), s.ad_value(13)));
        }

        s.b[1477] = (s.v[445] > 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        s.b[1478] = ((p.p752 - s.v[306]) < (p.p752 * 0.001));
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if (s.b[1477] && s.b[1478]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(451), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_ad_value(303, A::add_scaled_product(s.ad_value(303), 1.0, s.ad_value(445), s.ad_value(13), (-(s.v[35] * p.p2))));
        }

        if (s.b[1477] && (!s.b[1478])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(306), -1.0, s.ad_value(394), s.ad_value(451), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p752, A::sub_from_scalar(p.p752, s.ad_value(306)), 1.0), (-1.0));
            s.store_ad_value(303, A::add_scaled_product(s.ad_value(303), 1.0, s.ad_value(445), s.ad_value(13), (-(s.v[35] * p.p2))));
        }

        s.store_div(12, 307, 344);

        s.store_offset_limited_exp(13, 12, (-1.0));

        s.store_ad_value(14, A::add_scaled_product(s.ad_value(353), 1.0, s.ad_value(352), A::sub(s.ad_value(307), s.ad_value(354)), 1.0));

        s.store_mul3_lhs(15, 302, 13, 14);

        s.store_ad_value(13, A::div_scaled_offset_numerator(s.ad_value(307), 1.0, p.p732, s.ad_value(344), 1.0));

        s.store_limited_exp_neg_input(14, 13);

        s.store_mul_ad_product_rhs(16, 302, s.ad_value(342), A::add_scaled_inputs3_offset(A::limited_exp(s.ad_value(12)), 1.0, s.ad_value(358), 1.0, s.ad_value(14), (-p.p734), (-1.0)));

        s.store_mul_ad_rhs(17, 302, A::add_scaled_product(s.ad_value(356), 1.0, s.ad_value(355), A::sub(s.ad_value(307), s.ad_value(357)), 1.0));

        s.b[1479] = (s.v[342] > 0.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        s.b[1480] = (s.v[302] > 0.0);
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if (s.b[1479] && s.b[1480]) {
            s.store_ad_value(18, A::add_scaled_offset_product_rhs(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(307), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0 / (2.0)), 1.0, s.ad_value(16), A::tanh(A::div_scaled_inputs2(s.ad_value(307), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0, 1.0 / (2.0)));
            s.store_ad_value(304, A::add_scaled_offset_product_rhs(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(18), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(307), 1.0, s.ad_value(357), (-1.0), s.ad_value(344), 1.0)), 1.0 / (2.0)), 1.0, s.ad_value(17), A::tanh(A::div_scaled_inputs2(s.ad_value(307), 1.0, s.ad_value(357), (-1.0), s.ad_value(344), 1.0)), 1.0, 1.0 / (2.0)));
        }

        if (s.b[1479] && (!s.b[1480])) {
            s.store_scalar(304, 0.0);
        }

        s.b[1481] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if (s.b[1479] && s.b[1481]) {
            s.store_div(12, 309, 344);
            s.store_offset_limited_exp(13, 12, (-1.0));
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(353), 1.0, s.ad_value(352), A::sub(s.ad_value(309), s.ad_value(354)), 1.0));
            s.store_scaled_mul(15, 13, 14, p.p1128);
            s.store_ad_value(13, A::div_scaled_offset_numerator(s.ad_value(309), 1.0, p.p732, s.ad_value(344), 1.0));
            s.store_limited_exp_neg_input(14, 13);
            s.store_mul_scaled_ad_rhs(16, 342, p.p1128, A::add_scaled_inputs3_offset(A::limited_exp(s.ad_value(12)), 1.0, s.ad_value(358), 1.0, s.ad_value(14), (-p.p734), (-1.0)));
            s.store_ad_value(17, A::add_scaled_product(s.ad_value(356), p.p1128, s.ad_value(355), A::sub(s.ad_value(309), s.ad_value(357)), p.p1128));
            s.store_ad_value(18, A::add_scaled_offset_product_rhs(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(15), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(309), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0 / (2.0)), 1.0, s.ad_value(16), A::tanh(A::div_scaled_inputs2(s.ad_value(309), 1.0, s.ad_value(354), (-1.0), s.ad_value(344), 1.0)), 1.0, 1.0 / (2.0)));
            s.store_ad_value(305, A::add_scaled_offset_product_rhs(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(18), 1.0, A::tanh(A::div_scaled_inputs2(s.ad_value(309), 1.0, s.ad_value(357), (-1.0), s.ad_value(344), 1.0)), 1.0 / (2.0)), 1.0, s.ad_value(17), A::tanh(A::div_scaled_inputs2(s.ad_value(309), 1.0, s.ad_value(357), (-1.0), s.ad_value(344), 1.0)), 1.0, 1.0 / (2.0)));
        }

        if (s.b[1479] && (!s.b[1481])) {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[1479]) {
            s.store_scalar(304, 0.0);
            s.store_scalar(305, 0.0);
        }

        s.b[1482] = (s.v[442] > 0.0);
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        s.b[1483] = ((p.p749 - s.v[307]) < (p.p749 * 0.001));
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if (s.b[1482] && s.b[1483]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(448), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_ad_value(304, A::add_scaled_product(s.ad_value(304), 1.0, A::mul3(s.ad_value(302), s.ad_value(251), s.ad_value(442)), s.ad_value(13), (-1.0)));
        }

        if (s.b[1482] && (!s.b[1483])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(448), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p749, A::sub_from_scalar(p.p749, s.ad_value(307)), 1.0), (-1.0));
            s.store_ad_value(304, A::add_scaled_product(s.ad_value(304), 1.0, A::mul3(s.ad_value(302), s.ad_value(251), s.ad_value(442)), s.ad_value(13), (-1.0)));
        }

        s.b[1484] = (s.v[444] > 0.0);
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        s.b[1485] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        s.b[1486] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((s.b[1484] && s.b[1485]) && s.b[1486]) {
            s.store_mul_ad_product_lhs(14, s.ad_value(302), A::offset(s.ad_value(301), (-(s.v[35] * p.p2))), 444);
        }

        if ((s.b[1484] && s.b[1485]) && (!s.b[1486])) {
            s.store_mul3_lhs(14, 302, 301, 444);
        }

        if (s.b[1484] && (!s.b[1485])) {
            s.store_mul3_lhs(14, 302, 301, 444);
        }

        s.b[1487] = ((p.p751 - s.v[307]) < (p.p751 * 0.001));
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1487]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(450), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_ad_value(304, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(14), s.ad_value(13), (-1.0)));
        }

        if (s.b[1484] && (!s.b[1487])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(450), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p751, A::sub_from_scalar(p.p751, s.ad_value(307)), 1.0), (-1.0));
            s.store_ad_value(304, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(14), s.ad_value(13), (-1.0)));
        }

        s.b[1488] = (s.v[446] > 0.0);
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        s.b[1489] = ((p.p753 - s.v[307]) < (p.p753 * 0.001));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if (s.b[1488] && s.b[1489]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(452), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_ad_value(304, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(446), s.ad_value(13), (-(s.v[35] * p.p2))));
        }

        if (s.b[1488] && (!s.b[1489])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(307), -1.0, s.ad_value(394), s.ad_value(452), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p753, A::sub_from_scalar(p.p753, s.ad_value(307)), 1.0), (-1.0));
            s.store_ad_value(304, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(446), s.ad_value(13), (-(s.v[35] * p.p2))));
        }

        s.b[1490] = (p.p1128 > 0.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        s.b[1491] = (s.v[442] > 0.0);
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        s.b[1492] = ((p.p749 - s.v[309]) < (p.p749 * 0.001));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if ((s.b[1490] && s.b[1491]) && s.b[1492]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(448), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_sub_ad_rhs(305, 305, A::mul3_scaled_output(s.ad_value(251), s.ad_value(442), s.ad_value(13), p.p1128));
        }

        if ((s.b[1490] && s.b[1491]) && (!s.b[1492])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(448), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p749, A::sub_from_scalar(p.p749, s.ad_value(309)), 1.0), (-1.0));
            s.store_sub_ad_rhs(305, 305, A::mul3_scaled_output(s.ad_value(251), s.ad_value(442), s.ad_value(13), p.p1128));
        }

        s.b[1493] = (s.v[444] > 0.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        s.b[1494] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if ((s.b[1490] && s.b[1493]) && s.b[1494]) {
            s.store_mul_ad_lhs(14, A::scale_offset(s.ad_value(301), p.p1128, (((((-(s.v[35] * p.p2))) * (p.p1128))) + ((s.v[35] * p.p2)))), 444);
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1490] && s.b[1493]) && (!s.b[1494])) {
            s.store_scaled_mul(14, 301, 444, p.p1128);
        }

        s.b[1495] = ((p.p751 - s.v[309]) < (p.p751 * 0.001));
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if ((s.b[1490] && s.b[1493]) && s.b[1495]) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(450), 1.0));
            s.store_offset_ad(13, A::limited_exp_scaled_input(s.ad_value(12), 1000.0), (-1.0));
            s.store_ad_value(305, A::add_scaled_product(s.ad_value(305), 1.0, s.ad_value(14), s.ad_value(13), (-1.0)));
        }

        if ((s.b[1490] && s.b[1493]) && (!s.b[1495])) {
            s.store_ad_value(12, A::div_scaled_value_by_product(s.ad_value(309), -1.0, s.ad_value(394), s.ad_value(450), 1.0));
            s.store_offset_limited_exp_ad(13, A::div_scaled_inputs(s.ad_value(12), p.p751, A::sub_from_scalar(p.p751, s.ad_value(309)), 1.0), (-1.0));
            s.store_ad_value(305, A::add_scaled_product(s.ad_value(305), 1.0, s.ad_value(14), s.ad_value(13), (-1.0)));
        }

        s.store_mul(312, 423, 250);

        s.store_mul(315, 424, 300);

        s.store_scale(318, 428, (s.v[35] * p.p2));

        s.v[313] = ((0.1) as f64).powf((-p.p713));

        s.b[1496] = (p.p713 == 1.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if s.b[1496] {
            s.store_scalar(314, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1496]) {
            s.store_scalar(314, ((1.0 / (1.0 - p.p713)) * (1.0 - (((0.05 * p.p713) * (1.0 + p.p713)) * s.v[313]))));
        }

        s.v[316] = ((0.1) as f64).powf((-p.p715));

        s.b[1497] = (p.p715 == 1.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if s.b[1497] {
            s.store_scalar(317, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1497]) {
            s.store_scalar(317, ((1.0 / (1.0 - p.p715)) * (1.0 - (((0.05 * p.p715) * (1.0 + p.p715)) * s.v[316]))));
        }

        s.v[319] = ((0.1) as f64).powf((-p.p717));

        s.b[1498] = (p.p717 == 1.0);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if s.b[1498] {
            s.store_scalar(320, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1498]) {
            s.store_scalar(320, ((1.0 / (1.0 - p.p717)) * (1.0 - (((0.05 * p.p717) * (1.0 + p.p717)) * s.v[319]))));
        }

        s.b[1499] = (s.v[312] > 0.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if s.b[1499] {
            s.store_div(13, 306, 429);
        }

        s.b[1500] = (s.v[13] < 0.9);
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if (s.b[1499] && s.b[1500]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1501] = (p.p713 != 1.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        s.b[1502] = (p.p713 == 0.5);
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if (((s.b[1499] && s.b[1500]) && s.b[1501]) && s.b[1502]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1499] && s.b[1500]) && s.b[1501]) && (!s.b[1502])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p713)));
        }

        if ((s.b[1499] && s.b[1500]) && s.b[1501]) {
            s.store_mul_ad_affine_product_rhs(331, 429, s.ad_value(312), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p713)), 0.0);
        }

        if ((s.b[1499] && s.b[1500]) && (!s.b[1501])) {
            s.store_mul_ad_affine_product_rhs(331, 429, s.ad_value(312), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1499] && (!s.b[1500])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p713), (((((-1.0)) * ((5.0 * p.p713)))) + ((1.0 + p.p713)))), s.v[313]);
            s.store_mul_ad_product_rhs(331, 429, s.ad_value(312), A::add(s.ad_value(14), s.ad_value(314)));
        }

        if (!s.b[1499]) {
            s.store_scalar(331, 0.0);
        }

        s.b[1503] = (s.v[315] > 0.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if s.b[1503] {
            s.store_div(13, 306, 430);
        }

        s.b[1504] = (s.v[13] < 0.9);
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if (s.b[1503] && s.b[1504]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1505] = (p.p715 != 1.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        s.b[1506] = (p.p715 == 0.5);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if (((s.b[1503] && s.b[1504]) && s.b[1505]) && s.b[1506]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1503] && s.b[1504]) && s.b[1505]) && (!s.b[1506])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p715)));
        }

        if ((s.b[1503] && s.b[1504]) && s.b[1505]) {
            s.store_mul_ad_affine_product_rhs(332, 430, s.ad_value(315), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p715)), 0.0);
        }

        if ((s.b[1503] && s.b[1504]) && (!s.b[1505])) {
            s.store_mul_ad_affine_product_rhs(332, 430, s.ad_value(315), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1503] && (!s.b[1504])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p715), (((((-1.0)) * ((5.0 * p.p715)))) + ((1.0 + p.p715)))), s.v[316]);
            s.store_mul_ad_product_rhs(332, 430, s.ad_value(315), A::add(s.ad_value(14), s.ad_value(317)));
        }

        if (!s.b[1503]) {
            s.store_scalar(332, 0.0);
        }

        s.b[1507] = (s.v[318] > 0.0);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if s.b[1507] {
            s.store_div(13, 306, 431);
        }

        s.b[1508] = (s.v[13] < 0.9);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if (s.b[1507] && s.b[1508]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1509] = (p.p717 != 1.0);
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        s.b[1510] = (p.p717 == 0.5);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if (((s.b[1507] && s.b[1508]) && s.b[1509]) && s.b[1510]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1507] && s.b[1508]) && s.b[1509]) && (!s.b[1510])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p717)));
        }

        if ((s.b[1507] && s.b[1508]) && s.b[1509]) {
            s.store_mul_ad_affine_product_rhs(333, 431, s.ad_value(318), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p717)), 0.0);
        }

        if ((s.b[1507] && s.b[1508]) && (!s.b[1509])) {
            s.store_mul_ad_affine_product_rhs(333, 431, s.ad_value(318), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1507] && (!s.b[1508])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p717), (((((-1.0)) * ((5.0 * p.p717)))) + ((1.0 + p.p717)))), s.v[319]);
            s.store_mul_ad_product_rhs(333, 431, s.ad_value(318), A::add(s.ad_value(14), s.ad_value(320)));
        }

        if (!s.b[1507]) {
            s.store_scalar(333, 0.0);
        }

        s.store_ad_value(330, A::add_scaled_inputs3(s.ad_value(331), 1.0, s.ad_value(332), 1.0, s.ad_value(333), 1.0));

        s.store_mul3_lhs(321, 302, 426, 251);

        s.b[1511] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        s.b[1512] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if (s.b[1511] && s.b[1512]) {
            s.store_mul_ad_product_rhs(324, 302, s.ad_value(427), A::offset(s.ad_value(301), (-(s.v[35] * p.p2))));
        }

        if (s.b[1511] && (!s.b[1512])) {
            s.store_mul3_lhs(324, 302, 427, 301);
        }

        if (!s.b[1511]) {
            s.store_mul3_lhs(324, 302, 427, 301);
        }

        s.store_scale(327, 425, (s.v[35] * p.p2));

        s.v[322] = ((0.1) as f64).powf((-p.p714));

        s.b[1513] = (p.p714 == 1.0);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if s.b[1513] {
            s.store_scalar(323, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1513]) {
            s.store_scalar(323, ((1.0 / (1.0 - p.p714)) * (1.0 - (((0.05 * p.p714) * (1.0 + p.p714)) * s.v[322]))));
        }

        s.v[325] = ((0.1) as f64).powf((-p.p716));

        s.b[1514] = (p.p716 == 1.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if s.b[1514] {
            s.store_scalar(326, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1514]) {
            s.store_scalar(326, ((1.0 / (1.0 - p.p716)) * (1.0 - (((0.05 * p.p716) * (1.0 + p.p716)) * s.v[325]))));
        }

        s.v[328] = ((0.1) as f64).powf((-p.p718));

        s.b[1515] = (p.p718 == 1.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if s.b[1515] {
            s.store_scalar(329, (1.5 - ((0.1) as f64).ln()));
        }

        if (!s.b[1515]) {
            s.store_scalar(329, ((1.0 / (1.0 - p.p718)) * (1.0 - (((0.05 * p.p718) * (1.0 + p.p718)) * s.v[328]))));
        }

        s.b[1516] = (s.v[321] > 0.0);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if s.b[1516] {
            s.store_div(13, 308, 432);
        }

        s.b[1517] = (s.v[13] < 0.9);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1517]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1518] = (p.p714 != 1.0);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        s.b[1519] = (p.p714 == 0.5);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if (((s.b[1516] && s.b[1517]) && s.b[1518]) && s.b[1519]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1516] && s.b[1517]) && s.b[1518]) && (!s.b[1519])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p714)));
        }

        if ((s.b[1516] && s.b[1517]) && s.b[1518]) {
            s.store_mul_ad_affine_product_rhs(335, 432, s.ad_value(321), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p714)), 0.0);
        }

        if ((s.b[1516] && s.b[1517]) && (!s.b[1518])) {
            s.store_mul_ad_affine_product_rhs(335, 432, s.ad_value(321), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1516] && (!s.b[1517])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p714), (((((-1.0)) * ((5.0 * p.p714)))) + ((1.0 + p.p714)))), s.v[322]);
            s.store_mul_ad_product_rhs(335, 432, s.ad_value(321), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if (!s.b[1516]) {
            s.store_scalar(335, 0.0);
        }

        s.b[1520] = (s.v[324] > 0.0);
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if s.b[1520] {
            s.store_div(13, 308, 433);
        }

        s.b[1521] = (s.v[13] < 0.9);
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if (s.b[1520] && s.b[1521]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1522] = (p.p716 != 1.0);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        s.b[1523] = (p.p716 == 0.5);
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        if (((s.b[1520] && s.b[1521]) && s.b[1522]) && s.b[1523]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1520] && s.b[1521]) && s.b[1522]) && (!s.b[1523])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p716)));
        }

        if ((s.b[1520] && s.b[1521]) && s.b[1522]) {
            s.store_mul_ad_affine_product_rhs(336, 433, s.ad_value(324), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p716)), 0.0);
        }

        if ((s.b[1520] && s.b[1521]) && (!s.b[1522])) {
            s.store_mul_ad_affine_product_rhs(336, 433, s.ad_value(324), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1520] && (!s.b[1521])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p716), (((((-1.0)) * ((5.0 * p.p716)))) + ((1.0 + p.p716)))), s.v[325]);
            s.store_mul_ad_product_rhs(336, 433, s.ad_value(324), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if (!s.b[1520]) {
            s.store_scalar(336, 0.0);
        }

        s.b[1524] = (s.v[327] > 0.0);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        if s.b[1524] {
            s.store_div(13, 308, 434);
        }

        s.b[1525] = (s.v[13] < 0.9);
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if (s.b[1524] && s.b[1525]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1526] = (p.p718 != 1.0);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        s.b[1527] = (p.p718 == 0.5);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        if (((s.b[1524] && s.b[1525]) && s.b[1526]) && s.b[1527]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if (((s.b[1524] && s.b[1525]) && s.b[1526]) && (!s.b[1527])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p718)));
        }

        if ((s.b[1524] && s.b[1525]) && s.b[1526]) {
            s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p718)), 0.0);
        }

        if ((s.b[1524] && s.b[1525]) && (!s.b[1526])) {
            s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if (s.b[1524] && (!s.b[1525])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p718), (((((-1.0)) * ((5.0 * p.p718)))) + ((1.0 + p.p718)))), s.v[328]);
            s.store_mul_ad_product_rhs(337, 434, s.ad_value(327), A::add(s.ad_value(14), s.ad_value(329)));
        }

        if (!s.b[1524]) {
            s.store_scalar(337, 0.0);
        }

        s.store_ad_value(334, A::add_scaled_inputs3(s.ad_value(335), 1.0, s.ad_value(336), 1.0, s.ad_value(337), 1.0));

        s.b[1528] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if s.b[1528] {
            s.store_scaled_mul(321, 426, 251, p.p1128);
        }

        s.b[1529] = (s.v[301] > (s.v[35] * p.p2));
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if (s.b[1528] && s.b[1529]) {
            s.store_mul_ad_rhs(324, 427, A::scale_offset(s.ad_value(301), p.p1128, (((((-(s.v[35] * p.p2))) * (p.p1128))) + ((s.v[35] * p.p2)))));
        }

        if (s.b[1528] && (!s.b[1529])) {
            s.store_scaled_mul(324, 427, 301, p.p1128);
        }

        s.b[1530] = (s.v[321] > 0.0);
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        if (s.b[1528] && s.b[1530]) {
            s.store_div(13, 309, 432);
        }

        s.b[1531] = (s.v[13] < 0.9);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        if ((s.b[1528] && s.b[1530]) && s.b[1531]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1532] = (p.p714 != 1.0);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        s.b[1533] = (p.p714 == 0.5);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && s.b[1533]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && (!s.b[1533])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p714)));
        }

        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) {
            s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p714)), 0.0);
        }

        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && (!s.b[1532])) {
            s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if ((s.b[1528] && s.b[1530]) && (!s.b[1531])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p714), (((((-1.0)) * ((5.0 * p.p714)))) + ((1.0 + p.p714)))), s.v[322]);
            s.store_mul_ad_product_rhs(339, 432, s.ad_value(321), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if (s.b[1528] && (!s.b[1530])) {
            s.store_scalar(339, 0.0);
        }

        s.b[1534] = (s.v[324] > 0.0);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if (s.b[1528] && s.b[1534]) {
            s.store_div(13, 309, 433);
        }

        s.b[1535] = (s.v[13] < 0.9);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if ((s.b[1528] && s.b[1534]) && s.b[1535]) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.b[1536] = (p.p716 != 1.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_18(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1537] = (p.p716 == 0.5);
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && s.b[1537]) {
            s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));
        }

        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && (!s.b[1537])) {
            s.store_ad_value(311, A::limited_exp_scaled_input(A::ln(s.ad_value(310)), (-p.p716)));
        }

        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) {
            s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p716)), 0.0);
        }

        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && (!s.b[1536])) {
            s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::ln(s.ad_value(310)), -1.0, 0.0);
        }

        if ((s.b[1528] && s.b[1534]) && (!s.b[1535])) {
            s.store_scaled_mul_ad(14, A::offset(s.ad_value(13), (-1.0)), A::scale_offset(s.ad_value(13), (5.0 * p.p716), (((((-1.0)) * ((5.0 * p.p716)))) + ((1.0 + p.p716)))), s.v[325]);
            s.store_mul_ad_product_rhs(340, 433, s.ad_value(324), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if (s.b[1528] && (!s.b[1534])) {
            s.store_scalar(340, 0.0);
        }

        if s.b[1528] {
            s.store_add(338, 339, 340);
        }

        if (!s.b[1528]) {
            s.store_scalar(338, 0.0);
        }

        s.b[1538] = (p.p38 != 0.0);
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if s.b[1538] {
            s.store_powf_ad(13, A::scale(s.ad_value(481), 1.0000000000000001e-23), p.p954);
            s.store_powf_ad(14, A::div_from_scalar(300.0, s.ad_value(391)), p.p955);
            s.store_ad_value(15, A::div_scaled_product(s.ad_value(187), A::voltage(ctx, nodes, Some(11), Some(7)), p.p953, s.ad_value(108), 1.0));
            s.store_scaled_limited_exp_ad(707, A::mul_scaled_lhs(s.ad_value(13), -1.0, s.ad_value(14)), p.p948);
            s.store_scaled_mul(708, 14, 13, p.p949);
            s.store_scale_ad(709, A::tanh(A::limited_exp(A::mul_scaled_lhs(s.ad_value(187), p.p952, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(11)), 1.0, s.ad_value(857), (-1.0), A::voltage(ctx, nodes, Some(7), Some(11)), -1.0)))), p.p951);
            s.store_ad_value(706, A::mul_offset_rhs(A::mul3(A::mul3_scaled_output(s.ad_value(57), s.ad_value(707), A::limited_exp(s.ad_value(15)), (p.p2 * s.v[29])), A::limited_exp_scaled_input(s.ad_value(708), (-s.v[30])), A::limited_exp(A::div(s.ad_value(709), s.ad_value(108)))), A::limited_exp(A::div_scaled_inputs(s.ad_value(76), p.p950, s.ad_value(108), 1.0)), (-1.0)));
        }

        s.store_scale(377, 108, (4.0 * 1.60219e-19));

        s.store_scaled_div(360, 502, 157, 2.0);

        s.b[1539] = (p.p784 <= 0.0);
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if s.b[1539] {
            s.store_scalar(363, 0.0);
        }

        if (!s.b[1539]) {
            s.store_ad_value(12, A::div_scaled_offset_numerator(A::div(s.ad_value(167), s.ad_value(129)), 1.0, p.p784, s.ad_value(360), 1.0));
            s.store_mul_ln_ad_rhs(363, 129, A::max_with_scalar(s.ad_value(12), 1e-38));
        }

        s.b[1540] = (s.v[363] < 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if ((!s.b[1539]) && s.b[1540]) {
            s.store_scalar(363, 0.0);
        }

        s.store_mul_scaled_ad_rhs(367, 108, 1.0 / (1.60219e-19), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(483)));

        s.store_mul_ad_affine_product_lhs(366, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(144), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);

        s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(188)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);

        s.store_mul3_affine_lhs(737, 108, 188, 1.60219e-19, 0.0, 188);

        s.store_ad_value(738, A::add_scaled_product(A::scale_offset(s.ad_value(366), p.p799, p.p785), 1.0, s.ad_value(366), s.ad_value(366), p.p800));

        s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));

        s.store_scale(740, 108, (p.p785 * 1.60219e-19));

        s.b[1541] = (p.p1065 == 1.0);
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if s.b[1541] {
            s.store_scalar(745, s.v[30]);
            s.store_ad_value(712, A::div_scaled_inputs2(s.ad_value(64), 1.0, s.ad_value(482), (-1.0), s.ad_value(108), 1.0));
            s.store_scaled_sqrt_ad(713, A::div_from_scalar((((2.0 * 1.60219e-19) * s.v[26]) * p.p1068), s.ad_value(108)), 1.0 / (s.v[46]));
            s.store_ln_ad(714, A::div_from_scalar(p.p1068, s.ad_value(28)));
            s.store_scalar(13, 1.0);
            s.store_div(204, 712, 13);
            s.store_div(205, 713, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1542] = (s.v[204] < 0.0);
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1542]) {
            s.store_ad_value(15, A::div_scaled_inputs2(s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0));
            s.store_neg_ad(715, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1541] && (!s.b[1542])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(715, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1541] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(715), (-1.0), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_ad_value(12, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(713), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(713), 1.0));
            s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(715), 1.0, s.ad_value(714), (-2.0), s.ad_value(73), -1.0));
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1543] = (s.v[20] <= (-68.0));
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1543]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1544] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if ((s.b[1541] && s.b[1543]) && s.b[1544]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1545] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && s.b[1545]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && (!s.b[1545])) {
            s.store_ad_value(14, A::div_scaled_inputs2(s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0));
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1541] && s.b[1543]) {
            s.store_mul_ad_rhs(717, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (s.b[1541] && (!s.b[1543])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_ad_value(717, A::add_scaled_offset_product_rhs(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0)));
        }

        s.b[1546] = ((1.0 == 0.0) && (s.v[715] < ((-2500.0) * 2.0)));
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1546]) {
            s.store_div_from_scalar_scaled_input(716, ((-2.0) * 2.0), 715, 16.0);
        }

        if (s.b[1541] && (!s.b[1546])) {
            s.store_scaled_add_ad(716, A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(715), (-1.0), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1541] {
            s.store_offset_ad(718, A::div_scaled_inputs(s.ad_value(713), 1.0, A::sqrt(s.ad_value(716)), 2.0), 1.0);
            s.copy_ad(719, 157);
            s.store_scale(726, 719, (s.v[46] * s.v[29]));
            s.store_scale(725, 157, (s.v[46] * s.v[29]));
            s.store_ad_value(720, A::div_scaled_product_by_product(s.ad_value(188), s.ad_value(746), 1.0, A::mul3_scaled_output(s.ad_value(718), s.ad_value(726), s.ad_value(108), 2.0), s.ad_value(108), 1.0));
            s.store_ad_value(722, A::div_scaled_product_by_product(s.ad_value(188), A::sub(s.ad_value(745), s.ad_value(746)), 1.0, A::mul3_scaled_output(s.ad_value(90), s.ad_value(725), s.ad_value(106), 2.0), s.ad_value(106), 1.0));
            s.store_offset_ad(12, A::add_scaled_inputs3(A::square(s.ad_value(717)), 4.0, s.ad_value(717), 4.0, s.ad_value(720), (-4.0)), 1.0);
        }

        s.b[1547] = (s.v[12] < 1.0);
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1547]) {
            s.store_scalar(721, 0.0);
        }

        if (s.b[1541] && (!s.b[1547])) {
            s.store_offset_scaled_ad(721, A::sqrt(s.ad_value(12)), 0.5, (-0.5));
        }

        if s.b[1541] {
            s.store_offset_scaled_ad(723, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(144)), 4.0, s.ad_value(144), 4.0, s.ad_value(722), 4.0), 1.0)), 0.5, (-0.5));
            s.store_mul_ad_lhs(727, A::mul3_scaled_output(s.ad_value(718), s.ad_value(726), s.ad_value(108), 2.0), 721);
            s.store_mul_ad_lhs(728, A::mul3_scaled_output(s.ad_value(90), s.ad_value(725), s.ad_value(108), 2.0), 144);
            s.store_mul_ad_affine_product_rhs(729, 725, s.ad_value(108), A::sub(s.ad_value(723), s.ad_value(144)), 2.0, 0.0);
            s.store_mul_sub_rhs(730, 727, 745, 746);
            s.store_ad_value(731, A::add_scaled_products(s.ad_value(729), s.ad_value(746), 1.0, s.ad_value(728), s.ad_value(746), 1.0));
            s.store_ad_value(742, A::div_scalar_by_product(1.0, A::add(s.ad_value(730), s.ad_value(731)), A::add(s.ad_value(730), s.ad_value(731)), 1.0));
            s.store_mul_square_lhs(743, 730, 742);
            s.store_mul_square_lhs(744, 731, 742);
        }

        s.b[1548] = (s.v[30] != s.v[746]);
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1548]) {
            s.store_mul3_affine_lhs(724, 90, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 723);
            s.store_ad_value(361, A::add_scaled_inputs3(s.ad_value(745), 1.0, s.ad_value(359), (-2.0), s.ad_value(746), -1.0));
            s.store_square(362, 361);
            s.store_scale(13, 362, (10000000000.0 * s.v[46]));
            s.store_scaled_ln_ad(14, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(724), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38), p.p785);
            s.store_scaled_sub(15, 724, 366, p.p799);
            s.store_scaled_sub_ad(16, A::square(s.ad_value(724)), A::square(s.ad_value(366)), (0.5 * p.p800));
            s.store_scale(17, 362, (10000000000.0 * (s.v[29] * p.p2)));
            s.store_ad_value(732, A::add_scaled_product(A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(17), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(13)), A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0), 1.0));
            s.store_mul3_affine_lhs(18, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);
            s.store_mul_ad_product_lhs(733, A::div(s.ad_value(740), s.ad_value(18)), s.ad_value(188), 188);
            s.store_add(19, 733, 732);
        }

        s.b[1549] = (s.v[19] > 0.0);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if ((s.b[1541] && s.b[1548]) && s.b[1549]) {
            s.store_ad_value(734, A::div_scaled_product(s.ad_value(732), s.ad_value(733), 1.0, s.ad_value(19), 1.0));
        }

        if ((s.b[1541] && s.b[1548]) && (!s.b[1549])) {
            s.store_scalar(734, 0.0);
        }

        if (s.b[1541] && (!s.b[1548])) {
            s.store_scalar(734, 0.0);
        }

        if s.b[1541] {
            s.store_scale(20, 108, (p.p1067 * 1.60219e-19));
            s.store_mul3_affine_lhs(21, 746, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);
            s.store_mul_ad_product_lhs(741, A::div(s.ad_value(20), s.ad_value(21)), s.ad_value(188), 188);
            s.copy_ad(22, 741);
        }

        s.b[1550] = (s.v[22] > 0.0);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if (s.b[1541] && s.b[1550]) {
            s.copy_ad(735, 741);
        }

        if (s.b[1541] && (!s.b[1550])) {
            s.store_scalar(735, 0.0);
        }

        if s.b[1541] {
            s.store_ad_value(370, A::add_scaled_products(s.ad_value(734), s.ad_value(743), 1.0, s.ad_value(735), s.ad_value(744), 1.0));
        }

        s.b[1551] = (p.p801 >= (s.v[30] / 2.0));
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if ((!s.b[1541]) && s.b[1551]) {
            s.store_scalar(359, 0.0);
        }

        if ((!s.b[1541]) && (!s.b[1551])) {
            s.store_scalar(359, p.p801);
        }

        s.b[1552] = (((p.p785 > 0.0) || (p.p799 > 0.0)) || (p.p800 > 0.0));
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        s.b[1553] = ((p.p786 != 0.0) && (p.p785 > 0.0));
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if (((!s.b[1541]) && s.b[1552]) && s.b[1553]) {
            s.store_div(13, 80, 641);
            s.store_offset_pow_ad(14, s.ad_value(13), s.ad_value(642), 1.0);
            s.store_div(15, 640, 14);
            s.store_scale(16, 15, 1.0 / (p.p785));
            s.store_scaled_add_ad(17, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * p.p798) * p.p798))), 0.5);
            s.store_scale(364, 17, p.p785);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1541]) && s.b[1552]) && (!s.b[1553])) {
            s.store_scalar(364, p.p785);
        }

        if ((!s.b[1541]) && s.b[1552]) {
            s.store_sub_from_scalar_ad(361, s.v[30], A::scale(s.ad_value(359), 2.0));
            s.store_square(362, 361);
            s.store_scale(12, 362, (10000000000.0 * s.v[46]));
            s.store_mul_ad_affine_product_lhs(365, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(200), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);
            s.store_mul_ln_ad_rhs(13, 364, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));
            s.store_scaled_sub(14, 365, 366, p.p799);
            s.store_scaled_sub_ad(15, A::square(s.ad_value(365)), A::square(s.ad_value(366)), (0.5 * p.p800));
            s.store_scale(16, 362, (10000000000.0 * (s.v[29] * p.p2)));
            s.store_ad_value(368, A::add_scaled_product(A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0));
            s.store_mul3_affine_lhs(17, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);
            s.store_scaled_mul(740, 364, 108, 1.60219e-19);
            s.store_mul_ad_product_lhs(369, A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(188), 188);
            s.store_add(18, 369, 368);
        }

        s.b[1554] = (s.v[18] > 0.0);
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        if (((!s.b[1541]) && s.b[1552]) && s.b[1554]) {
            s.store_ad_value(370, A::div_scaled_product_by_product(s.ad_value(368), s.ad_value(369), 1.0, s.ad_value(18), A::scale_offset(A::powf(A::sub(s.ad_value(200), s.ad_value(144)), p.p803), p.p802, 1.0), 1.0));
        }

        if (((!s.b[1541]) && s.b[1552]) && (!s.b[1554])) {
            s.store_scalar(370, 0.0);
        }

        if ((!s.b[1541]) && (!s.b[1552])) {
            s.store_scalar(370, 0.0);
        }

        s.store_scaled_div(12, 80, 360, 1.0 / (s.v[30]));

        s.store_square(13, 12);

        s.store_offset_scaled(15, 13, (((p.p814 * s.v[30])) * (p.p811)), p.p811);

        s.store_offset_scaled(16, 13, (((p.p815 * s.v[30])) * (p.p812)), p.p812);

        s.store_offset_scaled(17, 13, (((p.p1044 * s.v[30])) * (p.p1043)), p.p1043);

        s.store_offset_scaled(386, 13, (((p.p816 * s.v[30])) * (p.p813)), p.p813);

        s.store_scaled_mul(387, 15, 15, 3.0);

        s.store_offset_scaled(387, 387, ((((-s.v[30]) / p.p1042)) as f64).exp(), (((((-1.0)) * (((((-s.v[30]) / p.p1042)) as f64).exp()))) + (1.0)));

        s.store_square(389, 17);

        s.store_square(388, 16);

        s.v[383] = 0.0;

        s.b[1555] = (p.p48 == 0.0);
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        s.b[1556] = (p.p48 == 1.0);
        s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };

        if s.b[1555] {
            s.store_scaled_mul(196, 108, 190, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));
            s.store_scaled_mul(197, 108, 193, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));
            s.store_mul_abs_ad_rhs(12, 157, A::add(s.ad_value(196), s.ad_value(197)));
            s.store_offset_mul(13, 12, 244, (s.v[30] * s.v[30]));
            s.store_scaled_div(375, 12, 13, p.p810);
            s.store_mul(376, 377, 375);
        }

        if (s.b[1556] && (!s.b[1555])) {
            s.store_scaled_mul(382, 90, 106, 2.0);
            s.store_mul_scale_ad_lhs(12, A::mul3(s.ad_value(157), s.ad_value(163), s.ad_value(175)), s.v[46], 382);
            s.store_scaled_add(13, 200, 144, 0.5);
            s.store_offset(15, 13, 0.5);
            s.store_square(16, 15);
            s.store_mul(17, 16, 15);
            s.store_sub(18, 200, 144);
            s.store_square(19, 18);
            s.store_mul(20, 19, 18);
            s.store_mul_ad_lhs(21, A::scale_offset(s.ad_value(13), 6.0, 0.5), 19);
            s.store_scale(381, 163, s.v[30]);
            s.store_scale(22, 381, 1.0 / (s.v[30]));
            s.store_offset_ad(24, A::div_scaled_product_by_product(s.ad_value(389), s.ad_value(139), 1.0, s.ad_value(140), A::offset(s.ad_value(80), p.p1045), 1.0), 1.0);
            s.store_offset_scaled(24, 24, ((((-s.v[30]) / p.p1042)) as f64).exp(), (((((-1.0)) * (((((-s.v[30]) / p.p1042)) as f64).exp()))) + (1.0)));
        }

        s.b[1557] = ((0.0 == 0.0) && (s.v[24] < ((-2500.0) * 0.1)));
        s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };

        if ((s.b[1556] && (!s.b[1555])) && s.b[1557]) {
            s.store_div_from_scalar_scaled_input(24, ((-0.1) * 0.1), 24, 16.0);
        }

        if ((s.b[1556] && (!s.b[1555])) && (!s.b[1557])) {
            s.store_scaled_add_ad_rhs(24, 24, A::sqrt(A::offset(A::mul(s.ad_value(24), s.ad_value(24)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if (s.b[1556] && (!s.b[1555])) {
            s.store_mul_ad(380, A::div_scaled_inputs(s.ad_value(12), (p.p2 * s.v[29]), s.ad_value(381), 1.0), A::add_scaled_product(A::div_scaled_product(s.ad_value(19), s.ad_value(387), 1.0, s.ad_value(15), 12.0), 1.0, s.ad_value(13), s.ad_value(24), 1.0));
        }

        if (s.b[1556] && (!s.b[1555])) {
            let assign29020_ad_e38614: A = A::div_scaled_product3(A::mul3(s.ad_value(381), s.ad_value(22), s.ad_value(22)), A::add_scaled_inputs3(A::div(s.ad_value(13), s.ad_value(16)), 1.0, A::div(s.ad_value(21), A::mul_scaled_lhs(s.ad_value(16), 60.0, s.ad_value(16))), (-1.0), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(19), 1.0, s.ad_value(16), s.ad_value(17), 144.0), 1.0), s.ad_value(388), (15.0 * 1.0 / (4.0)), s.ad_value(12), ((p.p2 * s.v[29]) * 12.0));
            s.store_ad_value(378, assign29020_ad_e38614);
        }

        if (s.b[1556] && (!s.b[1555])) {
            s.store_mul_ad_affine_product_lhs(379, s.ad_value(22), A::sub(A::div_scaled_inputs(s.ad_value(18), 1.0, s.ad_value(15), 12.0), A::div_scaled_inputs(s.ad_value(20), 1.0, s.ad_value(17), 144.0)), 2.531645569620253, 0.0, 386);
            s.store_sqrt_mul(384, 377, 380);
        }

        s.b[1558] = (s.v[378] > 0.0);
        s.v[1558] = if s.b[1558] { 1.0 } else { 0.0 };

        if ((s.b[1556] && (!s.b[1555])) && s.b[1558]) {
            s.store_sqrt_div(385, 377, 378);
        }

        s.b[1559] = (s.v[384] > 0.0);
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        if (((s.b[1556] && (!s.b[1555])) && s.b[1558]) && s.b[1559]) {
            s.store_ad_value(383, A::div_scaled_product(s.ad_value(379), s.ad_value(385), 1.0, s.ad_value(384), 1.0));
        }

        if (((s.b[1556] && (!s.b[1555])) && s.b[1558]) && (!s.b[1559])) {
            s.store_scalar(383, 0.0);
        }

        if ((s.b[1556] && (!s.b[1555])) && (!s.b[1558])) {
            s.store_scalar(385, 0.0);
            s.store_scalar(383, 0.0);
        }

        s.b[1560] = (p.p46 != 0.0);
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        s.b[1561] = (p.p47 != 0.0);
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        s.copy_ad(60, 59);

        s.v[218] = 0.0;

        s.b[1562] = (p.p40 == 1.0);
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        if s.b[1562] {
            s.store_offset(549, 549, p.p35);
            s.store_mul(65, 64, 109);
            s.store_mul(73, 72, 109);
            s.store_mul(58, 549, 109);
            s.store_sub(60, 65, 58);
            s.store_ln_ad(233, A::max_with_scalar(A::div(s.ad_value(550), s.ad_value(28)), 1e-38));
            s.store_scaled_sqrt_ad(234, A::mul_scaled_lhs(s.ad_value(550), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(109)), 1.0 / (s.v[46]));
            s.store_div_from_scalar(126, 1.0, 234);
            s.store_scaled_div(206, 479, 108, ((((2.0 * 1.60219e-19) * s.v[26])) * 1.0 / ((s.v[46] * s.v[46]))));
        }

        if s.b[1562] {
            s.store_ad_value(218, {
                if (s.v[479] > 0.0) {
                    A::div_from_scalar(1.0, s.ad_value(206))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1562] {
            s.store_ad_value(203, {
                if (s.v[479] > 0.0) {
                    A::div(s.ad_value(550), s.ad_value(479))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1562] {
            s.store_offset(13, 203, 1.0);
            s.store_div(204, 60, 13);
            s.store_div(205, 234, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1563] = (s.v[204] < 0.0);
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1563]) {
            s.store_ad_value(15, A::div_scaled_inputs2(s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0));
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1562] && (!s.b[1563])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1562] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_ad_value(12, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(234), 1.0));
            s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(233), (-2.0), s.ad_value(73), -1.0));
            s.store_sub_scaled_ad_rhs(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1564] = (s.v[20] <= (-68.0));
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1564]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1565] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };

        if ((s.b[1562] && s.b[1564]) && s.b[1565]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1566] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };

        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && s.b[1566]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && (!s.b[1566])) {
            s.store_ad_value(14, A::div_scaled_inputs2(s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0));
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1562] && s.b[1564]) {
            s.store_mul_ad_rhs(200, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0));
        }

        if (s.b[1562] && (!s.b[1564])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);
            s.store_ad_value(200, A::add_scaled_offset_product_rhs(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0)));
        }

        s.b[1567] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));
        s.v[1567] = if s.b[1567] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1567]) {
            s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);
        }

        if (s.b[1562] && (!s.b[1567])) {
            s.store_scaled_add_ad(93, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1562] {
            s.store_sqrt(96, 93);
            s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);
        }

        s.b[1568] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1568] = if s.b[1568] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1568]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (s.b[1562] && (!s.b[1568])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1562] {
            s.store_offset_div_ad(90, s.ad_value(234), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);
            s.store_mul_ad_rhs(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));
        }

        s.b[1569] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));
        s.v[1569] = if s.b[1569] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1569]) {
            s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);
        }

        if (s.b[1562] && (!s.b[1569])) {
            s.store_scaled_add_ad_rhs(84, 12, A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if s.b[1562] {
            s.store_mul3_affine_lhs(130, 90, 108, 2.0, 0.0, 200);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1562] {
            s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));
            s.store_mul_ad(15, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(62), 1.0), A::pow(s.ad_value(132), s.ad_value(407)));
            s.store_offset(16, 15, 1.0);
        }

        s.b[1570] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1570]) {
            s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (s.b[1562] && (!s.b[1570])) {
            s.store_scaled_add_ad(133, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        if s.b[1562] {
            s.store_ad_value(137, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(108), 1.0, s.ad_value(133), s.ad_value(411), s.v[34]));
            s.store_ad_value(131, A::div_scaled_product_offset_denominator(s.ad_value(137), A::add(A::square(s.ad_value(200)), s.ad_value(200)), 1.0, A::mul_offset_rhs(s.ad_value(137), s.ad_value(200), 1.0), 1.0, 1.0));
            s.store_ad_value(145, A::add_scaled_inputs4(s.ad_value(91), 1.0, s.ad_value(233), (-2.0), s.ad_value(131), (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(234), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0));
            s.store_mul(146, 145, 108);
        }

        s.b[1571] = ((0.0 == 0.0) && ((s.v[146] - s.v[72]) < ((-2500.0) * 0.001)));
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1571]) {
            s.store_div_from_scalar_ad(141, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(146), 16.0, s.ad_value(72), 16.0));
        }

        if (s.b[1562] && (!s.b[1571])) {
            s.store_ad_value(141, A::add_scaled_inputs3(s.ad_value(146), 0.5, s.ad_value(72), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(146), s.ad_value(72)), A::sub(s.ad_value(146), s.ad_value(72))), ((0.25 * 0.001) * 0.001))), 0.5));
        }

        s.b[1572] = ((p.p1134 == 0.0) && (p.p1135 == 0.0));
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1572]) {
            s.store_scalar(783, p.p1129);
        }

        if (s.b[1562] && (!s.b[1572])) {
            s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);
            s.store_offset_ad(783, A::div_scaled_inputs2(s.ad_value(13), p.p1134, A::mul3_scaled_output(s.ad_value(13), s.ad_value(200), s.ad_value(106), p.p1135), (-1.0), A::scale_offset(s.ad_value(61), p.p1136, 1.0), 1.0), 1.0);
        }

        s.b[1573] = ((0.1 == 0.0) && (s.v[783] < ((-2500.0) * 0.0005)));
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if ((s.b[1562] && (!s.b[1572])) && s.b[1573]) {
            s.store_div_from_scalar_scaled_input(783, ((-0.0005) * 0.0005), 783, 16.0);
        }

        if ((s.b[1562] && (!s.b[1572])) && (!s.b[1573])) {
            s.store_scaled_add_ad(783, A::offset(s.ad_value(783), 0.1), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(783), (-0.1), A::offset(s.ad_value(783), (-0.1))), ((0.25 * 0.0005) * 0.0005))), 0.5);
        }

        if s.b[1562] {
            s.store_div(141, 141, 783);
            s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(141)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
            s.store_mul(139, 75, 20);
            s.store_mul_add_lhs(142, 139, 72, 109);
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_ad_value(12, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(234), 1.0));
            s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(233), (-2.0), s.ad_value(142), -1.0));
            s.store_sub_scaled_ad_rhs(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1574] = (s.v[20] <= (-68.0));
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1574]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1575] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if ((s.b[1562] && s.b[1574]) && s.b[1575]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1576] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && s.b[1576]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && (!s.b[1576])) {
            s.store_ad_value(14, A::div_scaled_inputs2(s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0));
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1562] && s.b[1574]) {
            s.store_mul_ad_rhs(144, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0));
        }

        if (s.b[1562] && (!s.b[1574])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);
            s.store_ad_value(144, A::add_scaled_offset_product_rhs(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0)));
        }

        if s.b[1562] {
            s.store_offset_ad(92, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(200), (-1.0), s.ad_value(144), -1.0), (-1.0));
        }

        s.b[1577] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1577]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (s.b[1562] && (!s.b[1577])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1562] {
            s.store_sqrt(14, 12);
            s.store_add_ad(15, A::offset(s.ad_value(203), 1.0), A::div(s.ad_value(234), A::add(s.ad_value(96), s.ad_value(14))));
            s.store_offset_ad(16, A::mul3(s.ad_value(203), s.ad_value(14), s.ad_value(126)), 0.5);
            s.store_sqrt_add_ad(17, A::square(s.ad_value(16)), A::mul3(s.ad_value(15), A::add(s.ad_value(200), s.ad_value(144)), s.ad_value(218)));
            s.store_div_ad_rhs(90, 15, A::add(s.ad_value(16), s.ad_value(17)));
            s.store_mul_ad_rhs(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));
        }

        s.b[1578] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1578]) {
            s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);
        }

        if (s.b[1562] && (!s.b[1578])) {
            s.store_scaled_add_ad_rhs(84, 12, A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if s.b[1562] {
            s.store_mul_ad_rhs(13, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(144), A::offset(s.ad_value(90), (-1.0)), (-2.0)));
        }

        s.b[1579] = ((0.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.1)));
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1579]) {
            s.store_div_from_scalar_scaled_input(85, ((-0.1) * 0.1), 13, 16.0);
        }

        if (s.b[1562] && (!s.b[1579])) {
            s.store_scaled_add_ad_rhs(85, 13, A::sqrt(A::offset(A::mul(s.ad_value(13), s.ad_value(13)), ((0.25 * 0.1) * 0.1))), 0.5);
        }

        if s.b[1562] {
            s.store_scaled_add(86, 84, 85, 0.5);
            s.store_mul_ad_product_rhs(80, 90, s.ad_value(108), A::add(s.ad_value(200), s.ad_value(144)));
            s.store_add_scaled_inputs(156, 86, s.v[155], 80, (s.v[158] * s.v[155]));
            s.store_offset(13, 203, 1.0);
            s.store_ad_value(204, A::div_scaled_inputs2(s.ad_value(60), 1.0, s.ad_value(109), p.p136, s.ad_value(13), 1.0));
            s.store_div(205, 234, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1580] = (s.v[204] < 0.0);
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1580]) {
            s.store_ad_value(15, A::div_scaled_inputs2(s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0));
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1562] && (!s.b[1580])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1562] {
            s.store_mul_ad(15, A::add_scaled_product(s.ad_value(506), 1.0, s.ad_value(516), s.ad_value(62), 1.0), A::pow(s.ad_value(156), s.ad_value(407)));
            s.store_offset(16, 15, 1.0);
        }

        s.b[1581] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if (s.b[1562] && s.b[1581]) {
            s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);
        }

        if (s.b[1562] && (!s.b[1581])) {
            s.store_scaled_add_ad(159, A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(16), (-1.0), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015))), 0.5);
        }

        if s.b[1562] {
            s.store_ad_value(138, A::div_scaled_product_by_product(s.ad_value(499), s.ad_value(108), 2.0, s.ad_value(159), s.ad_value(411), s.v[34]));
            s.store_sub(87, 200, 144);
            s.store_mul_ad_affine_product_rhs(13, 138, s.ad_value(87), A::mul(s.ad_value(138), s.ad_value(87)), 2.0, 0.0);
            s.store_sqrt_offset_input(161, 13, 1.0);
            s.store_scaled_offset(162, 161, 1.0, 0.5);
            s.store_ad_value(134, A::div_scaled_inputs(s.ad_value(411), 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0));
            s.store_scale(135, 134, s.v[34]);
            s.store_add(170, 141, 135);
            s.store_sub(167, 75, 139);
        }

        s.b[1582] = (s.v[542] != 0.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if s.b[1582] {
            s.store_offset_mul_ad(176, s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(542), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);
        }

        if (!s.b[1582]) {
            s.store_scalar(176, 1.0);
        }

        s.store_square(207, 176);

        s.store_div_from_scalar(208, 1.0, 176);

        s.store_div_from_scalar(209, 1.0, 207);

        s.store_offset(210, 176, (-1.0));

        s.store_sub(213, 60, 91);

        s.store_sub(216, 200, 144);

        s.store_mul_ad(217, A::sub(s.ad_value(200), s.ad_value(144)), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_add_scaled_inputs(211, 213, 1.0, 200, 2.0);

        s.store_add_scaled_inputs(212, 213, 1.0, 144, 2.0);

        s.b[1583] = ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5)));
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if s.b[1583] {
            s.store_div_from_scalar_scaled_input(13, ((-0.5) * 0.5), 211, 16.0);
        }

        if (!s.b[1583]) {
            s.store_scaled_add_ad_rhs(13, 211, A::sqrt(A::offset(A::mul(s.ad_value(211), s.ad_value(211)), ((0.25 * 0.5) * 0.5))), 0.5);
        }

        s.b[1584] = ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5)));
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        if s.b[1584] {
            s.store_div_from_scalar_scaled_input(14, ((-0.5) * 0.5), 212, 16.0);
        }

        if (!s.b[1584]) {
            s.store_scaled_add_ad_rhs(14, 212, A::sqrt(A::offset(A::mul(s.ad_value(212), s.ad_value(212)), ((0.25 * 0.5) * 0.5))), 0.5);
        }

        s.store_sqrt_offset_ad(214, A::mul(s.ad_value(13), s.ad_value(218)), 0.25);

        s.store_sqrt_offset_ad(215, A::mul(s.ad_value(14), s.ad_value(218)), 0.25);

        s.store_div_ad_rhs(13, 211, A::scale_offset(s.ad_value(214), 2.0, 1.0));

        s.store_div_ad_rhs(14, 212, A::scale_offset(s.ad_value(215), 2.0, 1.0));

        s.store_add(15, 214, 215);

        s.store_scaled_div_ad_rhs(16, 217, A::mul(A::square(s.ad_value(15)), s.ad_value(15)), 0.3333333333333333);

        s.store_ad_value(17, A::div_scaled_product3(s.ad_value(783), s.ad_value(162), s.ad_value(208), 1.0, A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)), 1.0));

        s.store_mul_scale_ad_lhs(18, A::add_scaled_square_product(s.ad_value(15), 1.0, s.ad_value(214), s.ad_value(215), 1.0), 0.8, 17);

        s.store_add_scaled_inputs(19, 18, 1.0, 218, 2.0);

        s.store_scaled_mul(20, 217, 17, 0.3333333333333333);

        s.store_ad_value(202, A::div_scaled_product(s.ad_value(212), A::scale_offset(s.ad_value(215), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(215), 2.0, 1.0), 1.0));

        s.store_add_ad_lhs(201, A::add_scaled_offset_product_lhs(s.ad_value(213), 1.0, s.ad_value(90), (-1.0), s.ad_value(144), (-2.0)), 202);

        s.store_ad_value(189, A::add_scaled_products(s.ad_value(208), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, A::add_scaled_products(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(90), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(20), 1.0), (-1.0)), 1.0), 1.0, s.ad_value(210), s.ad_value(201), 1.0));

        s.store_add(21, 200, 144);

        s.store_mul3_lhs(22, 217, 17, 17);

        s.store_add_ad(194, A::mul3(s.ad_value(90), s.ad_value(208), A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(217), s.ad_value(17), 0.3333333333333333)), A::mul3_scaled_output(s.ad_value(90), s.ad_value(210), s.ad_value(144), 2.0));

        s.store_mul_ad_product_rhs(191, 90, s.ad_value(209), A::add_scaled_product(s.ad_value(21), 0.5, s.ad_value(216), A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(17))), 1.0, s.ad_value(22), 0.2), (-1.0 / (6.0))));

        s.store_mul_ad_product_lhs(192, s.ad_value(90), A::sub(s.ad_value(176), s.ad_value(208)), 144);

        s.store_add(193, 191, 192);

        s.store_sub(190, 194, 193);

        s.b[1585] = ((0.0 == 0.0) && ((s.v[108] * s.v[189]) < ((-2500.0) * p.p694)));
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if s.b[1585] {
            s.store_div_from_scalar_ad(83, ((-p.p694) * p.p694), A::mul_scaled_output(s.ad_value(108), s.ad_value(189), 16.0));
        }

        if (!s.b[1585]) {
            s.store_ad_value(83, A::add_scaled_product(A::sqrt(A::offset(A::mul3(s.ad_value(108), s.ad_value(189), A::mul(s.ad_value(108), s.ad_value(189))), ((0.25 * p.p694) * p.p694))), 0.5, s.ad_value(108), s.ad_value(189), 0.5));
        }

        s.store_mul_add_rhs(82, 108, 190, 193);

        s.store_add_scaled_inputs(12, 82, 1.0 / (p.p207), 83, (p.p208 * 1.0 / (p.p207)));

        s.store_offset_powf_ad(13, s.ad_value(12), (0.7 * p.p206), 1.0);

        s.store_div_from_scalar(227, (p.p205 * 1.9e-9), 13);

        s.store_div_from_scalar_ad(228, (3.9 * 8.85418e-12), A::add_scaled_inputs(s.ad_value(229), (3.9 * 1.0 / (p.p111)), s.ad_value(227), 1.0 / (s.v[47])));

        s.store_mul_ad_affine_product_lhs(195, A::div_from_scalar((8.85418e-12 * p.p111), s.ad_value(229)), s.ad_value(108), (((-p.p2) * s.v[33]) * s.v[34]), 0.0, 189);

        s.store_scaled_mul(199, 228, 108, ((p.p2 * s.v[33]) * s.v[34]));

        s.store_mul_neg_lhs(196, 199, 190);

        s.store_mul_neg_lhs(197, 199, 193);

        s.store_neg_ad(198, A::add_scaled_inputs3(s.ad_value(195), 1.0, s.ad_value(196), 1.0, s.ad_value(197), 1.0));

        s.b[1586] = (!param_given[666]);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1586] {
            s.store_scalar(544, ((((2.0 * p.p111) * 8.85418e-12) / 3.141592653589793) * ((((p.p670 * (1.0 + (4e-7 / p.p77)))).max(1e-38)) as f64).ln()));
        }

        s.store_offset(225, 544, p.p671);

        s.store_offset(226, 544, p.p672);

        s.b[1587] = (p.p41 == 0.0);
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        if s.b[1587] {
            s.store_scaled_mul(223, 225, 231, ((-s.v[33]) * p.p2));
            s.store_scaled_mul(224, 226, 232, ((-s.v[33]) * p.p2));
        }

        if (!s.b[1587]) {
            s.store_sqrt_offset_ad(12, A::mul_offset_lhs(A::sub(s.ad_value(231), s.ad_value(63)), 0.02, A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02)), (4.0 * 0.02));
            s.store_ad_value(219, A::add_scaled_inputs3_offset(s.ad_value(231), 0.5, s.ad_value(63), ((-1.0) * 0.5), s.ad_value(12), (-0.5), (0.02 * 0.5)));
            s.store_div_ad_rhs(18, 219, A::powf(A::offset(A::powf(A::scale(s.ad_value(219), (-1.0 / (p.p692))), p.p693), 1.0), (1.0 / p.p693)));
            s.store_sqrt_sub_from_scalar_ad(13, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(547), 1.0));
            s.store_ad_value(223, A::add_scaled_products(s.ad_value(225), s.ad_value(231), ((-s.v[33]) * p.p2), s.ad_value(545), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(231), 1.0, s.ad_value(63), (-1.0), s.ad_value(219), -1.0), 1.0, s.ad_value(547), s.ad_value(13), (-1.0), (-0.5)), ((-s.v[33]) * p.p2)));
            s.store_sqrt_offset_ad(12, A::mul_offset_lhs(A::sub(s.ad_value(232), s.ad_value(63)), 0.02, A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02)), (4.0 * 0.02));
            s.store_ad_value(220, A::add_scaled_inputs3_offset(s.ad_value(232), 0.5, s.ad_value(63), ((-1.0) * 0.5), s.ad_value(12), (-0.5), (0.02 * 0.5)));
            s.store_div_ad_rhs(18, 220, A::powf(A::offset(A::powf(A::scale(s.ad_value(220), (-1.0 / (p.p690))), p.p691), 1.0), (1.0 / p.p691)));
            s.store_sqrt_sub_from_scalar_ad(14, 1.0, A::div_scaled_inputs(s.ad_value(18), 4.0, s.ad_value(548), 1.0));
            s.store_ad_value(224, A::add_scaled_products(s.ad_value(226), s.ad_value(232), ((-s.v[33]) * p.p2), s.ad_value(546), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(232), 1.0, s.ad_value(63), (-1.0), s.ad_value(220), -1.0), 1.0, s.ad_value(548), s.ad_value(14), (-1.0), (-0.5)), ((-s.v[33]) * p.p2)));
        }

        s.store_ad_value(221, A::mul_scaled_lhs(s.ad_value(187), (((-p.p2) * s.v[34]) * p.p673), A::voltage(ctx, nodes, Some(10), Some(11))));

        s.b[1588] = (p.p37 == 1.0);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        if s.b[1588] {
            s.store_ln_ad(684, A::max_with_scalar(A::div(s.ad_value(686), s.ad_value(28)), 1e-38));
            s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(684)), 0.4), s.ad_value(489)), 0.4);
            s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(686), 1.60219e-19));
        }

        if s.b[1588] {
            let assign31550_ad_e41781: A = {
                if (!((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0, A::offset(A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(622), s.ad_value(395), (-1.0)), 1.0, 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(674, 612, assign31550_ad_e41781);
        }

        if s.b[1588] {
            s.store_mul_offset_ad_rhs(673, 616, A::mul_offset_rhs(s.ad_value(623), s.ad_value(395), (-1.0)), 1.0);
        }

        s.b[1589] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1589]) {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));
        }

        if (s.b[1588] && (!s.b[1589])) {
            s.store_ad_value(110, A::add_scaled_inputs3_offset(s.ad_value(127), 0.5, s.ad_value(61), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05), A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05))), ((0.25 * 0.1) * 0.1))), 0.5, (0.05 * 0.5)));
        }

        if s.b[1588] {
            s.store_sqrt(111, 110);
            s.store_mul(112, 114, 111);
            s.store_div_from_scalar(97, s.v[26], 112);
            s.store_ad_value(113, A::add_scaled_inputs_products(s.ad_value(613), 1.0, s.ad_value(674), 1.0, s.ad_value(614), s.ad_value(76), 1.0, s.ad_value(615), s.ad_value(61), (-1.0)));
            s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);
        }

        s.b[1590] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1590]) {
            s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);
        }

        if (s.b[1588] && (!s.b[1590])) {
            s.store_scaled_add_ad(104, A::offset(s.ad_value(13), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(13), (-1.0), A::offset(s.ad_value(13), (-1.0))), ((0.25 * 0.05) * 0.05))), 0.5);
        }

        if s.b[1588] {
            s.store_mul(106, 104, 108);
            s.store_div_from_scalar(107, 1.0, 106);
            s.store_mul(65, 64, 107);
            s.store_mul(73, 70, 107);
            s.store_mul(58, 482, 107);
            s.store_mul_neg_ad_lhs(677, A::add_scaled_product(s.ad_value(673), 1.0, s.ad_value(617), s.ad_value(61), 1.0), 76);
            s.store_ad_value(124, A::mul_offset_rhs(A::add_scaled_inputs_product(s.ad_value(618), 1.0, s.ad_value(619), 1.0 / (s.v[30]), s.ad_value(620), s.ad_value(61), 1.0), A::pow(s.ad_value(395), s.ad_value(621)), (-1.0)));
            s.store_mul_ad_rhs(679, 129, A::scale_offset(s.ad_value(61), p.p1016, 1.0));
        }

        s.b[1591] = (s.v[679] > 0.0);
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1591]) {
            s.store_div_from_scalar(12, (p.p1015 * s.v[30]), 679);
        }

        s.b[1592] = (s.v[12] < 40.0);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        if ((s.b[1588] && s.b[1591]) && s.b[1592]) {
            s.store_div_from_scalar_offset_ad(676, (0.5 * p.p1014), A::cosh(s.ad_value(12)), (-1.0));
        }

        if ((s.b[1588] && s.b[1591]) && (!s.b[1592])) {
            s.store_scaled_limited_exp_scaled_input(676, 12, -1.0, p.p1014);
        }

        if (s.b[1588] && (!s.b[1591])) {
            s.store_scalar(676, 0.0);
        }

        if s.b[1588] {
            s.store_mul_sub_rhs(678, 676, 675, 127);
            s.store_add_ad_lhs(79, A::add_scaled_product(A::add_scaled_inputs4_offset(s.ad_value(677), 1.0, s.ad_value(124), (-1.0), s.ad_value(678), 1.0, s.ad_value(688), 1.0, p.p961), 1.0, A::add(s.ad_value(624), s.ad_value(666)), s.ad_value(61), (-1.0)), 665);
            s.store_ad_value(59, A::add_scaled_inputs_product(s.ad_value(65), 1.0, s.ad_value(58), (-1.0), s.ad_value(79), s.ad_value(107), (-1.0)));
            s.store_scalar(680, (p.p958 * (1.0 + (p.p959 * ((s.v[30]) as f64).powf((-p.p960))))));
            s.store_scaled_sqrt_ad(687, A::mul_scaled_lhs(s.ad_value(686), ((2.0 * 1.60219e-19) * s.v[26]), s.ad_value(107)), 1.0 / (s.v[46]));
            s.store_mul_offset_rhs(687, 687, 680, 1.0);
            s.store_div(685, 684, 104);
            s.store_scalar(13, 1.0);
            s.store_div(204, 59, 13);
            s.store_div(205, 687, 13);
            s.store_sub_scaled_ad_rhs(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0));
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));
        }

        s.b[1593] = (s.v[204] < 0.0);
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1593]) {
            s.store_ad_value(15, A::div_scaled_inputs2(s.ad_value(204), 1.0, s.ad_value(14), (-1.0), s.ad_value(205), 1.0));
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (s.b[1588] && (!s.b[1593])) {
            s.store_limited_exp_neg_input(15, 14);
            s.store_scale(13, 205, 0.5);
            s.store_sub_ad_lhs(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if s.b[1588] {
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_ad_value(12, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(687), 1.0));
            s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(685), (-2.0), s.ad_value(73), -1.0));
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1594] = (s.v[20] <= (-68.0));
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1594]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1595] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        if ((s.b[1588] && s.b[1594]) && s.b[1595]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1596] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && s.b[1596]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1588] && s.b[1594]) && (!s.b[1595])) && (!s.b[1596])) {
            s.store_ad_value(14, A::div_scaled_inputs2(s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0));
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1588] && s.b[1594]) {
            s.store_mul_ad_rhs(693, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (s.b[1588] && (!s.b[1594])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_ad_value(693, A::add_scaled_offset_product_rhs(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0)));
        }

        if s.b[1588] {
            s.store_ad_value(681, A::add_scaled_product(s.ad_value(106), 2.0, s.ad_value(106), s.ad_value(693), 2.0));
            s.copy_ad(682, 681);
            s.store_add(682, 682, 70);
        }

        s.b[1597] = ((0.0 == 0.0) && ((s.v[682] - s.v[70]) < ((-2500.0) * 0.001)));
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1597]) {
            s.store_div_from_scalar_ad(683, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(682), 16.0, s.ad_value(70), 16.0));
        }

        if (s.b[1588] && (!s.b[1597])) {
            s.store_ad_value(683, A::add_scaled_inputs3(s.ad_value(682), 0.5, s.ad_value(70), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(682), s.ad_value(70)), A::sub(s.ad_value(682), s.ad_value(70))), ((0.25 * 0.001) * 0.001))), 0.5));
        }

        if s.b[1588] {
            s.store_pow_ad(19, A::div(s.ad_value(74), s.ad_value(683)), A::div_from_scalar(1.0, s.ad_value(412)));
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
            s.store_mul(139, 74, 20);
            s.store_mul_add_lhs(142, 139, 70, 107);
            s.store_scaled_add_ad(20, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(96, 20);
            s.store_ad_value(12, A::div_scaled_offset_numerator(A::div_scaled_inputs(s.ad_value(687), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, s.ad_value(687), 1.0));
            s.store_ad_value(13, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(685), (-2.0), s.ad_value(142), -1.0));
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));
            s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul_offset_rhs(s.ad_value(14), s.ad_value(14), 0.402982), 2.446562)), 0.5);
            s.copy_ad(94, 96);
        }

        s.b[1598] = (s.v[20] <= (-68.0));
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1598]) {
            s.store_scalar(16, (-100.0));
            s.store_scalar(17, 20.0);
        }

        s.b[1599] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if ((s.b[1588] && s.b[1598]) && s.b[1599]) {
            s.store_limited_exp(15, 16);
        }

        s.b[1600] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && s.b[1600]) {
            s.store_limited_exp(15, 20);
        }

        if (((s.b[1588] && s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_ad_value(14, A::div_scaled_inputs2(s.ad_value(20), 1.0, s.ad_value(16), (-1.0), s.ad_value(17), 1.0));
            s.store_square(18, 14);
            s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));
        }

        if (s.b[1588] && s.b[1598]) {
            s.store_mul_ad_rhs(692, 15, A::add_scaled_inputs3_offset(s.ad_value(13), 1.0, s.ad_value(20), (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0));
        }

        if (s.b[1588] && (!s.b[1598])) {
            s.store_limited_exp(15, 20);
            s.store_div_from_scalar(95, 1.0, 94);
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
            s.store_ad_value(16, A::add_scaled_inputs3(s.ad_value(15), 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, s.ad_value(13), -1.0));
        }

    }

    pub(super) fn stamp_transient_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1588] && (!s.b[1598])) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));
            s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);
            s.store_sub_ad_lhs(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);
            s.store_ad_value(692, A::add_scaled_offset_product_rhs(s.ad_value(15), 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0)));
        }

        s.b[1601] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1601]) {
            s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);
        }

        if (s.b[1588] && (!s.b[1601])) {
            s.store_scaled_add_ad(93, A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(91), (-1.0), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1588] {
            s.store_sqrt(96, 93);
            s.store_offset_ad(92, A::add_scaled_inputs3(s.ad_value(91), 1.0, s.ad_value(693), (-1.0), s.ad_value(692), -1.0), (-1.0));
        }

        s.b[1602] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1602]) {
            s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);
        }

        if (s.b[1588] && (!s.b[1602])) {
            s.store_scaled_add_ad(12, A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(92), (-1.0), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
        }

        if s.b[1588] {
            s.store_sqrt(14, 12);
            s.store_offset_div_ad(691, s.ad_value(687), A::add(s.ad_value(96), s.ad_value(14)), 1.0);
            s.store_mul_ad_lhs(672, A::mul3(A::mul3_scaled_output(s.ad_value(691), s.ad_value(157), s.ad_value(106), ((2.0 * p.p2) * ((p.p957 * 1.0 / (s.v[30])) * s.v[46]))), s.ad_value(106), A::mul(A::sub(s.ad_value(693), s.ad_value(692)), A::add(A::offset(s.ad_value(693), 1.0), s.ad_value(692)))), 175);
            s.store_add(188, 672, 188);
            s.store_scalar(696, (p.p785 * p.p1062));
            s.store_scalar(697, (p.p799 * p.p1062));
            s.store_scalar(698, (p.p800 * p.p1062));
            s.store_sub_from_scalar_ad(694, s.v[30], A::scale(s.ad_value(359), 2.0));
            s.store_square(695, 694);
            s.store_mul_scaled_ad_rhs(367, 108, 1.0 / (1.60219e-19), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(613)));
            s.store_mul3_affine_lhs(366, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 692);
            s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(672)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);
            s.store_mul3_affine_lhs(737, 108, 672, 1.60219e-19, 0.0, 672);
            s.store_add_ad(738, A::add_scaled_product(s.ad_value(696), 1.0, s.ad_value(697), s.ad_value(366), 1.0), A::mul3(s.ad_value(698), s.ad_value(366), s.ad_value(366)));
            s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));
            s.store_scaled_mul(740, 696, 108, 1.60219e-19);
            s.store_mul3_affine_lhs(365, 691, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 693);
            s.store_mul_ln_ad_rhs(13, 696, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));
            s.store_mul_sub_rhs(14, 697, 365, 366);
            s.store_mul_scaled_ad_rhs(15, 698, 0.5, A::sub(A::square(s.ad_value(365)), A::square(s.ad_value(366))));
            s.store_scale(16, 695, (10000000000.0 * (p.p957 * p.p2)));
            s.store_ad_value(368, A::add_scaled_product(A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0));
            s.store_mul3_affine_lhs(17, 694, 367, ((p.p957 * p.p2) * 10000000000.0), 0.0, 367);
            s.store_mul_ad_product_lhs(369, A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(672), 672);
            s.store_add(18, 369, 368);
        }

        s.b[1603] = (s.v[18] > 0.0);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if (s.b[1588] && s.b[1603]) {
            s.store_ad_value(19, A::div_scaled_product(s.ad_value(368), s.ad_value(369), 1.0, s.ad_value(18), 1.0));
            s.store_offset_scaled_ad(20, A::powf(A::sub(s.ad_value(693), s.ad_value(692)), p.p1064), p.p1063, 1.0);
            s.store_div(699, 19, 20);
        }

        if (s.b[1588] && (!s.b[1603])) {
            s.store_scalar(699, 0.0);
        }

        s.b[1604] = (s.v[57] > 0.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if s.b[1604] {
            s.store_scaled_mul(785, 187, 196, p.p29);
            s.store_scaled_mul(786, 187, 197, p.p29);
        }

        if (!s.b[1604]) {
            s.store_scaled_mul(785, 187, 197, p.p29);
            s.store_scaled_mul(786, 187, 196, p.p29);
        }

        s.b[1605] = ((p.p1094 == 1.0) && (p.p1095 == 1.0));
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if s.b[1605] {
            s.store_add(221, 221, 774);
            s.store_add(224, 224, 775);
        }

        s.b[1606] = (p.p1096 == 1.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if (s.b[1605] && s.b[1606]) {
            s.store_add(221, 221, 776);
            s.store_add(223, 223, 777);
        }

        s.store_scaled_mul(787, 187, 198, p.p29);

        s.b[1609] = (p.p47 != 0.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        s.b[1610] = (p.p46 != 0.0);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        s.b[1611] = (s.v[57] > 0.0);
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        s.b[1612] = ((p.p42 != 2.0) && (s.v[240] > 0.0));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if s.b[1612] {
            s.store_div_from_scalar(372, 1.0, 242);
        }

        s.b[1613] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (s.b[1612] && s.b[1613]) {
            s.store_div_from_scalar(374, 1.0, 759);
        }

        s.b[1614] = ((p.p42 != 2.0) && (s.v[239] > 0.0));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if s.b[1614] {
            s.store_div_from_scalar(371, 1.0, 241);
        }

        s.b[1615] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0));
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (s.b[1614] && s.b[1615]) {
            s.store_div_from_scalar(373, 1.0, 761);
        }

        s.b[1616] = (p.p7 == 0.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        s.b[1619] = (p.p7 == 2.0);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((!s.b[1616]) && s.b[1619]) {
            s.copy_ad(1617, 254);
            s.store_ad_value(1618, A::div_scaled_product(s.ad_value(254), s.ad_value(254), 1.0, s.ad_value(252), 1.0));
        }

        if ((!s.b[1616]) && (!s.b[1619])) {
            s.copy_ad(1617, 252);
            s.copy_ad(1618, 252);
        }

        s.b[1620] = (p.p7 == 3.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        s.b[1621] = ((p.p49 != 0.0) && (p.p909 > 0.0));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if s.b[1621] {
            s.store_mul_voltage_ad(749, A::mul3(s.ad_value(187), s.ad_value(57), s.ad_value(188)), ctx, nodes, Some(5), Some(7));
        }

        s.b[1622] = ((p.p42 != 2.0) && (s.v[240] > 0.0));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        s.b[1623] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0));
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((s.b[1621] && s.b[1622]) && s.b[1623]) {
            s.store_ad_value(749, A::add_scaled_value_products(s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), s.ad_value(372), 1.0, A::square(A::voltage(ctx, nodes, Some(6), Some(5))), s.ad_value(374), 1.0));
        }

        if ((s.b[1621] && s.b[1622]) && (!s.b[1623])) {
            s.store_ad_value(749, A::add_scaled_product(s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(0), Some(6))), s.ad_value(372), 1.0));
        }

        s.b[1624] = ((p.p42 != 2.0) && (s.v[239] > 0.0));
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        s.b[1625] = (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((s.b[1621] && s.b[1624]) && s.b[1625]) {
            s.store_ad_value(749, A::add_scaled_value_products(s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), s.ad_value(371), 1.0, A::square(A::voltage(ctx, nodes, Some(8), Some(7))), s.ad_value(373), 1.0));
        }

        if ((s.b[1621] && s.b[1624]) && (!s.b[1625])) {
            s.store_ad_value(749, A::add_scaled_product(s.ad_value(749), 1.0, A::square(A::voltage(ctx, nodes, Some(2), Some(8))), s.ad_value(371), 1.0));
        }

        s.b[1626] = (p.p8 != 0.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        s.b[1627] = (p.p8 != 0.0);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        s.b[1628] = (p.p1097 == 0.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        s.b[1629] = ((p.p8 != 0.0) && (p.p1097 == 1.0));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        s.b[1630] = ((p.p8 != 0.0) && (p.p1097 == 1.0));
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[485] = 0.0;

        s.v[466] = 0.0;

        s.v[535] = 0.0;

        s.v[505] = 0.0;

        s.v[512] = 0.0;

        s.v[510] = 0.0;

        s.v[467] = 0.0;

        s.v[649] = 0.0;

        s.v[661] = 0.0;

        s.v[669] = 0.0;

        s.v[606] = 0.0;

        s.v[610] = 0.0;

        s.v[616] = 0.0;

        s.v[620] = 0.0;

        s.v[624] = 0.0;

        s.v[628] = 0.0;

        s.v[634] = 0.0;

        s.v[638] = 0.0;

        s.v[491] = 0.0;

        s.v[540] = 0.0;

        s.v[414] = 0.0;

        s.v[400] = 0.0;

        s.v[406] = 0.0;

        s.v[501] = 0.0;

        s.v[650] = 0.0;

        s.v[670] = 0.0;

        s.v[607] = 0.0;

        s.v[613] = 0.0;

        s.v[617] = 0.0;

        s.v[621] = 0.0;

        s.v[625] = 0.0;

        s.v[631] = 0.0;

        s.v[635] = 0.0;

        s.v[639] = 0.0;

        s.v[762] = 1.0;

        s.v[421] = 0.0;

        s.v[518] = 0.0;

        s.v[498] = 0.0;

        s.v[515] = 0.0;

        s.v[509] = 0.0;

        s.v[410] = 0.0;

        s.v[688] = 0.0;

        s.v[690] = 0.0;

        s.v[671] = 0.0;

        s.v[608] = 0.0;

        s.v[614] = 0.0;

        s.v[618] = 0.0;

        s.v[622] = 0.0;

        s.v[626] = 0.0;

        s.v[632] = 0.0;

        s.v[636] = 0.0;

        s.v[759] = 0.0;

        s.v[763] = 1.0;

        s.v[460] = 0.0;

        s.v[165] = 0.0;

        s.v[398] = 0.0;

        s.v[402] = 0.0;

        s.v[404] = 0.0;

        s.v[461] = 0.0;

        s.v[689] = 0.0;

        s.v[605] = 0.0;

        s.v[609] = 0.0;

        s.v[615] = 0.0;

        s.v[619] = 0.0;

        s.v[623] = 0.0;

        s.v[627] = 0.0;

        s.v[633] = 0.0;

        s.v[637] = 0.0;

        s.v[761] = 0.0;

        s.v[629] = 0.0;

        s.v[630] = 0.0;

        s.v[247] = 0.0;

        s.v[246] = 0.0;

        s.v[249] = 0.0;

        s.v[248] = 0.0;

        s.v[782] = 1.0;

        s.v[783] = 1.0;

        s.v[372] = 0.0;

        s.v[371] = 0.0;

        s.v[374] = 0.0;

        s.v[373] = 0.0;

        s.v[67] = 0.0;

        s.v[71] = 0.0;

        s.v[750] = 0.0;

        s.v[147] = 0.0;

        s.v[183] = 0.0;

        s.v[416] = 0.0;

        s.v[552] = 0.0;

        s.v[557] = 0.0;

        s.v[760] = 0.0;

        s.b[859] = (p.p39 == 1.0);
        s.v[859] = if s.b[859] { 1.0 } else { 0.0 };

        if s.b[859] {
            s.store_scalar(187, 1.0);
        }

        if (!s.b[859]) {
            s.store_scalar(187, (-1.0));
        }

        s.v[26] = (p.p110 * 8.85418e-12);

        s.v[27] = (p.p111 * 8.85418e-12);

        s.v[46] = ((p.p111 * 8.85418e-12) / p.p77);

        s.v[47] = (p.p110 / p.p111);

        s.b[860] = (!param_given[78]);
        s.v[860] = if s.b[860] { 1.0 } else { 0.0 };

        if s.b[860] {
            s.store_scalar(229, (((p.p77 * p.p111) / 3.9) - p.p79));
        }

        if (!s.b[860]) {
            s.store_scalar(229, p.p78);
        }

        s.v[99] = (p.p0 * p.p52);

        s.v[101] = (p.p1 * p.p53);

        s.v[98] = (s.v[99] + p.p54);

        s.v[456] = (s.v[101] / p.p2);

        s.v[100] = (s.v[456] + p.p56);

        s.v[457] = ((s.v[98]) as f64).powf((-p.p61));

        s.v[458] = ((s.v[100]) as f64).powf((-p.p62));

        s.v[459] = (s.v[457] * s.v[458]);

        s.v[39] = (((p.p57 + (p.p58 * s.v[457])) + (p.p59 * s.v[458])) + (p.p60 * s.v[459]));

        s.v[463] = ((s.v[98]) as f64).powf((-p.p67));

        s.v[464] = ((s.v[100]) as f64).powf((-p.p68));

        s.v[465] = (s.v[463] * s.v[464]);

        s.v[40] = (((p.p63 + (p.p64 * s.v[463])) + (p.p65 * s.v[464])) + (p.p66 * s.v[465]));

        s.v[30] = (s.v[98] - (2.0 * s.v[39]));

        s.v[29] = (s.v[100] - (2.0 * s.v[40]));

        s.v[43] = (((p.p69 + (p.p70 * s.v[457])) + (p.p71 * s.v[458])) + (p.p72 * s.v[459]));

        s.v[44] = (((p.p73 + (p.p74 * s.v[463])) + (p.p75 * s.v[464])) + (p.p76 * s.v[465]));

        s.v[34] = (s.v[98] - (2.0 * s.v[43]));

        s.v[33] = (s.v[100] - (2.0 * s.v[44]));

        s.v[45] = (((p.p138 + (p.p74 / ((s.v[98]) as f64).powf(p.p67))) + (p.p75 / ((s.v[100]) as f64).powf(p.p68))) + ((p.p76 / ((s.v[98]) as f64).powf(p.p67)) / ((s.v[100]) as f64).powf(p.p68)));

        s.v[35] = (s.v[100] - (2.0 * s.v[45]));

        s.v[469] = (1e-6 / s.v[30]);

        s.v[470] = (1e-6 / s.v[29]);

        s.v[472] = (1e-6 / s.v[34]);

        s.v[473] = (1e-6 / s.v[33]);

        s.v[474] = (1e-6 / p.p51);

        s.v[475] = (1e-6 / p.p55);

        s.v[471] = (s.v[469] * s.v[470]);

        s.v[460] = s.v[457];

        s.v[466] = s.v[463];

        s.b[872] = (p.p818 != 0.0);
        s.v[872] = if s.b[872] { 1.0 } else { 0.0 };

        s.b[873] = (p.p818 <= (-s.v[98]));
        s.v[873] = if s.b[873] { 1.0 } else { 0.0 };

        if (s.b[872] && (!s.b[873])) {
            s.store_scalar(460, (((s.v[98] + p.p818)) as f64).powf((-p.p61)));
            s.store_scalar(466, (((s.v[98] + p.p818)) as f64).powf((-p.p67)));
        }

        s.v[461] = s.v[458];

        s.v[467] = s.v[464];

        s.b[874] = (p.p819 != 0.0);
        s.v[874] = if s.b[874] { 1.0 } else { 0.0 };

        s.b[875] = (p.p819 <= (-s.v[100]));
        s.v[875] = if s.b[875] { 1.0 } else { 0.0 };

        if (s.b[874] && (!s.b[875])) {
            s.store_scalar(461, (((s.v[100] + p.p819)) as f64).powf((-p.p62)));
            s.store_scalar(467, (((s.v[100] + p.p819)) as f64).powf((-p.p68)));
        }

        s.store_mul(462, 460, 461);

        s.store_ad_value(41, A::add_scaled_inputs3_offset(s.ad_value(460), p.p58, s.ad_value(461), p.p59, s.ad_value(462), p.p60, p.p57));

        s.store_mul(468, 466, 467);

        s.store_ad_value(42, A::add_scaled_inputs3_offset(s.ad_value(466), p.p64, s.ad_value(467), p.p65, s.ad_value(468), p.p66, p.p63));

        s.store_offset_sub_from_scalar_ad(32, s.v[98], A::scale(s.ad_value(41), 2.0), p.p818);

        s.store_offset_sub_from_scalar_ad(31, s.v[100], A::scale(s.ad_value(42), 2.0), p.p819);

        s.b[878] = (p.p817 == 1.0);
        s.v[878] = if s.b[878] { 1.0 } else { 0.0 };

        if s.b[878] {
            s.store_div_from_scalar(476, 1e-6, 32);
            s.store_div_from_scalar(477, 1e-6, 31);
        }

        if (!s.b[878]) {
            s.store_div_from_scalar(476, 1.0, 32);
            s.store_div_from_scalar(477, 1.0, 31);
        }

        s.store_mul(478, 476, 477);

        s.store_ad_value(482, A::add_scaled_inputs3_offset(s.ad_value(476), p.p117, s.ad_value(477), p.p118, s.ad_value(478), p.p119, p.p116));

        s.store_ad_value(549, A::add_scaled_inputs3_offset(s.ad_value(476), p.p127, s.ad_value(477), p.p128, s.ad_value(478), p.p129, p.p126));

        s.store_ad_value(480, A::add_scaled_inputs3_offset(s.ad_value(476), p.p140, s.ad_value(477), p.p141, s.ad_value(478), p.p142, p.p139));

        s.store_ad_value(481, A::add_scaled_inputs3_offset(s.ad_value(476), p.p89, s.ad_value(477), p.p90, s.ad_value(478), p.p91, p.p80));

        s.store_ad_value(550, A::add_scaled_inputs3_offset(s.ad_value(476), p.p101, s.ad_value(477), p.p102, s.ad_value(478), p.p103, p.p92));

        s.store_ad_value(479, A::add_scaled_inputs3_offset(s.ad_value(476), p.p105, s.ad_value(477), p.p106, s.ad_value(478), p.p107, p.p104));

        s.store_ad_value(483, A::add_scaled_inputs3_offset(s.ad_value(476), p.p210, s.ad_value(477), p.p211, s.ad_value(478), p.p212, p.p209));

        s.store_ad_value(488, A::add_scaled_inputs3_offset(s.ad_value(476), p.p220, s.ad_value(477), p.p221, s.ad_value(478), p.p222, p.p213));

        s.store_ad_value(484, A::add_scaled_inputs3_offset(s.ad_value(476), p.p226, s.ad_value(477), p.p227, s.ad_value(478), p.p228, p.p223));

        s.store_ad_value(487, A::add_scaled_inputs3_offset(s.ad_value(476), p.p236, s.ad_value(477), p.p237, s.ad_value(478), p.p238, p.p233));

        s.store_ad_value(116, A::add_scaled_inputs3_offset(s.ad_value(476), p.p144, s.ad_value(477), p.p145, s.ad_value(478), p.p146, p.p143));

        s.store_ad_value(117, A::add_scaled_inputs3_offset(s.ad_value(476), p.p148, s.ad_value(477), p.p149, s.ad_value(478), p.p150, p.p147));

        s.store_ad_value(118, A::add_scaled_inputs3_offset(s.ad_value(476), p.p152, s.ad_value(477), p.p153, s.ad_value(478), p.p154, p.p151));

        s.store_ad_value(119, A::add_scaled_inputs3_offset(s.ad_value(476), p.p156, s.ad_value(477), p.p157, s.ad_value(478), p.p158, p.p155));

        s.store_ad_value(120, A::add_scaled_inputs3_offset(s.ad_value(476), p.p160, s.ad_value(477), p.p161, s.ad_value(478), p.p162, p.p159));

        s.store_ad_value(121, A::add_scaled_inputs3_offset(s.ad_value(476), p.p164, s.ad_value(477), p.p165, s.ad_value(478), p.p166, p.p163));

        s.store_ad_value(494, A::add_scaled_inputs3_offset(s.ad_value(476), p.p202, s.ad_value(477), p.p203, s.ad_value(478), p.p204, p.p195));

        s.store_ad_value(495, A::add_scaled_inputs3_offset(s.ad_value(476), p.p192, s.ad_value(477), p.p193, s.ad_value(478), p.p194, p.p185));

        s.store_ad_value(538, A::add_scaled_inputs3_offset(s.ad_value(476), p.p113, s.ad_value(477), p.p114, s.ad_value(478), p.p115, p.p112));

        s.store_ad_value(489, A::add_scaled_inputs3_offset(s.ad_value(476), p.p168, s.ad_value(477), p.p169, s.ad_value(478), p.p170, p.p167));

        s.store_ad_value(490, A::add_scaled_inputs3_offset(s.ad_value(476), p.p172, s.ad_value(477), p.p173, s.ad_value(478), p.p174, p.p171));

        s.store_ad_value(493, A::add_scaled_inputs3_offset(s.ad_value(476), p.p182, s.ad_value(477), p.p183, s.ad_value(478), p.p184, p.p180));

        s.store_ad_value(496, A::add_scaled_inputs3_offset(s.ad_value(476), p.p254, s.ad_value(477), p.p255, s.ad_value(478), p.p256, p.p253));

        s.store_ad_value(497, A::add_scaled_inputs3_offset(s.ad_value(476), p.p276, s.ad_value(477), p.p277, s.ad_value(478), p.p278, p.p273));

        s.store_ad_value(504, A::add_scaled_inputs3_offset(s.ad_value(476), p.p291, s.ad_value(477), p.p292, s.ad_value(478), p.p293, p.p284));

        s.store_ad_value(508, A::add_scaled_inputs3_offset(s.ad_value(476), p.p311, s.ad_value(477), p.p312, s.ad_value(478), p.p313, p.p308));

        s.store_ad_value(507, A::add_scaled_inputs3_offset(s.ad_value(476), p.p299, s.ad_value(477), p.p300, s.ad_value(478), p.p301, p.p298));

        s.store_ad_value(511, A::add_scaled_inputs3_offset(s.ad_value(476), p.p319, s.ad_value(477), p.p320, s.ad_value(478), p.p321, p.p318));

        s.store_ad_value(514, A::add_scaled_inputs3_offset(s.ad_value(476), p.p333, s.ad_value(477), p.p334, s.ad_value(478), p.p335, p.p326));

        s.store_ad_value(539, A::add_scaled_inputs3_offset(s.ad_value(476), p.p343, s.ad_value(477), p.p344, s.ad_value(478), p.p345, p.p340));

        s.store_ad_value(542, A::add_scaled_inputs3_offset(s.ad_value(476), p.p354, s.ad_value(477), p.p355, s.ad_value(478), p.p356, p.p351));

        s.store_ad_value(531, A::add_scaled_inputs3_offset(s.ad_value(476), p.p394, s.ad_value(477), p.p395, s.ad_value(478), p.p396, p.p393));

        s.store_ad_value(530, A::add_scaled_inputs3_offset(s.ad_value(476), p.p404, s.ad_value(477), p.p405, s.ad_value(478), p.p406, p.p403));

        s.store_ad_value(526, A::add_scaled_inputs3_offset(s.ad_value(476), p.p376, s.ad_value(477), p.p377, s.ad_value(478), p.p378, p.p375));

        s.store_ad_value(543, A::add_scaled_inputs3_offset(s.ad_value(476), p.p380, s.ad_value(477), p.p381, s.ad_value(478), p.p382, p.p379));

        s.store_ad_value(527, A::add_scaled_inputs3_offset(s.ad_value(476), p.p386, s.ad_value(477), p.p387, s.ad_value(478), p.p388, p.p385));

        s.store_ad_value(529, A::add_scaled_inputs3_offset(s.ad_value(476), p.p390, s.ad_value(477), p.p391, s.ad_value(478), p.p392, p.p389));

        s.store_ad_value(528, A::add_scaled_inputs3_offset(s.ad_value(476), p.p400, s.ad_value(477), p.p401, s.ad_value(478), p.p402, p.p399));

        s.store_ad_value(532, A::add_scaled_inputs3_offset(s.ad_value(476), p.p416, s.ad_value(477), p.p417, s.ad_value(478), p.p418, p.p413));

        s.store_ad_value(533, A::add_scaled_inputs3_offset(s.ad_value(476), p.p410, s.ad_value(477), p.p411, s.ad_value(478), p.p412, p.p409));

        s.store_ad_value(534, A::add_scaled_inputs3_offset(s.ad_value(476), p.p435, s.ad_value(477), p.p436, s.ad_value(478), p.p437, p.p434));

        s.store_ad_value(517, A::add_scaled_inputs3_offset(s.ad_value(476), p.p463, s.ad_value(477), p.p464, s.ad_value(478), p.p465, p.p460));

        s.store_ad_value(520, A::add_scaled_inputs3_offset(s.ad_value(476), p.p471, s.ad_value(477), p.p472, s.ad_value(478), p.p473, p.p470));

        s.store_ad_value(521, A::add_scaled_inputs3_offset(s.ad_value(476), p.p358, s.ad_value(477), p.p359, s.ad_value(478), p.p360, p.p357));

        s.store_ad_value(522, A::add_scaled_inputs3_offset(s.ad_value(476), p.p362, s.ad_value(477), p.p363, s.ad_value(478), p.p364, p.p361));

        s.store_ad_value(523, A::add_scaled_inputs3_offset(s.ad_value(476), p.p366, s.ad_value(477), p.p367, s.ad_value(478), p.p368, p.p365));

        s.store_ad_value(524, A::add_scaled_inputs3_offset(s.ad_value(476), p.p371, s.ad_value(477), p.p372, s.ad_value(478), p.p373, p.p370));

        s.store_ad_value(525, A::add_scaled_inputs3_offset(s.ad_value(476), p.p481, s.ad_value(477), p.p482, s.ad_value(478), p.p483, p.p478));

        s.store_ad_value(537, A::add_scaled_inputs3_offset(s.ad_value(476), p.p475, s.ad_value(477), p.p476, s.ad_value(478), p.p477, p.p474));

        s.store_ad_value(500, A::add_scaled_inputs3_offset(s.ad_value(476), p.p240, s.ad_value(477), p.p241, s.ad_value(478), p.p242, p.p239));

        s.store_ad_value(164, A::add_scaled_inputs3_offset(s.ad_value(476), p.p420, s.ad_value(477), p.p421, s.ad_value(478), p.p422, p.p419));

        s.store_ad_value(503, A::add_scaled_inputs3_offset(s.ad_value(476), p.p260, s.ad_value(477), p.p261, s.ad_value(478), p.p262, p.p259));

        s.store_ad_value(544, A::add_scaled_inputs3_offset(s.ad_value(476), p.p667, s.ad_value(477), p.p668, s.ad_value(478), p.p669, p.p666));

        s.store_ad_value(545, A::add_scaled_inputs3_offset(s.ad_value(476), p.p675, s.ad_value(477), p.p676, s.ad_value(478), p.p677, p.p674));

        s.store_ad_value(546, A::add_scaled_inputs3_offset(s.ad_value(476), p.p679, s.ad_value(477), p.p680, s.ad_value(478), p.p681, p.p678));

        s.store_ad_value(547, A::add_scaled_inputs3_offset(s.ad_value(476), p.p683, s.ad_value(477), p.p684, s.ad_value(478), p.p685, p.p682));

        s.store_ad_value(548, A::add_scaled_inputs3_offset(s.ad_value(476), p.p687, s.ad_value(477), p.p688, s.ad_value(478), p.p689, p.p686));

        s.store_ad_value(551, A::add_scaled_inputs3_offset(s.ad_value(476), p.p489, s.ad_value(477), p.p490, s.ad_value(478), p.p491, p.p484));

        s.store_ad_value(554, A::add_scaled_inputs3_offset(s.ad_value(476), p.p497, s.ad_value(477), p.p498, s.ad_value(478), p.p499, p.p494));

        s.store_ad_value(578, A::add_scaled_inputs3_offset(s.ad_value(476), p.p936, s.ad_value(477), p.p937, s.ad_value(478), p.p938, p.p935));

        s.store_ad_value(579, A::add_scaled_inputs3_offset(s.ad_value(476), p.p940, s.ad_value(477), p.p941, s.ad_value(478), p.p942, p.p939));

        s.store_ad_value(580, A::add_scaled_inputs3_offset(s.ad_value(476), p.p944, s.ad_value(477), p.p945, s.ad_value(478), p.p946, p.p943));

        s.store_ad_value(559, A::add_scaled_inputs3_offset(s.ad_value(476), p.p633, s.ad_value(477), p.p634, s.ad_value(478), p.p635, p.p630));

        s.store_ad_value(560, A::add_scaled_inputs3_offset(s.ad_value(476), p.p637, s.ad_value(477), p.p638, s.ad_value(478), p.p639, p.p636));

        s.store_ad_value(561, A::add_scaled_inputs3_offset(s.ad_value(476), p.p641, s.ad_value(477), p.p642, s.ad_value(478), p.p643, p.p640));

        s.store_ad_value(562, A::add_scaled_inputs3_offset(s.ad_value(476), p.p645, s.ad_value(477), p.p646, s.ad_value(478), p.p647, p.p644));

        s.store_ad_value(563, A::add_scaled_inputs3_offset(s.ad_value(476), p.p651, s.ad_value(477), p.p652, s.ad_value(478), p.p653, p.p648));

        s.store_ad_value(564, A::add_scaled_inputs3_offset(s.ad_value(476), p.p655, s.ad_value(477), p.p656, s.ad_value(478), p.p657, p.p654));

        s.store_ad_value(565, A::add_scaled_inputs3_offset(s.ad_value(476), p.p659, s.ad_value(477), p.p660, s.ad_value(478), p.p661, p.p658));

        s.store_ad_value(566, A::add_scaled_inputs3_offset(s.ad_value(476), p.p663, s.ad_value(477), p.p664, s.ad_value(478), p.p665, p.p662));

        s.store_ad_value(567, A::add_scaled_inputs3_offset(s.ad_value(476), p.p825, s.ad_value(477), p.p826, s.ad_value(478), p.p827, p.p824));

        s.store_ad_value(568, A::add_scaled_inputs3_offset(s.ad_value(476), p.p830, s.ad_value(477), p.p831, s.ad_value(478), p.p832, p.p829));

        s.store_ad_value(569, A::add_scaled_inputs3_offset(s.ad_value(476), p.p835, s.ad_value(477), p.p836, s.ad_value(478), p.p837, p.p834));

        s.store_ad_value(570, A::add_scaled_inputs3_offset(s.ad_value(476), p.p839, s.ad_value(477), p.p840, s.ad_value(478), p.p841, p.p838));

        s.store_ad_value(577, A::add_scaled_inputs3_offset(s.ad_value(476), p.p844, s.ad_value(477), p.p845, s.ad_value(478), p.p846, p.p843));

        s.store_ad_value(571, A::add_scaled_inputs3_offset(s.ad_value(476), p.p848, s.ad_value(477), p.p849, s.ad_value(478), p.p850, p.p847));

        s.store_ad_value(572, A::add_scaled_inputs3_offset(s.ad_value(476), p.p853, s.ad_value(477), p.p854, s.ad_value(478), p.p855, p.p852));

        s.store_ad_value(573, A::add_scaled_inputs3_offset(s.ad_value(476), p.p857, s.ad_value(477), p.p858, s.ad_value(478), p.p859, p.p856));

        s.store_ad_value(574, A::add_scaled_inputs3_offset(s.ad_value(476), p.p863, s.ad_value(477), p.p864, s.ad_value(478), p.p865, p.p862));

        s.store_ad_value(575, A::add_scaled_inputs3_offset(s.ad_value(476), p.p878, s.ad_value(477), p.p879, s.ad_value(478), p.p880, p.p877));

        s.store_ad_value(576, A::add_scaled_inputs3_offset(s.ad_value(476), p.p886, s.ad_value(477), p.p887, s.ad_value(478), p.p888, p.p885));

        s.store_ad_value(581, A::add_scaled_inputs3_offset(s.ad_value(476), p.p564, s.ad_value(477), p.p565, s.ad_value(478), p.p566, p.p537));

        s.store_ad_value(582, A::add_scaled_inputs3_offset(s.ad_value(476), p.p567, s.ad_value(477), p.p568, s.ad_value(478), p.p569, p.p538));

        s.store_ad_value(583, A::add_scaled_inputs3_offset(s.ad_value(476), p.p570, s.ad_value(477), p.p571, s.ad_value(478), p.p572, p.p539));

        s.store_ad_value(584, A::add_scaled_inputs3_offset(s.ad_value(476), p.p573, s.ad_value(477), p.p574, s.ad_value(478), p.p575, p.p540));

        s.store_ad_value(585, A::add_scaled_inputs3_offset(s.ad_value(476), p.p576, s.ad_value(477), p.p577, s.ad_value(478), p.p578, p.p541));

        s.store_ad_value(586, A::add_scaled_inputs3_offset(s.ad_value(476), p.p579, s.ad_value(477), p.p580, s.ad_value(478), p.p581, p.p533));

        s.store_ad_value(587, A::add_scaled_inputs3_offset(s.ad_value(476), p.p582, s.ad_value(477), p.p583, s.ad_value(478), p.p584, p.p534));

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_ad_value(588, A::add_scaled_inputs3_offset(s.ad_value(476), p.p585, s.ad_value(477), p.p586, s.ad_value(478), p.p587, p.p535));

        s.store_ad_value(589, A::add_scaled_inputs3_offset(s.ad_value(476), p.p588, s.ad_value(477), p.p589, s.ad_value(478), p.p590, p.p536));

        s.store_ad_value(590, A::add_scaled_inputs3_offset(s.ad_value(476), p.p591, s.ad_value(477), p.p592, s.ad_value(478), p.p593, p.p542));

        s.store_ad_value(591, A::add_scaled_inputs3_offset(s.ad_value(476), p.p594, s.ad_value(477), p.p595, s.ad_value(478), p.p596, p.p543));

        s.store_ad_value(592, A::add_scaled_inputs3_offset(s.ad_value(476), p.p597, s.ad_value(477), p.p598, s.ad_value(478), p.p599, p.p544));

        s.store_ad_value(593, A::add_scaled_inputs3_offset(s.ad_value(476), p.p600, s.ad_value(477), p.p601, s.ad_value(478), p.p602, p.p545));

        s.store_ad_value(594, A::add_scaled_inputs3_offset(s.ad_value(476), p.p603, s.ad_value(477), p.p604, s.ad_value(478), p.p605, p.p546));

        s.store_ad_value(595, A::add_scaled_inputs3_offset(s.ad_value(476), p.p606, s.ad_value(477), p.p607, s.ad_value(478), p.p608, p.p547));

        s.store_ad_value(596, A::add_scaled_inputs3_offset(s.ad_value(476), p.p609, s.ad_value(477), p.p610, s.ad_value(478), p.p611, p.p548));

        s.store_ad_value(597, A::add_scaled_inputs3_offset(s.ad_value(476), p.p612, s.ad_value(477), p.p613, s.ad_value(478), p.p614, p.p549));

        s.store_ad_value(598, A::add_scaled_inputs3_offset(s.ad_value(476), p.p615, s.ad_value(477), p.p616, s.ad_value(478), p.p617, p.p550));

        s.store_ad_value(599, A::add_scaled_inputs3_offset(s.ad_value(476), p.p618, s.ad_value(477), p.p619, s.ad_value(478), p.p620, p.p553));

        s.store_ad_value(454, A::add_scaled_inputs3_offset(s.ad_value(476), p.p870, s.ad_value(477), p.p871, s.ad_value(478), p.p872, p.p867));

        s.store_ad_value(455, A::add_scaled_inputs3_offset(s.ad_value(476), p.p874, s.ad_value(477), p.p875, s.ad_value(478), p.p876, p.p873));

        s.store_ad_value(453, A::add_scaled_inputs3_offset(s.ad_value(476), p.p430, s.ad_value(477), p.p431, s.ad_value(478), p.p432, p.p425));

        s.store_ad_value(148, A::add_scaled_inputs3_offset(s.ad_value(476), p.p445, s.ad_value(477), p.p446, s.ad_value(478), p.p447, p.p444));

        s.store_ad_value(149, A::add_scaled_inputs3_offset(s.ad_value(476), p.p449, s.ad_value(477), p.p450, s.ad_value(478), p.p451, p.p448));

        s.store_ad_value(151, A::add_scaled_inputs3_offset(s.ad_value(476), p.p453, s.ad_value(477), p.p454, s.ad_value(478), p.p455, p.p452));

        s.store_ad_value(152, A::add_scaled_inputs3_offset(s.ad_value(476), p.p457, s.ad_value(477), p.p458, s.ad_value(478), p.p459, p.p456));

        s.store_ad_value(605, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1047, s.ad_value(477), p.p1048, s.ad_value(478), p.p1049, p.p1046));

        s.store_ad_value(606, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1055, s.ad_value(477), p.p1056, s.ad_value(478), p.p1057, p.p1054));

        s.store_ad_value(607, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1051, s.ad_value(477), p.p1052, s.ad_value(478), p.p1053, p.p1050));

        s.store_ad_value(608, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1059, s.ad_value(477), p.p1060, s.ad_value(478), p.p1061, p.p1058));

        s.store_ad_value(612, A::add_scaled_inputs3_offset(s.ad_value(476), p.p967, s.ad_value(477), p.p968, s.ad_value(478), p.p969, p.p966));

        s.store_ad_value(686, A::add_scaled_inputs3_offset(s.ad_value(476), p.p963, s.ad_value(477), p.p964, s.ad_value(478), p.p965, p.p962));

        s.store_ad_value(613, A::add_scaled_inputs3_offset(s.ad_value(476), p.p971, s.ad_value(477), p.p972, s.ad_value(478), p.p973, p.p970));

        s.store_ad_value(614, A::add_scaled_inputs3_offset(s.ad_value(476), p.p975, s.ad_value(477), p.p976, s.ad_value(478), p.p977, p.p974));

        s.store_ad_value(615, A::add_scaled_inputs3_offset(s.ad_value(476), p.p979, s.ad_value(477), p.p980, s.ad_value(478), p.p981, p.p978));

        s.store_ad_value(616, A::add_scaled_inputs3_offset(s.ad_value(476), p.p983, s.ad_value(477), p.p984, s.ad_value(478), p.p985, p.p982));

        s.store_ad_value(617, A::add_scaled_inputs3_offset(s.ad_value(476), p.p987, s.ad_value(477), p.p988, s.ad_value(478), p.p989, p.p986));

        s.store_ad_value(618, A::add_scaled_inputs3_offset(s.ad_value(476), p.p991, s.ad_value(477), p.p992, s.ad_value(478), p.p993, p.p990));

        s.store_ad_value(619, A::add_scaled_inputs3_offset(s.ad_value(476), p.p995, s.ad_value(477), p.p996, s.ad_value(478), p.p997, p.p994));

        s.store_ad_value(620, A::add_scaled_inputs3_offset(s.ad_value(476), p.p999, s.ad_value(477), p.p1000, s.ad_value(478), p.p1001, p.p998));

        s.store_ad_value(621, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1003, s.ad_value(477), p.p1004, s.ad_value(478), p.p1005, p.p1002));

        s.store_ad_value(622, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1007, s.ad_value(477), p.p1008, s.ad_value(478), p.p1009, p.p1006));

        s.store_ad_value(623, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1011, s.ad_value(477), p.p1012, s.ad_value(478), p.p1013, p.p1010));

        s.store_ad_value(624, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1018, s.ad_value(477), p.p1019, s.ad_value(478), p.p1020, p.p1017));

        s.store_ad_value(625, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1022, s.ad_value(477), p.p1023, s.ad_value(478), p.p1024, p.p1021));

        s.store_ad_value(629, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1030, s.ad_value(477), p.p1031, s.ad_value(478), p.p1032, p.p1029));

        s.store_ad_value(630, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1026, s.ad_value(477), p.p1027, s.ad_value(478), p.p1028, p.p1025));

        s.store_ad_value(626, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1034, s.ad_value(477), p.p1035, s.ad_value(478), p.p1036, p.p1033));

        s.store_ad_value(627, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1038, s.ad_value(477), p.p1039, s.ad_value(478), p.p1040, p.p1037));

        s.store_ad_value(631, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1070, s.ad_value(477), p.p1071, s.ad_value(478), p.p1072, p.p1069));

        s.store_ad_value(632, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1074, s.ad_value(477), p.p1075, s.ad_value(478), p.p1076, p.p1073));

        s.store_ad_value(634, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1078, s.ad_value(477), p.p1079, s.ad_value(478), p.p1080, p.p1077));

        s.store_ad_value(635, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1082, s.ad_value(477), p.p1083, s.ad_value(478), p.p1084, p.p1081));

        s.store_ad_value(637, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1086, s.ad_value(477), p.p1087, s.ad_value(478), p.p1088, p.p1085));

        s.store_ad_value(638, A::add_scaled_inputs3_offset(s.ad_value(476), p.p1090, s.ad_value(477), p.p1091, s.ad_value(478), p.p1092, p.p1089));

        s.store_ad_value(640, A::add_scaled_inputs3_offset(s.ad_value(476), p.p787, s.ad_value(477), p.p788, s.ad_value(478), p.p789, p.p786));

        s.store_ad_value(641, A::add_scaled_inputs3_offset(s.ad_value(476), p.p795, s.ad_value(477), p.p796, s.ad_value(478), p.p797, p.p794));

        s.store_ad_value(642, A::add_scaled_inputs3_offset(s.ad_value(476), p.p791, s.ad_value(477), p.p792, s.ad_value(478), p.p793, p.p790));

        s.b[879] = (p.p44 != 0.0);
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if s.b[879] {
            s.store_ad_value(485, A::add_scaled_inputs3_offset(s.ad_value(476), p.p230, s.ad_value(477), p.p231, s.ad_value(478), p.p232, p.p229));
            s.store_ad_value(491, A::add_scaled_inputs3_offset(s.ad_value(476), p.p176, s.ad_value(477), p.p177, s.ad_value(478), p.p178, p.p175));
            s.store_ad_value(498, A::add_scaled_inputs3_offset(s.ad_value(476), p.p280, s.ad_value(477), p.p281, s.ad_value(478), p.p282, p.p279));
            s.store_ad_value(505, A::add_scaled_inputs3_offset(s.ad_value(476), p.p295, s.ad_value(477), p.p296, s.ad_value(478), p.p297, p.p294));
            s.store_ad_value(509, A::add_scaled_inputs3_offset(s.ad_value(476), p.p315, s.ad_value(477), p.p316, s.ad_value(478), p.p317, p.p314));
            s.store_ad_value(512, A::add_scaled_inputs3_offset(s.ad_value(476), p.p323, s.ad_value(477), p.p324, s.ad_value(478), p.p325, p.p322));
            s.store_ad_value(515, A::add_scaled_inputs3_offset(s.ad_value(476), p.p337, s.ad_value(477), p.p338, s.ad_value(478), p.p339, p.p336));
            s.store_ad_value(540, A::add_scaled_inputs3_offset(s.ad_value(476), p.p347, s.ad_value(477), p.p348, s.ad_value(478), p.p349, p.p346));
            s.store_ad_value(518, A::add_scaled_inputs3_offset(s.ad_value(476), p.p467, s.ad_value(477), p.p468, s.ad_value(478), p.p469, p.p466));
            s.store_ad_value(501, A::add_scaled_inputs3_offset(s.ad_value(476), p.p250, s.ad_value(477), p.p251, s.ad_value(478), p.p252, p.p249));
            s.store_ad_value(165, A::add_scaled_inputs3_offset(s.ad_value(476), p.p427, s.ad_value(477), p.p428, s.ad_value(478), p.p429, p.p426));
            s.store_ad_value(535, A::add_scaled_inputs3_offset(s.ad_value(476), p.p441, s.ad_value(477), p.p442, s.ad_value(478), p.p443, p.p440));
            s.store_ad_value(552, A::add_scaled_inputs3_offset(s.ad_value(476), p.p526, s.ad_value(477), p.p527, s.ad_value(478), p.p528, p.p525));
            s.store_ad_value(557, A::add_scaled_inputs3_offset(s.ad_value(476), p.p530, s.ad_value(477), p.p531, s.ad_value(478), p.p532, p.p529));
        }

        s.v[12] = ((p.p81 * ((((s.v[469]) as f64).powf(p.p82) - ((s.v[474]) as f64).powf(p.p82))).max(0.0)) + (p.p83 * ((((s.v[469]) as f64).powf(p.p84) - ((s.v[474]) as f64).powf(p.p84))).max(0.0)));

        s.v[13] = ((p.p85 * ((((s.v[470]) as f64).powf(p.p86) - ((s.v[475]) as f64).powf(p.p86))).max(0.0)) + (p.p87 * (((s.v[470] * s.v[469])) as f64).powf(p.p88)));

        s.store_scale(481, 481, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p214 * ((((s.v[469]) as f64).powf(p.p215) - ((s.v[474]) as f64).powf(p.p215))).max(0.0));

        s.v[13] = ((p.p216 * ((((s.v[470]) as f64).powf(p.p217) - ((s.v[475]) as f64).powf(p.p217))).max(0.0)) + (p.p218 * ((s.v[471]) as f64).powf(p.p219)));

        s.store_scale(488, 488, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (1.0 + (p.p224 * ((((s.v[469]) as f64).powf(p.p225) - ((s.v[474]) as f64).powf(p.p225))).max(0.0)));

        s.store_scale(484, 484, s.v[12]);

        s.b[880] = (p.p44 != 0.0);
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

        if s.b[880] {
            s.store_scale(485, 485, s.v[12]);
        }

        s.store_scale(487, 487, (1.0 + (p.p234 * ((((s.v[469]) as f64).powf(p.p235) - ((s.v[474]) as f64).powf(p.p235))).max(0.0))));

        s.store_scale(497, 497, p.p34);

        s.b[881] = (p.p50 != 1.0);
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        s.b[882] = (p.p275 > 0.0);
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if (s.b[881] && s.b[882]) {
            s.store_scale(497, 497, (1.0 - (p.p274 * ((((s.v[469]) as f64).powf(p.p275) - ((s.v[474]) as f64).powf(p.p275))).max(0.0))));
        }

        s.b[883] = (p.p44 != 0.0);
        s.v[883] = if s.b[883] { 1.0 } else { 0.0 };

        if ((s.b[881] && s.b[882]) && s.b[883]) {
            s.store_scale(498, 498, (1.0 - (p.p274 * ((((s.v[469]) as f64).powf(p.p275) - ((s.v[474]) as f64).powf(p.p275))).max(0.0))));
        }

        if (s.b[881] && (!s.b[882])) {
            s.store_scale(497, 497, (1.0 - p.p274));
        }

        s.b[884] = (p.p44 != 0.0);
        s.v[884] = if s.b[884] { 1.0 } else { 0.0 };

        if ((s.b[881] && (!s.b[882])) && s.b[884]) {
            s.store_scale(498, 498, (1.0 - p.p274));
        }

        if (!s.b[881]) {
            let assign3470_ad_e4787: A = A::scale(s.ad_value(497), ((1.0 - (p.p269 * { let limited_exp_arg = ((-s.v[30]) / p.p270); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p271 * { let limited_exp_arg = ((-s.v[30]) / p.p272); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
            s.store_ad_value(497, assign3470_ad_e4787);
        }

        s.b[885] = (p.p44 != 0.0);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if ((!s.b[881]) && s.b[885]) {
            let assign3490_ad_e4815: A = A::scale(s.ad_value(498), ((1.0 - (p.p269 * { let limited_exp_arg = ((-s.v[30]) / p.p270); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - (p.p271 * { let limited_exp_arg = ((-s.v[30]) / p.p272); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
            s.store_ad_value(498, assign3490_ad_e4815);
        }

        s.v[12] = (p.p285 * ((((s.v[469]) as f64).powf(p.p286) - ((s.v[474]) as f64).powf(p.p286))).max(0.0));

        s.v[13] = ((p.p287 * ((((s.v[470]) as f64).powf(p.p288) - ((s.v[475]) as f64).powf(p.p288))).max(0.0)) + (p.p289 * ((s.v[471]) as f64).powf(p.p290)));

        s.store_scale(504, 504, ((1.0 + s.v[12]) + s.v[13]));

        s.b[886] = (p.p44 != 0.0);
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if s.b[886] {
            s.store_scale(505, 505, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[12] = (p.p302 * ((((s.v[469]) as f64).powf(p.p303) - ((s.v[474]) as f64).powf(p.p303))).max(0.0));

        s.v[13] = ((p.p304 * ((((s.v[470]) as f64).powf(p.p305) - ((s.v[475]) as f64).powf(p.p305))).max(0.0)) + (p.p306 * ((s.v[471]) as f64).powf(p.p307)));

        s.store_scale(507, 507, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (1.0 + (p.p309 * ((((s.v[469]) as f64).powf(p.p310) - ((s.v[474]) as f64).powf(p.p310))).max(0.0)));

        s.store_scale(508, 508, s.v[12]);

        s.b[887] = (p.p44 != 0.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if s.b[887] {
            s.store_scale(509, 509, s.v[12]);
        }

        s.v[12] = (p.p327 * ((((s.v[469]) as f64).powf(p.p328) - ((s.v[474]) as f64).powf(p.p328))).max(0.0));

        s.v[13] = ((p.p329 * ((((s.v[470]) as f64).powf(p.p330) - ((s.v[475]) as f64).powf(p.p330))).max(0.0)) + (p.p331 * ((s.v[471]) as f64).powf(p.p332)));

        s.store_scale(514, 514, ((1.0 + s.v[12]) + s.v[13]));

        s.b[888] = (p.p44 != 0.0);
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if s.b[888] {
            s.store_scale(515, 515, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[12] = ((((s.v[469]) as f64).powf(p.p179) - ((s.v[474]) as f64).powf(p.p179))).max(0.0);

        s.store_scale(490, 490, s.v[12]);

        s.b[889] = (p.p44 != 0.0);
        s.v[889] = if s.b[889] { 1.0 } else { 0.0 };

        if s.b[889] {
            s.store_scale(491, 491, s.v[12]);
        }

        s.store_scale(493, 493, ((((s.v[469]) as f64).powf(p.p181) - ((s.v[474]) as f64).powf(p.p181))).max(0.0));

        s.v[12] = (1.0 + (p.p461 * ((((s.v[469]) as f64).powf(p.p462) - ((s.v[474]) as f64).powf(p.p462))).max(0.0)));

        s.store_scale(517, 517, s.v[12]);

        s.b[890] = (p.p44 != 0.0);
        s.v[890] = if s.b[890] { 1.0 } else { 0.0 };

        if s.b[890] {
            s.store_scale(518, 518, s.v[12]);
        }

        s.store_scale(12, 496, (1.0 + (p.p257 * ((((s.v[469]) as f64).powf(p.p258) - ((s.v[474]) as f64).powf(p.p258))).max(0.0))));

        s.store_min_with_scalar(496, 12, 0.5);

        s.store_scale(525, 525, (1.0 + (p.p479 * ((((s.v[469]) as f64).powf(p.p480) - ((s.v[474]) as f64).powf(p.p480))).max(0.0))));

        s.v[12] = (1.0 + (p.p341 * ((((s.v[469]) as f64).powf(p.p342) - ((s.v[474]) as f64).powf(p.p342))).max(0.0)));

        s.store_scale(539, 539, s.v[12]);

        s.store_max_with_scalar(539, 539, 0.0);

        s.b[891] = (p.p44 != 0.0);
        s.v[891] = if s.b[891] { 1.0 } else { 0.0 };

        if s.b[891] {
            s.store_scale(540, 540, s.v[12]);
            s.store_max_with_scalar(540, 540, 0.0);
        }

        s.v[12] = (p.p243 * ((((s.v[469]) as f64).powf(p.p244) - ((s.v[474]) as f64).powf(p.p244))).max(0.0));

        s.v[13] = ((p.p245 * ((((s.v[470]) as f64).powf(p.p246) - ((s.v[475]) as f64).powf(p.p246))).max(0.0)) + (p.p247 * ((s.v[471]) as f64).powf(p.p248)));

        s.store_scale(500, 500, ((1.0 + s.v[12]) + s.v[13]));

        s.b[892] = (p.p44 != 0.0);
        s.v[892] = if s.b[892] { 1.0 } else { 0.0 };

        if s.b[892] {
            s.store_scale(501, 501, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.store_max_with_scalar_ad(164, A::scale(s.ad_value(164), (1.0 + (p.p423 * ((((s.v[469]) as f64).powf(p.p424) - ((s.v[474]) as f64).powf(p.p424))).max(0.0)))), 0.25);

        s.b[893] = (p.p44 != 0.0);
        s.v[893] = if s.b[893] { 1.0 } else { 0.0 };

        if s.b[893] {
            s.store_max_with_scalar_ad(165, A::scale(s.ad_value(165), (1.0 + (p.p423 * ((((s.v[469]) as f64).powf(p.p424) - ((s.v[474]) as f64).powf(p.p424))).max(0.0)))), 0.25);
        }

        s.v[12] = (1.0 + (p.p438 * ((((s.v[469]) as f64).powf(p.p439) - ((s.v[474]) as f64).powf(p.p439))).max(0.0)));

        s.store_scale(534, 534, s.v[12]);

        s.b[894] = (p.p44 != 0.0);
        s.v[894] = if s.b[894] { 1.0 } else { 0.0 };

        if s.b[894] {
            s.store_scale(535, 535, s.v[12]);
        }

        s.v[12] = (p.p485 * ((((s.v[469]) as f64).powf(p.p486) - ((s.v[474]) as f64).powf(p.p486))).max(0.0));

        s.v[13] = (p.p487 * ((((s.v[470]) as f64).powf(p.p488) - ((s.v[475]) as f64).powf(p.p488))).max(0.0));

        s.store_scale(551, 551, ((1.0 + s.v[12]) + s.v[13]));

        s.b[895] = (p.p44 != 0.0);
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if s.b[895] {
            s.store_scale(552, 552, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[13] = (p.p495 * ((((s.v[470]) as f64).powf(p.p496) - ((s.v[475]) as f64).powf(p.p496))).max(0.0));

        s.store_scale(554, 554, (1.0 + s.v[13]));

        s.v[13] = (p.p519 * ((((s.v[470]) as f64).powf(p.p520) - ((s.v[475]) as f64).powf(p.p520))).max(0.0));

        s.v[555] = p.p518;

        s.v[555] = (s.v[555] * (1.0 + s.v[13]));

        s.v[13] = (p.p522 * ((((s.v[470]) as f64).powf(p.p523) - ((s.v[475]) as f64).powf(p.p523))).max(0.0));

        s.v[556] = p.p521;

        s.v[556] = (s.v[556] * (1.0 + s.v[13]));

        s.store_scale(559, 559, ((1.0 + (p.p631 * s.v[469])) + (p.p632 * s.v[470])));

        s.store_scale(563, 563, ((1.0 + (p.p649 * s.v[469])) + (p.p650 * s.v[470])));

        s.store_scale(590, 590, ((1.0 + (p.p557 * s.v[469])) + (p.p558 * s.v[470])));

        s.store_scale(593, 593, ((1.0 + (p.p559 * s.v[469])) + (p.p560 * s.v[470])));

        s.store_scale(596, 596, ((1.0 + (p.p561 * s.v[469])) + (p.p562 * s.v[470])));

        s.v[600] = (p.p556 * (1.0 + (p.p563 * s.v[469])));

        s.v[12] = ((p.p93 * ((((s.v[472]) as f64).powf(p.p94) - ((s.v[474]) as f64).powf(p.p94))).max(0.0)) + (p.p95 * ((((s.v[472]) as f64).powf(p.p96) - ((s.v[474]) as f64).powf(p.p96))).max(0.0)));

        s.v[13] = ((p.p97 * ((((s.v[473]) as f64).powf(p.p98) - ((s.v[475]) as f64).powf(p.p98))).max(0.0)) + (p.p99 * (((s.v[473] * s.v[472])) as f64).powf(p.p100)));

        s.store_scale(550, 550, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p120 * ((((s.v[472]) as f64).powf(p.p121) - ((s.v[474]) as f64).powf(p.p121))).max(0.0));

        s.v[13] = ((p.p122 * ((((s.v[473]) as f64).powf(p.p123) - ((s.v[475]) as f64).powf(p.p123))).max(0.0)) + (p.p124 * ((s.v[471]) as f64).powf(p.p125)));

        s.store_scale(482, 482, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p130 * ((((s.v[472]) as f64).powf(p.p131) - ((s.v[474]) as f64).powf(p.p131))).max(0.0));

        s.v[13] = ((p.p132 * ((((s.v[473]) as f64).powf(p.p133) - ((s.v[475]) as f64).powf(p.p133))).max(0.0)) + (p.p134 * ((s.v[471]) as f64).powf(p.p135)));

        s.store_scale(549, 549, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p263 * ((((s.v[472]) as f64).powf(p.p264) - ((s.v[474]) as f64).powf(p.p264))).max(0.0));

        s.v[13] = ((p.p265 * ((((s.v[470]) as f64).powf(p.p266) - ((s.v[475]) as f64).powf(p.p266))).max(0.0)) + (p.p267 * ((s.v[471]) as f64).powf(p.p268)));

        s.store_scale(503, 503, ((1.0 + s.v[12]) + s.v[13]));

        s.store_scale(542, 542, (1.0 + (p.p352 * ((((s.v[472]) as f64).powf(p.p353) - ((s.v[474]) as f64).powf(p.p353))).max(0.0))));

        s.store_max_with_scalar(542, 542, 0.0);

        s.v[12] = (p.p186 * ((((s.v[469]) as f64).powf(p.p187) - ((s.v[474]) as f64).powf(p.p187))).max(0.0));

        s.v[13] = ((p.p188 * ((((s.v[470]) as f64).powf(p.p189) - ((s.v[475]) as f64).powf(p.p189))).max(0.0)) + (p.p190 * ((s.v[471]) as f64).powf(p.p191)));

        s.store_scale(495, 495, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p196 * ((((s.v[469]) as f64).powf(p.p197) - ((s.v[474]) as f64).powf(p.p197))).max(0.0));

        s.v[13] = ((p.p198 * ((((s.v[470]) as f64).powf(p.p199) - ((s.v[475]) as f64).powf(p.p199))).max(0.0)) + (p.p200 * ((s.v[471]) as f64).powf(p.p201)));

        s.store_scale(494, 494, ((1.0 + s.v[12]) + s.v[13]));

        s.store_scale(543, 543, (1.0 + (p.p383 * ((((s.v[469]) as f64).powf(p.p384) - ((s.v[474]) as f64).powf(p.p384))).max(0.0))));

        s.store_scale(567, 567, (1.0 + (s.v[469] * p.p828)));

        s.store_scale(568, 568, (1.0 + (s.v[469] * p.p833)));

        s.store_scale(570, 570, (1.0 + (s.v[469] * p.p842)));

        s.store_scale(573, 573, (1.0 + (s.v[469] * p.p860)));

        s.store_scale(574, 574, (1.0 + (s.v[469] * p.p866)));

        s.b[898] = (p.p42 == 1.0);
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if s.b[898] {
            s.store_scale(531, 531, (1.0 + (p.p397 * ((((s.v[469]) as f64).powf(p.p398) - ((s.v[474]) as f64).powf(p.p398))).max(0.0))));
            s.store_scale(530, 530, (1.0 + (p.p407 * ((((s.v[469]) as f64).powf(p.p408) - ((s.v[474]) as f64).powf(p.p408))).max(0.0))));
        }

        if (!s.b[898]) {
            s.store_scale(532, 532, (1.0 + (p.p414 * ((((s.v[469]) as f64).powf(p.p415) - ((s.v[474]) as f64).powf(p.p415))).max(0.0))));
        }

        s.b[899] = (s.v[511] < 1.0);
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if s.b[899] {
            s.store_scalar(511, 1.0);
        }

        s.b[900] = (s.v[511] > 2.0);
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if ((!s.b[899]) && s.b[900]) {
            s.store_scalar(511, 2.0);
        }

        s.b[901] = (p.p44 != 0.0);
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        s.b[902] = (s.v[512] < 1.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if (s.b[901] && s.b[902]) {
            s.store_scalar(512, 1.0);
        }

        s.b[903] = (s.v[512] > 2.0);
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if ((s.b[901] && (!s.b[902])) && s.b[903]) {
            s.store_scalar(512, 2.0);
        }

        s.b[925] = (s.v[606] < 0.0);
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if s.b[925] {
            s.store_scalar(606, 0.0);
        }

        s.b[926] = (s.v[497] <= 0.0);
        s.v[926] = if s.b[926] { 1.0 } else { 0.0 };

        if s.b[926] {
            s.store_scalar(497, 0.067);
        }

        s.b[927] = (s.v[504] < 0.0);
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if s.b[927] {
            s.store_scalar(504, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[928] = (s.v[507] < 0.0);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if s.b[928] {
            s.store_scalar(507, 0.0);
        }

        s.b[929] = (s.v[508] < 0.0);
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if s.b[929] {
            s.store_scalar(508, 0.0);
        }

        s.b[930] = (s.v[511] < 0.0);
        s.v[930] = if s.b[930] { 1.0 } else { 0.0 };

        if s.b[930] {
            s.store_scalar(511, 0.0);
        }

        s.b[931] = (s.v[555] < 0.0);
        s.v[931] = if s.b[931] { 1.0 } else { 0.0 };

        if s.b[931] {
            s.store_scalar(555, 0.0);
        }

        s.b[932] = (p.p1065 == 1.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

        if s.b[932] {
            s.store_scalar(746, p.p1066);
        }

        s.b[933] = (s.v[30] > s.v[746]);
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if (s.b[932] && s.b[933]) {
            s.store_sub_from_scalar(12, s.v[30], 746);
        }

        if (s.b[932] && (!s.b[933])) {
            s.store_scalar(746, s.v[30]);
            s.copy_ad(12, 746);
        }

        s.b[934] = (p.p801 >= (s.v[12] / 2.0));
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        if (s.b[932] && s.b[934]) {
            s.store_scalar(359, 0.0);
        }

        if (s.b[932] && (!s.b[934])) {
            s.store_scalar(359, p.p801);
        }

        s.v[701] = 0.0;

        s.v[703] = 0.0;

        s.v[700] = 0.0;

        s.v[702] = 0.0;

        s.v[705] = 0.0;

        s.v[704] = 0.0;

        s.v[236] = (p.p695 - p.p698);

        s.v[238] = p.p696;

        s.v[237] = (p.p697 - p.p698);

        s.b[935] = param_given[3];
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

        if s.b[935] {
            s.store_scalar(239, (p.p374 * p.p3));
        }

        s.b[936] = ((p.p10 > 0.0) && (p.p374 > 0.0));
        s.v[936] = if s.b[936] { 1.0 } else { 0.0 };

        s.b[937] = (p.p9 < 9.0);
        s.v[937] = if s.b[937] { 1.0 } else { 0.0 };

        s.b[938] = ((p.p2 % 2.0) != 0.0);
        s.v[938] = if s.b[938] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[938]) {
            s.store_scalar(701, 1.0);
            s.store_scalar(703, 1.0);
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(702, 700);
        }

        s.b[939] = (p.p6 == 1.0);
        s.v[939] = if s.b[939] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[938])) && s.b[939]) {
            s.store_scalar(701, 2.0);
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(703, 0.0);
            s.store_scalar(702, p.p2);
        }

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[938])) && (!s.b[939])) {
            s.store_scalar(701, 0.0);
            s.store_scalar(700, p.p2);
            s.store_scalar(703, 2.0);
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.b[940] = (1.0 == 1.0);
        s.v[940] = if s.b[940] { 1.0 } else { 0.0 };

        s.b[941] = (s.v[702] == 0.0);
        s.v[941] = if s.b[941] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[940]) && s.b[941]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && s.b[940]) && (!s.b[941])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 702, s.v[29]);
        }

        s.b[942] = (s.v[700] == 0.0);
        s.v[942] = if s.b[942] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[940])) && s.b[942]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && s.b[937]) && (!s.b[940])) && (!s.b[942])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 700, s.v[29]);
        }

        s.b[943] = (p.p9 == 0.0);
        s.v[943] = if s.b[943] { 1.0 } else { 0.0 };

        s.b[944] = (p.p9 == 1.0);
        s.v[944] = if s.b[944] { 1.0 } else { 0.0 };

        s.b[945] = (p.p9 == 2.0);
        s.v[945] = if s.b[945] { 1.0 } else { 0.0 };

        s.b[946] = (p.p9 == 3.0);
        s.v[946] = if s.b[946] { 1.0 } else { 0.0 };

        s.b[947] = (p.p9 == 4.0);
        s.v[947] = if s.b[947] { 1.0 } else { 0.0 };

        s.b[948] = (p.p9 == 5.0);
        s.v[948] = if s.b[948] { 1.0 } else { 0.0 };

        s.b[949] = (p.p9 == 6.0);
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        s.b[950] = (p.p9 == 7.0);
        s.v[950] = if s.b[950] { 1.0 } else { 0.0 };

        s.b[951] = (p.p9 == 8.0);
        s.v[951] = if s.b[951] { 1.0 } else { 0.0 };

        s.b[952] = (p.p9 == 9.0);
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        s.b[953] = (p.p9 == 10.0);
        s.v[953] = if s.b[953] { 1.0 } else { 0.0 };

        s.b[954] = (1.0 == 1.0);
        s.v[954] = if s.b[954] { 1.0 } else { 0.0 };

        s.b[955] = (1.0 == 1.0);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        s.b[956] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        s.b[957] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

        s.b[958] = (s.v[703] == 0.0);
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && s.b[956]) && s.b[958]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && s.b[956]) && (!s.b[958])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[960] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[960] = if s.b[960] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (s.b[957] && (!s.b[956]))) && s.b[960]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (s.b[957] && (!s.b[956]))) && (!s.b[960])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && s.b[955]) && (!(s.b[956] || s.b[957]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[961] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[961] = if s.b[961] { 1.0 } else { 0.0 };

        s.b[962] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[962] = if s.b[962] { 1.0 } else { 0.0 };

        s.b[963] = (s.v[703] == 0.0);
        s.v[963] = if s.b[963] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && s.b[961]) && s.b[963]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && s.b[961]) && (!s.b[963])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[965] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (s.b[962] && (!s.b[961]))) && s.b[965]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (s.b[962] && (!s.b[961]))) && (!s.b[965])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && s.b[954]) && (!s.b[955])) && (!(s.b[961] || s.b[962]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[966] = (0.0 == 1.0);
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        s.b[967] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[967] = if s.b[967] { 1.0 } else { 0.0 };

        s.b[968] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[968] = if s.b[968] { 1.0 } else { 0.0 };

        s.b[969] = (s.v[701] == 0.0);
        s.v[969] = if s.b[969] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && s.b[967]) && s.b[969]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && s.b[967]) && (!s.b[969])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[971] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[971] = if s.b[971] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (s.b[968] && (!s.b[967]))) && s.b[971]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (s.b[968] && (!s.b[967]))) && (!s.b[971])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && s.b[966]) && (!(s.b[967] || s.b[968]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[972] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[972] = if s.b[972] { 1.0 } else { 0.0 };

        s.b[973] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[973] = if s.b[973] { 1.0 } else { 0.0 };

        s.b[974] = (s.v[701] == 0.0);
        s.v[974] = if s.b[974] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && s.b[972]) && s.b[974]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && s.b[972]) && (!s.b[974])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[976] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[976] = if s.b[976] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (s.b[973] && (!s.b[972]))) && s.b[976]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (s.b[973] && (!s.b[972]))) && (!s.b[976])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && s.b[943]) && (!s.b[954])) && (!s.b[966])) && (!(s.b[972] || s.b[973]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[977] = (1.0 == 1.0);
        s.v[977] = if s.b[977] { 1.0 } else { 0.0 };

        s.b[978] = (1.0 == 1.0);
        s.v[978] = if s.b[978] { 1.0 } else { 0.0 };

        s.b[979] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[979] = if s.b[979] { 1.0 } else { 0.0 };

        s.b[980] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[980] = if s.b[980] { 1.0 } else { 0.0 };

        s.b[981] = (s.v[703] == 0.0);
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && s.b[979]) && s.b[981]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && s.b[979]) && (!s.b[981])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[983] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (s.b[980] && (!s.b[979]))) && s.b[983]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (s.b[980] && (!s.b[979]))) && (!s.b[983])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && s.b[978]) && (!(s.b[979] || s.b[980]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[984] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        s.b[985] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

        s.b[986] = (s.v[703] == 0.0);
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && s.b[984]) && s.b[986]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && s.b[984]) && (!s.b[986])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[988] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[988] = if s.b[988] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (s.b[985] && (!s.b[984]))) && s.b[988]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (s.b[985] && (!s.b[984]))) && (!s.b[988])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && s.b[977]) && (!s.b[978])) && (!(s.b[984] || s.b[985]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[989] = (0.0 == 1.0);
        s.v[989] = if s.b[989] { 1.0 } else { 0.0 };

        s.b[990] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[990] = if s.b[990] { 1.0 } else { 0.0 };

        s.b[991] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[991] = if s.b[991] { 1.0 } else { 0.0 };

        s.b[992] = (s.v[701] == 0.0);
        s.v[992] = if s.b[992] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && s.b[990]) && s.b[992]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && s.b[990]) && (!s.b[992])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[994] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (s.b[991] && (!s.b[990]))) && s.b[994]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (s.b[991] && (!s.b[990]))) && (!s.b[994])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && s.b[989]) && (!(s.b[990] || s.b[991]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[995] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[995] = if s.b[995] { 1.0 } else { 0.0 };

        s.b[996] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        s.b[997] = (s.v[701] == 0.0);
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && s.b[995]) && s.b[997]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && s.b[995]) && (!s.b[997])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[999] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[999] = if s.b[999] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (s.b[996] && (!s.b[995]))) && s.b[999]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (s.b[996] && (!s.b[995]))) && (!s.b[999])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[944] && (!s.b[943]))) && (!s.b[977])) && (!s.b[989])) && (!(s.b[995] || s.b[996]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1000] = (1.0 == 1.0);
        s.v[1000] = if s.b[1000] { 1.0 } else { 0.0 };

        s.b[1001] = (1.0 == 1.0);
        s.v[1001] = if s.b[1001] { 1.0 } else { 0.0 };

        s.b[1002] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1002] = if s.b[1002] { 1.0 } else { 0.0 };

        s.b[1003] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1003] = if s.b[1003] { 1.0 } else { 0.0 };

        s.b[1004] = (s.v[703] == 0.0);
        s.v[1004] = if s.b[1004] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && s.b[1002]) && s.b[1004]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && s.b[1002]) && (!s.b[1004])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1006] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1006] = if s.b[1006] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (s.b[1003] && (!s.b[1002]))) && s.b[1006]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (s.b[1003] && (!s.b[1002]))) && (!s.b[1006])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && s.b[1001]) && (!(s.b[1002] || s.b[1003]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1007] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };

        s.b[1008] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1008] = if s.b[1008] { 1.0 } else { 0.0 };

        s.b[1009] = (s.v[703] == 0.0);
        s.v[1009] = if s.b[1009] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && s.b[1007]) && s.b[1009]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && s.b[1007]) && (!s.b[1009])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1011] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (s.b[1008] && (!s.b[1007]))) && s.b[1011]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (s.b[1008] && (!s.b[1007]))) && (!s.b[1011])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && s.b[1000]) && (!s.b[1001])) && (!(s.b[1007] || s.b[1008]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1012] = (0.0 == 1.0);
        s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };

        s.b[1013] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        s.b[1014] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        s.b[1015] = (s.v[701] == 0.0);
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && s.b[1013]) && s.b[1015]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && s.b[1013]) && (!s.b[1015])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1017] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (s.b[1014] && (!s.b[1013]))) && s.b[1017]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (s.b[1014] && (!s.b[1013]))) && (!s.b[1017])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && s.b[1012]) && (!(s.b[1013] || s.b[1014]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1018] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        s.b[1019] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        s.b[1020] = (s.v[701] == 0.0);
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && s.b[1018]) && s.b[1020]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && s.b[1018]) && (!s.b[1020])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1022] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (s.b[1019] && (!s.b[1018]))) && s.b[1022]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (s.b[1019] && (!s.b[1018]))) && (!s.b[1022])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[945] && (!(s.b[943] || s.b[944])))) && (!s.b[1000])) && (!s.b[1012])) && (!(s.b[1018] || s.b[1019]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1023] = (1.0 == 1.0);
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        s.b[1024] = (1.0 == 1.0);
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        s.b[1025] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        s.b[1026] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        s.b[1027] = (s.v[703] == 0.0);
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && s.b[1025]) && s.b[1027]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && s.b[1025]) && (!s.b[1027])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1029] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (s.b[1026] && (!s.b[1025]))) && s.b[1029]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (s.b[1026] && (!s.b[1025]))) && (!s.b[1029])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && s.b[1024]) && (!(s.b[1025] || s.b[1026]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1030] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        s.b[1031] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        s.b[1032] = (s.v[703] == 0.0);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && s.b[1030]) && s.b[1032]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && s.b[1030]) && (!s.b[1032])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1034] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (s.b[1031] && (!s.b[1030]))) && s.b[1034]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (s.b[1031] && (!s.b[1030]))) && (!s.b[1034])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && s.b[1023]) && (!s.b[1024])) && (!(s.b[1030] || s.b[1031]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1035] = (0.0 == 1.0);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        s.b[1036] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        s.b[1037] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        s.b[1038] = (s.v[701] == 0.0);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && s.b[1036]) && s.b[1038]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && s.b[1036]) && (!s.b[1038])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1040] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (s.b[1037] && (!s.b[1036]))) && s.b[1040]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (s.b[1037] && (!s.b[1036]))) && (!s.b[1040])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && s.b[1035]) && (!(s.b[1036] || s.b[1037]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1041] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        s.b[1042] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        s.b[1043] = (s.v[701] == 0.0);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && s.b[1041]) && s.b[1043]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && s.b[1041]) && (!s.b[1043])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1045] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (s.b[1042] && (!s.b[1041]))) && s.b[1045]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (s.b[1042] && (!s.b[1041]))) && (!s.b[1045])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[946] && (!((s.b[943] || s.b[944]) || s.b[945])))) && (!s.b[1023])) && (!s.b[1035])) && (!(s.b[1041] || s.b[1042]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1046] = (1.0 == 1.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        s.b[1047] = (1.0 == 1.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        s.b[1048] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        s.b[1049] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        s.b[1050] = (s.v[703] == 0.0);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && s.b[1048]) && s.b[1050]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && s.b[1048]) && (!s.b[1050])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1052] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (s.b[1049] && (!s.b[1048]))) && s.b[1052]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (s.b[1049] && (!s.b[1048]))) && (!s.b[1052])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && s.b[1047]) && (!(s.b[1048] || s.b[1049]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1053] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        s.b[1054] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        s.b[1055] = (s.v[703] == 0.0);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && s.b[1053]) && s.b[1055]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && s.b[1053]) && (!s.b[1055])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1057] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (s.b[1054] && (!s.b[1053]))) && s.b[1057]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (s.b[1054] && (!s.b[1053]))) && (!s.b[1057])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && s.b[1046]) && (!s.b[1047])) && (!(s.b[1053] || s.b[1054]))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!s.b[935]) && s.b[936]) && (s.b[947] && (!(((s.b[943] || s.b[944]) || s.b[945]) || s.b[946])))) && (!s.b[1046])) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1058] = (1.0 == 1.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        s.b[1059] = (1.0 == 1.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        s.b[1060] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        s.b[1061] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        s.b[1062] = (s.v[703] == 0.0);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && s.b[1060]) && s.b[1062]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && s.b[1060]) && (!s.b[1062])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1064] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (s.b[1061] && (!s.b[1060]))) && s.b[1064]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (s.b[1061] && (!s.b[1060]))) && (!s.b[1064])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && s.b[1059]) && (!(s.b[1060] || s.b[1061]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1065] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        s.b[1066] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        s.b[1067] = (s.v[703] == 0.0);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && s.b[1065]) && s.b[1067]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && s.b[1065]) && (!s.b[1067])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1069] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (s.b[1066] && (!s.b[1065]))) && s.b[1069]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (s.b[1066] && (!s.b[1065]))) && (!s.b[1069])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && s.b[1058]) && (!s.b[1059])) && (!(s.b[1065] || s.b[1066]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1070] = (s.v[701] == 0.0);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && (!s.b[1058])) && s.b[1070]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[948] && (!((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947])))) && (!s.b[1058])) && (!s.b[1070])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 701, s.v[29]);
        }

        s.b[1071] = (1.0 == 1.0);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && s.b[1071]) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1072] = (0.0 == 1.0);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        s.b[1073] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        s.b[1074] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        s.b[1075] = (s.v[701] == 0.0);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && s.b[1073]) && s.b[1075]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && s.b[1073]) && (!s.b[1075])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1077] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (s.b[1074] && (!s.b[1073]))) && s.b[1077]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (s.b[1074] && (!s.b[1073]))) && (!s.b[1077])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && s.b[1072]) && (!(s.b[1073] || s.b[1074]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1078] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        s.b[1079] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        s.b[1080] = (s.v[701] == 0.0);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && s.b[1078]) && s.b[1080]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && s.b[1078]) && (!s.b[1080])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1082] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (s.b[1079] && (!s.b[1078]))) && s.b[1082]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (s.b[1079] && (!s.b[1078]))) && (!s.b[1082])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[949] && (!(((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948])))) && (!s.b[1071])) && (!s.b[1072])) && (!(s.b[1078] || s.b[1079]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1083] = (1.0 == 1.0);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        s.b[1084] = (s.v[703] == 0.0);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && s.b[1083]) && s.b[1084]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && s.b[1083]) && (!s.b[1084])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 703, s.v[29]);
        }

        s.b[1085] = (0.0 == 1.0);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        s.b[1086] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        s.b[1087] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        s.b[1088] = (s.v[701] == 0.0);
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && s.b[1086]) && s.b[1088]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && s.b[1086]) && (!s.b[1088])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1090] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (s.b[1087] && (!s.b[1086]))) && s.b[1090]) {
            s.store_scalar(705, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (s.b[1087] && (!s.b[1086]))) && (!s.b[1090])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && s.b[1085]) && (!(s.b[1086] || s.b[1087]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1091] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        s.b[1092] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        s.b[1093] = (s.v[701] == 0.0);
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && s.b[1091]) && s.b[1093]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && s.b[1091]) && (!s.b[1093])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1095] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (s.b[1092] && (!s.b[1091]))) && s.b[1095]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (s.b[1092] && (!s.b[1091]))) && (!s.b[1095])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[935]) && s.b[936]) && (s.b[950] && (!((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949])))) && (!s.b[1083])) && (!s.b[1085])) && (!(s.b[1091] || s.b[1092]))) {
            s.store_scalar(705, 0.0);
        }

        if (((!s.b[935]) && s.b[936]) && (s.b[951] && (!(((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950])))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1096] = (1.0 == 1.0);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1097] = (p.p2 == 2.0);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) && s.b[1097]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && s.b[1096]) && (!s.b[1097])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if ((((!s.b[935]) && s.b[936]) && (s.b[952] && (!((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951])))) && (!s.b[1096])) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        s.b[1098] = (1.0 == 1.0);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && s.b[1098]) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        if ((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1099] = (p.p2 == 2.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) && s.b[1099]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[935]) && s.b[936]) && (s.b[953] && (!(((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952])))) && (!s.b[1098])) && (!s.b[1099])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if (((!s.b[935]) && s.b[936]) && (!((((((((((s.b[943] || s.b[944]) || s.b[945]) || s.b[946]) || s.b[947]) || s.b[948]) || s.b[949]) || s.b[950]) || s.b[951]) || s.b[952]) || s.b[953]))) {
            s.store_scalar(704, 0.0);
        }

        s.b[1100] = (s.v[704] <= 0.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (((!s.b[935]) && s.b[936]) && s.b[1100]) {
            s.copy_ad(239, 705);
        }

        s.b[1101] = (s.v[705] <= 0.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if ((((!s.b[935]) && s.b[936]) && (!s.b[1100])) && s.b[1101]) {
            s.copy_ad(239, 704);
        }

        if ((((!s.b[935]) && s.b[936]) && (!s.b[1100])) && (!s.b[1101])) {
            s.store_ad_value(239, A::div_scaled_product(s.ad_value(704), s.ad_value(705), 1.0, A::add(s.ad_value(704), s.ad_value(705)), 1.0));
        }

        if ((!s.b[935]) && (!s.b[936])) {
            s.store_scalar(239, 0.0);
        }

        s.b[1103] = param_given[4];
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if s.b[1103] {
            s.store_scalar(240, (p.p374 * p.p4));
        }

        s.b[1104] = ((p.p10 > 0.0) && (p.p374 > 0.0));
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        s.b[1105] = (p.p9 < 9.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        s.b[1106] = ((p.p2 % 2.0) != 0.0);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1106]) {
            s.store_scalar(701, 1.0);
            s.store_scalar(703, 1.0);
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(702, 700);
        }

        s.b[1107] = (p.p6 == 1.0);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1106])) && s.b[1107]) {
            s.store_scalar(701, 2.0);
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(703, 0.0);
            s.store_scalar(702, p.p2);
        }

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1106])) && (!s.b[1107])) {
            s.store_scalar(701, 0.0);
            s.store_scalar(700, p.p2);
            s.store_scalar(703, 2.0);
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.b[1108] = (0.0 == 1.0);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        s.b[1109] = (s.v[702] == 0.0);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1108]) && s.b[1109]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && s.b[1108]) && (!s.b[1109])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 702, s.v[29]);
        }

        s.b[1110] = (s.v[700] == 0.0);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1108])) && s.b[1110]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && s.b[1105]) && (!s.b[1108])) && (!s.b[1110])) {
            s.store_div_from_scalar_scaled_input(704, (p.p374 * s.v[236]), 700, s.v[29]);
        }

        s.b[1111] = (p.p9 == 0.0);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        s.b[1112] = (p.p9 == 1.0);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        s.b[1113] = (p.p9 == 2.0);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        s.b[1114] = (p.p9 == 3.0);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        s.b[1115] = (p.p9 == 4.0);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        s.b[1116] = (p.p9 == 5.0);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        s.b[1117] = (p.p9 == 6.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        s.b[1118] = (p.p9 == 7.0);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        s.b[1119] = (p.p9 == 8.0);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        s.b[1120] = (p.p9 == 9.0);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        s.b[1121] = (p.p9 == 10.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        s.b[1122] = (0.0 == 1.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        s.b[1123] = (1.0 == 1.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        s.b[1124] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        s.b[1125] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        s.b[1126] = (s.v[703] == 0.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && s.b[1124]) && s.b[1126]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && s.b[1124]) && (!s.b[1126])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1128] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (s.b[1125] && (!s.b[1124]))) && s.b[1128]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (s.b[1125] && (!s.b[1124]))) && (!s.b[1128])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && s.b[1123]) && (!(s.b[1124] || s.b[1125]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1129] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        s.b[1130] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        s.b[1131] = (s.v[703] == 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && s.b[1129]) && s.b[1131]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && s.b[1129]) && (!s.b[1131])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1133] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (s.b[1130] && (!s.b[1129]))) && s.b[1133]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (s.b[1130] && (!s.b[1129]))) && (!s.b[1133])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && s.b[1122]) && (!s.b[1123])) && (!(s.b[1129] || s.b[1130]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1134] = (0.0 == 1.0);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        s.b[1135] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        s.b[1136] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        s.b[1137] = (s.v[701] == 0.0);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && s.b[1135]) && s.b[1137]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && s.b[1135]) && (!s.b[1137])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1139] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (s.b[1136] && (!s.b[1135]))) && s.b[1139]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (s.b[1136] && (!s.b[1135]))) && (!s.b[1139])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && s.b[1134]) && (!(s.b[1135] || s.b[1136]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1140] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        s.b[1141] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        s.b[1142] = (s.v[701] == 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && s.b[1140]) && s.b[1142]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && s.b[1140]) && (!s.b[1142])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1144] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (s.b[1141] && (!s.b[1140]))) && s.b[1144]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (s.b[1141] && (!s.b[1140]))) && (!s.b[1144])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && s.b[1111]) && (!s.b[1122])) && (!s.b[1134])) && (!(s.b[1140] || s.b[1141]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1145] = (0.0 == 1.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        s.b[1146] = (1.0 == 1.0);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        s.b[1147] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        s.b[1148] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        s.b[1149] = (s.v[703] == 0.0);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && s.b[1147]) && s.b[1149]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && s.b[1147]) && (!s.b[1149])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1151] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (s.b[1148] && (!s.b[1147]))) && s.b[1151]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (s.b[1148] && (!s.b[1147]))) && (!s.b[1151])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && s.b[1146]) && (!(s.b[1147] || s.b[1148]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1152] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        s.b[1153] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        s.b[1154] = (s.v[703] == 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && s.b[1152]) && s.b[1154]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && s.b[1152]) && (!s.b[1154])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1156] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (s.b[1153] && (!s.b[1152]))) && s.b[1156]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (s.b[1153] && (!s.b[1152]))) && (!s.b[1156])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && s.b[1145]) && (!s.b[1146])) && (!(s.b[1152] || s.b[1153]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1157] = (0.0 == 1.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        s.b[1158] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        s.b[1159] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        s.b[1160] = (s.v[701] == 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && s.b[1158]) && s.b[1160]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && s.b[1158]) && (!s.b[1160])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1162] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (s.b[1159] && (!s.b[1158]))) && s.b[1162]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (s.b[1159] && (!s.b[1158]))) && (!s.b[1162])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && s.b[1157]) && (!(s.b[1158] || s.b[1159]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1163] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        s.b[1164] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        s.b[1165] = (s.v[701] == 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && s.b[1163]) && s.b[1165]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && s.b[1163]) && (!s.b[1165])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1167] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (s.b[1164] && (!s.b[1163]))) && s.b[1167]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (s.b[1164] && (!s.b[1163]))) && (!s.b[1167])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1112] && (!s.b[1111]))) && (!s.b[1145])) && (!s.b[1157])) && (!(s.b[1163] || s.b[1164]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1168] = (0.0 == 1.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        s.b[1169] = (1.0 == 1.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        s.b[1170] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        s.b[1171] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        s.b[1172] = (s.v[703] == 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && s.b[1170]) && s.b[1172]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && s.b[1170]) && (!s.b[1172])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1174] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (s.b[1171] && (!s.b[1170]))) && s.b[1174]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (s.b[1171] && (!s.b[1170]))) && (!s.b[1174])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && s.b[1169]) && (!(s.b[1170] || s.b[1171]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1175] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        s.b[1176] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        s.b[1177] = (s.v[703] == 0.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && s.b[1175]) && s.b[1177]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && s.b[1175]) && (!s.b[1177])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1179] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (s.b[1176] && (!s.b[1175]))) && s.b[1179]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (s.b[1176] && (!s.b[1175]))) && (!s.b[1179])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && s.b[1168]) && (!s.b[1169])) && (!(s.b[1175] || s.b[1176]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1180] = (0.0 == 1.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        s.b[1181] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        s.b[1182] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        s.b[1183] = (s.v[701] == 0.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && s.b[1181]) && s.b[1183]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && s.b[1181]) && (!s.b[1183])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1185] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (s.b[1182] && (!s.b[1181]))) && s.b[1185]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (s.b[1182] && (!s.b[1181]))) && (!s.b[1185])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && s.b[1180]) && (!(s.b[1181] || s.b[1182]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1186] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        s.b[1187] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        s.b[1188] = (s.v[701] == 0.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && s.b[1186]) && s.b[1188]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && s.b[1186]) && (!s.b[1188])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1190] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (s.b[1187] && (!s.b[1186]))) && s.b[1190]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (s.b[1187] && (!s.b[1186]))) && (!s.b[1190])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1113] && (!(s.b[1111] || s.b[1112])))) && (!s.b[1168])) && (!s.b[1180])) && (!(s.b[1186] || s.b[1187]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1191] = (0.0 == 1.0);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        s.b[1192] = (1.0 == 1.0);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        s.b[1193] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        s.b[1194] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        s.b[1195] = (s.v[703] == 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && s.b[1193]) && s.b[1195]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && s.b[1193]) && (!s.b[1195])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1197] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (s.b[1194] && (!s.b[1193]))) && s.b[1197]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (s.b[1194] && (!s.b[1193]))) && (!s.b[1197])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && s.b[1192]) && (!(s.b[1193] || s.b[1194]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1198] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        s.b[1199] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        s.b[1200] = (s.v[703] == 0.0);
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && s.b[1198]) && s.b[1200]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && s.b[1198]) && (!s.b[1200])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1202] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (s.b[1199] && (!s.b[1198]))) && s.b[1202]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (s.b[1199] && (!s.b[1198]))) && (!s.b[1202])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && s.b[1191]) && (!s.b[1192])) && (!(s.b[1198] || s.b[1199]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1203] = (0.0 == 1.0);
        s.v[1203] = if s.b[1203] { 1.0 } else { 0.0 };

        s.b[1204] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1204] = if s.b[1204] { 1.0 } else { 0.0 };

        s.b[1205] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1205] = if s.b[1205] { 1.0 } else { 0.0 };

        s.b[1206] = (s.v[701] == 0.0);
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && s.b[1204]) && s.b[1206]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && s.b[1204]) && (!s.b[1206])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1208] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (s.b[1205] && (!s.b[1204]))) && s.b[1208]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (s.b[1205] && (!s.b[1204]))) && (!s.b[1208])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && s.b[1203]) && (!(s.b[1204] || s.b[1205]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1209] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        s.b[1210] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        s.b[1211] = (s.v[701] == 0.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && s.b[1209]) && s.b[1211]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && s.b[1209]) && (!s.b[1211])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1213] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (s.b[1210] && (!s.b[1209]))) && s.b[1213]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (s.b[1210] && (!s.b[1209]))) && (!s.b[1213])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1114] && (!((s.b[1111] || s.b[1112]) || s.b[1113])))) && (!s.b[1191])) && (!s.b[1203])) && (!(s.b[1209] || s.b[1210]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1214] = (0.0 == 1.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        s.b[1215] = (1.0 == 1.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        s.b[1216] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        s.b[1217] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        s.b[1218] = (s.v[703] == 0.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && s.b[1216]) && s.b[1218]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && s.b[1216]) && (!s.b[1218])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1220] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (s.b[1217] && (!s.b[1216]))) && s.b[1220]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (s.b[1217] && (!s.b[1216]))) && (!s.b[1220])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && s.b[1215]) && (!(s.b[1216] || s.b[1217]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1221] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        s.b[1222] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        s.b[1223] = (s.v[703] == 0.0);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && s.b[1221]) && s.b[1223]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && s.b[1221]) && (!s.b[1223])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1225] = ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (s.b[1222] && (!s.b[1221]))) && s.b[1225]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (s.b[1222] && (!s.b[1221]))) && (!s.b[1225])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && s.b[1214]) && (!s.b[1215])) && (!(s.b[1221] || s.b[1222]))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1115] && (!(((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114])))) && (!s.b[1214])) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1226] = (0.0 == 1.0);
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        s.b[1227] = (1.0 == 1.0);
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        s.b[1228] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        s.b[1229] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        s.b[1230] = (s.v[703] == 0.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && s.b[1228]) && s.b[1230]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && s.b[1228]) && (!s.b[1230])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1232] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (s.b[1229] && (!s.b[1228]))) && s.b[1232]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (s.b[1229] && (!s.b[1228]))) && (!s.b[1232])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && s.b[1227]) && (!(s.b[1228] || s.b[1229]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1233] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        s.b[1234] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        s.b[1235] = (s.v[703] == 0.0);
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && s.b[1233]) && s.b[1235]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && s.b[1233]) && (!s.b[1235])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 703, s.v[29]);
        }

        s.b[1237] = ((s.v[703] == 0.0) || (s.v[236] == 0.0));
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (s.b[1234] && (!s.b[1233]))) && s.b[1237]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (s.b[1234] && (!s.b[1233]))) && (!s.b[1237])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 703, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && s.b[1226]) && (!s.b[1227])) && (!(s.b[1233] || s.b[1234]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1238] = (s.v[701] == 0.0);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && (!s.b[1226])) && s.b[1238]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1116] && (!((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115])))) && (!s.b[1226])) && (!s.b[1238])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 701, s.v[29]);
        }

        s.b[1239] = (0.0 == 1.0);
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && s.b[1239]) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1240] = (0.0 == 1.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        s.b[1241] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        s.b[1242] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        s.b[1243] = (s.v[701] == 0.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && s.b[1241]) && s.b[1243]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && s.b[1241]) && (!s.b[1243])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1245] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (s.b[1242] && (!s.b[1241]))) && s.b[1245]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (s.b[1242] && (!s.b[1241]))) && (!s.b[1245])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && s.b[1240]) && (!(s.b[1241] || s.b[1242]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1246] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        s.b[1247] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        s.b[1248] = (s.v[701] == 0.0);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && s.b[1246]) && s.b[1248]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && s.b[1246]) && (!s.b[1248])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1250] = ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0));
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (s.b[1247] && (!s.b[1246]))) && s.b[1250]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (s.b[1247] && (!s.b[1246]))) && (!s.b[1250])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (3.0 * (s.v[236] + s.v[238])));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1117] && (!(((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116])))) && (!s.b[1239])) && (!s.b[1240])) && (!(s.b[1246] || s.b[1247]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1251] = (0.0 == 1.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        s.b[1252] = (s.v[703] == 0.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && s.b[1251]) && s.b[1252]) {
            s.store_scalar(705, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && s.b[1251]) && (!s.b[1252])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[237]), 703, s.v[29]);
        }

        s.b[1253] = (0.0 == 1.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        s.b[1254] = (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        s.b[1255] = (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0));
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        s.b[1256] = (s.v[701] == 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && s.b[1254]) && s.b[1256]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && s.b[1254]) && (!s.b[1256])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1258] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (s.b[1255] && (!s.b[1254]))) && s.b[1258]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (s.b[1255] && (!s.b[1254]))) && (!s.b[1258])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && s.b[1253]) && (!(s.b[1254] || s.b[1255]))) {
            s.store_scalar(705, 0.0);
        }

        s.b[1259] = (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0));
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        s.b[1260] = (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0));
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        s.b[1261] = (s.v[701] == 0.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && s.b[1259]) && s.b[1261]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && s.b[1259]) && (!s.b[1261])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[236]), 701, s.v[29]);
        }

        s.b[1263] = ((s.v[701] == 0.0) || (s.v[236] == 0.0));
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (s.b[1260] && (!s.b[1259]))) && s.b[1263]) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (s.b[1260] && (!s.b[1259]))) && (!s.b[1263])) {
            s.store_div_from_scalar_scaled_input(705, (p.p374 * s.v[29]), 701, (6.0 * s.v[236]));
        }

        if ((((((!s.b[1103]) && s.b[1104]) && (s.b[1118] && (!((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117])))) && (!s.b[1251])) && (!s.b[1253])) && (!(s.b[1259] || s.b[1260]))) {
            s.store_scalar(705, 0.0);
        }

        if (((!s.b[1103]) && s.b[1104]) && (s.b[1119] && (!(((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118])))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.b[1264] = (0.0 == 1.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1265] = (p.p2 == 2.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) && s.b[1265]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && s.b[1264]) && (!s.b[1265])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1120] && (!((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119])))) && (!s.b[1264])) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        s.b[1266] = (0.0 == 1.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && s.b[1266]) {
            s.store_scalar(705, 0.0);
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        if ((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.b[1267] = (p.p2 == 2.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) && s.b[1267]) {
            s.store_scalar(704, 0.0);
        }

        if (((((!s.b[1103]) && s.b[1104]) && (s.b[1121] && (!(((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120])))) && (!s.b[1266])) && (!s.b[1267])) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if (((!s.b[1103]) && s.b[1104]) && (!((((((((((s.b[1111] || s.b[1112]) || s.b[1113]) || s.b[1114]) || s.b[1115]) || s.b[1116]) || s.b[1117]) || s.b[1118]) || s.b[1119]) || s.b[1120]) || s.b[1121]))) {
            s.store_scalar(704, 0.0);
        }

        s.b[1268] = (s.v[704] <= 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (((!s.b[1103]) && s.b[1104]) && s.b[1268]) {
            s.copy_ad(240, 705);
        }

        s.b[1269] = (s.v[705] <= 0.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((((!s.b[1103]) && s.b[1104]) && (!s.b[1268])) && s.b[1269]) {
            s.copy_ad(240, 704);
        }

        if ((((!s.b[1103]) && s.b[1104]) && (!s.b[1268])) && (!s.b[1269])) {
            s.store_ad_value(240, A::div_scaled_product(s.ad_value(704), s.ad_value(705), 1.0, A::add(s.ad_value(704), s.ad_value(705)), 1.0));
        }

        if ((!s.b[1103]) && (!s.b[1104])) {
            s.store_scalar(240, 0.0);
        }

        s.b[1271] = (p.p42 == 0.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        s.b[1272] = (s.v[239] < p.p1093);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (s.b[1271] && s.b[1272]) {
            s.store_scalar(239, 0.0);
        }

        s.b[1273] = (s.v[240] < p.p1093);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if (s.b[1271] && s.b[1273]) {
            s.store_scalar(240, 0.0);
        }

        s.b[1274] = (s.v[239] <= p.p1093);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if ((!s.b[1271]) && s.b[1274]) {
            s.store_scalar(239, p.p1093);
        }

        s.b[1275] = (s.v[240] <= p.p1093);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if ((!s.b[1271]) && s.b[1275]) {
            s.store_scalar(240, p.p1093);
        }

        s.b[1276] = (p.p42 == 1.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        s.b[1277] = (s.v[529] <= 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1277]) {
            s.store_scalar(529, 0.0);
        }

        s.b[1278] = (s.v[528] <= 0.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1278]) {
            s.store_scalar(528, 0.0);
        }

        s.b[1279] = (s.v[531] <= 0.0);
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1279]) {
            s.store_scalar(531, 0.0);
        }

        s.b[1280] = (s.v[530] <= 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (s.b[1276] && s.b[1280]) {
            s.store_scalar(530, 0.0);
        }

        s.b[1281] = (s.v[533] <= 0.0);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if ((!s.b[1276]) && s.b[1281]) {
            s.store_scalar(533, 0.0);
        }

        s.b[1282] = (s.v[532] <= 0.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((!s.b[1276]) && s.b[1282]) {
            s.store_scalar(532, 0.0);
        }

        s.b[1301] = (p.p1097 == 1.0);
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if s.b[1301] {
            s.store_scalar(302, (1.0 - p.p1128));
        }

        if (!s.b[1301]) {
            s.store_scalar(302, 1.0);
        }

        s.v[252] = ((p.p700 * (p.p31 + ((s.v[35] / 3.0) / p.p32))) / ((p.p32 * p.p2) * (s.v[98] - p.p699)));

        s.b[1303] = (s.v[252] > 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if s.b[1303] {
            s.store_scalar(252, (1.0 / s.v[252]));
        }

        if (!s.b[1303]) {
            s.store_scalar(252, 1000.0);
        }

        s.v[12] = (p.p77 * p.p77);

        s.store_scale(13, 599, p.p77);

        s.store_square(14, 13);

        s.v[295] = (if (p.p39 == 1.0) { 745669000000.0 } else { 1166450000000.0 });

        s.store_scale(297, 599, ((-s.v[295]) * p.p77));

        s.v[295] = ((-s.v[295]) * p.p77);

        s.v[38] = (p.p911 + s.v[29]);

        s.b[1305] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if s.b[1305] {
            s.store_scalar(747, ((s.v[38] * p.p2) / p.p909));
            s.store_scalar(748, ((p.p910 * s.v[38]) * p.p2));
        }

        if (!s.b[1305]) {
            s.store_scalar(747, 1.0);
            s.store_scalar(748, 0.0);
        }

        s.b[1306] = (p.p820 <= (-273.15));
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if s.b[1306] {
            s.store_scalar(12, (300.15 - 273.15));
            s.store_scalar(392, 300.15);
        }

        if (!s.b[1306]) {
            s.store_scalar(392, (p.p820 + 273.15));
        }

        s.v[391] = (ctx_temp + p.p33);

        s.b[1307] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if s.b[1307] {
            s.store_voltage(390, ctx, nodes, Some(4), None);
        }

        if (!s.b[1307]) {
            s.store_scalar(390, 0.0);
        }

        s.store_offset(391, 390, s.v[391]);

        s.store_scale(108, 391, 8.617087e-5);

        s.store_div_from_scalar(109, 1.0, 108);

        s.store_div(395, 391, 392);

        s.store_sub(396, 391, 392);

        s.store_scale(393, 391, 8.617087e-5);

        s.store_scale(394, 392, 8.617087e-5);

        s.store_sub_from_scalar_ad(36, p.p109, A::div_scaled_product_offset_denominator(s.ad_value(391), s.ad_value(391), p.p821, s.ad_value(391), p.p822, 1.0));

        s.store_sub_from_scalar_ad(37, p.p109, A::div_scaled_product_offset_denominator(s.ad_value(392), s.ad_value(392), p.p821, s.ad_value(392), p.p822, 1.0));

        s.store_mul_ad(13, A::div(s.ad_value(391), s.ad_value(392)), A::sqrt(A::div(s.ad_value(391), s.ad_value(392))));

        s.store_mul_scaled_ad_rhs(28, 13, p.p108, A::limited_exp(A::sub(A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(394), 2.0), A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(393), 2.0))));

        s.b[1308] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if s.b[1308] {
            s.store_ln_ad(12, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));
            s.store_sqrt_square_offset(88, 12, 1e-6);
        }

        if (!s.b[1308]) {
            s.store_ln_ad(88, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));
        }

        s.b[1309] = (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if s.b[1309] {
            s.store_ln_ad(12, A::max_with_scalar(A::div_scaled_product(s.ad_value(686), s.ad_value(480), 1.0, A::square(s.ad_value(28)), 1.0), 1e-38));
            s.store_sqrt_square_offset(675, 12, 1e-6);
        }

        if (!s.b[1309]) {
            s.store_ln_ad(675, A::max_with_scalar(A::div_scaled_product(s.ad_value(686), s.ad_value(480), 1.0, A::square(s.ad_value(28)), 1.0), 1e-38));
        }

        s.b[1310] = (s.v[479] > 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if s.b[1310] {
            s.store_offset_ad(63, A::mul3_scaled_output(s.ad_value(187), s.ad_value(108), A::ln(A::max_with_scalar(A::div(s.ad_value(479), s.ad_value(480)), 1e-38)), -1.0), p.p5);
        }

        if (!s.b[1310]) {
            s.store_scalar(63, 0.0);
        }

        s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(88)), 0.4), s.ad_value(489)), 0.4);

        s.store_sqrt(128, 127);

        s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(481), 1.60219e-19));

        s.store_sqrt_scaled_input(129, 538, ((s.v[26] / s.v[27]) * p.p77));

        let assign13230_ad_e18111: A = {
    if (!((1.0 + (p.p823 * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0))), A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0)))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if ((1.0 + (p.p823 * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), p.p823, (((((-1.0)) * (p.p823))) + (1.0))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(422, 488, assign13230_ad_e18111);

        s.store_mul_ad_rhs(420, 490, A::scale_offset(s.ad_value(395), p.p851, (((((-1.0)) * (p.p851))) + (1.0))));

        s.b[1311] = (p.p44 != 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if s.b[1311] {
            s.store_mul_ad_rhs(421, 491, A::scale_offset(s.ad_value(395), p.p851, (((((-1.0)) * (p.p851))) + (1.0))));
        }

        s.v[158] = (if (p.p39 != 1.0) { (0.3333333333333333 * p.p283) } else { (0.5 * p.p283) });

        s.store_mul_pow_ad_rhs(397, 497, s.ad_value(395), s.ad_value(567));

        let assign13290_ad_e18224: A = {
    if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(399, 504, assign13290_ad_e18224);

        let assign13300_ad_e18298: A = {
    if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(401, 514, assign13300_ad_e18298);

        s.store_mul_pow_ad_rhs(403, 508, s.ad_value(395), s.ad_value(570));

        s.store_mul_pow_ad_rhs(405, 511, s.ad_value(395), s.ad_value(571));

        let assign13330_ad_e18382: A = {
    if (!((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0, A::offset(A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0)), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if ((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0, 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(407, 507, assign13330_ad_e18382);

        s.b[1312] = (p.p44 != 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if s.b[1312] {
            s.store_mul_pow_ad_rhs(398, 498, s.ad_value(395), s.ad_value(567));
        }

        if s.b[1312] {
            let assign13360_ad_e18468: A = {
                if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(400, 505, assign13360_ad_e18468);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1312] {
            let assign13370_ad_e18545: A = {
                if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(402, 515, assign13370_ad_e18545);
        }

        if s.b[1312] {
            s.store_mul_pow_ad_rhs(404, 509, s.ad_value(395), s.ad_value(570));
            s.store_mul_pow_ad_rhs(406, 512, s.ad_value(395), s.ad_value(571));
        }

        s.store_pow_ad(408, s.ad_value(395), s.ad_value(572));

        s.store_mul_pow_ad_rhs(409, 500, s.ad_value(395), A::neg(s.ad_value(573)));

        s.b[1313] = (s.v[409] < 100.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if s.b[1313] {
            s.store_scalar(409, 100.0);
        }

        s.b[1314] = (p.p1094 == 1.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if s.b[1314] {
            s.store_powf(762, 395, p.p1120);
            s.store_scale_ad(763, A::powf(s.ad_value(395), (-p.p1121)), p.p1100);
        }

        s.b[1315] = (p.p44 != 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if s.b[1315] {
            s.store_mul_pow_ad_rhs(410, 501, s.ad_value(395), A::neg(s.ad_value(573)));
        }

        s.b[1316] = (s.v[410] < 100.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (s.b[1315] && s.b[1316]) {
            s.store_scalar(410, 100.0);
        }

        s.store_mul_pow_ad_rhs(411, 503, s.ad_value(395), A::neg(s.ad_value(573)));

        s.b[1317] = (s.v[411] < 100.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if s.b[1317] {
            s.store_scalar(411, 100.0);
        }

        let assign13540_ad_e18729: A = {
    if (!((((1.0 / s.v[496]) * (1.0 + (p.p861 * s.v[396]))) - 2.0) < ((-10000.0) * 0.001))) {
        let assign13540_ad_e18693: A = A::add(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0)), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0), A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign13540_ad_e18693, 0.5)
    } else {
        {
            if ((((1.0 / s.v[496]) * (1.0 + (p.p861 * s.v[396]))) - 2.0) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p.p861, 1.0)), (-2.0), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_offset_ad(412, 1.0, assign13540_ad_e18729, 2.0);

        let assign13550_ad_e18805: A = {
    if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(413, 534, assign13550_ad_e18805);

        s.b[1318] = (p.p44 != 0.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if s.b[1318] {
            let assign13570_ad_e18883: A = {
                if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(414, 535, assign13570_ad_e18883);
        }

        let assign13580_ad_e18959: A = {
    if (!(((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(150, 148, assign13580_ad_e18959);

        let assign13590_ad_e19033: A = {
    if (!(((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(153, 151, assign13590_ad_e19033);

        s.store_mul_pow_ad_rhs(415, 554, s.ad_value(395), s.ad_value(575));

        s.b[1319] = (p.p44 != 0.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if s.b[1319] {
            s.store_mul_pow_ad_rhs(416, 557, s.ad_value(395), s.ad_value(575));
        }

        let assign13630_ad_e19123: A = {
    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(417, 560, assign13630_ad_e19123);

        let assign13640_ad_e19197: A = {
    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(418, 564, assign13640_ad_e19197);

        let assign13660_ad_e19278: A = {
    if (!(((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(609, 605, assign13660_ad_e19278);

        let assign13670_ad_e19352: A = {
    if (!(((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(610, 606, assign13670_ad_e19352);

        let assign13680_ad_e19426: A = {
    if (!(((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(633, 631, assign13680_ad_e19426);

        let assign13690_ad_e19500: A = {
    if (!(((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(636, 634, assign13690_ad_e19500);

        let assign13700_ad_e19574: A = {
    if (!(((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6))), A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(639, 637, assign13700_ad_e19574);

        let assign13710_ad_e19648: A = {
    if (!(((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(423, assign13710_ad_e19648, p.p701);

        let assign13720_ad_e19722: A = {
    if (!(((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p889, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(426, assign13720_ad_e19722, p.p702);

        let assign13730_ad_e19796: A = {
    if (!(((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(424, assign13730_ad_e19796, p.p703);

        let assign13740_ad_e19870: A = {
    if (!(((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p890, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(427, assign13740_ad_e19870, p.p704);

        let assign13750_ad_e19944: A = {
    if (!(((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(428, assign13750_ad_e19944, p.p705);

        let assign13760_ad_e20018: A = {
    if (!(((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p.p891, ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(425, assign13760_ad_e20018, p.p706);

        let assign13770_ad_e20091: A = {
    if (!(((p.p707 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01), A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p707 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(429, assign13770_ad_e20091, 0.01);

        let assign13780_ad_e20165: A = {
    if (!(((p.p708 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01), A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p708 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(432, assign13780_ad_e20165, 0.01);

        let assign13790_ad_e20239: A = {
    if (!(((p.p709 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01), A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p709 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(430, assign13790_ad_e20239, 0.01);

        let assign13800_ad_e20313: A = {
    if (!(((p.p710 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01), A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p710 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(433, assign13800_ad_e20313, 0.01);

        let assign13810_ad_e20387: A = {
    if (!(((p.p711 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01), A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p711 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(431, assign13810_ad_e20387, 0.01);

        let assign13820_ad_e20461: A = {
    if (!(((p.p712 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01), A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((p.p712 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01), 1.0)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(434, assign13820_ad_e20461, 0.01);

        s.store_sub_ad(12, A::div(s.ad_value(37), s.ad_value(394)), A::div(s.ad_value(36), s.ad_value(393)));

        s.store_ln_ad(13, A::max_with_scalar(s.ad_value(395), 1e-38));

        s.store_ad_value(15, A::limited_exp_scaled_input(A::add_scaled_inputs(s.ad_value(12), 1.0, s.ad_value(13), p.p895), 1.0 / (p.p725)));

        s.store_scale(435, 15, p.p719);

        s.store_scale(436, 15, p.p721);

        s.store_scale(437, 15, p.p723);

        s.store_ad_value(15, A::limited_exp_scaled_input(A::add_scaled_inputs(s.ad_value(12), 1.0, s.ad_value(13), p.p896), 1.0 / (p.p726)));

        s.store_scale(438, 15, p.p720);

        s.store_scale(439, 15, p.p722);

        s.store_scale(440, 15, p.p724);

        s.store_scaled_limited_exp_ad(441, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p.p897, s.ad_value(393), 1.0), p.p735);

        s.store_scaled_limited_exp_ad(443, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p.p899, s.ad_value(393), 1.0), p.p737);

        s.store_scaled_limited_exp_ad(445, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p.p901, s.ad_value(393), 1.0), (p.p739 * ((((p.p741 / s.v[35])) as f64).sqrt() + 1.0)));

        s.store_scaled_limited_exp_ad(442, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p.p898, s.ad_value(393), 1.0), p.p736);

        s.store_scaled_limited_exp_ad(444, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p.p900, s.ad_value(393), 1.0), p.p738);

        s.store_scaled_limited_exp_ad(446, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p.p902, s.ad_value(393), 1.0), (p.p740 * ((((p.p741 / s.v[35])) as f64).sqrt() + 1.0)));

        let assign13990_ad_e20690: A = {
    if (!(((p.p742 * (1.0 + (p.p903 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign13990_ad_e20654: A = A::add(A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign13990_ad_e20654, 0.5)
    } else {
        {
            if (((p.p742 * (1.0 + (p.p903 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p903) * (p.p742)), (((((((((-1.0)) * (p.p903))) + (1.0))) * (p.p742))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(447, assign13990_ad_e20690, 0.01);

        let assign14000_ad_e20788: A = {
    if (!(((p.p744 * (1.0 + (p.p905 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14000_ad_e20752: A = A::add(A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14000_ad_e20752, 0.5)
    } else {
        {
            if (((p.p744 * (1.0 + (p.p905 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p905) * (p.p744)), (((((((((-1.0)) * (p.p905))) + (1.0))) * (p.p744))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(449, assign14000_ad_e20788, 0.01);

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let assign14010_ad_e20886: A = {
    if (!(((p.p746 * (1.0 + (p.p907 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14010_ad_e20850: A = A::add(A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14010_ad_e20850, 0.5)
    } else {
        {
            if (((p.p746 * (1.0 + (p.p907 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p907) * (p.p746)), (((((((((-1.0)) * (p.p907))) + (1.0))) * (p.p746))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(451, assign14010_ad_e20886, 0.01);

        let assign14020_ad_e20984: A = {
    if (!(((p.p743 * (1.0 + (p.p904 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14020_ad_e20948: A = A::add(A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14020_ad_e20948, 0.5)
    } else {
        {
            if (((p.p743 * (1.0 + (p.p904 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p904) * (p.p743)), (((((((((-1.0)) * (p.p904))) + (1.0))) * (p.p743))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(448, assign14020_ad_e20984, 0.01);

        let assign14030_ad_e21082: A = {
    if (!(((p.p745 * (1.0 + (p.p906 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14030_ad_e21046: A = A::add(A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14030_ad_e21046, 0.5)
    } else {
        {
            if (((p.p745 * (1.0 + (p.p906 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p906) * (p.p745)), (((((((((-1.0)) * (p.p906))) + (1.0))) * (p.p745))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(450, assign14030_ad_e21082, 0.01);

        let assign14040_ad_e21180: A = {
    if (!(((p.p747 * (1.0 + (p.p908 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14040_ad_e21144: A = A::add(A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01)))), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01)))), A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01))))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14040_ad_e21144, 0.5)
    } else {
        {
            if (((p.p747 * (1.0 + (p.p908 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p.p908) * (p.p747)), (((((((((-1.0)) * (p.p908))) + (1.0))) * (p.p747))) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(452, assign14040_ad_e21180, 0.01);

        s.b[1320] = (p.p9 < 9.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        s.b[1321] = ((p.p2 % 2.0) != 0.0);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (s.b[1320] && s.b[1321]) {
            s.store_scalar(701, 1.0);
            s.store_scalar(703, 1.0);
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
            s.copy_ad(702, 700);
        }

        s.b[1322] = (p.p6 == 1.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if ((s.b[1320] && (!s.b[1321])) && s.b[1322]) {
            s.store_scalar(701, 2.0);
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
            s.store_scalar(703, 0.0);
            s.store_scalar(702, p.p2);
        }

        if ((s.b[1320] && (!s.b[1321])) && (!s.b[1322])) {
            s.store_scalar(701, 0.0);
            s.store_scalar(700, p.p2);
            s.store_scalar(703, 2.0);
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.v[12] = (s.v[236] + s.v[238]);

        s.v[13] = (s.v[236] + s.v[236]);

        s.v[14] = (s.v[237] + s.v[237]);

        s.v[0] = ((s.v[12] + s.v[12]) + s.v[35]);

        s.v[1] = ((s.v[12] + s.v[12]) + s.v[35]);

        s.v[2] = s.v[13];

        s.v[3] = s.v[13];

        s.v[4] = s.v[14];

        s.v[5] = s.v[14];

        s.v[6] = (s.v[12] * s.v[35]);

        s.v[7] = (s.v[12] * s.v[35]);

        s.v[8] = (s.v[236] * s.v[35]);

        s.v[9] = (s.v[236] * s.v[35]);

        s.v[10] = (s.v[237] * s.v[35]);

        s.v[11] = (s.v[237] * s.v[35]);

        s.b[1323] = (p.p9 == 0.0);
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        s.b[1324] = (p.p9 == 1.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        s.b[1325] = (p.p9 == 2.0);
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        s.b[1326] = (p.p9 == 3.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        s.b[1327] = (p.p9 == 4.0);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        s.b[1328] = (p.p9 == 5.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        s.b[1329] = (p.p9 == 6.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        s.b[1330] = (p.p9 == 7.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        s.b[1331] = (p.p9 == 8.0);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        s.b[1332] = (p.p9 == 9.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        s.b[1333] = (p.p9 == 10.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if s.b[1323] {
            s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);
        }

        if (s.b[1324] && (!s.b[1323])) {
            s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);
            s.store_scaled_add(249, 701, 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if (s.b[1325] && (!(s.b[1323] || s.b[1324]))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);
            s.store_scaled_add(246, 703, 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);
        }

        if (s.b[1326] && (!((s.b[1323] || s.b[1324]) || s.b[1325]))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
            s.store_scaled_add(249, 701, 700, s.v[3]);
            s.store_scaled_add(246, 703, 702, s.v[8]);
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if (s.b[1327] && (!(((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]))) {
            s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);
        }

        if (s.b[1328] && (!((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);
            s.store_scaled_add(246, 703, 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);
        }

        if (s.b[1329] && (!(((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]))) {
            s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);
        }

        if (s.b[1330] && (!((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]))) {
            s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);
            s.store_scaled_add(249, 701, 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if (s.b[1331] && (!(((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]))) {
            s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);
            s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);
            s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);
            s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);
        }

        if (s.b[1332] && (!((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]))) {
            s.store_scalar(248, (s.v[0] + ((p.p2 - 1.0) * s.v[2])));
            s.store_scalar(249, (p.p2 * s.v[3]));
            s.store_scalar(246, (s.v[6] + ((p.p2 - 1.0) * s.v[8])));
            s.store_scalar(247, (p.p2 * s.v[9]));
        }

        if (s.b[1333] && (!(((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]))) {
            s.store_scalar(248, (p.p2 * s.v[2]));
            s.store_scalar(249, (s.v[1] + ((p.p2 - 1.0) * s.v[3])));
            s.store_scalar(246, (p.p2 * s.v[8]));
            s.store_scalar(247, (s.v[7] + ((p.p2 - 1.0) * s.v[9])));
        }

        if (!((((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]) || s.b[1333])) {
            s.store_scalar(248, 0.0);
            s.store_scalar(249, 0.0);
            s.store_scalar(246, 0.0);
            s.store_scalar(247, 0.0);
        }

        s.b[1334] = param_given[24];
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if s.b[1334] {
            s.store_scalar(250, ((p.p24 * p.p53) * p.p52));
        }

        if (!s.b[1334]) {
            s.copy_ad(250, 246);
        }

        s.b[1335] = (s.v[250] < 0.0);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if s.b[1335] {
            s.store_scalar(250, 0.0);
        }

        s.b[1336] = param_given[25];
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if s.b[1336] {
            s.store_scalar(251, ((p.p25 * p.p53) * p.p52));
        }

        if (!s.b[1336]) {
            s.copy_ad(251, 247);
        }

        s.b[1337] = (s.v[251] < 0.0);
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if s.b[1337] {
            s.store_scalar(251, 0.0);
        }

        s.b[1338] = param_given[26];
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        s.b[1339] = (p.p137 == 0.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (s.b[1338] && s.b[1339]) {
            s.store_scalar(300, (p.p26 * p.p53));
        }

        if (s.b[1338] && (!s.b[1339])) {
            s.store_scalar(300, (((p.p26 * p.p53) - (s.v[35] * p.p2))).max(0.0));
        }

        if (!s.b[1338]) {
            s.copy_ad(300, 248);
        }

        s.b[1340] = (s.v[300] < 0.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if ((!s.b[1338]) && s.b[1340]) {
            s.store_scalar(300, 0.0);
        }

        s.b[1341] = param_given[27];
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        s.b[1342] = (p.p137 == 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1342]) {
            s.store_scalar(301, (p.p27 * p.p53));
        }

        if (s.b[1341] && (!s.b[1342])) {
            s.store_scalar(301, (((p.p27 * p.p53) - (s.v[35] * p.p2))).max(0.0));
        }

        if (!s.b[1341]) {
            s.copy_ad(301, 249);
        }

        s.b[1343] = (s.v[301] < 0.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((!s.b[1341]) && s.b[1343]) {
            s.store_scalar(301, 0.0);
        }

        s.store_add_scaled_ad_lhs(341, A::add_scaled_products(s.ad_value(250), s.ad_value(435), 1.0, s.ad_value(300), s.ad_value(436), 1.0), 437, (s.v[35] * p.p2));

        s.b[1344] = (s.v[341] > 0.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if s.b[1344] {
            s.store_scale(343, 393, p.p725);
            s.store_scaled_limited_exp_ad(351, A::div_from_scalar((-p.p731), s.ad_value(343)), p.p733);
            s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p727, s.ad_value(341)), 10.0);
            s.store_sub_ad_lhs(25, A::offset(s.ad_value(14), 1.0), 351);
            s.store_mul_ln_ad_rhs(350, 343, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(351), 4.0)), 0.5), 1e-38));
            s.store_limited_exp_div(12, 350, 343);
            s.store_mul_offset_ad_rhs(349, 341, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(351), s.ad_value(12)), (-1.0), s.ad_value(351), 1.0), (-1.0));
            s.store_ad_value(348, A::div_scaled_product(s.ad_value(341), A::add(s.ad_value(12), A::div(s.ad_value(351), s.ad_value(12))), 1.0, s.ad_value(343), 1.0));
        }

    }
}
