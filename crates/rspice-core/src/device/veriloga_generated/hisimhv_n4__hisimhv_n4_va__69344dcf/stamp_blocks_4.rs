#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && s.b[1440]) && s.b[1817]) {s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1457), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[1818] = (s.v[336] < 0.0);s.store_scalar(1818, if s.b[1818] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1817]) && s.b[1818]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1439] && s.b[1440]) && s.b[1817]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p.p284);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1435, p.p285, 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 1457, 1.0, 340, 1.0, 1434, -1.0);s.store_add_product3_rhs_indices(338, 338, 1435, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1817])) {s.store_scalar(343, 0.0);}
        s.b[1819] = (p.p287 != 0.0);s.store_scalar(1819, if s.b[1819] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1819]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1435);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1819])) {s.store_scalar(342, 0.0);}
        s.b[1820] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(1820, if s.b[1820] { 1.0 } else { 0.0 });
        if ((s.b[1439] && s.b[1440]) && s.b[1820]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        s.b[1821] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(1821, if s.b[1821] { 1.0 } else { 0.0 });s.b[1822] = (p.p296 > 0.0);s.store_scalar(1822, if s.b[1822] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p.p296 + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1822]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1822])) {s.copy_ad(341, 647);}
        s.b[1823] = (s.v[793] >= 0.0);s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1823]) {s.copy_ad(369, 793);}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1823])) {s.store_scalar(369, 0.0);}
        s.b[1824] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && s.b[1824]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if (((s.b[1439] && s.b[1440]) && s.b[1821]) && (!s.b[1824])) {s.store_powf_offset_input(335, 369, 1e-12, p.p297);}
        if ((s.b[1439] && s.b[1440]) && s.b[1821]) {s.store_powf_offset_input(343, 369, 1e-12, p.p299);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if ((s.b[1439] && s.b[1440]) && (!s.b[1821])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        if (s.b[1439] && s.b[1440]) {s.store_add_scaled_inputs4_indices(131, 1473, (-0.5), 1474, (-0.5), 1494, (-0.5), 1496, (-0.5));s.store_scaled_add_mixed_ai(133, A::add(A::add_scaled_inputs4(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(1513), 1.0, s.ad_value(1514), 1.0), s.ad_value(1493)), 1495, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 1534, 1535, (-0.5));s.store_neg(238, 1534);s.copy_ad(255, 1554);}
        s.b[1825] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(1825, if s.b[1825] { 1.0 } else { 0.0 });
        let (t0,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1825]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t0);
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.copy_ad(1851, 960);s.store_scale(1901, 964, 1.6021918e-19);s.store_scale(1880, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1900, 622, 1.6021918e-19);s.store_square(1899, 965);s.store_div_from_scalar(1904, (2.0 * 1.034943e-10), 1901);s.store_div_from_scalar(1905, (2.0 * 1.034943e-10), 1900);s.store_div(1898, 964, 622);s.store_div_from_scalar_offset_input(1897, 1.0, 1898, 1.0);s.store_div_square_rhs(1902, 1880, 185);s.store_div_from_scalar(1903, 2.0, 1902);s.store_scalar(1906, 4.0);s.store_scalar(1907, 0.1);s.store_scalar(1908, 0.1);s.store_offset(1909, 961, p.p407);s.store_scalar(1910, 3.0);s.store_scalar(1849, 0.0);s.store_scalar(1850, 0.0);s.store_scalar(1858, 0.0);s.store_scalar(1859, 0.0);s.store_scalar(1891, 0.0);s.store_scalar(1892, 0.0);s.store_scalar(1862, 0.0);s.store_scalar(1864, 0.0);s.store_scalar(1863, 0.0);s.store_scalar(1865, 0.0);s.store_scalar(1835, 0.0);s.store_scalar(1830, 0.0);s.copy_ad(1883, 1431);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));s.store_div_scaled_product_add_scaled_denominator_indices(962, 1904, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);s.store_sub(335, 1851, 1434);}
        s.b[1913] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1913, if s.b[1913] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t1,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t1);
        let (t2,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1914] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1914, if s.b[1914] { 1.0 } else { 0.0 });s.b[1915] = (4.0 == 1.0);s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && s.b[1915]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1916] = (4.0 == 2.0);s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && s.b[1916]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[1917] = (4.0 == 4.0);s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && s.b[1917]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);s.b[1918] = (4.0 == 8.0);s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });
        let (t6,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && (!s.b[1917])) && s.b[1918]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6);
        let (t7,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7);let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {s.store_sqrt(726, 726);}
            let (t9,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {
        let t8: f64 = (s.v[719] + 1.0);
        (t8,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t9);
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && (!s.b[1914])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1913])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(1834, 962, 336);s.store_sqrt(1832, 1834);}
        s.b[1919] = (p.p345 != 0.0);s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {s.store_mul_scale_offset_mixed_ia(335, 965, A::scale(s.ad_value(790), p.p345), -1.0, 1.0);s.store_scale(336, 965, 0.001);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_scalar(1847, 0.0);}
        s.b[1920] = (s.v[1832] > s.v[965]);s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1920]) {s.copy_ad(1831, 965);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1920])) {s.copy_ad(1831, 1832);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));s.store_scalar(782, ((4.0 * 0.3) * 0.01));}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(1856, 781, (-0.5), 782, (-0.5), 0.3);s.store_add_scaled_inputs3_offset_indices(781, 1856, 1.0, 1883, -1.0, 1851, 1.0, (-0.01));s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1856, 1883, 1.0, 1851, (-1.0), 781, 0.5, 782, 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(1884, 1851, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);s.store_offset_sub(1830, 965, 1831, 1e-15);}
        let (tc,) = {
    if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, tc);
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_scalar(1846, 0.2);s.copy_ad(1849, 1856);s.copy_ad(1852, 1847);s.copy_ad(1854, 1884);}
        let (td,) = {
    if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, td);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t40: usize = 0;
        while {
            let t3f: f64 = if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t3f != 0.0
        } {
            t40 += 1;assert!(t40 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul_sub_mixed_iai(1854, 1897, A::add_scaled_product(s.ad_value(1883), 1.0, s.ad_value(1898), s.ad_value(1852), 1.0), 1851);s.store_mul(1838, 1897, 1898);s.store_sub(335, 1852, 1854);}
            s.b[1921] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t3e,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t3e);
            let (t10,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t10);
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1922] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });s.b[1923] = (2.0 == 1.0);s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });
            let (t28,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && s.b[1923]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t28);s.b[1924] = (2.0 == 2.0);s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });
            let (t29,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && s.b[1924]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t29);s.b[1925] = (2.0 == 4.0);s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });
            let (t2a,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && s.b[1925]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2a);s.b[1926] = (2.0 == 8.0);s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });
            let (t2b,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && (!s.b[1925])) && s.b[1926]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2b);
            let (t2c,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2c);let mut t30: usize = 0;
            while {
                let t2f: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2f != 0.0
            } {
                t30 += 1;assert!(t30 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {s.store_sqrt(726, 726);}
                let (t2e,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
        let t2d: f64 = (s.v[719] + 1.0);
        (t2d,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t2e);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && (!s.b[1922])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1921])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_mul(1826, 1904, 336);}
            s.b[1927] = ((s.v[1826] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {s.store_offset_sub(781, 1826, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t31,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t31);
            let (t32,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t32);
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1928] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });s.b[1929] = (2.0 == 1.0);s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });
            let (t33,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && s.b[1929]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t33);s.b[1930] = (2.0 == 2.0);s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });
            let (t34,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && s.b[1930]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t34);s.b[1931] = (2.0 == 4.0);s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });
            let (t35,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && s.b[1931]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t35);s.b[1932] = (2.0 == 8.0);s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });
            let (t36,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && (!s.b[1931])) && s.b[1932]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t36);
            let (t37,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t37);let mut t3b: usize = 0;
            while {
                let t3a: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t3a != 0.0
            } {
                t3b += 1;assert!(t3b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {s.store_sqrt(726, 726);}
                let (t39,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
        let t38: f64 = (s.v[719] + 1.0);
        (t38,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t39);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && (!s.b[1928])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1826, 965, (-1e-8), 780);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {s.store_scalar(337, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(1862, 1826, 1901);s.store_mul_ad_product_lhs_mixed_ai(1840, A::div_from_scalar(1.034943e-10, s.ad_value(1826)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1842, A::div_from_scalar((-1.034943e-10), s.ad_value(1826)), 334, 337);}
            s.b[1933] = (p.p49 == 0.0);s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1933]) {s.store_add_mixed_ai(1835, A::div_scaled_inputs_product(s.ad_value(1899), 1.0, s.ad_value(1834), 1.0, s.ad_value(965), s.ad_value(1831), (-2.0), s.ad_value(1904), 1.0), 1849);s.store_scalar(1836, 1.0);s.store_scalar(1837, 0.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1933])) {s.store_add_mixed_ia(1835, 1849, A::div_scaled_add_product(s.ad_value(1899), 1.0, s.ad_value(1826), A::sub_scaled_inputs(s.ad_value(1826), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1904), 1.0));s.store_scalar(1836, 1.0);s.store_mul_scale_offset_mixed_ai(1837, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1826)), s.ad_value(334), (-1.0)), 1838, -1.0, 1.0);}
            s.b[1934] = ((s.v[1835] > (s.v[1847] - s.v[1846])) && (s.v[1846] >= 0.0));s.store_scalar(1934, if s.b[1934] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {s.store_add_scaled_inputs3_indices(781, 1835, 1.0, 1847, (-1.0), 1846, 1.0);s.store_square(722, 781);s.store_square(723, 1846);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t3c,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t3c);
            let (t3d,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t3d);
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1935] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1935, if s.b[1935] { 1.0 } else { 0.0 });s.b[1936] = (4.0 == 1.0);s.store_scalar(1936, if s.b[1936] { 1.0 } else { 0.0 });
            let (te,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && s.b[1936]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, te);s.b[1937] = (4.0 == 2.0);s.store_scalar(1937, if s.b[1937] { 1.0 } else { 0.0 });
            let (tf,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && s.b[1937]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tf);s.b[1938] = (4.0 == 4.0);s.store_scalar(1938, if s.b[1938] { 1.0 } else { 0.0 });
            let (t11,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && s.b[1938]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t11);s.b[1939] = (4.0 == 8.0);s.store_scalar(1939, if s.b[1939] { 1.0 } else { 0.0 });
            let (t12,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && (!s.b[1938])) && s.b[1939]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t12);
            let (t13,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t13);let mut t17: usize = 0;
            while {
                let t16: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t16 != 0.0
            } {
                t17 += 1;assert!(t17 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {s.store_sqrt(726, 726);}
                let (t15,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
        let t14: f64 = (s.v[719] + 1.0);
        (t14,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t15);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && (!s.b[1935])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1846, 726);s.store_div_scaled_product3_indices(334, 1846, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1835, 1847, 1.0, 1846, (-1.0), 780, 1.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {s.store_scalar(334, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(1836, 1836, 334);s.store_mul(1837, 1837, 334);s.store_add_scaled_inputs3_indices(335, 1854, 1.0, 1883, (-1.0), 1851, 1.0);}
            s.b[1940] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1940, if s.b[1940] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t18,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t18);
            let (t19,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t19);
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1941] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1941, if s.b[1941] { 1.0 } else { 0.0 });s.b[1942] = (2.0 == 1.0);s.store_scalar(1942, if s.b[1942] { 1.0 } else { 0.0 });
            let (t1a,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && s.b[1942]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1a);s.b[1943] = (2.0 == 2.0);s.store_scalar(1943, if s.b[1943] { 1.0 } else { 0.0 });
            let (t1b,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && s.b[1943]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1b);s.b[1944] = (2.0 == 4.0);s.store_scalar(1944, if s.b[1944] { 1.0 } else { 0.0 });
            let (t1c,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && s.b[1944]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1c);s.b[1945] = (2.0 == 8.0);s.store_scalar(1945, if s.b[1945] { 1.0 } else { 0.0 });
            let (t1d,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && (!s.b[1944])) && s.b[1945]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1d);
            let (t1e,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1e);let mut t22: usize = 0;
            while {
                let t21: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t21 != 0.0
            } {
                t22 += 1;assert!(t22 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {s.store_sqrt(726, 726);}
                let (t20,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
        let t1f: f64 = (s.v[719] + 1.0);
        (t1f,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t20);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && (!s.b[1941])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1940])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sqrt_mul(1828, 1905, 336);s.store_mul_scale_offset_indices(1863, 1900, 1828, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1844, (-1.034943e-10), 1828, 337);s.store_mul_sub_rhs(335, 154, 1849, 1852);s.store_exp(336, 335);}
            s.b[1946] = (s.v[1849] >= s.v[1852]);s.store_scalar(1946, if s.b[1946] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1946]) {s.store_mul_scaled_sqrt_ad_rhs(1858, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1893, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1858, 1.0);s.store_neg(1895, 1893);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1946])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1849), s.ad_value(1883)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1852), s.ad_value(1883)));s.store_mul_sqrt_mixed_ia(1858, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1858, 1.0);s.store_mul_add_mixed_iaa(1893, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1895, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            let (t24,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] != 0.0)) {
        let t23: f64 = (150.0 + 1.0);
        (t23,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t24);
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1866, A::add_scaled_product(s.ad_value(1858), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1849)), 1.0), 1.0, 1862, 1.0, 1863, 1.0);s.store_sub(1867, 1893, 185);s.store_add_mixed_ia(1868, 1895, A::add_scaled_value_products(s.ad_value(1840), 1.0, s.ad_value(1842), s.ad_value(1838), 1.0, s.ad_value(1844), s.ad_value(1838), 1.0));s.store_sub(1869, 1852, 1835);s.store_neg(1870, 1836);s.store_sub_from_scalar(1871, 1.0, 1837);s.store_add_scaled_products_indices(1872, 1867, 1871, 1.0, 1868, 1870, (-1.0));}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                if (s.v[1872] > 0.0) {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, (-1e-25));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {s.copy_ad(1874, 1871);s.store_neg(1875, 1868);s.store_neg(1876, 1870);s.copy_ad(1877, 1867);s.store_mul_add_scaled_products_indices_rhs(1878, 1873, 1874, 1866, -1.0, 1875, 1869, -1.0);s.store_mul_add_scaled_products_indices_rhs(1879, 1873, 1876, 1866, -1.0, 1877, 1869, -1.0);s.store_abs(335, 1878);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                    s.store_abs(335, 1879);
                } else {
                }
            }
            s.b[1947] = (s.v[335] > 0.1);s.store_scalar(1947, if s.b[1947] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1947]) {s.store_mul_div_from_scalar_lhs_ad_indices(1878, 0.1, 335, 1878);s.store_mul_div_from_scalar_lhs_ad_indices(1879, 0.1, 335, 1879);}
            s.b[1948] = (s.v[335] < 1e-12);s.store_scalar(1948, if s.b[1948] { 1.0 } else { 0.0 });
            let (t25,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1948]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t25);
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {s.store_add(1849, 1849, 1878);s.store_add(1852, 1852, 1879);}
            let (t27,) = {
    if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
        let t26: f64 = (s.v[97] + 1.0);
        (t26,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t27);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul_sub_rhs(335, 154, 1849, 1852);s.store_exp(336, 335);}
        s.b[1950] = (s.v[1849] >= s.v[1852]);s.store_scalar(1950, if s.b[1950] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1950]) {s.copy_ad(1888, 1858);s.store_scalar(1891, 0.0);s.store_scalar(1860, 0.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) {s.store_scalar(1888, 0.0);s.store_mul_sqrt_mixed_ia(1891, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[1951] = (s.v[1832] > s.v[965]);s.store_scalar(1951, if s.b[1951] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) && s.b[1951]) {s.store_scalar(1860, 0.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) && (!s.b[1951])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1849), s.ad_value(1883)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1883)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1860, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[1952] = (((s.v[1849] - s.v[1847]) < s.v[1907]) && (s.v[1907] >= 0.0));s.store_scalar(1952, if s.b[1952] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {s.store_add_scaled_inputs3_indices(781, 1907, 1.0, 1849, -1.0, 1847, 1.0);s.store_square(722, 781);s.store_square(723, 1907);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t41,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t41);
        let (t42,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t42);
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1953] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1953, if s.b[1953] { 1.0 } else { 0.0 });s.b[1954] = (4.0 == 1.0);s.store_scalar(1954, if s.b[1954] { 1.0 } else { 0.0 });
        let (t43,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && s.b[1954]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t43);s.b[1955] = (4.0 == 2.0);s.store_scalar(1955, if s.b[1955] { 1.0 } else { 0.0 });
        let (t44,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && s.b[1955]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t44);s.b[1956] = (4.0 == 4.0);s.store_scalar(1956, if s.b[1956] { 1.0 } else { 0.0 });
        let (t45,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && s.b[1956]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t45);s.b[1957] = (4.0 == 8.0);s.store_scalar(1957, if s.b[1957] { 1.0 } else { 0.0 });
        let (t46,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && (!s.b[1956])) && s.b[1957]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t46);
        let (t47,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t47);let mut t4b: usize = 0;
        while {
            let t4a: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4a != 0.0
        } {
            t4b += 1;assert!(t4b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {s.store_sqrt(726, 726);}
            let (t49,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {
        let t48: f64 = (s.v[719] + 1.0);
        (t48,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t49);
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && (!s.b[1953])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1907, 726);s.store_div_scaled_product3_indices(334, 1907, 725, 726, 1.0, 770, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {s.store_sub(336, 1907, 780);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1952])) {s.store_sub(336, 1849, 1847);s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1885, 209, -1.0, 338);s.copy_ad(349, 790);}
        s.b[1958] = (s.v[790] > 1e-6);s.store_scalar(1958, if s.b[1958] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {s.store_scalar(344, 1e-25);s.store_offset_mul_ad(338, s.ad_value(1903), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1903, 1.0);}
        s.b[1959] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(1959, if s.b[1959] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t4c,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t4c);
        let (t4d,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4d);
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1960] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1960, if s.b[1960] { 1.0 } else { 0.0 });s.b[1961] = (2.0 == 1.0);s.store_scalar(1961, if s.b[1961] { 1.0 } else { 0.0 });
        let (t4e,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && s.b[1961]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4e);s.b[1962] = (2.0 == 2.0);s.store_scalar(1962, if s.b[1962] { 1.0 } else { 0.0 });
        let (t4f,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && s.b[1962]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4f);s.b[1963] = (2.0 == 4.0);s.store_scalar(1963, if s.b[1963] { 1.0 } else { 0.0 });
        let (t50,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && s.b[1963]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t50);s.b[1964] = (2.0 == 8.0);s.store_scalar(1964, if s.b[1964] { 1.0 } else { 0.0 });
        let (t51,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && (!s.b[1963])) && s.b[1964]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t51);
        let (t52,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t52);let mut t56: usize = 0;
        while {
            let t55: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t55 != 0.0
        } {
            t56 += 1;assert!(t56 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {s.store_sqrt(726, 726);}
            let (t54,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {
        let t53: f64 = (s.v[719] + 1.0);
        (t53,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t54);
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && (!s.b[1960])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1959])) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1959])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1902, 1.0, 337);}
        s.b[1965] = ((s.v[344] < 1.0) && (1.0 >= 0.0));s.store_scalar(1965, if s.b[1965] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {s.store_sub_from_scalar(781, 1.0, 344);s.store_square(722, 781);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t57,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t57);
        let (t58,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t58);
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1966] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1966, if s.b[1966] { 1.0 } else { 0.0 });s.b[1967] = (2.0 == 1.0);s.store_scalar(1967, if s.b[1967] { 1.0 } else { 0.0 });
        let (t59,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && s.b[1967]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t59);s.b[1968] = (2.0 == 2.0);s.store_scalar(1968, if s.b[1968] { 1.0 } else { 0.0 });
        let (t5a,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && s.b[1968]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5a);s.b[1969] = (2.0 == 4.0);s.store_scalar(1969, if s.b[1969] { 1.0 } else { 0.0 });
        let (t5b,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && (!s.b[1968])) && s.b[1969]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5b);s.b[1970] = (2.0 == 8.0);s.store_scalar(1970, if s.b[1970] { 1.0 } else { 0.0 });
        let (t5c,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && (!s.b[1968])) && (!s.b[1969])) && s.b[1970]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5c);
        let (t5d,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t5d);let mut t61: usize = 0;
        while {
            let t60: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t60 != 0.0
        } {
            t61 += 1;assert!(t61 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {s.store_sqrt(726, 726);}
            let (t5f,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {
        let t5e: f64 = (s.v[719] + 1.0);
        (t5e,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t5f);
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && (!s.b[1966])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1.0);s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(344, 1.0, 780);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1965])) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1965])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {s.store_div(335, 790, 344);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);s.copy_ad(790, 348);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1958])) {s.copy_ad(348, 790);}
        s.b[1971] = (s.v[790] < 0.0);s.store_scalar(1971, if s.b[1971] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1971]) {s.copy_ad(1850, 1849);s.copy_ad(1855, 1854);s.copy_ad(1853, 1852);s.copy_ad(1861, 1860);s.copy_ad(1889, 1888);s.copy_ad(1886, 1885);s.copy_ad(1864, 1862);s.copy_ad(1865, 1863);}
        let (t62,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        (s.v[1832],)
    } else {
        (s.v[1833],)
    }
};
        s.store_scalar(1833, t62);
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.copy_ad(1848, 790);s.store_add_scaled_inputs3_offset_indices(781, 1849, 1.0, 1848, 1.0, 85, -1.0, (-0.01));s.store_scaled_add(782, 1849, 1848, (4.0 * 0.01));}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1857, 1849, 1.0, 1848, 1.0, 781, (-0.5), 782, (-0.5));s.store_add_scaled_inputs3_offset_indices(781, 1857, 1.0, 1883, -1.0, 1851, 1.0, (-0.01));s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1857, 1883, 1.0, 1851, (-1.0), 781, 0.5, 782, 0.5);s.store_mul(212, 209, 186);s.store_square(213, 212);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1883))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_add_product3_rhs_mixed_iia(92, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);}
        let (t63,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t63);
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.copy_ad(1850, 1857);s.copy_ad(1853, 1848);}
        let (t64,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t64);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t97: usize = 0;
        while {
            let t96: f64 = if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t96 != 0.0
        } {
            t97 += 1;assert!(t97 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_mul_sub_mixed_iai(1855, 1897, A::add_scaled_product(s.ad_value(1883), 1.0, s.ad_value(1898), s.ad_value(1853), 1.0), 1851);s.store_mul(1839, 1897, 1898);s.store_sub(335, 1853, 1855);}
            s.b[1972] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1972, if s.b[1972] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t95,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t95);
            let (t67,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t67);
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1973] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1973, if s.b[1973] { 1.0 } else { 0.0 });s.b[1974] = (2.0 == 1.0);s.store_scalar(1974, if s.b[1974] { 1.0 } else { 0.0 });
            let (t7f,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && s.b[1974]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t7f);s.b[1975] = (2.0 == 2.0);s.store_scalar(1975, if s.b[1975] { 1.0 } else { 0.0 });
            let (t80,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && s.b[1975]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t80);s.b[1976] = (2.0 == 4.0);s.store_scalar(1976, if s.b[1976] { 1.0 } else { 0.0 });
            let (t81,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && (!s.b[1975])) && s.b[1976]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t81);s.b[1977] = (2.0 == 8.0);s.store_scalar(1977, if s.b[1977] { 1.0 } else { 0.0 });
            let (t82,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && (!s.b[1975])) && (!s.b[1976])) && s.b[1977]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t82);
            let (t83,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t83);let mut t87: usize = 0;
            while {
                let t86: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t86 != 0.0
            } {
                t87 += 1;assert!(t87 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {s.store_sqrt(726, 726);}
                let (t85,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {
        let t84: f64 = (s.v[719] + 1.0);
        (t84,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t85);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && (!s.b[1973])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1972])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_sqrt_mul(1827, 1904, 336);}
            s.b[1978] = ((s.v[1827] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1978, if s.b[1978] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {s.store_offset_sub(781, 1827, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t88,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t88);
            let (t89,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t89);
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1979] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1979, if s.b[1979] { 1.0 } else { 0.0 });s.b[1980] = (2.0 == 1.0);s.store_scalar(1980, if s.b[1980] { 1.0 } else { 0.0 });
            let (t8a,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && s.b[1980]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t8a);s.b[1981] = (2.0 == 2.0);s.store_scalar(1981, if s.b[1981] { 1.0 } else { 0.0 });
            let (t8b,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && s.b[1981]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t8b);s.b[1982] = (2.0 == 4.0);s.store_scalar(1982, if s.b[1982] { 1.0 } else { 0.0 });
            let (t8c,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && (!s.b[1981])) && s.b[1982]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t8c);s.b[1983] = (2.0 == 8.0);s.store_scalar(1983, if s.b[1983] { 1.0 } else { 0.0 });
            let (t8d,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && (!s.b[1981])) && (!s.b[1982])) && s.b[1983]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t8d);
            let (t8e,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8e);let mut t92: usize = 0;
            while {
                let t91: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t91 != 0.0
            } {
                t92 += 1;assert!(t92 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {s.store_sqrt(726, 726);}
                let (t90,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {
        let t8f: f64 = (s.v[719] + 1.0);
        (t8f,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t90);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && (!s.b[1979])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1827, 965, (-1e-8), 780);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1978])) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1978])) {s.store_scalar(337, 1.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_mul(1864, 1827, 1901);s.store_mul_ad_product_lhs_mixed_ai(1841, A::div_from_scalar(1.034943e-10, s.ad_value(1827)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1843, A::div_from_scalar((-1.034943e-10), s.ad_value(1827)), 334, 337);}
            s.b[1984] = (p.p49 == 0.0);s.store_scalar(1984, if s.b[1984] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1984]) {s.store_add_mixed_ai(1835, A::div_scaled_inputs_product(s.ad_value(1899), 1.0, s.ad_value(1834), 1.0, s.ad_value(965), s.ad_value(1831), (-2.0), s.ad_value(1904), 1.0), 1850);s.store_scalar(1836, 1.0);s.store_scalar(1837, 0.0);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1984])) {s.store_add_mixed_ia(1835, 1850, A::div_scaled_add_product(s.ad_value(1899), 1.0, s.ad_value(1827), A::sub_scaled_inputs(s.ad_value(1827), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1904), 1.0));s.store_scalar(1836, 1.0);s.store_mul_scale_offset_mixed_ai(1837, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1827)), s.ad_value(334), (-1.0)), 1839, -1.0, 1.0);}
            s.b[1985] = ((s.v[1835] > (s.v[1848] - s.v[1846])) && (s.v[1846] >= 0.0));s.store_scalar(1985, if s.b[1985] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {s.store_add_scaled_inputs3_indices(781, 1835, 1.0, 1848, (-1.0), 1846, 1.0);s.store_square(722, 781);s.store_square(723, 1846);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t93,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t93);
            let (t94,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t94);
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1986] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1986, if s.b[1986] { 1.0 } else { 0.0 });s.b[1987] = (4.0 == 1.0);s.store_scalar(1987, if s.b[1987] { 1.0 } else { 0.0 });
            let (t65,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && s.b[1987]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t65);s.b[1988] = (4.0 == 2.0);s.store_scalar(1988, if s.b[1988] { 1.0 } else { 0.0 });
            let (t66,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && s.b[1988]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t66);s.b[1989] = (4.0 == 4.0);s.store_scalar(1989, if s.b[1989] { 1.0 } else { 0.0 });
            let (t68,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) && s.b[1989]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t68);s.b[1990] = (4.0 == 8.0);s.store_scalar(1990, if s.b[1990] { 1.0 } else { 0.0 });
            let (t69,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) && (!s.b[1989])) && s.b[1990]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t69);
            let (t6a,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t6a);let mut t6e: usize = 0;
            while {
                let t6d: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t6d != 0.0
            } {
                t6e += 1;assert!(t6e <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {s.store_sqrt(726, 726);}
                let (t6c,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {
        let t6b: f64 = (s.v[719] + 1.0);
        (t6b,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t6c);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && (!s.b[1986])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1846, 726);s.store_div_scaled_product3_indices(334, 1846, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1835, 1848, 1.0, 1846, (-1.0), 780, 1.0);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1985])) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1985])) {s.store_scalar(334, 1.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_mul(1836, 1836, 334);s.store_mul(1837, 1837, 334);s.store_add_scaled_inputs3_indices(335, 1855, 1.0, 1883, (-1.0), 1851, 1.0);}
            s.b[1991] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1991, if s.b[1991] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t6f,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t6f);
            let (t70,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t70);
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1992] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1992, if s.b[1992] { 1.0 } else { 0.0 });s.b[1993] = (2.0 == 1.0);s.store_scalar(1993, if s.b[1993] { 1.0 } else { 0.0 });
            let (t71,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && s.b[1993]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t71);s.b[1994] = (2.0 == 2.0);s.store_scalar(1994, if s.b[1994] { 1.0 } else { 0.0 });
            let (t72,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && s.b[1994]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t72);s.b[1995] = (2.0 == 4.0);s.store_scalar(1995, if s.b[1995] { 1.0 } else { 0.0 });
            let (t73,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && (!s.b[1994])) && s.b[1995]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t73);s.b[1996] = (2.0 == 8.0);s.store_scalar(1996, if s.b[1996] { 1.0 } else { 0.0 });
            let (t74,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && (!s.b[1994])) && (!s.b[1995])) && s.b[1996]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t74);
            let (t75,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t75);let mut t79: usize = 0;
            while {
                let t78: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t78 != 0.0
            } {
                t79 += 1;assert!(t79 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {s.store_sqrt(726, 726);}
                let (t77,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {
        let t76: f64 = (s.v[719] + 1.0);
        (t76,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t77);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && (!s.b[1992])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1991])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_sqrt_mul(1829, 1905, 336);s.store_mul_scale_offset_indices(1865, 1900, 1829, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1845, (-1.034943e-10), 1829, 337);s.store_mul_sub_rhs(335, 154, 1850, 1853);s.store_exp(336, 335);}
            s.b[1997] = (s.v[1850] >= s.v[1853]);s.store_scalar(1997, if s.b[1997] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1997]) {s.store_mul_scaled_sqrt_ad_rhs(1859, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1894, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1859, 1.0);s.store_neg(1896, 1894);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1997])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1850), s.ad_value(1883)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1853), s.ad_value(1883)));s.store_mul_sqrt_mixed_ia(1859, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1859, 1.0);s.store_mul_add_mixed_iaa(1894, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1896, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            let (t7b,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] != 0.0)) {
        let t7a: f64 = (150.0 + 1.0);
        (t7a,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t7b);
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1866, A::add_scaled_product(s.ad_value(1859), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1850)), 1.0), 1.0, 1864, 1.0, 1865, 1.0);s.store_sub(1867, 1894, 185);s.store_add_mixed_ia(1868, 1896, A::add_scaled_value_products(s.ad_value(1841), 1.0, s.ad_value(1843), s.ad_value(1839), 1.0, s.ad_value(1845), s.ad_value(1839), 1.0));s.store_sub(1869, 1853, 1835);s.store_neg(1870, 1836);s.store_sub_from_scalar(1871, 1.0, 1837);s.store_add_scaled_products_indices(1872, 1867, 1871, 1.0, 1868, 1870, (-1.0));}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                if (s.v[1872] > 0.0) {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, (-1e-25));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {s.copy_ad(1874, 1871);s.store_neg(1875, 1868);s.store_neg(1876, 1870);s.copy_ad(1877, 1867);s.store_mul_add_scaled_products_indices_rhs(1878, 1873, 1874, 1866, -1.0, 1875, 1869, -1.0);s.store_mul_add_scaled_products_indices_rhs(1879, 1873, 1876, 1866, -1.0, 1877, 1869, -1.0);s.store_abs(335, 1878);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                    s.store_abs(335, 1879);
                } else {
                }
            }
            s.b[1998] = (s.v[335] > 0.1);s.store_scalar(1998, if s.b[1998] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) && s.b[1998]) {s.store_mul_div_from_scalar_lhs_ad_indices(1878, 0.1, 335, 1878);s.store_mul_div_from_scalar_lhs_ad_indices(1879, 0.1, 335, 1879);}
            s.b[1999] = (s.v[335] < 1e-12);s.store_scalar(1999, if s.b[1999] { 1.0 } else { 0.0 });
            let (t7c,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) && s.b[1999]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t7c);
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {s.store_add(1850, 1850, 1878);s.store_add(1853, 1853, 1879);}
            let (t7e,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        let t7d: f64 = (s.v[97] + 1.0);
        (t7d,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t7e);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_mul_sub_rhs(335, 154, 1850, 1853);s.store_exp(336, 335);}
        s.b[2001] = (s.v[1850] >= s.v[1853]);s.store_scalar(2001, if s.b[2001] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2001]) {s.copy_ad(1889, 1859);s.store_scalar(1892, 0.0);s.store_scalar(1861, 0.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2001])) {s.store_scalar(1889, 0.0);s.store_mul_sqrt_mixed_ia(1892, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[2002] = (s.v[1833] > s.v[965]);s.store_scalar(2002, if s.b[2002] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2001])) && s.b[2002]) {s.store_scalar(1861, 0.0);}
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2001])) && (!s.b[2002])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1850), s.ad_value(1883)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1853), s.ad_value(1883)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1861, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[2003] = (((s.v[1850] - s.v[1848]) < s.v[1907]) && (s.v[1907] >= 0.0));s.store_scalar(2003, if s.b[2003] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {s.store_add_scaled_inputs3_indices(781, 1907, 1.0, 1850, -1.0, 1848, 1.0);s.store_square(722, 781);s.store_square(723, 1907);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t98,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t98);
        let (t99,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t99);
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2004] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2004, if s.b[2004] { 1.0 } else { 0.0 });s.b[2005] = (4.0 == 1.0);s.store_scalar(2005, if s.b[2005] { 1.0 } else { 0.0 });
        let (t9a,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && s.b[2005]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9a);s.b[2006] = (4.0 == 2.0);s.store_scalar(2006, if s.b[2006] { 1.0 } else { 0.0 });
        let (t9b,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && s.b[2006]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9b);s.b[2007] = (4.0 == 4.0);s.store_scalar(2007, if s.b[2007] { 1.0 } else { 0.0 });
        let (t9c,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && (!s.b[2006])) && s.b[2007]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9c);s.b[2008] = (4.0 == 8.0);s.store_scalar(2008, if s.b[2008] { 1.0 } else { 0.0 });
        let (t9d,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && (!s.b[2006])) && (!s.b[2007])) && s.b[2008]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9d);
        let (t9e,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9e);let mut ta2: usize = 0;
        while {
            let ta1: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta1 != 0.0
        } {
            ta2 += 1;assert!(ta2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {s.store_sqrt(726, 726);}
            let (ta0,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {
        let t9f: f64 = (s.v[719] + 1.0);
        (t9f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta0);
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && (!s.b[2004])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1907, 726);s.store_div_scaled_product3_indices(334, 1907, 725, 726, 1.0, 770, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {s.store_sub(336, 1907, 780);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2003])) {s.store_sub(336, 1850, 1848);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1886, 209, -1.0, 338);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.copy_ad(87, 1849);s.copy_ad(91, 1850);s.store_sub(94, 1850, 1849);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, (p.p263 * 0.1), 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[2009] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2009, if s.b[2009] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (ta3,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta3);
        let (ta4,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta4);
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2010] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2010, if s.b[2010] { 1.0 } else { 0.0 });s.b[2011] = (2.0 == 1.0);s.store_scalar(2011, if s.b[2011] { 1.0 } else { 0.0 });
        let (ta5,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && s.b[2011]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta5);s.b[2012] = (2.0 == 2.0);s.store_scalar(2012, if s.b[2012] { 1.0 } else { 0.0 });
        let (ta6,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && s.b[2012]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta6);s.b[2013] = (2.0 == 4.0);s.store_scalar(2013, if s.b[2013] { 1.0 } else { 0.0 });
        let (ta7,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && (!s.b[2012])) && s.b[2013]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta7);s.b[2014] = (2.0 == 8.0);s.store_scalar(2014, if s.b[2014] { 1.0 } else { 0.0 });
        let (ta8,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && (!s.b[2012])) && (!s.b[2013])) && s.b[2014]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta8);
        let (ta9,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta9);let mut tad: usize = 0;
        while {
            let tac: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tac != 0.0
        } {
            tad += 1;assert!(tad <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {s.store_sqrt(726, 726);}
            let (tab,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {
        let taa: f64 = (s.v[719] + 1.0);
        (taa,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tab);
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && (!s.b[2010])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2009])) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2009])) {s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_add(109, 87, 110);}
        s.b[2015] = (((s.v[109] - s.v[1847]) < s.v[1907]) && (s.v[1907] >= 0.0));s.store_scalar(2015, if s.b[2015] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {s.store_add_scaled_inputs3_indices(781, 1907, 1.0, 109, -1.0, 1847, 1.0);s.store_square(722, 781);s.store_square(723, 1907);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tae,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tae);
        let (taf,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, taf);
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2016] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2016, if s.b[2016] { 1.0 } else { 0.0 });s.b[2017] = (4.0 == 1.0);s.store_scalar(2017, if s.b[2017] { 1.0 } else { 0.0 });
        let (tb0,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && s.b[2017]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb0);s.b[2018] = (4.0 == 2.0);s.store_scalar(2018, if s.b[2018] { 1.0 } else { 0.0 });
        let (tb1,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && s.b[2018]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb1);s.b[2019] = (4.0 == 4.0);s.store_scalar(2019, if s.b[2019] { 1.0 } else { 0.0 });
        let (tb2,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && (!s.b[2018])) && s.b[2019]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb2);s.b[2020] = (4.0 == 8.0);s.store_scalar(2020, if s.b[2020] { 1.0 } else { 0.0 });
        let (tb3,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && (!s.b[2018])) && (!s.b[2019])) && s.b[2020]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb3);
        let (tb4,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb4);let mut tb8: usize = 0;
        while {
            let tb7: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb7 != 0.0
        } {
            tb8 += 1;assert!(tb8 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {s.store_sqrt(726, 726);}
            let (tb6,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {
        let tb5: f64 = (s.v[719] + 1.0);
        (tb5,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb6);
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && (!s.b[2016])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1907, 726);s.store_div_scaled_product3_indices(334, 1907, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1907, 780);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2015])) {s.store_sub(336, 109, 1847);s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1887, 209, -1.0, 338);}
        s.b[2026] = (s.v[1832] > s.v[965]);s.store_scalar(2026, if s.b[2026] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2026]) {s.copy_ad(981, 1830);}
        s.b[2027] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));s.store_scalar(2027, if s.b[2027] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {s.store_offset(781, 87, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb9,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb9);
        let (tba,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tba);
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (tbb,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tbb);let mut tbf: usize = 0;
        while {
            let tbe: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && (s.v[719] < s.v[1910])) { 1.0 } else { 0.0 };
            tbe != 0.0
        } {
            tbf += 1;assert!(tbf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (tbd,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        let tbc: f64 = (s.v[719] + 1.0);
        (tbc,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tbd);
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2028] = ((((s.v[1910] == 1.0) || (s.v[1910] == 2.0)) || (s.v[1910] == 4.0)) || (s.v[1910] == 8.0));s.store_scalar(2028, if s.b[2028] { 1.0 } else { 0.0 });s.b[2029] = (s.v[1910] == 1.0);s.store_scalar(2029, if s.b[2029] { 1.0 } else { 0.0 });
        let (tc0,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && s.b[2029]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc0);s.b[2030] = (s.v[1910] == 2.0);s.store_scalar(2030, if s.b[2030] { 1.0 } else { 0.0 });
        let (tc1,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && s.b[2030]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc1);s.b[2031] = (s.v[1910] == 4.0);s.store_scalar(2031, if s.b[2031] { 1.0 } else { 0.0 });
        let (tc2,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2031]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc2);s.b[2032] = (s.v[1910] == 8.0);s.store_scalar(2032, if s.b[2032] { 1.0 } else { 0.0 });
        let (tc3,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && (!s.b[2031])) && s.b[2032]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc3);
        let (tc4,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc4);let mut tc8: usize = 0;
        while {
            let tc7: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc7 != 0.0
        } {
            tc8 += 1;assert!(tc8 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {s.store_sqrt(726, 726);}
            let (tc6,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {
        let tc5: f64 = (s.v[719] + 1.0);
        (tc5,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc6);
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && (!s.b[2028])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1910), 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_offset(983, 780, (-0.1));}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2027])) {s.copy_ad(983, 87);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {s.store_add_scaled_inputs3_offset_indices(1912, 791, 1.0, 85, (-1.0), 1908, 1.0, (-(s.v[462] - p.p392)));s.store_sub(1911, 791, 1912);}
        s.b[2033] = ((s.v[1911] > (-s.v[1909])) && (s.v[1909] >= 0.0));s.store_scalar(2033, if s.b[2033] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {s.store_add(781, 1911, 1909);s.store_square(722, 781);s.store_square(723, 1909);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tc9,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc9);
        let (tca,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tca);
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (tcb,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tcb);let mut tcf: usize = 0;
        while {
            let tce: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && (s.v[719] < s.v[1910])) { 1.0 } else { 0.0 };
            tce != 0.0
        } {
            tcf += 1;assert!(tcf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (tcd,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        let tcc: f64 = (s.v[719] + 1.0);
        (tcc,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tcd);
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2034] = ((((s.v[1910] == 1.0) || (s.v[1910] == 2.0)) || (s.v[1910] == 4.0)) || (s.v[1910] == 8.0));s.store_scalar(2034, if s.b[2034] { 1.0 } else { 0.0 });s.b[2035] = (s.v[1910] == 1.0);s.store_scalar(2035, if s.b[2035] { 1.0 } else { 0.0 });
        let (td0,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && s.b[2035]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td0);s.b[2036] = (s.v[1910] == 2.0);s.store_scalar(2036, if s.b[2036] { 1.0 } else { 0.0 });
        let (td1,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && s.b[2036]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td1);s.b[2037] = (s.v[1910] == 4.0);s.store_scalar(2037, if s.b[2037] { 1.0 } else { 0.0 });
        let (td2,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && (!s.b[2036])) && s.b[2037]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td2);s.b[2038] = (s.v[1910] == 8.0);s.store_scalar(2038, if s.b[2038] { 1.0 } else { 0.0 });
        let (td3,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && (!s.b[2036])) && (!s.b[2037])) && s.b[2038]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td3);
        let (td4,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td4);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut td8: usize = 0;
        while {
            let td7: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            td7 != 0.0
        } {
            td8 += 1;assert!(td8 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {s.store_sqrt(726, 726);}
            let (td6,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {
        let td5: f64 = (s.v[719] + 1.0);
        (td5,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, td6);
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && (!s.b[2034])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1910), 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1909, 726);s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(1911, 1909, -1.0, 780, 1.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2033])) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2033])) {s.store_scalar(334, 1.0);}
        let (td9,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, td9);
        let (tda,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, tda);let mut te1: usize = 0;
        while {
            let te0: f64 = if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            te0 != 0.0
        } {
            te1 += 1;assert!(te1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2039] = (s.v[983] >= 0.0);s.store_scalar(2039, if s.b[2039] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2039]) {s.store_mul_scaled_sqrt_ad_rhs(2024, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2025, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2024, 1.0);}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2039])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1883)));s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1883);s.store_mul_sqrt_mixed_ia(2024, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2024, 1.0);s.store_mul_add_mixed_iaa(2025, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            let (tdc,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] != 0.0)) {
        let tdb: f64 = (150.0 + 1.0);
        (tdb,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, tdc);
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(1866, 2024, 1.0, 185, 1911, 983, 1.0);s.store_sub(1867, 2025, 185);s.store_div_scaled_inputs_indices(1878, 1866, -1.0, 1867, 1.0);}
            s.b[2040] = (((s.v[1878]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2040, if s.b[2040] { 1.0 } else { 0.0 });
            let (tdd,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) && s.b[2040]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, tdd);s.b[2041] = (s.v[1878] > 0.1);s.store_scalar(2041, if s.b[2041] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) && (!s.b[2040])) && s.b[2041]) {s.store_scalar(1878, 0.1);}
            s.b[2042] = (s.v[1878] < (-0.1));s.store_scalar(2042, if s.b[2042] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) && (!s.b[2040])) && (!s.b[2041])) && s.b[2042]) {s.store_scalar(1878, (-0.1));}
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 1878);}
            let (tdf,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
        let tde: f64 = (s.v[97] + 1.0);
        (tde,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, tdf);
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2022, 1901, 1830, (0.5 * 9662367879.197212), 0.0, 1830);s.store_scaled_sqrt_mul_scaled_lhs(334, 154, 2.0, 2022, p.p394);s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2023, 335, 2022);s.store_mul(332, 2023, 983);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {s.store_exp_mul_scaled_lhs_indices(334, 2023, -1.0, 2022);}
        s.b[2044] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2044, if s.b[2044] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2044]) {s.store_mul_exp_lhs(335, 332, 334);s.store_sub(336, 335, 334);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2044])) {s.store_mul_scale_offset_indices(335, 334, 332, 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);}
        s.b[2045] = (((s.v[336]) as f64).abs() > 1e-8);s.store_scalar(2045, if s.b[2045] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2045]) {s.store_div_ln_offset_lhs(2021, 336, 1.0, 2023);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2045])) {s.store_div(2021, 336, 2023);}
        s.b[2046] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2021])) / s.v[1901]) <= 0.0);s.store_scalar(2046, if s.b[2046] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2046]) {s.store_scalar(981, 0.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2046])) {s.store_sqrt_ad(981, A::div_scaled_inputs2(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2021), (-(2.0 * 1.034943e-10)), s.ad_value(1901), 1.0));}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
            if (s.v[981] > s.v[1830]) {
                s.copy_ad(981, 1830);
            } else {
            }
        }
        s.b[2047] = (s.v[981] < s.v[1830]);s.store_scalar(2047, if s.b[2047] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2047]) {s.store_sub(990, 1830, 981);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2047])) {s.store_scalar(990, 0.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_neg_add(1890, 1885, 1886);}
        s.b[2048] = (s.v[94] < 0.0);s.store_scalar(2048, if s.b[2048] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2048]) {s.store_scalar(94, 0.0);s.copy_ad(1850, 1849);s.store_scalar(248, 0.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2048])) {s.store_mul3_affine_lhs(248, 154, 1890, 1.0 / (2.0), 0.0, 94);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2048])) {
            if (s.v[248] < 0.0) {
                s.store_scalar(248, 0.0);
            } else {
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_neg(238, 1887);s.copy_ad(170, 162);s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));s.store_offset_mul(338, 980, 334, 1.0);s.store_mul(339, 336, 238);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(342, 339, 251);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 238, 343);s.store_scalar(336, s.v[474]);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_inputs(s.ad_value(336), 1.0, s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2049] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2049, if s.b[2049] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2049]) {s.store_scalar(337, 1.0);}
        s.b[2050] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2050, if s.b[2050] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2049])) && s.b[2050]) {s.copy_ad(337, 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2049])) && (!s.b[2050])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[2051] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2051, if s.b[2051] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2051]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2052] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2052, if s.b[2052] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2051])) && s.b[2052]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2051])) && (!s.b[2052])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2051])) && (!s.b[2052])) {s.store_mul(339, 338, 340);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(253, 254, 339);s.copy_ad(984, 253);s.copy_ad(1882, 255);s.copy_ad(989, 349);}
        s.b[2053] = (s.v[349] > 1e-6);s.store_scalar(2053, if s.b[2053] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {s.store_scaled_add(344, 1883, 155, p.p396);s.store_offset_mul_ad(338, s.ad_value(1903), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1903, 1.0);}
        s.b[2054] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2054, if s.b[2054] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (te2,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te2);
        let (te3,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te3);
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2055] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2055, if s.b[2055] { 1.0 } else { 0.0 });s.b[2056] = (2.0 == 1.0);s.store_scalar(2056, if s.b[2056] { 1.0 } else { 0.0 });
        let (te4,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && s.b[2056]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te4);s.b[2057] = (2.0 == 2.0);s.store_scalar(2057, if s.b[2057] { 1.0 } else { 0.0 });
        let (te5,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && s.b[2057]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te5);s.b[2058] = (2.0 == 4.0);s.store_scalar(2058, if s.b[2058] { 1.0 } else { 0.0 });
        let (te6,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && (!s.b[2057])) && s.b[2058]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te6);s.b[2059] = (2.0 == 8.0);s.store_scalar(2059, if s.b[2059] { 1.0 } else { 0.0 });
        let (te7,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && (!s.b[2057])) && (!s.b[2058])) && s.b[2059]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te7);
        let (te8,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te8);let mut tec: usize = 0;
        while {
            let teb: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            teb != 0.0
        } {
            tec += 1;assert!(tec <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {s.store_sqrt(726, 726);}
            let (tea,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {
        let te9: f64 = (s.v[719] + 1.0);
        (te9,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tea);
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && (!s.b[2055])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2054])) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2054])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1902, 1.0, 337);}
        s.b[2060] = ((s.v[344] < (s.v[972] + s.v[1906])) && (s.v[1906] >= 0.0));s.store_scalar(2060, if s.b[2060] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1906, 1.0, 344, -1.0);s.store_square(722, 781);}
    }
}
