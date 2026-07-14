#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        s: &mut Scratch,
    ) {
        if ((s.b[2180] && (!s.b[2196])) && s.b[2198]) {s.store_add_scaled_inputs_product_mixed_iiia(2114, 2108, 2.0, 2113, (-2.0), 2007, A::add(A::sub_from_scalar(1.0, s.ad_value(2104)), s.ad_value(2106)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2115, 2113, 2113, 1.0, 2108, 2.0);s.store_sub_from_scalar_scaled_mul_mixed_ia(2116, 1.0, 2007, A::add(s.ad_value(2104), s.ad_value(2106)), 0.5);s.store_div_scaled_product_mixed_iia(2117, 2115, 2114, 1.0, A::add_scaled_square_product(s.ad_value(2114), 1.0, s.ad_value(2116), s.ad_value(2115), (-1.0)), 1.0);s.store_add(2103, 2103, 2117);s.store_exp(2118, 2117);s.store_div(2104, 2104, 2118);s.store_mul(2106, 2106, 2118);s.store_add_offset_lhs(2107, 2103, (-1.0), 2104);s.store_mul_sqrt_mixed_ia(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));s.store_add_ad(2119, A::sub_from_scalar(1.0, s.ad_value(2104)), A::mul3_scaled_output(s.ad_value(2108), s.ad_value(2109), s.ad_value(2023), 2.0));s.store_div_scaled_product3_mixed_iiaa(2093, 2093, 2118, A::add(s.ad_value(2112), s.ad_value(2105)), 1.0, A::add_scaled_product(s.ad_value(2119), 1.0, s.ad_value(2118), s.ad_value(2105), 1.0), 1.0);s.store_mul(2096, 2093, 2021);}
        if (s.b[2180] && (!s.b[2196])) {s.store_sqrt(2110, 2107);s.store_add_scaled_inputs_mixed_ia(2111, 2109, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2104)), s.ad_value(2110)), 0.5);}
        if s.b[2180] {s.store_mul_div_scaled_product_mixed_iiia(2120, 2021, 2007, 2106, 1.0, A::add_scaled_product(s.ad_value(2108), 1.0, s.ad_value(2006), s.ad_value(2110), 1.0), 1.0);s.store_add_scaled_product_indices(2121, 2120, 1.0, 2021, 2111, 1.0);s.store_mul3_lhs(2122, 2110, 2006, 2021);}
        s.b[2199] = (s.v[213] < 0.0);s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });
        if (s.b[2180] && s.b[2199]) {s.store_sub_from_scalar_scaled_mul(2061, 1.0, 213, 2120, 1.0);}
        if (s.b[2180] && (!s.b[2199])) {s.store_div_from_scalar_offset_product(2061, 1.0, 213, 2120, 1.0);}
        if s.b[2180] {s.store_mul_product3_indices(2062, 2120, 751, 2060, 2061, 1.0);s.store_add_scaled_product_indices(2123, 2122, 1.0, 769, 2120, 1.0);s.store_add_scaled_product_indices(2124, 2122, 1.0, 770, 2120, 1.0);s.store_mul(2125, 768, 2123);s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2107), 1.0, A::add(s.ad_value(2107), s.ad_value(2106)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2125), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);s.store_mul_add_mixed_iai(2126, 2056, A::offset(s.ad_value(2064), 1.0), 2062);s.store_ln_ad(2127, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(820), s.ad_value(2096)), s.ad_value(773)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2087), s.ad_value(2096)), s.ad_value(773)), 1.0), 1.0));s.store_mul(1921, 2120, 2066);s.store_div_add_scaled_inputs_rhs_indices(2067, 1921, 218, 1.0, 1921, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2200] = (s.v[217] < 0.0);s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });
        if (s.b[2180] && s.b[2200]) {s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));}
        if (s.b[2180] && (!s.b[2200])) {s.store_offset_mul(2068, 217, 2067, 1.0);}
        if s.b[2180] {s.store_mul(2129, 2001, 2068);s.store_mul(2128, 2108, 2021);}
        s.copy_ad(1843, 2069);s.copy_ad(1845, 2087);s.copy_ad(1846, 2088);s.copy_ad(1847, 2093);s.copy_ad(1848, 2096);s.copy_ad(1850, 2103);s.copy_ad(1849, 2102);s.copy_ad(1851, 2109);s.copy_ad(1852, 2111);s.copy_ad(1853, 2120);s.copy_ad(1854, 2121);s.copy_ad(1855, 2122);s.copy_ad(1856, 2124);s.copy_ad(1857, 2126);s.copy_ad(1859, 2127);s.copy_ad(1858, 2129);s.copy_ad(1860, 2128);s.store_scalar(1861, 1.0);s.store_scalar(1862, 1.0);s.store_scalar(1864, 1.0);s.store_scalar(1865, 1.0);s.store_scalar(832, 0.0);s.b[2201] = (s.v[1817] > 0.0);s.store_scalar(2201, if s.b[2201] { 1.0 } else { 0.0 });
        if s.b[2201] {s.store_ln_ad(1929, A::offset(A::mul(s.ad_value(824), s.ad_value(773)), 1.0));s.store_div_scaled_product_indices(1919, 1812, 1852, 1.0, 1854, 1.0);s.store_add_scaled_product_mixed_aai(1928, A::mul3(A::mul3(s.ad_value(222), s.ad_value(1855), s.ad_value(1919)), s.ad_value(1919), s.ad_value(1929)), 1.0, A::div_scaled_product(A::add(s.ad_value(220), A::div(s.ad_value(221), s.ad_value(1854))), s.ad_value(1853), 1.0, s.ad_value(1854), 1.0), 1859, 1.0);s.store_div_from_scalar_add_ad(1861, 1.0, A::offset(s.ad_value(1928), 1.0), A::square(s.ad_value(1928)));s.store_mul(1862, 1857, 1861);s.store_div(1863, 1858, 1862);s.store_mul_ad_product_lhs_mixed_ai(1930, A::square(s.ad_value(1863)), 1848, 1848);}
        s.b[2202] = (s.v[0] == (-1.0));s.store_scalar(2202, if s.b[2202] { 1.0 } else { 0.0 });
        if (s.b[2201] && s.b[2202]) {s.store_div_scaled_value_offset_denominator(1930, s.ad_value(1930), 1.0, A::mul(s.ad_value(1863), s.ad_value(1848)), 1.0, 1.0);}
        if s.b[2201] {s.store_mul_scale_offset_mixed_ia(1931, 1862, A::sqrt(A::scale_offset(s.ad_value(1930), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_from_scalar(1864, 1.0, 1931);s.store_mul(1919, 1862, 1864);s.store_mul_scale_offset_mixed_ia(1932, 1852, A::mul3_scaled_output(s.ad_value(1930), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1865, 1919, 1854, 1.0, 1932, 1.0);s.store_mul_product3_indices(832, 1864, 710, 1854, 1848, 1.0);}
        s.store_scalar(1934, 0.0);s.store_scalar(1935, 0.0);s.store_scalar(1866, 0.0);s.store_scalar(1867, 0.0);s.b[2203] = (((((p.p40 != 0.0) && ((s.v[232] > 0.0) || (s.v[233] > 0.0))) || ((p.p42 != 0.0) && ((s.v[242] > 0.0) || (s.v[243] > 0.0)))) || (s.v[257] > 0.0)) || (s.v[258] > 0.0));s.store_scalar(2203, if s.b[2203] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2203] {s.store_scaled_add_mixed_ia(1933, 1805, A::sqrt(A::add(A::square(s.ad_value(1805)), s.ad_value(783))), 0.5);s.store_add_mixed_ai(1934, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(778), (-0.5), s.ad_value(776), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), 1.0), 785);s.store_scaled_add_mixed_ia(1933, 1806, A::sqrt(A::add(A::square(s.ad_value(1806)), s.ad_value(786))), 0.5);s.store_add_mixed_ai(1935, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(779), (-0.5), s.ad_value(777), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), 1.0), 788);s.store_scaled_add(1866, 1805, 1934, (-s.v[348]));s.store_scaled_add(1867, 1806, 1935, (-s.v[348]));}
        s.store_scalar(833, 0.0);s.store_scalar(834, 0.0);s.store_scalar(1962, 0.0);s.store_scalar(837, 0.0);s.store_scalar(835, 0.0);s.store_scalar(836, 0.0);s.b[2204] = (p.p40 != 0.0);s.store_scalar(2204, if s.b[2204] { 1.0 } else { 0.0 });s.b[2205] = (s.v[232] > 0.0);s.store_scalar(2205, if s.b[2205] { 1.0 } else { 0.0 });
        if (s.b[2204] && s.b[2205]) {s.store_mul_sqrt_mixed_ia(1936, 789, A::offset(A::square(s.ad_value(1866)), 1e-6));}
        s.b[2206] = (s.v[238] < 0.0);s.store_scalar(2206, if s.b[2206] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2205]) && s.b[2206]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 795, 0.5, 1936, 795, 1e-6, (-0.5));}
        if (s.b[2204] && s.b[2205]) {s.store_mul_scale_offset_mixed_ia(1919, 792, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(237), 1.0, s.ad_value(238), s.ad_value(1936), 1.0)), 1.0, (-1.5));}
        s.b[2207] = (s.v[1919] > 0.0);s.store_scalar(2207, if s.b[2207] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2205]) && s.b[2207]) {s.store_offset_mul_offset_rhs_mixed_ia(1937, 1919, A::mul_scaled_output(s.ad_value(1919), A::scale_offset(s.ad_value(1919), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);}
        s.b[2208] = (s.v[1919] > (-230.25850929940458));s.store_scalar(2208, if s.b[2208] { 1.0 } else { 0.0 });
        if (((s.b[2204] && s.b[2205]) && (!s.b[2207])) && s.b[2208]) {s.store_exp(1937, 1919);}
        if (((s.b[2204] && s.b[2205]) && (!s.b[2207])) && (!s.b[2208])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1937, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2204] && s.b[2205]) {s.store_offset(1938, 1934, 3.0);s.store_primal_sub_from_scalar(1939, (-3.0), 230);s.store_scale(1940, 828, 30.0);s.store_scalar(812, (4.0 - 0.9));s.store_add(813, 1938, 1940);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1919, 2.0, 812, A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));s.store_scalar(812, (4.0 - 0.3));s.store_add(813, 1939, 1919);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        s: &mut Scratch,
    ) {
        if (s.b[2204] && s.b[2205]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1941, 2.0, 812, A::add(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1939), s.ad_value(1919))))));s.store_mul3_lhs(833, 232, 1937, 1941);}
        s.b[2209] = (s.v[233] > 0.0);s.store_scalar(2209, if s.b[2209] { 1.0 } else { 0.0 });
        if (s.b[2204] && s.b[2209]) {s.store_mul_sqrt_mixed_ia(1936, 789, A::offset(A::square(s.ad_value(1867)), 1e-6));}
        s.b[2210] = (s.v[240] < 0.0);s.store_scalar(2210, if s.b[2210] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2209]) && s.b[2210]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 796, 0.5, 1936, 796, 1e-6, (-0.5));}
        if (s.b[2204] && s.b[2209]) {s.store_mul_scale_offset_mixed_ia(1919, 793, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(239), 1.0, s.ad_value(240), s.ad_value(1936), 1.0)), 1.0, (-1.5));}
        s.b[2211] = (s.v[1919] > 0.0);s.store_scalar(2211, if s.b[2211] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2209]) && s.b[2211]) {s.store_offset_mul_offset_rhs_mixed_ia(1937, 1919, A::mul_scaled_output(s.ad_value(1919), A::scale_offset(s.ad_value(1919), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);}
        s.b[2212] = (s.v[1919] > (-230.25850929940458));s.store_scalar(2212, if s.b[2212] { 1.0 } else { 0.0 });
        if (((s.b[2204] && s.b[2209]) && (!s.b[2211])) && s.b[2212]) {s.store_exp(1937, 1919);}
        if (((s.b[2204] && s.b[2209]) && (!s.b[2211])) && (!s.b[2212])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1937, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2204] && s.b[2209]) {s.store_offset(1938, 1935, 3.0);s.store_primal_sub_from_scalar(1939, (-3.0), 230);s.store_scale(1940, 831, 30.0);s.store_scalar(812, (4.0 - 0.9));s.store_add(813, 1938, 1940);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1919, 2.0, 812, A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));s.store_scalar(812, (4.0 - 0.3));s.store_add(813, 1939, 1919);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1941, 2.0, 812, A::add(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1939), s.ad_value(1919))))));s.store_mul3_lhs(834, 233, 1937, 1941);}
        s.b[2213] = (s.v[231] > 0.0);s.store_scalar(2213, if s.b[2213] { 1.0 } else { 0.0 });s.b[2214] = (s.v[1817] <= 0.0);s.store_scalar(2214, if s.b[2214] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2213]) && s.b[2214]) {s.store_offset(1919, 771, 1.0);s.store_div_scaled_product_mixed_aii(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 1843, 1.0);s.store_add_mixed_ai(1921, A::square(s.ad_value(1920)), 1919);s.store_scale(1919, 1920, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        s: &mut Scratch,
    ) {
        if ((s.b[2204] && s.b[2213]) && s.b[2214]) {s.store_div_scaled_product3_mixed_iiia(1846, 1843, 1813, 1919, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0);}
        s.b[2215] = ((s.v[1847] - s.v[1846]) > (-230.25850929940458));s.store_scalar(2215, if s.b[2215] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2213]) && s.b[2215]) {s.store_exp_sub(1919, 1847, 1846);}
        if ((s.b[2204] && s.b[2213]) && (!s.b[2215])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1919, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2204] && s.b[2213]) {s.store_add_scaled_product_mixed_iia(1942, 1922, 1.0, 1812, A::sub_scaled_inputs(s.ad_value(1847), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1919), 1.0), 0.5), 1.0), 1.0);s.store_mul(1943, 230, 1812);s.store_add(1944, 1860, 1943);s.store_scaled_sub_mixed_ia(1945, 1944, A::sqrt_square_offset(A::neg(s.ad_value(1944)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(1936, 789, A::offset(A::square(s.ad_value(1860)), 1e-6));}
        s.b[2216] = (s.v[236] < 0.0);s.store_scalar(2216, if s.b[2216] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2213]) && s.b[2216]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 794, 0.5, 1936, 794, 1e-6, (-0.5));}
        if (s.b[2204] && s.b[2213]) {s.store_add_scaled_product_mixed_iai(1946, 1850, 1.0, A::add_scaled_inputs3(s.ad_value(1945), 1.0, s.ad_value(736), (-1.0), s.ad_value(1942), -1.0), 1813, 1.0);}
        s.b[2217] = (((s.v[1946]) as f64).abs() < 230.25850929940458);s.store_scalar(2217, if s.b[2217] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2213]) && s.b[2217]) {s.store_exp(1947, 1946);}
        s.b[2218] = (s.v[1946] < 0.0);s.store_scalar(2218, if s.b[2218] { 1.0 } else { 0.0 });
        if (((s.b[2204] && s.b[2213]) && (!s.b[2217])) && s.b[2218]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1947, 1e-100, (-230.25850929940458), 1946, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2204] && s.b[2213]) && (!s.b[2217])) && (!s.b[2218])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1947, 1946, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2204] && s.b[2213]) {s.store_mul_scale_offset_mixed_ia(1946, 1813, A::add_scaled_inputs3(s.ad_value(819), 1.0, s.ad_value(1922), 1.0, s.ad_value(1942), -1.0), -1.0, 0.0);}
        s.b[2219] = (((s.v[1946]) as f64).abs() < 230.25850929940458);s.store_scalar(2219, if s.b[2219] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2213]) && s.b[2219]) {s.store_exp(1919, 1946);}
        s.b[2220] = (s.v[1946] < 0.0);s.store_scalar(2220, if s.b[2220] { 1.0 } else { 0.0 });
        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && s.b[2220]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1919, 1e-100, (-230.25850929940458), 1946, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && (!s.b[2220])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1919, 1946, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2204] && s.b[2213]) {s.store_mul(1948, 1947, 1919);s.store_mul_scale_offset_mixed_ia(1919, 791, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(235), 1.0, s.ad_value(236), s.ad_value(1936), 1.0)), 1.0, (-1.5));}
        s.b[2221] = (s.v[1919] > 0.0);s.store_scalar(2221, if s.b[2221] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2213]) && s.b[2221]) {s.store_offset_mul_offset_rhs_mixed_ia(1937, 1919, A::mul_scaled_output(s.ad_value(1919), A::scale_offset(s.ad_value(1919), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);}
        s.b[2222] = (s.v[1919] > (-230.25850929940458));s.store_scalar(2222, if s.b[2222] { 1.0 } else { 0.0 });
        if (((s.b[2204] && s.b[2213]) && (!s.b[2221])) && s.b[2222]) {s.store_exp(1937, 1919);}
        if (((s.b[2204] && s.b[2213]) && (!s.b[2221])) && (!s.b[2222])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1937, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2204] && s.b[2213]) {s.store_mul_ad_product_rhs_mixed_ia(1949, 231, 1937, A::ln(A::div_scaled_offset_numerator(s.ad_value(1947), 1.0, 1.0, A::offset(s.ad_value(1948), 1.0), 1.0)));}
        s.b[2223] = ((s.v[1817] <= 0.0) || ((s.v[235] == 0.0) && (s.v[236] == 0.0)));s.store_scalar(2223, if s.b[2223] { 1.0 } else { 0.0 });
        if ((s.b[2204] && s.b[2213]) && s.b[2223]) {s.store_scalar(1956, 1.0);s.store_scalar(1957, 0.5);}
        if ((s.b[2204] && s.b[2213]) && (!s.b[2223])) {s.store_add_scaled_product_indices(1919, 235, 1.0, 236, 1936, 2.0);s.store_div_scaled_value_by_product_indices(1950, 241, 1.0, 1919, 791, 1.0);s.store_scaled_div(1951, 1848, 1950, 0.5);s.store_div(1952, 1950, 1865);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1953, 1952, 1.0, 1952, 1.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2204] && s.b[2213]) && (!s.b[2223])) {s.store_sub_from_scalar_scaled_input(1954, 0.5, 1953, 3.0);}
        s.b[2224] = (s.v[1951] < 0.001);s.store_scalar(2224, if s.b[2224] { 1.0 } else { 0.0 });
        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && s.b[2224]) {s.store_square(1955, 1951);s.store_offset_mul_ad(1956, s.ad_value(1955), A::add_scaled_product(A::scale_offset(s.ad_value(1952), 0.3333333333333333, 0.16666666666666666), 1.0, s.ad_value(1955), A::scale_offset(s.ad_value(1952), 0.2, 0.05), 0.16666666666666666), 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(1957, 1956, 0.5, 1951, A::mul(s.ad_value(1955), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(1953), 0.25, 0.4), 1.0, s.ad_value(1955), s.ad_value(1953), 0.125, 0.0285714285714)), 1.0, (-0.16666666666666666));}
        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) {s.store_div_from_scalar(1958, 1.0, 1951);}
        s.b[2225] = (((s.v[1951]) as f64).abs() < 230.25850929940458);s.store_scalar(2225, if s.b[2225] { 1.0 } else { 0.0 });
        if ((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && s.b[2225]) {s.store_exp(1959, 1951);}
        s.b[2226] = (s.v[1951] < 0.0);s.store_scalar(2226, if s.b[2226] { 1.0 } else { 0.0 });
        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && s.b[2226]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1959, 1e-100, (-230.25850929940458), 1951, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && (!s.b[2226])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1959, 1951, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) {s.store_div_from_scalar(1960, 1.0, 1959);s.store_sub(1919, 1959, 1960);s.store_add(1921, 1959, 1960);s.store_add_scaled_products_mixed_aiii(1956, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1952), s.ad_value(1919)), 1958, 0.5, 1952, 1921, 0.5);s.store_scaled_sub_ad(1957, A::add_scaled_product(s.ad_value(1956), 1.0, s.ad_value(1919), A::sub(s.ad_value(1953), A::mul3(s.ad_value(1954), s.ad_value(1958), s.ad_value(1958))), (-1.0)), A::mul3(s.ad_value(1954), s.ad_value(1921), s.ad_value(1958)), 0.5);}
        if (s.b[2204] && s.b[2213]) {s.store_scaled_offset_ad(1961, A::div(s.ad_value(1817), A::sqrt_square_offset(s.ad_value(1817), 1e-6)), 1.0, 0.5);s.store_mul3_lhs(1962, 1949, 1956, 1961);s.store_mul3_lhs(836, 1949, 1957, 1961);s.store_sub(835, 1962, 836);s.store_mul_ad_product_rhs_mixed_ia(837, 1949, 1956, A::sub_from_scalar(1.0, s.ad_value(1961)));}
        s.store_scalar(839, 0.0);s.store_scalar(838, 0.0);s.b[2227] = (p.p42 != 0.0);s.store_scalar(2227, if s.b[2227] { 1.0 } else { 0.0 });s.b[2228] = ((s.v[243] > 0.0) && (s.v[1867] < 0.0));s.store_scalar(2228, if s.b[2228] { 1.0 } else { 0.0 });
        if (s.b[2227] && s.b[2228]) {s.store_sqrt_offset_ad(1963, A::add_scaled_square_product(s.ad_value(1867), 1.0, A::square(s.ad_value(249)), A::square(s.ad_value(830)), 1.0), 1e-6);s.store_div_scaled_inputs_indices(1919, 801, -1.0, 1963, 1.0);}
        s.b[2229] = (s.v[1919] > (-230.25850929940458));s.store_scalar(2229, if s.b[2229] { 1.0 } else { 0.0 });
        if ((s.b[2227] && s.b[2228]) && s.b[2229]) {s.store_exp(1921, 1919);}
        if ((s.b[2227] && s.b[2228]) && (!s.b[2229])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1921, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2227] && s.b[2228]) {s.store_mul_ad_affine_product_lhs(839, s.ad_value(799), A::mul3(s.ad_value(830), s.ad_value(1867), s.ad_value(1963)), -1.0, 0.0, 1921);}
        s.b[2230] = ((s.v[242] > 0.0) && (s.v[1866] < 0.0));s.store_scalar(2230, if s.b[2230] { 1.0 } else { 0.0 });
        if (s.b[2227] && s.b[2230]) {s.store_sqrt_offset_ad(1964, A::add_scaled_square_product(s.ad_value(1866), 1.0, A::square(s.ad_value(248)), A::square(s.ad_value(829)), 1.0), 1e-6);s.store_div_scaled_inputs_indices(1919, 800, -1.0, 1964, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2231] = (s.v[1919] > (-230.25850929940458));s.store_scalar(2231, if s.b[2231] { 1.0 } else { 0.0 });
        if ((s.b[2227] && s.b[2230]) && s.b[2231]) {s.store_exp(1921, 1919);}
        if ((s.b[2227] && s.b[2230]) && (!s.b[2231])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1921, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2227] && s.b[2230]) {s.store_mul_ad_affine_product_lhs(838, s.ad_value(798), A::mul3(s.ad_value(829), s.ad_value(1866), s.ad_value(1964)), -1.0, 0.0, 1921);}
        s.store_scalar(1968, s.v[709]);s.store_scalar(1868, 0.0);s.store_scalar(1869, 0.0);s.store_scalar(1870, 0.0);s.store_scalar(1871, 1e-40);s.store_scalar(1872, 1.0);s.store_scalar(840, 0.0);s.b[2232] = ((p.p46 != 0.0) && (s.v[282] > 0.0));s.store_scalar(2232, if s.b[2232] { 1.0 } else { 0.0 });
        if s.b[2232] {s.store_add_scaled_inputs4_mixed_iiai(1919, 822, 0.5, 821, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(758))), (-0.5), 756, 1.0);s.store_add_scaled_inputs4_mixed_iiai(1965, 821, 1.0, 1919, (-0.5), A::sqrt(A::add(A::square(s.ad_value(1919)), s.ad_value(757))), (-(-0.5)), 760, 1.0);s.store_add_scaled_inputs3_indices(1966, 1965, 1.0, 820, 0.5, 824, (-0.5));s.store_mul_ad_product_rhs(1967, 284, A::offset(A::mul(s.ad_value(286), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(285), s.ad_value(1966)), 1.0));s.store_mul_scale_offset_indices(1968, 717, 1967, 1.0, 1.0);s.store_div_from_scalar(1969, 1.0, 1968);s.store_div_scaled_value_offset_denominator(1970, s.ad_value(824), 2.0, A::sqrt_product_offset(s.ad_value(288), s.ad_value(824), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1971, 287, 1970, A::offset(A::mul(s.ad_value(289), s.ad_value(1966)), 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1868, 1969, 823, 1.0, 1971, 1.0, 707, -1.0, 0.0);s.store_mul(1972, 1969, 754);s.store_scaled_ln_ad(1973, A::add(A::div(s.ad_value(1972), s.ad_value(755)), A::sqrt(s.ad_value(1972))), 2.0);s.store_mul(1974, 1969, 1965);s.store_add(1979, 1972, 1974);s.store_add_scaled_product_mixed_iia(1980, 1979, 1.0, 755, A::sqrt(s.ad_value(1979)), 1.0);s.store_add(1981, 1980, 1973);s.store_offset_div_scaled_inputs_sqrt_rhs(1982, 755, 1.0, 1979, 2.0, 1.0);s.store_div_from_scalar(1983, 1.0, 1982);s.store_sub(1984, 1868, 1981);}
        s.b[2233] = (s.v[1984] > (-12.0));s.store_scalar(2233, if s.b[2233] { 1.0 } else { 0.0 });
        if (s.b[2232] && s.b[2233]) {s.store_offset_add(1985, 1984, 719, (-1.0));s.store_scaled_add_mixed_ia(1986, 1985, A::sqrt_square_offset(s.ad_value(1985), 10.0), 0.5);s.store_add_mixed_ai(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);s.store_scaled_add_mixed_ia(1988, 1987, A::sqrt_square_offset(s.ad_value(1987), 2.0), 0.5);}
        s.b[2234] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);s.store_scalar(2234, if s.b[2234] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        s: &mut Scratch,
    ) {
        if ((s.b[2232] && s.b[2233]) && s.b[2234]) {s.store_exp_sub(1989, 1984, 1988);}
        if ((s.b[2232] && s.b[2233]) && (!s.b[2234])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1989, A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2232] && s.b[2233]) {s.store_mul(1990, 718, 1989);s.store_pow_indices(1991, 1990, 1983);s.store_add_scaled_square_product_mixed_iai(1992, 1982, 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), 1991, 1.0);s.store_mul_scale_offset_mixed_ia(1993, 1982, A::div_scaled_inputs2(A::sqrt(s.ad_value(1992)), 1.0, s.ad_value(1982), (-1.0), s.ad_value(1991), 1.0), 1.0, (-1.0));s.store_sub(1975, 1988, 1993);}
        s.b[2235] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));s.store_scalar(2235, if s.b[2235] { 1.0 } else { 0.0 });
        if ((s.b[2232] && (!s.b[2233])) && s.b[2235]) {s.store_exp_ad(1975, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));}
        if ((s.b[2232] && (!s.b[2233])) && (!s.b[2235])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1975, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if s.b[2232] {s.store_mul_add_rhs(1976, 1969, 1845, 1965);}
        s.b[2236] = ((s.v[1975] < 0.001) && (s.v[1845] < 1e-6));s.store_scalar(2236, if s.b[2236] { 1.0 } else { 0.0 });s.b[2237] = (((-s.v[1976]) + s.v[1974]) > (-230.25850929940458));s.store_scalar(2237, if s.b[2237] { 1.0 } else { 0.0 });
        if ((s.b[2232] && s.b[2236]) && s.b[2237]) {s.store_exp_sub(1919, 1974, 1976);}
        if ((s.b[2232] && s.b[2236]) && (!s.b[2237])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1919, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2232] && s.b[2236]) {s.store_mul_scale_offset_indices(1869, 1975, 1919, 1.0, (-1.0));s.store_add(1977, 1869, 1975);}
        if (s.b[2232] && (!s.b[2236])) {s.store_add(1979, 1972, 1976);s.store_add_scaled_product_mixed_iia(1980, 1979, 1.0, 755, A::sqrt(s.ad_value(1979)), 1.0);s.store_add(1981, 1980, 1973);s.store_offset_div_scaled_inputs_sqrt_rhs(1982, 755, 1.0, 1979, 2.0, 1.0);s.store_div_from_scalar(1983, 1.0, 1982);s.store_sub(1984, 1868, 1981);}
        s.b[2238] = (s.v[1984] > (-12.0));s.store_scalar(2238, if s.b[2238] { 1.0 } else { 0.0 });
        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {s.store_offset_add(1985, 1984, 719, (-1.0));s.store_scaled_add_mixed_ia(1986, 1985, A::sqrt_square_offset(s.ad_value(1985), 10.0), 0.5);s.store_add_mixed_ai(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);s.store_scaled_add_mixed_ia(1988, 1987, A::sqrt_square_offset(s.ad_value(1987), 2.0), 0.5);}
        s.b[2239] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);s.store_scalar(2239, if s.b[2239] { 1.0 } else { 0.0 });
        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && s.b[2239]) {s.store_exp_sub(1989, 1984, 1988);}
        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && (!s.b[2239])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1989, A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {s.store_mul(1990, 718, 1989);s.store_pow_indices(1991, 1990, 1983);s.store_add_scaled_square_product_mixed_iai(1992, 1982, 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), 1991, 1.0);s.store_mul_scale_offset_mixed_ia(1993, 1982, A::div_scaled_inputs2(A::sqrt(s.ad_value(1992)), 1.0, s.ad_value(1982), (-1.0), s.ad_value(1991), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {s.store_sub(1977, 1988, 1993);}
        s.b[2240] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));s.store_scalar(2240, if s.b[2240] { 1.0 } else { 0.0 });
        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && s.b[2240]) {s.store_exp_ad(1977, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));}
        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && (!s.b[2240])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1977, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2232] && (!s.b[2236])) {s.store_sub(1869, 1977, 1975);}
        if s.b[2232] {s.store_scaled_add(1870, 1977, 1975, 0.5);}
        if s.b[2232] {
            if ((s.v[1868] - s.v[1870]) > 1e-40) {
                s.store_sub(1871, 1868, 1870);
            } else {
                s.store_scalar(1871, 1e-40);
            }
        }
        if s.b[2232] {s.store_sub_from_scalar_ad(1872, 1.0, A::div_scaled_inputs(s.ad_value(755), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1871), 1.0, s.ad_value(718), 0.25)), 1.0));s.store_div_scaled_product3_mixed_aaii(840, A::mul3_scaled_output(s.ad_value(711), s.ad_value(1968), s.ad_value(1968), -1.0), A::offset(A::mul(s.ad_value(1872), s.ad_value(1870)), 1.0), 1869, 1.0, 1857, 1.0);}
        s.store_scalar(1873, 0.0);s.store_scalar(841, 0.0);s.b[2241] = ((s.v[1817] > 0.0) && (p.p41 != 0.0));s.store_scalar(2241, if s.b[2241] { 1.0 } else { 0.0 });
        if s.b[2241] {s.store_add_scaled_product_indices(1978, 820, 1.0, 227, 1848, (-1.0));}
        s.b[2242] = (s.v[1978] > 0.0);s.store_scalar(2242, if s.b[2242] { 1.0 } else { 0.0 });
        if (s.b[2241] && s.b[2242]) {s.store_mul_div_scaled_offset_numerator_rhs(1921, 706, A::mul(s.ad_value(228), A::sub(A::sqrt(A::add(s.ad_value(722), s.ad_value(1922))), s.ad_value(730))), 1.0, 1.0, A::offset(s.ad_value(1978), 1e-30), 1.0);}
        s.b[2243] = ((((-s.v[1921])) as f64).abs() < 230.25850929940458);s.store_scalar(2243, if s.b[2243] { 1.0 } else { 0.0 });
        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {s.store_exp_neg_input(1919, 1921);}
        s.b[2244] = ((-s.v[1921]) < 0.0);s.store_scalar(2244, if s.b[2244] { 1.0 } else { 0.0 });
        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && s.b[2244]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1919, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1921)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && (!s.b[2244])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1919, A::neg(s.ad_value(1921)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2241] && s.b[2242]) {s.store_mul3_lhs(1873, 224, 1978, 1919);s.store_mul_add_rhs(841, 1873, 832, 840);}
        s.b[2245] = (s.v[841] > (0.5 * s.v[229]));s.store_scalar(2245, if s.b[2245] { 1.0 } else { 0.0 });
        if ((s.b[2241] && s.b[2242]) && s.b[2245]) {s.store_offset_div_scaled_inputs_indices(1919, 841, 2.0, 229, 1.0, (-1.0));s.store_mul_scaled_offset_ad_rhs(841, 229, 0.5, A::div(s.ad_value(1919), A::sqrt_square_offset(s.ad_value(1919), 1.0)), 1.0);}
        s.b[2439] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));s.store_scalar(2439, if s.b[2439] { 1.0 } else { 0.0 });s.b[2440] = ((p.p45 > 0.0) || (p.p47 > 0.0));s.store_scalar(2440, if s.b[2440] { 1.0 } else { 0.0 });
        if (s.b[2439] && s.b[2440]) {s.copy_ad(2280, 722);s.copy_ad(2281, 732);s.copy_ad(2282, 723);s.copy_ad(2283, 1808);s.copy_ad(2284, 1809);s.store_scalar(2288, 0.0);}
        s.b[2441] = (p.p47 > 0.0);s.store_scalar(2441, if s.b[2441] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        s: &mut Scratch,
    ) {
        if ((s.b[2439] && s.b[2440]) && s.b[2441]) {s.store_add_scaled_inputs4_mixed_iiai(2283, 822, 0.5, 821, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(743))), (-0.5), 741, 1.0);s.store_add_scaled_inputs4_mixed_iiai(1874, 821, 1.0, 2283, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2283)), s.ad_value(742))), (-(-0.5)), 744, 1.0);s.copy_ad(2284, 1874);s.copy_ad(2280, 739);s.copy_ad(2281, 742);s.copy_ad(2282, 740);}
        if (s.b[2439] && s.b[2440]) {s.store_add_scaled_inputs3_indices(2287, 823, 1.0, 2288, (-1.0), 694, -1.0);s.store_add_scaled_inputs3_indices(2289, 2284, 1.0, 820, 0.5, 824, (-0.5));s.store_scalar(2301, 1.0);}
        s.b[2442] = (s.v[185] > 0.0);s.store_scalar(2442, if s.b[2442] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2440]) && s.b[2442]) {s.store_primal_scale(2292, 2280, s.v[355]);s.store_scale(2293, 2289, s.v[355]);s.store_scale(2294, 2287, s.v[355]);s.store_offset_div_scaled_inputs_sqrt_rhs(1920, 2282, 0.5, 2292, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(1921, 2292, 1.0, 2282, A::sqrt(s.ad_value(2292)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2295, A::div_scaled_inputs2(s.ad_value(2294), 1.0, s.ad_value(1921), (-1.0), s.ad_value(1920), 1.0), 1.0, 2292, 0.5, A::offset(s.ad_value(186), 1.0), 2293, (-1.0));s.store_primal_offset_scaled(2296, 2292, 0.5, 2.0);s.store_add(2297, 2292, 2293);s.store_sub_scaled_inputs_ad(1920, A::add_scaled_inputs_product(s.ad_value(2294), 1.0, s.ad_value(2297), (-1.0), s.ad_value(2282), A::sqrt(s.ad_value(2297)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2292), s.ad_value(2282)), A::sqrt(s.ad_value(2292)))), 2.0);s.store_add_scaled_inputs(2298, 1920, 2.0, 2296, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2295, 0.5, 2298, 0.5, 2295, 2298, 20.0, 0.5);s.store_add_scaled_inputs3_indices(1921, 2294, 2.0, 2293, (-2.0), 2296, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2299, 1920, 0.5, 1921, 0.5, 1920, 1921, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2299, 0.5, 2296, 0.5, 2299, 2296, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2300, 1920, 0.5, 2296, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2296), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(1921, 696, A::div(s.ad_value(2300), s.ad_value(2296)), 1.0, 1.0);}
        s.b[2443] = (s.v[1921] > (-230.25850929940458));s.store_scalar(2443, if s.b[2443] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && s.b[2443]) {s.store_exp(2301, 1921);}
        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && (!s.b[2443])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2301, 1e-100, (-230.25850929940458), 1921, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2439] && s.b[2440]) {s.store_offset_mul(2302, 695, 2301, 1.0);s.store_scale(2303, 2302, s.v[709]);s.store_mul_ad_product_rhs(2304, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2289)), 1.0));s.store_mul_scale_offset_indices(2305, 2303, 2304, 1.0, 1.0);s.store_div_from_scalar(2306, 1.0, 2305);s.store_mul_mixed_ia(2290, 2282, A::sqrt_scaled_input(s.ad_value(2306), s.v[709]));s.store_square(2291, 2290);s.store_div_from_scalar(2307, 1.0, 2291);s.store_mul(2308, 2284, 2306);s.store_mul(2309, 2287, 2306);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2439] && s.b[2440]) {s.store_div_scaled_value_offset_denominator(2310, s.ad_value(824), 2.0, A::sqrt_product_offset(s.ad_value(192), s.ad_value(824), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2311, 191, 2310, A::offset(A::mul(s.ad_value(193), s.ad_value(2289)), 1.0));s.store_mul(2312, 2280, 2306);s.store_sqrt_square_add(1920, 2283, 2281);s.store_sqrt_add_ad(1921, A::square(A::sub(s.ad_value(2283), s.ad_value(2311))), s.ad_value(2281));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2313, 2306, 2311, 0.5, 1920, 0.5, 1921, ((-1.0) * (0.5)), 0.0);s.store_add(2314, 2312, 2308);s.store_sub(2315, 2314, 2313);}
        s.b[2444] = (p.p45 > 0.0);s.store_scalar(2444, if s.b[2444] { 1.0 } else { 0.0 });s.b[2445] = (((s.v[2315]) as f64).abs() < 1e-5);s.store_scalar(2445, if s.b[2445] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && s.b[2445]) {s.store_offset_ad(2316, A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2315), 1.0, A::scale(s.ad_value(2315), 0.3125), 0.5)), 1.0);}
        s.b[2446] = (s.v[2315] < 460.51701859880916);s.store_scalar(2446, if s.b[2446] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && s.b[2446]) {s.store_exp_neg_input(2330, 2315);}
        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && (!s.b[2446])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2330, 1e-200, 2315, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {s.store_scalar(1919, (if (s.v[2315] > 0.0) { 1.0 } else { (-1.0) }));}
        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {s.store_offset_ad(2316, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2290), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2315))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2315), 1.0, s.ad_value(2330))), 2.0), 1.0);}
        if ((s.b[2439] && s.b[2440]) && (!s.b[2444])) {s.store_offset_div_scaled_inputs_sqrt_rhs(2316, 2290, 0.5, 2315, 1.0, 1.0);}
        if (s.b[2439] && s.b[2440]) {s.store_add_scaled_value_products_mixed_iiaia(2317, 2315, 1.0, 2290, A::sqrt(s.ad_value(2315)), 1.0, 2316, A::ln(A::offset(s.ad_value(2316), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2318, 2309, 1.0, 2317, (-1.0), 2316, 1.0);s.store_mul_scaled_offset_ad_rhs(2324, 2291, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2291)), 1.0)), (-1.0));s.store_scalar(2323, 0.0);s.store_scalar(2325, 1.0);}
        s.b[2447] = (s.v[2318] > (-30.0));s.store_scalar(2447, if s.b[2447] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {s.store_offset_mul(2319, 2316, 2318, (-1.0));s.store_scaled_add_mixed_ia(1919, 2319, A::sqrt_square_offset(s.ad_value(2319), 10.0), 0.5);s.store_sub_mixed_ia(2320, 2318, A::ln(s.ad_value(1919)));s.store_scaled_add_mixed_ia(2321, 2320, A::sqrt_square_offset(s.ad_value(2320), 2.0), 0.5);}
        s.b[2448] = ((s.v[2318] - s.v[2321]) < 230.25850929940458);s.store_scalar(2448, if s.b[2448] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2448]) {s.store_exp_sub(1919, 2318, 2321);}
        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2448])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1919, A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {s.store_div(2322, 1919, 2316);s.store_sub_mixed_ai(1919, A::scaled_offset(s.ad_value(2321), 1.0, 2.0), 2322);}
        s.b[2449] = (s.v[2322] > 1e-6);s.store_scalar(2449, if s.b[2449] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_107(
        s: &mut Scratch,
    ) {
        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2449]) {s.store_mul_scale_offset_mixed_ia(2323, 2316, A::sub(s.ad_value(2321), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2322), s.ad_value(1919), 1.0), 1.0, (-1.0), s.ad_value(2322), 1.0)), 1.0, 1.0);}
        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2449])) {s.store_mul_ad_affine_product_rhs(2323, 2316, s.ad_value(2322), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);}
        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {s.store_add_scaled_inputs3_offset_mixed_iia(1919, 2309, 0.5, 2323, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));s.store_mul_scaled_offset_ad_rhs(2324, 2291, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2291)), s.ad_value(1919), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2325, 2324, 2324, 1.0, 2323, 1.0);s.store_add_scaled_product_indices(2315, 2314, 1.0, 2325, 2313, (-1.0));}
        if (s.b[2439] && s.b[2440]) {s.store_offset_scaled(2326, 2290, 0.7071067811865475, 1.0);}
        let (t1,) = {
    if (s.b[2439] && s.b[2440]) {
        let t0: f64 = (1e-5 * s.v[2326]);
        (t0,)
    } else {
        (s.v[2327],)
    }
};
        s.store_scalar(2327, t1);
        if (s.b[2439] && s.b[2440]) {s.store_div_from_scalar(2328, 1.0, 2326);s.store_scalar(2435, 0.0);s.store_scalar(2329, 0.0);}
        s.b[2450] = (s.v[2315] < 460.51701859880916);s.store_scalar(2450, if s.b[2450] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2440]) && s.b[2450]) {s.store_exp_neg_input(2330, 2315);}
        if ((s.b[2439] && s.b[2440]) && (!s.b[2450])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2330, 1e-200, 2315, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2451] = (((s.v[2309]) as f64).abs() <= s.v[2327]);s.store_scalar(2451, if s.b[2451] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2440]) && s.b[2451]) {s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2329, 2309, 2328, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2330)), s.ad_value(2290), s.ad_value(2415)), 1.0));}
        s.b[2452] = (s.v[2309] < (-s.v[2327]));s.store_scalar(2452, if s.b[2452] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {s.store_neg(2417, 2309);s.store_scaled_mul(2418, 2417, 2328, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2419, 2418, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2414, 2417, 2419);s.store_add_scaled_square_product_mixed_iia(2420, 2414, 1.0, 2291, A::offset(s.ad_value(2419), 1.0), 1.0);s.store_sub_scaled_inputs(2421, 2414, 2.0, 2291, 1.0);s.store_sub_ln_mul_lhs(2422, 2420, 2307, 2419);s.store_add(818, 2420, 2421);s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.5, s.ad_value(2420), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_108(
        s: &mut Scratch,
    ) {
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {s.store_add_mixed_ia(2423, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.3333333333333333, s.ad_value(2420), 1.0))), 1.0));}
        s.b[2453] = (s.v[2423] < 230.25850929940458);s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && s.b[2453]) {s.store_exp(2424, 2423);}
        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && (!s.b[2453])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2424, 2423, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {s.store_div_from_scalar(2425, 1.0, 2424);s.store_div_from_scalar_offset_square(2414, 1.0, 2423, 2.0);s.store_mul_square_lhs(2426, 2423, 2414);s.store_mul3_affine_lhs(2427, 2423, 2414, 4.0, 0.0, 2414);s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), 2414, 2414);s.store_sub(2414, 2417, 2423);s.store_mul(2415, 2330, 2425);s.store_add_scaled_product_mixed_iia(2429, 2414, 2.0, 2291, A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2415), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2427)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2423), (-1.0), s.ad_value(2415), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::sub(A::offset(s.ad_value(2423), (-1.0)), s.ad_value(2426)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2414, 2.0, 2291, A::add_scaled_inputs_product(s.ad_value(2424), 1.0, s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2329, 2423, -1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);}
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {s.store_div_from_scalar_offset_scaled_input(2431, 1.0, 2290, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2432, 2431, A::mul_scaled_lhs(s.ad_value(2326), 1.25, s.ad_value(2431)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(2433, 2309, 2328, A::offset(A::mul(s.ad_value(2432), s.ad_value(2309)), 1.0));}
        s.b[2454] = ((-s.v[2433]) > (-230.25850929940458));s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2454]) {s.store_exp_neg_input(2414, 2433);}
        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2454])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2414, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2433)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {s.store_sub_from_scalar(2434, 1.0, 2414);s.store_add_scaled_inputs_product_mixed_iiia(2435, 2309, 1.0, 2291, 0.5, 2290, A::sqrt(A::add_scaled_inputs3(s.ad_value(2309), 1.0, s.ad_value(2291), 0.25, s.ad_value(2434), -1.0)), (-1.0));s.store_offset(2436, 2315, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_109(
        s: &mut Scratch,
    ) {
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {s.store_sub_ad(2419, A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2435), s.ad_value(2436)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2436), 0.5, A::sqrt_square_offset(s.ad_value(2436), 5.0), 0.5));s.store_sub(2414, 2309, 2419);s.store_exp_neg_input(2415, 2419);s.store_div_from_scalar_offset_square(2416, 1.0, 2419, 2.0);s.store_mul_square_lhs(2426, 2419, 2416);s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), 2416, 2416);}
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2330] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                s.store_scalar(2420, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2420, 2414, 1.0, 2291, A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            }
        }
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2437, 1.0, 2291, A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2421, 2414, 2.0, 2291, A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2422, 2315, 1.0, 2419, (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0);s.store_add(818, 2420, 2421);s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0);s.store_add_mixed_ia(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));}
        s.b[2455] = (s.v[2438] < 230.25850929940458);s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2455]) {s.store_exp(2424, 2438);s.store_div_from_scalar(2425, 1.0, 2424);s.store_mul(2424, 2330, 2424);}
        s.b[2456] = (s.v[2438] > (s.v[2315] - 230.25850929940458));s.store_scalar(2456, if s.b[2456] { 1.0 } else { 0.0 });
        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && s.b[2456]) {s.store_exp_sub(2424, 2438, 2315);s.store_div(2425, 2330, 2424);}
        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && (!s.b[2456])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2424, 1e-100, A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2425, 1e-100, 2438, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {s.store_div_from_scalar_offset_square(2414, 1.0, 2438, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_110(
        s: &mut Scratch,
    ) {
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {s.store_mul_square_lhs(2426, 2438, 2414);s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), 2414, 2414);s.store_sub(2414, 2309, 2438);s.store_add_scaled_product_mixed_iia(2429, 2414, 2.0, 2291, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2414, 2.0, 2291, A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));s.store_add_scaled_inputs_mixed_ia(2329, 2438, 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);}
        if (s.b[2439] && s.b[2440]) {s.store_scalar(2332, 0.0);s.store_scalar(2333, 0.0);s.store_scalar(2334, 0.0);s.store_scalar(2335, 0.0);s.store_scalar(2336, 0.0);s.store_scalar(2337, 0.0);s.store_scalar(2338, 0.0);s.store_scalar(2339, 1.0);s.store_scalar(2340, 1.0);s.store_sub(2341, 2309, 2329);s.store_scalar(2342, 0.0);s.store_mul(2343, 2305, 2341);s.store_scalar(2344, 1.0);s.store_scalar(2345, 1.0);s.store_scalar(2349, 1.0);s.store_scalar(2350, 1.0);s.store_scalar(2352, 1.0);}
        s.b[2457] = (s.v[2309] > 0.0);s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {s.store_div_from_scalar_offset_square(1919, 1.0, 2329, 2.0);s.store_mul_square_lhs(2331, 2329, 1919);s.store_mul3_affine_lhs(2332, 2329, 1919, 4.0, 0.0, 1919);s.store_mul_ad_product_lhs_mixed_ai(2333, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2331), 12.0), 1919, 1919);s.store_scalar(2334, 0.0);}
        s.b[2458] = (s.v[2329] < 230.25850929940458);s.store_scalar(2458, if s.b[2458] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2458]) {s.store_exp(2334, 2329);s.store_div_from_scalar(2335, 1.0, 2334);s.store_mul(2334, 2330, 2334);}
        s.b[2459] = (s.v[2329] > (s.v[2315] - 230.25850929940458));s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && s.b[2459]) {s.store_exp_sub(2334, 2329, 2315);s.store_div(2335, 2330, 2334);}
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2459])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2334, 1e-100, A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2335, 1e-100, 2329, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {s.store_add_scaled_product_mixed_iia(2336, 2334, 1.0, 2330, A::add(A::offset(s.ad_value(2329), 1.0), s.ad_value(2331)), (-1.0));}
        s.b[2460] = (s.v[2329] < 1e-5);s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2460]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2337, 2329, 1.0, 2329, 1.0, 2329, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2336, A::mul3(s.ad_value(2330), s.ad_value(2329), s.ad_value(2329)), 2329, A::scale_offset(s.ad_value(2329), 1.75, 1.0), 0.16666666666666666);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_111(
        s: &mut Scratch,
    ) {
        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2460]) {s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.25), 0.3333333333333333));s.store_scaled_mul(2338, 2329, 1919, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2339, 2290, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.5)), 1.0, A::square(s.ad_value(2329)), 0.16666666666666666), 0.7071067811865475, 1919, 1.0, 1.0);}
        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2460])) {s.store_add_offset_lhs(2337, 2329, (-1.0), 2335);s.store_sqrt(2338, 2337);s.store_offset_scaled_ad(2339, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2335)), s.ad_value(2338)), 0.5, 1.0);}
        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {s.store_div_scaled_offset_numerator(2340, A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2289)), 1.0, 1.0, A::offset(A::mul(s.ad_value(702), s.ad_value(2289)), 1.0), 1.0);}
        s.b[2461] = (s.v[2336] > 1e-100);s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {s.store_mul_sqrt_mixed_ia(2341, 2290, A::add(s.ad_value(2337), s.ad_value(2336)));s.store_div_scaled_product3_mixed_iiia(2342, 2291, 2336, 2305, 1.0, A::add_scaled_product(s.ad_value(2341), 1.0, s.ad_value(2290), s.ad_value(2338), 1.0), 1.0);s.store_mul3_lhs(2343, 2338, 2290, 2305);}
        s.b[2462] = (s.v[212] < 0.0);s.store_scalar(2462, if s.b[2462] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2462]) {s.store_div_from_scalar_sub_from_scalar_ad(2344, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2289)));}
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2462])) {s.store_offset_mul(2344, 212, 2289, 1.0);}
        s.b[2463] = (s.v[213] < 0.0);s.store_scalar(2463, if s.b[2463] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2463]) {s.store_sub_from_scalar_scaled_mul(2345, 1.0, 213, 2342, 1.0);}
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2463])) {s.store_div_from_scalar_offset_product(2345, 1.0, 213, 2342, 1.0);}
        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {s.store_mul_product3_indices(2346, 2342, 751, 2344, 2345, 1.0);s.store_mul_add_scaled_product_rhs_indices(2347, 768, 2343, 1.0, 769, 2342, 1.0);s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2337), 1.0, A::add(s.ad_value(2337), s.ad_value(2336)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2348, A::pow(A::mul(s.ad_value(2347), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);s.store_mul_add_mixed_iai(2349, 2340, A::offset(s.ad_value(2348), 1.0), 2346);}
        s.b[2464] = (s.v[216] < 0.0);s.store_scalar(2464, if s.b[2464] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2464]) {s.store_div_from_scalar_sub_from_scalar_ad(2350, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2289)));}
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2464])) {s.store_offset_mul(2350, 216, 2289, 1.0);}
        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {s.store_mul(1921, 2342, 2350);s.store_div_add_scaled_inputs_rhs_indices(2351, 1921, 218, 1.0, 1921, 1.0);}
        s.b[2465] = (s.v[217] < 0.0);s.store_scalar(2465, if s.b[2465] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2465]) {s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));}
        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2465])) {s.store_offset_mul(2352, 217, 2351, 1.0);}
        if (s.b[2439] && (!s.b[2440])) {s.copy_ad(2287, 1810);s.copy_ad(2289, 1811);s.copy_ad(2305, 1812);s.copy_ad(2306, 1813);s.copy_ad(2290, 1814);s.copy_ad(2291, 1815);s.copy_ad(2307, 1816);}
    }
}
