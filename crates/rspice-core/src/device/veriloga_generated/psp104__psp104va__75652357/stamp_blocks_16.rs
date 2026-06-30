#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1994] {
            s.store_scaled_voltage(826, ctx, nodes, Some(6), Some(10), -1.0);
            s.store_scaled_voltage(827, ctx, nodes, Some(7), Some(11), -1.0);
        }

        if (!s.b[1994]) {
            s.store_scaled_voltage(819, ctx, nodes, Some(5), Some(6), -1.0);
            s.store_scaled_voltage(820, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(821, ctx, nodes, Some(6), Some(8), -1.0);
            s.store_voltage(826, ctx, nodes, Some(6), Some(10));
            s.store_voltage(827, ctx, nodes, Some(7), Some(11));
        }

        s.store_add(823, 819, 821);

        s.copy_ad(828, 819);

        s.copy_ad(829, 821);

        s.store_add(830, 820, 821);

        s.store_sub(831, 819, 820);

        s.store_scale(1805, 828, (-s.v[349]));

        s.store_scale(1806, 831, (-s.v[349]));

        s.store_scaled_sub(1807, 823, 694, (-s.v[349]));

        s.store_scalar(825, 1.0);

        s.b[1995] = (s.v[820] < 0.0);
        s.store_scalar(1995, if s.b[1995] { 1.0 } else { 0.0 });

        if s.b[1995] {
            s.store_scalar(825, (-1.0));
            s.store_sub(819, 819, 820);
            s.store_add(821, 821, 820);
            s.store_neg(820, 820);
        }

        s.store_add(822, 820, 821);

        s.store_div_scaled_product_offset_denominator(824, s.ad_value(820), s.ad_value(820), 1.0, A::sqrt_square_offset(s.ad_value(820), 0.01), 0.1, 1.0);

        s.store_add_scaled_inputs4_mixed_iiai(1999, 822, 0.5, 821, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(733))), (-0.5), 731, 1.0);

        s.copy_ad(1808, 1999);

        s.store_add_scaled_inputs4_mixed_iiai(1922, 821, 1.0, 1999, (-0.5), A::sqrt(A::add(A::square(s.ad_value(1999)), s.ad_value(732))), (-(-0.5)), 735, 1.0);

        s.copy_ad(1809, 1922);

        s.store_scalar(1923, 0.0);

        s.b[2155] = ((p.p45 != 0.0) && (s.v[179] != 1.0));
        s.store_scalar(2155, if s.b[2155] { 1.0 } else { 0.0 });

        if s.b[2155] {
            s.store_add_scaled_inputs3_indices(1924, 1922, 1.0, 820, 0.5, 824, (-0.5));
            s.store_sub_ad_lhs(1925, A::sqrt(A::add(s.ad_value(1924), s.ad_value(722))), 730);
            s.store_offset_div_scaled_inputs2_indices(1919, 1925, 2.0, 737, (-2.0), 738, 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(1926, 1925, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(179), s.ad_value(738), 0.25), A::add(s.ad_value(1919), A::sqrt_square_offset(s.ad_value(1919), 0.4804530139182)), (-1.0));
            s.store_add_scaled_square_product_indices(1927, 1926, 1.0, 730, 1926, 2.0);
            s.store_add_scaled_inputs3_indices(1922, 1927, 1.0, 820, (-0.5), 824, (-(-0.5)));
            s.store_sub(1923, 1809, 1922);
        }

        s.copy_ad(1996, 722);

        s.copy_ad(1997, 732);

        s.copy_ad(1998, 723);

        s.copy_ad(2000, 1922);

        s.copy_ad(2004, 1923);

        s.copy_ad(2001, 714);

        s.copy_ad(2002, 771);

        s.store_add_scaled_inputs3_indices(2003, 823, 1.0, 2004, (-1.0), 694, -1.0);

        s.store_add_scaled_inputs3_indices(2005, 2000, 1.0, 820, 0.5, 824, (-0.5));

        s.store_scalar(2017, 1.0);

        s.b[2156] = (s.v[185] > 0.0);
        s.store_scalar(2156, if s.b[2156] { 1.0 } else { 0.0 });

        if s.b[2156] {
            s.store_scale(2008, 1996, s.v[355]);
            s.store_scale(2009, 2005, s.v[355]);
            s.store_scale(2010, 2003, s.v[355]);
            s.store_offset_div_scaled_inputs_mixed_ia(1920, 1998, 0.5, A::sqrt(s.ad_value(2008)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(1921, 2008, 1.0, 1998, A::sqrt(s.ad_value(2008)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2011, A::div_scaled_inputs2(s.ad_value(2010), 1.0, s.ad_value(1921), (-1.0), s.ad_value(1920), 1.0), 1.0, 2008, 0.5, A::offset(s.ad_value(186), 1.0), 2009, (-1.0));
            s.store_offset_scaled(2012, 2008, 0.5, 2.0);
            s.store_add(2013, 2008, 2009);
            s.store_sub_scaled_inputs_ad(1920, A::add_scaled_inputs_product(s.ad_value(2010), 1.0, s.ad_value(2013), (-1.0), s.ad_value(1998), A::sqrt(s.ad_value(2013)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2008), s.ad_value(1998)), A::sqrt(s.ad_value(2008)))), 2.0);
            s.store_add_scaled_inputs(2014, 1920, 2.0, 2012, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2011, 0.5, 2014, 0.5, 2011, 2014, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(1921, 2010, 2.0, 2009, (-2.0), 2012, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2015, 1920, 0.5, 1921, 0.5, 1920, 1921, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2015, 0.5, 2012, 0.5, 2015, 2012, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2016, 1920, 0.5, 2012, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2012), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1921, 696, A::div(s.ad_value(2016), s.ad_value(2012)), 1.0);
        }

        s.b[2157] = (s.v[1921] > (-230.25850929940458));
        s.store_scalar(2157, if s.b[2157] { 1.0 } else { 0.0 });

        if (s.b[2156] && s.b[2157]) {
            s.store_exp(2017, 1921);
        }

        if (s.b[2156] && (!s.b[2157])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2017, 1e-100, (-230.25850929940458), 1921, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2018, 695, 2017, 1.0);

        s.store_scale(2019, 2018, s.v[709]);

        s.store_mul_ad_product_rhs(2020, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2005)), 1.0));

        s.store_mul_offset_rhs(2021, 2019, 2020, 1.0);

        s.store_div_from_scalar(2022, 1.0, 2021);

        s.store_mul_ad_rhs(2006, 1998, A::sqrt_scaled_input(s.ad_value(2022), s.v[709]));

        s.store_square(2007, 2006);

        s.store_div_from_scalar(2023, 1.0, 2007);

        s.store_mul(2024, 2000, 2022);

        s.store_mul(2025, 2003, 2022);

        s.store_div_scaled_value_offset_denominator(2026, s.ad_value(824), 2.0, A::sqrt_product_offset(s.ad_value(192), s.ad_value(824), 1.0), 1.0, 1.0);

        s.store_mul_ad_product_rhs_mixed_ia(2027, 191, 2026, A::offset(A::mul(s.ad_value(193), s.ad_value(2005)), 1.0));

        s.store_mul(2028, 1996, 2022);

        s.store_sqrt_square_add(1920, 1999, 1997);

        s.store_sqrt_add_ad(1921, A::square(A::sub(s.ad_value(1999), s.ad_value(2027))), s.ad_value(1997));

        s.store_mul_add_scaled_inputs3_offset_rhs(2029, 2022, s.ad_value(2027), 0.5, s.ad_value(1920), 0.5, s.ad_value(1921), ((-1.0) * (0.5)), 0.0);

        s.store_add(2030, 2028, 2024);

        s.store_sub(2031, 2030, 2029);

        s.b[2158] = (p.p45 > 0.0);
        s.store_scalar(2158, if s.b[2158] { 1.0 } else { 0.0 });

        s.b[2159] = (((s.v[2031]) as f64).abs() < 1e-5);
        s.store_scalar(2159, if s.b[2159] { 1.0 } else { 0.0 });

        if (s.b[2158] && s.b[2159]) {
            s.store_offset_ad(2032, A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2031), 1.0, A::scale(s.ad_value(2031), 0.3125), 0.5)), 1.0);
        }

        s.b[2160] = (s.v[2031] < 460.51701859880916);
        s.store_scalar(2160, if s.b[2160] { 1.0 } else { 0.0 });

        if ((s.b[2158] && (!s.b[2159])) && s.b[2160]) {
            s.store_exp_neg_input(2046, 2031);
        }

        if ((s.b[2158] && (!s.b[2159])) && (!s.b[2160])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2046, 1e-200, 2031, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2158] && (!s.b[2159])) {
            s.store_scalar(1919, (if (s.v[2031] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2158] && (!s.b[2159])) {
            s.store_offset_ad(2032, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2006), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2031))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2031), 1.0, s.ad_value(2046))), 2.0), 1.0);
        }

        if (!s.b[2158]) {
            s.store_offset_div_scaled_inputs_mixed_ia(2032, 2006, 0.5, A::sqrt(s.ad_value(2031)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2033, s.ad_value(2031), 1.0, s.ad_value(2006), A::sqrt(s.ad_value(2031)), 1.0, s.ad_value(2032), A::ln(A::offset(s.ad_value(2032), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2_indices(2034, 2025, 1.0, 2033, (-1.0), 2032, 1.0);

        s.store_mul_scaled_offset_ad_rhs(2040, 2007, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2007)), 1.0)), (-1.0));

        s.store_scalar(2039, 0.0);

        s.store_scalar(2041, 1.0);

        s.b[2161] = (s.v[2034] > (-30.0));
        s.store_scalar(2161, if s.b[2161] { 1.0 } else { 0.0 });

        if s.b[2161] {
            s.store_offset_mul(2035, 2032, 2034, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1919, 2035, 2035, 10.0, 0.5);
            s.store_sub_ad_rhs(2036, 2034, A::ln(s.ad_value(1919)));
            s.store_scaled_add_sqrt_square_offset_rhs(2037, 2036, 2036, 2.0, 0.5);
        }

        s.b[2162] = ((s.v[2034] - s.v[2037]) < 230.25850929940458);
        s.store_scalar(2162, if s.b[2162] { 1.0 } else { 0.0 });

        if (s.b[2161] && s.b[2162]) {
            s.store_exp_sub(1919, 2034, 2037);
        }

        if (s.b[2161] && (!s.b[2162])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1919, A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if s.b[2161] {
            s.store_div(2038, 1919, 2032);
            s.store_sub_ad_lhs(1919, A::scaled_offset(s.ad_value(2037), 1.0, 2.0), 2038);
        }

        s.b[2163] = (s.v[2038] > 1e-6);
        s.store_scalar(2163, if s.b[2163] { 1.0 } else { 0.0 });

        if (s.b[2161] && s.b[2163]) {
            s.store_mul_offset_ad_rhs(2039, 2032, A::sub(s.ad_value(2037), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2038), s.ad_value(1919), 1.0), 1.0, (-1.0), s.ad_value(2038), 1.0)), 1.0);
        }

        if (s.b[2161] && (!s.b[2163])) {
            s.store_mul_ad_affine_product_rhs(2039, 2032, s.ad_value(2038), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);
        }

        if s.b[2161] {
            s.store_add_scaled_inputs3_offset_mixed_iia(1919, 2025, 0.5, 2039, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2025), s.ad_value(2039)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2040, 2007, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2007)), s.ad_value(1919), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2041, 2040, 2040, 1.0, 2039, 1.0);
            s.store_add_scaled_product_indices(2031, 2030, 1.0, 2041, 2029, (-1.0));
        }

        s.store_offset_scaled(2042, 2006, 0.7071067811865475, 1.0);

        s.store_scale(2043, 2042, 1e-5);

        s.store_div_from_scalar(2044, 1.0, 2042);

        s.store_scalar(2151, 0.0);

        s.store_scalar(2045, 0.0);

        s.b[2164] = (s.v[2031] < 460.51701859880916);
        s.store_scalar(2164, if s.b[2164] { 1.0 } else { 0.0 });

        if s.b[2164] {
            s.store_exp_neg_input(2046, 2031);
        }

        if (!s.b[2164]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2046, 1e-200, 2031, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2165] = (((s.v[2025]) as f64).abs() <= s.v[2043]);
        s.store_scalar(2165, if s.b[2165] { 1.0 } else { 0.0 });

        if s.b[2165] {
            s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2045, 2025, 2044, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2046)), s.ad_value(2006), s.ad_value(2131)), 1.0));
        }

        s.b[2166] = (s.v[2025] < (-s.v[2043]));
        s.store_scalar(2166, if s.b[2166] { 1.0 } else { 0.0 });

        if ((!s.b[2165]) && s.b[2166]) {
            s.store_neg(2133, 2025);
            s.store_scaled_mul(2134, 2133, 2044, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2135, 2134, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2130, 2133, 2135);
            s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::offset(s.ad_value(2135), 1.0), 1.0);
            s.store_sub_scaled_inputs(2137, 2130, 2.0, 2007, 1.0);
            s.store_sub_ln_mul_lhs(2138, 2136, 2023, 2135);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.5, s.ad_value(2136), 1.0), 1.0);
            s.store_add_ad_rhs(2139, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.3333333333333333, s.ad_value(2136), 1.0))), 1.0));
        }

        s.b[2167] = (s.v[2139] < 230.25850929940458);
        s.store_scalar(2167, if s.b[2167] { 1.0 } else { 0.0 });

        if (((!s.b[2165]) && s.b[2166]) && s.b[2167]) {
            s.store_exp(2140, 2139);
        }

        if (((!s.b[2165]) && s.b[2166]) && (!s.b[2167])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2140, 2139, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((!s.b[2165]) && s.b[2166]) {
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_div_from_scalar_offset_square(2130, 1.0, 2139, 2.0);
            s.store_mul_square_lhs(2142, 2139, 2130);
            s.store_mul3_affine_lhs(2143, 2139, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);
            s.store_sub(2130, 2133, 2139);
            s.store_mul(2131, 2046, 2141);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2131), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2143)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2139), (-1.0), s.ad_value(2131), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::sub(A::offset(s.ad_value(2139), (-1.0)), s.ad_value(2142)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2140), 1.0, s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2045, 2139, -1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_div_from_scalar_offset_scaled_input(2147, 1.0, 2006, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2148, A::mul_scaled_lhs(s.ad_value(2042), 1.25, s.ad_value(2147)), (-1.0), 2147);
            s.store_mul_ad_product_rhs_mixed_ia(2149, 2025, 2044, A::offset(A::mul(s.ad_value(2148), s.ad_value(2025)), 1.0));
        }

        s.b[2168] = ((-s.v[2149]) > (-230.25850929940458));
        s.store_scalar(2168, if s.b[2168] { 1.0 } else { 0.0 });

        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2168]) {
            s.store_exp_neg_input(2130, 2149);
        }

        if (((!s.b[2165]) && (!s.b[2166])) && (!s.b[2168])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2130, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2149)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_sub_from_scalar(2150, 1.0, 2130);
            s.store_add_scaled_inputs_product_right_ad(2151, 2025, 1.0, 2007, 0.5, 2006, A::sqrt(A::add_scaled_inputs3(s.ad_value(2025), 1.0, s.ad_value(2007), 0.25, s.ad_value(2150), -1.0)), (-1.0));
            s.store_offset(2152, 2031, 3.0);
            s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2151), s.ad_value(2152)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt_square_offset(s.ad_value(2152), 5.0), 0.5));
            s.store_sub(2130, 2025, 2135);
            s.store_exp_neg_input(2131, 2135);
            s.store_div_from_scalar_offset_square(2132, 1.0, 2135, 2.0);
            s.store_mul_square_lhs(2142, 2135, 2132);
            s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), 2132, 2132);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2046] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                s.store_scalar(2136, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            }
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2153, 1.0, 2007, A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2138, 2031, 1.0, 2135, (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);
            s.store_add_ad_rhs(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));
        }

        s.b[2169] = (s.v[2154] < 230.25850929940458);
        s.store_scalar(2169, if s.b[2169] { 1.0 } else { 0.0 });

        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2169]) {
            s.store_exp(2140, 2154);
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_mul(2140, 2046, 2140);
        }

        s.b[2170] = (s.v[2154] > (s.v[2031] - 230.25850929940458));
        s.store_scalar(2170, if s.b[2170] { 1.0 } else { 0.0 });

        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && s.b[2170]) {
            s.store_exp_sub(2140, 2154, 2031);
            s.store_div(2141, 2046, 2140);
        }

        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && (!s.b[2170])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2140, 1e-100, A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2141, 1e-100, 2154, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_div_from_scalar_offset_square(2130, 1.0, 2154, 2.0);
            s.store_mul_square_lhs(2142, 2154, 2130);
            s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);
            s.store_sub(2130, 2025, 2154);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2045, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        s.store_scalar(2048, 0.0);

        s.store_scalar(2049, 0.0);

        s.store_scalar(2050, 0.0);

        s.store_scalar(2051, 0.0);

        s.store_scalar(2052, 0.0);

        s.store_scalar(2053, 0.0);

        s.store_scalar(2054, 0.0);

        s.store_scalar(2055, 1.0);

        s.store_scalar(2056, 1.0);

        s.store_sub(2057, 2025, 2045);

        s.store_scalar(2058, 0.0);

        s.store_mul(2059, 2021, 2057);

        s.store_scalar(2060, 1.0);

        s.store_scalar(2061, 1.0);

        s.store_scalar(2065, 1.0);

        s.store_scalar(2066, 1.0);

        s.store_scalar(2068, 1.0);

        s.b[2171] = (s.v[2025] > 0.0);
        s.store_scalar(2171, if s.b[2171] { 1.0 } else { 0.0 });

        if s.b[2171] {
            s.store_div_from_scalar_offset_square(1919, 1.0, 2045, 2.0);
            s.store_mul_square_lhs(2047, 2045, 1919);
            s.store_mul3_affine_lhs(2048, 2045, 1919, 4.0, 0.0, 1919);
            s.store_mul_ad_product_lhs_mixed_ai(2049, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2047), 12.0), 1919, 1919);
            s.store_scalar(2050, 0.0);
        }

        s.b[2172] = (s.v[2045] < 230.25850929940458);
        s.store_scalar(2172, if s.b[2172] { 1.0 } else { 0.0 });

        if (s.b[2171] && s.b[2172]) {
            s.store_exp(2050, 2045);
            s.store_div_from_scalar(2051, 1.0, 2050);
            s.store_mul(2050, 2046, 2050);
        }

        s.b[2173] = (s.v[2045] > (s.v[2031] - 230.25850929940458));
        s.store_scalar(2173, if s.b[2173] { 1.0 } else { 0.0 });

        if ((s.b[2171] && (!s.b[2172])) && s.b[2173]) {
            s.store_exp_sub(2050, 2045, 2031);
            s.store_div(2051, 2046, 2050);
        }

        if ((s.b[2171] && (!s.b[2172])) && (!s.b[2173])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2050, 1e-100, A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2051, 1e-100, 2045, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2171] {
            s.store_add_scaled_product_right_ad(2052, 2050, 1.0, 2046, A::add(A::offset(s.ad_value(2045), 1.0), s.ad_value(2047)), (-1.0));
        }

        s.b[2174] = (s.v[2045] < 1e-5);
        s.store_scalar(2174, if s.b[2174] { 1.0 } else { 0.0 });

        if (s.b[2171] && s.b[2174]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2053, 2045, 1.0, 2045, 1.0, 2045, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2052, A::mul3(s.ad_value(2046), s.ad_value(2045), s.ad_value(2045)), 2045, A::scale_offset(s.ad_value(2045), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2054, 2045, 1919, 0.7071067811865475);
            s.store_offset_div_scaled_product(2055, s.ad_value(2006), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.5)), 1.0, A::square(s.ad_value(2045)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0, 1.0);
        }

        if (s.b[2171] && (!s.b[2174])) {
            s.store_add_offset_lhs(2053, 2045, (-1.0), 2051);
            s.store_sqrt(2054, 2053);
            s.store_offset_scaled_ad(2055, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2051)), s.ad_value(2054)), 0.5, 1.0);
        }

        if s.b[2171] {
            s.store_div_scaled_offset_numerator(2056, A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2005)), 1.0, 1.0, A::offset(A::mul(s.ad_value(702), s.ad_value(2005)), 1.0), 1.0);
        }

        s.b[2175] = (s.v[2052] > 1e-100);
        s.store_scalar(2175, if s.b[2175] { 1.0 } else { 0.0 });

        if (s.b[2171] && s.b[2175]) {
            s.store_mul_sqrt_ad_rhs(2057, 2006, A::add(s.ad_value(2053), s.ad_value(2052)));
            s.store_div_scaled_product3_mixed_iiia(2058, 2007, 2052, 2021, 1.0, A::add_scaled_product(s.ad_value(2057), 1.0, s.ad_value(2006), s.ad_value(2054), 1.0), 1.0);
            s.store_mul3_lhs(2059, 2054, 2006, 2021);
        }

        s.b[2176] = (s.v[212] < 0.0);
        s.store_scalar(2176, if s.b[2176] { 1.0 } else { 0.0 });

        if ((s.b[2171] && s.b[2175]) && s.b[2176]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2060, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2005)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2176])) {
            s.store_offset_mul(2060, 212, 2005, 1.0);
        }

        s.b[2177] = (s.v[213] < 0.0);
        s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });

        if ((s.b[2171] && s.b[2175]) && s.b[2177]) {
            s.store_sub_from_scalar_scaled_mul(2061, 1.0, 213, 2058, 1.0);
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2177])) {
            s.store_div_from_scalar_offset_product(2061, 1.0, 213, 2058, 1.0);
        }

        if (s.b[2171] && s.b[2175]) {
            s.store_mul_product3_indices(2062, 2058, 751, 2060, 2061, 1.0);
            s.store_mul_add_scaled_product_rhs(2063, 768, s.ad_value(2059), 1.0, s.ad_value(769), s.ad_value(2058), 1.0);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2053), 1.0, A::add(s.ad_value(2053), s.ad_value(2052)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2063), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2065, A::offset(s.ad_value(2064), 1.0), s.ad_value(2062), 2056);
        }

        s.b[2178] = (s.v[216] < 0.0);
        s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });

        if ((s.b[2171] && s.b[2175]) && s.b[2178]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2066, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2005)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2178])) {
            s.store_offset_mul(2066, 216, 2005, 1.0);
        }

        if (s.b[2171] && s.b[2175]) {
            s.store_mul(1921, 2058, 2066);
            s.store_div_add_scaled_inputs_rhs_indices(2067, 1921, 218, 1.0, 1921, 1.0);
        }

        s.b[2179] = (s.v[217] < 0.0);
        s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });

        if ((s.b[2171] && s.b[2175]) && s.b[2179]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2179])) {
            s.store_offset_mul(2068, 217, 2067, 1.0);
        }

        s.copy_ad(1810, 2003);

        s.copy_ad(1811, 2005);

        s.copy_ad(1812, 2021);

        s.copy_ad(1813, 2022);

        s.copy_ad(1814, 2006);

        s.copy_ad(1815, 2007);

        s.copy_ad(1816, 2023);

        s.copy_ad(1817, 2025);

        s.copy_ad(1818, 2030);

        s.copy_ad(1819, 2031);

        s.copy_ad(1820, 2042);

        s.copy_ad(1821, 2043);

        s.copy_ad(1822, 2044);

        s.copy_ad(1823, 2151);

        s.copy_ad(1824, 2046);

        s.copy_ad(1825, 2045);

        s.copy_ad(1826, 2048);

        s.copy_ad(1827, 2049);

        s.copy_ad(1828, 2050);

        s.copy_ad(1829, 2051);

        s.copy_ad(1830, 2053);

        s.copy_ad(1831, 2052);

        s.copy_ad(1832, 2054);

        s.copy_ad(1833, 2055);

        s.copy_ad(1834, 2056);

        s.copy_ad(1835, 2057);

        s.copy_ad(1836, 2058);

        s.copy_ad(1837, 2059);

        s.copy_ad(1838, 2060);

        s.copy_ad(1839, 2061);

        s.copy_ad(1840, 2065);

        s.copy_ad(1841, 2066);

        s.copy_ad(1842, 2068);

        s.store_scalar(2070, 0.0);

        s.store_scale(2069, 2021, 4.60517018598809);

        s.copy_ad(2086, 2069);

        s.copy_ad(2087, 820);

        s.store_mul(2088, 820, 2022);

        s.copy_ad(2092, 2045);

        s.store_scalar(2093, 0.0);

        s.store_scalar(2096, 0.0);

        s.copy_ad(2098, 2051);

        s.copy_ad(2099, 2053);

        s.copy_ad(2101, 2052);

        s.copy_ad(2102, 2059);

        s.copy_ad(2103, 2045);

        s.copy_ad(2104, 2051);

        s.copy_ad(2106, 2052);

        s.copy_ad(2107, 2053);

        s.store_sub(2108, 2025, 2045);

        s.store_scalar(2109, 1.0);

        s.store_scalar(2111, 1.0);

        s.store_scalar(2110, 0.0);

        s.copy_ad(2120, 2058);

        s.store_mul(2124, 2108, 2021);

        s.store_scalar(2121, 0.0);

        s.copy_ad(2122, 2059);

        s.store_scalar(2127, 0.0);

        s.store_scalar(2126, 1.0);

        s.copy_ad(2129, 2001);

        s.copy_ad(2128, 2124);

        s.b[2180] = (s.v[2025] > 0.0);
        s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });

        s.b[2181] = (s.v[2052] > 1e-100);
        s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2181]) {
            s.store_mul(2129, 2001, 2068);
            s.store_div(2070, 2129, 2065);
            s.store_add_scaled_inputs(2071, 2057, 1.0, 2007, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2180] && s.b[2181]) {
            s.store_div_scaled_product_by_product(1919, s.ad_value(2007), s.ad_value(2050), 1.0, s.ad_value(2071), s.ad_value(2071), 1.0);
        }

        s.b[2182] = (s.v[1919] > 0.0001);
        s.store_scalar(2182, if s.b[2182] { 1.0 } else { 0.0 });

        if ((s.b[2180] && s.b[2181]) && s.b[2182]) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.b[2183] = (s.v[1920] < 1e-10);
        s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });

        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && s.b[2183]) {
            s.store_scalar(1921, 1.0);
        }

        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && (!s.b[2183])) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if ((s.b[2180] && s.b[2181]) && (!s.b[2182])) {
            s.store_scale(1921, 1919, 0.5);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul(2072, 1921, 2071);
        }

        s.b[2184] = ((s.v[700] > 0.0) && (s.v[701] > 0.0));
        s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });

        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {
            s.store_scaled_mul(2073, 2021, 2072, 0.475);
            s.store_add_scaled_product_indices(1919, 2058, 1.0, 2055, 2073, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2074, 1919, 1919, 1e-12, 0.5);
            s.store_add_scaled_value_products(2075, s.ad_value(2058), (-1.0), s.ad_value(2021), s.ad_value(2057), 1.0, A::offset(s.ad_value(2055), (-1.0)), s.ad_value(2073), 1.0);
            s.store_offset_div_scaled_product(2076, s.ad_value(2007), s.ad_value(2021), 0.5, s.ad_value(2075), 1.0, 1.0);
            s.store_add_scaled_product_indices(1919, 2075, 1.0, 769, 2074, 1.0);
            s.store_pow_ad(2077, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));
            s.store_mul_ad_lhs(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2076), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2077);
            s.store_div(1919, 2074, 2075);
            s.store_mul_pow_ad_rhs(2078, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));
            s.store_mul_div_scaled_product_mixed_iiai(1921, 2078, 701, A::add(A::offset(s.ad_value(2076), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1919), 1.0, 1.0)), 1.0, 2075, 1.0);
            s.store_mul_product3_indices(2079, 2074, 751, 2060, 2061, 1.0);
            s.store_offset_ad(1919, A::div_scaled_add_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2060), s.ad_value(2061)), s.ad_value(2076), (-1.0), s.ad_value(1921), 1.0), 1.0);
        }

        s.b[2185] = (s.v[1919] < 230.25850929940458);
        s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });

        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && s.b[2185]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);
        }

        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && (!s.b[2185])) {
            s.copy_ad(1920, 1919);
        }

        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {
            s.store_div_scaled_product3_mixed_iiia(2080, 2073, 1921, 1920, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2077), 1.0, s.ad_value(2078), 1.0, s.ad_value(2079), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2081, 2072, A::div_scaled_value_offset_denominator(s.ad_value(2080), 1.0, A::sqrt_square_offset(s.ad_value(2080), 1.0), 1.0, 1.0), 1.0);
        }

        if ((s.b[2180] && s.b[2181]) && (!s.b[2184])) {
            s.copy_ad(2081, 2072);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul3_affine_lhs(2082, 2021, 2070, 0.7071067811865475, 0.0, 2081);
        }

        s.b[2186] = (s.v[0] == (-1.0));
        s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });

        if ((s.b[2180] && s.b[2181]) && s.b[2186]) {
            s.store_div_ad_rhs(2082, 2082, A::sqrt(A::offset(s.ad_value(2082), 1.0)));
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_div_from_scalar_offset_ad(2083, 2.0, A::sqrt(A::scale_offset(s.ad_value(2082), 4.0, 1.0)), 1.0);
            s.store_mul(1919, 2083, 2082);
            s.store_mul_ad_product_rhs_mixed_ia(2084, 2081, 2083, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2083)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2083), 4.0), 1.0)), 1.0));
            s.store_scale(2085, 2084, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1919, 2085, A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2071), 2.0), 2023, 1.0, 2052, 1.0);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul_sub_ad_rhs(2086, 2021, s.ad_value(2085), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2180] && (!s.b[2181])) {
            s.copy_ad(2086, 2069);
        }

        if s.b[2180] {
            s.store_offset(1919, 2002, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 2086, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2087, 2086, 1919, 1.0, A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), 1.0, A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919))), 1.0, 1.0);
            s.store_mul(2088, 2087, 2022);
            s.store_add(2089, 2031, 2088);
        }

        s.b[2187] = (s.v[2088] < 460.51701859880916);
        s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2187]) {
            s.store_exp_neg_input(2090, 2088);
        }

        if (s.b[2180] && (!s.b[2187])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2090, 1e-200, 2088, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2180] {
            s.store_mul(2091, 2046, 2090);
        }

        s.b[2188] = (((s.v[2025]) as f64).abs() <= s.v[2043]);
        s.store_scalar(2188, if s.b[2188] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2188]) {
            s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2092, 2025, 2044, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2091)), s.ad_value(2006), s.ad_value(2131)), 1.0));
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_offset(2152, 2089, 3.0);
            s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2151), s.ad_value(2152)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt_square_offset(s.ad_value(2152), 5.0), 0.5));
            s.store_sub(2130, 2025, 2135);
            s.store_exp_neg_input(2131, 2135);
            s.store_div_from_scalar_offset_square(2132, 1.0, 2135, 2.0);
            s.store_mul_square_lhs(2142, 2135, 2132);
            s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), 2132, 2132);
        }

        if (s.b[2180] && (!s.b[2188])) {
            if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2091] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                s.store_scalar(2136, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            }
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2153, 1.0, 2007, A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2138, 2089, 1.0, 2135, (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);
            s.store_add_ad_rhs(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));
        }

        s.b[2189] = (s.v[2154] < 230.25850929940458);
        s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });

        if ((s.b[2180] && (!s.b[2188])) && s.b[2189]) {
            s.store_exp(2140, 2154);
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_mul(2140, 2091, 2140);
        }

        s.b[2190] = (s.v[2154] > (s.v[2089] - 230.25850929940458));
        s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });

        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && s.b[2190]) {
            s.store_exp_sub(2140, 2154, 2089);
            s.store_div(2141, 2091, 2140);
        }

        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && (!s.b[2190])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2140, 1e-100, A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2141, 1e-100, 2154, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_div_from_scalar_offset_square(2130, 1.0, 2154, 2.0);
            s.store_mul_square_lhs(2142, 2154, 2130);
            s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);
            s.store_sub(2130, 2025, 2154);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2092, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        if s.b[2180] {
            s.store_sub(2093, 2092, 2045);
        }

        s.b[2191] = (s.v[2093] < 1e-10);
        s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2191]) {
            s.store_add_scaled_inputs_product_right_ad(2094, 2025, 2.0, 2045, (-2.0), 2007, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0), 1.0, s.ad_value(2091), s.ad_value(2048), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2095, A::mul_sub_from_scalar_rhs(s.ad_value(2007), 1.0, s.ad_value(2090)), 2052);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1919, 2.0, 2007, A::add_scaled_value_products(s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0, s.ad_value(2091), s.ad_value(2049), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1919, 2094, 1.0, 1919, 2095, (-2.0));
            s.store_scaled_div_ad_rhs(2093, 2095, A::add(s.ad_value(2094), A::sqrt(s.ad_value(1919))), 2.0);
            s.store_add(2092, 2045, 2093);
        }

        if s.b[2180] {
            s.store_mul(2096, 2093, 2021);
            s.store_div_scaled_product_offset_denominator(2097, s.ad_value(2092), s.ad_value(2092), 1.0, A::square(s.ad_value(2092)), 2.0, 1.0);
        }

        s.b[2192] = (s.v[2092] < 230.25850929940458);
        s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2192]) {
            s.store_exp_neg_input(2098, 2092);
        }

        s.b[2193] = (s.v[2092] < 1e-5);
        s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });

        if ((s.b[2180] && s.b[2192]) && s.b[2193]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2099, 2092, 1.0, 2092, 1.0, 2092, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2092), 1.0, A::scale(s.ad_value(2092), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2100, 2092, 1919, 0.7071067811865475);
            s.store_mul3_ad_middle(2101, A::mul3_scaled_output(s.ad_value(2091), s.ad_value(2092), s.ad_value(2092), 0.16666666666666666), 2092, A::scale_offset(s.ad_value(2092), 1.75, 1.0));
        }

        if ((s.b[2180] && s.b[2192]) && (!s.b[2193])) {
            s.store_add_offset_lhs(2099, 2092, (-1.0), 2098);
            s.store_sqrt(2100, 2099);
            s.store_mul_add_scaled_inputs3_offset_rhs(2101, 2091, A::div_from_scalar(1.0, s.ad_value(2098)), 1.0, s.ad_value(2092), (-1.0), s.ad_value(2097), -1.0, (-1.0));
        }

        s.b[2194] = (s.v[2092] > (s.v[2089] - 230.25850929940458));
        s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });

        if ((s.b[2180] && (!s.b[2192])) && s.b[2194]) {
            s.store_exp_sub(1919, 2092, 2089);
            s.store_div(2098, 2091, 1919);
            s.store_add_scaled_product_right_ad(2101, 1919, 1.0, 2091, A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097)), (-1.0));
        }

        if ((s.b[2180] && (!s.b[2192])) && (!s.b[2194])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2098, 1e-100, 2092, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1919, 1e-100, A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2101, 1919, 1.0, 2091, A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097)), (-1.0));
        }

        if (s.b[2180] && (!s.b[2192])) {
            s.store_add_offset_lhs(2099, 2092, (-1.0), 2098);
            s.store_sqrt(2100, 2099);
        }

        if s.b[2180] {
            s.store_mul3_lhs(2102, 2100, 2006, 2021);
            s.store_scaled_add(2103, 2045, 2092, 0.5);
            s.store_scalar(2104, 0.0);
            s.store_mul(1919, 2098, 2051);
        }

        s.b[2195] = (s.v[1919] > 0.0);
        s.store_scalar(2195, if s.b[2195] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2195]) {
            s.store_sqrt(2104, 1919);
        }

        if s.b[2180] {
            s.store_scaled_add(2105, 2052, 2101, 0.5);
            s.store_add_scaled_product_mixed_iaa(2106, 2105, 1.0, A::square(s.ad_value(2093)), A::sub_scaled_inputs(s.ad_value(2104), 1.0, s.ad_value(2023), 2.0), 0.125);
        }

        s.b[2196] = (s.v[2103] < 1e-5);
        s.store_scalar(2196, if s.b[2196] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2196]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2107, 2103, 1.0, 2103, 1.0, 2103, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
        }

        s.b[2197] = (s.v[724] > 0.0);
        s.store_scalar(2197, if s.b[2197] { 1.0 } else { 0.0 });

        if ((s.b[2180] && s.b[2196]) && s.b[2197]) {
            s.store_div_from_scalar_sqrt_ad(2109, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0));
        }

        if (s.b[2180] && s.b[2196]) {
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2103), 1.0, A::scale(s.ad_value(2103), 0.25), 0.3333333333333333));
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2180] && s.b[2196]) {
            s.store_scaled_mul(2110, 2103, 1919, 0.7071067811865475);
            s.store_add_ad_rhs(2111, 2109, A::div_scaled_product(s.ad_value(2006), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2103), 0.5)), 1.0, A::square(s.ad_value(2103)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0));
        }

        if (s.b[2180] && (!s.b[2196])) {
            s.store_add_offset_lhs(2107, 2103, (-1.0), 2104);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
        }

        s.b[2198] = (s.v[724] > 0.0);
        s.store_scalar(2198, if s.b[2198] { 1.0 } else { 0.0 });

        if ((s.b[2180] && (!s.b[2196])) && s.b[2198]) {
            s.store_add_scaled_sub_value_product_indices(2112, 1.0, 2104, 1.0, 2108, 2023, 2.0);
            s.store_div_from_scalar_sqrt_ad(2109, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0));
            s.store_div_scaled_value_offset_denominator(1919, s.ad_value(2109), 1.0, s.ad_value(2109), 1.0, 1.0);
            s.store_mul_product3_mixed_iaii(2113, 724, A::square(s.ad_value(1919)), 2007, 2106, 1.0);
            s.store_add_scaled_inputs_product_right_ad(2114, 2108, 2.0, 2113, (-2.0), 2007, A::add(A::sub_from_scalar(1.0, s.ad_value(2104)), s.ad_value(2106)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2115, 2113, s.ad_value(2113), 1.0, s.ad_value(2108), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2116, 1.0, 2007, A::add(s.ad_value(2104), s.ad_value(2106)), 0.5);
            s.store_div_scaled_product_denominator_ad(2117, 2115, 2114, 1.0, A::add_scaled_square_product(s.ad_value(2114), 1.0, s.ad_value(2116), s.ad_value(2115), (-1.0)), 1.0);
            s.store_add(2103, 2103, 2117);
            s.store_exp(2118, 2117);
            s.store_div(2104, 2104, 2118);
            s.store_mul(2106, 2106, 2118);
            s.store_add_offset_lhs(2107, 2103, (-1.0), 2104);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
            s.store_add_ad(2119, A::sub_from_scalar(1.0, s.ad_value(2104)), A::mul3_scaled_output(s.ad_value(2108), s.ad_value(2109), s.ad_value(2023), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2093, 2093, 2118, A::add(s.ad_value(2112), s.ad_value(2105)), 1.0, A::add_scaled_product(s.ad_value(2119), 1.0, s.ad_value(2118), s.ad_value(2105), 1.0), 1.0);
            s.store_mul(2096, 2093, 2021);
        }

        if (s.b[2180] && (!s.b[2196])) {
            s.store_sqrt(2110, 2107);
            s.store_add_scaled_inputs_ad_rhs(2111, 2109, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2104)), s.ad_value(2110)), 0.5);
        }

        if s.b[2180] {
            s.store_mul_div_scaled_product_mixed_iiia(2120, 2021, 2007, 2106, 1.0, A::add_scaled_product(s.ad_value(2108), 1.0, s.ad_value(2006), s.ad_value(2110), 1.0), 1.0);
            s.store_add_scaled_product_indices(2121, 2120, 1.0, 2021, 2111, 1.0);
            s.store_mul3_lhs(2122, 2110, 2006, 2021);
        }

        s.b[2199] = (s.v[213] < 0.0);
        s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2199]) {
            s.store_sub_from_scalar_scaled_mul(2061, 1.0, 213, 2120, 1.0);
        }

        if (s.b[2180] && (!s.b[2199])) {
            s.store_div_from_scalar_offset_product(2061, 1.0, 213, 2120, 1.0);
        }

        if s.b[2180] {
            s.store_mul_product3_indices(2062, 2120, 751, 2060, 2061, 1.0);
            s.store_add_scaled_product_indices(2123, 2122, 1.0, 769, 2120, 1.0);
            s.store_add_scaled_product_indices(2124, 2122, 1.0, 770, 2120, 1.0);
            s.store_mul(2125, 768, 2123);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2107), 1.0, A::add(s.ad_value(2107), s.ad_value(2106)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2125), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2126, A::offset(s.ad_value(2064), 1.0), s.ad_value(2062), 2056);
            s.store_ln_ad(2127, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(820), s.ad_value(2096)), s.ad_value(773)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2087), s.ad_value(2096)), s.ad_value(773)), 1.0), 1.0));
            s.store_mul(1921, 2120, 2066);
            s.store_div_add_scaled_inputs_rhs_indices(2067, 1921, 218, 1.0, 1921, 1.0);
        }

        s.b[2200] = (s.v[217] < 0.0);
        s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });

        if (s.b[2180] && s.b[2200]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));
        }

        if (s.b[2180] && (!s.b[2200])) {
            s.store_offset_mul(2068, 217, 2067, 1.0);
        }

        if s.b[2180] {
            s.store_mul(2129, 2001, 2068);
            s.store_mul(2128, 2108, 2021);
        }

        s.copy_ad(1843, 2069);

        s.copy_ad(1845, 2087);

        s.copy_ad(1846, 2088);

        s.copy_ad(1847, 2093);

        s.copy_ad(1848, 2096);

        s.copy_ad(1850, 2103);

        s.copy_ad(1849, 2102);

        s.copy_ad(1851, 2109);

        s.copy_ad(1852, 2111);

        s.copy_ad(1853, 2120);

        s.copy_ad(1854, 2121);

        s.copy_ad(1855, 2122);

        s.copy_ad(1856, 2124);

        s.copy_ad(1857, 2126);

        s.copy_ad(1859, 2127);

        s.copy_ad(1858, 2129);

        s.copy_ad(1860, 2128);

        s.store_scalar(1861, 1.0);

        s.store_scalar(1862, 1.0);

        s.store_scalar(1864, 1.0);

        s.store_scalar(1865, 1.0);

        s.store_scalar(832, 0.0);

        s.b[2201] = (s.v[1817] > 0.0);
        s.store_scalar(2201, if s.b[2201] { 1.0 } else { 0.0 });

        if s.b[2201] {
            s.store_ln_ad(1929, A::offset(A::mul(s.ad_value(824), s.ad_value(773)), 1.0));
            s.store_div_scaled_product_indices(1919, 1812, 1852, 1.0, 1854, 1.0);
            s.store_add_scaled_product_mixed_aai(1928, A::mul3(A::mul3(s.ad_value(222), s.ad_value(1855), s.ad_value(1919)), s.ad_value(1919), s.ad_value(1929)), 1.0, A::div_scaled_product(A::add(s.ad_value(220), A::div(s.ad_value(221), s.ad_value(1854))), s.ad_value(1853), 1.0, s.ad_value(1854), 1.0), 1859, 1.0);
            s.store_div_from_scalar_add_ad(1861, 1.0, A::offset(s.ad_value(1928), 1.0), A::square(s.ad_value(1928)));
            s.store_mul(1862, 1857, 1861);
            s.store_div(1863, 1858, 1862);
            s.store_mul_ad_product_lhs_mixed_ai(1930, A::square(s.ad_value(1863)), 1848, 1848);
        }

        s.b[2202] = (s.v[0] == (-1.0));
        s.store_scalar(2202, if s.b[2202] { 1.0 } else { 0.0 });

        if (s.b[2201] && s.b[2202]) {
            s.store_div_scaled_value_offset_denominator(1930, s.ad_value(1930), 1.0, A::mul(s.ad_value(1863), s.ad_value(1848)), 1.0, 1.0);
        }

        if s.b[2201] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1931, 1862, A::sqrt(A::scale_offset(s.ad_value(1930), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(1864, 1.0, 1931);
            s.store_mul(1919, 1862, 1864);
            s.store_mul_offset_ad_rhs(1932, 1852, A::mul3_scaled_output(s.ad_value(1930), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0);
            s.store_div_scaled_product_indices(1865, 1919, 1854, 1.0, 1932, 1.0);
            s.store_mul_product3_indices(832, 1864, 710, 1854, 1848, 1.0);
        }

        s.store_scalar(1934, 0.0);

        s.store_scalar(1935, 0.0);

        s.store_scalar(1866, 0.0);

        s.store_scalar(1867, 0.0);

        s.b[2203] = (((((p.p40 != 0.0) && ((s.v[232] > 0.0) || (s.v[233] > 0.0))) || ((p.p42 != 0.0) && ((s.v[242] > 0.0) || (s.v[243] > 0.0)))) || (s.v[257] > 0.0)) || (s.v[258] > 0.0));
        s.store_scalar(2203, if s.b[2203] { 1.0 } else { 0.0 });

        if s.b[2203] {
            s.store_scaled_add_ad_rhs(1933, 1805, A::sqrt(A::add(A::square(s.ad_value(1805)), s.ad_value(783))), 0.5);
            s.store_add_ad_lhs(1934, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(778), (-0.5), s.ad_value(776), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), 1.0), 785);
            s.store_scaled_add_ad_rhs(1933, 1806, A::sqrt(A::add(A::square(s.ad_value(1806)), s.ad_value(786))), 0.5);
            s.store_add_ad_lhs(1935, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(779), (-0.5), s.ad_value(777), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), 1.0), 788);
            s.store_scaled_add(1866, 1805, 1934, (-s.v[348]));
            s.store_scaled_add(1867, 1806, 1935, (-s.v[348]));
        }

        s.b[2204] = (p.p40 != 0.0);
        s.store_scalar(2204, if s.b[2204] { 1.0 } else { 0.0 });

        s.b[2205] = (s.v[232] > 0.0);
        s.store_scalar(2205, if s.b[2205] { 1.0 } else { 0.0 });

        if (s.b[2204] && s.b[2205]) {
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1866)), 1e-6), 789);
        }

        s.b[2206] = (s.v[238] < 0.0);
        s.store_scalar(2206, if s.b[2206] { 1.0 } else { 0.0 });

        if ((s.b[2204] && s.b[2205]) && s.b[2206]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 795, 0.5, 1936, 795, 1e-6, (-0.5));
        }

        if (s.b[2204] && s.b[2205]) {
            s.store_mul_offset_ad_rhs(1919, 792, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(237), 1.0, s.ad_value(238), s.ad_value(1936), 1.0)), (-1.5));
            s.store_offset(1938, 1934, 3.0);
            s.store_sub_from_scalar(1939, (-3.0), 230);
            s.store_scale(1940, 828, 30.0);
            s.store_scalar(812, (4.0 - 0.9));
            s.store_add(813, 1938, 1940);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1919, 2.0, 812, A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));
            s.store_scalar(812, (4.0 - 0.3));
            s.store_add(813, 1939, 1919);
        }

        s.b[2209] = (s.v[233] > 0.0);
        s.store_scalar(2209, if s.b[2209] { 1.0 } else { 0.0 });

        if (s.b[2204] && s.b[2209]) {
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1867)), 1e-6), 789);
        }

        s.b[2210] = (s.v[240] < 0.0);
        s.store_scalar(2210, if s.b[2210] { 1.0 } else { 0.0 });

        if ((s.b[2204] && s.b[2209]) && s.b[2210]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 796, 0.5, 1936, 796, 1e-6, (-0.5));
        }

        if (s.b[2204] && s.b[2209]) {
            s.store_mul_offset_ad_rhs(1919, 793, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(239), 1.0, s.ad_value(240), s.ad_value(1936), 1.0)), (-1.5));
            s.store_offset(1938, 1935, 3.0);
            s.store_sub_from_scalar(1939, (-3.0), 230);
            s.store_scale(1940, 831, 30.0);
            s.store_scalar(812, (4.0 - 0.9));
            s.store_add(813, 1938, 1940);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1919, 2.0, 812, A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));
            s.store_scalar(812, (4.0 - 0.3));
            s.store_add(813, 1939, 1919);
        }

        s.b[2213] = (s.v[231] > 0.0);
        s.store_scalar(2213, if s.b[2213] { 1.0 } else { 0.0 });

        s.b[2214] = (s.v[1817] <= 0.0);
        s.store_scalar(2214, if s.b[2214] { 1.0 } else { 0.0 });

        if ((s.b[2204] && s.b[2213]) && s.b[2214]) {
            s.store_offset(1919, 771, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 1843, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1846, 1843, 1813, 1919, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0);
        }

        s.b[2215] = ((s.v[1847] - s.v[1846]) > (-230.25850929940458));
        s.store_scalar(2215, if s.b[2215] { 1.0 } else { 0.0 });

        if ((s.b[2204] && s.b[2213]) && s.b[2215]) {
            s.store_exp_sub(1919, 1847, 1846);
        }

        if ((s.b[2204] && s.b[2213]) && (!s.b[2215])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1919, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_add_scaled_product_right_ad(1942, 1922, 1.0, 1812, A::sub_scaled_inputs(s.ad_value(1847), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1919), 1.0), 0.5), 1.0), 1.0);
            s.store_mul(1943, 230, 1812);
            s.store_add(1944, 1860, 1943);
            s.store_scaled_sub_ad_rhs(1945, 1944, A::sqrt_square_offset(A::neg(s.ad_value(1944)), 0.01), 0.5);
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1860)), 1e-6), 789);
        }

        s.b[2216] = (s.v[236] < 0.0);
        s.store_scalar(2216, if s.b[2216] { 1.0 } else { 0.0 });

        if ((s.b[2204] && s.b[2213]) && s.b[2216]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 794, 0.5, 1936, 794, 1e-6, (-0.5));
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_add_scaled_product_left_ad(1946, 1850, 1.0, A::add_scaled_inputs3(s.ad_value(1945), 1.0, s.ad_value(736), (-1.0), s.ad_value(1942), -1.0), 1813, 1.0);
            s.store_mul_neg_ad_lhs(1946, A::add_scaled_inputs3(s.ad_value(819), 1.0, s.ad_value(1922), 1.0, s.ad_value(1942), -1.0), 1813);
        }

        s.b[2219] = (((s.v[1946]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2219, if s.b[2219] { 1.0 } else { 0.0 });

        if ((s.b[2204] && s.b[2213]) && s.b[2219]) {
            s.store_exp(1919, 1946);
        }

        s.b[2220] = (s.v[1946] < 0.0);
        s.store_scalar(2220, if s.b[2220] { 1.0 } else { 0.0 });

        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && s.b[2220]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1919, 1e-100, (-230.25850929940458), 1946, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && (!s.b[2220])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1919, 1946, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2204] && s.b[2213]) {
            s.store_mul_offset_ad_rhs(1919, 791, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(235), 1.0, s.ad_value(236), s.ad_value(1936), 1.0)), (-1.5));
        }

        s.b[2223] = ((s.v[1817] <= 0.0) || ((s.v[235] == 0.0) && (s.v[236] == 0.0)));
        s.store_scalar(2223, if s.b[2223] { 1.0 } else { 0.0 });

        if ((s.b[2204] && s.b[2213]) && (!s.b[2223])) {
            s.store_add_scaled_product_indices(1919, 235, 1.0, 236, 1936, 2.0);
            s.store_div_ad_rhs(1950, 241, A::mul(s.ad_value(1919), s.ad_value(791)));
            s.store_scaled_div(1951, 1848, 1950, 0.5);
        }

        s.b[2224] = (s.v[1951] < 0.001);
        s.store_scalar(2224, if s.b[2224] { 1.0 } else { 0.0 });

        s.b[2225] = (((s.v[1951]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2225, if s.b[2225] { 1.0 } else { 0.0 });

        if ((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && s.b[2225]) {
            s.store_exp(1959, 1951);
        }

        s.b[2226] = (s.v[1951] < 0.0);
        s.store_scalar(2226, if s.b[2226] { 1.0 } else { 0.0 });

        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && s.b[2226]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1959, 1e-100, (-230.25850929940458), 1951, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && (!s.b[2226])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1959, 1951, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) {
            s.store_div_from_scalar(1960, 1.0, 1959);
            s.store_sub(1919, 1959, 1960);
            s.store_add(1921, 1959, 1960);
        }

        s.b[2227] = (p.p42 != 0.0);
        s.store_scalar(2227, if s.b[2227] { 1.0 } else { 0.0 });

        s.b[2228] = ((s.v[243] > 0.0) && (s.v[1867] < 0.0));
        s.store_scalar(2228, if s.b[2228] { 1.0 } else { 0.0 });

        if (s.b[2227] && s.b[2228]) {
            s.store_sqrt_offset_ad(1963, A::add_scaled_square_product(s.ad_value(1867), 1.0, A::square(s.ad_value(249)), A::square(s.ad_value(830)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(1919, 801, -1.0, 1963, 1.0);
        }

        s.b[2229] = (s.v[1919] > (-230.25850929940458));
        s.store_scalar(2229, if s.b[2229] { 1.0 } else { 0.0 });

        if ((s.b[2227] && s.b[2228]) && s.b[2229]) {
            s.store_exp(1921, 1919);
        }

        if ((s.b[2227] && s.b[2228]) && (!s.b[2229])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1921, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2230] = ((s.v[242] > 0.0) && (s.v[1866] < 0.0));
        s.store_scalar(2230, if s.b[2230] { 1.0 } else { 0.0 });

        if (s.b[2227] && s.b[2230]) {
            s.store_sqrt_offset_ad(1964, A::add_scaled_square_product(s.ad_value(1866), 1.0, A::square(s.ad_value(248)), A::square(s.ad_value(829)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(1919, 800, -1.0, 1964, 1.0);
        }

        s.b[2231] = (s.v[1919] > (-230.25850929940458));
        s.store_scalar(2231, if s.b[2231] { 1.0 } else { 0.0 });

        if ((s.b[2227] && s.b[2230]) && s.b[2231]) {
            s.store_exp(1921, 1919);
        }

        if ((s.b[2227] && s.b[2230]) && (!s.b[2231])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1921, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_scalar(1968, s.v[709]);

        s.store_scalar(1868, 0.0);

        s.store_scalar(1869, 0.0);

        s.store_scalar(1870, 0.0);

        s.store_scalar(1871, 1e-40);

        s.store_scalar(1872, 1.0);

        s.store_scalar(840, 0.0);

        s.b[2232] = ((p.p46 != 0.0) && (s.v[282] > 0.0));
        s.store_scalar(2232, if s.b[2232] { 1.0 } else { 0.0 });

        if s.b[2232] {
            s.store_add_scaled_inputs4_mixed_iiai(1919, 822, 0.5, 821, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(758))), (-0.5), 756, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(1965, 821, 1.0, 1919, (-0.5), A::sqrt(A::add(A::square(s.ad_value(1919)), s.ad_value(757))), (-(-0.5)), 760, 1.0);
            s.store_add_scaled_inputs3_indices(1966, 1965, 1.0, 820, 0.5, 824, (-0.5));
            s.store_mul_ad_product_rhs(1967, 284, A::offset(A::mul(s.ad_value(286), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(285), s.ad_value(1966)), 1.0));
            s.store_mul_offset_rhs(1968, 717, 1967, 1.0);
            s.store_div_from_scalar(1969, 1.0, 1968);
            s.store_div_scaled_value_offset_denominator(1970, s.ad_value(824), 2.0, A::sqrt_product_offset(s.ad_value(288), s.ad_value(824), 1.0), 1.0, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(1971, 287, 1970, A::offset(A::mul(s.ad_value(289), s.ad_value(1966)), 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(1868, 1969, s.ad_value(823), 1.0, s.ad_value(1971), 1.0, s.ad_value(707), -1.0, 0.0);
            s.store_mul(1972, 1969, 754);
            s.store_scaled_ln_ad(1973, A::add(A::div(s.ad_value(1972), s.ad_value(755)), A::sqrt(s.ad_value(1972))), 2.0);
            s.store_mul(1974, 1969, 1965);
            s.store_add(1979, 1972, 1974);
            s.store_add_scaled_product_right_ad(1980, 1979, 1.0, 755, A::sqrt(s.ad_value(1979)), 1.0);
            s.store_add(1981, 1980, 1973);
            s.store_offset_div_scaled_inputs_mixed_ia(1982, 755, 1.0, A::sqrt(s.ad_value(1979)), 2.0, 1.0);
            s.store_div_from_scalar(1983, 1.0, 1982);
            s.store_sub(1984, 1868, 1981);
        }

        s.b[2233] = (s.v[1984] > (-12.0));
        s.store_scalar(2233, if s.b[2233] { 1.0 } else { 0.0 });

        if (s.b[2232] && s.b[2233]) {
            s.store_offset_add(1985, 1984, 719, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1986, 1985, 1985, 10.0, 0.5);
            s.store_add_ad_lhs(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);
            s.store_scaled_add_sqrt_square_offset_rhs(1988, 1987, 1987, 2.0, 0.5);
        }

        s.b[2234] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);
        s.store_scalar(2234, if s.b[2234] { 1.0 } else { 0.0 });

        if ((s.b[2232] && s.b[2233]) && s.b[2234]) {
            s.store_exp_sub(1989, 1984, 1988);
        }

        if ((s.b[2232] && s.b[2233]) && (!s.b[2234])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1989, A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2232] && s.b[2233]) {
            s.store_mul(1990, 718, 1989);
            s.store_pow_indices(1991, 1990, 1983);
            s.store_add_scaled_square_product_mixed_iai(1992, 1982, 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), 1991, 1.0);
            s.store_mul_offset_ad_rhs(1993, 1982, A::div_scaled_inputs2(A::sqrt(s.ad_value(1992)), 1.0, s.ad_value(1982), (-1.0), s.ad_value(1991), 1.0), (-1.0));
            s.store_sub(1975, 1988, 1993);
        }

        s.b[2235] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));
        s.store_scalar(2235, if s.b[2235] { 1.0 } else { 0.0 });

        if ((s.b[2232] && (!s.b[2233])) && s.b[2235]) {
            s.store_exp_ad(1975, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

        if ((s.b[2232] && (!s.b[2233])) && (!s.b[2235])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1975, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if s.b[2232] {
            s.store_mul_add_rhs(1976, 1969, 1845, 1965);
        }

        s.b[2236] = ((s.v[1975] < 0.001) && (s.v[1845] < 1e-6));
        s.store_scalar(2236, if s.b[2236] { 1.0 } else { 0.0 });

        s.b[2237] = (((-s.v[1976]) + s.v[1974]) > (-230.25850929940458));
        s.store_scalar(2237, if s.b[2237] { 1.0 } else { 0.0 });

        if ((s.b[2232] && s.b[2236]) && s.b[2237]) {
            s.store_exp_sub(1919, 1974, 1976);
        }

        if ((s.b[2232] && s.b[2236]) && (!s.b[2237])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1919, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2232] && s.b[2236]) {
            s.store_mul_offset_rhs(1869, 1975, 1919, (-1.0));
            s.store_add(1977, 1869, 1975);
        }

        if (s.b[2232] && (!s.b[2236])) {
            s.store_add(1979, 1972, 1976);
            s.store_add_scaled_product_right_ad(1980, 1979, 1.0, 755, A::sqrt(s.ad_value(1979)), 1.0);
            s.store_add(1981, 1980, 1973);
            s.store_offset_div_scaled_inputs_mixed_ia(1982, 755, 1.0, A::sqrt(s.ad_value(1979)), 2.0, 1.0);
            s.store_div_from_scalar(1983, 1.0, 1982);
            s.store_sub(1984, 1868, 1981);
        }

        s.b[2238] = (s.v[1984] > (-12.0));
        s.store_scalar(2238, if s.b[2238] { 1.0 } else { 0.0 });

        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {
            s.store_offset_add(1985, 1984, 719, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1986, 1985, 1985, 10.0, 0.5);
            s.store_add_ad_lhs(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);
            s.store_scaled_add_sqrt_square_offset_rhs(1988, 1987, 1987, 2.0, 0.5);
        }

        s.b[2239] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);
        s.store_scalar(2239, if s.b[2239] { 1.0 } else { 0.0 });

        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && s.b[2239]) {
            s.store_exp_sub(1989, 1984, 1988);
        }

        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && (!s.b[2239])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1989, A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {
            s.store_mul(1990, 718, 1989);
            s.store_pow_indices(1991, 1990, 1983);
            s.store_add_scaled_square_product_mixed_iai(1992, 1982, 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), 1991, 1.0);
            s.store_mul_offset_ad_rhs(1993, 1982, A::div_scaled_inputs2(A::sqrt(s.ad_value(1992)), 1.0, s.ad_value(1982), (-1.0), s.ad_value(1991), 1.0), (-1.0));
            s.store_sub(1977, 1988, 1993);
        }

        s.b[2240] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));
        s.store_scalar(2240, if s.b[2240] { 1.0 } else { 0.0 });

        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && s.b[2240]) {
            s.store_exp_ad(1977, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && (!s.b[2240])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1977, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2232] && (!s.b[2236])) {
            s.store_sub(1869, 1977, 1975);
        }

        if s.b[2232] {
            s.store_scaled_add(1870, 1977, 1975, 0.5);
        }

        if s.b[2232] {
            if ((s.v[1868] - s.v[1870]) > 1e-40) {
                s.store_sub(1871, 1868, 1870);
            } else {
                s.store_scalar(1871, 1e-40);
            }
        }

        if s.b[2232] {
            s.store_sub_from_scalar_ad(1872, 1.0, A::div_scaled_inputs(s.ad_value(755), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1871), 1.0, s.ad_value(718), 0.25)), 1.0));
            s.store_div_scaled_product3_mixed_aaii(840, A::mul3_scaled_output(s.ad_value(711), s.ad_value(1968), s.ad_value(1968), -1.0), A::offset(A::mul(s.ad_value(1872), s.ad_value(1870)), 1.0), 1869, 1.0, 1857, 1.0);
        }

        s.store_scalar(1873, 0.0);

        s.store_scalar(841, 0.0);

        s.b[2241] = ((s.v[1817] > 0.0) && (p.p41 != 0.0));
        s.store_scalar(2241, if s.b[2241] { 1.0 } else { 0.0 });

        if s.b[2241] {
            s.store_add_scaled_product_indices(1978, 820, 1.0, 227, 1848, (-1.0));
        }

        s.b[2242] = (s.v[1978] > 0.0);
        s.store_scalar(2242, if s.b[2242] { 1.0 } else { 0.0 });

        if (s.b[2241] && s.b[2242]) {
            s.store_mul_div_scaled_offset_numerator_rhs(1921, 706, A::mul(s.ad_value(228), A::sub(A::sqrt(A::add(s.ad_value(722), s.ad_value(1922))), s.ad_value(730))), 1.0, 1.0, A::offset(s.ad_value(1978), 1e-30), 1.0);
        }

        s.b[2243] = ((((-s.v[1921])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2243, if s.b[2243] { 1.0 } else { 0.0 });

        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {
            s.store_exp_neg_input(1919, 1921);
        }

        s.b[2244] = ((-s.v[1921]) < 0.0);
        s.store_scalar(2244, if s.b[2244] { 1.0 } else { 0.0 });

        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && s.b[2244]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1919, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1921)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && (!s.b[2244])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1919, A::neg(s.ad_value(1921)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2241] && s.b[2242]) {
            s.store_mul3_lhs(1873, 224, 1978, 1919);
            s.store_mul_add_rhs(841, 1873, 832, 840);
        }

        s.b[2245] = (s.v[841] > (0.5 * s.v[229]));
        s.store_scalar(2245, if s.b[2245] { 1.0 } else { 0.0 });

        if ((s.b[2241] && s.b[2242]) && s.b[2245]) {
            s.store_offset_div_scaled_inputs_indices(1919, 841, 2.0, 229, 1.0, (-1.0));
            s.store_mul_scaled_offset_ad_rhs(841, 229, 0.5, A::div(s.ad_value(1919), A::sqrt_square_offset(s.ad_value(1919), 1.0)), 1.0);
        }

        s.b[2439] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.store_scalar(2439, if s.b[2439] { 1.0 } else { 0.0 });

        s.b[2440] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.store_scalar(2440, if s.b[2440] { 1.0 } else { 0.0 });

        if (s.b[2439] && s.b[2440]) {
            s.copy_ad(2280, 722);
            s.copy_ad(2281, 732);
            s.copy_ad(2282, 723);
            s.copy_ad(2283, 1808);
            s.copy_ad(2284, 1809);
            s.store_scalar(2288, 0.0);
        }

        s.b[2441] = (p.p47 > 0.0);
        s.store_scalar(2441, if s.b[2441] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2440]) && s.b[2441]) {
            s.store_add_scaled_inputs4_mixed_iiai(2283, 822, 0.5, 821, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(743))), (-0.5), 741, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(1874, 821, 1.0, 2283, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2283)), s.ad_value(742))), (-(-0.5)), 744, 1.0);
            s.copy_ad(2284, 1874);
            s.copy_ad(2280, 739);
            s.copy_ad(2281, 742);
            s.copy_ad(2282, 740);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_add_scaled_inputs3_indices(2287, 823, 1.0, 2288, (-1.0), 694, -1.0);
            s.store_add_scaled_inputs3_indices(2289, 2284, 1.0, 820, 0.5, 824, (-0.5));
            s.store_scalar(2301, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2442] = (s.v[185] > 0.0);
        s.store_scalar(2442, if s.b[2442] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2440]) && s.b[2442]) {
            s.store_scale(2292, 2280, s.v[355]);
            s.store_scale(2293, 2289, s.v[355]);
            s.store_scale(2294, 2287, s.v[355]);
            s.store_offset_div_scaled_inputs_mixed_ia(1920, 2282, 0.5, A::sqrt(s.ad_value(2292)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(1921, 2292, 1.0, 2282, A::sqrt(s.ad_value(2292)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2295, A::div_scaled_inputs2(s.ad_value(2294), 1.0, s.ad_value(1921), (-1.0), s.ad_value(1920), 1.0), 1.0, 2292, 0.5, A::offset(s.ad_value(186), 1.0), 2293, (-1.0));
            s.store_offset_scaled(2296, 2292, 0.5, 2.0);
            s.store_add(2297, 2292, 2293);
            s.store_sub_scaled_inputs_ad(1920, A::add_scaled_inputs_product(s.ad_value(2294), 1.0, s.ad_value(2297), (-1.0), s.ad_value(2282), A::sqrt(s.ad_value(2297)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2292), s.ad_value(2282)), A::sqrt(s.ad_value(2292)))), 2.0);
            s.store_add_scaled_inputs(2298, 1920, 2.0, 2296, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2295, 0.5, 2298, 0.5, 2295, 2298, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(1921, 2294, 2.0, 2293, (-2.0), 2296, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2299, 1920, 0.5, 1921, 0.5, 1920, 1921, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2299, 0.5, 2296, 0.5, 2299, 2296, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2300, 1920, 0.5, 2296, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2296), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1921, 696, A::div(s.ad_value(2300), s.ad_value(2296)), 1.0);
        }

        s.b[2443] = (s.v[1921] > (-230.25850929940458));
        s.store_scalar(2443, if s.b[2443] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && s.b[2443]) {
            s.store_exp(2301, 1921);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && (!s.b[2443])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2301, 1e-100, (-230.25850929940458), 1921, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_offset_mul(2302, 695, 2301, 1.0);
            s.store_scale(2303, 2302, s.v[709]);
            s.store_mul_ad_product_rhs(2304, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2289)), 1.0));
            s.store_mul_offset_rhs(2305, 2303, 2304, 1.0);
            s.store_div_from_scalar(2306, 1.0, 2305);
            s.store_mul_ad_rhs(2290, 2282, A::sqrt_scaled_input(s.ad_value(2306), s.v[709]));
            s.store_square(2291, 2290);
            s.store_div_from_scalar(2307, 1.0, 2291);
            s.store_mul(2308, 2284, 2306);
            s.store_mul(2309, 2287, 2306);
            s.store_div_scaled_value_offset_denominator(2310, s.ad_value(824), 2.0, A::sqrt_product_offset(s.ad_value(192), s.ad_value(824), 1.0), 1.0, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(2311, 191, 2310, A::offset(A::mul(s.ad_value(193), s.ad_value(2289)), 1.0));
            s.store_mul(2312, 2280, 2306);
            s.store_sqrt_square_add(1920, 2283, 2281);
            s.store_sqrt_add_ad(1921, A::square(A::sub(s.ad_value(2283), s.ad_value(2311))), s.ad_value(2281));
            s.store_mul_add_scaled_inputs3_offset_rhs(2313, 2306, s.ad_value(2311), 0.5, s.ad_value(1920), 0.5, s.ad_value(1921), ((-1.0) * (0.5)), 0.0);
            s.store_add(2314, 2312, 2308);
            s.store_sub(2315, 2314, 2313);
        }

        s.b[2444] = (p.p45 > 0.0);
        s.store_scalar(2444, if s.b[2444] { 1.0 } else { 0.0 });

        s.b[2445] = (((s.v[2315]) as f64).abs() < 1e-5);
        s.store_scalar(2445, if s.b[2445] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && s.b[2445]) {
            s.store_offset_ad(2316, A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2315), 1.0, A::scale(s.ad_value(2315), 0.3125), 0.5)), 1.0);
        }

        s.b[2446] = (s.v[2315] < 460.51701859880916);
        s.store_scalar(2446, if s.b[2446] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && s.b[2446]) {
            s.store_exp_neg_input(2330, 2315);
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && (!s.b[2446])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2330, 1e-200, 2315, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {
            s.store_scalar(1919, (if (s.v[2315] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {
            s.store_offset_ad(2316, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2290), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2315))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2315), 1.0, s.ad_value(2330))), 2.0), 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && (!s.b[2444])) {
            s.store_offset_div_scaled_inputs_mixed_ia(2316, 2290, 0.5, A::sqrt(s.ad_value(2315)), 1.0, 1.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_add_scaled_value_products(2317, s.ad_value(2315), 1.0, s.ad_value(2290), A::sqrt(s.ad_value(2315)), 1.0, s.ad_value(2316), A::ln(A::offset(s.ad_value(2316), (-1.0))), (-1.0));
            s.store_div_scaled_inputs2_indices(2318, 2309, 1.0, 2317, (-1.0), 2316, 1.0);
            s.store_mul_scaled_offset_ad_rhs(2324, 2291, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2291)), 1.0)), (-1.0));
            s.store_scalar(2323, 0.0);
            s.store_scalar(2325, 1.0);
        }

        s.b[2447] = (s.v[2318] > (-30.0));
        s.store_scalar(2447, if s.b[2447] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_offset_mul(2319, 2316, 2318, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1919, 2319, 2319, 10.0, 0.5);
            s.store_sub_ad_rhs(2320, 2318, A::ln(s.ad_value(1919)));
            s.store_scaled_add_sqrt_square_offset_rhs(2321, 2320, 2320, 2.0, 0.5);
        }

        s.b[2448] = ((s.v[2318] - s.v[2321]) < 230.25850929940458);
        s.store_scalar(2448, if s.b[2448] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2448]) {
            s.store_exp_sub(1919, 2318, 2321);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2448])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1919, A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_div(2322, 1919, 2316);
            s.store_sub_ad_lhs(1919, A::scaled_offset(s.ad_value(2321), 1.0, 2.0), 2322);
        }

        s.b[2449] = (s.v[2322] > 1e-6);
        s.store_scalar(2449, if s.b[2449] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2449]) {
            s.store_mul_offset_ad_rhs(2323, 2316, A::sub(s.ad_value(2321), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2322), s.ad_value(1919), 1.0), 1.0, (-1.0), s.ad_value(2322), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2449])) {
            s.store_mul_ad_affine_product_rhs(2323, 2316, s.ad_value(2322), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_add_scaled_inputs3_offset_mixed_iia(1919, 2309, 0.5, 2323, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2324, 2291, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2291)), s.ad_value(1919), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2325, 2324, 2324, 1.0, 2323, 1.0);
            s.store_add_scaled_product_indices(2315, 2314, 1.0, 2325, 2313, (-1.0));
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_offset_scaled(2326, 2290, 0.7071067811865475, 1.0);
            s.store_scale(2327, 2326, 1e-5);
            s.store_div_from_scalar(2328, 1.0, 2326);
            s.store_scalar(2435, 0.0);
            s.store_scalar(2329, 0.0);
        }

        s.b[2450] = (s.v[2315] < 460.51701859880916);
        s.store_scalar(2450, if s.b[2450] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2440]) && s.b[2450]) {
            s.store_exp_neg_input(2330, 2315);
        }

        if ((s.b[2439] && s.b[2440]) && (!s.b[2450])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2330, 1e-200, 2315, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2451] = (((s.v[2309]) as f64).abs() <= s.v[2327]);
        s.store_scalar(2451, if s.b[2451] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2440]) && s.b[2451]) {
            s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2329, 2309, 2328, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2330)), s.ad_value(2290), s.ad_value(2415)), 1.0));
        }

        s.b[2452] = (s.v[2309] < (-s.v[2327]));
        s.store_scalar(2452, if s.b[2452] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {
            s.store_neg(2417, 2309);
            s.store_scaled_mul(2418, 2417, 2328, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2419, 2418, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2414, 2417, 2419);
            s.store_add_scaled_square_product_mixed_iia(2420, 2414, 1.0, 2291, A::offset(s.ad_value(2419), 1.0), 1.0);
            s.store_sub_scaled_inputs(2421, 2414, 2.0, 2291, 1.0);
            s.store_sub_ln_mul_lhs(2422, 2420, 2307, 2419);
            s.store_add(818, 2420, 2421);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.5, s.ad_value(2420), 1.0), 1.0);
            s.store_add_ad_rhs(2423, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.3333333333333333, s.ad_value(2420), 1.0))), 1.0));
        }

        s.b[2453] = (s.v[2423] < 230.25850929940458);
        s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && s.b[2453]) {
            s.store_exp(2424, 2423);
        }

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && (!s.b[2453])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2424, 2423, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_div_from_scalar_offset_square(2414, 1.0, 2423, 2.0);
            s.store_mul_square_lhs(2426, 2423, 2414);
            s.store_mul3_affine_lhs(2427, 2423, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), 2414, 2414);
            s.store_sub(2414, 2417, 2423);
            s.store_mul(2415, 2330, 2425);
            s.store_add_scaled_product_right_ad(2429, 2414, 2.0, 2291, A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2415), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2427)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2423), (-1.0), s.ad_value(2415), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::sub(A::offset(s.ad_value(2423), (-1.0)), s.ad_value(2426)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2414, 2.0, 2291, A::add_scaled_inputs_product(s.ad_value(2424), 1.0, s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2329, 2423, -1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_div_from_scalar_offset_scaled_input(2431, 1.0, 2290, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2432, A::mul_scaled_lhs(s.ad_value(2326), 1.25, s.ad_value(2431)), (-1.0), 2431);
            s.store_mul_ad_product_rhs_mixed_ia(2433, 2309, 2328, A::offset(A::mul(s.ad_value(2432), s.ad_value(2309)), 1.0));
        }

        s.b[2454] = ((-s.v[2433]) > (-230.25850929940458));
        s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2454]) {
            s.store_exp_neg_input(2414, 2433);
        }

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2454])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2414, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2433)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_sub_from_scalar(2434, 1.0, 2414);
            s.store_add_scaled_inputs_product_right_ad(2435, 2309, 1.0, 2291, 0.5, 2290, A::sqrt(A::add_scaled_inputs3(s.ad_value(2309), 1.0, s.ad_value(2291), 0.25, s.ad_value(2434), -1.0)), (-1.0));
            s.store_offset(2436, 2315, 3.0);
            s.store_sub_ad(2419, A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2435), s.ad_value(2436)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2436), 0.5, A::sqrt_square_offset(s.ad_value(2436), 5.0), 0.5));
            s.store_sub(2414, 2309, 2419);
            s.store_exp_neg_input(2415, 2419);
            s.store_div_from_scalar_offset_square(2416, 1.0, 2419, 2.0);
            s.store_mul_square_lhs(2426, 2419, 2416);
            s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);
            s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), 2416, 2416);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2330] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                s.store_scalar(2420, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2420, 2414, 1.0, 2291, A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            }
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2437, 1.0, 2291, A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2421, 2414, 2.0, 2291, A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2422, 2315, 1.0, 2419, (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0);
            s.store_add(818, 2420, 2421);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0);
            s.store_add_ad_rhs(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));
        }

        s.b[2455] = (s.v[2438] < 230.25850929940458);
        s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2455]) {
            s.store_exp(2424, 2438);
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_mul(2424, 2330, 2424);
        }

        s.b[2456] = (s.v[2438] > (s.v[2315] - 230.25850929940458));
        s.store_scalar(2456, if s.b[2456] { 1.0 } else { 0.0 });

        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && s.b[2456]) {
            s.store_exp_sub(2424, 2438, 2315);
            s.store_div(2425, 2330, 2424);
        }

        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && (!s.b[2456])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2424, 1e-100, A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2425, 1e-100, 2438, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_div_from_scalar_offset_square(2414, 1.0, 2438, 2.0);
            s.store_mul_square_lhs(2426, 2438, 2414);
            s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), 2414, 2414);
            s.store_sub(2414, 2309, 2438);
            s.store_add_scaled_product_right_ad(2429, 2414, 2.0, 2291, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2414, 2.0, 2291, A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2329, 2438, 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_scalar(2332, 0.0);
            s.store_scalar(2333, 0.0);
            s.store_scalar(2334, 0.0);
            s.store_scalar(2335, 0.0);
            s.store_scalar(2336, 0.0);
            s.store_scalar(2337, 0.0);
            s.store_scalar(2338, 0.0);
            s.store_scalar(2339, 1.0);
            s.store_scalar(2340, 1.0);
            s.store_sub(2341, 2309, 2329);
            s.store_scalar(2342, 0.0);
            s.store_mul(2343, 2305, 2341);
            s.store_scalar(2344, 1.0);
            s.store_scalar(2345, 1.0);
            s.store_scalar(2349, 1.0);
            s.store_scalar(2350, 1.0);
            s.store_scalar(2352, 1.0);
        }

        s.b[2457] = (s.v[2309] > 0.0);
        s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_div_from_scalar_offset_square(1919, 1.0, 2329, 2.0);
            s.store_mul_square_lhs(2331, 2329, 1919);
            s.store_mul3_affine_lhs(2332, 2329, 1919, 4.0, 0.0, 1919);
            s.store_mul_ad_product_lhs_mixed_ai(2333, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2331), 12.0), 1919, 1919);
            s.store_scalar(2334, 0.0);
        }

        s.b[2458] = (s.v[2329] < 230.25850929940458);
        s.store_scalar(2458, if s.b[2458] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2458]) {
            s.store_exp(2334, 2329);
            s.store_div_from_scalar(2335, 1.0, 2334);
            s.store_mul(2334, 2330, 2334);
        }

        s.b[2459] = (s.v[2329] > (s.v[2315] - 230.25850929940458));
        s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && s.b[2459]) {
            s.store_exp_sub(2334, 2329, 2315);
            s.store_div(2335, 2330, 2334);
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2459])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2334, 1e-100, A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2335, 1e-100, 2329, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_add_scaled_product_right_ad(2336, 2334, 1.0, 2330, A::add(A::offset(s.ad_value(2329), 1.0), s.ad_value(2331)), (-1.0));
        }

        s.b[2460] = (s.v[2329] < 1e-5);
        s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2460]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2337, 2329, 1.0, 2329, 1.0, 2329, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2336, A::mul3(s.ad_value(2330), s.ad_value(2329), s.ad_value(2329)), 2329, A::scale_offset(s.ad_value(2329), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2338, 2329, 1919, 0.7071067811865475);
            s.store_offset_div_scaled_product(2339, s.ad_value(2290), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.5)), 1.0, A::square(s.ad_value(2329)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2460])) {
            s.store_add_offset_lhs(2337, 2329, (-1.0), 2335);
            s.store_sqrt(2338, 2337);
            s.store_offset_scaled_ad(2339, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2335)), s.ad_value(2338)), 0.5, 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_div_scaled_offset_numerator(2340, A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2289)), 1.0, 1.0, A::offset(A::mul(s.ad_value(702), s.ad_value(2289)), 1.0), 1.0);
        }

        s.b[2461] = (s.v[2336] > 1e-100);
        s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul_sqrt_ad_rhs(2341, 2290, A::add(s.ad_value(2337), s.ad_value(2336)));
            s.store_div_scaled_product3_mixed_iiia(2342, 2291, 2336, 2305, 1.0, A::add_scaled_product(s.ad_value(2341), 1.0, s.ad_value(2290), s.ad_value(2338), 1.0), 1.0);
            s.store_mul3_lhs(2343, 2338, 2290, 2305);
        }

        s.b[2462] = (s.v[212] < 0.0);
        s.store_scalar(2462, if s.b[2462] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2462]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2344, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2289)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2462])) {
            s.store_offset_mul(2344, 212, 2289, 1.0);
        }

        s.b[2463] = (s.v[213] < 0.0);
        s.store_scalar(2463, if s.b[2463] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2463]) {
            s.store_sub_from_scalar_scaled_mul(2345, 1.0, 213, 2342, 1.0);
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2463])) {
            s.store_div_from_scalar_offset_product(2345, 1.0, 213, 2342, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul_product3_indices(2346, 2342, 751, 2344, 2345, 1.0);
            s.store_mul_add_scaled_product_rhs(2347, 768, s.ad_value(2343), 1.0, s.ad_value(769), s.ad_value(2342), 1.0);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2337), 1.0, A::add(s.ad_value(2337), s.ad_value(2336)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2348, A::pow(A::mul(s.ad_value(2347), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2349, A::offset(s.ad_value(2348), 1.0), s.ad_value(2346), 2340);
        }

        s.b[2464] = (s.v[216] < 0.0);
        s.store_scalar(2464, if s.b[2464] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2464]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2350, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2289)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2464])) {
            s.store_offset_mul(2350, 216, 2289, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul(1921, 2342, 2350);
            s.store_div_add_scaled_inputs_rhs_indices(2351, 1921, 218, 1.0, 1921, 1.0);
        }

        s.b[2465] = (s.v[217] < 0.0);
        s.store_scalar(2465, if s.b[2465] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2465]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2465])) {
            s.store_offset_mul(2352, 217, 2351, 1.0);
        }

        if (s.b[2439] && (!s.b[2440])) {
            s.copy_ad(2287, 1810);
            s.copy_ad(2289, 1811);
            s.copy_ad(2305, 1812);
            s.copy_ad(2306, 1813);
            s.copy_ad(2290, 1814);
            s.copy_ad(2291, 1815);
            s.copy_ad(2307, 1816);
            s.copy_ad(2309, 1817);
            s.copy_ad(2314, 1818);
            s.copy_ad(2315, 1819);
            s.copy_ad(2326, 1820);
            s.copy_ad(2327, 1821);
            s.copy_ad(2328, 1822);
            s.copy_ad(2435, 1823);
            s.copy_ad(2330, 1824);
            s.copy_ad(2329, 1825);
            s.copy_ad(2332, 1826);
            s.copy_ad(2333, 1827);
            s.copy_ad(2334, 1828);
            s.copy_ad(2335, 1829);
            s.copy_ad(2337, 1830);
            s.copy_ad(2336, 1831);
            s.copy_ad(2338, 1832);
            s.copy_ad(2339, 1833);
            s.copy_ad(2340, 1834);
            s.copy_ad(2341, 1835);
            s.copy_ad(2342, 1836);
            s.copy_ad(2343, 1837);
            s.copy_ad(2344, 1838);
            s.copy_ad(2345, 1839);
            s.copy_ad(2349, 1840);
            s.copy_ad(2350, 1841);
            s.copy_ad(2352, 1842);
        }

        if s.b[2439] {
            s.copy_ad(2285, 714);
            s.copy_ad(2286, 771);
        }

        s.b[2466] = (p.p48 != 0.0);
        s.store_scalar(2466, if s.b[2466] { 1.0 } else { 0.0 });

        if (s.b[2439] && s.b[2466]) {
            s.copy_ad(2285, 715);
            s.copy_ad(2286, 772);
        }

        if s.b[2439] {
            s.store_scalar(2354, 0.0);
            s.store_scale(2353, 2305, 4.60517018598809);
            s.copy_ad(2370, 2353);
            s.copy_ad(2371, 820);
            s.store_mul(2372, 820, 2306);
            s.copy_ad(2376, 2329);
            s.store_scalar(2377, 0.0);
            s.store_scalar(2380, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2439] {
            s.copy_ad(2382, 2335);
            s.copy_ad(2383, 2337);
            s.copy_ad(2385, 2336);
            s.copy_ad(2386, 2343);
            s.copy_ad(2387, 2329);
            s.copy_ad(2388, 2335);
            s.copy_ad(2390, 2336);
            s.copy_ad(2391, 2337);
            s.store_sub(2392, 2309, 2329);
            s.store_scalar(2393, 1.0);
            s.store_scalar(2395, 1.0);
            s.store_scalar(2394, 0.0);
            s.copy_ad(2404, 2342);
            s.store_mul(2408, 2392, 2305);
            s.store_scalar(2405, 0.0);
            s.copy_ad(2406, 2343);
            s.store_scalar(2411, 0.0);
            s.store_scalar(2410, 1.0);
            s.copy_ad(2413, 2285);
            s.copy_ad(2412, 2408);
        }

        s.b[2467] = (s.v[2309] > 0.0);
        s.store_scalar(2467, if s.b[2467] { 1.0 } else { 0.0 });

        s.b[2468] = (s.v[2336] > 1e-100);
        s.store_scalar(2468, if s.b[2468] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul(2413, 2285, 2352);
            s.store_div(2354, 2413, 2349);
            s.store_add_scaled_inputs(2355, 2341, 1.0, 2291, 0.5);
            s.store_div_scaled_product_by_product(1919, s.ad_value(2291), s.ad_value(2334), 1.0, s.ad_value(2355), s.ad_value(2355), 1.0);
        }

        s.b[2469] = (s.v[1919] > 0.0001);
        s.store_scalar(2469, if s.b[2469] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.b[2470] = (s.v[1920] < 1e-10);
        s.store_scalar(2470, if s.b[2470] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) && s.b[2470]) {
            s.store_scalar(1921, 1.0);
        }

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) && (!s.b[2470])) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && (!s.b[2469])) {
            s.store_scale(1921, 1919, 0.5);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul(2356, 1921, 2355);
        }

        s.b[2471] = ((s.v[700] > 0.0) && (s.v[701] > 0.0));
        s.store_scalar(2471, if s.b[2471] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {
            s.store_scaled_mul(2357, 2305, 2356, 0.475);
            s.store_add_scaled_product_indices(1919, 2342, 1.0, 2339, 2357, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2358, 1919, 1919, 1e-12, 0.5);
            s.store_add_scaled_value_products(2359, s.ad_value(2342), (-1.0), s.ad_value(2305), s.ad_value(2341), 1.0, A::offset(s.ad_value(2339), (-1.0)), s.ad_value(2357), 1.0);
            s.store_offset_div_scaled_product(2360, s.ad_value(2291), s.ad_value(2305), 0.5, s.ad_value(2359), 1.0, 1.0);
            s.store_add_scaled_product_indices(1919, 2359, 1.0, 769, 2358, 1.0);
            s.store_pow_ad(2361, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));
            s.store_mul_ad_lhs(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2360), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2361);
            s.store_div(1919, 2358, 2359);
            s.store_mul_pow_ad_rhs(2362, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));
            s.store_mul_div_scaled_product_mixed_iiai(1921, 2362, 701, A::add(A::offset(s.ad_value(2360), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1919), 1.0, 1.0)), 1.0, 2359, 1.0);
            s.store_mul_product3_indices(2363, 2358, 751, 2344, 2345, 1.0);
            s.store_offset_ad(1919, A::div_scaled_add_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), s.ad_value(2360), (-1.0), s.ad_value(1921), 1.0), 1.0);
        }

        s.b[2472] = (s.v[1919] < 230.25850929940458);
        s.store_scalar(2472, if s.b[2472] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && s.b[2472]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);
        }

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && (!s.b[2472])) {
            s.copy_ad(1920, 1919);
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {
            s.store_div_scaled_product3_mixed_iiia(2364, 2357, 1921, 1920, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2361), 1.0, s.ad_value(2362), 1.0, s.ad_value(2363), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2365, 2356, A::div_scaled_value_offset_denominator(s.ad_value(2364), 1.0, A::sqrt_square_offset(s.ad_value(2364), 1.0), 1.0, 1.0), 1.0);
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && (!s.b[2471])) {
            s.copy_ad(2365, 2356);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul3_affine_lhs(2366, 2305, 2354, 0.7071067811865475, 0.0, 2365);
        }

        s.b[2473] = (s.v[0] == (-1.0));
        s.store_scalar(2473, if s.b[2473] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2473]) {
            s.store_div_ad_rhs(2366, 2366, A::sqrt(A::offset(s.ad_value(2366), 1.0)));
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_div_from_scalar_offset_ad(2367, 2.0, A::sqrt(A::scale_offset(s.ad_value(2366), 4.0, 1.0)), 1.0);
            s.store_mul(1919, 2367, 2366);
            s.store_mul_ad_product_rhs_mixed_ia(2368, 2365, 2367, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2367)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2367), 4.0), 1.0)), 1.0));
            s.store_scale(2369, 2368, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1919, 2369, A::sub_scaled_inputs(s.ad_value(2369), 1.0, s.ad_value(2355), 2.0), 2307, 1.0, 2336, 1.0);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul_sub_ad_rhs(2370, 2305, s.ad_value(2369), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2468])) {
            s.copy_ad(2370, 2353);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_offset(1919, 2286, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 2370, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2371, 2370, 1919, 1.0, A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), 1.0, A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919))), 1.0, 1.0);
            s.store_mul(2372, 2371, 2306);
            s.store_add(2373, 2315, 2372);
        }

        s.b[2474] = (s.v[2372] < 460.51701859880916);
        s.store_scalar(2474, if s.b[2474] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2474]) {
            s.store_exp_neg_input(2374, 2372);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2474])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2374, 1e-200, 2372, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2375, 2330, 2374);
        }

        s.b[2475] = (((s.v[2309]) as f64).abs() <= s.v[2327]);
        s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2475]) {
            s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2376, 2309, 2328, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2375)), s.ad_value(2290), s.ad_value(2415)), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_offset(2436, 2373, 3.0);
            s.store_sub_ad(2419, A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2435), s.ad_value(2436)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2436), 0.5, A::sqrt_square_offset(s.ad_value(2436), 5.0), 0.5));
            s.store_sub(2414, 2309, 2419);
            s.store_exp_neg_input(2415, 2419);
            s.store_div_from_scalar_offset_square(2416, 1.0, 2419, 2.0);
            s.store_mul_square_lhs(2426, 2419, 2416);
            s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);
            s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), 2416, 2416);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2375] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                s.store_scalar(2420, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2420, 2414, 1.0, 2291, A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            }
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2437, 1.0, 2291, A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2421, 2414, 2.0, 2291, A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2422, 2373, 1.0, 2419, (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0);
            s.store_add(818, 2420, 2421);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0);
            s.store_add_ad_rhs(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));
        }

        s.b[2476] = (s.v[2438] < 230.25850929940458);
        s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && (!s.b[2475])) && s.b[2476]) {
            s.store_exp(2424, 2438);
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_mul(2424, 2375, 2424);
        }

        s.b[2477] = (s.v[2438] > (s.v[2373] - 230.25850929940458));
        s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });

        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && s.b[2477]) {
            s.store_exp_sub(2424, 2438, 2373);
            s.store_div(2425, 2375, 2424);
        }

        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && (!s.b[2477])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2424, 1e-100, A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2425, 1e-100, 2438, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_div_from_scalar_offset_square(2414, 1.0, 2438, 2.0);
            s.store_mul_square_lhs(2426, 2438, 2414);
            s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), 2414, 2414);
            s.store_sub(2414, 2309, 2438);
            s.store_add_scaled_product_right_ad(2429, 2414, 2.0, 2291, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2414, 2.0, 2291, A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2376, 2438, 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_sub(2377, 2376, 2329);
        }

        s.b[2478] = (s.v[2377] < 1e-10);
        s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2478]) {
            s.store_add_scaled_inputs_product_right_ad(2378, 2309, 2.0, 2329, (-2.0), 2291, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0), 1.0, s.ad_value(2375), s.ad_value(2332), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2379, A::mul_sub_from_scalar_rhs(s.ad_value(2291), 1.0, s.ad_value(2374)), 2336);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1919, 2.0, 2291, A::add_scaled_value_products(s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0, s.ad_value(2375), s.ad_value(2333), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1919, 2378, 1.0, 1919, 2379, (-2.0));
            s.store_scaled_div_ad_rhs(2377, 2379, A::add(s.ad_value(2378), A::sqrt(s.ad_value(1919))), 2.0);
            s.store_add(2376, 2329, 2377);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2380, 2377, 2305);
            s.store_div_scaled_product_offset_denominator(2381, s.ad_value(2376), s.ad_value(2376), 1.0, A::square(s.ad_value(2376)), 2.0, 1.0);
        }

        s.b[2479] = (s.v[2376] < 230.25850929940458);
        s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2479]) {
            s.store_exp_neg_input(2382, 2376);
        }

        s.b[2480] = (s.v[2376] < 1e-5);
        s.store_scalar(2480, if s.b[2480] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && s.b[2480]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2383, 2376, 1.0, 2376, 1.0, 2376, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2376), 1.0, A::scale(s.ad_value(2376), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2384, 2376, 1919, 0.7071067811865475);
            s.store_mul3_ad_middle(2385, A::mul3_scaled_output(s.ad_value(2375), s.ad_value(2376), s.ad_value(2376), 0.16666666666666666), 2376, A::scale_offset(s.ad_value(2376), 1.75, 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && (!s.b[2480])) {
            s.store_add_offset_lhs(2383, 2376, (-1.0), 2382);
            s.store_sqrt(2384, 2383);
            s.store_mul_add_scaled_inputs3_offset_rhs(2385, 2375, A::div_from_scalar(1.0, s.ad_value(2382)), 1.0, s.ad_value(2376), (-1.0), s.ad_value(2381), -1.0, (-1.0));
        }

        s.b[2481] = (s.v[2376] > (s.v[2373] - 230.25850929940458));
        s.store_scalar(2481, if s.b[2481] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && s.b[2481]) {
            s.store_exp_sub(1919, 2376, 2373);
            s.store_div(2382, 2375, 1919);
            s.store_add_scaled_product_right_ad(2385, 1919, 1.0, 2375, A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0));
        }

        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && (!s.b[2481])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2382, 1e-100, 2376, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1919, 1e-100, A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2385, 1919, 1.0, 2375, A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2479])) {
            s.store_add_offset_lhs(2383, 2376, (-1.0), 2382);
            s.store_sqrt(2384, 2383);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul3_lhs(2386, 2384, 2290, 2305);
            s.store_scaled_add(2387, 2329, 2376, 0.5);
            s.store_scalar(2388, 0.0);
            s.store_mul(1919, 2382, 2335);
        }

        s.b[2482] = (s.v[1919] > 0.0);
        s.store_scalar(2482, if s.b[2482] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2482]) {
            s.store_sqrt(2388, 1919);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_scaled_add(2389, 2336, 2385, 0.5);
            s.store_add_scaled_product_mixed_iaa(2390, 2389, 1.0, A::square(s.ad_value(2377)), A::sub_scaled_inputs(s.ad_value(2388), 1.0, s.ad_value(2307), 2.0), 0.125);
        }

        s.b[2483] = (s.v[2387] < 1e-5);
        s.store_scalar(2483, if s.b[2483] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2391, 2387, 1.0, 2387, 1.0, 2387, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
        }

        s.b[2484] = (s.v[724] > 0.0);
        s.store_scalar(2484, if s.b[2484] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && s.b[2483]) && s.b[2484]) {
            s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2387), 1.0, A::scale(s.ad_value(2387), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2394, 2387, 1919, 0.7071067811865475);
            s.store_add_ad_rhs(2395, 2393, A::div_scaled_product(s.ad_value(2290), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.5)), 1.0, A::square(s.ad_value(2387)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {
            s.store_add_offset_lhs(2391, 2387, (-1.0), 2388);
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
        }

        s.b[2485] = (s.v[724] > 0.0);
        s.store_scalar(2485, if s.b[2485] { 1.0 } else { 0.0 });

        if (((s.b[2439] && s.b[2467]) && (!s.b[2483])) && s.b[2485]) {
            s.store_add_scaled_sub_value_product_indices(2396, 1.0, 2388, 1.0, 2392, 2307, 2.0);
            s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));
            s.store_div_scaled_value_offset_denominator(1919, s.ad_value(2393), 1.0, s.ad_value(2393), 1.0, 1.0);
            s.store_mul_product3_mixed_iaii(2397, 724, A::square(s.ad_value(1919)), 2291, 2390, 1.0);
            s.store_add_scaled_inputs_product_right_ad(2398, 2392, 2.0, 2397, (-2.0), 2291, A::add(A::sub_from_scalar(1.0, s.ad_value(2388)), s.ad_value(2390)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2399, 2397, s.ad_value(2397), 1.0, s.ad_value(2392), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2400, 1.0, 2291, A::add(s.ad_value(2388), s.ad_value(2390)), 0.5);
            s.store_div_scaled_product_denominator_ad(2401, 2399, 2398, 1.0, A::add_scaled_square_product(s.ad_value(2398), 1.0, s.ad_value(2400), s.ad_value(2399), (-1.0)), 1.0);
            s.store_add(2387, 2387, 2401);
            s.store_exp(2402, 2401);
            s.store_div(2388, 2388, 2402);
            s.store_mul(2390, 2390, 2402);
            s.store_add_offset_lhs(2391, 2387, (-1.0), 2388);
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
            s.store_add_ad(2403, A::sub_from_scalar(1.0, s.ad_value(2388)), A::mul3_scaled_output(s.ad_value(2392), s.ad_value(2393), s.ad_value(2307), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2377, 2377, 2402, A::add(s.ad_value(2396), s.ad_value(2389)), 1.0, A::add_scaled_product(s.ad_value(2403), 1.0, s.ad_value(2402), s.ad_value(2389), 1.0), 1.0);
            s.store_mul(2380, 2377, 2305);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {
            s.store_sqrt(2394, 2391);
            s.store_add_scaled_inputs_ad_rhs(2395, 2393, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2388)), s.ad_value(2394)), 0.5);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul_div_scaled_product_mixed_iiia(2404, 2305, 2291, 2390, 1.0, A::add_scaled_product(s.ad_value(2392), 1.0, s.ad_value(2290), s.ad_value(2394), 1.0), 1.0);
            s.store_add_scaled_product_indices(2405, 2404, 1.0, 2305, 2395, 1.0);
            s.store_mul3_lhs(2406, 2394, 2290, 2305);
        }

        s.b[2486] = (s.v[213] < 0.0);
        s.store_scalar(2486, if s.b[2486] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2486]) {
            s.store_sub_from_scalar_scaled_mul(2345, 1.0, 213, 2404, 1.0);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2486])) {
            s.store_div_from_scalar_offset_product(2345, 1.0, 213, 2404, 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul_product3_indices(2346, 2404, 751, 2344, 2345, 1.0);
            s.store_add_scaled_product_indices(2407, 2406, 1.0, 769, 2404, 1.0);
            s.store_add_scaled_product_indices(2408, 2406, 1.0, 770, 2404, 1.0);
            s.store_mul(2409, 768, 2407);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2391), 1.0, A::add(s.ad_value(2391), s.ad_value(2390)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2348, A::pow(A::mul(s.ad_value(2409), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2410, A::offset(s.ad_value(2348), 1.0), s.ad_value(2346), 2340);
            s.store_ln_ad(2411, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(820), s.ad_value(2380)), s.ad_value(773)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2371), s.ad_value(2380)), s.ad_value(773)), 1.0), 1.0));
            s.store_mul(1921, 2404, 2350);
            s.store_div_add_scaled_inputs_rhs_indices(2351, 1921, 218, 1.0, 1921, 1.0);
        }

        s.b[2487] = (s.v[217] < 0.0);
        s.store_scalar(2487, if s.b[2487] { 1.0 } else { 0.0 });

        if ((s.b[2439] && s.b[2467]) && s.b[2487]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2487])) {
            s.store_offset_mul(2352, 217, 2351, 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2413, 2285, 2352);
            s.store_mul(2412, 2392, 2305);
        }

        if s.b[2439] {
            s.copy_ad(1875, 2287);
            s.copy_ad(1876, 2305);
            s.copy_ad(1877, 2290);
            s.copy_ad(1878, 2309);
            s.copy_ad(1879, 2314);
            s.copy_ad(1880, 2343);
            s.copy_ad(1881, 2380);
            s.copy_ad(1882, 2386);
            s.copy_ad(1883, 2393);
            s.copy_ad(1884, 2395);
            s.copy_ad(1885, 2404);
            s.copy_ad(1886, 2405);
            s.copy_ad(1887, 2408);
            s.copy_ad(1888, 2410);
            s.copy_ad(1889, 2411);
            s.copy_ad(1890, 2413);
            s.copy_ad(1891, 2412);
        }

        if (!s.b[2439]) {
            s.copy_ad(739, 722);
            s.copy_ad(1875, 1810);
            s.copy_ad(1876, 1812);
            s.copy_ad(1877, 1814);
            s.copy_ad(1878, 1817);
            s.copy_ad(1879, 1818);
            s.copy_ad(1880, 1837);
            s.copy_ad(1881, 1848);
            s.copy_ad(1882, 1849);
            s.copy_ad(1883, 1851);
            s.copy_ad(1884, 1852);
            s.copy_ad(1885, 1853);
            s.copy_ad(1886, 1854);
            s.copy_ad(1887, 1856);
            s.copy_ad(1888, 1857);
            s.copy_ad(1889, 1859);
            s.copy_ad(1890, 1858);
            s.copy_ad(1891, 1860);
        }

        s.copy_ad(1892, 250);

        s.b[2488] = (s.v[767] > 0.0);
        s.store_scalar(2488, if s.b[2488] { 1.0 } else { 0.0 });

        if s.b[2488] {
            s.store_div_scaled_value_offset_denominator(1892, s.ad_value(250), 1.0, A::mul(s.ad_value(767), A::powf(A::offset(A::square(s.ad_value(1887)), s.v[727]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);
        }

        s.store_scalar(1893, 1.0);

        s.store_scalar(1894, 1.0);

        s.store_scalar(1895, 0.0);

        s.store_scalar(1896, 1.0);

        s.store_scalar(1897, 1.0);

        s.copy_ad(2251, 1891);

        s.store_scalar(2254, 0.0);

        s.store_scalar(2253, 0.0);

        s.copy_ad(2255, 2251);

        s.b[2489] = (s.v[1878] > 0.0);
        s.store_scalar(2489, if s.b[2489] { 1.0 } else { 0.0 });

        if s.b[2489] {
            s.store_mul_div_scaled_product_mixed_iaii(2246, 1889, A::add(s.ad_value(255), A::div(s.ad_value(256), s.ad_value(1886))), 1885, 1.0, 1886, 1.0);
        }

        s.b[2490] = (s.v[2246] > 0.0);
        s.store_scalar(2490, if s.b[2490] { 1.0 } else { 0.0 });

        if (s.b[2489] && s.b[2490]) {
            s.store_div_from_scalar_add_ad(1893, 1.0, A::offset(s.ad_value(2246), 1.0), A::square(s.ad_value(2246)));
        }

        if (s.b[2489] && (!s.b[2490])) {
            s.store_sub_from_scalar(1893, 1.0, 2246);
        }

        if s.b[2489] {
            s.store_mul(1894, 1888, 1893);
            s.store_div(1895, 1890, 1894);
            s.store_mul_ad_product_lhs_mixed_ai(2247, A::square(s.ad_value(1895)), 1881, 1881);
        }

        s.b[2491] = (s.v[0] == (-1.0));
        s.store_scalar(2491, if s.b[2491] { 1.0 } else { 0.0 });

        if (s.b[2489] && s.b[2491]) {
            s.store_div_scaled_value_offset_denominator(2247, s.ad_value(2247), 1.0, A::mul(s.ad_value(1895), s.ad_value(1881)), 1.0, 1.0);
        }

        if s.b[2489] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1896, 1894, A::sqrt(A::scale_offset(s.ad_value(2247), 2.0, 1.0)), 1.0, 0.5);
            s.store_div(1919, 1894, 1896);
            s.store_mul_offset_ad_rhs(2248, 1884, A::mul3_scaled_output(s.ad_value(2247), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0);
            s.store_div_scaled_product_indices(1897, 1919, 1886, 1.0, 2248, 1.0);
            s.store_scaled_div(2249, 1881, 1897, 0.5);
            s.store_square(2250, 2249);
            s.store_add_product3_rhs_mixed_iia(2251, 1891, 1883, 1881, A::add(A::offset(A::mul_scaled_output(s.ad_value(2249), s.ad_value(1893), 0.3333333333333333), (-1.0)), s.ad_value(1893)), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2489] {
            s.store_scaled_mul(1919, 1884, 1881, 0.16666666666666666);
        }

        s.b[2492] = (p.p49 == 1.0);
        s.store_scalar(2492, if s.b[2492] { 1.0 } else { 0.0 });

        if (s.b[2489] && s.b[2492]) {
            s.store_scalar(2252, 0.0);
            s.store_mul_ad_affine_product_rhs(2253, 1893, s.ad_value(1893), A::sub(s.ad_value(1885), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 2.0, s.ad_value(2249), 3.0)), 0.5, 0.0);
        }

        if (s.b[2489] && (!s.b[2492])) {
            s.store_mul_sub_from_scalar_lhs_ad_rhs(2252, 1.0, 1893, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1884), s.ad_value(1881), (-0.5)));
            s.store_add_scaled_products_mixed_aaia(2253, A::square(s.ad_value(1893)), A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2250), 0.2), (-1.0)), 0.5, 2252, A::offset(s.ad_value(1893), 1.0), 0.5);
        }

        if s.b[2489] {
            s.store_add_scaled_product_right_ad(2254, 2252, 1.0, 1893, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), s.ad_value(2249), 1.0), 1.0);
            s.store_sub(2255, 2251, 2254);
        }

        s.store_mul(845, 2251, 1892);

        s.store_mul_neg_lhs(847, 2253, 1892);

        s.store_mul_neg_lhs(846, 2255, 1892);

        s.store_scalar(2271, 0.0);

        s.store_scalar(2272, 0.0);

        s.store_scalar(2270, 0.0);

        s.b[2493] = ((s.v[263] > 0.0) || (s.v[264] > 0.0));
        s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });

        if s.b[2493] {
            s.store_scalar(2260, 1.0);
            s.copy_ad(2259, 1875);
        }

        s.b[2494] = (s.v[267] > 1e-10);
        s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });

        if (s.b[2493] && s.b[2494]) {
            s.store_add_scaled_inputs3_indices(2256, 1875, 1.0, 265, (-1.0), 802, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1919, 2256, 0.5, 802, 0.5, A::add(A::square(A::sub(s.ad_value(2256), s.ad_value(802))), s.ad_value(803)), 0.5);
            s.store_mul_add_scaled_inputs3_offset_rhs(1920, 1919, s.ad_value(1919), 2.0, s.ad_value(802), (-1.0), s.ad_value(2256), -1.0, 0.0);
            s.store_div(1921, 802, 1919);
            s.store_mul(2257, 2256, 1921);
            s.store_sqrt_sub_from_scalar_ad(2258, 1.0, A::mul(s.ad_value(2257), s.ad_value(267)));
            s.store_add_scaled_inputs3_mixed_aii(2259, A::div(A::sub_from_scalar(1.0, s.ad_value(2258)), s.ad_value(267)), 1.0, 2256, 1.0, 2257, -1.0);
            s.store_offset_ad(2260, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2258)), (-1.0)), A::add_scaled_product(s.ad_value(1920), 1.0, s.ad_value(2256), A::sub(s.ad_value(802), s.ad_value(1919)), 1.0), s.ad_value(1921), 1.0, s.ad_value(1920), 1.0), 1.0);
        }

        if s.b[2493] {
            s.store_scalar(2262, 1.0);
            s.store_scalar(2263, 0.0);
        }

        s.b[2495] = (s.v[266] > 0.0);
        s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });

        if (s.b[2493] && s.b[2495]) {
            s.store_add_scaled_product_right_ad(1919, 739, 0.5, 1876, A::scale_offset(s.ad_value(1877), 0.7071067811865475, 1.0), 1.0);
            s.store_div(2261, 1875, 1919);
        }

        s.b[2496] = (((s.v[2261]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });

        if ((s.b[2493] && s.b[2495]) && s.b[2496]) {
            s.store_div_from_scalar_offset_ad(2262, 1.0, A::exp_scaled_input(s.ad_value(2261), -1.0), 1.0);
        }

        s.b[2497] = (s.v[2261] < 0.0);
        s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });

        if (((s.b[2493] && s.b[2495]) && (!s.b[2496])) && s.b[2497]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2262, 1e-100, 2261, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2498] = (s.v[2261] < 230.25850929940458);
        s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });

        if ((s.b[2493] && s.b[2495]) && s.b[2498]) {
            s.store_ln_one_plus_exp(1920, 2261);
        }

        if ((s.b[2493] && s.b[2495]) && (!s.b[2498])) {
            s.copy_ad(1920, 2261);
        }

        if (s.b[2493] && s.b[2495]) {
            s.store_mul(2263, 1919, 1920);
        }

        if s.b[2493] {
            s.store_add_scaled_product_right_sub(2264, 2260, 1.0, 266, 2262, 2260, 1.0);
            s.store_add_scaled_product_right_sub(2265, 2259, 1.0, 266, 2263, 2259, 1.0);
            s.store_add_scaled_inputs3_mixed_aii(2266, A::add_scaled_product(s.ad_value(1875), 1.0, s.ad_value(1876), s.ad_value(1879), (-1.0)), 1.0, 1891, (-1.0), 1881, (-0.5));
            s.store_add_scaled_inputs3_indices(2267, 1875, 1.0, 2266, (-1.0), 1880, -1.0);
            s.store_add_scaled_inputs3_indices(2268, 1881, 1.0, 2266, 1.0, 820, -1.0);
            s.store_add_scaled_inputs3_indices(2269, 1875, 1.0, 2268, (-1.0), 1882, -1.0);
        }

        s.b[2499] = (s.v[825] > 0.0);
        s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });

        if (s.b[2493] && s.b[2499]) {
            s.store_mul_ad_rhs(2270, 2264, A::add_scaled_products(s.ad_value(264), s.ad_value(2268), 1.0, s.ad_value(263), s.ad_value(2266), 1.0));
            s.store_mul_sub_rhs(2271, 263, 2267, 2265);
            s.store_mul_sub_rhs(2272, 264, 2269, 2265);
        }

        if (s.b[2493] && (!s.b[2499])) {
            s.store_mul_ad_rhs(2270, 2264, A::add_scaled_products(s.ad_value(263), s.ad_value(2268), 1.0, s.ad_value(264), s.ad_value(2266), 1.0));
            s.store_mul_sub_rhs(2271, 264, 2267, 2265);
            s.store_mul_sub_rhs(2272, 263, 2269, 2265);
        }

        if s.b[2493] {
            s.store_add(845, 845, 2270);
            s.store_add(847, 847, 2272);
            s.store_add_scaled_inputs4_indices(846, 846, 1.0, 2270, (-1.0), 2272, -1.0, 2271, -1.0);
        }

        s.store_mul(1898, 257, 1866);

        s.store_mul(1899, 258, 1867);

        s.store_scalar(2275, 0.0);

        s.store_scalar(2273, 0.0);

        s.b[2500] = ((s.v[257] > 0.0) && (s.v[259] > 0.0));
        s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });

        if s.b[2500] {
            s.store_mul_add_scaled_inputs_rhs(1919, 261, s.ad_value(1807), 0.5, s.ad_value(781), 1.0);
        }

        s.b[2501] = (s.v[1919] < 230.25850929940458);
        s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });

        s.b[2502] = (s.v[1919] > (-230.25850929940458));
        s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });

        if ((s.b[2500] && s.b[2501]) && s.b[2502]) {
            s.store_exp(2273, 1919);
        }

        if ((s.b[2500] && s.b[2501]) && (!s.b[2502])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2273, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2503] = (s.v[2273] > 1e-10);
        s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });

        if ((s.b[2500] && s.b[2501]) && s.b[2503]) {
            s.store_ln_offset_input(2274, 2273, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2274, 1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)));
        }

        if ((s.b[2500] && s.b[2501]) && (!s.b[2503])) {
            s.copy_ad(2274, 2273);
            s.store_div_scaled_value_offset_denominator(1920, s.ad_value(2274), 2.0, s.ad_value(2274), 2.0, 1.0);
        }

        if (s.b[2500] && (!s.b[2501])) {
            s.copy_ad(2274, 1919);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2274, 1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)));
        }

        if s.b[2500] {
            s.store_mul_ad_affine_product_lhs(2275, A::div_scaled_inputs(s.ad_value(259), (-2.0), s.ad_value(261), 1.0), s.ad_value(257), s.v[348], 0.0, 1920);
        }

        s.store_scalar(2278, 0.0);

        s.store_scalar(2276, 0.0);

        s.b[2504] = ((s.v[258] > 0.0) && (s.v[260] > 0.0));
        s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });

        if s.b[2504] {
            s.store_mul_add_scaled_inputs_rhs(1919, 261, s.ad_value(1807), 0.5, s.ad_value(782), 1.0);
        }

        s.b[2505] = (s.v[1919] < 230.25850929940458);
        s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });

        s.b[2506] = (s.v[1919] > (-230.25850929940458));
        s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });

        if ((s.b[2504] && s.b[2505]) && s.b[2506]) {
            s.store_exp(2276, 1919);
        }

        if ((s.b[2504] && s.b[2505]) && (!s.b[2506])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2276, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2507] = (s.v[2276] > 1e-10);
        s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });

        if ((s.b[2504] && s.b[2505]) && s.b[2507]) {
            s.store_ln_offset_input(2277, 2276, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2277, 1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)));
        }

        if ((s.b[2504] && s.b[2505]) && (!s.b[2507])) {
            s.copy_ad(2277, 2276);
            s.store_div_scaled_value_offset_denominator(1920, s.ad_value(2277), 2.0, s.ad_value(2277), 2.0, 1.0);
        }

        if (s.b[2504] && (!s.b[2505])) {
            s.copy_ad(2277, 1919);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2277, 1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)));
        }

        if s.b[2504] {
            s.store_mul_ad_affine_product_lhs(2278, A::div_scaled_inputs(s.ad_value(260), (-2.0), s.ad_value(261), 1.0), s.ad_value(258), s.v[348], 0.0, 1920);
        }

        s.store_add(2279, 2275, 2278);

        s.store_add_scaled_product_indices(850, 2279, 1.0, 262, 823, 1.0);

        s.store_mul(848, 269, 828);

        s.store_mul(849, 270, 831);

        s.store_scalar(2508, 0.0);

        s.store_scalar(2511, 0.0);

        s.store_scalar(2512, 0.0);

        s.store_scalar(2513, 0.0);

        s.store_scalar(2514, 0.0);

        s.store_scalar(2515, 0.0);

        s.store_scalar(2516, 0.0);

        s.store_scalar(2517, 0.0);

        s.store_scalar(2518, 0.0);

        s.store_scalar(2519, 0.0);

        s.store_scalar(2520, 0.0);

        s.store_scalar(2521, 0.0);

        s.store_scalar(2522, 0.0);

        s.store_scalar(2523, 0.0);

        s.store_scalar(2524, 0.0);

        s.store_scalar(2525, 0.0);

        s.store_scalar(2526, 0.0);

        s.store_scalar(2529, 0.0);

        s.store_scalar(2533, 0.0);

        s.store_scalar(2536, 0.0);

        s.store_scalar(2537, 0.0);

        s.store_scalar(2538, 0.0);

        s.store_scalar(2539, 0.0);

        s.store_scalar(2540, 0.0);

        s.store_scalar(2541, 0.0);

        s.store_scalar(2544, 0.0);

        s.store_scalar(2545, 0.0);

        s.store_scalar(2546, 0.0);

        s.store_scalar(2547, 0.0);

        s.store_scalar(2551, 0.0);

        s.store_scalar(2553, 0.0);

        s.store_scalar(2554, 0.0);

        s.store_scalar(851, 0.0);

        s.store_scalar(1906, 0.0);

        s.store_scalar(1907, 0.0);

        s.store_scalar(1908, 0.0);

        s.store_scalar(852, 0.0);

        s.store_scalar(1909, 0.0);

        s.store_scalar(1910, 0.0);

        s.store_scalar(1911, 0.0);

        s.b[2555] = (p.p43 > 0.0);
        s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });

        s.b[2556] = (s.v[468] == 1.0);
        s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });

        if (s.b[2555] && s.b[2556]) {
            s.store_scalar(2559, 0.0);
            s.store_scalar(2560, 0.0);
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product_indices(2513, 826, 1.0, 651, 2512, 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2560, 826, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2561] = (s.v[645] > 0.5);
        s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });

        s.b[2562] = (s.v[402] == 0.5);
        s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });

        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && s.b[2562]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[399]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && (!s.b[2562])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])), s.v[402]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2561]) {
            s.store_add_scaled_inputs3_offset_indices(1906, 2559, (-s.v[411]), 826, s.v[414], 2560, (-s.v[414]), s.v[411]);
        }

        s.b[2563] = (s.v[646] > 0.5);
        s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });

        s.b[2564] = (s.v[403] == 0.5);
        s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });

        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && s.b[2564]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[400]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && (!s.b[2564])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])), s.v[403]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2563]) {
            s.store_add_scaled_inputs3_offset_indices(1907, 2559, (-s.v[412]), 826, s.v[415], 2560, (-s.v[415]), s.v[412]);
        }

        s.b[2565] = (s.v[647] > 0.5);
        s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });

        s.b[2566] = (s.v[404] == 0.5);
        s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });

        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && s.b[2566]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[401]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && (!s.b[2566])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])), s.v[404]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2565]) {
            s.store_add_scaled_inputs3_offset_indices(1908, 2559, (-s.v[413]), 826, s.v[416], 2560, (-s.v[416]), s.v[413]);
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_scalar(2559, 0.0);
            s.store_scalar(2560, 0.0);
            s.store_scaled_mul(2511, 678, 678, 4.0);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2555] && s.b[2556]) {
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product_indices(2513, 827, 1.0, 678, 2512, 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2560, 827, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2567] = (s.v[672] > 0.5);
        s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });

        s.b[2568] = (s.v[569] == 0.5);
        s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });

        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && s.b[2568]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(566)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && (!s.b[2568])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2559, 1.0, 2560, 566, 569);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2567]) {
            s.store_add_scaled_product_mixed_aia(1909, A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2559)), 1.0, 581, A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);
        }

        s.b[2569] = (s.v[673] > 0.5);
        s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });

        s.b[2570] = (s.v[570] == 0.5);
        s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });

        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && s.b[2570]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(567)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && (!s.b[2570])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2559, 1.0, 2560, 567, 570);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2569]) {
            s.store_add_scaled_product_mixed_aia(1910, A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2559)), 1.0, 582, A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);
        }

        s.b[2571] = (s.v[674] > 0.5);
        s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });

        s.b[2572] = (s.v[571] == 0.5);
        s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });

        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && s.b[2572]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(568)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && (!s.b[2572])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2559, 1.0, 2560, 568, 571);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2571]) {
            s.store_add_scaled_product_mixed_aia(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2559)), 1.0, 583, A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);
        }

        s.b[2573] = (p.p865 > 0.0);
        s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2573]) {
            s.store_scaled_offset_ad(636, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), p.p866), (-(((0.5 * 0.001)) as f64).powf(p.p866)), p.p865);
            s.store_offset(634, 636, p.p855);
            s.store_div_from_scalar(444, 1.0, 634);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2573])) {
            s.store_scalar(634, p.p855);
        }

        s.b[2574] = (p.p867 > 0.0);
        s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2574]) {
            s.store_scaled_offset_ad(638, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), p.p868), (-(((0.5 * 0.001)) as f64).powf(p.p868)), p.p867);
            s.store_mul_offset_rhs(437, 437, 638, 1.0);
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_scalar(2524, 0.0);
            s.store_scalar(2521, 0.0);
        }

        s.b[2575] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product_indices(2513, 826, 1.0, 651, 2512, 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2518, 826, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2576] = (s.v[826] < s.v[648]);
        s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });

        s.b[2577] = (((((-0.5) * (s.v[826] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && s.b[2577]) {
            s.store_exp_scaled_input(2519, 826, (s.v[365] * (-0.5)));
        }

        s.b[2578] = (((-0.5) * (s.v[826] * s.v[365])) < 0.0);
        s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && s.b[2578]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2519, 1e-100, (-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && (!s.b[2578])) {
            s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(826), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) {
            s.store_div_from_scalar(2520, 1.0, 2519);
            s.store_square(2517, 2520);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2576])) {
            s.store_mul_offset_ad_lhs(2517, A::sub_scaled_inputs(s.ad_value(826), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
            s.store_sqrt(2520, 2517);
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.b[2579] = (s.v[826] > 0.0);
        s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2579]) {
            s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2579])) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 826);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_sub(2522, 650, 2521);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2523, 826, 0.5, 2522, 0.5, 826, 2522, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2524, 826, 0.5, 653, 0.5, 826, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2525, 826, 826, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2580] = (s.v[640] == 0.0);
        s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2580]) {
            s.store_scalar(1906, 0.0);
        }

        s.b[2581] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {
            s.store_sub_from_scalar(2529, s.v[387], 2523);
        }

        s.b[2583] = (p.p824 == 0.5);
        s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && s.b[2583]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[423]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2583])) {
            s.store_powf_scaled_input(2526, 2529, s.v[423], p.p824);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {
            s.store_scale(2533, 2526, s.v[417]);
        }

        s.b[2584] = (p.p838 == 0.0);
        s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) {
            s.store_div_scaled_inputs_indices(2536, 2533, (s.v[402] * s.v[432]), 2529, 1.0);
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[429]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);
            s.store_add_scaled_value_products(2546, s.ad_value(2539), (-s.v[429]), s.ad_value(2537), s.ad_value(2540), s.v[429], s.ad_value(2536), s.ad_value(2541), 0.5);
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2587] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && s.b[2587]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2587])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2588] = (s.v[2547] > 0.0);
        s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });

        s.b[2589] = (s.v[2546] > (-230.25850929940458));
        s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && s.b[2589]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && (!s.b[2589])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2590] = (p.p844 == 0.0);
        s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });

        s.b[2591] = (p.p824 == 0.5);
        s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2591]) {
            s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2591])) {
            s.store_powf_scale_offset_input(2526, 2524, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) {
            s.store_div_scaled_offset_numerator(2551, s.ad_value(2524), ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), s.ad_value(2526), 1.0);
        }

        s.b[2592] = (((((-s.v[435]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2592]) {
            s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2593] = (((-s.v[435]) / s.v[2551]) < 0.0);
        s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && s.b[2593]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 435, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && (!s.b[2593])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 435, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2594] = (p.p853 > 1000.0);
        s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });

        s.b[2595] = (s.v[2525] > ((-s.v[438]) * p.p853));
        s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });

        s.b[2596] = (p.p856 == 4.0);
        s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && s.b[2596]) {
            s.store_mul_scaled_ad_lhs(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[442] * s.v[442]) * s.v[442])), 2525, s.v[442]);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && (!s.b[2596])) {
            s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[442]), p.p856);
        }

        s.b[2597] = (s.v[402] == 0.5);
        s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2597]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[399]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2597])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])), s.v[402]);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) {
            s.store_add_scaled_inputs3_offset_indices(1906, 2526, ((-s.v[411]) * p.p30), 826, (s.v[414] * p.p30), 2518, ((-s.v[414]) * p.p30), (s.v[411] * p.p30));
        }

        s.b[2598] = (s.v[641] == 0.0);
        s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2598]) {
            s.store_scalar(1907, 0.0);
        }

        s.b[2599] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_sub_from_scalar(2529, s.v[388], 2523);
        }

        s.b[2601] = (p.p825 == 0.5);
        s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && s.b[2601]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[424]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && (!s.b[2601])) {
            s.store_powf_scaled_input(2526, 2529, s.v[424], p.p825);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_scale(2533, 2526, s.v[418]);
        }

        s.b[2602] = (p.p839 == 0.0);
        s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {
            s.store_div_scaled_inputs_indices(2536, 2533, (s.v[403] * s.v[433]), 2529, 1.0);
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[430]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);
            s.store_add_scaled_value_products(2546, s.ad_value(2539), (-s.v[430]), s.ad_value(2537), s.ad_value(2540), s.v[430], s.ad_value(2536), s.ad_value(2541), 0.5);
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2605] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2605]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2605])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2606] = (s.v[2547] > 0.0);
        s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });

        s.b[2607] = (s.v[2546] > (-230.25850929940458));
        s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && s.b[2607]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && (!s.b[2607])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2608] = (p.p845 == 0.0);
        s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });

        s.b[2609] = (p.p825 == 0.5);
        s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2609]) {
            s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2609])) {
            s.store_powf_scale_offset_input(2526, 2524, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) {
            s.store_div_scaled_offset_numerator(2551, s.ad_value(2524), ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), s.ad_value(2526), 1.0);
        }

        s.b[2610] = (((((-s.v[436]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2610]) {
            s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2611] = (((-s.v[436]) / s.v[2551]) < 0.0);
        s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && s.b[2611]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 436, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && (!s.b[2611])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 436, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2612] = (p.p854 > 1000.0);
        s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });

        s.b[2613] = (s.v[2525] > ((-s.v[438]) * p.p854));
        s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });

        s.b[2614] = (p.p857 == 4.0);
        s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && s.b[2614]) {
            s.store_mul_scaled_ad_lhs(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[443] * s.v[443]) * s.v[443])), 2525, s.v[443]);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && (!s.b[2614])) {
            s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[443]), p.p857);
        }

        s.b[2615] = (s.v[403] == 0.5);
        s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2615]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[400]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2615])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])), s.v[403]);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {
            s.store_add_scaled_inputs3_offset_indices(1907, 2526, ((-s.v[412]) * p.p30), 826, (s.v[415] * p.p30), 2518, ((-s.v[415]) * p.p30), (s.v[412] * p.p30));
        }

        s.b[2616] = (s.v[642] == 0.0);
        s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2616]) {
            s.store_scalar(1908, 0.0);
        }

        s.b[2617] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_sub_from_scalar(2529, s.v[389], 2523);
        }

        s.b[2619] = (p.p826 == 0.5);
        s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && s.b[2619]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[425]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && (!s.b[2619])) {
            s.store_powf_scaled_input(2526, 2529, s.v[425], p.p826);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_scale(2533, 2526, s.v[419]);
        }

        s.b[2620] = (p.p840 == 0.0);
        s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {
            s.store_div_scaled_inputs_indices(2536, 2533, (s.v[404] * s.v[434]), 2529, 1.0);
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[431]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);
            s.store_add_scaled_value_products(2546, s.ad_value(2539), (-s.v[431]), s.ad_value(2537), s.ad_value(2540), s.v[431], s.ad_value(2536), s.ad_value(2541), 0.5);
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2623] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.store_scalar(2623, if s.b[2623] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2623]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2623])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2624] = (s.v[2547] > 0.0);
        s.store_scalar(2624, if s.b[2624] { 1.0 } else { 0.0 });

        s.b[2625] = (s.v[2546] > (-230.25850929940458));
        s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && s.b[2625]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && (!s.b[2625])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2626] = (p.p846 == 0.0);
        s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });

        s.b[2627] = (p.p826 == 0.5);
        s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2627]) {
            s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2627])) {
            s.store_powf_scale_offset_input(2526, 2524, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) {
            s.store_div_scaled_offset_numerator(2551, s.ad_value(2524), ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), s.ad_value(2526), 1.0);
        }

        s.b[2628] = (((((-s.v[437]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2628]) {
            s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2629] = (((-s.v[437]) / s.v[2551]) < 0.0);
        s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && s.b[2629]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 437, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && (!s.b[2629])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 437, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2630] = (s.v[634] > 1000.0);
        s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });

        s.b[2631] = (s.v[2525] > ((-s.v[438]) * s.v[634]));
        s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });

        s.b[2632] = (p.p858 == 4.0);
        s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && s.b[2632]) {
            s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(444))), s.ad_value(2525), s.ad_value(444)), 2525, 444);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && (!s.b[2632])) {
            s.store_powf_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(444))), p.p858);
        }

        s.b[2633] = (s.v[467] == 1.0);
        s.store_scalar(2633, if s.b[2633] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            if (s.v[826] < p.p863) {
                if (((s.v[826] - p.p863) / p.p864) < (-37.0)) {
                    s.store_scalar(2553, p.p863);
                } else {
                    s.store_offset_scaled_ad(2553, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(826), (-p.p863), 1.0 / (p.p864))), p.p864, p.p863);
                }
            } else {
                if (((s.v[826] - p.p863) / p.p864) > 37.0) {
                    s.copy_ad(2553, 826);
                } else {
                    s.store_add_scaled_inputs_ad_rhs(2553, 826, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(826), (-1.0 / (p.p864)), ((p.p863) * (1.0 / (p.p864))))), p.p864);
                }
            }
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product_indices(2513, 2553, 1.0, 651, 2512, 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2634] = (s.v[404] == 0.5);
        s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2634]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2554), s.v[401]));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2634])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])), s.v[404]);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_add_scaled_inputs3_offset_indices(1908, 2526, ((-s.v[413]) * p.p30), 2553, (s.v[416] * p.p30), 2554, ((-s.v[416]) * p.p30), (s.v[413] * p.p30));
            s.store_sub_offset_lhs(2553, 826, p.p863, 2553);
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product_indices(2513, 2553, 1.0, 651, 2512, 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2635] = (s.v[461] == 0.5);
        s.store_scalar(2635, if s.b[2635] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2635]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(460)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2635])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2554, 460, 461);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_add_scaled_product_mixed_aia(466, A::mul_sub_from_scalar_rhs(s.ad_value(464), 1.0, s.ad_value(2526)), p.p30, 465, A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);
            s.store_add(1908, 1908, 466);
        }

        s.b[2636] = (s.v[404] == 0.5);
        s.store_scalar(2636, if s.b[2636] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && s.b[2636]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[401]));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && (!s.b[2636])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])), s.v[404]);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) {
            s.store_add_scaled_inputs3_offset_indices(1908, 2526, ((-s.v[413]) * p.p30), 826, (s.v[416] * p.p30), 2518, ((-s.v[416]) * p.p30), (s.v[413] * p.p30));
        }

        s.b[2637] = (s.v[630] > 0.0);
        s.store_scalar(2637, if s.b[2637] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2637]) {
            s.store_mul_sub_ad_rhs(637, 630, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), s.ad_value(631)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(631)));
            s.store_add(635, 536, 637);
            s.store_div_from_scalar(610, 1.0, 635);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2637])) {
            s.copy_ad(635, 536);
        }

        s.b[2638] = (s.v[632] > 0.0);
        s.store_scalar(2638, if s.b[2638] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2638]) {
            s.store_mul_sub_ad_rhs(639, 632, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), s.ad_value(633)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(633)));
            s.store_mul_offset_rhs(604, 604, 639, 1.0);
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_scalar(2524, 0.0);
            s.store_scalar(2521, 0.0);
        }

        s.b[2639] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.store_scalar(2639, if s.b[2639] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product_indices(2513, 827, 1.0, 678, 2512, 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2518, 827, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2640] = (s.v[827] < s.v[675]);
        s.store_scalar(2640, if s.b[2640] { 1.0 } else { 0.0 });

        s.b[2641] = (((((-0.5) * (s.v[827] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.store_scalar(2641, if s.b[2641] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && s.b[2641]) {
            s.store_exp_scaled_input(2519, 827, (s.v[365] * (-0.5)));
        }

        s.b[2642] = (((-0.5) * (s.v[827] * s.v[365])) < 0.0);
        s.store_scalar(2642, if s.b[2642] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && s.b[2642]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2519, 1e-100, (-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && (!s.b[2642])) {
            s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(827), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) {
            s.store_div_from_scalar(2520, 1.0, 2519);
            s.store_square(2517, 2520);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2640])) {
            s.store_mul_offset_ad_lhs(2517, A::sub_scaled_inputs(s.ad_value(827), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(2520, 2517);
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.b[2643] = (s.v[827] > 0.0);
        s.store_scalar(2643, if s.b[2643] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2643]) {
            s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2643])) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 827);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_sub(2522, 677, 2521);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2523, 827, 0.5, 2522, 0.5, 827, 2522, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2524, 827, 0.5, 680, 0.5, 827, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2525, 827, 827, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2644] = (s.v[667] == 0.0);
        s.store_scalar(2644, if s.b[2644] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2644]) {
            s.store_scalar(1909, 0.0);
        }

        s.b[2645] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.store_scalar(2645, if s.b[2645] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_sub(2529, 563, 2523);
        }

        s.b[2647] = (s.v[505] == 0.5);
        s.store_scalar(2647, if s.b[2647] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && s.b[2647]) {
            s.store_sqrt_mul(2526, 2529, 590);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && (!s.b[2647])) {
            s.store_pow_mul_base_indices(2526, 2529, 590, 505);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_mul(2533, 584, 2526);
        }

        s.b[2648] = (s.v[519] == 0.0);
        s.store_scalar(2648, if s.b[2648] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {
            s.store_mul_div_scaled_product_indices(2536, 599, 2533, 569, 1.0, 2529, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {
            s.store_div_scaled_inputs_indices(2537, 596, 0.666666666666667, 2536, 1.0);
            s.store_square(2538, 2537);
            s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);
            s.store_add_scaled_value_products(2546, A::mul3(s.ad_value(596), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(596), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5);
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2651] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2651]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2651])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2652] = (s.v[2547] > 0.0);
        s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });

        s.b[2653] = (s.v[2546] > (-230.25850929940458));
        s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && s.b[2653]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && (!s.b[2653])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2654] = (s.v[525] == 0.0);
        s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });

        s.b[2655] = (s.v[505] == 0.5);
        s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2655]) {
            s.store_sqrt_mul_sub_lhs(2526, 502, 2524, 590);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2655])) {
            s.store_pow_mul_base_mixed_ai(2526, A::sub(s.ad_value(502), s.ad_value(2524)), 590, 505);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) {
            s.store_mul_div_scaled_product_mixed_iaii(2551, 572, A::sub(s.ad_value(502), s.ad_value(2524)), 587, 1.0, 2526, 1.0);
        }

        s.b[2656] = (((((-s.v[602]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2656]) {
            s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2657] = (((-s.v[602]) / s.v[2551]) < 0.0);
        s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && s.b[2657]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 602, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && (!s.b[2657])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 602, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2658] = (s.v[534] > 1000.0);
        s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });

        s.b[2659] = (s.v[2525] > ((-s.v[438]) * s.v[534]));
        s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });

        s.b[2660] = (s.v[537] == 4.0);
        s.store_scalar(2660, if s.b[2660] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && s.b[2660]) {
            s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(2525), s.ad_value(608)), 2525, 608);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && (!s.b[2660])) {
            s.store_pow_abs_mul_base_indices(2526, 2525, 608, 537);
        }

        s.b[2661] = (s.v[569] == 0.5);
        s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2661]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(566)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2661])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2518, 566, 569);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {
            s.store_add_scaled_product_mixed_aia(1909, A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2526)), p.p30, 581, A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);
        }

        s.b[2662] = (s.v[668] == 0.0);
        s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2662]) {
            s.store_scalar(1910, 0.0);
        }

        s.b[2663] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.store_scalar(2663, if s.b[2663] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_sub(2529, 564, 2523);
        }

        s.b[2665] = (s.v[506] == 0.5);
        s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && s.b[2665]) {
            s.store_sqrt_mul(2526, 2529, 591);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && (!s.b[2665])) {
            s.store_pow_mul_base_indices(2526, 2529, 591, 506);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_mul(2533, 585, 2526);
        }

        s.b[2666] = (s.v[520] == 0.0);
        s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {
            s.store_mul_div_scaled_product_indices(2536, 600, 2533, 570, 1.0, 2529, 1.0);
            s.store_div_scaled_inputs_indices(2537, 597, 0.666666666666667, 2536, 1.0);
            s.store_square(2538, 2537);
            s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);
            s.store_add_scaled_value_products(2546, A::mul3(s.ad_value(597), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(597), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5);
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2669] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2669]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2669])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2670] = (s.v[2547] > 0.0);
        s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });

        s.b[2671] = (s.v[2546] > (-230.25850929940458));
        s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && s.b[2671]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && (!s.b[2671])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2672] = (s.v[526] == 0.0);
        s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });

        s.b[2673] = (s.v[506] == 0.5);
        s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2673]) {
            s.store_sqrt_mul_sub_lhs(2526, 503, 2524, 591);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2673])) {
            s.store_pow_mul_base_mixed_ai(2526, A::sub(s.ad_value(503), s.ad_value(2524)), 591, 506);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) {
            s.store_mul_div_scaled_product_mixed_iaii(2551, 573, A::sub(s.ad_value(503), s.ad_value(2524)), 588, 1.0, 2526, 1.0);
        }

        s.b[2674] = (((((-s.v[603]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2674]) {
            s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2675] = (((-s.v[603]) / s.v[2551]) < 0.0);
        s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && s.b[2675]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 603, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && (!s.b[2675])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 603, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2676] = (s.v[535] > 1000.0);
        s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });

        s.b[2677] = (s.v[2525] > ((-s.v[438]) * s.v[535]));
        s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });

        s.b[2678] = (s.v[538] == 4.0);
        s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && s.b[2678]) {
            s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(2525), s.ad_value(609)), 2525, 609);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && (!s.b[2678])) {
            s.store_pow_abs_mul_base_indices(2526, 2525, 609, 538);
        }

        s.b[2679] = (s.v[570] == 0.5);
        s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2679]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(567)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2679])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2518, 567, 570);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {
            s.store_add_scaled_product_mixed_aia(1910, A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2526)), p.p30, 582, A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);
        }

        s.b[2680] = (s.v[669] == 0.0);
        s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });

        if ((s.b[2555] && (!s.b[2556])) && s.b[2680]) {
            s.store_scalar(1911, 0.0);
        }

        s.b[2681] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_sub(2529, 565, 2523);
        }

        s.b[2683] = (s.v[507] == 0.5);
        s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && s.b[2683]) {
            s.store_sqrt_mul(2526, 2529, 592);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && (!s.b[2683])) {
            s.store_pow_mul_base_indices(2526, 2529, 592, 507);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_mul(2533, 586, 2526);
        }

        s.b[2684] = (s.v[521] == 0.0);
        s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {
            s.store_mul_div_scaled_product_indices(2536, 601, 2533, 571, 1.0, 2529, 1.0);
            s.store_div_scaled_inputs_indices(2537, 598, 0.666666666666667, 2536, 1.0);
            s.store_square(2538, 2537);
            s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);
            s.store_add_scaled_value_products(2546, A::mul3(s.ad_value(598), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(598), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5);
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2687] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2687]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2687])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2688] = (s.v[2547] > 0.0);
        s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });

        s.b[2689] = (s.v[2546] > (-230.25850929940458));
        s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && s.b[2689]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && (!s.b[2689])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2690] = (s.v[527] == 0.0);
        s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });

        s.b[2691] = (s.v[507] == 0.5);
        s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2691]) {
            s.store_sqrt_mul_sub_lhs(2526, 504, 2524, 592);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_pow_mul_base_mixed_ai(2526, A::sub(s.ad_value(504), s.ad_value(2524)), 592, 507);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) {
            s.store_mul_div_scaled_product_mixed_iaii(2551, 574, A::sub(s.ad_value(504), s.ad_value(2524)), 589, 1.0, 2526, 1.0);
        }

        s.b[2692] = (((((-s.v[604]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2692]) {
            s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2693] = (((-s.v[604]) / s.v[2551]) < 0.0);
        s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && s.b[2693]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 604, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && (!s.b[2693])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 604, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2694] = (s.v[635] > 1000.0);
        s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });

        s.b[2695] = (s.v[2525] > ((-s.v[438]) * s.v[635]));
        s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });

        s.b[2696] = (s.v[539] == 4.0);
        s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && s.b[2696]) {
            s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(2525), s.ad_value(610)), 2525, 610);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && (!s.b[2696])) {
            s.store_pow_abs_mul_base_indices(2526, 2525, 610, 539);
        }

        s.b[2697] = (s.v[629] == 1.0);
        s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            if (s.v[827] < s.v[544]) {
                if (((s.v[827] - s.v[544]) / s.v[545]) < (-37.0)) {
                    s.copy_ad(2553, 544);
                } else {
                    s.store_add_scaled_product_left_ad(2553, 544, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(827), 1.0, s.ad_value(544), (-1.0), s.ad_value(545), 1.0)), 545, 1.0);
                }
            } else {
                if (((s.v[827] - s.v[544]) / s.v[545]) > 37.0) {
                    s.copy_ad(2553, 827);
                } else {
                    s.store_add_scaled_product_left_ad(2553, 827, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(544), 1.0, s.ad_value(827), (-1.0), s.ad_value(545), 1.0)), 545, 1.0);
                }
            }
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product_indices(2513, 2553, 1.0, 678, 2512, 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2698] = (s.v[571] == 0.5);
        s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2698]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(568)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2698])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2554, 568, 571);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_add_scaled_product_mixed_aia(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, 583, A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);
            s.store_add_scaled_inputs3_indices(2553, 827, 1.0, 544, 1.0, 2553, -1.0);
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product_indices(2513, 2553, 1.0, 678, 2512, 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);
        }

        s.b[2699] = (s.v[624] == 0.5);
        s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2699]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(623)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2699])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2554, 623, 624);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_add_scaled_product_mixed_aia(466, A::mul_sub_from_scalar_rhs(s.ad_value(627), 1.0, s.ad_value(2526)), p.p30, 628, A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);
            s.store_add(1911, 1911, 466);
        }

        s.b[2700] = (s.v[571] == 0.5);
        s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && s.b[2700]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(568)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && (!s.b[2700])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2518, 568, 571);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) {
            s.store_add_scaled_product_mixed_aia(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, 583, A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);
        }

        s.store_add_scaled_inputs3_indices(844, 845, (-1.0), 846, (-1.0), 847, (-1.0));

        s.store_add(848, 848, 1898);

        s.store_add(849, 849, 1899);

        s.store_add_scaled_products3(851, s.ad_value(640), s.ad_value(1906), 1.0, s.ad_value(641), s.ad_value(1907), 1.0, s.ad_value(642), s.ad_value(1908), 1.0);

        s.store_add_scaled_products3(852, s.ad_value(667), s.ad_value(1909), 1.0, s.ad_value(668), s.ad_value(1910), 1.0, s.ad_value(669), s.ad_value(1911), 1.0);

        s.b[2710] = (s.v[825] < 0.0);
        s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });

        if s.b[2710] {
            s.copy_ad(2709, 847);
            s.copy_ad(847, 844);
            s.copy_ad(844, 2709);
        }

        s.store_mul(854, 1892, 1883);

        s.b[2743] = ((s.v[1817] > 0.0) && (s.v[710] > 0.0));
        s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });

        s.b[2748] = ((((p.p50 == 1.0) && (s.v[713] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.store_scalar(2748, if s.b[2748] { 1.0 } else { 0.0 });

        if (s.b[2743] && s.b[2748]) {
            s.store_div_scaled_product3_mixed_aiia(854, A::square(s.ad_value(1896)), 1892, 1883, 1.0, A::square(s.ad_value(1894)), 1.0);
        }

        s.b[2752] = (((p.p46 != 0.0) && (s.v[282] > 0.0)) && (s.v[1868] > 0.0));
        s.store_scalar(2752, if s.b[2752] { 1.0 } else { 0.0 });

        if s.b[2752] {
            s.store_div_scaled_inputs_indices(1920, 1871, 4.0, 718, 1.0);
            s.store_scale(1920, 765, s.v[709]);
            s.store_mul(1920, 1852, 1865);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        var_chnl_type: f64,
        var_guard1718: f64,
        var_i_ds: f64,
        var_i_ds_db0: f64,
        var_i_ds_db1: f64,
        var_i_ds_db2: f64,
        var_i_ds_db3: f64,
        var_i_ds_db4: f64,
        var_i_ds_db5: f64,
        var_i_ds_db6: f64,
        var_i_ds_dn0: f64,
        var_i_ds_dn1: f64,
        var_i_ds_dn10: f64,
        var_i_ds_dn11: f64,
        var_i_ds_dn2: f64,
        var_i_ds_dn3: f64,
        var_i_ds_dn4: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_ds_dn9: f64,
        var_i_dsedge: f64,
        var_i_dsedge_db0: f64,
        var_i_dsedge_db1: f64,
        var_i_dsedge_db2: f64,
        var_i_dsedge_db3: f64,
        var_i_dsedge_db4: f64,
        var_i_dsedge_db5: f64,
        var_i_dsedge_db6: f64,
        var_i_dsedge_dn0: f64,
        var_i_dsedge_dn1: f64,
        var_i_dsedge_dn10: f64,
        var_i_dsedge_dn11: f64,
        var_i_dsedge_dn2: f64,
        var_i_dsedge_dn3: f64,
        var_i_dsedge_dn4: f64,
        var_i_dsedge_dn5: f64,
        var_i_dsedge_dn6: f64,
        var_i_dsedge_dn7: f64,
        var_i_dsedge_dn8: f64,
        var_i_dsedge_dn9: f64,
        var_i_gb: f64,
        var_i_gb_db0: f64,
        var_i_gb_db1: f64,
        var_i_gb_db2: f64,
        var_i_gb_db3: f64,
        var_i_gb_db4: f64,
        var_i_gb_db5: f64,
        var_i_gb_db6: f64,
        var_i_gb_dn0: f64,
        var_i_gb_dn1: f64,
        var_i_gb_dn10: f64,
        var_i_gb_dn11: f64,
        var_i_gb_dn2: f64,
        var_i_gb_dn3: f64,
        var_i_gb_dn4: f64,
        var_i_gb_dn5: f64,
        var_i_gb_dn6: f64,
        var_i_gb_dn7: f64,
        var_i_gb_dn8: f64,
        var_i_gb_dn9: f64,
        var_i_gcd: f64,
        var_i_gcd_db0: f64,
        var_i_gcd_db1: f64,
        var_i_gcd_db2: f64,
        var_i_gcd_db3: f64,
        var_i_gcd_db4: f64,
        var_i_gcd_db5: f64,
        var_i_gcd_db6: f64,
        var_i_gcd_dn0: f64,
        var_i_gcd_dn1: f64,
        var_i_gcd_dn10: f64,
        var_i_gcd_dn11: f64,
        var_i_gcd_dn2: f64,
        var_i_gcd_dn3: f64,
        var_i_gcd_dn4: f64,
        var_i_gcd_dn5: f64,
        var_i_gcd_dn6: f64,
        var_i_gcd_dn7: f64,
        var_i_gcd_dn8: f64,
        var_i_gcd_dn9: f64,
        var_i_gcs: f64,
        var_i_gcs_db0: f64,
        var_i_gcs_db1: f64,
        var_i_gcs_db2: f64,
        var_i_gcs_db3: f64,
        var_i_gcs_db4: f64,
        var_i_gcs_db5: f64,
        var_i_gcs_db6: f64,
        var_i_gcs_dn0: f64,
        var_i_gcs_dn1: f64,
        var_i_gcs_dn10: f64,
        var_i_gcs_dn11: f64,
        var_i_gcs_dn2: f64,
        var_i_gcs_dn3: f64,
        var_i_gcs_dn4: f64,
        var_i_gcs_dn5: f64,
        var_i_gcs_dn6: f64,
        var_i_gcs_dn7: f64,
        var_i_gcs_dn8: f64,
        var_i_gcs_dn9: f64,
        var_i_gisl: f64,
        var_i_gisl_db0: f64,
        var_i_gisl_db1: f64,
        var_i_gisl_db2: f64,
        var_i_gisl_db3: f64,
        var_i_gisl_db4: f64,
        var_i_gisl_db5: f64,
        var_i_gisl_db6: f64,
        var_i_gisl_dn0: f64,
        var_i_gisl_dn1: f64,
        var_i_gisl_dn10: f64,
        var_i_gisl_dn11: f64,
        var_i_gisl_dn2: f64,
        var_i_gisl_dn3: f64,
        var_i_gisl_dn4: f64,
        var_i_gisl_dn5: f64,
        var_i_gisl_dn6: f64,
        var_i_gisl_dn7: f64,
        var_i_gisl_dn8: f64,
        var_i_gisl_dn9: f64,
        var_igdov: f64,
        var_igdov_db0: f64,
        var_igdov_db1: f64,
        var_igdov_db2: f64,
        var_igdov_db3: f64,
        var_igdov_db4: f64,
        var_igdov_db5: f64,
        var_igdov_db6: f64,
        var_igdov_dn0: f64,
        var_igdov_dn1: f64,
        var_igdov_dn10: f64,
        var_igdov_dn11: f64,
        var_igdov_dn2: f64,
        var_igdov_dn3: f64,
        var_igdov_dn4: f64,
        var_igdov_dn5: f64,
        var_igdov_dn6: f64,
        var_igdov_dn7: f64,
        var_igdov_dn8: f64,
        var_igdov_dn9: f64,
        var_igsov: f64,
        var_igsov_db0: f64,
        var_igsov_db1: f64,
        var_igsov_db2: f64,
        var_igsov_db3: f64,
        var_igsov_db4: f64,
        var_igsov_db5: f64,
        var_igsov_db6: f64,
        var_igsov_dn0: f64,
        var_igsov_dn1: f64,
        var_igsov_dn10: f64,
        var_igsov_dn11: f64,
        var_igsov_dn2: f64,
        var_igsov_dn3: f64,
        var_igsov_dn4: f64,
        var_igsov_dn5: f64,
        var_igsov_dn6: f64,
        var_igsov_dn7: f64,
        var_igsov_dn8: f64,
        var_igsov_dn9: f64,
        var_iimpact: f64,
        var_iimpact_db0: f64,
        var_iimpact_db1: f64,
        var_iimpact_db2: f64,
        var_iimpact_db3: f64,
        var_iimpact_db4: f64,
        var_iimpact_db5: f64,
        var_iimpact_db6: f64,
        var_iimpact_dn0: f64,
        var_iimpact_dn1: f64,
        var_iimpact_dn10: f64,
        var_iimpact_dn11: f64,
        var_iimpact_dn2: f64,
        var_iimpact_dn3: f64,
        var_iimpact_dn4: f64,
        var_iimpact_dn5: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_iimpact_dn9: f64,
        var_mult_inst: f64,
    ) {
        let (eq0_e948, eq0_e948_d_n0, eq0_e948_d_n1, eq0_e948_d_n2, eq0_e948_d_n3, eq0_e948_d_n4, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8, eq0_e948_d_n9, eq0_e948_d_n10, eq0_e948_d_n11, eq0_e948_d_b0, eq0_e948_d_b1, eq0_e948_d_b2, eq0_e948_d_b3, eq0_e948_d_b4, eq0_e948_d_b5, eq0_e948_d_b6,) = {
    if (var_guard1718 != 0.0) {
        let eq0_e942: f64 = (var_chnl_type * var_mult_inst);
        let eq0_e944: f64 = (eq0_e942 * p.p32);
        let eq0_e946: f64 = (eq0_e944 * var_iimpact);
        let eq0_e946_d_n0: f64 = (eq0_e944 * var_iimpact_dn0);
        let eq0_e946_d_n1: f64 = (eq0_e944 * var_iimpact_dn1);
        let eq0_e946_d_n2: f64 = (eq0_e944 * var_iimpact_dn2);
        let eq0_e946_d_n3: f64 = (eq0_e944 * var_iimpact_dn3);
        let eq0_e946_d_n4: f64 = (eq0_e944 * var_iimpact_dn4);
        let eq0_e946_d_n5: f64 = (eq0_e944 * var_iimpact_dn5);
        let eq0_e946_d_n6: f64 = (eq0_e944 * var_iimpact_dn6);
        let eq0_e946_d_n7: f64 = (eq0_e944 * var_iimpact_dn7);
        let eq0_e946_d_n8: f64 = (eq0_e944 * var_iimpact_dn8);
        let eq0_e946_d_n9: f64 = (eq0_e944 * var_iimpact_dn9);
        let eq0_e946_d_n10: f64 = (eq0_e944 * var_iimpact_dn10);
        let eq0_e946_d_n11: f64 = (eq0_e944 * var_iimpact_dn11);
        let eq0_e946_d_b0: f64 = (eq0_e944 * var_iimpact_db0);
        let eq0_e946_d_b1: f64 = (eq0_e944 * var_iimpact_db1);
        let eq0_e946_d_b2: f64 = (eq0_e944 * var_iimpact_db2);
        let eq0_e946_d_b3: f64 = (eq0_e944 * var_iimpact_db3);
        let eq0_e946_d_b4: f64 = (eq0_e944 * var_iimpact_db4);
        let eq0_e946_d_b5: f64 = (eq0_e944 * var_iimpact_db5);
        let eq0_e946_d_b6: f64 = (eq0_e944 * var_iimpact_db6);
        (eq0_e946, eq0_e946_d_n0, eq0_e946_d_n1, eq0_e946_d_n2, eq0_e946_d_n3, eq0_e946_d_n4, eq0_e946_d_n5, eq0_e946_d_n6, eq0_e946_d_n7, eq0_e946_d_n8, eq0_e946_d_n9, eq0_e946_d_n10, eq0_e946_d_n11, eq0_e946_d_b0, eq0_e946_d_b1, eq0_e946_d_b2, eq0_e946_d_b3, eq0_e946_d_b4, eq0_e946_d_b5, eq0_e946_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e948;
        let eq0_node_derivatives: [f64; 12] = [eq0_e948_d_n0, eq0_e948_d_n1, eq0_e948_d_n2, eq0_e948_d_n3, eq0_e948_d_n4, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8, eq0_e948_d_n9, eq0_e948_d_n10, eq0_e948_d_n11];
        let eq0_branch_derivatives: [f64; 7] = [eq0_e948_d_b0, eq0_e948_d_b1, eq0_e948_d_b2, eq0_e948_d_b3, eq0_e948_d_b4, eq0_e948_d_b5, eq0_e948_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e960, eq1_e960_d_n0, eq1_e960_d_n1, eq1_e960_d_n2, eq1_e960_d_n3, eq1_e960_d_n4, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8, eq1_e960_d_n9, eq1_e960_d_n10, eq1_e960_d_n11, eq1_e960_d_b0, eq1_e960_d_b1, eq1_e960_d_b2, eq1_e960_d_b3, eq1_e960_d_b4, eq1_e960_d_b5, eq1_e960_d_b6,) = {
    if (var_guard1718 != 0.0) {
        let eq1_e952: f64 = (var_chnl_type * var_mult_inst);
        let eq1_e954: f64 = (eq1_e952 * p.p32);
        let eq1_e957: f64 = (var_i_ds + var_i_dsedge);
        let eq1_e957_d_n0: f64 = (var_i_ds_dn0 + var_i_dsedge_dn0);
        let eq1_e957_d_n1: f64 = (var_i_ds_dn1 + var_i_dsedge_dn1);
        let eq1_e957_d_n2: f64 = (var_i_ds_dn2 + var_i_dsedge_dn2);
        let eq1_e957_d_n3: f64 = (var_i_ds_dn3 + var_i_dsedge_dn3);
        let eq1_e957_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq1_e957_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq1_e957_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq1_e957_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq1_e957_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq1_e957_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq1_e957_d_n10: f64 = (var_i_ds_dn10 + var_i_dsedge_dn10);
        let eq1_e957_d_n11: f64 = (var_i_ds_dn11 + var_i_dsedge_dn11);
        let eq1_e957_d_b0: f64 = (var_i_ds_db0 + var_i_dsedge_db0);
        let eq1_e957_d_b1: f64 = (var_i_ds_db1 + var_i_dsedge_db1);
        let eq1_e957_d_b2: f64 = (var_i_ds_db2 + var_i_dsedge_db2);
        let eq1_e957_d_b3: f64 = (var_i_ds_db3 + var_i_dsedge_db3);
        let eq1_e957_d_b4: f64 = (var_i_ds_db4 + var_i_dsedge_db4);
        let eq1_e957_d_b5: f64 = (var_i_ds_db5 + var_i_dsedge_db5);
        let eq1_e957_d_b6: f64 = (var_i_ds_db6 + var_i_dsedge_db6);
        let eq1_e958: f64 = (eq1_e954 * eq1_e957);
        let eq1_e958_d_n0: f64 = (eq1_e954 * eq1_e957_d_n0);
        let eq1_e958_d_n1: f64 = (eq1_e954 * eq1_e957_d_n1);
        let eq1_e958_d_n2: f64 = (eq1_e954 * eq1_e957_d_n2);
        let eq1_e958_d_n3: f64 = (eq1_e954 * eq1_e957_d_n3);
        let eq1_e958_d_n4: f64 = (eq1_e954 * eq1_e957_d_n4);
        let eq1_e958_d_n5: f64 = (eq1_e954 * eq1_e957_d_n5);
        let eq1_e958_d_n6: f64 = (eq1_e954 * eq1_e957_d_n6);
        let eq1_e958_d_n7: f64 = (eq1_e954 * eq1_e957_d_n7);
        let eq1_e958_d_n8: f64 = (eq1_e954 * eq1_e957_d_n8);
        let eq1_e958_d_n9: f64 = (eq1_e954 * eq1_e957_d_n9);
        let eq1_e958_d_n10: f64 = (eq1_e954 * eq1_e957_d_n10);
        let eq1_e958_d_n11: f64 = (eq1_e954 * eq1_e957_d_n11);
        let eq1_e958_d_b0: f64 = (eq1_e954 * eq1_e957_d_b0);
        let eq1_e958_d_b1: f64 = (eq1_e954 * eq1_e957_d_b1);
        let eq1_e958_d_b2: f64 = (eq1_e954 * eq1_e957_d_b2);
        let eq1_e958_d_b3: f64 = (eq1_e954 * eq1_e957_d_b3);
        let eq1_e958_d_b4: f64 = (eq1_e954 * eq1_e957_d_b4);
        let eq1_e958_d_b5: f64 = (eq1_e954 * eq1_e957_d_b5);
        let eq1_e958_d_b6: f64 = (eq1_e954 * eq1_e957_d_b6);
        (eq1_e958, eq1_e958_d_n0, eq1_e958_d_n1, eq1_e958_d_n2, eq1_e958_d_n3, eq1_e958_d_n4, eq1_e958_d_n5, eq1_e958_d_n6, eq1_e958_d_n7, eq1_e958_d_n8, eq1_e958_d_n9, eq1_e958_d_n10, eq1_e958_d_n11, eq1_e958_d_b0, eq1_e958_d_b1, eq1_e958_d_b2, eq1_e958_d_b3, eq1_e958_d_b4, eq1_e958_d_b5, eq1_e958_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e960;
        let eq1_node_derivatives: [f64; 12] = [eq1_e960_d_n0, eq1_e960_d_n1, eq1_e960_d_n2, eq1_e960_d_n3, eq1_e960_d_n4, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8, eq1_e960_d_n9, eq1_e960_d_n10, eq1_e960_d_n11];
        let eq1_branch_derivatives: [f64; 7] = [eq1_e960_d_b0, eq1_e960_d_b1, eq1_e960_d_b2, eq1_e960_d_b3, eq1_e960_d_b4, eq1_e960_d_b5, eq1_e960_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e970, eq2_e970_d_n0, eq2_e970_d_n1, eq2_e970_d_n2, eq2_e970_d_n3, eq2_e970_d_n4, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8, eq2_e970_d_n9, eq2_e970_d_n10, eq2_e970_d_n11, eq2_e970_d_b0, eq2_e970_d_b1, eq2_e970_d_b2, eq2_e970_d_b3, eq2_e970_d_b4, eq2_e970_d_b5, eq2_e970_d_b6,) = {
    if (var_guard1718 != 0.0) {
        let eq2_e964: f64 = (var_chnl_type * var_mult_inst);
        let eq2_e966: f64 = (eq2_e964 * p.p32);
        let eq2_e968: f64 = (eq2_e966 * var_i_gcs);
        let eq2_e968_d_n0: f64 = (eq2_e966 * var_i_gcs_dn0);
        let eq2_e968_d_n1: f64 = (eq2_e966 * var_i_gcs_dn1);
        let eq2_e968_d_n2: f64 = (eq2_e966 * var_i_gcs_dn2);
        let eq2_e968_d_n3: f64 = (eq2_e966 * var_i_gcs_dn3);
        let eq2_e968_d_n4: f64 = (eq2_e966 * var_i_gcs_dn4);
        let eq2_e968_d_n5: f64 = (eq2_e966 * var_i_gcs_dn5);
        let eq2_e968_d_n6: f64 = (eq2_e966 * var_i_gcs_dn6);
        let eq2_e968_d_n7: f64 = (eq2_e966 * var_i_gcs_dn7);
        let eq2_e968_d_n8: f64 = (eq2_e966 * var_i_gcs_dn8);
        let eq2_e968_d_n9: f64 = (eq2_e966 * var_i_gcs_dn9);
        let eq2_e968_d_n10: f64 = (eq2_e966 * var_i_gcs_dn10);
        let eq2_e968_d_n11: f64 = (eq2_e966 * var_i_gcs_dn11);
        let eq2_e968_d_b0: f64 = (eq2_e966 * var_i_gcs_db0);
        let eq2_e968_d_b1: f64 = (eq2_e966 * var_i_gcs_db1);
        let eq2_e968_d_b2: f64 = (eq2_e966 * var_i_gcs_db2);
        let eq2_e968_d_b3: f64 = (eq2_e966 * var_i_gcs_db3);
        let eq2_e968_d_b4: f64 = (eq2_e966 * var_i_gcs_db4);
        let eq2_e968_d_b5: f64 = (eq2_e966 * var_i_gcs_db5);
        let eq2_e968_d_b6: f64 = (eq2_e966 * var_i_gcs_db6);
        (eq2_e968, eq2_e968_d_n0, eq2_e968_d_n1, eq2_e968_d_n2, eq2_e968_d_n3, eq2_e968_d_n4, eq2_e968_d_n5, eq2_e968_d_n6, eq2_e968_d_n7, eq2_e968_d_n8, eq2_e968_d_n9, eq2_e968_d_n10, eq2_e968_d_n11, eq2_e968_d_b0, eq2_e968_d_b1, eq2_e968_d_b2, eq2_e968_d_b3, eq2_e968_d_b4, eq2_e968_d_b5, eq2_e968_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e970;
        let eq2_node_derivatives: [f64; 12] = [eq2_e970_d_n0, eq2_e970_d_n1, eq2_e970_d_n2, eq2_e970_d_n3, eq2_e970_d_n4, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8, eq2_e970_d_n9, eq2_e970_d_n10, eq2_e970_d_n11];
        let eq2_branch_derivatives: [f64; 7] = [eq2_e970_d_b0, eq2_e970_d_b1, eq2_e970_d_b2, eq2_e970_d_b3, eq2_e970_d_b4, eq2_e970_d_b5, eq2_e970_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e980, eq3_e980_d_n0, eq3_e980_d_n1, eq3_e980_d_n2, eq3_e980_d_n3, eq3_e980_d_n4, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8, eq3_e980_d_n9, eq3_e980_d_n10, eq3_e980_d_n11, eq3_e980_d_b0, eq3_e980_d_b1, eq3_e980_d_b2, eq3_e980_d_b3, eq3_e980_d_b4, eq3_e980_d_b5, eq3_e980_d_b6,) = {
    if (var_guard1718 != 0.0) {
        let eq3_e974: f64 = (var_chnl_type * var_mult_inst);
        let eq3_e976: f64 = (eq3_e974 * p.p32);
        let eq3_e978: f64 = (eq3_e976 * var_i_gcd);
        let eq3_e978_d_n0: f64 = (eq3_e976 * var_i_gcd_dn0);
        let eq3_e978_d_n1: f64 = (eq3_e976 * var_i_gcd_dn1);
        let eq3_e978_d_n2: f64 = (eq3_e976 * var_i_gcd_dn2);
        let eq3_e978_d_n3: f64 = (eq3_e976 * var_i_gcd_dn3);
        let eq3_e978_d_n4: f64 = (eq3_e976 * var_i_gcd_dn4);
        let eq3_e978_d_n5: f64 = (eq3_e976 * var_i_gcd_dn5);
        let eq3_e978_d_n6: f64 = (eq3_e976 * var_i_gcd_dn6);
        let eq3_e978_d_n7: f64 = (eq3_e976 * var_i_gcd_dn7);
        let eq3_e978_d_n8: f64 = (eq3_e976 * var_i_gcd_dn8);
        let eq3_e978_d_n9: f64 = (eq3_e976 * var_i_gcd_dn9);
        let eq3_e978_d_n10: f64 = (eq3_e976 * var_i_gcd_dn10);
        let eq3_e978_d_n11: f64 = (eq3_e976 * var_i_gcd_dn11);
        let eq3_e978_d_b0: f64 = (eq3_e976 * var_i_gcd_db0);
        let eq3_e978_d_b1: f64 = (eq3_e976 * var_i_gcd_db1);
        let eq3_e978_d_b2: f64 = (eq3_e976 * var_i_gcd_db2);
        let eq3_e978_d_b3: f64 = (eq3_e976 * var_i_gcd_db3);
        let eq3_e978_d_b4: f64 = (eq3_e976 * var_i_gcd_db4);
        let eq3_e978_d_b5: f64 = (eq3_e976 * var_i_gcd_db5);
        let eq3_e978_d_b6: f64 = (eq3_e976 * var_i_gcd_db6);
        (eq3_e978, eq3_e978_d_n0, eq3_e978_d_n1, eq3_e978_d_n2, eq3_e978_d_n3, eq3_e978_d_n4, eq3_e978_d_n5, eq3_e978_d_n6, eq3_e978_d_n7, eq3_e978_d_n8, eq3_e978_d_n9, eq3_e978_d_n10, eq3_e978_d_n11, eq3_e978_d_b0, eq3_e978_d_b1, eq3_e978_d_b2, eq3_e978_d_b3, eq3_e978_d_b4, eq3_e978_d_b5, eq3_e978_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e980;
        let eq3_node_derivatives: [f64; 12] = [eq3_e980_d_n0, eq3_e980_d_n1, eq3_e980_d_n2, eq3_e980_d_n3, eq3_e980_d_n4, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8, eq3_e980_d_n9, eq3_e980_d_n10, eq3_e980_d_n11];
        let eq3_branch_derivatives: [f64; 7] = [eq3_e980_d_b0, eq3_e980_d_b1, eq3_e980_d_b2, eq3_e980_d_b3, eq3_e980_d_b4, eq3_e980_d_b5, eq3_e980_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e991, eq4_e991_d_n0, eq4_e991_d_n1, eq4_e991_d_n2, eq4_e991_d_n3, eq4_e991_d_n4, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8, eq4_e991_d_n9, eq4_e991_d_n10, eq4_e991_d_n11, eq4_e991_d_b0, eq4_e991_d_b1, eq4_e991_d_b2, eq4_e991_d_b3, eq4_e991_d_b4, eq4_e991_d_b5, eq4_e991_d_b6,) = {
    if (var_guard1718 == 0.0) {
        let eq4_e985: f64 = (var_chnl_type * var_mult_inst);
        let eq4_e987: f64 = (eq4_e985 * p.p32);
        let eq4_e989: f64 = (eq4_e987 * var_iimpact);
        let eq4_e989_d_n0: f64 = (eq4_e987 * var_iimpact_dn0);
        let eq4_e989_d_n1: f64 = (eq4_e987 * var_iimpact_dn1);
        let eq4_e989_d_n2: f64 = (eq4_e987 * var_iimpact_dn2);
        let eq4_e989_d_n3: f64 = (eq4_e987 * var_iimpact_dn3);
        let eq4_e989_d_n4: f64 = (eq4_e987 * var_iimpact_dn4);
        let eq4_e989_d_n5: f64 = (eq4_e987 * var_iimpact_dn5);
        let eq4_e989_d_n6: f64 = (eq4_e987 * var_iimpact_dn6);
        let eq4_e989_d_n7: f64 = (eq4_e987 * var_iimpact_dn7);
        let eq4_e989_d_n8: f64 = (eq4_e987 * var_iimpact_dn8);
        let eq4_e989_d_n9: f64 = (eq4_e987 * var_iimpact_dn9);
        let eq4_e989_d_n10: f64 = (eq4_e987 * var_iimpact_dn10);
        let eq4_e989_d_n11: f64 = (eq4_e987 * var_iimpact_dn11);
        let eq4_e989_d_b0: f64 = (eq4_e987 * var_iimpact_db0);
        let eq4_e989_d_b1: f64 = (eq4_e987 * var_iimpact_db1);
        let eq4_e989_d_b2: f64 = (eq4_e987 * var_iimpact_db2);
        let eq4_e989_d_b3: f64 = (eq4_e987 * var_iimpact_db3);
        let eq4_e989_d_b4: f64 = (eq4_e987 * var_iimpact_db4);
        let eq4_e989_d_b5: f64 = (eq4_e987 * var_iimpact_db5);
        let eq4_e989_d_b6: f64 = (eq4_e987 * var_iimpact_db6);
        (eq4_e989, eq4_e989_d_n0, eq4_e989_d_n1, eq4_e989_d_n2, eq4_e989_d_n3, eq4_e989_d_n4, eq4_e989_d_n5, eq4_e989_d_n6, eq4_e989_d_n7, eq4_e989_d_n8, eq4_e989_d_n9, eq4_e989_d_n10, eq4_e989_d_n11, eq4_e989_d_b0, eq4_e989_d_b1, eq4_e989_d_b2, eq4_e989_d_b3, eq4_e989_d_b4, eq4_e989_d_b5, eq4_e989_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e991;
        let eq4_node_derivatives: [f64; 12] = [eq4_e991_d_n0, eq4_e991_d_n1, eq4_e991_d_n2, eq4_e991_d_n3, eq4_e991_d_n4, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8, eq4_e991_d_n9, eq4_e991_d_n10, eq4_e991_d_n11];
        let eq4_branch_derivatives: [f64; 7] = [eq4_e991_d_b0, eq4_e991_d_b1, eq4_e991_d_b2, eq4_e991_d_b3, eq4_e991_d_b4, eq4_e991_d_b5, eq4_e991_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1004, eq5_e1004_d_n0, eq5_e1004_d_n1, eq5_e1004_d_n2, eq5_e1004_d_n3, eq5_e1004_d_n4, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8, eq5_e1004_d_n9, eq5_e1004_d_n10, eq5_e1004_d_n11, eq5_e1004_d_b0, eq5_e1004_d_b1, eq5_e1004_d_b2, eq5_e1004_d_b3, eq5_e1004_d_b4, eq5_e1004_d_b5, eq5_e1004_d_b6,) = {
    if (var_guard1718 == 0.0) {
        let eq5_e996: f64 = (var_chnl_type * var_mult_inst);
        let eq5_e998: f64 = (eq5_e996 * p.p32);
        let eq5_e1001: f64 = (var_i_ds + var_i_dsedge);
        let eq5_e1001_d_n0: f64 = (var_i_ds_dn0 + var_i_dsedge_dn0);
        let eq5_e1001_d_n1: f64 = (var_i_ds_dn1 + var_i_dsedge_dn1);
        let eq5_e1001_d_n2: f64 = (var_i_ds_dn2 + var_i_dsedge_dn2);
        let eq5_e1001_d_n3: f64 = (var_i_ds_dn3 + var_i_dsedge_dn3);
        let eq5_e1001_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq5_e1001_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq5_e1001_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq5_e1001_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq5_e1001_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq5_e1001_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq5_e1001_d_n10: f64 = (var_i_ds_dn10 + var_i_dsedge_dn10);
        let eq5_e1001_d_n11: f64 = (var_i_ds_dn11 + var_i_dsedge_dn11);
        let eq5_e1001_d_b0: f64 = (var_i_ds_db0 + var_i_dsedge_db0);
        let eq5_e1001_d_b1: f64 = (var_i_ds_db1 + var_i_dsedge_db1);
        let eq5_e1001_d_b2: f64 = (var_i_ds_db2 + var_i_dsedge_db2);
        let eq5_e1001_d_b3: f64 = (var_i_ds_db3 + var_i_dsedge_db3);
        let eq5_e1001_d_b4: f64 = (var_i_ds_db4 + var_i_dsedge_db4);
        let eq5_e1001_d_b5: f64 = (var_i_ds_db5 + var_i_dsedge_db5);
        let eq5_e1001_d_b6: f64 = (var_i_ds_db6 + var_i_dsedge_db6);
        let eq5_e1002: f64 = (eq5_e998 * eq5_e1001);
        let eq5_e1002_d_n0: f64 = (eq5_e998 * eq5_e1001_d_n0);
        let eq5_e1002_d_n1: f64 = (eq5_e998 * eq5_e1001_d_n1);
        let eq5_e1002_d_n2: f64 = (eq5_e998 * eq5_e1001_d_n2);
        let eq5_e1002_d_n3: f64 = (eq5_e998 * eq5_e1001_d_n3);
        let eq5_e1002_d_n4: f64 = (eq5_e998 * eq5_e1001_d_n4);
        let eq5_e1002_d_n5: f64 = (eq5_e998 * eq5_e1001_d_n5);
        let eq5_e1002_d_n6: f64 = (eq5_e998 * eq5_e1001_d_n6);
        let eq5_e1002_d_n7: f64 = (eq5_e998 * eq5_e1001_d_n7);
        let eq5_e1002_d_n8: f64 = (eq5_e998 * eq5_e1001_d_n8);
        let eq5_e1002_d_n9: f64 = (eq5_e998 * eq5_e1001_d_n9);
        let eq5_e1002_d_n10: f64 = (eq5_e998 * eq5_e1001_d_n10);
        let eq5_e1002_d_n11: f64 = (eq5_e998 * eq5_e1001_d_n11);
        let eq5_e1002_d_b0: f64 = (eq5_e998 * eq5_e1001_d_b0);
        let eq5_e1002_d_b1: f64 = (eq5_e998 * eq5_e1001_d_b1);
        let eq5_e1002_d_b2: f64 = (eq5_e998 * eq5_e1001_d_b2);
        let eq5_e1002_d_b3: f64 = (eq5_e998 * eq5_e1001_d_b3);
        let eq5_e1002_d_b4: f64 = (eq5_e998 * eq5_e1001_d_b4);
        let eq5_e1002_d_b5: f64 = (eq5_e998 * eq5_e1001_d_b5);
        let eq5_e1002_d_b6: f64 = (eq5_e998 * eq5_e1001_d_b6);
        (eq5_e1002, eq5_e1002_d_n0, eq5_e1002_d_n1, eq5_e1002_d_n2, eq5_e1002_d_n3, eq5_e1002_d_n4, eq5_e1002_d_n5, eq5_e1002_d_n6, eq5_e1002_d_n7, eq5_e1002_d_n8, eq5_e1002_d_n9, eq5_e1002_d_n10, eq5_e1002_d_n11, eq5_e1002_d_b0, eq5_e1002_d_b1, eq5_e1002_d_b2, eq5_e1002_d_b3, eq5_e1002_d_b4, eq5_e1002_d_b5, eq5_e1002_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1004;
        let eq5_node_derivatives: [f64; 12] = [eq5_e1004_d_n0, eq5_e1004_d_n1, eq5_e1004_d_n2, eq5_e1004_d_n3, eq5_e1004_d_n4, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8, eq5_e1004_d_n9, eq5_e1004_d_n10, eq5_e1004_d_n11];
        let eq5_branch_derivatives: [f64; 7] = [eq5_e1004_d_b0, eq5_e1004_d_b1, eq5_e1004_d_b2, eq5_e1004_d_b3, eq5_e1004_d_b4, eq5_e1004_d_b5, eq5_e1004_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1015, eq6_e1015_d_n0, eq6_e1015_d_n1, eq6_e1015_d_n2, eq6_e1015_d_n3, eq6_e1015_d_n4, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8, eq6_e1015_d_n9, eq6_e1015_d_n10, eq6_e1015_d_n11, eq6_e1015_d_b0, eq6_e1015_d_b1, eq6_e1015_d_b2, eq6_e1015_d_b3, eq6_e1015_d_b4, eq6_e1015_d_b5, eq6_e1015_d_b6,) = {
    if (var_guard1718 == 0.0) {
        let eq6_e1009: f64 = (var_chnl_type * var_mult_inst);
        let eq6_e1011: f64 = (eq6_e1009 * p.p32);
        let eq6_e1013: f64 = (eq6_e1011 * var_i_gcs);
        let eq6_e1013_d_n0: f64 = (eq6_e1011 * var_i_gcs_dn0);
        let eq6_e1013_d_n1: f64 = (eq6_e1011 * var_i_gcs_dn1);
        let eq6_e1013_d_n2: f64 = (eq6_e1011 * var_i_gcs_dn2);
        let eq6_e1013_d_n3: f64 = (eq6_e1011 * var_i_gcs_dn3);
        let eq6_e1013_d_n4: f64 = (eq6_e1011 * var_i_gcs_dn4);
        let eq6_e1013_d_n5: f64 = (eq6_e1011 * var_i_gcs_dn5);
        let eq6_e1013_d_n6: f64 = (eq6_e1011 * var_i_gcs_dn6);
        let eq6_e1013_d_n7: f64 = (eq6_e1011 * var_i_gcs_dn7);
        let eq6_e1013_d_n8: f64 = (eq6_e1011 * var_i_gcs_dn8);
        let eq6_e1013_d_n9: f64 = (eq6_e1011 * var_i_gcs_dn9);
        let eq6_e1013_d_n10: f64 = (eq6_e1011 * var_i_gcs_dn10);
        let eq6_e1013_d_n11: f64 = (eq6_e1011 * var_i_gcs_dn11);
        let eq6_e1013_d_b0: f64 = (eq6_e1011 * var_i_gcs_db0);
        let eq6_e1013_d_b1: f64 = (eq6_e1011 * var_i_gcs_db1);
        let eq6_e1013_d_b2: f64 = (eq6_e1011 * var_i_gcs_db2);
        let eq6_e1013_d_b3: f64 = (eq6_e1011 * var_i_gcs_db3);
        let eq6_e1013_d_b4: f64 = (eq6_e1011 * var_i_gcs_db4);
        let eq6_e1013_d_b5: f64 = (eq6_e1011 * var_i_gcs_db5);
        let eq6_e1013_d_b6: f64 = (eq6_e1011 * var_i_gcs_db6);
        (eq6_e1013, eq6_e1013_d_n0, eq6_e1013_d_n1, eq6_e1013_d_n2, eq6_e1013_d_n3, eq6_e1013_d_n4, eq6_e1013_d_n5, eq6_e1013_d_n6, eq6_e1013_d_n7, eq6_e1013_d_n8, eq6_e1013_d_n9, eq6_e1013_d_n10, eq6_e1013_d_n11, eq6_e1013_d_b0, eq6_e1013_d_b1, eq6_e1013_d_b2, eq6_e1013_d_b3, eq6_e1013_d_b4, eq6_e1013_d_b5, eq6_e1013_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1015;
        let eq6_node_derivatives: [f64; 12] = [eq6_e1015_d_n0, eq6_e1015_d_n1, eq6_e1015_d_n2, eq6_e1015_d_n3, eq6_e1015_d_n4, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8, eq6_e1015_d_n9, eq6_e1015_d_n10, eq6_e1015_d_n11];
        let eq6_branch_derivatives: [f64; 7] = [eq6_e1015_d_b0, eq6_e1015_d_b1, eq6_e1015_d_b2, eq6_e1015_d_b3, eq6_e1015_d_b4, eq6_e1015_d_b5, eq6_e1015_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1026, eq7_e1026_d_n0, eq7_e1026_d_n1, eq7_e1026_d_n2, eq7_e1026_d_n3, eq7_e1026_d_n4, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8, eq7_e1026_d_n9, eq7_e1026_d_n10, eq7_e1026_d_n11, eq7_e1026_d_b0, eq7_e1026_d_b1, eq7_e1026_d_b2, eq7_e1026_d_b3, eq7_e1026_d_b4, eq7_e1026_d_b5, eq7_e1026_d_b6,) = {
    if (var_guard1718 == 0.0) {
        let eq7_e1020: f64 = (var_chnl_type * var_mult_inst);
        let eq7_e1022: f64 = (eq7_e1020 * p.p32);
        let eq7_e1024: f64 = (eq7_e1022 * var_i_gcd);
        let eq7_e1024_d_n0: f64 = (eq7_e1022 * var_i_gcd_dn0);
        let eq7_e1024_d_n1: f64 = (eq7_e1022 * var_i_gcd_dn1);
        let eq7_e1024_d_n2: f64 = (eq7_e1022 * var_i_gcd_dn2);
        let eq7_e1024_d_n3: f64 = (eq7_e1022 * var_i_gcd_dn3);
        let eq7_e1024_d_n4: f64 = (eq7_e1022 * var_i_gcd_dn4);
        let eq7_e1024_d_n5: f64 = (eq7_e1022 * var_i_gcd_dn5);
        let eq7_e1024_d_n6: f64 = (eq7_e1022 * var_i_gcd_dn6);
        let eq7_e1024_d_n7: f64 = (eq7_e1022 * var_i_gcd_dn7);
        let eq7_e1024_d_n8: f64 = (eq7_e1022 * var_i_gcd_dn8);
        let eq7_e1024_d_n9: f64 = (eq7_e1022 * var_i_gcd_dn9);
        let eq7_e1024_d_n10: f64 = (eq7_e1022 * var_i_gcd_dn10);
        let eq7_e1024_d_n11: f64 = (eq7_e1022 * var_i_gcd_dn11);
        let eq7_e1024_d_b0: f64 = (eq7_e1022 * var_i_gcd_db0);
        let eq7_e1024_d_b1: f64 = (eq7_e1022 * var_i_gcd_db1);
        let eq7_e1024_d_b2: f64 = (eq7_e1022 * var_i_gcd_db2);
        let eq7_e1024_d_b3: f64 = (eq7_e1022 * var_i_gcd_db3);
        let eq7_e1024_d_b4: f64 = (eq7_e1022 * var_i_gcd_db4);
        let eq7_e1024_d_b5: f64 = (eq7_e1022 * var_i_gcd_db5);
        let eq7_e1024_d_b6: f64 = (eq7_e1022 * var_i_gcd_db6);
        (eq7_e1024, eq7_e1024_d_n0, eq7_e1024_d_n1, eq7_e1024_d_n2, eq7_e1024_d_n3, eq7_e1024_d_n4, eq7_e1024_d_n5, eq7_e1024_d_n6, eq7_e1024_d_n7, eq7_e1024_d_n8, eq7_e1024_d_n9, eq7_e1024_d_n10, eq7_e1024_d_n11, eq7_e1024_d_b0, eq7_e1024_d_b1, eq7_e1024_d_b2, eq7_e1024_d_b3, eq7_e1024_d_b4, eq7_e1024_d_b5, eq7_e1024_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1026;
        let eq7_node_derivatives: [f64; 12] = [eq7_e1026_d_n0, eq7_e1026_d_n1, eq7_e1026_d_n2, eq7_e1026_d_n3, eq7_e1026_d_n4, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8, eq7_e1026_d_n9, eq7_e1026_d_n10, eq7_e1026_d_n11];
        let eq7_branch_derivatives: [f64; 7] = [eq7_e1026_d_b0, eq7_e1026_d_b1, eq7_e1026_d_b2, eq7_e1026_d_b3, eq7_e1026_d_b4, eq7_e1026_d_b5, eq7_e1026_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1029: f64 = (var_chnl_type * var_mult_inst);
        let eq8_e1031: f64 = (eq8_e1029 * p.p32);
        let eq8_e1033: f64 = (eq8_e1031 * var_i_gb);
        let eq8_e1033_d_n0: f64 = (eq8_e1031 * var_i_gb_dn0);
        let eq8_e1033_d_n1: f64 = (eq8_e1031 * var_i_gb_dn1);
        let eq8_e1033_d_n2: f64 = (eq8_e1031 * var_i_gb_dn2);
        let eq8_e1033_d_n3: f64 = (eq8_e1031 * var_i_gb_dn3);
        let eq8_e1033_d_n4: f64 = (eq8_e1031 * var_i_gb_dn4);
        let eq8_e1033_d_n5: f64 = (eq8_e1031 * var_i_gb_dn5);
        let eq8_e1033_d_n6: f64 = (eq8_e1031 * var_i_gb_dn6);
        let eq8_e1033_d_n7: f64 = (eq8_e1031 * var_i_gb_dn7);
        let eq8_e1033_d_n8: f64 = (eq8_e1031 * var_i_gb_dn8);
        let eq8_e1033_d_n9: f64 = (eq8_e1031 * var_i_gb_dn9);
        let eq8_e1033_d_n10: f64 = (eq8_e1031 * var_i_gb_dn10);
        let eq8_e1033_d_n11: f64 = (eq8_e1031 * var_i_gb_dn11);
        let eq8_e1033_d_b0: f64 = (eq8_e1031 * var_i_gb_db0);
        let eq8_e1033_d_b1: f64 = (eq8_e1031 * var_i_gb_db1);
        let eq8_e1033_d_b2: f64 = (eq8_e1031 * var_i_gb_db2);
        let eq8_e1033_d_b3: f64 = (eq8_e1031 * var_i_gb_db3);
        let eq8_e1033_d_b4: f64 = (eq8_e1031 * var_i_gb_db4);
        let eq8_e1033_d_b5: f64 = (eq8_e1031 * var_i_gb_db5);
        let eq8_e1033_d_b6: f64 = (eq8_e1031 * var_i_gb_db6);
        let eq8_value: f64 = eq8_e1033;
        let eq8_node_derivatives: [f64; 12] = [eq8_e1033_d_n0, eq8_e1033_d_n1, eq8_e1033_d_n2, eq8_e1033_d_n3, eq8_e1033_d_n4, eq8_e1033_d_n5, eq8_e1033_d_n6, eq8_e1033_d_n7, eq8_e1033_d_n8, eq8_e1033_d_n9, eq8_e1033_d_n10, eq8_e1033_d_n11];
        let eq8_branch_derivatives: [f64; 7] = [eq8_e1033_d_b0, eq8_e1033_d_b1, eq8_e1033_d_b2, eq8_e1033_d_b3, eq8_e1033_d_b4, eq8_e1033_d_b5, eq8_e1033_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1036: f64 = (var_chnl_type * var_mult_inst);
        let eq9_e1038: f64 = (eq9_e1036 * p.p32);
        let eq9_e1040: f64 = (eq9_e1038 * var_igsov);
        let eq9_e1040_d_n0: f64 = (eq9_e1038 * var_igsov_dn0);
        let eq9_e1040_d_n1: f64 = (eq9_e1038 * var_igsov_dn1);
        let eq9_e1040_d_n2: f64 = (eq9_e1038 * var_igsov_dn2);
        let eq9_e1040_d_n3: f64 = (eq9_e1038 * var_igsov_dn3);
        let eq9_e1040_d_n4: f64 = (eq9_e1038 * var_igsov_dn4);
        let eq9_e1040_d_n5: f64 = (eq9_e1038 * var_igsov_dn5);
        let eq9_e1040_d_n6: f64 = (eq9_e1038 * var_igsov_dn6);
        let eq9_e1040_d_n7: f64 = (eq9_e1038 * var_igsov_dn7);
        let eq9_e1040_d_n8: f64 = (eq9_e1038 * var_igsov_dn8);
        let eq9_e1040_d_n9: f64 = (eq9_e1038 * var_igsov_dn9);
        let eq9_e1040_d_n10: f64 = (eq9_e1038 * var_igsov_dn10);
        let eq9_e1040_d_n11: f64 = (eq9_e1038 * var_igsov_dn11);
        let eq9_e1040_d_b0: f64 = (eq9_e1038 * var_igsov_db0);
        let eq9_e1040_d_b1: f64 = (eq9_e1038 * var_igsov_db1);
        let eq9_e1040_d_b2: f64 = (eq9_e1038 * var_igsov_db2);
        let eq9_e1040_d_b3: f64 = (eq9_e1038 * var_igsov_db3);
        let eq9_e1040_d_b4: f64 = (eq9_e1038 * var_igsov_db4);
        let eq9_e1040_d_b5: f64 = (eq9_e1038 * var_igsov_db5);
        let eq9_e1040_d_b6: f64 = (eq9_e1038 * var_igsov_db6);
        let eq9_value: f64 = eq9_e1040;
        let eq9_node_derivatives: [f64; 12] = [eq9_e1040_d_n0, eq9_e1040_d_n1, eq9_e1040_d_n2, eq9_e1040_d_n3, eq9_e1040_d_n4, eq9_e1040_d_n5, eq9_e1040_d_n6, eq9_e1040_d_n7, eq9_e1040_d_n8, eq9_e1040_d_n9, eq9_e1040_d_n10, eq9_e1040_d_n11];
        let eq9_branch_derivatives: [f64; 7] = [eq9_e1040_d_b0, eq9_e1040_d_b1, eq9_e1040_d_b2, eq9_e1040_d_b3, eq9_e1040_d_b4, eq9_e1040_d_b5, eq9_e1040_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1043: f64 = (var_chnl_type * var_mult_inst);
        let eq10_e1045: f64 = (eq10_e1043 * p.p32);
        let eq10_e1047: f64 = (eq10_e1045 * var_igdov);
        let eq10_e1047_d_n0: f64 = (eq10_e1045 * var_igdov_dn0);
        let eq10_e1047_d_n1: f64 = (eq10_e1045 * var_igdov_dn1);
        let eq10_e1047_d_n2: f64 = (eq10_e1045 * var_igdov_dn2);
        let eq10_e1047_d_n3: f64 = (eq10_e1045 * var_igdov_dn3);
        let eq10_e1047_d_n4: f64 = (eq10_e1045 * var_igdov_dn4);
        let eq10_e1047_d_n5: f64 = (eq10_e1045 * var_igdov_dn5);
        let eq10_e1047_d_n6: f64 = (eq10_e1045 * var_igdov_dn6);
        let eq10_e1047_d_n7: f64 = (eq10_e1045 * var_igdov_dn7);
        let eq10_e1047_d_n8: f64 = (eq10_e1045 * var_igdov_dn8);
        let eq10_e1047_d_n9: f64 = (eq10_e1045 * var_igdov_dn9);
        let eq10_e1047_d_n10: f64 = (eq10_e1045 * var_igdov_dn10);
        let eq10_e1047_d_n11: f64 = (eq10_e1045 * var_igdov_dn11);
        let eq10_e1047_d_b0: f64 = (eq10_e1045 * var_igdov_db0);
        let eq10_e1047_d_b1: f64 = (eq10_e1045 * var_igdov_db1);
        let eq10_e1047_d_b2: f64 = (eq10_e1045 * var_igdov_db2);
        let eq10_e1047_d_b3: f64 = (eq10_e1045 * var_igdov_db3);
        let eq10_e1047_d_b4: f64 = (eq10_e1045 * var_igdov_db4);
        let eq10_e1047_d_b5: f64 = (eq10_e1045 * var_igdov_db5);
        let eq10_e1047_d_b6: f64 = (eq10_e1045 * var_igdov_db6);
        let eq10_value: f64 = eq10_e1047;
        let eq10_node_derivatives: [f64; 12] = [eq10_e1047_d_n0, eq10_e1047_d_n1, eq10_e1047_d_n2, eq10_e1047_d_n3, eq10_e1047_d_n4, eq10_e1047_d_n5, eq10_e1047_d_n6, eq10_e1047_d_n7, eq10_e1047_d_n8, eq10_e1047_d_n9, eq10_e1047_d_n10, eq10_e1047_d_n11];
        let eq10_branch_derivatives: [f64; 7] = [eq10_e1047_d_b0, eq10_e1047_d_b1, eq10_e1047_d_b2, eq10_e1047_d_b3, eq10_e1047_d_b4, eq10_e1047_d_b5, eq10_e1047_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1050: f64 = (var_chnl_type * var_mult_inst);
        let eq11_e1052: f64 = (eq11_e1050 * p.p32);
        let eq11_e1054: f64 = (eq11_e1052 * var_i_gisl);
        let eq11_e1054_d_n0: f64 = (eq11_e1052 * var_i_gisl_dn0);
        let eq11_e1054_d_n1: f64 = (eq11_e1052 * var_i_gisl_dn1);
        let eq11_e1054_d_n2: f64 = (eq11_e1052 * var_i_gisl_dn2);
        let eq11_e1054_d_n3: f64 = (eq11_e1052 * var_i_gisl_dn3);
        let eq11_e1054_d_n4: f64 = (eq11_e1052 * var_i_gisl_dn4);
        let eq11_e1054_d_n5: f64 = (eq11_e1052 * var_i_gisl_dn5);
        let eq11_e1054_d_n6: f64 = (eq11_e1052 * var_i_gisl_dn6);
        let eq11_e1054_d_n7: f64 = (eq11_e1052 * var_i_gisl_dn7);
        let eq11_e1054_d_n8: f64 = (eq11_e1052 * var_i_gisl_dn8);
        let eq11_e1054_d_n9: f64 = (eq11_e1052 * var_i_gisl_dn9);
        let eq11_e1054_d_n10: f64 = (eq11_e1052 * var_i_gisl_dn10);
        let eq11_e1054_d_n11: f64 = (eq11_e1052 * var_i_gisl_dn11);
        let eq11_e1054_d_b0: f64 = (eq11_e1052 * var_i_gisl_db0);
        let eq11_e1054_d_b1: f64 = (eq11_e1052 * var_i_gisl_db1);
        let eq11_e1054_d_b2: f64 = (eq11_e1052 * var_i_gisl_db2);
        let eq11_e1054_d_b3: f64 = (eq11_e1052 * var_i_gisl_db3);
        let eq11_e1054_d_b4: f64 = (eq11_e1052 * var_i_gisl_db4);
        let eq11_e1054_d_b5: f64 = (eq11_e1052 * var_i_gisl_db5);
        let eq11_e1054_d_b6: f64 = (eq11_e1052 * var_i_gisl_db6);
        let eq11_value: f64 = eq11_e1054;
        let eq11_node_derivatives: [f64; 12] = [eq11_e1054_d_n0, eq11_e1054_d_n1, eq11_e1054_d_n2, eq11_e1054_d_n3, eq11_e1054_d_n4, eq11_e1054_d_n5, eq11_e1054_d_n6, eq11_e1054_d_n7, eq11_e1054_d_n8, eq11_e1054_d_n9, eq11_e1054_d_n10, eq11_e1054_d_n11];
        let eq11_branch_derivatives: [f64; 7] = [eq11_e1054_d_b0, eq11_e1054_d_b1, eq11_e1054_d_b2, eq11_e1054_d_b3, eq11_e1054_d_b4, eq11_e1054_d_b5, eq11_e1054_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
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
        var_chnl_type: f64,
        var_gbulk: f64,
        var_gdrain: f64,
        var_ggate: f64,
        var_gjund: f64,
        var_gjuns: f64,
        var_gsource: f64,
        var_guard1719: f64,
        var_guard1720: f64,
        var_guard1721: f64,
        var_guard1722: f64,
        var_guard1723: f64,
        var_guard1724: f64,
        var_guard1725: f64,
        var_gwell: f64,
        var_i_gidl: f64,
        var_i_gidl_db0: f64,
        var_i_gidl_db1: f64,
        var_i_gidl_db2: f64,
        var_i_gidl_db3: f64,
        var_i_gidl_db4: f64,
        var_i_gidl_db5: f64,
        var_i_gidl_db6: f64,
        var_i_gidl_dn0: f64,
        var_i_gidl_dn1: f64,
        var_i_gidl_dn10: f64,
        var_i_gidl_dn11: f64,
        var_i_gidl_dn2: f64,
        var_i_gidl_dn3: f64,
        var_i_gidl_dn4: f64,
        var_i_gidl_dn5: f64,
        var_i_gidl_dn6: f64,
        var_i_gidl_dn7: f64,
        var_i_gidl_dn8: f64,
        var_i_gidl_dn9: f64,
        var_ijun_d: f64,
        var_ijun_d_db0: f64,
        var_ijun_d_db1: f64,
        var_ijun_d_db2: f64,
        var_ijun_d_db3: f64,
        var_ijun_d_db4: f64,
        var_ijun_d_db5: f64,
        var_ijun_d_db6: f64,
        var_ijun_d_dn0: f64,
        var_ijun_d_dn1: f64,
        var_ijun_d_dn10: f64,
        var_ijun_d_dn11: f64,
        var_ijun_d_dn2: f64,
        var_ijun_d_dn3: f64,
        var_ijun_d_dn4: f64,
        var_ijun_d_dn5: f64,
        var_ijun_d_dn6: f64,
        var_ijun_d_dn7: f64,
        var_ijun_d_dn8: f64,
        var_ijun_d_dn9: f64,
        var_ijun_s: f64,
        var_ijun_s_db0: f64,
        var_ijun_s_db1: f64,
        var_ijun_s_db2: f64,
        var_ijun_s_db3: f64,
        var_ijun_s_db4: f64,
        var_ijun_s_db5: f64,
        var_ijun_s_db6: f64,
        var_ijun_s_dn0: f64,
        var_ijun_s_dn1: f64,
        var_ijun_s_dn10: f64,
        var_ijun_s_dn11: f64,
        var_ijun_s_dn2: f64,
        var_ijun_s_dn3: f64,
        var_ijun_s_dn4: f64,
        var_ijun_s_dn5: f64,
        var_ijun_s_dn6: f64,
        var_ijun_s_dn7: f64,
        var_ijun_s_dn8: f64,
        var_ijun_s_dn9: f64,
        var_mult_inst: f64,
        var_qb: f64,
        var_qb_db0: f64,
        var_qb_db1: f64,
        var_qb_db2: f64,
        var_qb_db3: f64,
        var_qb_db4: f64,
        var_qb_db5: f64,
        var_qb_db6: f64,
        var_qb_dn0: f64,
        var_qb_dn1: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn2: f64,
        var_qb_dn3: f64,
        var_qb_dn4: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qd: f64,
        var_qd_db0: f64,
        var_qd_db1: f64,
        var_qd_db2: f64,
        var_qd_db3: f64,
        var_qd_db4: f64,
        var_qd_db5: f64,
        var_qd_db6: f64,
        var_qd_dn0: f64,
        var_qd_dn1: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn2: f64,
        var_qd_dn3: f64,
        var_qd_dn4: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qfgd: f64,
        var_qfgd_db0: f64,
        var_qfgd_db1: f64,
        var_qfgd_db2: f64,
        var_qfgd_db3: f64,
        var_qfgd_db4: f64,
        var_qfgd_db5: f64,
        var_qfgd_db6: f64,
        var_qfgd_dn0: f64,
        var_qfgd_dn1: f64,
        var_qfgd_dn10: f64,
        var_qfgd_dn11: f64,
        var_qfgd_dn2: f64,
        var_qfgd_dn3: f64,
        var_qfgd_dn4: f64,
        var_qfgd_dn5: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgd_dn8: f64,
        var_qfgd_dn9: f64,
        var_qfgs: f64,
        var_qfgs_db0: f64,
        var_qfgs_db1: f64,
        var_qfgs_db2: f64,
        var_qfgs_db3: f64,
        var_qfgs_db4: f64,
        var_qfgs_db5: f64,
        var_qfgs_db6: f64,
        var_qfgs_dn0: f64,
        var_qfgs_dn1: f64,
        var_qfgs_dn10: f64,
        var_qfgs_dn11: f64,
        var_qfgs_dn2: f64,
        var_qfgs_dn3: f64,
        var_qfgs_dn4: f64,
        var_qfgs_dn5: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qfgs_dn8: f64,
        var_qfgs_dn9: f64,
        var_qg: f64,
        var_qg_db0: f64,
        var_qg_db1: f64,
        var_qg_db2: f64,
        var_qg_db3: f64,
        var_qg_db4: f64,
        var_qg_db5: f64,
        var_qg_db6: f64,
        var_qg_dn0: f64,
        var_qg_dn1: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn2: f64,
        var_qg_dn3: f64,
        var_qg_dn4: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq12_e1057: f64 = (var_chnl_type * var_mult_inst);
        let eq12_e1059: f64 = (eq12_e1057 * p.p32);
        let eq12_e1061: f64 = (eq12_e1059 * var_i_gidl);
        let eq12_e1061_d_n0: f64 = (eq12_e1059 * var_i_gidl_dn0);
        let eq12_e1061_d_n1: f64 = (eq12_e1059 * var_i_gidl_dn1);
        let eq12_e1061_d_n2: f64 = (eq12_e1059 * var_i_gidl_dn2);
        let eq12_e1061_d_n3: f64 = (eq12_e1059 * var_i_gidl_dn3);
        let eq12_e1061_d_n4: f64 = (eq12_e1059 * var_i_gidl_dn4);
        let eq12_e1061_d_n5: f64 = (eq12_e1059 * var_i_gidl_dn5);
        let eq12_e1061_d_n6: f64 = (eq12_e1059 * var_i_gidl_dn6);
        let eq12_e1061_d_n7: f64 = (eq12_e1059 * var_i_gidl_dn7);
        let eq12_e1061_d_n8: f64 = (eq12_e1059 * var_i_gidl_dn8);
        let eq12_e1061_d_n9: f64 = (eq12_e1059 * var_i_gidl_dn9);
        let eq12_e1061_d_n10: f64 = (eq12_e1059 * var_i_gidl_dn10);
        let eq12_e1061_d_n11: f64 = (eq12_e1059 * var_i_gidl_dn11);
        let eq12_e1061_d_b0: f64 = (eq12_e1059 * var_i_gidl_db0);
        let eq12_e1061_d_b1: f64 = (eq12_e1059 * var_i_gidl_db1);
        let eq12_e1061_d_b2: f64 = (eq12_e1059 * var_i_gidl_db2);
        let eq12_e1061_d_b3: f64 = (eq12_e1059 * var_i_gidl_db3);
        let eq12_e1061_d_b4: f64 = (eq12_e1059 * var_i_gidl_db4);
        let eq12_e1061_d_b5: f64 = (eq12_e1059 * var_i_gidl_db5);
        let eq12_e1061_d_b6: f64 = (eq12_e1059 * var_i_gidl_db6);
        let eq12_value: f64 = eq12_e1061;
        let eq12_node_derivatives: [f64; 12] = [eq12_e1061_d_n0, eq12_e1061_d_n1, eq12_e1061_d_n2, eq12_e1061_d_n3, eq12_e1061_d_n4, eq12_e1061_d_n5, eq12_e1061_d_n6, eq12_e1061_d_n7, eq12_e1061_d_n8, eq12_e1061_d_n9, eq12_e1061_d_n10, eq12_e1061_d_n11];
        let eq12_branch_derivatives: [f64; 7] = [eq12_e1061_d_b0, eq12_e1061_d_b1, eq12_e1061_d_b2, eq12_e1061_d_b3, eq12_e1061_d_b4, eq12_e1061_d_b5, eq12_e1061_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1064: f64 = (var_chnl_type * var_mult_inst);
        let eq13_e1066: f64 = (eq13_e1064 * p.p32);
        let eq13_e1068: f64 = (eq13_e1066 * var_ijun_s);
        let eq13_e1068_d_n0: f64 = (eq13_e1066 * var_ijun_s_dn0);
        let eq13_e1068_d_n1: f64 = (eq13_e1066 * var_ijun_s_dn1);
        let eq13_e1068_d_n2: f64 = (eq13_e1066 * var_ijun_s_dn2);
        let eq13_e1068_d_n3: f64 = (eq13_e1066 * var_ijun_s_dn3);
        let eq13_e1068_d_n4: f64 = (eq13_e1066 * var_ijun_s_dn4);
        let eq13_e1068_d_n5: f64 = (eq13_e1066 * var_ijun_s_dn5);
        let eq13_e1068_d_n6: f64 = (eq13_e1066 * var_ijun_s_dn6);
        let eq13_e1068_d_n7: f64 = (eq13_e1066 * var_ijun_s_dn7);
        let eq13_e1068_d_n8: f64 = (eq13_e1066 * var_ijun_s_dn8);
        let eq13_e1068_d_n9: f64 = (eq13_e1066 * var_ijun_s_dn9);
        let eq13_e1068_d_n10: f64 = (eq13_e1066 * var_ijun_s_dn10);
        let eq13_e1068_d_n11: f64 = (eq13_e1066 * var_ijun_s_dn11);
        let eq13_e1068_d_b0: f64 = (eq13_e1066 * var_ijun_s_db0);
        let eq13_e1068_d_b1: f64 = (eq13_e1066 * var_ijun_s_db1);
        let eq13_e1068_d_b2: f64 = (eq13_e1066 * var_ijun_s_db2);
        let eq13_e1068_d_b3: f64 = (eq13_e1066 * var_ijun_s_db3);
        let eq13_e1068_d_b4: f64 = (eq13_e1066 * var_ijun_s_db4);
        let eq13_e1068_d_b5: f64 = (eq13_e1066 * var_ijun_s_db5);
        let eq13_e1068_d_b6: f64 = (eq13_e1066 * var_ijun_s_db6);
        let eq13_value: f64 = eq13_e1068;
        let eq13_node_derivatives: [f64; 12] = [eq13_e1068_d_n0, eq13_e1068_d_n1, eq13_e1068_d_n2, eq13_e1068_d_n3, eq13_e1068_d_n4, eq13_e1068_d_n5, eq13_e1068_d_n6, eq13_e1068_d_n7, eq13_e1068_d_n8, eq13_e1068_d_n9, eq13_e1068_d_n10, eq13_e1068_d_n11];
        let eq13_branch_derivatives: [f64; 7] = [eq13_e1068_d_b0, eq13_e1068_d_b1, eq13_e1068_d_b2, eq13_e1068_d_b3, eq13_e1068_d_b4, eq13_e1068_d_b5, eq13_e1068_d_b6];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1071: f64 = (var_chnl_type * var_mult_inst);
        let eq14_e1073: f64 = (eq14_e1071 * p.p32);
        let eq14_e1075: f64 = (eq14_e1073 * var_ijun_d);
        let eq14_e1075_d_n0: f64 = (eq14_e1073 * var_ijun_d_dn0);
        let eq14_e1075_d_n1: f64 = (eq14_e1073 * var_ijun_d_dn1);
        let eq14_e1075_d_n2: f64 = (eq14_e1073 * var_ijun_d_dn2);
        let eq14_e1075_d_n3: f64 = (eq14_e1073 * var_ijun_d_dn3);
        let eq14_e1075_d_n4: f64 = (eq14_e1073 * var_ijun_d_dn4);
        let eq14_e1075_d_n5: f64 = (eq14_e1073 * var_ijun_d_dn5);
        let eq14_e1075_d_n6: f64 = (eq14_e1073 * var_ijun_d_dn6);
        let eq14_e1075_d_n7: f64 = (eq14_e1073 * var_ijun_d_dn7);
        let eq14_e1075_d_n8: f64 = (eq14_e1073 * var_ijun_d_dn8);
        let eq14_e1075_d_n9: f64 = (eq14_e1073 * var_ijun_d_dn9);
        let eq14_e1075_d_n10: f64 = (eq14_e1073 * var_ijun_d_dn10);
        let eq14_e1075_d_n11: f64 = (eq14_e1073 * var_ijun_d_dn11);
        let eq14_e1075_d_b0: f64 = (eq14_e1073 * var_ijun_d_db0);
        let eq14_e1075_d_b1: f64 = (eq14_e1073 * var_ijun_d_db1);
        let eq14_e1075_d_b2: f64 = (eq14_e1073 * var_ijun_d_db2);
        let eq14_e1075_d_b3: f64 = (eq14_e1073 * var_ijun_d_db3);
        let eq14_e1075_d_b4: f64 = (eq14_e1073 * var_ijun_d_db4);
        let eq14_e1075_d_b5: f64 = (eq14_e1073 * var_ijun_d_db5);
        let eq14_e1075_d_b6: f64 = (eq14_e1073 * var_ijun_d_db6);
        let eq14_value: f64 = eq14_e1075;
        let eq14_node_derivatives: [f64; 12] = [eq14_e1075_d_n0, eq14_e1075_d_n1, eq14_e1075_d_n2, eq14_e1075_d_n3, eq14_e1075_d_n4, eq14_e1075_d_n5, eq14_e1075_d_n6, eq14_e1075_d_n7, eq14_e1075_d_n8, eq14_e1075_d_n9, eq14_e1075_d_n10, eq14_e1075_d_n11];
        let eq14_branch_derivatives: [f64; 7] = [eq14_e1075_d_b0, eq14_e1075_d_b1, eq14_e1075_d_b2, eq14_e1075_d_b3, eq14_e1075_d_b4, eq14_e1075_d_b5, eq14_e1075_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1085, eq15_e1085_d_n1, eq15_e1085_d_n5,) = {
    if (var_guard1719 != 0.0) {
        let eq15_e1079: f64 = (var_mult_inst * p.p32);
        let eq15_e1081: f64 = (eq15_e1079 * var_ggate);
        let eq15_e1083: f64 = (eq15_e1081 * (nv1 - nv5));
        (eq15_e1083, eq15_e1081, (-eq15_e1081),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1085;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1085_d_n1),
            5,
            multiplicity * (eq15_e1085_d_n5),
        );
        let (eq17_e1100,) = {
    if (var_guard1719 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1100;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1110, eq18_e1110_d_n2, eq18_e1110_d_n6,) = {
    if (var_guard1720 != 0.0) {
        let eq18_e1104: f64 = (var_mult_inst * p.p32);
        let eq18_e1106: f64 = (eq18_e1104 * var_gsource);
        let eq18_e1108: f64 = (eq18_e1106 * (nv2 - nv6));
        (eq18_e1108, eq18_e1106, (-eq18_e1106),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1110;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(6),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1110_d_n2),
            6,
            multiplicity * (eq18_e1110_d_n6),
        );
        let (eq20_e1125,) = {
    if (var_guard1720 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1135, eq21_e1135_d_n0, eq21_e1135_d_n7,) = {
    if (var_guard1721 != 0.0) {
        let eq21_e1129: f64 = (var_mult_inst * p.p32);
        let eq21_e1131: f64 = (eq21_e1129 * var_gdrain);
        let eq21_e1133: f64 = (eq21_e1131 * (nv0 - nv7));
        (eq21_e1133, eq21_e1131, (-eq21_e1131),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1135;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(7),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1135_d_n0),
            7,
            multiplicity * (eq21_e1135_d_n7),
        );
        let (eq23_e1150,) = {
    if (var_guard1721 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1150;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1160, eq24_e1160_d_n8, eq24_e1160_d_n9,) = {
    if (var_guard1722 != 0.0) {
        let eq24_e1154: f64 = (var_mult_inst * p.p32);
        let eq24_e1156: f64 = (eq24_e1154 * var_gbulk);
        let eq24_e1158: f64 = (eq24_e1156 * (nv8 - nv9));
        (eq24_e1158, eq24_e1156, (-eq24_e1156),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1160;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            8,
            multiplicity * (eq24_e1160_d_n8),
            9,
            multiplicity * (eq24_e1160_d_n9),
        );
        let (eq26_e1175,) = {
    if (var_guard1722 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1175;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1185, eq27_e1185_d_n9, eq27_e1185_d_n10,) = {
    if (var_guard1723 != 0.0) {
        let eq27_e1179: f64 = (var_mult_inst * p.p32);
        let eq27_e1181: f64 = (eq27_e1179 * var_gjuns);
        let eq27_e1183: f64 = (eq27_e1181 * (nv10 - nv9));
        (eq27_e1183, (-eq27_e1181), eq27_e1181,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1185;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            9,
            multiplicity * (eq27_e1185_d_n9),
            10,
            multiplicity * (eq27_e1185_d_n10),
        );
        let (eq29_e1200,) = {
    if (var_guard1723 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1200;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1210, eq30_e1210_d_n9, eq30_e1210_d_n11,) = {
    if (var_guard1724 != 0.0) {
        let eq30_e1204: f64 = (var_mult_inst * p.p32);
        let eq30_e1206: f64 = (eq30_e1204 * var_gjund);
        let eq30_e1208: f64 = (eq30_e1206 * (nv11 - nv9));
        (eq30_e1208, (-eq30_e1206), eq30_e1206,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1210;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            9,
            multiplicity * (eq30_e1210_d_n9),
            11,
            multiplicity * (eq30_e1210_d_n11),
        );
        let (eq32_e1225,) = {
    if (var_guard1724 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1225;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1235, eq33_e1235_d_n3, eq33_e1235_d_n9,) = {
    if (var_guard1725 != 0.0) {
        let eq33_e1229: f64 = (var_mult_inst * p.p32);
        let eq33_e1231: f64 = (eq33_e1229 * var_gwell);
        let eq33_e1233: f64 = (eq33_e1231 * (nv3 - nv9));
        (eq33_e1233, eq33_e1231, (-eq33_e1231),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1235;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1235_d_n3),
            9,
            multiplicity * (eq33_e1235_d_n9),
        );
        let (eq35_e1250,) = {
    if (var_guard1725 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1250;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq38_e1263: f64 = (var_chnl_type * var_mult_inst);
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * var_qg);
        let eq38_e1267_d_n0: f64 = (eq38_e1265 * var_qg_dn0);
        let eq38_e1267_d_n1: f64 = (eq38_e1265 * var_qg_dn1);
        let eq38_e1267_d_n2: f64 = (eq38_e1265 * var_qg_dn2);
        let eq38_e1267_d_n3: f64 = (eq38_e1265 * var_qg_dn3);
        let eq38_e1267_d_n4: f64 = (eq38_e1265 * var_qg_dn4);
        let eq38_e1267_d_n5: f64 = (eq38_e1265 * var_qg_dn5);
        let eq38_e1267_d_n6: f64 = (eq38_e1265 * var_qg_dn6);
        let eq38_e1267_d_n7: f64 = (eq38_e1265 * var_qg_dn7);
        let eq38_e1267_d_n8: f64 = (eq38_e1265 * var_qg_dn8);
        let eq38_e1267_d_n9: f64 = (eq38_e1265 * var_qg_dn9);
        let eq38_e1267_d_n10: f64 = (eq38_e1265 * var_qg_dn10);
        let eq38_e1267_d_n11: f64 = (eq38_e1265 * var_qg_dn11);
        let eq38_e1267_d_b0: f64 = (eq38_e1265 * var_qg_db0);
        let eq38_e1267_d_b1: f64 = (eq38_e1265 * var_qg_db1);
        let eq38_e1267_d_b2: f64 = (eq38_e1265 * var_qg_db2);
        let eq38_e1267_d_b3: f64 = (eq38_e1265 * var_qg_db3);
        let eq38_e1267_d_b4: f64 = (eq38_e1265 * var_qg_db4);
        let eq38_e1267_d_b5: f64 = (eq38_e1265 * var_qg_db5);
        let eq38_e1267_d_b6: f64 = (eq38_e1265 * var_qg_db6);
        let eq38_e1268: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq38_e1267);
        let eq38_value: f64 = eq38_e1268;
        let eq38_node_derivatives: [f64; 12] = [(eq38_e1267_d_n0 * ddt_scale), (eq38_e1267_d_n1 * ddt_scale), (eq38_e1267_d_n2 * ddt_scale), (eq38_e1267_d_n3 * ddt_scale), (eq38_e1267_d_n4 * ddt_scale), (eq38_e1267_d_n5 * ddt_scale), (eq38_e1267_d_n6 * ddt_scale), (eq38_e1267_d_n7 * ddt_scale), (eq38_e1267_d_n8 * ddt_scale), (eq38_e1267_d_n9 * ddt_scale), (eq38_e1267_d_n10 * ddt_scale), (eq38_e1267_d_n11 * ddt_scale)];
        let eq38_branch_derivatives: [f64; 7] = [(eq38_e1267_d_b0 * ddt_scale), (eq38_e1267_d_b1 * ddt_scale), (eq38_e1267_d_b2 * ddt_scale), (eq38_e1267_d_b3 * ddt_scale), (eq38_e1267_d_b4 * ddt_scale), (eq38_e1267_d_b5 * ddt_scale), (eq38_e1267_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let eq39_e1271: f64 = (var_chnl_type * var_mult_inst);
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * var_qb);
        let eq39_e1275_d_n0: f64 = (eq39_e1273 * var_qb_dn0);
        let eq39_e1275_d_n1: f64 = (eq39_e1273 * var_qb_dn1);
        let eq39_e1275_d_n2: f64 = (eq39_e1273 * var_qb_dn2);
        let eq39_e1275_d_n3: f64 = (eq39_e1273 * var_qb_dn3);
        let eq39_e1275_d_n4: f64 = (eq39_e1273 * var_qb_dn4);
        let eq39_e1275_d_n5: f64 = (eq39_e1273 * var_qb_dn5);
        let eq39_e1275_d_n6: f64 = (eq39_e1273 * var_qb_dn6);
        let eq39_e1275_d_n7: f64 = (eq39_e1273 * var_qb_dn7);
        let eq39_e1275_d_n8: f64 = (eq39_e1273 * var_qb_dn8);
        let eq39_e1275_d_n9: f64 = (eq39_e1273 * var_qb_dn9);
        let eq39_e1275_d_n10: f64 = (eq39_e1273 * var_qb_dn10);
        let eq39_e1275_d_n11: f64 = (eq39_e1273 * var_qb_dn11);
        let eq39_e1275_d_b0: f64 = (eq39_e1273 * var_qb_db0);
        let eq39_e1275_d_b1: f64 = (eq39_e1273 * var_qb_db1);
        let eq39_e1275_d_b2: f64 = (eq39_e1273 * var_qb_db2);
        let eq39_e1275_d_b3: f64 = (eq39_e1273 * var_qb_db3);
        let eq39_e1275_d_b4: f64 = (eq39_e1273 * var_qb_db4);
        let eq39_e1275_d_b5: f64 = (eq39_e1273 * var_qb_db5);
        let eq39_e1275_d_b6: f64 = (eq39_e1273 * var_qb_db6);
        let eq39_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq39_e1275);
        let eq39_value: f64 = eq39_e1276;
        let eq39_node_derivatives: [f64; 12] = [(eq39_e1275_d_n0 * ddt_scale), (eq39_e1275_d_n1 * ddt_scale), (eq39_e1275_d_n2 * ddt_scale), (eq39_e1275_d_n3 * ddt_scale), (eq39_e1275_d_n4 * ddt_scale), (eq39_e1275_d_n5 * ddt_scale), (eq39_e1275_d_n6 * ddt_scale), (eq39_e1275_d_n7 * ddt_scale), (eq39_e1275_d_n8 * ddt_scale), (eq39_e1275_d_n9 * ddt_scale), (eq39_e1275_d_n10 * ddt_scale), (eq39_e1275_d_n11 * ddt_scale)];
        let eq39_branch_derivatives: [f64; 7] = [(eq39_e1275_d_b0 * ddt_scale), (eq39_e1275_d_b1 * ddt_scale), (eq39_e1275_d_b2 * ddt_scale), (eq39_e1275_d_b3 * ddt_scale), (eq39_e1275_d_b4 * ddt_scale), (eq39_e1275_d_b5 * ddt_scale), (eq39_e1275_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1279: f64 = (var_chnl_type * var_mult_inst);
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * var_qd);
        let eq40_e1283_d_n0: f64 = (eq40_e1281 * var_qd_dn0);
        let eq40_e1283_d_n1: f64 = (eq40_e1281 * var_qd_dn1);
        let eq40_e1283_d_n2: f64 = (eq40_e1281 * var_qd_dn2);
        let eq40_e1283_d_n3: f64 = (eq40_e1281 * var_qd_dn3);
        let eq40_e1283_d_n4: f64 = (eq40_e1281 * var_qd_dn4);
        let eq40_e1283_d_n5: f64 = (eq40_e1281 * var_qd_dn5);
        let eq40_e1283_d_n6: f64 = (eq40_e1281 * var_qd_dn6);
        let eq40_e1283_d_n7: f64 = (eq40_e1281 * var_qd_dn7);
        let eq40_e1283_d_n8: f64 = (eq40_e1281 * var_qd_dn8);
        let eq40_e1283_d_n9: f64 = (eq40_e1281 * var_qd_dn9);
        let eq40_e1283_d_n10: f64 = (eq40_e1281 * var_qd_dn10);
        let eq40_e1283_d_n11: f64 = (eq40_e1281 * var_qd_dn11);
        let eq40_e1283_d_b0: f64 = (eq40_e1281 * var_qd_db0);
        let eq40_e1283_d_b1: f64 = (eq40_e1281 * var_qd_db1);
        let eq40_e1283_d_b2: f64 = (eq40_e1281 * var_qd_db2);
        let eq40_e1283_d_b3: f64 = (eq40_e1281 * var_qd_db3);
        let eq40_e1283_d_b4: f64 = (eq40_e1281 * var_qd_db4);
        let eq40_e1283_d_b5: f64 = (eq40_e1281 * var_qd_db5);
        let eq40_e1283_d_b6: f64 = (eq40_e1281 * var_qd_db6);
        let eq40_e1284: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq40_e1283);
        let eq40_value: f64 = eq40_e1284;
        let eq40_node_derivatives: [f64; 12] = [(eq40_e1283_d_n0 * ddt_scale), (eq40_e1283_d_n1 * ddt_scale), (eq40_e1283_d_n2 * ddt_scale), (eq40_e1283_d_n3 * ddt_scale), (eq40_e1283_d_n4 * ddt_scale), (eq40_e1283_d_n5 * ddt_scale), (eq40_e1283_d_n6 * ddt_scale), (eq40_e1283_d_n7 * ddt_scale), (eq40_e1283_d_n8 * ddt_scale), (eq40_e1283_d_n9 * ddt_scale), (eq40_e1283_d_n10 * ddt_scale), (eq40_e1283_d_n11 * ddt_scale)];
        let eq40_branch_derivatives: [f64; 7] = [(eq40_e1283_d_b0 * ddt_scale), (eq40_e1283_d_b1 * ddt_scale), (eq40_e1283_d_b2 * ddt_scale), (eq40_e1283_d_b3 * ddt_scale), (eq40_e1283_d_b4 * ddt_scale), (eq40_e1283_d_b5 * ddt_scale), (eq40_e1283_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1287: f64 = (var_chnl_type * var_mult_inst);
        let eq41_e1289: f64 = (eq41_e1287 * p.p33);
        let eq41_e1291: f64 = (eq41_e1289 * var_qfgs);
        let eq41_e1291_d_n0: f64 = (eq41_e1289 * var_qfgs_dn0);
        let eq41_e1291_d_n1: f64 = (eq41_e1289 * var_qfgs_dn1);
        let eq41_e1291_d_n2: f64 = (eq41_e1289 * var_qfgs_dn2);
        let eq41_e1291_d_n3: f64 = (eq41_e1289 * var_qfgs_dn3);
        let eq41_e1291_d_n4: f64 = (eq41_e1289 * var_qfgs_dn4);
        let eq41_e1291_d_n5: f64 = (eq41_e1289 * var_qfgs_dn5);
        let eq41_e1291_d_n6: f64 = (eq41_e1289 * var_qfgs_dn6);
        let eq41_e1291_d_n7: f64 = (eq41_e1289 * var_qfgs_dn7);
        let eq41_e1291_d_n8: f64 = (eq41_e1289 * var_qfgs_dn8);
        let eq41_e1291_d_n9: f64 = (eq41_e1289 * var_qfgs_dn9);
        let eq41_e1291_d_n10: f64 = (eq41_e1289 * var_qfgs_dn10);
        let eq41_e1291_d_n11: f64 = (eq41_e1289 * var_qfgs_dn11);
        let eq41_e1291_d_b0: f64 = (eq41_e1289 * var_qfgs_db0);
        let eq41_e1291_d_b1: f64 = (eq41_e1289 * var_qfgs_db1);
        let eq41_e1291_d_b2: f64 = (eq41_e1289 * var_qfgs_db2);
        let eq41_e1291_d_b3: f64 = (eq41_e1289 * var_qfgs_db3);
        let eq41_e1291_d_b4: f64 = (eq41_e1289 * var_qfgs_db4);
        let eq41_e1291_d_b5: f64 = (eq41_e1289 * var_qfgs_db5);
        let eq41_e1291_d_b6: f64 = (eq41_e1289 * var_qfgs_db6);
        let eq41_e1292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq41_e1291);
        let eq41_value: f64 = eq41_e1292;
        let eq41_node_derivatives: [f64; 12] = [(eq41_e1291_d_n0 * ddt_scale), (eq41_e1291_d_n1 * ddt_scale), (eq41_e1291_d_n2 * ddt_scale), (eq41_e1291_d_n3 * ddt_scale), (eq41_e1291_d_n4 * ddt_scale), (eq41_e1291_d_n5 * ddt_scale), (eq41_e1291_d_n6 * ddt_scale), (eq41_e1291_d_n7 * ddt_scale), (eq41_e1291_d_n8 * ddt_scale), (eq41_e1291_d_n9 * ddt_scale), (eq41_e1291_d_n10 * ddt_scale), (eq41_e1291_d_n11 * ddt_scale)];
        let eq41_branch_derivatives: [f64; 7] = [(eq41_e1291_d_b0 * ddt_scale), (eq41_e1291_d_b1 * ddt_scale), (eq41_e1291_d_b2 * ddt_scale), (eq41_e1291_d_b3 * ddt_scale), (eq41_e1291_d_b4 * ddt_scale), (eq41_e1291_d_b5 * ddt_scale), (eq41_e1291_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1295: f64 = (var_chnl_type * var_mult_inst);
        let eq42_e1297: f64 = (eq42_e1295 * p.p33);
        let eq42_e1299: f64 = (eq42_e1297 * var_qfgd);
        let eq42_e1299_d_n0: f64 = (eq42_e1297 * var_qfgd_dn0);
        let eq42_e1299_d_n1: f64 = (eq42_e1297 * var_qfgd_dn1);
        let eq42_e1299_d_n2: f64 = (eq42_e1297 * var_qfgd_dn2);
        let eq42_e1299_d_n3: f64 = (eq42_e1297 * var_qfgd_dn3);
        let eq42_e1299_d_n4: f64 = (eq42_e1297 * var_qfgd_dn4);
        let eq42_e1299_d_n5: f64 = (eq42_e1297 * var_qfgd_dn5);
        let eq42_e1299_d_n6: f64 = (eq42_e1297 * var_qfgd_dn6);
        let eq42_e1299_d_n7: f64 = (eq42_e1297 * var_qfgd_dn7);
        let eq42_e1299_d_n8: f64 = (eq42_e1297 * var_qfgd_dn8);
        let eq42_e1299_d_n9: f64 = (eq42_e1297 * var_qfgd_dn9);
        let eq42_e1299_d_n10: f64 = (eq42_e1297 * var_qfgd_dn10);
        let eq42_e1299_d_n11: f64 = (eq42_e1297 * var_qfgd_dn11);
        let eq42_e1299_d_b0: f64 = (eq42_e1297 * var_qfgd_db0);
        let eq42_e1299_d_b1: f64 = (eq42_e1297 * var_qfgd_db1);
        let eq42_e1299_d_b2: f64 = (eq42_e1297 * var_qfgd_db2);
        let eq42_e1299_d_b3: f64 = (eq42_e1297 * var_qfgd_db3);
        let eq42_e1299_d_b4: f64 = (eq42_e1297 * var_qfgd_db4);
        let eq42_e1299_d_b5: f64 = (eq42_e1297 * var_qfgd_db5);
        let eq42_e1299_d_b6: f64 = (eq42_e1297 * var_qfgd_db6);
        let eq42_e1300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq42_e1299);
        let eq42_value: f64 = eq42_e1300;
        let eq42_node_derivatives: [f64; 12] = [(eq42_e1299_d_n0 * ddt_scale), (eq42_e1299_d_n1 * ddt_scale), (eq42_e1299_d_n2 * ddt_scale), (eq42_e1299_d_n3 * ddt_scale), (eq42_e1299_d_n4 * ddt_scale), (eq42_e1299_d_n5 * ddt_scale), (eq42_e1299_d_n6 * ddt_scale), (eq42_e1299_d_n7 * ddt_scale), (eq42_e1299_d_n8 * ddt_scale), (eq42_e1299_d_n9 * ddt_scale), (eq42_e1299_d_n10 * ddt_scale), (eq42_e1299_d_n11 * ddt_scale)];
        let eq42_branch_derivatives: [f64; 7] = [(eq42_e1299_d_b0 * ddt_scale), (eq42_e1299_d_b1 * ddt_scale), (eq42_e1299_d_b2 * ddt_scale), (eq42_e1299_d_b3 * ddt_scale), (eq42_e1299_d_b4 * ddt_scale), (eq42_e1299_d_b5 * ddt_scale), (eq42_e1299_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
    }
}
