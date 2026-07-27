#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p[296] + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1826]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && (!s.b[1826])) {s.copy_ad(341, 647);}
        s.b[1827] = (s.v[793] >= 0.0);s.store_scalar(1827, if s.b[1827] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1827]) {s.copy_ad(369, 793);}
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && (!s.b[1827])) {s.store_scalar(369, 0.0);}
        s.b[1828] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(1828, if s.b[1828] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && s.b[1828]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p[297] - 1.0)) * ((20.0 + 1.0) - ((0.5 * p[297]) * 20.0))) * ((1e-12) as f64).powf(p[297])));s.store_scalar(379, ((((0.5 * p[297]) * (((20.0 + 1.0)) as f64).powf((p[297] - 1.0))) / 20.0) * ((1e-12) as f64).powf((p[297] - 2.0))));s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1825]) && (!s.b[1828])) {s.store_powf_offset_input(335, 369, 1e-12, p[297]);}
        if ((s.b[1443] && s.b[1444]) && s.b[1825]) {s.store_powf_offset_input(343, 369, 1e-12, p[299]);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1825])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        if (s.b[1443] && s.b[1444]) {s.store_add_scaled_inputs4_indices(131, 1477, (-0.5), 1478, (-0.5), 1498, (-0.5), 1500, (-0.5));s.store_scaled_add_mixed_ai(133, A::add(A::add_scaled_inputs4(s.ad_value(1538), 1.0, s.ad_value(1539), 1.0, s.ad_value(1517), 1.0, s.ad_value(1518), 1.0), s.ad_value(1497)), 1499, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 1538, 1539, (-0.5));s.store_neg(238, 1538);s.copy_ad(255, 1558);}
        s.b[1829] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(1829, if s.b[1829] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1829]) {s.store_scalar(78, 1.0);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.copy_ad(1855, 960);s.store_scale(1905, 964, 1.6021918e-19);s.store_scale(1884, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1904, 622, 1.6021918e-19);s.store_square(1903, 965);s.store_div_from_scalar(1908, (2.0 * 1.034943e-10), 1905);s.store_div_from_scalar(1909, (2.0 * 1.034943e-10), 1904);s.store_div(1902, 964, 622);s.store_div_from_scalar_offset_input(1901, 1.0, 1902, 1.0);s.store_div_square_rhs(1906, 1884, 185);s.store_div_from_scalar(1907, 2.0, 1906);s.store_scalar(1910, 4.0);s.store_scalar(1911, 0.1);s.store_scalar(1912, 0.1);s.store_offset(1913, 961, p[407]);s.store_scalar(1914, 3.0);s.store_scalar(1853, 0.0);s.store_scalar(1854, 0.0);s.store_scalar(1862, 0.0);s.store_scalar(1863, 0.0);s.store_scalar(1895, 0.0);s.store_scalar(1896, 0.0);s.store_scalar(1866, 0.0);s.store_scalar(1868, 0.0);s.store_scalar(1867, 0.0);s.store_scalar(1869, 0.0);s.store_scalar(1839, 0.0);s.store_scalar(1834, 0.0);s.copy_ad(1887, 1435);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));s.store_div_scaled_product_add_scaled_denominator_indices(962, 1908, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);s.store_sub(335, 1855, 1438);}
        s.b[1917] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1918] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });s.b[1919] = (4.0 == 1.0);s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && s.b[1919]) {s.store_scalar(720, 1.0);}
        s.b[1920] = (4.0 == 2.0);s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && s.b[1920]) {s.store_scalar(720, 2.0);}
        s.b[1921] = (4.0 == 4.0);s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && (!s.b[1920])) && s.b[1921]) {s.store_scalar(720, 3.0);}
        s.b[1922] = (4.0 == 8.0);s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && (!s.b[1920])) && (!s.b[1921])) && s.b[1922]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && (!s.b[1918])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1917])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul(1838, 962, 336);s.store_sqrt(1836, 1838);}
        s.b[1923] = (p[345] != 0.0);s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {s.store_mul_scale_offset_mixed_ia(335, 965, A::scale(s.ad_value(790), p[345]), -1.0, 1.0);s.store_scale(336, 965, 0.001);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_scalar(1851, 0.0);}
        s.b[1924] = (s.v[1836] > s.v[965]);s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1924]) {s.copy_ad(1835, 965);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1924])) {s.copy_ad(1835, 1836);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));s.store_scalar(782, ((4.0 * 0.3) * 0.01));}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(1860, 781, (-0.5), 782, (-0.5), 0.3);s.store_add_scaled_inputs3_offset_indices(781, 1860, 1.0, 1887, -1.0, 1855, 1.0, (-0.01));s.store_scaled_sub(782, 1887, 1855, (4.0 * 0.01));}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_sqrt_square_add(782, 781, 782);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1860, 1887, 1.0, 1855, (-1.0), 781, 0.5, 782, 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(1888, 1855, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);s.store_offset_sub(1834, 965, 1835, 1e-15);s.store_scalar(79, 0.0);s.store_scalar(1850, 0.2);s.copy_ad(1853, 1860);s.copy_ad(1856, 1851);s.copy_ad(1858, 1888);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul_sub_mixed_iai(1858, 1901, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1902), s.ad_value(1856), 1.0), 1855);s.store_mul(1842, 1901, 1902);s.store_sub(335, 1856, 1858);}
            s.b[1925] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1926] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });s.b[1927] = (2.0 == 1.0);s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && s.b[1927]) {s.store_scalar(720, 1.0);}
            s.b[1928] = (2.0 == 2.0);s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && s.b[1928]) {s.store_scalar(720, 2.0);}
            s.b[1929] = (2.0 == 4.0);s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && (!s.b[1928])) && s.b[1929]) {s.store_scalar(720, 3.0);}
            s.b[1930] = (2.0 == 8.0);s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && (!s.b[1928])) && (!s.b[1929])) && s.b[1930]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) {s.store_scalar(719, 0.0);}
            let mut t7: usize = 0;
            while {
                let t6: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t6 != 0.0
            } {
                t7 += 1;
                if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && (!s.b[1926])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1925])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_sqrt_mul(1830, 1908, 336);}
            s.b[1931] = ((s.v[1830] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {s.store_offset_sub(781, 1830, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1932] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });s.b[1933] = (2.0 == 1.0);s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && s.b[1933]) {s.store_scalar(720, 1.0);}
            s.b[1934] = (2.0 == 2.0);s.store_scalar(1934, if s.b[1934] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && s.b[1934]) {s.store_scalar(720, 2.0);}
            s.b[1935] = (2.0 == 4.0);s.store_scalar(1935, if s.b[1935] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && (!s.b[1934])) && s.b[1935]) {s.store_scalar(720, 3.0);}
            s.b[1936] = (2.0 == 8.0);s.store_scalar(1936, if s.b[1936] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && (!s.b[1934])) && (!s.b[1935])) && s.b[1936]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) {s.store_scalar(719, 0.0);}
            let mut t9: usize = 0;
            while {
                let t8: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t8 != 0.0
            } {
                t9 += 1;
                if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && (!s.b[1932])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1830, 965, (-1e-8), 780);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1931])) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1931])) {s.store_scalar(337, 1.0);}
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul(1866, 1830, 1905);s.store_mul_ad_product_lhs_mixed_ai(1844, A::div_from_scalar(1.034943e-10, s.ad_value(1830)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1846, A::div_from_scalar((-1.034943e-10), s.ad_value(1830)), 334, 337);}
            s.b[1937] = (p[49] == 0.0);s.store_scalar(1937, if s.b[1937] { 1.0 } else { 0.0 });
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1937]) {s.store_add_mixed_ai(1839, A::div_scaled_inputs_product(s.ad_value(1903), 1.0, s.ad_value(1838), 1.0, s.ad_value(965), s.ad_value(1835), (-2.0), s.ad_value(1908), 1.0), 1853);s.store_scalar(1840, 1.0);s.store_scalar(1841, 0.0);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1937])) {s.store_add_mixed_ia(1839, 1853, A::div_scaled_add_product(s.ad_value(1903), 1.0, s.ad_value(1830), A::sub_scaled_inputs(s.ad_value(1830), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1908), 1.0));s.store_scalar(1840, 1.0);s.store_mul_scale_offset_mixed_ai(1841, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1830)), s.ad_value(334), (-1.0)), 1842, -1.0, 1.0);}
            s.b[1938] = ((s.v[1839] > (s.v[1851] - s.v[1850])) && (s.v[1850] >= 0.0));s.store_scalar(1938, if s.b[1938] { 1.0 } else { 0.0 });
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {s.store_add_scaled_inputs3_indices(781, 1839, 1.0, 1851, (-1.0), 1850, 1.0);s.store_square(722, 781);s.store_square(723, 1850);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1939] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1939, if s.b[1939] { 1.0 } else { 0.0 });s.b[1940] = (4.0 == 1.0);s.store_scalar(1940, if s.b[1940] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && s.b[1940]) {s.store_scalar(720, 1.0);}
            s.b[1941] = (4.0 == 2.0);s.store_scalar(1941, if s.b[1941] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && s.b[1941]) {s.store_scalar(720, 2.0);}
            s.b[1942] = (4.0 == 4.0);s.store_scalar(1942, if s.b[1942] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && (!s.b[1941])) && s.b[1942]) {s.store_scalar(720, 3.0);}
            s.b[1943] = (4.0 == 8.0);s.store_scalar(1943, if s.b[1943] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && (!s.b[1941])) && (!s.b[1942])) && s.b[1943]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) {s.store_scalar(719, 0.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;
                if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && (!s.b[1939])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1850, 726);s.store_div_scaled_product3_indices(334, 1850, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1839, 1851, 1.0, 1850, (-1.0), 780, 1.0);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1938])) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1938])) {s.store_scalar(334, 1.0);}
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul(1840, 1840, 334);s.store_mul(1841, 1841, 334);s.store_add_scaled_inputs3_indices(335, 1858, 1.0, 1887, (-1.0), 1855, 1.0);}
            s.b[1944] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1944, if s.b[1944] { 1.0 } else { 0.0 });
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1945] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1945, if s.b[1945] { 1.0 } else { 0.0 });s.b[1946] = (2.0 == 1.0);s.store_scalar(1946, if s.b[1946] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && s.b[1946]) {s.store_scalar(720, 1.0);}
            s.b[1947] = (2.0 == 2.0);s.store_scalar(1947, if s.b[1947] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && s.b[1947]) {s.store_scalar(720, 2.0);}
            s.b[1948] = (2.0 == 4.0);s.store_scalar(1948, if s.b[1948] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && (!s.b[1947])) && s.b[1948]) {s.store_scalar(720, 3.0);}
            s.b[1949] = (2.0 == 8.0);s.store_scalar(1949, if s.b[1949] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && (!s.b[1947])) && (!s.b[1948])) && s.b[1949]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) {s.store_scalar(719, 0.0);}
            let mut t5: usize = 0;
            while {
                let t4: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t4 != 0.0
            } {
                t5 += 1;
                if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && (!s.b[1945])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1944])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_sqrt_mul(1832, 1909, 336);s.store_mul_scale_offset_indices(1867, 1904, 1832, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1848, (-1.034943e-10), 1832, 337);s.store_mul_sub_rhs(335, 154, 1853, 1856);s.store_exp(336, 335);}
            s.b[1950] = (s.v[1853] >= s.v[1856]);s.store_scalar(1950, if s.b[1950] { 1.0 } else { 0.0 });
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1950]) {s.store_mul_scaled_sqrt_ad_rhs(1862, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1897, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1862, 1.0);s.store_neg(1899, 1897);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1950])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1853), s.ad_value(1887)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1856), s.ad_value(1887)));s.store_mul_sqrt_mixed_ia(1862, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1862, 1.0);s.store_mul_add_mixed_iaa(1897, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1899, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1870, A::add_scaled_product(s.ad_value(1862), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1853)), 1.0), 1.0, 1866, 1.0, 1867, 1.0);s.store_sub(1871, 1897, 185);s.store_add_mixed_ia(1872, 1899, A::add_scaled_value_products(s.ad_value(1844), 1.0, s.ad_value(1846), s.ad_value(1842), 1.0, s.ad_value(1848), s.ad_value(1842), 1.0));s.store_sub(1873, 1856, 1839);s.store_neg(1874, 1840);s.store_sub_from_scalar(1875, 1.0, 1841);s.store_add_scaled_products_indices(1876, 1871, 1875, 1.0, 1872, 1874, (-1.0));}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                if (s.v[1876] > 0.0) {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, (-1e-25));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {s.copy_ad(1878, 1875);s.store_neg(1879, 1872);s.store_neg(1880, 1874);s.copy_ad(1881, 1871);s.store_mul_add_scaled_products_indices_rhs(1882, 1877, 1878, 1870, -1.0, 1879, 1873, -1.0);s.store_mul_add_scaled_products_indices_rhs(1883, 1877, 1880, 1870, -1.0, 1881, 1873, -1.0);s.store_abs(335, 1882);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1883]) as f64).abs()) {
                    s.store_abs(335, 1883);
                } else {
                }
            }
            s.b[1951] = (s.v[335] > 0.1);s.store_scalar(1951, if s.b[1951] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) && s.b[1951]) {s.store_mul_div_from_scalar_lhs_ad_indices(1882, 0.1, 335, 1882);s.store_mul_div_from_scalar_lhs_ad_indices(1883, 0.1, 335, 1883);}
            s.b[1952] = (s.v[335] < 1e-12);s.store_scalar(1952, if s.b[1952] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) && s.b[1952]) {s.store_scalar(79, 1.0);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {s.store_add(1853, 1853, 1882);s.store_add(1856, 1856, 1883);}
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul_sub_rhs(335, 154, 1853, 1856);s.store_exp(336, 335);}
        s.b[1954] = (s.v[1853] >= s.v[1856]);s.store_scalar(1954, if s.b[1954] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1954]) {s.copy_ad(1892, 1862);s.store_scalar(1895, 0.0);s.store_scalar(1864, 0.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1954])) {s.store_scalar(1892, 0.0);s.store_mul_sqrt_mixed_ia(1895, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[1955] = (s.v[1836] > s.v[965]);s.store_scalar(1955, if s.b[1955] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1954])) && s.b[1955]) {s.store_scalar(1864, 0.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1954])) && (!s.b[1955])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1853), s.ad_value(1887)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1856), s.ad_value(1887)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1864, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[1956] = (((s.v[1853] - s.v[1851]) < s.v[1911]) && (s.v[1911] >= 0.0));s.store_scalar(1956, if s.b[1956] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {s.store_add_scaled_inputs3_indices(781, 1911, 1.0, 1853, -1.0, 1851, 1.0);s.store_square(722, 781);s.store_square(723, 1911);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1957] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1957, if s.b[1957] { 1.0 } else { 0.0 });s.b[1958] = (4.0 == 1.0);s.store_scalar(1958, if s.b[1958] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && s.b[1958]) {s.store_scalar(720, 1.0);}
        s.b[1959] = (4.0 == 2.0);s.store_scalar(1959, if s.b[1959] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (!s.b[1958])) && s.b[1959]) {s.store_scalar(720, 2.0);}
        s.b[1960] = (4.0 == 4.0);s.store_scalar(1960, if s.b[1960] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (!s.b[1958])) && (!s.b[1959])) && s.b[1960]) {s.store_scalar(720, 3.0);}
        s.b[1961] = (4.0 == 8.0);s.store_scalar(1961, if s.b[1961] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (!s.b[1958])) && (!s.b[1959])) && (!s.b[1960])) && s.b[1961]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && (!s.b[1957])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1911, 726);s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1911, 780);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1956])) {s.store_sub(336, 1853, 1851);s.store_scalar(334, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1889, 209, -1.0, 338);s.copy_ad(349, 790);}
        s.b[1962] = (s.v[790] > 1e-6);s.store_scalar(1962, if s.b[1962] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {s.store_scalar(344, 1e-25);s.store_offset_mul_ad(338, s.ad_value(1907), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1907, 1.0);}
        s.b[1963] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(1963, if s.b[1963] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1964] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1964, if s.b[1964] { 1.0 } else { 0.0 });s.b[1965] = (2.0 == 1.0);s.store_scalar(1965, if s.b[1965] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && s.b[1965]) {s.store_scalar(720, 1.0);}
        s.b[1966] = (2.0 == 2.0);s.store_scalar(1966, if s.b[1966] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (!s.b[1965])) && s.b[1966]) {s.store_scalar(720, 2.0);}
        s.b[1967] = (2.0 == 4.0);s.store_scalar(1967, if s.b[1967] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (!s.b[1965])) && (!s.b[1966])) && s.b[1967]) {s.store_scalar(720, 3.0);}
        s.b[1968] = (2.0 == 8.0);s.store_scalar(1968, if s.b[1968] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (!s.b[1965])) && (!s.b[1966])) && (!s.b[1967])) && s.b[1968]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;
            if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && (!s.b[1964])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1963])) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1963])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1906, 1.0, 337);}
        s.b[1969] = ((s.v[344] < 1.0) && (1.0 >= 0.0));s.store_scalar(1969, if s.b[1969] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {s.store_sub_from_scalar(781, 1.0, 344);s.store_square(722, 781);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1970] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1970, if s.b[1970] { 1.0 } else { 0.0 });s.b[1971] = (2.0 == 1.0);s.store_scalar(1971, if s.b[1971] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && s.b[1971]) {s.store_scalar(720, 1.0);}
        s.b[1972] = (2.0 == 2.0);s.store_scalar(1972, if s.b[1972] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (!s.b[1971])) && s.b[1972]) {s.store_scalar(720, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1973] = (2.0 == 4.0);s.store_scalar(1973, if s.b[1973] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (!s.b[1971])) && (!s.b[1972])) && s.b[1973]) {s.store_scalar(720, 3.0);}
        s.b[1974] = (2.0 == 8.0);s.store_scalar(1974, if s.b[1974] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (!s.b[1971])) && (!s.b[1972])) && (!s.b[1973])) && s.b[1974]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && (!s.b[1970])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1.0);s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(344, 1.0, 780);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1969])) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1969])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {s.store_div(335, 790, 344);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);s.copy_ad(790, 348);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1962])) {s.copy_ad(348, 790);}
        s.b[1975] = (s.v[790] < 0.0);s.store_scalar(1975, if s.b[1975] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1975]) {s.copy_ad(1854, 1853);s.copy_ad(1859, 1858);s.copy_ad(1857, 1856);s.copy_ad(1865, 1864);s.copy_ad(1893, 1892);s.copy_ad(1890, 1889);s.copy_ad(1868, 1866);s.copy_ad(1869, 1867);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.copy_ad(1837, 1836);s.copy_ad(1852, 790);s.store_add_scaled_inputs3_offset_indices(781, 1853, 1.0, 1852, 1.0, 85, -1.0, (-0.01));s.store_scaled_add(782, 1853, 1852, (4.0 * 0.01));}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1861, 1853, 1.0, 1852, 1.0, 781, (-0.5), 782, (-0.5));s.store_add_scaled_inputs3_offset_indices(781, 1861, 1.0, 1887, -1.0, 1855, 1.0, (-0.01));s.store_scaled_sub(782, 1887, 1855, (4.0 * 0.01));}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1861, 1887, 1.0, 1855, (-1.0), 781, 0.5, 782, 0.5);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1887))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_add_product3_rhs_mixed_iia(92, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);s.store_scalar(79, 0.0);s.copy_ad(1854, 1861);s.copy_ad(1857, 1852);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_mul_sub_mixed_iai(1859, 1901, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1902), s.ad_value(1857), 1.0), 1855);s.store_mul(1843, 1901, 1902);s.store_sub(335, 1857, 1859);}
            s.b[1976] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1976, if s.b[1976] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1977] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1977, if s.b[1977] { 1.0 } else { 0.0 });s.b[1978] = (2.0 == 1.0);s.store_scalar(1978, if s.b[1978] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && s.b[1978]) {s.store_scalar(720, 1.0);}
            s.b[1979] = (2.0 == 2.0);s.store_scalar(1979, if s.b[1979] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (!s.b[1978])) && s.b[1979]) {s.store_scalar(720, 2.0);}
            s.b[1980] = (2.0 == 4.0);s.store_scalar(1980, if s.b[1980] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (!s.b[1978])) && (!s.b[1979])) && s.b[1980]) {s.store_scalar(720, 3.0);}
            s.b[1981] = (2.0 == 8.0);s.store_scalar(1981, if s.b[1981] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (!s.b[1978])) && (!s.b[1979])) && (!s.b[1980])) && s.b[1981]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) {s.store_scalar(719, 0.0);}
            let mut t17: usize = 0;
            while {
                let t16: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t16 != 0.0
            } {
                t17 += 1;
                if t17 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t17, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && (!s.b[1977])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1976])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_sqrt_mul(1831, 1908, 336);}
            s.b[1982] = ((s.v[1831] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1982, if s.b[1982] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {s.store_offset_sub(781, 1831, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1983] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1983, if s.b[1983] { 1.0 } else { 0.0 });s.b[1984] = (2.0 == 1.0);s.store_scalar(1984, if s.b[1984] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && s.b[1984]) {s.store_scalar(720, 1.0);}
            s.b[1985] = (2.0 == 2.0);s.store_scalar(1985, if s.b[1985] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (!s.b[1984])) && s.b[1985]) {s.store_scalar(720, 2.0);}
            s.b[1986] = (2.0 == 4.0);s.store_scalar(1986, if s.b[1986] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (!s.b[1984])) && (!s.b[1985])) && s.b[1986]) {s.store_scalar(720, 3.0);}
            s.b[1987] = (2.0 == 8.0);s.store_scalar(1987, if s.b[1987] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (!s.b[1984])) && (!s.b[1985])) && (!s.b[1986])) && s.b[1987]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) {s.store_scalar(719, 0.0);}
            let mut t19: usize = 0;
            while {
                let t18: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t18 != 0.0
            } {
                t19 += 1;
                if t19 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t19, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && (!s.b[1983])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1831, 965, (-1e-8), 780);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1982])) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1982])) {s.store_scalar(337, 1.0);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_mul(1868, 1831, 1905);s.store_mul_ad_product_lhs_mixed_ai(1845, A::div_from_scalar(1.034943e-10, s.ad_value(1831)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1847, A::div_from_scalar((-1.034943e-10), s.ad_value(1831)), 334, 337);}
            s.b[1988] = (p[49] == 0.0);s.store_scalar(1988, if s.b[1988] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1988]) {s.store_add_mixed_ai(1839, A::div_scaled_inputs_product(s.ad_value(1903), 1.0, s.ad_value(1838), 1.0, s.ad_value(965), s.ad_value(1835), (-2.0), s.ad_value(1908), 1.0), 1854);s.store_scalar(1840, 1.0);s.store_scalar(1841, 0.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1988])) {s.store_add_mixed_ia(1839, 1854, A::div_scaled_add_product(s.ad_value(1903), 1.0, s.ad_value(1831), A::sub_scaled_inputs(s.ad_value(1831), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1908), 1.0));s.store_scalar(1840, 1.0);s.store_mul_scale_offset_mixed_ai(1841, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1831)), s.ad_value(334), (-1.0)), 1843, -1.0, 1.0);}
            s.b[1989] = ((s.v[1839] > (s.v[1852] - s.v[1850])) && (s.v[1850] >= 0.0));s.store_scalar(1989, if s.b[1989] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {s.store_add_scaled_inputs3_indices(781, 1839, 1.0, 1852, (-1.0), 1850, 1.0);s.store_square(722, 781);s.store_square(723, 1850);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1990] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1990, if s.b[1990] { 1.0 } else { 0.0 });s.b[1991] = (4.0 == 1.0);s.store_scalar(1991, if s.b[1991] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && s.b[1991]) {s.store_scalar(720, 1.0);}
            s.b[1992] = (4.0 == 2.0);s.store_scalar(1992, if s.b[1992] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (!s.b[1991])) && s.b[1992]) {s.store_scalar(720, 2.0);}
            s.b[1993] = (4.0 == 4.0);s.store_scalar(1993, if s.b[1993] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (!s.b[1991])) && (!s.b[1992])) && s.b[1993]) {s.store_scalar(720, 3.0);}
            s.b[1994] = (4.0 == 8.0);s.store_scalar(1994, if s.b[1994] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (!s.b[1991])) && (!s.b[1992])) && (!s.b[1993])) && s.b[1994]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) {s.store_scalar(719, 0.0);}
            let mut t13: usize = 0;
            while {
                let t12: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t12 != 0.0
            } {
                t13 += 1;
                if t13 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t13, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && (!s.b[1990])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1850, 726);s.store_div_scaled_product3_indices(334, 1850, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1839, 1852, 1.0, 1850, (-1.0), 780, 1.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1989])) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1989])) {s.store_scalar(334, 1.0);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_mul(1840, 1840, 334);s.store_mul(1841, 1841, 334);s.store_add_scaled_inputs3_indices(335, 1859, 1.0, 1887, (-1.0), 1855, 1.0);}
            s.b[1995] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1995, if s.b[1995] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1996] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1996, if s.b[1996] { 1.0 } else { 0.0 });s.b[1997] = (2.0 == 1.0);s.store_scalar(1997, if s.b[1997] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && s.b[1997]) {s.store_scalar(720, 1.0);}
            s.b[1998] = (2.0 == 2.0);s.store_scalar(1998, if s.b[1998] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) && s.b[1998]) {s.store_scalar(720, 2.0);}
            s.b[1999] = (2.0 == 4.0);s.store_scalar(1999, if s.b[1999] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) && (!s.b[1998])) && s.b[1999]) {s.store_scalar(720, 3.0);}
            s.b[2000] = (2.0 == 8.0);s.store_scalar(2000, if s.b[2000] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) && (!s.b[1998])) && (!s.b[1999])) && s.b[2000]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) {s.store_scalar(719, 0.0);}
            let mut t15: usize = 0;
            while {
                let t14: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t14 != 0.0
            } {
                t15 += 1;
                if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && (!s.b[1996])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1995])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_sqrt_mul(1833, 1909, 336);s.store_mul_scale_offset_indices(1869, 1904, 1833, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1849, (-1.034943e-10), 1833, 337);s.store_mul_sub_rhs(335, 154, 1854, 1857);s.store_exp(336, 335);}
            s.b[2001] = (s.v[1854] >= s.v[1857]);s.store_scalar(2001, if s.b[2001] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2001]) {s.store_mul_scaled_sqrt_ad_rhs(1863, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1898, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1863, 1.0);s.store_neg(1900, 1898);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2001])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1854), s.ad_value(1887)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1857), s.ad_value(1887)));s.store_mul_sqrt_mixed_ia(1863, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1863, 1.0);s.store_mul_add_mixed_iaa(1898, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1900, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1870, A::add_scaled_product(s.ad_value(1863), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1854)), 1.0), 1.0, 1868, 1.0, 1869, 1.0);s.store_sub(1871, 1898, 185);s.store_add_mixed_ia(1872, 1900, A::add_scaled_value_products(s.ad_value(1845), 1.0, s.ad_value(1847), s.ad_value(1843), 1.0, s.ad_value(1849), s.ad_value(1843), 1.0));s.store_sub(1873, 1857, 1839);s.store_neg(1874, 1840);s.store_sub_from_scalar(1875, 1.0, 1841);s.store_add_scaled_products_indices(1876, 1871, 1875, 1.0, 1872, 1874, (-1.0));}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {
                if (s.v[1876] > 0.0) {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, (-1e-25));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {s.copy_ad(1878, 1875);s.store_neg(1879, 1872);s.store_neg(1880, 1874);s.copy_ad(1881, 1871);s.store_mul_add_scaled_products_indices_rhs(1882, 1877, 1878, 1870, -1.0, 1879, 1873, -1.0);s.store_mul_add_scaled_products_indices_rhs(1883, 1877, 1880, 1870, -1.0, 1881, 1873, -1.0);s.store_abs(335, 1882);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1883]) as f64).abs()) {
                    s.store_abs(335, 1883);
                } else {
                }
            }
            s.b[2002] = (s.v[335] > 0.1);s.store_scalar(2002, if s.b[2002] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) && s.b[2002]) {s.store_mul_div_from_scalar_lhs_ad_indices(1882, 0.1, 335, 1882);s.store_mul_div_from_scalar_lhs_ad_indices(1883, 0.1, 335, 1883);}
            s.b[2003] = (s.v[335] < 1e-12);s.store_scalar(2003, if s.b[2003] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) && s.b[2003]) {s.store_scalar(79, 1.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {s.store_add(1854, 1854, 1882);s.store_add(1857, 1857, 1883);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_mul_sub_rhs(335, 154, 1854, 1857);s.store_exp(336, 335);}
        s.b[2005] = (s.v[1854] >= s.v[1857]);s.store_scalar(2005, if s.b[2005] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2005]) {s.copy_ad(1893, 1863);s.store_scalar(1896, 0.0);s.store_scalar(1865, 0.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2005])) {s.store_scalar(1893, 0.0);s.store_mul_sqrt_mixed_ia(1896, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[2006] = (s.v[1837] > s.v[965]);s.store_scalar(2006, if s.b[2006] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2005])) && s.b[2006]) {s.store_scalar(1865, 0.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2005])) && (!s.b[2006])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1887)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1857), s.ad_value(1887)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1865, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[2007] = (((s.v[1854] - s.v[1852]) < s.v[1911]) && (s.v[1911] >= 0.0));s.store_scalar(2007, if s.b[2007] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {s.store_add_scaled_inputs3_indices(781, 1911, 1.0, 1854, -1.0, 1852, 1.0);s.store_square(722, 781);s.store_square(723, 1911);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2008] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2008, if s.b[2008] { 1.0 } else { 0.0 });s.b[2009] = (4.0 == 1.0);s.store_scalar(2009, if s.b[2009] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && s.b[2009]) {s.store_scalar(720, 1.0);}
        s.b[2010] = (4.0 == 2.0);s.store_scalar(2010, if s.b[2010] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (!s.b[2009])) && s.b[2010]) {s.store_scalar(720, 2.0);}
        s.b[2011] = (4.0 == 4.0);s.store_scalar(2011, if s.b[2011] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (!s.b[2009])) && (!s.b[2010])) && s.b[2011]) {s.store_scalar(720, 3.0);}
        s.b[2012] = (4.0 == 8.0);s.store_scalar(2012, if s.b[2012] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (!s.b[2009])) && (!s.b[2010])) && (!s.b[2011])) && s.b[2012]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;
            if t1d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && (!s.b[2008])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1911, 726);s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1911, 780);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2007])) {s.store_sub(336, 1854, 1852);s.store_scalar(334, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1890, 209, -1.0, 338);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.copy_ad(87, 1853);s.copy_ad(91, 1854);s.store_sub(94, 1854, 1853);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / ((p[263] * 0.1))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, (p[263] * 0.1), 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[2013] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2013, if s.b[2013] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2014] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2014, if s.b[2014] { 1.0 } else { 0.0 });s.b[2015] = (2.0 == 1.0);s.store_scalar(2015, if s.b[2015] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && s.b[2015]) {s.store_scalar(720, 1.0);}
        s.b[2016] = (2.0 == 2.0);s.store_scalar(2016, if s.b[2016] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (!s.b[2015])) && s.b[2016]) {s.store_scalar(720, 2.0);}
        s.b[2017] = (2.0 == 4.0);s.store_scalar(2017, if s.b[2017] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (!s.b[2015])) && (!s.b[2016])) && s.b[2017]) {s.store_scalar(720, 3.0);}
        s.b[2018] = (2.0 == 8.0);s.store_scalar(2018, if s.b[2018] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (!s.b[2015])) && (!s.b[2016])) && (!s.b[2017])) && s.b[2018]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) {s.store_scalar(719, 0.0);}
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;
            if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && (!s.b[2014])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2013])) {
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2013])) {s.store_scalar(334, 1.0);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_add(109, 87, 110);}
        s.b[2019] = (((s.v[109] - s.v[1851]) < s.v[1911]) && (s.v[1911] >= 0.0));s.store_scalar(2019, if s.b[2019] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {s.store_add_scaled_inputs3_indices(781, 1911, 1.0, 109, -1.0, 1851, 1.0);s.store_square(722, 781);s.store_square(723, 1911);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2020] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2020, if s.b[2020] { 1.0 } else { 0.0 });s.b[2021] = (4.0 == 1.0);s.store_scalar(2021, if s.b[2021] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && s.b[2021]) {s.store_scalar(720, 1.0);}
        s.b[2022] = (4.0 == 2.0);s.store_scalar(2022, if s.b[2022] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (!s.b[2021])) && s.b[2022]) {s.store_scalar(720, 2.0);}
        s.b[2023] = (4.0 == 4.0);s.store_scalar(2023, if s.b[2023] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (!s.b[2021])) && (!s.b[2022])) && s.b[2023]) {s.store_scalar(720, 3.0);}
        s.b[2024] = (4.0 == 8.0);s.store_scalar(2024, if s.b[2024] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (!s.b[2021])) && (!s.b[2022])) && (!s.b[2023])) && s.b[2024]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) {s.store_scalar(719, 0.0);}
        let mut t21: usize = 0;
        while {
            let t20: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;
            if t21 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t21, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && (!s.b[2020])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1911, 726);s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1911, 780);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2019])) {s.store_sub(336, 109, 1851);s.store_scalar(334, 1.0);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1891, 209, -1.0, 338);}
        s.b[2030] = (s.v[1836] > s.v[965]);s.store_scalar(2030, if s.b[2030] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2030]) {s.copy_ad(981, 1834);}
        s.b[2031] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));s.store_scalar(2031, if s.b[2031] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {s.store_offset(781, 87, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
        let mut t23: usize = 0;
        while {
            let t22: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && (s.v[719] < s.v[1914])) { 1.0 } else { 0.0 };
            t22 != 0.0
        } {
            t23 += 1;
            if t23 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t23, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2032] = ((((s.v[1914] == 1.0) || (s.v[1914] == 2.0)) || (s.v[1914] == 4.0)) || (s.v[1914] == 8.0));s.store_scalar(2032, if s.b[2032] { 1.0 } else { 0.0 });s.b[2033] = (s.v[1914] == 1.0);s.store_scalar(2033, if s.b[2033] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && s.b[2033]) {s.store_scalar(720, 1.0);}
        s.b[2034] = (s.v[1914] == 2.0);s.store_scalar(2034, if s.b[2034] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (!s.b[2033])) && s.b[2034]) {s.store_scalar(720, 2.0);}
        s.b[2035] = (s.v[1914] == 4.0);s.store_scalar(2035, if s.b[2035] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (!s.b[2033])) && (!s.b[2034])) && s.b[2035]) {s.store_scalar(720, 3.0);}
        s.b[2036] = (s.v[1914] == 8.0);s.store_scalar(2036, if s.b[2036] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (!s.b[2033])) && (!s.b[2034])) && (!s.b[2035])) && s.b[2036]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) {s.store_scalar(719, 0.0);}
        let mut t25: usize = 0;
        while {
            let t24: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;
            if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && (!s.b[2032])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1914), 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_offset(983, 780, (-0.1));}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2031])) {s.copy_ad(983, 87);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {s.store_add_scaled_inputs3_offset_indices(1916, 791, 1.0, 85, (-1.0), 1912, 1.0, (-(s.v[462] - p[392])));s.store_sub(1915, 791, 1916);}
        s.b[2037] = ((s.v[1915] > (-s.v[1913])) && (s.v[1913] >= 0.0));s.store_scalar(2037, if s.b[2037] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {s.store_add(781, 1915, 1913);s.store_square(722, 781);s.store_square(723, 1913);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
        let mut t27: usize = 0;
        while {
            let t26: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && (s.v[719] < s.v[1914])) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;
            if t27 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t27, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2038] = ((((s.v[1914] == 1.0) || (s.v[1914] == 2.0)) || (s.v[1914] == 4.0)) || (s.v[1914] == 8.0));s.store_scalar(2038, if s.b[2038] { 1.0 } else { 0.0 });s.b[2039] = (s.v[1914] == 1.0);s.store_scalar(2039, if s.b[2039] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && s.b[2039]) {s.store_scalar(720, 1.0);}
        s.b[2040] = (s.v[1914] == 2.0);s.store_scalar(2040, if s.b[2040] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (!s.b[2039])) && s.b[2040]) {s.store_scalar(720, 2.0);}
        s.b[2041] = (s.v[1914] == 4.0);s.store_scalar(2041, if s.b[2041] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (!s.b[2039])) && (!s.b[2040])) && s.b[2041]) {s.store_scalar(720, 3.0);}
        s.b[2042] = (s.v[1914] == 8.0);s.store_scalar(2042, if s.b[2042] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (!s.b[2039])) && (!s.b[2040])) && (!s.b[2041])) && s.b[2042]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) {s.store_scalar(719, 0.0);}
        let mut t29: usize = 0;
        while {
            let t28: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;
            if t29 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t29, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && (!s.b[2038])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1914), 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1913, 726);s.store_div_scaled_product3_indices(334, 1913, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(1915, 1913, -1.0, 780, 1.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2037])) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2037])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t2b: usize = 0;
        while {
            let t2a: f64 = if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t2a != 0.0
        } {
            t2b += 1;
            if t2b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2043] = (s.v[983] >= 0.0);s.store_scalar(2043, if s.b[2043] { 1.0 } else { 0.0 });
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2043]) {s.store_mul_scaled_sqrt_ad_rhs(2028, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2029, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2028, 1.0);}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2043])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1887)));s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1887);s.store_mul_sqrt_mixed_ia(2028, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2028, 1.0);s.store_mul_add_mixed_iaa(2029, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(1870, 2028, 1.0, 185, 1915, 983, 1.0);s.store_sub(1871, 2029, 185);s.store_div_scaled_inputs_indices(1882, 1870, -1.0, 1871, 1.0);}
            s.b[2044] = (((s.v[1882]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2044, if s.b[2044] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) && s.b[2044]) {s.store_scalar(79, 1.0);}
            s.b[2045] = (s.v[1882] > 0.1);s.store_scalar(2045, if s.b[2045] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) && (!s.b[2044])) && s.b[2045]) {s.store_scalar(1882, 0.1);}
            s.b[2046] = (s.v[1882] < (-0.1));s.store_scalar(2046, if s.b[2046] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) && (!s.b[2044])) && (!s.b[2045])) && s.b[2046]) {s.store_scalar(1882, (-0.1));}
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 1882);}
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {s.store_primal_offset(97, 97, 1.0);}
        }
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2026, 1905, 1834, (0.5 * 9662367879.197212), 0.0, 1834);s.store_scaled_sqrt_mul_scaled_lhs(334, 154, 2.0, 2026, p[394]);s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2027, 335, 2026);s.store_mul(332, 2027, 983);s.store_exp_mul_scaled_lhs_indices(334, 2027, -1.0, 2026);}
        s.b[2048] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2048, if s.b[2048] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2048]) {s.store_mul_exp_lhs(335, 332, 334);s.store_sub(336, 335, 334);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2048])) {s.store_mul_scale_offset_indices(335, 334, 332, 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);}
        s.b[2049] = (((s.v[336]) as f64).abs() > 1e-8);s.store_scalar(2049, if s.b[2049] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2049]) {s.store_div_ln_offset_lhs(2025, 336, 1.0, 2027);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2049])) {s.store_div(2025, 336, 2027);}
        s.b[2050] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2025])) / s.v[1905]) <= 0.0);s.store_scalar(2050, if s.b[2050] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2050]) {s.store_scalar(981, 0.0);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2050])) {s.store_sqrt_ad(981, A::div_scaled_inputs2(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2025), (-(2.0 * 1.034943e-10)), s.ad_value(1905), 1.0));}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
            if (s.v[981] > s.v[1834]) {
                s.copy_ad(981, 1834);
            } else {
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2051] = (s.v[981] < s.v[1834]);s.store_scalar(2051, if s.b[2051] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2051]) {s.store_sub(990, 1834, 981);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2051])) {s.store_scalar(990, 0.0);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_neg_add(1894, 1889, 1890);}
        s.b[2052] = (s.v[94] < 0.0);s.store_scalar(2052, if s.b[2052] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2052]) {s.store_scalar(94, 0.0);s.copy_ad(1854, 1853);s.store_scalar(248, 0.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2052])) {s.store_mul3_affine_lhs(248, 154, 1894, 1.0 / (2.0), 0.0, 94);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2052])) {
            if (s.v[248] < 0.0) {
                s.store_scalar(248, 0.0);
            } else {
            }
        }
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_neg(238, 1891);s.copy_ad(170, 162);s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p[262]), (-((p[262]) as f64).sqrt()));s.store_offset_mul(338, 980, 334, 1.0);s.store_mul(339, 336, 238);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[160] - 1.0));
            }
        }
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul(342, 339, 251);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 238, 343);s.store_scalar(336, s.v[474]);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_inputs(s.ad_value(336), 1.0, s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2053] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2053, if s.b[2053] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2053]) {s.store_scalar(337, 1.0);}
        s.b[2054] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2054, if s.b[2054] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2053])) && s.b[2054]) {s.copy_ad(337, 335);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2053])) && (!s.b[2054])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p[178] - 1.0));
            }
        }
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[2055] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2055, if s.b[2055] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2055]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2056] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2056, if s.b[2056] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2055])) && s.b[2056]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2055])) && (!s.b[2056])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p[178]) - 1.0));
            }
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2055])) && (!s.b[2056])) {s.store_mul(339, 338, 340);}
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {s.store_mul(253, 254, 339);s.copy_ad(984, 253);s.copy_ad(1886, 255);s.copy_ad(989, 349);}
        s.b[2057] = (s.v[349] > 1e-6);s.store_scalar(2057, if s.b[2057] { 1.0 } else { 0.0 });
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {s.store_scaled_add(344, 1887, 155, p[396]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {s.store_offset_mul_ad(338, s.ad_value(1907), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1907, 1.0);}
        s.b[2058] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2058, if s.b[2058] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2059] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2059, if s.b[2059] { 1.0 } else { 0.0 });s.b[2060] = (2.0 == 1.0);s.store_scalar(2060, if s.b[2060] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && s.b[2060]) {s.store_scalar(720, 1.0);}
        s.b[2061] = (2.0 == 2.0);s.store_scalar(2061, if s.b[2061] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (!s.b[2060])) && s.b[2061]) {s.store_scalar(720, 2.0);}
        s.b[2062] = (2.0 == 4.0);s.store_scalar(2062, if s.b[2062] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (!s.b[2060])) && (!s.b[2061])) && s.b[2062]) {s.store_scalar(720, 3.0);}
        s.b[2063] = (2.0 == 8.0);s.store_scalar(2063, if s.b[2063] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (!s.b[2060])) && (!s.b[2061])) && (!s.b[2062])) && s.b[2063]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) {s.store_scalar(719, 0.0);}
        let mut t2d: usize = 0;
        while {
            let t2c: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2c != 0.0
        } {
            t2d += 1;
            if t2d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && (!s.b[2059])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && (!s.b[2058])) {
        }
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && (!s.b[2058])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1906, 1.0, 337);}
        s.b[2064] = ((s.v[344] < (s.v[972] + s.v[1910])) && (s.v[1910] >= 0.0));s.store_scalar(2064, if s.b[2064] { 1.0 } else { 0.0 });
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) {s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1910, 1.0, 344, -1.0);s.store_square(722, 781);s.store_square(723, 1910);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2065] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2065, if s.b[2065] { 1.0 } else { 0.0 });s.b[2066] = (2.0 == 1.0);s.store_scalar(2066, if s.b[2066] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && s.b[2066]) {s.store_scalar(720, 1.0);}
        s.b[2067] = (2.0 == 2.0);s.store_scalar(2067, if s.b[2067] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && (!s.b[2066])) && s.b[2067]) {s.store_scalar(720, 2.0);}
        s.b[2068] = (2.0 == 4.0);s.store_scalar(2068, if s.b[2068] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && (!s.b[2066])) && (!s.b[2067])) && s.b[2068]) {s.store_scalar(720, 3.0);}
        s.b[2069] = (2.0 == 8.0);s.store_scalar(2069, if s.b[2069] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && (!s.b[2066])) && (!s.b[2067])) && (!s.b[2068])) && s.b[2069]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) {s.store_scalar(719, 0.0);}
    }
}
