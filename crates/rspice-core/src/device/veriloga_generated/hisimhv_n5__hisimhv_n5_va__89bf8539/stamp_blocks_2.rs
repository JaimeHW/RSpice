#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_scalar(1912, 3.0);
            s.store_scalar(1851, 0.0);
            s.store_scalar(1852, 0.0);
            s.store_scalar(1860, 0.0);
            s.store_scalar(1861, 0.0);
            s.store_scalar(1893, 0.0);
            s.store_scalar(1894, 0.0);
            s.store_scalar(1864, 0.0);
            s.store_scalar(1866, 0.0);
            s.store_scalar(1865, 0.0);
            s.store_scalar(1867, 0.0);
            s.store_scalar(1837, 0.0);
            s.store_scalar(1832, 0.0);
            s.copy_ad(1885, 1433);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));
            s.store_div_scaled_product_add_scaled_denominator_indices(962, 1906, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);
            s.store_sub(335, 1853, 1436);
        }

        s.b[1915] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
            s.store_sub_from_scalar(781, 0.1, 335);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign37350_e42950,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign37350_e42950;

        let (assign37360_e42961,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37360_e42961;

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1916] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        s.b[1917] = (4.0 == 1.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        let (assign37510_e43144,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && s.b[1917]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37510_e43144;

        s.b[1918] = (4.0 == 2.0);
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        let (assign37530_e43165,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && s.b[1918]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37530_e43165;

        s.b[1919] = (4.0 == 4.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        let (assign37550_e43189,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && (!s.b[1918])) && s.b[1919]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37550_e43189;

        s.b[1920] = (4.0 == 8.0);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        let (assign37570_e43216,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (!s.b[1917])) && (!s.b[1918])) && (!s.b[1919])) && s.b[1920]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37570_e43216;

        let (assign37580_e43229,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign37580_e43229;

        let mut assign37590_loop_guard: usize = 0;
        while {
            let assign37590_cond_e43243: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign37590_cond_e43243 != 0.0
        } {
            assign37590_loop_guard += 1;
            assert!(assign37590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) {
                s.store_sqrt(726, 726);
            }
            let (assign37590_body1_e43272,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && s.b[1916]) {
        let assign37590_body1_e43270: f64 = (s.v[719] + 1.0);
        (assign37590_body1_e43270,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign37590_body1_e43272;
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) && (!s.b[1916])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1915]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1915])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(1836, 962, 336);
            s.store_sqrt(1834, 1836);
        }

        s.b[1921] = (p.p345 != 0.0);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1921]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_scalar(1849, 0.0);
        }

        s.b[1922] = (s.v[1834] > s.v[965]);
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1922]) {
            s.copy_ad(1833, 965);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1922])) {
            s.copy_ad(1833, 1834);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(1858, 781, (-0.5), 782, (-0.5), 0.3);
            s.store_add_scaled_inputs3_offset_indices(781, 1858, 1.0, 1885, -1.0, 1853, 1.0, (-0.01));
            s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1858, 1885, 1.0, 1853, (-1.0), 781, 0.5, 782, 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(1886, 1853, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);
            s.store_offset_sub(1832, 965, 1833, 1e-15);
        }

        let (assign38030_e43901,) = {
    if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign38030_e43901;

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_scalar(1848, 0.2);
            s.copy_ad(1851, 1858);
            s.copy_ad(1854, 1849);
            s.copy_ad(1856, 1886);
        }

        let (assign38080_e43946,) = {
    if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign38080_e43946;

    }

    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign38090_loop_guard: usize = 0;
        while {
            let assign38090_cond_e43956: f64 = if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign38090_cond_e43956 != 0.0
        } {
            assign38090_loop_guard += 1;
            assert!(assign38090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul_sub_ad_rhs(1856, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1854), 1.0), s.ad_value(1853));
                s.store_mul(1840, 1899, 1900);
                s.store_sub(335, 1854, 1856);
            }
            s.b[1923] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38090_body9_e44078,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body9_e44078;
            let (assign38090_body10_e44089,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body10_e44089;
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1924] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };
            s.b[1925] = (2.0 == 1.0);
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            let (assign38090_body21_e44220,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && s.b[1925]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body21_e44220;
            s.b[1926] = (2.0 == 2.0);
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            let (assign38090_body23_e44241,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && s.b[1926]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body23_e44241;
            s.b[1927] = (2.0 == 4.0);
            s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };
            let (assign38090_body25_e44265,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && s.b[1927]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body25_e44265;
            s.b[1928] = (2.0 == 8.0);
            s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };
            let (assign38090_body27_e44292,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && (!s.b[1927])) && s.b[1928]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body27_e44292;
            let (assign38090_body28_e44305,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body28_e44305;
            let mut assign38090_body29_loop_guard: usize = 0;
            while {
                let assign38090_body29_cond_e44319: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body29_cond_e44319 != 0.0
            } {
                assign38090_body29_loop_guard += 1;
                assert!(assign38090_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38090_body29_body1_e44348,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {
        let assign38090_body29_body1_e44346: f64 = (s.v[719] + 1.0);
        (assign38090_body29_body1_e44346,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38090_body29_body1_e44348;
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && (!s.b[1924])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1923])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_sqrt_mul(1828, 1906, 336);
            }
            s.b[1929] = ((s.v[1828] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_offset_sub(781, 1828, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38090_body45_e44563,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body45_e44563;
            let (assign38090_body46_e44574,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body46_e44574;
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1930] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            s.b[1931] = (2.0 == 1.0);
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            let (assign38090_body57_e44705,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && s.b[1931]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body57_e44705;
            s.b[1932] = (2.0 == 2.0);
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            let (assign38090_body59_e44726,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && s.b[1932]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body59_e44726;
            s.b[1933] = (2.0 == 4.0);
            s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };
            let (assign38090_body61_e44750,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && s.b[1933]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body61_e44750;
            s.b[1934] = (2.0 == 8.0);
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            let (assign38090_body63_e44777,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && (!s.b[1933])) && s.b[1934]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body63_e44777;
            let (assign38090_body64_e44790,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body64_e44790;
            let mut assign38090_body65_loop_guard: usize = 0;
            while {
                let assign38090_body65_cond_e44804: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body65_cond_e44804 != 0.0
            } {
                assign38090_body65_loop_guard += 1;
                assert!(assign38090_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38090_body65_body1_e44833,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {
        let assign38090_body65_body1_e44831: f64 = (s.v[719] + 1.0);
        (assign38090_body65_body1_e44831,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38090_body65_body1_e44833;
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && (!s.b[1930])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1828, 965, (-1e-8), 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul(1864, 1828, 1903);
                s.store_mul_ad_product_lhs(1842, A::div_from_scalar(1.034943e-10, s.ad_value(1828)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1844, A::div_from_scalar((-1.034943e-10), s.ad_value(1828)), s.ad_value(334), 337);
            }
            s.b[1935] = (p.p49 == 0.0);
            s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1935]) {
                s.store_add_ad_lhs(1837, A::div_scaled_inputs_product(s.ad_value(1901), 1.0, s.ad_value(1836), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0), s.ad_value(1906), 1.0), 1851);
                s.store_scalar(1838, 1.0);
                s.store_scalar(1839, 0.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1935])) {
                s.store_add_ad_rhs(1837, 1851, A::div_scaled_add_product(s.ad_value(1901), 1.0, s.ad_value(1828), A::sub_scaled_inputs(s.ad_value(1828), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1906), 1.0));
                s.store_scalar(1838, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1828)), s.ad_value(334), (-1.0)), 1.0, 1840);
            }
            s.b[1936] = ((s.v[1837] > (s.v[1849] - s.v[1848])) && (s.v[1848] >= 0.0));
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_add_scaled_inputs3_indices(781, 1837, 1.0, 1849, (-1.0), 1848, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1848);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38090_body90_e45184,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body90_e45184;
            let (assign38090_body91_e45195,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body91_e45195;
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1937] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            s.b[1938] = (4.0 == 1.0);
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            let (assign38090_body106_e45378,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && s.b[1938]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body106_e45378;
            s.b[1939] = (4.0 == 2.0);
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            let (assign38090_body108_e45399,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && s.b[1939]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body108_e45399;
            s.b[1940] = (4.0 == 4.0);
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            let (assign38090_body110_e45423,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && s.b[1940]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body110_e45423;
            s.b[1941] = (4.0 == 8.0);
            s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };
            let (assign38090_body112_e45450,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && (!s.b[1940])) && s.b[1941]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body112_e45450;
            let (assign38090_body113_e45463,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body113_e45463;
            let mut assign38090_body114_loop_guard: usize = 0;
            while {
                let assign38090_body114_cond_e45477: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body114_cond_e45477 != 0.0
            } {
                assign38090_body114_loop_guard += 1;
                assert!(assign38090_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38090_body114_body1_e45506,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {
        let assign38090_body114_body1_e45504: f64 = (s.v[719] + 1.0);
        (assign38090_body114_body1_e45504,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38090_body114_body1_e45506;
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && (!s.b[1937])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1848, 726);
                s.store_div_scaled_product3_indices(334, 1848, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1837, 1849, 1.0, 1848, (-1.0), 780, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul(1838, 1838, 334);
                s.store_mul(1839, 1839, 334);
                s.store_add_scaled_inputs3_indices(335, 1856, 1.0, 1885, (-1.0), 1853, 1.0);
            }
            s.b[1942] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38090_body132_e45744,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body132_e45744;
            let (assign38090_body133_e45755,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body133_e45755;
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1943] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            s.b[1944] = (2.0 == 1.0);
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            let (assign38090_body144_e45886,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && s.b[1944]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body144_e45886;
            s.b[1945] = (2.0 == 2.0);
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            let (assign38090_body146_e45907,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && s.b[1945]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body146_e45907;
            s.b[1946] = (2.0 == 4.0);
            s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };
            let (assign38090_body148_e45931,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && s.b[1946]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body148_e45931;
            s.b[1947] = (2.0 == 8.0);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            let (assign38090_body150_e45958,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && (!s.b[1946])) && s.b[1947]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38090_body150_e45958;
            let (assign38090_body151_e45971,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38090_body151_e45971;
            let mut assign38090_body152_loop_guard: usize = 0;
            while {
                let assign38090_body152_cond_e45985: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body152_cond_e45985 != 0.0
            } {
                assign38090_body152_loop_guard += 1;
                assert!(assign38090_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38090_body152_body1_e46014,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {
        let assign38090_body152_body1_e46012: f64 = (s.v[719] + 1.0);
        (assign38090_body152_body1_e46012,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38090_body152_body1_e46014;
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && (!s.b[1943])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1942])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_sqrt_mul(1830, 1907, 336);
                s.store_mul_neg_lhs(1865, 1830, 1902);
                s.store_mul_div_from_scalar_lhs(1846, (-1.034943e-10), 1830, 337);
                s.store_mul_sub_rhs(335, 154, 1851, 1854);
                s.store_exp(336, 335);
            }
            s.b[1948] = (s.v[1851] >= s.v[1854]);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1948]) {
                s.store_mul_scaled_sqrt_ad_rhs(1860, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1895, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1860), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1897, 1895);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1948])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1851), s.ad_value(1885)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1854), s.ad_value(1885)));
                s.store_mul_sqrt_ad_rhs(1860, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1860, 1.0);
                s.store_mul_add_ad_rhs(1895, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1897, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            let (assign38090_body176_e46398,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] != 0.0)) {
        let assign38090_body176_e46396: f64 = (150.0 + 1.0);
        (assign38090_body176_e46396,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign38090_body176_e46398;
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1868, A::add_scaled_product(s.ad_value(1860), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1851)), 1.0), 1.0, 1864, 1.0, 1865, 1.0);
                s.store_sub(1869, 1895, 185);
                s.store_add_ad_rhs(1870, 1897, A::add_scaled_value_products(s.ad_value(1842), 1.0, s.ad_value(1844), s.ad_value(1840), 1.0, s.ad_value(1846), s.ad_value(1840), 1.0));
                s.store_sub(1871, 1854, 1837);
                s.store_neg(1872, 1838);
                s.store_sub_from_scalar(1873, 1.0, 1839);
                s.store_add_scaled_products_indices(1874, 1869, 1873, 1.0, 1870, 1872, (-1.0));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                if (s.v[1874] > 0.0) {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, (-1e-25));
                }
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1876, 1873);
                s.store_neg(1877, 1870);
                s.store_neg(1878, 1872);
                s.copy_ad(1879, 1869);
                s.store_mul_add_scaled_products_indices_rhs(1880, 1875, 1876, 1868, -1.0, 1877, 1871, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1881, 1875, 1878, 1868, -1.0, 1879, 1871, -1.0);
                s.store_abs(335, 1880);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                    s.store_abs(335, 1881);
                } else {
                }
            }
            s.b[1949] = (s.v[335] > 0.1);
            s.v[1949] = if s.b[1949] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1949]) {
                s.store_mul_div_from_scalar_rhs(1880, 1880, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1881, 1881, 0.1, 335);
            }
            s.b[1950] = (s.v[335] < 1e-12);
            s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };
            let (assign38090_body197_e46721,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1950]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign38090_body197_e46721;
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_add(1851, 1851, 1880);
                s.store_add(1854, 1854, 1881);
            }
            let (assign38090_body200_e46760,) = {
    if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
        let assign38090_body200_e46758: f64 = (s.v[97] + 1.0);
        (assign38090_body200_e46758,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign38090_body200_e46760;
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul_sub_rhs(335, 154, 1851, 1854);
            s.store_exp(336, 335);
        }

        s.b[1952] = (s.v[1851] >= s.v[1854]);
        s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1952]) {
            s.copy_ad(1890, 1860);
            s.store_scalar(1893, 0.0);
            s.store_scalar(1862, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) {
            s.store_scalar(1890, 0.0);
            s.store_mul_sqrt_ad_rhs(1893, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[1953] = (s.v[1834] > s.v[965]);
        s.v[1953] = if s.b[1953] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && s.b[1953]) {
            s.store_scalar(1862, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && (!s.b[1953])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1851), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1885)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1862, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[1954] = (((s.v[1851] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[1954] = if s.b[1954] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
            s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 1851, -1.0, 1849, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1909);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign38290_e47017,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38290_e47017;

        let (assign38300_e47028,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38300_e47028;

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1955] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        s.b[1956] = (4.0 == 1.0);
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        let (assign38450_e47211,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && s.b[1956]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38450_e47211;

        s.b[1957] = (4.0 == 2.0);
        s.v[1957] = if s.b[1957] { 1.0 } else { 0.0 };

        let (assign38470_e47232,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && s.b[1957]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38470_e47232;

        s.b[1958] = (4.0 == 4.0);
        s.v[1958] = if s.b[1958] { 1.0 } else { 0.0 };

        let (assign38490_e47256,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && s.b[1958]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38490_e47256;

        s.b[1959] = (4.0 == 8.0);
        s.v[1959] = if s.b[1959] { 1.0 } else { 0.0 };

        let (assign38510_e47283,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && (!s.b[1958])) && s.b[1959]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38510_e47283;

        let (assign38520_e47296,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38520_e47296;

        let mut assign38530_loop_guard: usize = 0;
        while {
            let assign38530_cond_e47310: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38530_cond_e47310 != 0.0
        } {
            assign38530_loop_guard += 1;
            assert!(assign38530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {
                s.store_sqrt(726, 726);
            }
            let (assign38530_body1_e47339,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {
        let assign38530_body1_e47337: f64 = (s.v[719] + 1.0);
        (assign38530_body1_e47337,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38530_body1_e47339;
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && (!s.b[1955])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1909, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1954])) {
            s.store_sub(336, 1851, 1849);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1887, 209, -1.0, 338);
            s.copy_ad(349, 790);
        }

        s.b[1960] = (s.v[790] > 1e-6);
        s.v[1960] = if s.b[1960] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_scalar(344, 1e-25);
            s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1905, 1.0);
        }

        s.b[1961] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[1961] = if s.b[1961] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign38750_e47642,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38750_e47642;

        let (assign38760_e47655,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38760_e47655;

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1962] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        s.b[1963] = (2.0 == 1.0);
        s.v[1963] = if s.b[1963] { 1.0 } else { 0.0 };

        let (assign38870_e47804,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && s.b[1963]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38870_e47804;

        s.b[1964] = (2.0 == 2.0);
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        let (assign38890_e47827,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && s.b[1964]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38890_e47827;

        s.b[1965] = (2.0 == 4.0);
        s.v[1965] = if s.b[1965] { 1.0 } else { 0.0 };

        let (assign38910_e47853,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && s.b[1965]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38910_e47853;

        s.b[1966] = (2.0 == 8.0);
        s.v[1966] = if s.b[1966] { 1.0 } else { 0.0 };

        let (assign38930_e47882,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && (!s.b[1965])) && s.b[1966]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38930_e47882;

        let (assign38940_e47897,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38940_e47897;

        let mut assign38950_loop_guard: usize = 0;
        while {
            let assign38950_cond_e47913: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38950_cond_e47913 != 0.0
        } {
            assign38950_loop_guard += 1;
            assert!(assign38950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {
                s.store_sqrt(726, 726);
            }
            let (assign38950_body1_e47946,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {
        let assign38950_body1_e47944: f64 = (s.v[719] + 1.0);
        (assign38950_body1_e47944,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38950_body1_e47946;
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && (!s.b[1962])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1904), 1.0, s.ad_value(337)));
        }

        s.b[1967] = ((s.v[344] < 1.0) && (1.0 >= 0.0));
        s.v[1967] = if s.b[1967] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
            s.store_sub_from_scalar(781, 1.0, 344);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign39120_e48206,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign39120_e48206;

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
    ) {
        let (assign39130_e48219,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39130_e48219;

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1968] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1968] = if s.b[1968] { 1.0 } else { 0.0 };

        s.b[1969] = (2.0 == 1.0);
        s.v[1969] = if s.b[1969] { 1.0 } else { 0.0 };

        let (assign39240_e48368,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && s.b[1969]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39240_e48368;

        s.b[1970] = (2.0 == 2.0);
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        let (assign39260_e48391,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && s.b[1970]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39260_e48391;

        s.b[1971] = (2.0 == 4.0);
        s.v[1971] = if s.b[1971] { 1.0 } else { 0.0 };

        let (assign39280_e48417,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && s.b[1971]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39280_e48417;

        s.b[1972] = (2.0 == 8.0);
        s.v[1972] = if s.b[1972] { 1.0 } else { 0.0 };

        let (assign39300_e48446,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && (!s.b[1971])) && s.b[1972]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39300_e48446;

        let (assign39310_e48461,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign39310_e48461;

        let mut assign39320_loop_guard: usize = 0;
        while {
            let assign39320_cond_e48477: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign39320_cond_e48477 != 0.0
        } {
            assign39320_loop_guard += 1;
            assert!(assign39320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {
                s.store_sqrt(726, 726);
            }
            let (assign39320_body1_e48510,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {
        let assign39320_body1_e48508: f64 = (s.v[719] + 1.0);
        (assign39320_body1_e48508,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39320_body1_e48510;
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && (!s.b[1968])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(344, 1.0, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_div(335, 790, 344);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1960])) {
            s.copy_ad(348, 790);
        }

        s.b[1973] = (s.v[790] < 0.0);
        s.v[1973] = if s.b[1973] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1973]) {
            s.copy_ad(1852, 1851);
            s.copy_ad(1857, 1856);
            s.copy_ad(1855, 1854);
            s.copy_ad(1863, 1862);
            s.copy_ad(1891, 1890);
            s.copy_ad(1888, 1887);
            s.copy_ad(1866, 1864);
            s.copy_ad(1867, 1865);
        }

        let (assign39580_e48868,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
        (s.v[1834],)
    } else {
        (s.v[1835],)
    }
};
        s.v[1835] = assign39580_e48868;

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.copy_ad(1850, 790);
            s.store_add_scaled_inputs3_offset_indices(781, 1851, 1.0, 1850, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 1851, 1850, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1859, 1851, 1.0, 1850, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 1859, 1.0, 1885, -1.0, 1853, 1.0, (-0.01));
            s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1859, 1885, 1.0, 1853, (-1.0), 781, 0.5, 782, 0.5);
            s.store_mul(212, 209, 186);
            s.store_square(213, 212);
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1885))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_add_ad_rhs(92, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
        }

        let (assign39770_e49208,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign39770_e49208;

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.copy_ad(1852, 1859);
            s.copy_ad(1855, 1850);
        }

        let (assign39800_e49244,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign39800_e49244;

    }

    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign39810_loop_guard: usize = 0;
        while {
            let assign39810_cond_e49257: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign39810_cond_e49257 != 0.0
        } {
            assign39810_loop_guard += 1;
            assert!(assign39810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul_sub_ad_rhs(1857, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1855), 1.0), s.ad_value(1853));
                s.store_mul(1841, 1899, 1900);
                s.store_sub(335, 1855, 1857);
            }
            s.b[1974] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1974] = if s.b[1974] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39810_body9_e49406,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body9_e49406;
            let (assign39810_body10_e49420,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body10_e49420;
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1975] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };
            s.b[1976] = (2.0 == 1.0);
            s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };
            let (assign39810_body21_e49578,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && s.b[1976]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body21_e49578;
            s.b[1977] = (2.0 == 2.0);
            s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };
            let (assign39810_body23_e49602,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && s.b[1977]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body23_e49602;
            s.b[1978] = (2.0 == 4.0);
            s.v[1978] = if s.b[1978] { 1.0 } else { 0.0 };
            let (assign39810_body25_e49629,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && s.b[1978]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body25_e49629;
            s.b[1979] = (2.0 == 8.0);
            s.v[1979] = if s.b[1979] { 1.0 } else { 0.0 };
            let (assign39810_body27_e49659,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && (!s.b[1978])) && s.b[1979]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body27_e49659;
            let (assign39810_body28_e49675,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body28_e49675;
            let mut assign39810_body29_loop_guard: usize = 0;
            while {
                let assign39810_body29_cond_e49692: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body29_cond_e49692 != 0.0
            } {
                assign39810_body29_loop_guard += 1;
                assert!(assign39810_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39810_body29_body1_e49727,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {
        let assign39810_body29_body1_e49725: f64 = (s.v[719] + 1.0);
        (assign39810_body29_body1_e49725,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39810_body29_body1_e49727;
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && (!s.b[1975])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1974])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_sqrt_mul(1829, 1906, 336);
            }
            s.b[1980] = ((s.v[1829] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1980] = if s.b[1980] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
                s.store_offset_sub(781, 1829, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39810_body45_e49987,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body45_e49987;
            let (assign39810_body46_e50001,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body46_e50001;
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1981] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };
            s.b[1982] = (2.0 == 1.0);
            s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };
            let (assign39810_body57_e50159,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && s.b[1982]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body57_e50159;
            s.b[1983] = (2.0 == 2.0);
            s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };
            let (assign39810_body59_e50183,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && s.b[1983]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body59_e50183;
            s.b[1984] = (2.0 == 4.0);
            s.v[1984] = if s.b[1984] { 1.0 } else { 0.0 };
            let (assign39810_body61_e50210,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && s.b[1984]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body61_e50210;
            s.b[1985] = (2.0 == 8.0);
            s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };
            let (assign39810_body63_e50240,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && (!s.b[1984])) && s.b[1985]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body63_e50240;
            let (assign39810_body64_e50256,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body64_e50256;
            let mut assign39810_body65_loop_guard: usize = 0;
            while {
                let assign39810_body65_cond_e50273: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body65_cond_e50273 != 0.0
            } {
                assign39810_body65_loop_guard += 1;
                assert!(assign39810_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39810_body65_body1_e50308,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {
        let assign39810_body65_body1_e50306: f64 = (s.v[719] + 1.0);
        (assign39810_body65_body1_e50306,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39810_body65_body1_e50308;
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && (!s.b[1981])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1829, 965, (-1e-8), 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul(1866, 1829, 1903);
                s.store_mul_ad_product_lhs(1843, A::div_from_scalar(1.034943e-10, s.ad_value(1829)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1845, A::div_from_scalar((-1.034943e-10), s.ad_value(1829)), s.ad_value(334), 337);
            }
            s.b[1986] = (p.p49 == 0.0);
            s.v[1986] = if s.b[1986] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1986]) {
                s.store_add_ad_lhs(1837, A::div_scaled_inputs_product(s.ad_value(1901), 1.0, s.ad_value(1836), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0), s.ad_value(1906), 1.0), 1852);
                s.store_scalar(1838, 1.0);
                s.store_scalar(1839, 0.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1986])) {
                s.store_add_ad_rhs(1837, 1852, A::div_scaled_add_product(s.ad_value(1901), 1.0, s.ad_value(1829), A::sub_scaled_inputs(s.ad_value(1829), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1906), 1.0));
                s.store_scalar(1838, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1829)), s.ad_value(334), (-1.0)), 1.0, 1841);
            }
            s.b[1987] = ((s.v[1837] > (s.v[1850] - s.v[1848])) && (s.v[1848] >= 0.0));
            s.v[1987] = if s.b[1987] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
                s.store_add_scaled_inputs3_indices(781, 1837, 1.0, 1850, (-1.0), 1848, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1848);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39810_body90_e50728,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body90_e50728;
            let (assign39810_body91_e50742,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body91_e50742;
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1988] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };
            s.b[1989] = (4.0 == 1.0);
            s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };
            let (assign39810_body106_e50964,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && s.b[1989]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body106_e50964;
            s.b[1990] = (4.0 == 2.0);
            s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };
            let (assign39810_body108_e50988,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && s.b[1990]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body108_e50988;
            s.b[1991] = (4.0 == 4.0);
            s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };
            let (assign39810_body110_e51015,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && s.b[1991]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body110_e51015;
            s.b[1992] = (4.0 == 8.0);
            s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };
            let (assign39810_body112_e51045,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && (!s.b[1991])) && s.b[1992]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body112_e51045;
            let (assign39810_body113_e51061,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body113_e51061;
            let mut assign39810_body114_loop_guard: usize = 0;
            while {
                let assign39810_body114_cond_e51078: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body114_cond_e51078 != 0.0
            } {
                assign39810_body114_loop_guard += 1;
                assert!(assign39810_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39810_body114_body1_e51113,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {
        let assign39810_body114_body1_e51111: f64 = (s.v[719] + 1.0);
        (assign39810_body114_body1_e51111,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39810_body114_body1_e51113;
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && (!s.b[1988])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1848, 726);
                s.store_div_scaled_product3_indices(334, 1848, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1837, 1850, 1.0, 1848, (-1.0), 780, 1.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul(1838, 1838, 334);
                s.store_mul(1839, 1839, 334);
                s.store_add_scaled_inputs3_indices(335, 1857, 1.0, 1885, (-1.0), 1853, 1.0);
            }
            s.b[1993] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39810_body132_e51402,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body132_e51402;
            let (assign39810_body133_e51416,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body133_e51416;
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1994] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };
            s.b[1995] = (2.0 == 1.0);
            s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };
            let (assign39810_body144_e51574,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && s.b[1995]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body144_e51574;
            s.b[1996] = (2.0 == 2.0);
            s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };
            let (assign39810_body146_e51598,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && s.b[1996]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body146_e51598;
            s.b[1997] = (2.0 == 4.0);
            s.v[1997] = if s.b[1997] { 1.0 } else { 0.0 };
            let (assign39810_body148_e51625,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && s.b[1997]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body148_e51625;
            s.b[1998] = (2.0 == 8.0);
            s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };
            let (assign39810_body150_e51655,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && (!s.b[1997])) && s.b[1998]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39810_body150_e51655;
            let (assign39810_body151_e51671,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39810_body151_e51671;
            let mut assign39810_body152_loop_guard: usize = 0;
            while {
                let assign39810_body152_cond_e51688: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body152_cond_e51688 != 0.0
            } {
                assign39810_body152_loop_guard += 1;
                assert!(assign39810_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39810_body152_body1_e51723,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {
        let assign39810_body152_body1_e51721: f64 = (s.v[719] + 1.0);
        (assign39810_body152_body1_e51721,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39810_body152_body1_e51723;
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && (!s.b[1994])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1993])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_sqrt_mul(1831, 1907, 336);
                s.store_mul_neg_lhs(1867, 1831, 1902);
                s.store_mul_div_from_scalar_lhs(1847, (-1.034943e-10), 1831, 337);
                s.store_mul_sub_rhs(335, 154, 1852, 1855);
                s.store_exp(336, 335);
            }
            s.b[1999] = (s.v[1852] >= s.v[1855]);
            s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1999]) {
                s.store_mul_scaled_sqrt_ad_rhs(1861, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1896, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1861), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1898, 1896);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1999])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)));
                s.store_mul_sqrt_ad_rhs(1861, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1861, 1.0);
                s.store_mul_add_ad_rhs(1896, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1898, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            let (assign39810_body176_e52176,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] != 0.0)) {
        let assign39810_body176_e52174: f64 = (150.0 + 1.0);
        (assign39810_body176_e52174,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign39810_body176_e52176;
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1868, A::add_scaled_product(s.ad_value(1861), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1852)), 1.0), 1.0, 1866, 1.0, 1867, 1.0);
                s.store_sub(1869, 1896, 185);
                s.store_add_ad_rhs(1870, 1898, A::add_scaled_value_products(s.ad_value(1843), 1.0, s.ad_value(1845), s.ad_value(1841), 1.0, s.ad_value(1847), s.ad_value(1841), 1.0));
                s.store_sub(1871, 1855, 1837);
                s.store_neg(1872, 1838);
                s.store_sub_from_scalar(1873, 1.0, 1839);
                s.store_add_scaled_products_indices(1874, 1869, 1873, 1.0, 1870, 1872, (-1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[1874] > 0.0) {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, (-1e-25));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.copy_ad(1876, 1873);
                s.store_neg(1877, 1870);
                s.store_neg(1878, 1872);
                s.copy_ad(1879, 1869);
                s.store_mul_add_scaled_products_indices_rhs(1880, 1875, 1876, 1868, -1.0, 1877, 1871, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1881, 1875, 1878, 1868, -1.0, 1879, 1871, -1.0);
                s.store_abs(335, 1880);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                    s.store_abs(335, 1881);
                } else {
                }
            }
            s.b[2000] = (s.v[335] > 0.1);
            s.v[2000] = if s.b[2000] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2000]) {
                s.store_mul_div_from_scalar_rhs(1880, 1880, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1881, 1881, 0.1, 335);
            }
            s.b[2001] = (s.v[335] < 1e-12);
            s.v[2001] = if s.b[2001] { 1.0 } else { 0.0 };
            let (assign39810_body197_e52556,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2001]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign39810_body197_e52556;
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_add(1852, 1852, 1880);
                s.store_add(1855, 1855, 1881);
            }
            let (assign39810_body200_e52604,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
        let assign39810_body200_e52602: f64 = (s.v[97] + 1.0);
        (assign39810_body200_e52602,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign39810_body200_e52604;
        }

    }

    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_mul_sub_rhs(335, 154, 1852, 1855);
            s.store_exp(336, 335);
        }

        s.b[2003] = (s.v[1852] >= s.v[1855]);
        s.v[2003] = if s.b[2003] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2003]) {
            s.copy_ad(1891, 1861);
            s.store_scalar(1894, 0.0);
            s.store_scalar(1863, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) {
            s.store_scalar(1891, 0.0);
            s.store_mul_sqrt_ad_rhs(1894, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[2004] = (s.v[1835] > s.v[965]);
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && s.b[2004]) {
            s.store_scalar(1863, 0.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && (!s.b[2004])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1863, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[2005] = (((s.v[1852] - s.v[1850]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
            s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 1852, -1.0, 1850, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1909);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign40010_e52909,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40010_e52909;

        let (assign40020_e52923,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40020_e52923;

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2006] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        s.b[2007] = (4.0 == 1.0);
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        let (assign40170_e53145,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && s.b[2007]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40170_e53145;

        s.b[2008] = (4.0 == 2.0);
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        let (assign40190_e53169,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && s.b[2008]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40190_e53169;

        s.b[2009] = (4.0 == 4.0);
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        let (assign40210_e53196,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && s.b[2009]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40210_e53196;

        s.b[2010] = (4.0 == 8.0);
        s.v[2010] = if s.b[2010] { 1.0 } else { 0.0 };

        let (assign40230_e53226,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && (!s.b[2009])) && s.b[2010]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40230_e53226;

        let (assign40240_e53242,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40240_e53242;

        let mut assign40250_loop_guard: usize = 0;
        while {
            let assign40250_cond_e53259: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40250_cond_e53259 != 0.0
        } {
            assign40250_loop_guard += 1;
            assert!(assign40250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {
                s.store_sqrt(726, 726);
            }
            let (assign40250_body1_e53294,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {
        let assign40250_body1_e53292: f64 = (s.v[719] + 1.0);
        (assign40250_body1_e53292,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign40250_body1_e53294;
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && (!s.b[2006])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1909, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2005])) {
            s.store_sub(336, 1852, 1850);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1888, 209, -1.0, 338);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.copy_ad(87, 1851);
            s.copy_ad(91, 1852);
            s.store_sub(94, 1852, 1851);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs_mixed_ia(336, 783, (-2.0), A::square(s.ad_value(782)), 1.0);
        }

        s.b[2011] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2011] = if s.b[2011] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign40510_e53748,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40510_e53748;

        let (assign40520_e53759,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40520_e53759;

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2012] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2012] = if s.b[2012] { 1.0 } else { 0.0 };

        s.b[2013] = (2.0 == 1.0);
        s.v[2013] = if s.b[2013] { 1.0 } else { 0.0 };

        let (assign40630_e53890,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && s.b[2013]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40630_e53890;

        s.b[2014] = (2.0 == 2.0);
        s.v[2014] = if s.b[2014] { 1.0 } else { 0.0 };

        let (assign40650_e53911,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && s.b[2014]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40650_e53911;

        s.b[2015] = (2.0 == 4.0);
        s.v[2015] = if s.b[2015] { 1.0 } else { 0.0 };

        let (assign40670_e53935,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && s.b[2015]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40670_e53935;

        s.b[2016] = (2.0 == 8.0);
        s.v[2016] = if s.b[2016] { 1.0 } else { 0.0 };

        let (assign40690_e53962,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && (!s.b[2015])) && s.b[2016]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40690_e53962;

        let (assign40700_e53975,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40700_e53975;

        let mut assign40710_loop_guard: usize = 0;
        while {
            let assign40710_cond_e53989: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40710_cond_e53989 != 0.0
        } {
            assign40710_loop_guard += 1;
            assert!(assign40710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {
                s.store_sqrt(726, 726);
            }
            let (assign40710_body1_e54018,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {
        let assign40710_body1_e54016: f64 = (s.v[719] + 1.0);
        (assign40710_body1_e54016,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign40710_body1_e54018;
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && (!s.b[2012])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_add(109, 87, 110);
        }

        s.b[2017] = (((s.v[109] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[2017] = if s.b[2017] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 109, -1.0, 1849, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1909);
            s.store_scalar(724, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_scalar(725, 1.0);
        }

        let (assign40870_e54244,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40870_e54244;

        let (assign40880_e54255,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40880_e54255;

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2018] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2018] = if s.b[2018] { 1.0 } else { 0.0 };

        s.b[2019] = (4.0 == 1.0);
        s.v[2019] = if s.b[2019] { 1.0 } else { 0.0 };

        let (assign41030_e54438,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && s.b[2019]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41030_e54438;

        s.b[2020] = (4.0 == 2.0);
        s.v[2020] = if s.b[2020] { 1.0 } else { 0.0 };

        let (assign41050_e54459,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && s.b[2020]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41050_e54459;

        s.b[2021] = (4.0 == 4.0);
        s.v[2021] = if s.b[2021] { 1.0 } else { 0.0 };

        let (assign41070_e54483,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && s.b[2021]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41070_e54483;

        s.b[2022] = (4.0 == 8.0);
        s.v[2022] = if s.b[2022] { 1.0 } else { 0.0 };

        let (assign41090_e54510,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && (!s.b[2021])) && s.b[2022]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41090_e54510;

        let (assign41100_e54523,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41100_e54523;

        let mut assign41110_loop_guard: usize = 0;
        while {
            let assign41110_cond_e54537: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41110_cond_e54537 != 0.0
        } {
            assign41110_loop_guard += 1;
            assert!(assign41110_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {
                s.store_sqrt(726, 726);
            }
            let (assign41110_body1_e54566,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {
        let assign41110_body1_e54564: f64 = (s.v[719] + 1.0);
        (assign41110_body1_e54564,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41110_body1_e54566;
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && (!s.b[2018])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1909, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2017])) {
            s.store_sub(336, 109, 1849);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1889, 209, -1.0, 338);
        }

        s.b[2028] = (s.v[1834] > s.v[965]);
        s.v[2028] = if s.b[2028] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2028]) {
            s.copy_ad(981, 1832);
        }

        s.b[2029] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));
        s.v[2029] = if s.b[2029] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_offset(781, 87, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign41300_e54836,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41300_e54836;

        let (assign41310_e54850,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41310_e54850;

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign41340_e54892,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41340_e54892;

        let mut assign41350_loop_guard: usize = 0;
        while {
            let assign41350_cond_e54907: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            assign41350_cond_e54907 != 0.0
        } {
            assign41350_loop_guard += 1;
            assert!(assign41350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign41350_body2_e54955,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        let assign41350_body2_e54953: f64 = (s.v[719] + 1.0);
        (assign41350_body2_e54953,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41350_body2_e54955;
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2030] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        s.b[2031] = (s.v[1912] == 1.0);
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        let (assign41400_e55021,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && s.b[2031]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41400_e55021;

        s.b[2032] = (s.v[1912] == 2.0);
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        let (assign41420_e55045,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && s.b[2032]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41420_e55045;

        s.b[2033] = (s.v[1912] == 4.0);
        s.v[2033] = if s.b[2033] { 1.0 } else { 0.0 };

        let (assign41440_e55072,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && s.b[2033]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41440_e55072;

        s.b[2034] = (s.v[1912] == 8.0);
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

        let (assign41460_e55102,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && (!s.b[2033])) && s.b[2034]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41460_e55102;

        let (assign41470_e55118,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41470_e55118;

        let mut assign41480_loop_guard: usize = 0;
        while {
            let assign41480_cond_e55135: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41480_cond_e55135 != 0.0
        } {
            assign41480_loop_guard += 1;
            assert!(assign41480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
                s.store_sqrt(726, 726);
            }
            let (assign41480_body1_e55170,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
        let assign41480_body1_e55168: f64 = (s.v[719] + 1.0);
        (assign41480_body1_e55168,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41480_body1_e55170;
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (!s.b[2030])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_offset(983, 780, (-0.1));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2029])) {
            s.copy_ad(983, 87);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_add_scaled_inputs3_offset_indices(1914, 791, 1.0, 85, (-1.0), 1910, 1.0, (-(s.v[462] - p.p392)));
            s.store_sub(1913, 791, 1914);
        }

        s.b[2035] = ((s.v[1913] > (-s.v[1911])) && (s.v[1911] >= 0.0));
        s.v[2035] = if s.b[2035] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_add(781, 1913, 1911);
            s.store_square(722, 781);
            s.store_square(723, 1911);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign41650_e55449,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41650_e55449;

        let (assign41660_e55463,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41660_e55463;

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign41690_e55505,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41690_e55505;

        let mut assign41700_loop_guard: usize = 0;
        while {
            let assign41700_cond_e55520: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            assign41700_cond_e55520 != 0.0
        } {
            assign41700_loop_guard += 1;
            assert!(assign41700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign41700_body2_e55568,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        let assign41700_body2_e55566: f64 = (s.v[719] + 1.0);
        (assign41700_body2_e55566,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41700_body2_e55568;
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2036] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2037] = (s.v[1912] == 1.0);
        s.v[2037] = if s.b[2037] { 1.0 } else { 0.0 };

        let (assign41750_e55634,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && s.b[2037]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41750_e55634;

        s.b[2038] = (s.v[1912] == 2.0);
        s.v[2038] = if s.b[2038] { 1.0 } else { 0.0 };

        let (assign41770_e55658,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && s.b[2038]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41770_e55658;

        s.b[2039] = (s.v[1912] == 4.0);
        s.v[2039] = if s.b[2039] { 1.0 } else { 0.0 };

        let (assign41790_e55685,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && s.b[2039]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41790_e55685;

        s.b[2040] = (s.v[1912] == 8.0);
        s.v[2040] = if s.b[2040] { 1.0 } else { 0.0 };

        let (assign41810_e55715,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && (!s.b[2039])) && s.b[2040]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41810_e55715;

        let (assign41820_e55731,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41820_e55731;

        let mut assign41830_loop_guard: usize = 0;
        while {
            let assign41830_cond_e55748: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41830_cond_e55748 != 0.0
        } {
            assign41830_loop_guard += 1;
            assert!(assign41830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
                s.store_sqrt(726, 726);
            }
            let (assign41830_body1_e55783,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
        let assign41830_body1_e55781: f64 = (s.v[719] + 1.0);
        (assign41830_body1_e55781,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41830_body1_e55783;
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (!s.b[2036])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1911, 726);
            s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(1913, 1911, -1.0, 780, 1.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {
            s.store_scalar(334, 1.0);
        }

        let (assign41920_e55939,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign41920_e55939;

        let (assign41930_e55951,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign41930_e55951;

        let mut assign41940_loop_guard: usize = 0;
        while {
            let assign41940_cond_e55964: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign41940_cond_e55964 != 0.0
        } {
            assign41940_loop_guard += 1;
            assert!(assign41940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2041] = (s.v[983] >= 0.0);
            s.v[2041] = if s.b[2041] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2041]) {
                s.store_mul_scaled_sqrt_ad_rhs(2026, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2027, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2026), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2041])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1885)));
                s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1885);
                s.store_mul_sqrt_ad_rhs(2026, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2026, 1.0);
                s.store_mul_add_ad_rhs(2027, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            let (assign41940_body10_e56180,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] != 0.0)) {
        let assign41940_body10_e56178: f64 = (150.0 + 1.0);
        (assign41940_body10_e56178,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign41940_body10_e56180;
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(1868, 2026, 1.0, 185, A::sub(s.ad_value(1913), s.ad_value(983)), 1.0);
                s.store_sub(1869, 2027, 185);
                s.store_div_scaled_inputs_indices(1880, 1868, -1.0, 1869, 1.0);
            }
            s.b[2042] = (((s.v[1880]) as f64).abs() < (1e-10 * 100.0));
            s.v[2042] = if s.b[2042] { 1.0 } else { 0.0 };
            let (assign41940_body15_e56260,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && s.b[2042]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign41940_body15_e56260;
            s.b[2043] = (s.v[1880] > 0.1);
            s.v[2043] = if s.b[2043] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && s.b[2043]) {
                s.store_scalar(1880, 0.1);
            }
            s.b[2044] = (s.v[1880] < (-0.1));
            s.v[2044] = if s.b[2044] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && (!s.b[2043])) && s.b[2044]) {
                s.store_scalar(1880, (-0.1));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 1880);
            }
            let (assign41940_body21_e56342,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
        let assign41940_body21_e56340: f64 = (s.v[97] + 1.0);
        (assign41940_body21_e56340,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign41940_body21_e56342;
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2024, 1903, 1832, (0.5 * 9662367879.197212), 0.0, 1832);
            s.store_scaled_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2024)), p.p394);
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(2025, 335, 2024);
            s.store_mul(332, 2025, 983);
            s.store_exp_mul_scaled_lhs_indices(334, 2025, -1.0, 2024);
        }

        s.b[2046] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2046] = if s.b[2046] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2046]) {
            s.store_mul_exp_lhs(335, 332, 334);
            s.store_sub(336, 335, 334);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2046])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2047] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2047] = if s.b[2047] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2047]) {
            s.store_div_ln_offset_lhs(2023, 336, 1.0, 2025);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2047])) {
            s.store_div(2023, 336, 2025);
        }

        s.b[2048] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2023])) / s.v[1903]) <= 0.0);
        s.v[2048] = if s.b[2048] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2048]) {
            s.store_scalar(981, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2048])) {
            s.store_sqrt_ad(981, A::div_scaled_inputs2(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2023), (-(2.0 * 1.034943e-10)), s.ad_value(1903), 1.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            if (s.v[981] > s.v[1832]) {
                s.copy_ad(981, 1832);
            } else {
            }
        }

        s.b[2049] = (s.v[981] < s.v[1832]);
        s.v[2049] = if s.b[2049] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2049]) {
            s.store_sub(990, 1832, 981);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2049])) {
            s.store_scalar(990, 0.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_neg_add(1892, 1887, 1888);
        }

        s.b[2050] = (s.v[94] < 0.0);
        s.v[2050] = if s.b[2050] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2050]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(1852, 1851);
            s.store_scalar(248, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {
            s.store_mul3_affine_lhs(248, 154, 1892, 1.0 / (2.0), 0.0, 94);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {
            if (s.v[248] < 0.0) {
                s.store_scalar(248, 0.0);
            } else {
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_neg(238, 1889);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 980, 334, 1.0);
            s.store_mul(339, 336, 238);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_ad(341, s.ad_value(251), A::offset(s.ad_value(624), (-1.0)));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_inputs(s.ad_value(336), 1.0, s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2051] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2051] = if s.b[2051] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2051]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2052] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2052] = if s.b[2052] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && s.b[2052]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && (!s.b[2052])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[2053] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2053] = if s.b[2053] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2053]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2054] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2054] = if s.b[2054] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && s.b[2054]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

    }

    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(1884, 255);
            s.copy_ad(989, 349);
        }

        s.b[2055] = (s.v[349] > 1e-6);
        s.v[2055] = if s.b[2055] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_scaled_add(344, 1885, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1905, 1.0);
        }

        s.b[2056] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2056] = if s.b[2056] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign42790_e57478,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign42790_e57478;

        let (assign42800_e57491,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42800_e57491;

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2057] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2057] = if s.b[2057] { 1.0 } else { 0.0 };

        s.b[2058] = (2.0 == 1.0);
        s.v[2058] = if s.b[2058] { 1.0 } else { 0.0 };

        let (assign42910_e57640,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && s.b[2058]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42910_e57640;

        s.b[2059] = (2.0 == 2.0);
        s.v[2059] = if s.b[2059] { 1.0 } else { 0.0 };

        let (assign42930_e57663,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && s.b[2059]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42930_e57663;

        s.b[2060] = (2.0 == 4.0);
        s.v[2060] = if s.b[2060] { 1.0 } else { 0.0 };

        let (assign42950_e57689,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && s.b[2060]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42950_e57689;

        s.b[2061] = (2.0 == 8.0);
        s.v[2061] = if s.b[2061] { 1.0 } else { 0.0 };

        let (assign42970_e57718,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && (!s.b[2060])) && s.b[2061]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42970_e57718;

        let (assign42980_e57733,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign42980_e57733;

        let mut assign42990_loop_guard: usize = 0;
        while {
            let assign42990_cond_e57749: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign42990_cond_e57749 != 0.0
        } {
            assign42990_loop_guard += 1;
            assert!(assign42990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
                s.store_sqrt(726, 726);
            }
            let (assign42990_body1_e57782,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
        let assign42990_body1_e57780: f64 = (s.v[719] + 1.0);
        (assign42990_body1_e57780,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign42990_body1_e57782;
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && (!s.b[2057])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1904), 1.0, s.ad_value(337)));
        }

        s.b[2062] = ((s.v[344] < (s.v[972] + s.v[1908])) && (s.v[1908] >= 0.0));
        s.v[2062] = if s.b[2062] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1908, 1.0, 344, -1.0);
            s.store_square(722, 781);
            s.store_square(723, 1908);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign43160_e58042,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43160_e58042;

        let (assign43170_e58055,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43170_e58055;

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2063] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2063] = if s.b[2063] { 1.0 } else { 0.0 };

        s.b[2064] = (2.0 == 1.0);
        s.v[2064] = if s.b[2064] { 1.0 } else { 0.0 };

        let (assign43280_e58204,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && s.b[2064]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43280_e58204;

        s.b[2065] = (2.0 == 2.0);
        s.v[2065] = if s.b[2065] { 1.0 } else { 0.0 };

        let (assign43300_e58227,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && s.b[2065]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43300_e58227;

        s.b[2066] = (2.0 == 4.0);
        s.v[2066] = if s.b[2066] { 1.0 } else { 0.0 };

        let (assign43320_e58253,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && s.b[2066]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43320_e58253;

        s.b[2067] = (2.0 == 8.0);
        s.v[2067] = if s.b[2067] { 1.0 } else { 0.0 };

        let (assign43340_e58282,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && (!s.b[2066])) && s.b[2067]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43340_e58282;

        let (assign43350_e58297,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43350_e58297;

        let mut assign43360_loop_guard: usize = 0;
        while {
            let assign43360_cond_e58313: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43360_cond_e58313 != 0.0
        } {
            assign43360_loop_guard += 1;
            assert!(assign43360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
                s.store_sqrt(726, 726);
            }
            let (assign43360_body1_e58346,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
        let assign43360_body1_e58344: f64 = (s.v[719] + 1.0);
        (assign43360_body1_e58344,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign43360_body1_e58346;
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && (!s.b[2063])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1908, 726);
            s.store_div_scaled_product3_indices(334, 1908, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(344, 972, 1.0, 1908, 1.0, 780, -1.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_div(335, 989, 344);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_mul(340, 338, 337);
            s.store_div(989, 989, 340);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sub(335, 791, 1885);
        }

        s.b[2068] = ((s.v[335] < 1.0) && (1.0 >= 0.0));
        s.v[2068] = if s.b[2068] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
            s.store_sub_from_scalar(781, 1.0, 335);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign43580_e58672,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43580_e58672;

    }

    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign43590_e58683,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43590_e58683;

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2069] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2069] = if s.b[2069] { 1.0 } else { 0.0 };

        s.b[2070] = (2.0 == 1.0);
        s.v[2070] = if s.b[2070] { 1.0 } else { 0.0 };

        let (assign43700_e58814,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && s.b[2070]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43700_e58814;

        s.b[2071] = (2.0 == 2.0);
        s.v[2071] = if s.b[2071] { 1.0 } else { 0.0 };

        let (assign43720_e58835,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && s.b[2071]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43720_e58835;

        s.b[2072] = (2.0 == 4.0);
        s.v[2072] = if s.b[2072] { 1.0 } else { 0.0 };

        let (assign43740_e58859,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && s.b[2072]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43740_e58859;

        s.b[2073] = (2.0 == 8.0);
        s.v[2073] = if s.b[2073] { 1.0 } else { 0.0 };

        let (assign43760_e58886,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && (!s.b[2072])) && s.b[2073]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43760_e58886;

        let (assign43770_e58899,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43770_e58899;

        let mut assign43780_loop_guard: usize = 0;
        while {
            let assign43780_cond_e58913: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43780_cond_e58913 != 0.0
        } {
            assign43780_loop_guard += 1;
            assert!(assign43780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
                s.store_sqrt(726, 726);
            }
            let (assign43780_body1_e58942,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
        let assign43780_body1_e58940: f64 = (s.v[719] + 1.0);
        (assign43780_body1_e58940,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign43780_body1_e58942;
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && (!s.b[2069])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(335, 1.0, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_div(251, 335, 965);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p353 - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(342, 339, 251);
            s.store_offset(336, 966, 1e-25);
            s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1883, 989, 170);
            s.store_powf(781, 989, 2.0);
            s.store_scalar(782, ((0.1) as f64).powf(2.0));
            s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div(335, 335, 170);
            s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_div(985, 254, 338);
            s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));
            s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1883);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add(135, 986, 987);
            s.copy_ad(790, 349);
        }

        s.b[2074] = (p.p283 != 0.0);
        s.v[2074] = if s.b[2074] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_mixed_ia(336, 783, (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1851), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2075] = (s.v[336] < 0.0);
        s.v[2075] = if s.b[2075] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) && s.b[2075]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1437, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 1851, 1.0, 340, 1.0, 1436, -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1437), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2074])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2076] = (p.p287 != 0.0);
        s.v[2076] = if s.b[2076] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2076]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1437);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2076])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2077] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2077] = if s.b[2077] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2077]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[2078] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2078] = if s.b[2078] { 1.0 } else { 0.0 };

        s.b[2079] = (p.p296 > 0.0);
        s.v[2079] = if s.b[2079] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2079])) {
            s.copy_ad(341, 647);
        }

        s.b[2080] = (s.v[793] >= 0.0);
        s.v[2080] = if s.b[2080] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2080]) {
            s.copy_ad(369, 793);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2080])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2081] = (s.v[369] < (20.0 * 1e-12));
        s.v[2081] = if s.b[2081] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2081]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2081])) {
            s.store_powf_offset_input(335, 369, 1e-12, p.p297);
        }

    }

    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) {
            s.store_powf_offset_input(343, 369, 1e-12, p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2078])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_add_scaled_inputs4_indices(131, 1862, (-0.5), 1863, (-0.5), 1865, (-0.5), 1867, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1890), 1.0, s.ad_value(1891), 1.0, s.ad_value(1893), 1.0, s.ad_value(1894), 1.0), s.ad_value(1864)), 1866, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1890, 1891, (-0.5));
            s.store_neg(238, 1890);
            s.copy_ad(255, 1884);
        }

        s.b[2082] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2082] = if s.b[2082] { 1.0 } else { 0.0 };

        let (assign44870_e60533,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2082]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign44870_e60533;

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(2089, 960);
            s.store_scale(2131, 964, 1.6021918e-19);
            s.store_scale(2112, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_div_from_scalar(2134, (2.0 * 1.034943e-10), 2131);
            s.store_div(2128, 964, 622);
            s.store_div_from_scalar_offset_input(2127, 1.0, 2128, 1.0);
            s.store_div_ad_rhs(2132, 2112, A::square(s.ad_value(185)));
            s.store_div_from_scalar(2133, 2.0, 2132);
            s.store_scalar(2141, 2.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p.p343 * p.p340)) }));
        }

        s.b[2170] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
            s.store_sub_from_scalar(781, (2.0 + 0.1), 508);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign45040_e60768,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45040_e60768;

        let (assign45050_e60781,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45050_e60781;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2171] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        s.b[2172] = (2.0 == 1.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        let (assign45160_e60930,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && s.b[2172]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45160_e60930;

        s.b[2173] = (2.0 == 2.0);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        let (assign45180_e60953,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && s.b[2173]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45180_e60953;

        s.b[2174] = (2.0 == 4.0);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        let (assign45200_e60979,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && s.b[2174]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45200_e60979;

        s.b[2175] = (2.0 == 8.0);
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        let (assign45220_e61008,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && (!s.b[2174])) && s.b[2175]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45220_e61008;

        let (assign45230_e61023,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45230_e61023;

        let mut assign45240_loop_guard: usize = 0;
        while {
            let assign45240_cond_e61039: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45240_cond_e61039 != 0.0
        } {
            assign45240_loop_guard += 1;
            assert!(assign45240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
                s.store_sqrt(726, 726);
            }
            let (assign45240_body1_e61072,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
        let assign45240_body1_e61070: f64 = (s.v[719] + 1.0);
        (assign45240_body1_e61070,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign45240_body1_e61072;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && (!s.b[2171])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(508, (2.0 + 0.1), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(2087, 0.0);
            s.store_scalar(2088, 0.0);
            s.store_scalar(2096, 0.0);
            s.store_scalar(2097, 0.0);
            s.store_scalar(2169, 0.0);
            s.store_scalar(2144, 0.0);
            s.copy_ad(2115, 1433);
            s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, (-1.0), (-s.v[160]));
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2094, 781, (-0.5), 782, (-0.5), 0.3);
            s.store_add_scaled_inputs3_offset_indices(781, 2094, 1.0, 2115, -1.0, 2089, 1.0, (-0.01));
            s.store_scaled_sub(782, 2115, 2089, (4.0 * 0.01));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2094, 2115, 1.0, 2089, (-1.0), 781, 0.5, 782, 0.5);
            s.copy_ad(2087, 2094);
            s.store_scalar(2085, 0.0);
            s.copy_ad(2090, 2085);
            s.store_mul_sub_rhs(2092, 2127, 1436, 2089);
            s.store_mul_neg_rhs(2148, 2127, 2089);
        }

        s.b[2176] = (((-s.v[2092]) < 0.001) && (0.001 >= 0.0));
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2092)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign45640_e61663,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45640_e61663;

        let (assign45650_e61676,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45650_e61676;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2177] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        s.b[2178] = (2.0 == 1.0);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        let (assign45760_e61825,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && s.b[2178]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45760_e61825;

        s.b[2179] = (2.0 == 2.0);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign45780_e61848,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && s.b[2179]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45780_e61848;

        s.b[2180] = (2.0 == 4.0);
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        let (assign45800_e61874,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && s.b[2180]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45800_e61874;

        s.b[2181] = (2.0 == 8.0);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        let (assign45820_e61903,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && (!s.b[2180])) && s.b[2181]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45820_e61903;

        let (assign45830_e61918,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45830_e61918;

        let mut assign45840_loop_guard: usize = 0;
        while {
            let assign45840_cond_e61934: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45840_cond_e61934 != 0.0
        } {
            assign45840_loop_guard += 1;
            assert!(assign45840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
                s.store_sqrt(726, 726);
            }
            let (assign45840_body1_e61967,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
        let assign45840_body1_e61965: f64 = (s.v[719] + 1.0);
        (assign45840_body1_e61965,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign45840_body1_e61967;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && (!s.b[2177])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2176])) {
            s.store_neg(335, 2092);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_mul(2083, 2134, 335);
        }

        s.b[2182] = (((-s.v[2148]) < 0.001) && (0.001 >= 0.0));
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2148)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign46000_e62215,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46000_e62215;

        let (assign46010_e62228,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46010_e62228;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2183] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        s.b[2184] = (2.0 == 1.0);
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        let (assign46120_e62377,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && s.b[2184]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46120_e62377;

        s.b[2185] = (2.0 == 2.0);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        let (assign46140_e62400,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && s.b[2185]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46140_e62400;

        s.b[2186] = (2.0 == 4.0);
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        let (assign46160_e62426,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && s.b[2186]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46160_e62426;

        s.b[2187] = (2.0 == 8.0);
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        let (assign46180_e62455,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && (!s.b[2186])) && s.b[2187]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46180_e62455;

        let (assign46190_e62470,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46190_e62470;

        let mut assign46200_loop_guard: usize = 0;
        while {
            let assign46200_cond_e62486: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46200_cond_e62486 != 0.0
        } {
            assign46200_loop_guard += 1;
            assert!(assign46200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
                s.store_sqrt(726, 726);
            }
            let (assign46200_body1_e62519,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
        let assign46200_body1_e62517: f64 = (s.v[719] + 1.0);
        (assign46200_body1_e62517,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign46200_body1_e62519;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && (!s.b[2183])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2182])) {
            s.store_neg(335, 2148);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_mul(2149, 2134, 335);
        }

        s.b[2188] = (p.p345 != 0.0);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(2129, 965);
            s.store_sub(2146, 965, 2083);
            s.store_sub(2147, 965, 2149);
        }

        s.b[2189] = ((s.v[2146] < (p.p344 + (p.p344 * 0.1))) && ((p.p344 * 0.1) >= 0.0));
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
            s.store_sub_from_scalar(781, (p.p344 + (p.p344 * 0.1)), 2146);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign46540_e63079,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46540_e63079;

        let (assign46550_e63092,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46550_e63092;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2190] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        s.b[2191] = (1.0 == 1.0);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        let (assign46640_e63211,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && s.b[2191]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46640_e63211;

        s.b[2192] = (1.0 == 2.0);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        let (assign46660_e63234,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && s.b[2192]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46660_e63234;

        s.b[2193] = (1.0 == 4.0);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        let (assign46680_e63260,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && s.b[2193]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46680_e63260;

        s.b[2194] = (1.0 == 8.0);
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        let (assign46700_e63289,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && (!s.b[2193])) && s.b[2194]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46700_e63289;

        let (assign46710_e63304,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46710_e63304;

    }

    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign46720_loop_guard: usize = 0;
        while {
            let assign46720_cond_e63320: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46720_cond_e63320 != 0.0
        } {
            assign46720_loop_guard += 1;
            assert!(assign46720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
                s.store_sqrt(726, 726);
            }
            let (assign46720_body1_e63353,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
        let assign46720_body1_e63351: f64 = (s.v[719] + 1.0);
        (assign46720_body1_e63351,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign46720_body1_e63353;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && (!s.b[2190])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2146, (p.p344 + (p.p344 * 0.1)), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2195] = ((s.v[2147] < (p.p344 * 0.1)) && ((p.p344 * 0.1) >= 0.0));
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
            s.store_sub_from_scalar(781, (p.p344 * 0.1), 2147);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign46870_e63600,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46870_e63600;

        let (assign46880_e63613,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46880_e63613;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2196] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        s.b[2197] = (1.0 == 1.0);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        let (assign46970_e63732,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && s.b[2197]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46970_e63732;

        s.b[2198] = (1.0 == 2.0);
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        let (assign46990_e63755,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && s.b[2198]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46990_e63755;

        s.b[2199] = (1.0 == 4.0);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        let (assign47010_e63781,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && s.b[2199]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47010_e63781;

        s.b[2200] = (1.0 == 8.0);
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        let (assign47030_e63810,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && (!s.b[2199])) && s.b[2200]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47030_e63810;

        let (assign47040_e63825,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47040_e63825;

        let mut assign47050_loop_guard: usize = 0;
        while {
            let assign47050_cond_e63841: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47050_cond_e63841 != 0.0
        } {
            assign47050_loop_guard += 1;
            assert!(assign47050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
                s.store_sqrt(726, 726);
            }
            let (assign47050_body1_e63874,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
        let assign47050_body1_e63872: f64 = (s.v[719] + 1.0);
        (assign47050_body1_e63872,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47050_body1_e63874;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && (!s.b[2196])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2147, (p.p344 * 0.1), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset_scaled_div(2150, 2146, 2147, (p.p394 - p.p395), p.p395);
        }

        let (assign47150_e64046,) = {
    if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign47150_e64046;

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul(2138, 2127, 2128);
        }

        let (assign47170_e64070,) = {
    if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign47170_e64070;

    }

    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign47180_loop_guard: usize = 0;
        while {
            let assign47180_cond_e64082: f64 = if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47180_cond_e64082 != 0.0
        } {
            assign47180_loop_guard += 1;
            assert!(assign47180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul_sub_ad_rhs(2092, 2127, A::add_scaled_product(s.ad_value(2115), 1.0, s.ad_value(2128), s.ad_value(2090), 1.0), s.ad_value(2089));
                s.store_sub(335, 2090, 2092);
            }
            s.b[2201] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign47180_body8_e64209,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47180_body8_e64209;
            let (assign47180_body9_e64222,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body9_e64222;
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2202] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };
            s.b[2203] = (2.0 == 1.0);
            s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };
            let (assign47180_body20_e64371,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && s.b[2203]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body20_e64371;
            s.b[2204] = (2.0 == 2.0);
            s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };
            let (assign47180_body22_e64394,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && s.b[2204]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body22_e64394;
            s.b[2205] = (2.0 == 4.0);
            s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };
            let (assign47180_body24_e64420,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && s.b[2205]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body24_e64420;
            s.b[2206] = (2.0 == 8.0);
            s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };
            let (assign47180_body26_e64449,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && (!s.b[2205])) && s.b[2206]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body26_e64449;
            let (assign47180_body27_e64464,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47180_body27_e64464;
            let mut assign47180_body28_loop_guard: usize = 0;
            while {
                let assign47180_body28_cond_e64480: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body28_cond_e64480 != 0.0
            } {
                assign47180_body28_loop_guard += 1;
                assert!(assign47180_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47180_body28_body1_e64513,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
        let assign47180_body28_body1_e64511: f64 = (s.v[719] + 1.0);
        (assign47180_body28_body1_e64511,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47180_body28_body1_e64513;
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && (!s.b[2202])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {
                s.store_scalar(336, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_sqrt_mul(2083, 2134, 335);
            }
            s.b[2207] = ((s.v[2083] > (s.v[2129] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
                s.store_offset_sub(781, 2083, 2129, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign47180_body44_e64758,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47180_body44_e64758;
            let (assign47180_body45_e64771,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body45_e64771;
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2208] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };
            s.b[2209] = (2.0 == 1.0);
            s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };
            let (assign47180_body56_e64920,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && s.b[2209]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body56_e64920;
            s.b[2210] = (2.0 == 2.0);
            s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };
            let (assign47180_body58_e64943,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && s.b[2210]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body58_e64943;
            s.b[2211] = (2.0 == 4.0);
            s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };
            let (assign47180_body60_e64969,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && s.b[2211]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body60_e64969;
            s.b[2212] = (2.0 == 8.0);
            s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };
            let (assign47180_body62_e64998,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && (!s.b[2211])) && s.b[2212]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body62_e64998;
            let (assign47180_body63_e65013,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47180_body63_e65013;
            let mut assign47180_body64_loop_guard: usize = 0;
            while {
                let assign47180_body64_cond_e65029: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body64_cond_e65029 != 0.0
            } {
                assign47180_body64_loop_guard += 1;
                assert!(assign47180_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47180_body64_body1_e65062,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
        let assign47180_body64_body1_e65060: f64 = (s.v[719] + 1.0);
        (assign47180_body64_body1_e65060,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47180_body64_body1_e65062;
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && (!s.b[2208])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_offset_lhs(2083, 2129, (-1e-12), 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul(337, 336, 337);
                s.store_add_div_rhs_mixed_ai(2135, 2087, A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2083), A::sub_scaled_inputs(s.ad_value(2083), 1.0, s.ad_value(2129), 2.0), 1.0), 2134);
                s.store_scalar(2136, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2083)), s.ad_value(337), (-1.0)), 1.0, 2138);
            }
            s.b[2213] = ((s.v[2135] > (s.v[2085] - p.p406)) && (p.p406 >= 0.0));
            s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
                s.store_offset_sub(781, 2135, 2085, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign47180_body83_e65363,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47180_body83_e65363;
            let (assign47180_body84_e65376,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body84_e65376;
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2214] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };
            s.b[2215] = (4.0 == 1.0);
            s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };
            let (assign47180_body99_e65585,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && s.b[2215]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body99_e65585;
            s.b[2216] = (4.0 == 2.0);
            s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };
            let (assign47180_body101_e65608,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && s.b[2216]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body101_e65608;
            s.b[2217] = (4.0 == 4.0);
            s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };
            let (assign47180_body103_e65634,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && s.b[2217]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body103_e65634;
            s.b[2218] = (4.0 == 8.0);
            s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };
            let (assign47180_body105_e65663,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && (!s.b[2217])) && s.b[2218]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47180_body105_e65663;
            let (assign47180_body106_e65678,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47180_body106_e65678;
            let mut assign47180_body107_loop_guard: usize = 0;
            while {
                let assign47180_body107_cond_e65694: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body107_cond_e65694 != 0.0
            } {
                assign47180_body107_loop_guard += 1;
                assert!(assign47180_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47180_body107_body1_e65727,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
        let assign47180_body107_body1_e65725: f64 = (s.v[719] + 1.0);
        (assign47180_body107_body1_e65725,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47180_body107_body1_e65727;
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && (!s.b[2214])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_offset_lhs(2135, 2085, (-p.p406), 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul(2136, 2136, 334);
                s.store_mul(2137, 2137, 334);
                s.store_mul_sub_rhs(339, 154, 2087, 2090);
                s.store_exp(340, 339);
                s.store_sub_offset_lhs(344, 340, (-1.0), 339);
            }
            s.b[2219] = (s.v[339] >= 1e-7);
            s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };
            let (assign47180_body122_e65948,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2219]) {
        let assign47180_body122_e65946: f64 = (-1.0);
        (assign47180_body122_e65946,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign47180_body122_e65948;
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2219]) {
                s.store_mul_scaled_sqrt_rhs(2096, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2096, 1.0);
                s.store_mul_offset_rhs(2123, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2125, 345, 1.0, 340);
            }
            s.b[2220] = (s.v[339] < (-1e-7));
            s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };
            let (assign47180_body128_e66041,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && s.b[2220]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign47180_body128_e66041;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && s.b[2220]) {
                s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2087), 1.0, s.ad_value(2115), p.p398));
                s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2090), 1.0, s.ad_value(2115), p.p398));
                s.store_mul_sqrt_ad_rhs(2096, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2096, 1.0);
                s.store_mul_add_ad_rhs(2123, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2125, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2221] = (s.v[339] > 0.0);
            s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && s.b[2221]) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2125, 2123);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && (!s.b[2221])) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2125, 2123);
            }
            let (assign47180_body146_e66457,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] != 0.0)) {
        let assign47180_body146_e66455: f64 = (150.0 + 1.0);
        (assign47180_body146_e66455,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47180_body146_e66457;
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2098, 2096, 1.0, 185, A::sub(s.ad_value(85), s.ad_value(2087)), 1.0);
                s.store_sub(2099, 2123, 185);
                s.copy_ad(2100, 2125);
                s.store_sub(2101, 2090, 2135);
                s.store_neg(2102, 2136);
                s.store_sub_from_scalar(2103, 1.0, 2137);
                s.store_add_scaled_products_indices(2104, 2099, 2103, 1.0, 2100, 2102, (-1.0));
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                if (s.v[2104] > 0.0) {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, (-1e-25));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.copy_ad(2106, 2103);
                s.store_neg(2107, 2100);
                s.store_neg(2108, 2102);
                s.copy_ad(2109, 2099);
                s.store_mul_add_scaled_products_indices_rhs(2110, 2105, 2106, 2098, -1.0, 2107, 2101, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(2111, 2105, 2108, 2098, -1.0, 2109, 2101, -1.0);
                s.store_abs(335, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2111]) as f64).abs()) {
                    s.store_abs(335, 2111);
                } else {
                }
            }
            s.b[2222] = (s.v[335] > 0.1);
            s.v[2222] = if s.b[2222] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2222]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2223] = (s.v[335] < 1e-10);
            s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };
            let (assign47180_body167_e66804,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2223]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign47180_body167_e66804;
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_add(2087, 2087, 2110);
                s.store_add(2090, 2090, 2111);
            }
            let (assign47180_body170_e66849,) = {
    if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
        let assign47180_body170_e66847: f64 = (s.v[97] + 1.0);
        (assign47180_body170_e66847,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47180_body170_e66849;
        }

    }

    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul_sub_rhs(339, 154, 2087, 2090);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[339] > 0.0) {
                s.store_mul_scaled_sqrt_rhs(2120, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2120, 209, 344);
            }
        }

        s.b[2225] = (1.0 == 1.0);
        s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };

        s.b[2226] = (((s.v[2087] - s.v[2085]) < p.p403) && (p.p403 >= 0.0));
        s.v[2226] = if s.b[2226] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2087), s.ad_value(2085)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign47310_e67033,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47310_e67033;

        let (assign47320_e67048,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47320_e67048;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2227] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = (6.0 == 1.0);
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        let (assign47510_e67351,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && s.b[2228]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47510_e67351;

        s.b[2229] = (6.0 == 2.0);
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        let (assign47530_e67376,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && s.b[2229]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47530_e67376;

        s.b[2230] = (6.0 == 4.0);
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        let (assign47550_e67404,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && s.b[2230]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47550_e67404;

        s.b[2231] = (6.0 == 8.0);
        s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };

        let (assign47570_e67435,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && (!s.b[2230])) && s.b[2231]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47570_e67435;

        let (assign47580_e67452,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47580_e67452;

        let mut assign47590_loop_guard: usize = 0;
        while {
            let assign47590_cond_e67470: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47590_cond_e67470 != 0.0
        } {
            assign47590_loop_guard += 1;
            assert!(assign47590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
                s.store_sqrt(726, 726);
            }
            let (assign47590_body1_e67507,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
        let assign47590_body1_e67505: f64 = (s.v[719] + 1.0);
        (assign47590_body1_e67505,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47590_body1_e67507;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && (!s.b[2227])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && (!s.b[2226])) {
            s.store_sub(336, 2087, 2085);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2116, 209, -1.0, 338);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2225])) {
            s.copy_ad(2116, 2120);
        }

        s.b[2232] = (1.0 == 1.0);
        s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.copy_ad(2157, 85);
            s.store_offset_mul(338, 2133, 2157, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_neg_ad(337, A::sqrt_scaled_input(s.ad_value(338), -1.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.store_offset_add_ad(2158, s.ad_value(2157), A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)), p.p397);
            s.copy_ad(2154, 2158);
        }

        let (assign47770_e67818,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign47770_e67818;

        let (assign47780_e67831,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign47780_e67831;

        let mut assign47790_loop_guard: usize = 0;
        while {
            let assign47790_cond_e67845: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47790_cond_e67845 != 0.0
        } {
            assign47790_loop_guard += 1;
            assert!(assign47790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_mul_neg_lhs(335, 154, 2154);
                s.store_exp(336, 335);
                s.store_sqrt_ad(338, A::div_scaled_inputs(s.ad_value(2112), 2.0, s.ad_value(154), 1.0));
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2155, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2233] = (s.v[335] > 0.0);
            s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2233]) {
                s.store_neg(2155, 2155);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2155, 1.0);
                s.store_mul_sub_from_scalar_rhs(2156, 345, 1.0, 336);
            }
            let (assign47790_body9_e68003,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] != 0.0)) {
        let assign47790_body9_e68001: f64 = (150.0 + 1.0);
        (assign47790_body9_e68001,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47790_body9_e68003;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {
                s.store_add_scaled_offset_product_rhs_mixed_iia(2098, 2155, 1.0, 185, A::sub(s.ad_value(2157), s.ad_value(2154)), p.p397, -1.0);
                s.store_add(2099, 185, 2156);
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2234] = (((s.v[2110]) as f64).abs() < 1e-10);
            s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };
            let (assign47790_body14_e68087,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && s.b[2234]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign47790_body14_e68087;
            s.b[2235] = (s.v[2110] > 0.1);
            s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && s.b[2235]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2236] = (s.v[2110] < (-0.1));
            s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && (!s.b[2235])) && s.b[2236]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {
                s.store_add(2154, 2154, 2110);
            }
            let (assign47790_body20_e68173,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
        let assign47790_body20_e68171: f64 = (s.v[97] + 1.0);
        (assign47790_body20_e68171,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47790_body20_e68173;
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.copy_ad(2159, 2154);
            s.store_sqrt_square_offset(782, 2159, ((4.0 * p.p404) * p.p404));
            s.store_offset_scaled_div(334, 2159, 782, 0.5, 0.5);
            s.store_scaled_add(2160, 2159, 782, 0.5);
        }

        s.b[2237] = (s.v[2160] < 0.0);
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2237]) {
            s.store_scalar(2160, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_offset_mul(338, 2133, 85, 1.0);
            s.store_offset(339, 2133, 1.0);
        }

        s.b[2238] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign47950_e68424,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47950_e68424;

        let (assign47960_e68440,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47960_e68440;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2239] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        s.b[2240] = (2.0 == 1.0);
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign48070_e68616,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && s.b[2240]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48070_e68616;

        s.b[2241] = (2.0 == 2.0);
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        let (assign48090_e68642,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && s.b[2241]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48090_e68642;

        s.b[2242] = (2.0 == 4.0);
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        let (assign48110_e68671,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && s.b[2242]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48110_e68671;

        s.b[2243] = (2.0 == 8.0);
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        let (assign48130_e68703,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && (!s.b[2242])) && s.b[2243]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48130_e68703;

        let (assign48140_e68721,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48140_e68721;

        let mut assign48150_loop_guard: usize = 0;
        while {
            let assign48150_cond_e68740: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48150_cond_e68740 != 0.0
        } {
            assign48150_loop_guard += 1;
            assert!(assign48150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
                s.store_sqrt(726, 726);
            }
            let (assign48150_body1_e68779,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
        let assign48150_body1_e68777: f64 = (s.v[719] + 1.0);
        (assign48150_body1_e68777,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48150_body1_e68779;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && (!s.b[2239])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)));
        }

        s.b[2244] = ((s.v[344] < p.p404) && (p.p404 >= 0.0));
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
            s.store_sub_from_scalar(781, p.p404, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p404 * p.p404));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign48320_e69087,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48320_e69087;

        let (assign48330_e69103,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48330_e69103;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2245] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        s.b[2246] = (2.0 == 1.0);
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        let (assign48440_e69279,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && s.b[2246]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48440_e69279;

        s.b[2247] = (2.0 == 2.0);
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        let (assign48460_e69305,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && s.b[2247]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48460_e69305;

        s.b[2248] = (2.0 == 4.0);
        s.v[2248] = if s.b[2248] { 1.0 } else { 0.0 };

        let (assign48480_e69334,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && s.b[2248]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48480_e69334;

        s.b[2249] = (2.0 == 8.0);
        s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };

        let (assign48500_e69366,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && (!s.b[2248])) && s.b[2249]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48500_e69366;

        let (assign48510_e69384,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48510_e69384;

        let mut assign48520_loop_guard: usize = 0;
        while {
            let assign48520_cond_e69403: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48520_cond_e69403 != 0.0
        } {
            assign48520_loop_guard += 1;
            assert!(assign48520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
                s.store_sqrt(726, 726);
            }
            let (assign48520_body1_e69442,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
        let assign48520_body1_e69440: f64 = (s.v[719] + 1.0);
        (assign48520_body1_e69440,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48520_body1_e69442;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && (!s.b[2245])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p404);
            s.store_div_scaled_product_indices(334, 725, 726, p.p404, 770, 1.0);
            s.store_sub_from_scalar(2160, p.p404, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2244])) {
            s.copy_ad(2160, 344);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(349, 790);
            s.store_div(335, 790, 2160);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), s.ad_value(658));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div(348, 790, 338);
            s.copy_ad(790, 348);
        }

        s.b[2250] = (s.v[790] < 0.0);
        s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2250]) {
            s.copy_ad(2088, 2087);
            s.copy_ad(2093, 2092);
            s.copy_ad(2091, 2090);
            s.copy_ad(2121, 2120);
            s.copy_ad(2117, 2116);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.copy_ad(2086, 790);
            s.store_add_scaled_inputs3_offset_indices(781, 2087, 1.0, 2086, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 2087, 2086, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2095, 2087, 1.0, 2086, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 2095, 1.0, 2115, -1.0, 2089, 1.0, (-0.01));
            s.store_scaled_sub(782, 2115, 2089, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2095, 2115, 1.0, 2089, (-1.0), 781, 0.5, 782, 0.5);
            s.copy_ad(2091, 2086);
            s.copy_ad(2088, 2095);
        }

        let (assign48890_e70067,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign48890_e70067;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_mul(2139, 2127, 2128);
        }

        let (assign48910_e70097,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.v[98] = assign48910_e70097;

    }
}
