#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_192(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1901])) {s.store_div_from_scalar_offset_product(170, 1.0, 873, 227, 1.0);s.store_mul3_lhs(368, 168, 169, 170);s.store_add(369, 370, 368);s.store_sub(371, 227, 369);s.store_add_ad(167, A::add_scaled_product(s.ad_value(868), 1.0, s.ad_value(867), s.ad_value(371), 1.0), A::mul3(s.ad_value(659), s.ad_value(371), s.ad_value(371)));s.store_sqrt_square_offset(168, 167, 1e-10);let t0: A = A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)));s.store_neg_ad(372, A::offset(A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, t0), (((-(-10.0))) + ((-p.p645)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(865), -1.0, t0), (((-(-10.0))) + ((-p.p645)))), (-((4.0 * (-10.0)) * p.p645))), 0.5), (-10.0)));s.store_mul(376, 372, 380);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_add_scaled_inputs(167, 878, 1.0 / (s.v[184]), 877, (s.v[184] * 1.0 / (s.v[184])));s.store_mul_scale_offset_rhs(378, 880, 639, p.p666, (((((-1.0)) * (p.p666))) + (1.0)));}
        s.b[1902] = (s.v[211] > 0.0);s.store_scalar(1902, if s.b[1902] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1902]) {s.store_sub(168, 378, 499);}
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1902])) {s.store_sub(168, 378, 498);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_offset(169, 881, (-1.0));}
        s.b[1903] = (s.v[168] > 0.0);s.store_scalar(1903, if s.b[1903] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && s.b[1903]) {s.store_mul_scaled_pow_ad_rhs(170, 879, -1.0, s.ad_value(168), s.ad_value(169));}
        if ((((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) && (!s.b[1903])) {s.store_scalar(170, 0.0);}
        if (((!s.b[1620]) && (!s.b[1896])) && (!s.b[1899])) {s.store_limited_exp(171, 170);s.store_mul_ad_product_lhs_mixed_ai(377, A::mul3(s.ad_value(167), s.ad_value(211), s.ad_value(579)), 168, 171);s.store_add(373, 376, 377);}
        if (!s.b[1620]) {s.store_mul(1095, 373, 379);s.store_add_scaled_offset_product_rhs(810, 810, 1.0, 813, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(816, 816, 1.0, 814, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(819, 819, 1.0, 815, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(884, 884, 1.0, 886, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(882, 882, 1.0, 887, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(888, 888, 1.0, 891, 639, (-1.0), 1.0);s.store_scalar(477, 0.0);s.store_scalar(479, 0.0);s.store_scalar(480, 0.0);s.store_scalar(483, 0.0);s.store_scalar(484, 0.0);}
        s.b[1904] = ((p.p37 != 0.0) || (p.p38 != 0.0));s.store_scalar(1904, if s.b[1904] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1904]) {s.store_mul_add_scaled_inputs4_indices_rhs(469, 269, 213, 1.0, 254, (-1.0), 400, 1.0, 320, 1.0);s.store_sqrt_square_offset(168, 469, 0.0001);s.store_scaled_sub(471, 168, 469, 0.5);s.store_scaled_add(470, 469, 168, 0.5);}
        s.b[1905] = (p.p38 != 0.0);s.store_scalar(1905, if s.b[1905] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 469, 1.0 / (p.p671));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale_ad(474, {
                if ((!((-s.v[168]) > 37.0)) && (!((-s.v[168]) < (-37.0)))) {
                    A::ln_one_plus_exp(A::neg(s.ad_value(168)))
                } else {
                    {
                        if ((!((-s.v[168]) > 37.0)) && ((-s.v[168]) < (-37.0))) {
                            A::exp_scaled_input(s.ad_value(168), -1.0)
                        } else {
                            {
                                if ((-s.v[168]) > 37.0) {
                                    A::neg(s.ad_value(168))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, p.p671);
        }
        s.b[1906] = (p.p696 != 0.0);s.store_scalar(1906, if s.b[1906] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1906]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 471, 1.0 / (p.p696));}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1906])) {s.store_scalar(167, 1.0);}
        s.b[1907] = (s.v[167] < 0.01);s.store_scalar(1907, if s.b[1907] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1907]) {s.store_scalar(167, 0.01);}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));s.store_scalar(169, (p.p701 * p.p76));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(882), 1.0, s.ad_value(883), s.ad_value(471), (-1.0)), 1.0, 167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_193(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_limited_exp(171, 170);s.store_mul_product3_indices(476, 171, 168, 221, 474, 1.0);s.store_mul(476, 476, 662);s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {
            s.store_scale_ad(473, {
                if ((!(s.v[168] > 37.0)) && (!(s.v[168] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(168))
                } else {
                    {
                        if ((!(s.v[168] > 37.0)) && (s.v[168] < (-37.0))) {
                            A::exp(s.ad_value(168))
                        } else {
                            {
                                if (s.v[168] > 37.0) {
                                    s.ad_value(168)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, p.p671);
        }
        s.b[1908] = (p.p697 != 0.0);s.store_scalar(1908, if s.b[1908] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1908]) {s.store_sub_from_scalar_scaled_input(167, 1.0, 470, 1.0 / (p.p697));}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && (!s.b[1908])) {s.store_scalar(167, 1.0);}
        s.b[1909] = (s.v[167] < 0.01);s.store_scalar(1909, if s.b[1909] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1905]) && s.b[1909]) {s.store_scalar(167, 0.01);}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1905]) {s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));s.store_scalar(169, (p.p699 * p.p76));s.store_div_scaled_product_mixed_iai(170, 169, A::add_scaled_product(s.ad_value(884), 1.0, s.ad_value(885), s.ad_value(470), (-1.0)), 1.0, 167, 1.0);s.store_limited_exp(171, 170);s.store_mul_product3_indices(475, 171, 168, 221, 473, 1.0);s.store_mul(475, 475, 662);s.store_scaled_add(477, 476, 475, p.p2);s.store_offset_mul(478, 212, 269, p.p1383);}
        s.b[1910] = (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0));s.store_scalar(1910, if s.b[1910] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_mul_voltage_ad(208, s.ad_value(379), ctx, nodes, Some(8), Some(11));s.store_sub(167, 208, 478);s.store_sqrt_square_offset(168, 167, 0.0001);s.store_offset_scaled_sub(209, 168, 167, 0.5, (((-0.01)) * (0.5)));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1910]) {s.store_mul(169, 208, 209);s.store_add_scaled_product_indices(170, 889, (-1.0), 888, 890, 1.0);s.store_mul(171, 889, 890);s.store_mul_sub_scaled_inputs_rhs(172, 179, A::add_scaled_product(s.ad_value(888), 1.0, s.ad_value(170), s.ad_value(209), 1.0), (-p.p76), A::mul3(s.ad_value(171), s.ad_value(209), s.ad_value(209)), (-p.p76));s.store_limited_exp(173, 172);s.store_scaled_mul(178, 178, 492, p.p1380);s.store_mul_product3_indices(210, 662, 178, 169, 173, 1.0);}
        if (((!s.b[1620]) && s.b[1904]) && (!s.b[1910])) {s.store_scalar(210, 0.0);}
        s.b[1911] = (p.p37 != 0.0);s.store_scalar(1911, if s.b[1911] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_add_scaled_product_indices(168, 810, 1.0, 811, 470, (-1.0));s.store_offset_mul(169, 812, 470, 1.0);s.store_scaled_mul(170, 168, 169, s.v[488]);s.store_mul_product3_mixed_aiia(171, A::limited_exp(s.ad_value(170)), 253, 269, A::add(s.ad_value(400), s.ad_value(320)), 1.0);s.store_mul_product3_mixed_iiia(481, 662, 487, 171, A::add_scaled_inputs4(s.ad_value(221), 1.0, s.ad_value(227), 0.5, s.ad_value(224), (-0.5), s.ad_value(223), (-0.5)), p.p2);s.store_offset_sqrt_ad(472, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));s.store_scale(168, 472, s.v[823]);s.store_limited_exp_neg_input(482, 168);s.store_offset_add(170, 168, 482, (((-1.0)) + (0.0001)));s.store_offset_sub_from_scalar_ad(171, 1.0, A::mul_offset_lhs(s.ad_value(168), 1.0, s.ad_value(482)), 0.0001);s.store_offset_square(172, 168, 0.0002);}
        s.b[1912] = (s.v[211] > 0.0);s.store_scalar(1912, if s.b[1912] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1912]) {s.store_div_scaled_product_indices(480, 481, 171, 1.0, 172, 1.0);s.store_div_scaled_product_indices(479, 481, 170, 1.0, 172, 1.0);}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1912])) {s.store_div_scaled_product_indices(479, 481, 171, 1.0, 172, 1.0);s.store_div_scaled_product_indices(480, 481, 170, 1.0, 172, 1.0);}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_sub(169, 203, 219);s.store_sqrt_square_offset(228, 169, 0.0001);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_194(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1913] = (p.p1295 == 1.0);s.store_scalar(1913, if s.b[1913] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) {s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(816), 1.0, s.ad_value(817), s.ad_value(228), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1914] = (s.v[818] < 0.01);s.store_scalar(1914, if s.b[1914] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1913]) && s.b[1914]) {s.store_scalar(818, 0.01);}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1913])) {s.store_add_scaled_product_indices(168, 816, 1.0, 817, 228, (-1.0));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_offset_mul(169, 818, 228, 1.0);s.store_mul3_lhs(170, 491, 168, 169);s.store_limited_exp(171, 170);s.store_mul3_affine_lhs(485, 662, 489, p.p2, 0.0, 824);s.store_mul_product3_indices(483, 171, 485, 203, 228, 1.0);s.store_sub(169, 204, 219);s.store_sqrt_square_offset(229, 169, 0.0001);}
        s.b[1915] = (p.p1295 == 1.0);s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) {s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(820), s.ad_value(229), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1916] = (s.v[821] < 0.01);s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && s.b[1915]) && s.b[1916]) {s.store_scalar(821, 0.01);}
        if ((((!s.b[1620]) && s.b[1904]) && s.b[1911]) && (!s.b[1915])) {s.store_add_scaled_product_indices(168, 819, 1.0, 820, 229, (-1.0));}
        if (((!s.b[1620]) && s.b[1904]) && s.b[1911]) {s.store_offset_mul(169, 821, 229, 1.0);s.store_mul3_lhs(170, 491, 168, 169);s.store_limited_exp(171, 170);s.store_mul3_affine_lhs(486, 662, 490, p.p2, 0.0, 825);s.store_mul_product3_indices(484, 171, 486, 204, 229, 1.0);}
        if (!s.b[1620]) {s.store_mul(1098, 379, 483);s.store_mul(1099, 379, 484);s.store_mul(1102, 379, 477);s.store_mul(1100, 379, 479);s.store_mul(1101, 379, 480);s.store_mul(502, 666, 463);s.store_mul(505, 667, 494);s.store_scale(508, 671, (s.v[189] * p.p2));s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));}
        s.b[1917] = (p.p913 == 1.0);s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1917]) {s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1917])) {s.store_primal_offset_scaled_ad(504, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913))), (-(1.0 / (1.0 - p.p913))), (1.0 / (1.0 - p.p913)));}
        if (!s.b[1620]) {s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));}
        s.b[1918] = (p.p915 == 1.0);s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1918]) {s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1918])) {s.store_primal_offset_scaled_ad(507, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915))), (-(1.0 / (1.0 - p.p915))), (1.0 / (1.0 - p.p915)));}
        if (!s.b[1620]) {s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));}
        s.b[1919] = (p.p917 == 1.0);s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1919]) {s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1919])) {s.store_primal_offset_scaled_ad(510, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917))), (-(1.0 / (1.0 - p.p917))), (1.0 / (1.0 - p.p917)));}
        s.b[1920] = (s.v[502] > 0.0);s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1920]) {s.store_div(168, 498, 672);}
        s.b[1921] = (s.v[168] < 0.9);s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1920]) && s.b[1921]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1922] = (p.p913 != 1.0);s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });s.b[1923] = (p.p913 == 0.5);s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && s.b[1923]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p913));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_195(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && s.b[1922]) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p913)), 0.0);}
        if ((((!s.b[1620]) && s.b[1920]) && s.b[1921]) && (!s.b[1922])) {s.store_mul_ad_affine_product_rhs(521, 672, s.ad_value(502), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1920]) && (!s.b[1921])) {s.store_mul_ad_product_rhs(169, 503, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p913), (((((-1.0)) * ((5.0 * p.p913)))) + ((1.0 + p.p913)))));s.store_mul_ad_product_rhs_mixed_ia(521, 672, 502, A::add(s.ad_value(169), s.ad_value(504)));}
        if ((!s.b[1620]) && (!s.b[1920])) {s.store_scalar(521, 0.0);}
        s.b[1924] = (s.v[505] > 0.0);s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1924]) {s.store_div(168, 498, 673);}
        s.b[1925] = (s.v[168] < 0.9);s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1924]) && s.b[1925]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1926] = (p.p915 != 1.0);s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });s.b[1927] = (p.p915 == 0.5);s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && s.b[1927]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p915));}
        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && s.b[1926]) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p915)), 0.0);}
        if ((((!s.b[1620]) && s.b[1924]) && s.b[1925]) && (!s.b[1926])) {s.store_mul_ad_affine_product_rhs(522, 673, s.ad_value(505), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1924]) && (!s.b[1925])) {s.store_mul_ad_product_rhs(169, 506, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p915), (((((-1.0)) * ((5.0 * p.p915)))) + ((1.0 + p.p915)))));s.store_mul_ad_product_rhs_mixed_ia(522, 673, 505, A::add(s.ad_value(169), s.ad_value(507)));}
        if ((!s.b[1620]) && (!s.b[1924])) {s.store_scalar(522, 0.0);}
        s.b[1928] = (s.v[508] > 0.0);s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1928]) {s.store_div(168, 498, 674);}
        s.b[1929] = (s.v[168] < 0.9);s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1928]) && s.b[1929]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1930] = (p.p917 != 1.0);s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });s.b[1931] = (p.p917 == 0.5);s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && s.b[1931]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p917));}
        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && s.b[1930]) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p917)), 0.0);}
        if ((((!s.b[1620]) && s.b[1928]) && s.b[1929]) && (!s.b[1930])) {s.store_mul_ad_affine_product_rhs(523, 674, s.ad_value(508), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1928]) && (!s.b[1929])) {s.store_mul_ad_product_rhs(169, 509, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p917), (((((-1.0)) * ((5.0 * p.p917)))) + ((1.0 + p.p917)))));s.store_mul_ad_product_rhs_mixed_ia(523, 674, 508, A::add(s.ad_value(169), s.ad_value(510)));}
        if ((!s.b[1620]) && (!s.b[1928])) {s.store_scalar(523, 0.0);}
        if (!s.b[1620]) {s.store_scale(524, 533, (p.p919 * p.p2));s.store_add_scaled_inputs4_indices(520, 521, 1.0, 522, 1.0, 523, 1.0, 524, 1.0);s.store_mul(511, 669, 464);s.store_mul(514, 670, 495);s.store_scale(517, 668, (s.v[189] * p.p2));s.store_scalar(512, ((0.1) as f64).powf((-p.p914)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_196(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1932] = (p.p914 == 1.0);s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1932]) {s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1932])) {s.store_primal_offset_scaled_ad(513, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914))), (-(1.0 / (1.0 - p.p914))), (1.0 / (1.0 - p.p914)));}
        if (!s.b[1620]) {s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));}
        s.b[1933] = (p.p916 == 1.0);s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1933]) {s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1933])) {s.store_primal_offset_scaled_ad(516, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916))), (-(1.0 / (1.0 - p.p916))), (1.0 / (1.0 - p.p916)));}
        if (!s.b[1620]) {s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));}
        s.b[1934] = (p.p918 == 1.0);s.store_scalar(1934, if s.b[1934] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1934]) {s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));}
        if ((!s.b[1620]) && (!s.b[1934])) {s.store_primal_offset_scaled_ad(519, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918))), (-(1.0 / (1.0 - p.p918))), (1.0 / (1.0 - p.p918)));}
        s.b[1935] = (s.v[511] > 0.0);s.store_scalar(1935, if s.b[1935] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1935]) {s.store_div(168, 499, 675);}
        s.b[1936] = (s.v[168] < 0.9);s.store_scalar(1936, if s.b[1936] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1935]) && s.b[1936]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1937] = (p.p914 != 1.0);s.store_scalar(1937, if s.b[1937] { 1.0 } else { 0.0 });s.b[1938] = (p.p914 == 0.5);s.store_scalar(1938, if s.b[1938] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && s.b[1938]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p914));}
        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && s.b[1937]) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p914)), 0.0);}
        if ((((!s.b[1620]) && s.b[1935]) && s.b[1936]) && (!s.b[1937])) {s.store_mul_ad_affine_product_rhs(526, 675, s.ad_value(511), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1935]) && (!s.b[1936])) {s.store_mul_ad_product_rhs(169, 512, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p914), (((((-1.0)) * ((5.0 * p.p914)))) + ((1.0 + p.p914)))));s.store_mul_ad_product_rhs_mixed_ia(526, 675, 511, A::add(s.ad_value(169), s.ad_value(513)));}
        if ((!s.b[1620]) && (!s.b[1935])) {s.store_scalar(526, 0.0);}
        s.b[1939] = (s.v[514] > 0.0);s.store_scalar(1939, if s.b[1939] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1939]) {s.store_div(168, 499, 676);}
        s.b[1940] = (s.v[168] < 0.9);s.store_scalar(1940, if s.b[1940] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1939]) && s.b[1940]) {s.store_sub_from_scalar(500, 1.0, 168);}
        s.b[1941] = (p.p916 != 1.0);s.store_scalar(1941, if s.b[1941] { 1.0 } else { 0.0 });s.b[1942] = (p.p916 == 0.5);s.store_scalar(1942, if s.b[1942] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && s.b[1942]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p916));}
        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && s.b[1941]) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p916)), 0.0);}
        if ((((!s.b[1620]) && s.b[1939]) && s.b[1940]) && (!s.b[1941])) {s.store_mul_ad_affine_product_rhs(527, 676, s.ad_value(514), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1939]) && (!s.b[1940])) {s.store_mul_ad_product_rhs(169, 515, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p916), (((((-1.0)) * ((5.0 * p.p916)))) + ((1.0 + p.p916)))));s.store_mul_ad_product_rhs_mixed_ia(527, 676, 514, A::add(s.ad_value(169), s.ad_value(516)));}
        if ((!s.b[1620]) && (!s.b[1939])) {s.store_scalar(527, 0.0);}
        s.b[1943] = (s.v[517] > 0.0);s.store_scalar(1943, if s.b[1943] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1943]) {s.store_div(168, 499, 677);}
        s.b[1944] = (s.v[168] < 0.9);s.store_scalar(1944, if s.b[1944] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1943]) && s.b[1944]) {s.store_sub_from_scalar(500, 1.0, 168);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_197(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1945] = (p.p918 != 1.0);s.store_scalar(1945, if s.b[1945] { 1.0 } else { 0.0 });s.b[1946] = (p.p918 == 0.5);s.store_scalar(1946, if s.b[1946] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && s.b[1946]) {s.store_div_from_scalar_sqrt_ad(501, 1.0, s.ad_value(500));}
        if (((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) {s.store_limited_exp_scaled_input_ad(501, A::ln(s.ad_value(500)), (-p.p918));}
        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && s.b[1945]) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501))), 1.0 / ((1.0 - p.p918)), 0.0);}
        if ((((!s.b[1620]) && s.b[1943]) && s.b[1944]) && (!s.b[1945])) {s.store_mul_ad_affine_product_rhs(528, 677, s.ad_value(517), A::ln(s.ad_value(500)), -1.0, 0.0);}
        if (((!s.b[1620]) && s.b[1943]) && (!s.b[1944])) {s.store_mul_ad_product_rhs(169, 518, A::offset(s.ad_value(168), (-1.0)), A::scale_offset(s.ad_value(168), (5.0 * p.p918), (((((-1.0)) * ((5.0 * p.p918)))) + ((1.0 + p.p918)))));s.store_mul_ad_product_rhs_mixed_ia(528, 677, 517, A::add(s.ad_value(169), s.ad_value(519)));}
        if ((!s.b[1620]) && (!s.b[1943])) {s.store_scalar(528, 0.0);}
        if (!s.b[1620]) {s.store_scale(529, 534, (p.p919 * p.p2));s.store_add_scaled_inputs4_indices(525, 526, 1.0, 527, 1.0, 528, 1.0, 529, 1.0);}
        s.b[1947] = (p.p28 != 0.0);s.store_scalar(1947, if s.b[1947] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1947]) {s.store_powf_scaled_input(168, 706, 1.0000000000000001e-23, p.p1144);s.store_powf_ad(169, A::div_from_scalar(300.0, s.ad_value(635)), p.p1145);s.store_div_scaled_product_mixed_iai(170, 379, A::voltage(ctx, nodes, Some(10), Some(7)), p.p1143, 271, 1.0);s.store_scaled_limited_exp_ad(975, A::mul_scaled_lhs(s.ad_value(168), -1.0, s.ad_value(169)), p.p1138);s.store_scaled_mul(976, 169, 168, p.p1139);s.store_scale_ad(977, A::tanh(A::limited_exp(A::mul_scaled_lhs(s.ad_value(379), p.p1142, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(8), Some(10)), 1.0, s.ad_value(1128), (-1.0), A::voltage(ctx, nodes, Some(7), Some(10)), -1.0)))), p.p1141);s.store_mul_scale_offset(974, A::mul3(A::mul3_scaled_output(s.ad_value(211), s.ad_value(975), A::limited_exp(s.ad_value(170)), (p.p2 * s.v[183])), A::limited_exp_scaled_input(s.ad_value(976), (-s.v[184])), A::limited_exp(A::div(s.ad_value(977), s.ad_value(271)))), A::limited_exp_div_scaled_inputs(s.ad_value(227), p.p1140, s.ad_value(271), 1.0), 1.0, (-1.0));}
        if (!s.b[1620]) {s.store_scale(621, 271, (4.0 * 1.602176462e-19));s.store_div_scaled_inputs_indices(607, 746, 2.0, 337, 1.0);}
        s.b[1948] = (p.p1011 <= 0.0);s.store_scalar(1948, if s.b[1948] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1948]) {s.store_scalar(610, 0.0);}
        if ((!s.b[1620]) && (!s.b[1948])) {s.store_div_scaled_offset_numerator_mixed_ai(167, A::div(s.ad_value(355), s.ad_value(300)), 1.0, p.p1011, 607, 1.0);s.store_mul_ln_mixed_ia(610, 300, A::max_with_scalar(s.ad_value(167), 1e-38));}
        s.b[1949] = (s.v[610] < 0.0);s.store_scalar(1949, if s.b[1949] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1948])) && s.b[1949]) {s.store_scalar(610, 0.0);}
        if (!s.b[1620]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(613, 271, A::offset(s.ad_value(260), s.v[199]), 1.0 / (1.602176462e-19), 709, 1.0 / (1.602176462e-19));s.store_mul_ad_affine_product_lhs(612, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(320), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_198(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1620]) {s.store_mul_ad_affine_product_lhs(1004, s.ad_value(271), A::abs(s.ad_value(380)), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19), 0.0, 337);s.store_mul3_affine_lhs(1005, 271, 380, 1.602176462e-19, 0.0, 380);s.store_add_scaled_product_mixed_aii(1006, A::scale_offset(s.ad_value(612), p.p1013, p.p1012), 1.0, 612, 612, p.p1014);s.store_square_ad(1007, A::add(s.ad_value(612), s.ad_value(613)));s.store_scale(1008, 271, (p.p1012 * 1.602176462e-19));}
        s.b[1950] = (p.p1319 == 1.0);s.store_scalar(1950, if s.b[1950] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1950]) {s.store_scalar(1014, p.p1320);}
        s.b[1951] = (s.v[184] > s.v[1014]);s.store_scalar(1951, if s.b[1951] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1951]) {s.store_sub_from_scalar(167, s.v[184], 1014);}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1951])) {s.store_scalar(1014, s.v[184]);s.copy_ad(167, 1014);}
        s.b[1952] = (p.p1015 >= (s.v[167] / 2.0));s.store_scalar(1952, if s.b[1952] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1952]) {s.store_scalar(606, 0.0);}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1952])) {s.store_scalar(606, p.p1015);}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scalar(1013, s.v[184]);s.store_div_scaled_inputs2_indices(980, 221, 1.0, 707, (-1.0), 271, 1.0);s.store_scaled_sqrt_ad(981, A::div_from_scalar((((2.0 * 1.602176462e-19) * s.v[180]) * p.p1322), s.ad_value(271)), 1.0 / (s.v[199]));s.store_ln_ad(982, A::div_from_scalar(p.p1322, s.ad_value(182)));s.store_scalar(168, 1.0);s.store_div(404, 980, 168);s.store_div(405, 981, 168);s.store_sub_scaled_inputs_mixed_ia(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));}
        s.b[1953] = (s.v[404] < 0.0);s.store_scalar(1953, if s.b[1953] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1953]) {s.store_div_scaled_inputs2_indices(170, 404, 1.0, 169, (-1.0), 405, 1.0);s.store_neg_ad(983, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1953])) {s.store_limited_exp_neg_input(170, 169);s.store_scale(168, 405, 0.5);s.store_sub_mixed_ai(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);s.store_sub_offset_lhs_mixed_ai(983, A::square(s.ad_value(169)), 1.0, 170);}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scaled_add_offset_sqrt_square_offset(175, 983, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 175);s.store_div_scaled_offset_numerator_mixed_ai(167, A::div_scaled_inputs(s.ad_value(981), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, 981, 1.0);s.store_add_scaled_inputs3_indices(168, 983, 1.0, 982, (-2.0), 225, -1.0);s.store_sub_mixed_ia(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_199(
        s: &mut Scratch,
    ) {
        if ((!s.b[1620]) && s.b[1950]) {s.copy_ad(257, 259);}
        s.b[1954] = (s.v[175] <= (-68.0));s.store_scalar(1954, if s.b[1954] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {s.store_scalar(171, (-100.0));s.store_scalar(172, 20.0);}
        s.b[1955] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));s.store_scalar(1955, if s.b[1955] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && s.b[1955]) {s.store_limited_exp(170, 171);}
        s.b[1956] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));s.store_scalar(1956, if s.b[1956] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && s.b[1956]) {s.store_limited_exp(170, 175);}
        if (((((!s.b[1620]) && s.b[1950]) && s.b[1954]) && (!s.b[1955])) && (!s.b[1956])) {s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);s.store_square(173, 169);s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));}
        if (((!s.b[1620]) && s.b[1950]) && s.b[1954]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(985, 170, 168, 1.0, 175, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);}
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1954])) {s.store_limited_exp(170, 175);s.store_div_from_scalar(258, 1.0, 257);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_sub_div_rhs_indices(170, 170, 171, 172);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_square_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_200(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1950]) && (!s.b[1954])) {s.store_add_scaled_inputs3_mixed_aai(174, A::square(A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), 173, -1.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(985, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scaled_add_offset_sqrt_square_offset(984, 983, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_offset_div_scaled_inputs_sqrt_rhs(986, 981, 1.0, 984, 2.0, 1.0);s.copy_ad(987, 337);s.store_scale(994, 987, (s.v[199] * s.v[183]));s.store_scale(993, 337, (s.v[199] * s.v[183]));s.store_div_scaled_product_by_product_mixed_iiai(988, 380, 1014, 1.0, A::mul3_scaled_output(s.ad_value(986), s.ad_value(994), s.ad_value(271), 2.0), 271, 1.0);s.store_div_scaled_product_by_product_mixed_iaai(990, 380, A::sub(s.ad_value(1013), s.ad_value(1014)), 1.0, A::mul3_scaled_output(s.ad_value(253), s.ad_value(993), s.ad_value(269), 2.0), 269, 1.0);s.store_add_scaled_inputs3_offset_mixed_aii(167, A::square(s.ad_value(985)), 4.0, 985, 4.0, 988, (-4.0), 1.0);s.store_offset_scaled_ad(991, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(320)), 4.0, s.ad_value(320), 4.0, s.ad_value(990), 4.0), 1.0)), 0.5, (-0.5));}
        s.b[1958] = (s.v[184] != s.v[1014]);s.store_scalar(1958, if s.b[1958] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1950]) && s.b[1958]) {s.store_mul3_affine_lhs(992, 253, 271, ((2.0 * s.v[199]) * 6.241509744511525e18), 0.0, 991);s.store_primal_add_scaled_inputs3_indices(608, 1013, 1.0, 606, (-2.0), 1014, -1.0);s.store_primal_square(609, 608);s.store_scale(168, 609, (10000000000.0 * s.v[199]));s.store_scaled_ln_ad(169, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(992), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);s.store_scaled_sub(170, 992, 612, p.p1013);s.store_scaled_sub_ad(171, A::square(s.ad_value(992)), A::square(s.ad_value(612)), (0.5 * p.p1014));s.store_scale(172, 609, (10000000000.0 * (s.v[183] * p.p2)));s.store_add_scaled_product(1000, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(172), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(168)), A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(170), 1.0, s.ad_value(171), 1.0), 1.0);s.store_mul3_affine_lhs(173, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_201(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1620]) && s.b[1950]) && s.b[1958]) {s.store_mul_ad_product_lhs_mixed_ai(1001, A::div(s.ad_value(1008), s.ad_value(173)), 380, 380);s.store_add(174, 1001, 1000);}
        if ((!s.b[1620]) && s.b[1950]) {s.store_scale(175, 271, (p.p1321 * 1.602176462e-19));s.store_mul3_affine_lhs(176, 1014, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);s.store_mul_ad_product_lhs_mixed_ai(1009, A::div(s.ad_value(175), s.ad_value(176)), 380, 380);s.copy_ad(177, 1009);}
        s.b[1961] = (p.p1015 >= (s.v[184] / 2.0));s.store_scalar(1961, if s.b[1961] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1961]) {s.store_scalar(606, 0.0);}
        if (((!s.b[1620]) && (!s.b[1950])) && (!s.b[1961])) {s.store_scalar(606, p.p1015);}
        s.b[1962] = (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0));s.store_scalar(1962, if s.b[1962] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (!s.b[1950])) && s.b[1962]) {s.store_primal_sub_from_scalar_scaled_input(608, s.v[184], 606, 2.0);s.store_primal_square(609, 608);s.store_scale(167, 609, (10000000000.0 * s.v[199]));s.store_mul_ad_affine_product_lhs(611, A::mul3_scaled_output(s.ad_value(253), s.ad_value(271), s.ad_value(400), (2.0 * s.v[199])), s.ad_value(853), 6.241509744511525e18, 0.0, 834);s.store_scaled_ln_ad(168, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(611), 1.0, s.ad_value(613), 1.0, A::add(s.ad_value(612), s.ad_value(613)), 1.0), 1e-38), p.p1012);s.store_scaled_sub(169, 611, 612, p.p1013);s.store_scaled_sub_ad(170, A::square(s.ad_value(611)), A::square(s.ad_value(612)), (0.5 * p.p1014));s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p.p2)));s.store_add_scaled_product(614, A::div_scaled_product3_by_product(s.ad_value(1005), s.ad_value(610), s.ad_value(1006), 1.0, s.ad_value(171), s.ad_value(1007), 1.0), 1.0, A::div(s.ad_value(1004), s.ad_value(167)), A::add_scaled_inputs3(s.ad_value(168), 1.0, s.ad_value(169), 1.0, s.ad_value(170), 1.0), 1.0);s.store_mul3_affine_lhs(172, 608, 613, ((s.v[183] * p.p2) * 10000000000.0), 0.0, 613);s.store_mul_ad_product_lhs_mixed_ai(615, A::div(s.ad_value(1008), s.ad_value(172)), 380, 380);s.store_add(173, 615, 614);}
        if (!s.b[1620]) {s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));s.store_square(168, 167);s.store_offset_scaled(170, 168, (((p.p1022 * s.v[184])) * (p.p1019)), p.p1019);s.store_offset_scaled(171, 168, (((p.p1023 * s.v[184])) * (p.p1020)), p.p1020);s.store_offset_scaled(172, 168, (((p.p1298 * s.v[184])) * (p.p1297)), p.p1297);s.store_scaled_mul(631, 170, 170, 3.0);}
        if (!s.b[1620]) {s.store_offset_scaled(631, 631, { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));}
        if (!s.b[1620]) {s.store_square(633, 172);s.store_square(632, 171);}
        s.b[1964] = (p.p39 == 0.0);s.store_scalar(1964, if s.b[1964] { 1.0 } else { 0.0 });s.b[1965] = (p.p39 == 1.0);s.store_scalar(1965, if s.b[1965] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1964]) {s.store_scaled_mul(388, 271, 382, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));s.store_scaled_mul(389, 271, 385, ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199]));s.store_mul_abs_mixed_ia(167, 337, A::add(s.ad_value(388), s.ad_value(389)));s.store_offset_mul(168, 167, 457, (s.v[184] * s.v[184]));}
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_scaled_mul(626, 253, 269, 2.0);s.store_mul_scale_offset_mixed_ia(167, 626, A::mul3(s.ad_value(337), s.ad_value(345), s.ad_value(363)), s.v[199], 0.0);s.store_scaled_add(168, 400, 320, 0.5);s.store_offset(170, 168, 0.5);s.store_square(171, 170);s.store_mul(172, 171, 170);s.store_sub(173, 400, 320);s.store_square(174, 173);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_202(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_mul(175, 174, 173);s.store_mul_scale_offset_rhs(176, 174, 168, 6.0, 0.5);s.store_scale(625, 345, s.v[184]);s.store_scale(177, 625, 1.0 / (s.v[184]));s.store_offset_ad(179, A::div_scaled_product_by_product(s.ad_value(633), s.ad_value(315), 1.0, s.ad_value(316), A::offset(s.ad_value(243), p.p1299), 1.0), 1.0);}
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_offset_scaled(179, 179, { let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }, (((((-1.0)) * ({ let limited_exp_arg = ((-s.v[184]) / p.p1296); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))) + (1.0)));}
        if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {s.store_scaled_add_mixed_ia(179, 179, A::sqrt_square_offset(s.ad_value(179), ((0.25 * 0.1) * 0.1)), 0.5);s.store_mul_div_scaled_inputs_mixed_aii(624, A::add_scaled_product(A::div_scaled_product(s.ad_value(174), s.ad_value(631), 1.0, s.ad_value(170), 12.0), 1.0, s.ad_value(168), s.ad_value(179), 1.0), 167, (p.p2 * s.v[183]), 625, 1.0);s.store_div_scaled_product3_mixed_aaii(622, A::mul3(s.ad_value(625), s.ad_value(177), s.ad_value(177)), A::add_scaled_inputs3(A::div(s.ad_value(168), s.ad_value(171)), 1.0, A::div(s.ad_value(176), A::mul_scaled_lhs(s.ad_value(171), 60.0, s.ad_value(171))), (-1.0), A::div_scaled_product_by_product(s.ad_value(174), s.ad_value(174), 1.0, s.ad_value(171), s.ad_value(172), 144.0), 1.0), 632, (15.0 * 1.0 / (4.0)), 167, ((p.p2 * s.v[183]) * 12.0));s.store_sqrt_mul(628, 621, 624);}
        s.b[1966] = (s.v[622] > 0.0);s.store_scalar(1966, if s.b[1966] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) && s.b[1966]) {s.store_sqrt_div(629, 621, 622);}
        if (((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) && (!s.b[1966])) {s.store_scalar(629, 0.0);}
        s.b[1968] = (p.p37 != 0.0);s.store_scalar(1968, if s.b[1968] { 1.0 } else { 0.0 });s.b[1969] = (p.p38 != 0.0);s.store_scalar(1969, if s.b[1969] { 1.0 } else { 0.0 });
        if (!s.b[1620]) {s.copy_ad(217, 213);s.store_scalar(418, 0.0);}
        s.b[1970] = (p.p31 == 1.0);s.store_scalar(1970, if s.b[1970] { 1.0 } else { 0.0 });
        if ((!s.b[1620]) && s.b[1970]) {s.store_offset(793, 793, p.p25);s.store_mul(222, 221, 272);s.store_mul(225, 224, 272);s.store_mul(212, 793, 272);s.store_sub(217, 222, 212);s.store_ln_ad(432, A::max_with_scalar(A::div(s.ad_value(794), s.ad_value(182)), 1e-38));s.store_scaled_sqrt_mul_scaled_lhs(433, 794, ((2.0 * 1.602176462e-19) * s.v[180]), 272, 1.0 / (s.v[199]));s.store_div_from_scalar(295, 1.0, 433);s.store_div_scaled_inputs_indices(406, 704, ((2.0 * 1.602176462e-19) * s.v[180]), 271, (s.v[199] * s.v[199]));}
        if ((!s.b[1620]) && s.b[1970]) {
            if (s.v[704] > 0.0) {
                s.store_div_from_scalar(418, 1.0, 406);
            } else {
                s.store_scalar(418, 0.0);
            }
        }
        if ((!s.b[1620]) && s.b[1970]) {
            if (s.v[704] > 0.0) {
                s.store_div(403, 794, 704);
            } else {
                s.store_scalar(403, 0.0);
            }
        }
        if ((!s.b[1620]) && s.b[1970]) {s.store_offset(168, 403, 1.0);s.store_div(404, 217, 168);s.store_div(405, 433, 168);s.store_sub_scaled_inputs_mixed_ia(168, 404, 0.5, A::scale_offset(s.ad_value(405), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(169, 168, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(168)), 1.0, s.ad_value(404), 6.0)));}
        s.b[1971] = (s.v[404] < 0.0);s.store_scalar(1971, if s.b[1971] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1970]) && s.b[1971]) {s.store_div_scaled_inputs2_indices(170, 404, 1.0, 169, (-1.0), 405, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_203(
        s: &mut Scratch,
    ) {
        if (((!s.b[1620]) && s.b[1970]) && s.b[1971]) {s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));}
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1971])) {s.store_limited_exp_neg_input(170, 169);s.store_scale(168, 405, 0.5);s.store_sub_mixed_ai(169, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(404), 1.0, s.ad_value(170), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);s.store_sub_offset_lhs_mixed_ai(254, A::square(s.ad_value(169)), 1.0, 170);}
        if ((!s.b[1620]) && s.b[1970]) {s.store_scaled_add_offset_sqrt_square_offset(175, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 175);s.store_div_scaled_offset_numerator_mixed_ai(167, A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, 433, 1.0);s.store_add_scaled_inputs3_indices(168, 254, 1.0, 432, (-2.0), 225, -1.0);s.store_sub_mixed_ia(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);s.copy_ad(257, 259);}
        s.b[1972] = (s.v[175] <= (-68.0));s.store_scalar(1972, if s.b[1972] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {s.store_scalar(171, (-100.0));s.store_scalar(172, 20.0);}
        s.b[1973] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));s.store_scalar(1973, if s.b[1973] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && s.b[1973]) {s.store_limited_exp(170, 171);}
        s.b[1974] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));s.store_scalar(1974, if s.b[1974] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && s.b[1974]) {s.store_limited_exp(170, 175);}
        if (((((!s.b[1620]) && s.b[1970]) && s.b[1972]) && (!s.b[1973])) && (!s.b[1974])) {s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);s.store_square(173, 169);s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));}
        if (((!s.b[1620]) && s.b[1970]) && s.b[1972]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(400, 170, 168, 1.0, 175, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);}
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1972])) {s.store_limited_exp(170, 175);s.store_div_from_scalar(258, 1.0, 257);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_204(
        s: &mut Scratch,
    ) {
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1972])) {s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_sub_div_rhs_indices(170, 170, 171, 172);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_square_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_add_scaled_inputs3_mixed_aai(174, A::square(A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), 173, -1.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(400, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));}
        if ((!s.b[1620]) && s.b[1970]) {s.store_scaled_add_offset_sqrt_square_offset(256, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 256);s.store_sub_scaled_inputs(255, 254, 1.0, 400, 2.0);s.store_scaled_add_offset_sqrt_square_offset(167, 255, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_offset_div_ad(253, s.ad_value(433), A::add(s.ad_value(259), A::sqrt(s.ad_value(167))), 1.0);s.store_mul_mixed_ia(167, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_205(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1620]) && s.b[1970]) {s.store_scaled_add_mixed_ia(247, 167, A::sqrt_square_offset(s.ad_value(167), ((0.25 * 0.1) * 0.1)), 0.5);s.store_mul3_affine_lhs(306, 253, 271, 2.0, 0.0, 400);s.store_mul_add_scaled_inputs_rhs_indices(308, 335, 247, 1.0, 306, s.v[338]);s.store_mul_add_scaled_product_pow_rhs(170, 750, 1.0, 760, 218, 1.0, 308, 651);s.store_offset(171, 170, 1.0);s.store_scaled_add_offset_sqrt_square_offset(309, 171, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);s.store_div_scaled_product_by_product_indices(313, 740, 271, 1.0, 309, 655, s.v[188]);s.store_div_scaled_product_offset_denominator_mixed_iaa(307, 313, A::add(A::square(s.ad_value(400)), s.ad_value(400)), 1.0, A::mul_offset_rhs(s.ad_value(313), s.ad_value(400), 1.0), 1.0, 1.0);s.store_add_scaled_inputs4_mixed_iiia(321, 254, 1.0, 432, (-2.0), 307, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::add(A::mul3_scaled_output(s.ad_value(307), s.ad_value(253), s.ad_value(295), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(433), 1.0, s.ad_value(253), (-1.0), 1.0))), 1e-38)), -1.0);s.store_mul(322, 321, 271);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(317, 322, 0.5, 224, ((-1.0) * 0.5), 322, 224, ((0.25 * 0.001) * 0.001), 0.5);}
        s.b[1975] = ((p.p1353 == 0.0) && (p.p1354 == 0.0));s.store_scalar(1975, if s.b[1975] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1970]) && s.b[1975]) {s.store_scalar(1020, p.p1348);}
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1975])) {s.store_div_from_scalar_offset_ad(168, s.v[184], A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]);s.store_offset_div_scaled_inputs2_mixed_iaa(1020, 168, p.p1353, A::mul3_scaled_output(s.ad_value(168), s.ad_value(400), s.ad_value(269), p.p1354), (-1.0), A::scale_offset(s.ad_value(218), p.p1355, 1.0), 1.0, 1.0);s.store_scaled_add_offset_sqrt_square_offset(1020, 1020, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
        if ((!s.b[1620]) && s.b[1970]) {s.store_div(317, 317, 1020);s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(317)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));s.store_mul(315, 226, 175);s.store_mul_add_lhs(318, 315, 224, 272);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_206(
        s: &mut Scratch,
    ) {
        if ((!s.b[1620]) && s.b[1970]) {s.store_scaled_add_offset_sqrt_square_offset(175, 254, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(259, 175);s.store_div_scaled_offset_numerator_mixed_ai(167, A::div_scaled_inputs(s.ad_value(433), 1.0, s.ad_value(259), 2.0), 1.0, 1.0, 433, 1.0);s.store_add_scaled_inputs3_indices(168, 254, 1.0, 432, (-2.0), 318, -1.0);s.store_sub_mixed_ia(169, 168, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 4.0, s.ad_value(259)), 1e-38)));s.store_scaled_sub_ad(175, A::offset(s.ad_value(169), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(169), s.ad_value(169), 0.402982, 2.446562), 0.5);s.copy_ad(257, 259);}
        s.b[1976] = (s.v[175] <= (-68.0));s.store_scalar(1976, if s.b[1976] { 1.0 } else { 0.0 });
        if (((!s.b[1620]) && s.b[1970]) && s.b[1976]) {s.store_scalar(171, (-100.0));s.store_scalar(172, 20.0);}
        s.b[1977] = (s.v[175] < (s.v[171] - (0.5 * s.v[172])));s.store_scalar(1977, if s.b[1977] { 1.0 } else { 0.0 });
        if ((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && s.b[1977]) {s.store_limited_exp(170, 171);}
        s.b[1978] = (s.v[175] > (s.v[171] + (0.5 * s.v[172])));s.store_scalar(1978, if s.b[1978] { 1.0 } else { 0.0 });
        if (((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && (!s.b[1977])) && s.b[1978]) {s.store_limited_exp(170, 175);}
        if (((((!s.b[1620]) && s.b[1970]) && s.b[1976]) && (!s.b[1977])) && (!s.b[1978])) {s.store_div_scaled_inputs2_indices(169, 175, 1.0, 171, (-1.0), 172, 1.0);s.store_square(173, 169);s.store_limited_exp_ad(170, A::add_scaled_product(s.ad_value(171), 1.0, s.ad_value(172), A::add(A::scale_offset(s.ad_value(169), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(173), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(173), 1.25, s.ad_value(173)))), 1.0));}
        if (((!s.b[1620]) && s.b[1970]) && s.b[1976]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(320, 170, 168, 1.0, 175, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(167), 2.0, A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0)), 1e-38)), -1.0, 1.0);}
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1976])) {s.store_limited_exp(170, 175);s.store_div_from_scalar(258, 1.0, 257);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_sub_div_rhs_indices(170, 170, 171, 172);s.store_add_scaled_inputs3_mixed_iai(171, 170, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(170), s.ad_value(167), A::add_scaled_product(s.ad_value(257), 2.0, s.ad_value(170), s.ad_value(167), 2.0), 2.0), 1e-38)), 1.0, 168, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_207(
        s: &mut Scratch,
    ) {
        if (((!s.b[1620]) && s.b[1970]) && (!s.b[1976])) {s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_square_ad(173, A::div_scaled_inputs2(s.ad_value(167), 1.0, s.ad_value(258), 1.0, A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0), 1.0));s.store_add_scaled_inputs3_mixed_aai(174, A::square(A::div_from_scalar(1.0, s.ad_value(170))), -1.0, A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(257)), s.ad_value(257), A::add_scaled_product(s.ad_value(257), 1.0, s.ad_value(167), s.ad_value(170), 1.0))), (-1.0), 173, -1.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(320, 170, 1.0, A::div(s.ad_value(171), s.ad_value(172)), A::div_scaled_product_by_product(s.ad_value(171), s.ad_value(174), 1.0, s.ad_value(172), s.ad_value(172), 2.0), 1.0, (-1.0));}
        if ((!s.b[1620]) && s.b[1970]) {s.store_add_scaled_inputs3_offset_indices(255, 254, 1.0, 400, (-1.0), 320, -1.0, (-1.0));s.store_scaled_add_offset_sqrt_square_offset(167, 255, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(169, 167);s.store_add_offset_lhs_mixed_ia(170, 403, 1.0, A::div(s.ad_value(433), A::add(s.ad_value(259), s.ad_value(169))));s.store_offset_product3(171, s.ad_value(403), s.ad_value(169), s.ad_value(295), 1.0, 0.5);s.store_sqrt_add_ad(172, A::square(s.ad_value(171)), A::mul3(s.ad_value(170), A::add(s.ad_value(400), s.ad_value(320)), s.ad_value(418)));s.store_div_add_scaled_inputs_rhs_indices(253, 170, 171, 1.0, 172, 1.0);s.store_mul_mixed_ia(167, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(400), A::offset(s.ad_value(253), (-1.0)), (-2.0)));s.store_scaled_add_mixed_ia(247, 167, A::sqrt_square_offset(s.ad_value(167), ((0.25 * 0.1) * 0.1)), 0.5);s.store_mul_mixed_ia(168, 271, A::add_scaled_inputs_product(s.ad_value(217), 1.0, s.ad_value(254), (-1.0), s.ad_value(320), A::offset(s.ad_value(253), (-1.0)), (-2.0)));s.store_scaled_add_mixed_ia(248, 168, A::sqrt_square_offset(s.ad_value(168), ((0.25 * 0.1) * 0.1)), 0.5);s.store_scaled_add(249, 247, 248, 0.5);}
    }
}
