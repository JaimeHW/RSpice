#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_scalar(1914, 3.0);
            s.store_scalar(1853, 0.0);
            s.store_scalar(1854, 0.0);
            s.store_scalar(1862, 0.0);
            s.store_scalar(1863, 0.0);
            s.store_scalar(1895, 0.0);
            s.store_scalar(1896, 0.0);
            s.store_scalar(1866, 0.0);
            s.store_scalar(1868, 0.0);
            s.store_scalar(1867, 0.0);
            s.store_scalar(1869, 0.0);
            s.store_scalar(1839, 0.0);
            s.store_scalar(1834, 0.0);
            s.copy_ad(1887, 1435);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));
            s.store_div_scaled_product_add_scaled_denominator_indices(962, 1908, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);
            s.store_sub(335, 1855, 1438);
        }

        s.b[1917] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
            s.store_sub_from_scalar(781, 0.1, 335);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign37370_e42957,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign37370_e42957;

        let (assign37380_e42968,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37380_e42968;

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
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

        s.b[1918] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        s.b[1919] = (4.0 == 1.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        let (assign37530_e43151,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && s.b[1919]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37530_e43151;

        s.b[1920] = (4.0 == 2.0);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        let (assign37550_e43172,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && s.b[1920]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37550_e43172;

        s.b[1921] = (4.0 == 4.0);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        let (assign37570_e43196,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && (!s.b[1920])) && s.b[1921]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37570_e43196;

        s.b[1922] = (4.0 == 8.0);
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        let (assign37590_e43223,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (!s.b[1919])) && (!s.b[1920])) && (!s.b[1921])) && s.b[1922]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37590_e43223;

        let (assign37600_e43236,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign37600_e43236;

        let mut assign37610_loop_guard: usize = 0;
        while {
            let assign37610_cond_e43250: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign37610_cond_e43250 != 0.0
        } {
            assign37610_loop_guard += 1;
            assert!(assign37610_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) {
                s.store_sqrt(726, 726);
            }
            let (assign37610_body1_e43279,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && s.b[1918]) {
        let assign37610_body1_e43277: f64 = (s.v[719] + 1.0);
        (assign37610_body1_e43277,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign37610_body1_e43279;
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) && (!s.b[1918])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1917]) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1917])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_mul(1838, 962, 336);
            s.store_sqrt(1836, 1838);
        }

        s.b[1923] = (p.p345 != 0.0);
        s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1923]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_scalar(1851, 0.0);
        }

        s.b[1924] = (s.v[1836] > s.v[965]);
        s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1924]) {
            s.copy_ad(1835, 965);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1924])) {
            s.copy_ad(1835, 1836);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(1860, 781, (-0.5), 782, (-0.5), 0.3);
            s.store_add_scaled_inputs3_offset_indices(781, 1860, 1.0, 1887, -1.0, 1855, 1.0, (-0.01));
            s.store_scaled_sub(782, 1887, 1855, (4.0 * 0.01));
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1860, 1887, 1.0, 1855, (-1.0), 781, 0.5, 782, 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(1888, 1855, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);
            s.store_offset_sub(1834, 965, 1835, 1e-15);
        }

        let (assign38050_e43908,) = {
    if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign38050_e43908;

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_scalar(1850, 0.2);
            s.copy_ad(1853, 1860);
            s.copy_ad(1856, 1851);
            s.copy_ad(1858, 1888);
        }

        let (assign38100_e43953,) = {
    if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign38100_e43953;

    }

    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign38110_loop_guard: usize = 0;
        while {
            let assign38110_cond_e43963: f64 = if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign38110_cond_e43963 != 0.0
        } {
            assign38110_loop_guard += 1;
            assert!(assign38110_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_mul_sub_ad_rhs(1858, 1901, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1902), s.ad_value(1856), 1.0), s.ad_value(1855));
                s.store_mul(1842, 1901, 1902);
                s.store_sub(335, 1856, 1858);
            }
            s.b[1925] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38110_body9_e44085,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body9_e44085;
            let (assign38110_body10_e44096,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body10_e44096;
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1926] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            s.b[1927] = (2.0 == 1.0);
            s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };
            let (assign38110_body21_e44227,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && s.b[1927]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body21_e44227;
            s.b[1928] = (2.0 == 2.0);
            s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };
            let (assign38110_body23_e44248,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && s.b[1928]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body23_e44248;
            s.b[1929] = (2.0 == 4.0);
            s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };
            let (assign38110_body25_e44272,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && (!s.b[1928])) && s.b[1929]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body25_e44272;
            s.b[1930] = (2.0 == 8.0);
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            let (assign38110_body27_e44299,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (!s.b[1927])) && (!s.b[1928])) && (!s.b[1929])) && s.b[1930]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body27_e44299;
            let (assign38110_body28_e44312,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body28_e44312;
            let mut assign38110_body29_loop_guard: usize = 0;
            while {
                let assign38110_body29_cond_e44326: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body29_cond_e44326 != 0.0
            } {
                assign38110_body29_loop_guard += 1;
                assert!(assign38110_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38110_body29_body1_e44355,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && s.b[1926]) {
        let assign38110_body29_body1_e44353: f64 = (s.v[719] + 1.0);
        (assign38110_body29_body1_e44353,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38110_body29_body1_e44355;
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) && (!s.b[1926])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1925]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1925])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_sqrt_mul(1830, 1908, 336);
            }
            s.b[1931] = ((s.v[1830] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
                s.store_offset_sub(781, 1830, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38110_body45_e44570,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body45_e44570;
            let (assign38110_body46_e44581,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body46_e44581;
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1932] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            s.b[1933] = (2.0 == 1.0);
            s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };
            let (assign38110_body57_e44712,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && s.b[1933]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body57_e44712;
            s.b[1934] = (2.0 == 2.0);
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            let (assign38110_body59_e44733,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && s.b[1934]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body59_e44733;
            s.b[1935] = (2.0 == 4.0);
            s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };
            let (assign38110_body61_e44757,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && (!s.b[1934])) && s.b[1935]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body61_e44757;
            s.b[1936] = (2.0 == 8.0);
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            let (assign38110_body63_e44784,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (!s.b[1933])) && (!s.b[1934])) && (!s.b[1935])) && s.b[1936]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body63_e44784;
            let (assign38110_body64_e44797,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body64_e44797;
            let mut assign38110_body65_loop_guard: usize = 0;
            while {
                let assign38110_body65_cond_e44811: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body65_cond_e44811 != 0.0
            } {
                assign38110_body65_loop_guard += 1;
                assert!(assign38110_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38110_body65_body1_e44840,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && s.b[1932]) {
        let assign38110_body65_body1_e44838: f64 = (s.v[719] + 1.0);
        (assign38110_body65_body1_e44838,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38110_body65_body1_e44840;
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) && (!s.b[1932])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1830, 965, (-1e-8), 780);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1931]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1931])) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1931])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_mul(1866, 1830, 1905);
                s.store_mul_ad_product_lhs(1844, A::div_from_scalar(1.034943e-10, s.ad_value(1830)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1846, A::div_from_scalar((-1.034943e-10), s.ad_value(1830)), s.ad_value(334), 337);
            }
            s.b[1937] = (p.p49 == 0.0);
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1937]) {
                s.store_add_ad_lhs(1839, A::div_scaled_inputs_product(s.ad_value(1903), 1.0, s.ad_value(1838), 1.0, s.ad_value(965), s.ad_value(1835), (-2.0), s.ad_value(1908), 1.0), 1853);
                s.store_scalar(1840, 1.0);
                s.store_scalar(1841, 0.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1937])) {
                s.store_add_ad_rhs(1839, 1853, A::div_scaled_add_product(s.ad_value(1903), 1.0, s.ad_value(1830), A::sub_scaled_inputs(s.ad_value(1830), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1908), 1.0));
                s.store_scalar(1840, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1841, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1830)), s.ad_value(334), (-1.0)), 1.0, 1842);
            }
            s.b[1938] = ((s.v[1839] > (s.v[1851] - s.v[1850])) && (s.v[1850] >= 0.0));
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
                s.store_add_scaled_inputs3_indices(781, 1839, 1.0, 1851, (-1.0), 1850, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1850);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38110_body90_e45191,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body90_e45191;
            let (assign38110_body91_e45202,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body91_e45202;
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
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
            s.b[1939] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            s.b[1940] = (4.0 == 1.0);
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            let (assign38110_body106_e45385,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && s.b[1940]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body106_e45385;
            s.b[1941] = (4.0 == 2.0);
            s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };
            let (assign38110_body108_e45406,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && s.b[1941]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body108_e45406;
            s.b[1942] = (4.0 == 4.0);
            s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };
            let (assign38110_body110_e45430,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && (!s.b[1941])) && s.b[1942]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body110_e45430;
            s.b[1943] = (4.0 == 8.0);
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            let (assign38110_body112_e45457,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (!s.b[1940])) && (!s.b[1941])) && (!s.b[1942])) && s.b[1943]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body112_e45457;
            let (assign38110_body113_e45470,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body113_e45470;
            let mut assign38110_body114_loop_guard: usize = 0;
            while {
                let assign38110_body114_cond_e45484: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body114_cond_e45484 != 0.0
            } {
                assign38110_body114_loop_guard += 1;
                assert!(assign38110_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38110_body114_body1_e45513,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && s.b[1939]) {
        let assign38110_body114_body1_e45511: f64 = (s.v[719] + 1.0);
        (assign38110_body114_body1_e45511,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38110_body114_body1_e45513;
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) && (!s.b[1939])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1850, 726);
                s.store_div_scaled_product3_indices(334, 1850, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1839, 1851, 1.0, 1850, (-1.0), 780, 1.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1938]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1938])) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1938])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_mul(1840, 1840, 334);
                s.store_mul(1841, 1841, 334);
                s.store_add_scaled_inputs3_indices(335, 1858, 1.0, 1887, (-1.0), 1855, 1.0);
            }
            s.b[1944] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38110_body132_e45751,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body132_e45751;
            let (assign38110_body133_e45762,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body133_e45762;
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1945] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            s.b[1946] = (2.0 == 1.0);
            s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };
            let (assign38110_body144_e45893,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && s.b[1946]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body144_e45893;
            s.b[1947] = (2.0 == 2.0);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            let (assign38110_body146_e45914,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && s.b[1947]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body146_e45914;
            s.b[1948] = (2.0 == 4.0);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            let (assign38110_body148_e45938,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && (!s.b[1947])) && s.b[1948]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body148_e45938;
            s.b[1949] = (2.0 == 8.0);
            s.v[1949] = if s.b[1949] { 1.0 } else { 0.0 };
            let (assign38110_body150_e45965,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (!s.b[1946])) && (!s.b[1947])) && (!s.b[1948])) && s.b[1949]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38110_body150_e45965;
            let (assign38110_body151_e45978,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38110_body151_e45978;
            let mut assign38110_body152_loop_guard: usize = 0;
            while {
                let assign38110_body152_cond_e45992: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38110_body152_cond_e45992 != 0.0
            } {
                assign38110_body152_loop_guard += 1;
                assert!(assign38110_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38110_body152_body1_e46021,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && s.b[1945]) {
        let assign38110_body152_body1_e46019: f64 = (s.v[719] + 1.0);
        (assign38110_body152_body1_e46019,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38110_body152_body1_e46021;
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) && (!s.b[1945])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1944]) {
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1944])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
                s.store_sqrt_mul(1832, 1909, 336);
                s.store_mul_neg_lhs(1867, 1832, 1904);
                s.store_mul_div_from_scalar_lhs(1848, (-1.034943e-10), 1832, 337);
                s.store_mul_sub_rhs(335, 154, 1853, 1856);
                s.store_exp(336, 335);
            }
            s.b[1950] = (s.v[1853] >= s.v[1856]);
            s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1950]) {
                s.store_mul_scaled_sqrt_ad_rhs(1862, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1897, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1862), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1899, 1897);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1950])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1853), s.ad_value(1887)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1856), s.ad_value(1887)));
                s.store_mul_sqrt_ad_rhs(1862, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1862, 1.0);
                s.store_mul_add_ad_rhs(1897, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1899, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            let (assign38110_body176_e46405,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] != 0.0)) {
        let assign38110_body176_e46403: f64 = (150.0 + 1.0);
        (assign38110_body176_e46403,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign38110_body176_e46405;
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1870, A::add_scaled_product(s.ad_value(1862), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1853)), 1.0), 1.0, 1866, 1.0, 1867, 1.0);
                s.store_sub(1871, 1897, 185);
                s.store_add_ad_rhs(1872, 1899, A::add_scaled_value_products(s.ad_value(1844), 1.0, s.ad_value(1846), s.ad_value(1842), 1.0, s.ad_value(1848), s.ad_value(1842), 1.0));
                s.store_sub(1873, 1856, 1839);
                s.store_neg(1874, 1840);
                s.store_sub_from_scalar(1875, 1.0, 1841);
                s.store_add_scaled_products_indices(1876, 1871, 1875, 1.0, 1872, 1874, (-1.0));
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                if (s.v[1876] > 0.0) {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, (-1e-25));
                }
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1878, 1875);
                s.store_neg(1879, 1872);
                s.store_neg(1880, 1874);
                s.copy_ad(1881, 1871);
                s.store_mul_add_scaled_products_indices_rhs(1882, 1877, 1878, 1870, -1.0, 1879, 1873, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1883, 1877, 1880, 1870, -1.0, 1881, 1873, -1.0);
                s.store_abs(335, 1882);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1883]) as f64).abs()) {
                    s.store_abs(335, 1883);
                } else {
                }
            }
            s.b[1951] = (s.v[335] > 0.1);
            s.v[1951] = if s.b[1951] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) && s.b[1951]) {
                s.store_mul_div_from_scalar_rhs(1882, 1882, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1883, 1883, 0.1, 335);
            }
            s.b[1952] = (s.v[335] < 1e-12);
            s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };
            let (assign38110_body197_e46728,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) && s.b[1952]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign38110_body197_e46728;
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (s.v[79] == 0.0)) {
                s.store_add(1853, 1853, 1882);
                s.store_add(1856, 1856, 1883);
            }
            let (assign38110_body200_e46767,) = {
    if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
        let assign38110_body200_e46765: f64 = (s.v[97] + 1.0);
        (assign38110_body200_e46765,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign38110_body200_e46767;
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_mul_sub_rhs(335, 154, 1853, 1856);
            s.store_exp(336, 335);
        }

        s.b[1954] = (s.v[1853] >= s.v[1856]);
        s.v[1954] = if s.b[1954] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1954]) {
            s.copy_ad(1892, 1862);
            s.store_scalar(1895, 0.0);
            s.store_scalar(1864, 0.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1954])) {
            s.store_scalar(1892, 0.0);
            s.store_mul_sqrt_ad_rhs(1895, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[1955] = (s.v[1836] > s.v[965]);
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1954])) && s.b[1955]) {
            s.store_scalar(1864, 0.0);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1954])) && (!s.b[1955])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1853), s.ad_value(1887)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1856), s.ad_value(1887)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1864, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[1956] = (((s.v[1853] - s.v[1851]) < s.v[1911]) && (s.v[1911] >= 0.0));
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {
            s.store_add_scaled_inputs3_indices(781, 1911, 1.0, 1853, -1.0, 1851, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1911);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign38310_e47024,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38310_e47024;

        let (assign38320_e47035,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38320_e47035;

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {
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

        s.b[1957] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1957] = if s.b[1957] { 1.0 } else { 0.0 };

        s.b[1958] = (4.0 == 1.0);
        s.v[1958] = if s.b[1958] { 1.0 } else { 0.0 };

        let (assign38470_e47218,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && s.b[1958]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38470_e47218;

        s.b[1959] = (4.0 == 2.0);
        s.v[1959] = if s.b[1959] { 1.0 } else { 0.0 };

        let (assign38490_e47239,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (!s.b[1958])) && s.b[1959]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38490_e47239;

        s.b[1960] = (4.0 == 4.0);
        s.v[1960] = if s.b[1960] { 1.0 } else { 0.0 };

        let (assign38510_e47263,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (!s.b[1958])) && (!s.b[1959])) && s.b[1960]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38510_e47263;

        s.b[1961] = (4.0 == 8.0);
        s.v[1961] = if s.b[1961] { 1.0 } else { 0.0 };

        let (assign38530_e47290,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (!s.b[1958])) && (!s.b[1959])) && (!s.b[1960])) && s.b[1961]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38530_e47290;

        let (assign38540_e47303,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38540_e47303;

        let mut assign38550_loop_guard: usize = 0;
        while {
            let assign38550_cond_e47317: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38550_cond_e47317 != 0.0
        } {
            assign38550_loop_guard += 1;
            assert!(assign38550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) {
                s.store_sqrt(726, 726);
            }
            let (assign38550_body1_e47346,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && s.b[1957]) {
        let assign38550_body1_e47344: f64 = (s.v[719] + 1.0);
        (assign38550_body1_e47344,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38550_body1_e47346;
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) && (!s.b[1957])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1911, 726);
            s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1911, 780);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1956]) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1956])) {
            s.store_sub(336, 1853, 1851);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1889, 209, -1.0, 338);
            s.copy_ad(349, 790);
        }

        s.b[1962] = (s.v[790] > 1e-6);
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            s.store_scalar(344, 1e-25);
            s.store_offset_mul_ad(338, s.ad_value(1907), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1907, 1.0);
        }

        s.b[1963] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[1963] = if s.b[1963] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign38770_e47649,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38770_e47649;

        let (assign38780_e47662,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38780_e47662;

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1964] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        s.b[1965] = (2.0 == 1.0);
        s.v[1965] = if s.b[1965] { 1.0 } else { 0.0 };

        let (assign38890_e47811,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && s.b[1965]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38890_e47811;

        s.b[1966] = (2.0 == 2.0);
        s.v[1966] = if s.b[1966] { 1.0 } else { 0.0 };

        let (assign38910_e47834,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (!s.b[1965])) && s.b[1966]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38910_e47834;

        s.b[1967] = (2.0 == 4.0);
        s.v[1967] = if s.b[1967] { 1.0 } else { 0.0 };

        let (assign38930_e47860,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (!s.b[1965])) && (!s.b[1966])) && s.b[1967]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38930_e47860;

        s.b[1968] = (2.0 == 8.0);
        s.v[1968] = if s.b[1968] { 1.0 } else { 0.0 };

        let (assign38950_e47889,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (!s.b[1965])) && (!s.b[1966])) && (!s.b[1967])) && s.b[1968]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38950_e47889;

        let (assign38960_e47904,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38960_e47904;

        let mut assign38970_loop_guard: usize = 0;
        while {
            let assign38970_cond_e47920: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38970_cond_e47920 != 0.0
        } {
            assign38970_loop_guard += 1;
            assert!(assign38970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) {
                s.store_sqrt(726, 726);
            }
            let (assign38970_body1_e47953,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && s.b[1964]) {
        let assign38970_body1_e47951: f64 = (s.v[719] + 1.0);
        (assign38970_body1_e47951,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38970_body1_e47953;
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) && (!s.b[1964])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1963]) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1963])) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1963])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1906), 1.0, s.ad_value(337)));
        }

        s.b[1969] = ((s.v[344] < 1.0) && (1.0 >= 0.0));
        s.v[1969] = if s.b[1969] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {
            s.store_sub_from_scalar(781, 1.0, 344);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign39140_e48213,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign39140_e48213;

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
    ) {
        let (assign39150_e48226,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39150_e48226;

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1970] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        s.b[1971] = (2.0 == 1.0);
        s.v[1971] = if s.b[1971] { 1.0 } else { 0.0 };

        let (assign39260_e48375,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && s.b[1971]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39260_e48375;

        s.b[1972] = (2.0 == 2.0);
        s.v[1972] = if s.b[1972] { 1.0 } else { 0.0 };

        let (assign39280_e48398,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (!s.b[1971])) && s.b[1972]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39280_e48398;

        s.b[1973] = (2.0 == 4.0);
        s.v[1973] = if s.b[1973] { 1.0 } else { 0.0 };

        let (assign39300_e48424,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (!s.b[1971])) && (!s.b[1972])) && s.b[1973]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39300_e48424;

        s.b[1974] = (2.0 == 8.0);
        s.v[1974] = if s.b[1974] { 1.0 } else { 0.0 };

        let (assign39320_e48453,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (!s.b[1971])) && (!s.b[1972])) && (!s.b[1973])) && s.b[1974]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39320_e48453;

        let (assign39330_e48468,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign39330_e48468;

        let mut assign39340_loop_guard: usize = 0;
        while {
            let assign39340_cond_e48484: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign39340_cond_e48484 != 0.0
        } {
            assign39340_loop_guard += 1;
            assert!(assign39340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) {
                s.store_sqrt(726, 726);
            }
            let (assign39340_body1_e48517,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && s.b[1970]) {
        let assign39340_body1_e48515: f64 = (s.v[719] + 1.0);
        (assign39340_body1_e48515,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39340_body1_e48517;
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) && (!s.b[1970])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(344, 1.0, 780);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && s.b[1969]) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1969])) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) && (!s.b[1969])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            s.store_div(335, 790, 344);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1962]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1962])) {
            s.copy_ad(348, 790);
        }

        s.b[1975] = (s.v[790] < 0.0);
        s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[1975]) {
            s.copy_ad(1854, 1853);
            s.copy_ad(1859, 1858);
            s.copy_ad(1857, 1856);
            s.copy_ad(1865, 1864);
            s.copy_ad(1893, 1892);
            s.copy_ad(1890, 1889);
            s.copy_ad(1868, 1866);
            s.copy_ad(1869, 1867);
        }

        let (assign39600_e48875,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
        (s.v[1836],)
    } else {
        (s.v[1837],)
    }
};
        s.v[1837] = assign39600_e48875;

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            s.copy_ad(1852, 790);
            s.store_add_scaled_inputs3_offset_indices(781, 1853, 1.0, 1852, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 1853, 1852, (4.0 * 0.01));
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1861, 1853, 1.0, 1852, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 1861, 1.0, 1887, -1.0, 1855, 1.0, (-0.01));
            s.store_scaled_sub(782, 1887, 1855, (4.0 * 0.01));
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1861, 1887, 1.0, 1855, (-1.0), 781, 0.5, 782, 0.5);
            s.store_mul(212, 209, 186);
            s.store_square(213, 212);
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1887))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            s.store_add_ad_rhs(92, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
        }

        let (assign39790_e49215,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign39790_e49215;

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            s.copy_ad(1854, 1861);
            s.copy_ad(1857, 1852);
        }

        let (assign39820_e49251,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign39820_e49251;

    }

    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign39830_loop_guard: usize = 0;
        while {
            let assign39830_cond_e49264: f64 = if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign39830_cond_e49264 != 0.0
        } {
            assign39830_loop_guard += 1;
            assert!(assign39830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
                s.store_mul_sub_ad_rhs(1859, 1901, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1902), s.ad_value(1857), 1.0), s.ad_value(1855));
                s.store_mul(1843, 1901, 1902);
                s.store_sub(335, 1857, 1859);
            }
            s.b[1976] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39830_body9_e49413,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body9_e49413;
            let (assign39830_body10_e49427,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body10_e49427;
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1977] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };
            s.b[1978] = (2.0 == 1.0);
            s.v[1978] = if s.b[1978] { 1.0 } else { 0.0 };
            let (assign39830_body21_e49585,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && s.b[1978]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body21_e49585;
            s.b[1979] = (2.0 == 2.0);
            s.v[1979] = if s.b[1979] { 1.0 } else { 0.0 };
            let (assign39830_body23_e49609,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (!s.b[1978])) && s.b[1979]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body23_e49609;
            s.b[1980] = (2.0 == 4.0);
            s.v[1980] = if s.b[1980] { 1.0 } else { 0.0 };
            let (assign39830_body25_e49636,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (!s.b[1978])) && (!s.b[1979])) && s.b[1980]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body25_e49636;
            s.b[1981] = (2.0 == 8.0);
            s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };
            let (assign39830_body27_e49666,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (!s.b[1978])) && (!s.b[1979])) && (!s.b[1980])) && s.b[1981]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body27_e49666;
            let (assign39830_body28_e49682,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body28_e49682;
            let mut assign39830_body29_loop_guard: usize = 0;
            while {
                let assign39830_body29_cond_e49699: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39830_body29_cond_e49699 != 0.0
            } {
                assign39830_body29_loop_guard += 1;
                assert!(assign39830_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39830_body29_body1_e49734,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && s.b[1977]) {
        let assign39830_body29_body1_e49732: f64 = (s.v[719] + 1.0);
        (assign39830_body29_body1_e49732,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39830_body29_body1_e49734;
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) && (!s.b[1977])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1976]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1976])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
                s.store_sqrt_mul(1831, 1908, 336);
            }
            s.b[1982] = ((s.v[1831] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {
                s.store_offset_sub(781, 1831, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39830_body45_e49994,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body45_e49994;
            let (assign39830_body46_e50008,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body46_e50008;
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1983] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };
            s.b[1984] = (2.0 == 1.0);
            s.v[1984] = if s.b[1984] { 1.0 } else { 0.0 };
            let (assign39830_body57_e50166,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && s.b[1984]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body57_e50166;
            s.b[1985] = (2.0 == 2.0);
            s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };
            let (assign39830_body59_e50190,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (!s.b[1984])) && s.b[1985]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body59_e50190;
            s.b[1986] = (2.0 == 4.0);
            s.v[1986] = if s.b[1986] { 1.0 } else { 0.0 };
            let (assign39830_body61_e50217,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (!s.b[1984])) && (!s.b[1985])) && s.b[1986]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body61_e50217;
            s.b[1987] = (2.0 == 8.0);
            s.v[1987] = if s.b[1987] { 1.0 } else { 0.0 };
            let (assign39830_body63_e50247,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (!s.b[1984])) && (!s.b[1985])) && (!s.b[1986])) && s.b[1987]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body63_e50247;
            let (assign39830_body64_e50263,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body64_e50263;
            let mut assign39830_body65_loop_guard: usize = 0;
            while {
                let assign39830_body65_cond_e50280: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39830_body65_cond_e50280 != 0.0
            } {
                assign39830_body65_loop_guard += 1;
                assert!(assign39830_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39830_body65_body1_e50315,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && s.b[1983]) {
        let assign39830_body65_body1_e50313: f64 = (s.v[719] + 1.0);
        (assign39830_body65_body1_e50313,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39830_body65_body1_e50315;
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) && (!s.b[1983])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1831, 965, (-1e-8), 780);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1982]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1982])) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1982])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
                s.store_mul(1868, 1831, 1905);
                s.store_mul_ad_product_lhs(1845, A::div_from_scalar(1.034943e-10, s.ad_value(1831)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1847, A::div_from_scalar((-1.034943e-10), s.ad_value(1831)), s.ad_value(334), 337);
            }
            s.b[1988] = (p.p49 == 0.0);
            s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1988]) {
                s.store_add_ad_lhs(1839, A::div_scaled_inputs_product(s.ad_value(1903), 1.0, s.ad_value(1838), 1.0, s.ad_value(965), s.ad_value(1835), (-2.0), s.ad_value(1908), 1.0), 1854);
                s.store_scalar(1840, 1.0);
                s.store_scalar(1841, 0.0);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1988])) {
                s.store_add_ad_rhs(1839, 1854, A::div_scaled_add_product(s.ad_value(1903), 1.0, s.ad_value(1831), A::sub_scaled_inputs(s.ad_value(1831), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1908), 1.0));
                s.store_scalar(1840, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1841, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1831)), s.ad_value(334), (-1.0)), 1.0, 1843);
            }
            s.b[1989] = ((s.v[1839] > (s.v[1852] - s.v[1850])) && (s.v[1850] >= 0.0));
            s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {
                s.store_add_scaled_inputs3_indices(781, 1839, 1.0, 1852, (-1.0), 1850, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1850);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39830_body90_e50735,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body90_e50735;
            let (assign39830_body91_e50749,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body91_e50749;
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {
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
            s.b[1990] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };
            s.b[1991] = (4.0 == 1.0);
            s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };
            let (assign39830_body106_e50971,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && s.b[1991]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body106_e50971;
            s.b[1992] = (4.0 == 2.0);
            s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };
            let (assign39830_body108_e50995,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (!s.b[1991])) && s.b[1992]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body108_e50995;
            s.b[1993] = (4.0 == 4.0);
            s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };
            let (assign39830_body110_e51022,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (!s.b[1991])) && (!s.b[1992])) && s.b[1993]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body110_e51022;
            s.b[1994] = (4.0 == 8.0);
            s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };
            let (assign39830_body112_e51052,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (!s.b[1991])) && (!s.b[1992])) && (!s.b[1993])) && s.b[1994]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body112_e51052;
            let (assign39830_body113_e51068,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body113_e51068;
            let mut assign39830_body114_loop_guard: usize = 0;
            while {
                let assign39830_body114_cond_e51085: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39830_body114_cond_e51085 != 0.0
            } {
                assign39830_body114_loop_guard += 1;
                assert!(assign39830_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39830_body114_body1_e51120,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && s.b[1990]) {
        let assign39830_body114_body1_e51118: f64 = (s.v[719] + 1.0);
        (assign39830_body114_body1_e51118,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39830_body114_body1_e51120;
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) && (!s.b[1990])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1850, 726);
                s.store_div_scaled_product3_indices(334, 1850, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1839, 1852, 1.0, 1850, (-1.0), 780, 1.0);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1989]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1989])) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1989])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
                s.store_mul(1840, 1840, 334);
                s.store_mul(1841, 1841, 334);
                s.store_add_scaled_inputs3_indices(335, 1859, 1.0, 1887, (-1.0), 1855, 1.0);
            }
            s.b[1995] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39830_body132_e51409,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body132_e51409;
            let (assign39830_body133_e51423,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body133_e51423;
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1996] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };
            s.b[1997] = (2.0 == 1.0);
            s.v[1997] = if s.b[1997] { 1.0 } else { 0.0 };
            let (assign39830_body144_e51581,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && s.b[1997]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body144_e51581;
            s.b[1998] = (2.0 == 2.0);
            s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };
            let (assign39830_body146_e51605,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) && s.b[1998]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body146_e51605;
            s.b[1999] = (2.0 == 4.0);
            s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };
            let (assign39830_body148_e51632,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) && (!s.b[1998])) && s.b[1999]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body148_e51632;
            s.b[2000] = (2.0 == 8.0);
            s.v[2000] = if s.b[2000] { 1.0 } else { 0.0 };
            let (assign39830_body150_e51662,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (!s.b[1997])) && (!s.b[1998])) && (!s.b[1999])) && s.b[2000]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39830_body150_e51662;
            let (assign39830_body151_e51678,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39830_body151_e51678;
            let mut assign39830_body152_loop_guard: usize = 0;
            while {
                let assign39830_body152_cond_e51695: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39830_body152_cond_e51695 != 0.0
            } {
                assign39830_body152_loop_guard += 1;
                assert!(assign39830_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39830_body152_body1_e51730,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && s.b[1996]) {
        let assign39830_body152_body1_e51728: f64 = (s.v[719] + 1.0);
        (assign39830_body152_body1_e51728,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39830_body152_body1_e51730;
            }
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) && (!s.b[1996])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[1995]) {
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[1995])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
                s.store_sqrt_mul(1833, 1909, 336);
                s.store_mul_neg_lhs(1869, 1833, 1904);
                s.store_mul_div_from_scalar_lhs(1849, (-1.034943e-10), 1833, 337);
                s.store_mul_sub_rhs(335, 154, 1854, 1857);
                s.store_exp(336, 335);
            }
            s.b[2001] = (s.v[1854] >= s.v[1857]);
            s.v[2001] = if s.b[2001] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2001]) {
                s.store_mul_scaled_sqrt_ad_rhs(1863, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1898, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1863), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1900, 1898);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2001])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1854), s.ad_value(1887)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1857), s.ad_value(1887)));
                s.store_mul_sqrt_ad_rhs(1863, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1863, 1.0);
                s.store_mul_add_ad_rhs(1898, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1900, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            let (assign39830_body176_e52183,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] != 0.0)) {
        let assign39830_body176_e52181: f64 = (150.0 + 1.0);
        (assign39830_body176_e52181,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign39830_body176_e52183;
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1870, A::add_scaled_product(s.ad_value(1863), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1854)), 1.0), 1.0, 1868, 1.0, 1869, 1.0);
                s.store_sub(1871, 1898, 185);
                s.store_add_ad_rhs(1872, 1900, A::add_scaled_value_products(s.ad_value(1845), 1.0, s.ad_value(1847), s.ad_value(1843), 1.0, s.ad_value(1849), s.ad_value(1843), 1.0));
                s.store_sub(1873, 1857, 1839);
                s.store_neg(1874, 1840);
                s.store_sub_from_scalar(1875, 1.0, 1841);
                s.store_add_scaled_products_indices(1876, 1871, 1875, 1.0, 1872, 1874, (-1.0));
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {
                if (s.v[1876] > 0.0) {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1877, 1.0, 1876, (-1e-25));
                }
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {
                s.copy_ad(1878, 1875);
                s.store_neg(1879, 1872);
                s.store_neg(1880, 1874);
                s.copy_ad(1881, 1871);
                s.store_mul_add_scaled_products_indices_rhs(1882, 1877, 1878, 1870, -1.0, 1879, 1873, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1883, 1877, 1880, 1870, -1.0, 1881, 1873, -1.0);
                s.store_abs(335, 1882);
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1883]) as f64).abs()) {
                    s.store_abs(335, 1883);
                } else {
                }
            }
            s.b[2002] = (s.v[335] > 0.1);
            s.v[2002] = if s.b[2002] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) && s.b[2002]) {
                s.store_mul_div_from_scalar_rhs(1882, 1882, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1883, 1883, 0.1, 335);
            }
            s.b[2003] = (s.v[335] < 1e-12);
            s.v[2003] = if s.b[2003] { 1.0 } else { 0.0 };
            let (assign39830_body197_e52563,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) && s.b[2003]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign39830_body197_e52563;
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (s.v[79] == 0.0)) {
                s.store_add(1854, 1854, 1882);
                s.store_add(1857, 1857, 1883);
            }
            let (assign39830_body200_e52611,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
        let assign39830_body200_e52609: f64 = (s.v[97] + 1.0);
        (assign39830_body200_e52609,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign39830_body200_e52611;
        }

    }

    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            s.store_mul_sub_rhs(335, 154, 1854, 1857);
            s.store_exp(336, 335);
        }

        s.b[2005] = (s.v[1854] >= s.v[1857]);
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2005]) {
            s.copy_ad(1893, 1863);
            s.store_scalar(1896, 0.0);
            s.store_scalar(1865, 0.0);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2005])) {
            s.store_scalar(1893, 0.0);
            s.store_mul_sqrt_ad_rhs(1896, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[2006] = (s.v[1837] > s.v[965]);
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2005])) && s.b[2006]) {
            s.store_scalar(1865, 0.0);
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2005])) && (!s.b[2006])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1887)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1857), s.ad_value(1887)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1865, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[2007] = (((s.v[1854] - s.v[1852]) < s.v[1911]) && (s.v[1911] >= 0.0));
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {
            s.store_add_scaled_inputs3_indices(781, 1911, 1.0, 1854, -1.0, 1852, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1911);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign40030_e52916,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40030_e52916;

        let (assign40040_e52930,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40040_e52930;

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {
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

        s.b[2008] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        s.b[2009] = (4.0 == 1.0);
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        let (assign40190_e53152,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && s.b[2009]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40190_e53152;

        s.b[2010] = (4.0 == 2.0);
        s.v[2010] = if s.b[2010] { 1.0 } else { 0.0 };

        let (assign40210_e53176,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (!s.b[2009])) && s.b[2010]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40210_e53176;

        s.b[2011] = (4.0 == 4.0);
        s.v[2011] = if s.b[2011] { 1.0 } else { 0.0 };

        let (assign40230_e53203,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (!s.b[2009])) && (!s.b[2010])) && s.b[2011]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40230_e53203;

        s.b[2012] = (4.0 == 8.0);
        s.v[2012] = if s.b[2012] { 1.0 } else { 0.0 };

        let (assign40250_e53233,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (!s.b[2009])) && (!s.b[2010])) && (!s.b[2011])) && s.b[2012]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40250_e53233;

        let (assign40260_e53249,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40260_e53249;

        let mut assign40270_loop_guard: usize = 0;
        while {
            let assign40270_cond_e53266: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40270_cond_e53266 != 0.0
        } {
            assign40270_loop_guard += 1;
            assert!(assign40270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) {
                s.store_sqrt(726, 726);
            }
            let (assign40270_body1_e53301,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && s.b[2008]) {
        let assign40270_body1_e53299: f64 = (s.v[719] + 1.0);
        (assign40270_body1_e53299,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign40270_body1_e53301;
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) && (!s.b[2008])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1911, 726);
            s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1911, 780);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && s.b[2007]) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) && (!s.b[2007])) {
            s.store_sub(336, 1854, 1852);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[1975])) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1890, 209, -1.0, 338);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.copy_ad(87, 1853);
            s.copy_ad(91, 1854);
            s.store_sub(94, 1854, 1853);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs_mixed_ia(336, 783, (-2.0), A::square(s.ad_value(782)), 1.0);
        }

        s.b[2013] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2013] = if s.b[2013] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign40530_e53755,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40530_e53755;

        let (assign40540_e53766,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40540_e53766;

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2014] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2014] = if s.b[2014] { 1.0 } else { 0.0 };

        s.b[2015] = (2.0 == 1.0);
        s.v[2015] = if s.b[2015] { 1.0 } else { 0.0 };

        let (assign40650_e53897,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && s.b[2015]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40650_e53897;

        s.b[2016] = (2.0 == 2.0);
        s.v[2016] = if s.b[2016] { 1.0 } else { 0.0 };

        let (assign40670_e53918,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (!s.b[2015])) && s.b[2016]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40670_e53918;

        s.b[2017] = (2.0 == 4.0);
        s.v[2017] = if s.b[2017] { 1.0 } else { 0.0 };

        let (assign40690_e53942,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (!s.b[2015])) && (!s.b[2016])) && s.b[2017]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40690_e53942;

        s.b[2018] = (2.0 == 8.0);
        s.v[2018] = if s.b[2018] { 1.0 } else { 0.0 };

        let (assign40710_e53969,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (!s.b[2015])) && (!s.b[2016])) && (!s.b[2017])) && s.b[2018]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40710_e53969;

        let (assign40720_e53982,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40720_e53982;

        let mut assign40730_loop_guard: usize = 0;
        while {
            let assign40730_cond_e53996: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40730_cond_e53996 != 0.0
        } {
            assign40730_loop_guard += 1;
            assert!(assign40730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) {
                s.store_sqrt(726, 726);
            }
            let (assign40730_body1_e54025,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && s.b[2014]) {
        let assign40730_body1_e54023: f64 = (s.v[719] + 1.0);
        (assign40730_body1_e54023,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign40730_body1_e54025;
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) && (!s.b[2014])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2013]) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2013])) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2013])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_add(109, 87, 110);
        }

        s.b[2019] = (((s.v[109] - s.v[1851]) < s.v[1911]) && (s.v[1911] >= 0.0));
        s.v[2019] = if s.b[2019] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
            s.store_add_scaled_inputs3_indices(781, 1911, 1.0, 109, -1.0, 1851, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1911);
            s.store_scalar(724, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
            s.store_scalar(725, 1.0);
        }

        let (assign40890_e54251,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40890_e54251;

        let (assign40900_e54262,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40900_e54262;

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
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

        s.b[2020] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2020] = if s.b[2020] { 1.0 } else { 0.0 };

        s.b[2021] = (4.0 == 1.0);
        s.v[2021] = if s.b[2021] { 1.0 } else { 0.0 };

        let (assign41050_e54445,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && s.b[2021]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41050_e54445;

        s.b[2022] = (4.0 == 2.0);
        s.v[2022] = if s.b[2022] { 1.0 } else { 0.0 };

        let (assign41070_e54466,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (!s.b[2021])) && s.b[2022]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41070_e54466;

        s.b[2023] = (4.0 == 4.0);
        s.v[2023] = if s.b[2023] { 1.0 } else { 0.0 };

        let (assign41090_e54490,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (!s.b[2021])) && (!s.b[2022])) && s.b[2023]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41090_e54490;

        s.b[2024] = (4.0 == 8.0);
        s.v[2024] = if s.b[2024] { 1.0 } else { 0.0 };

        let (assign41110_e54517,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (!s.b[2021])) && (!s.b[2022])) && (!s.b[2023])) && s.b[2024]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41110_e54517;

        let (assign41120_e54530,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41120_e54530;

        let mut assign41130_loop_guard: usize = 0;
        while {
            let assign41130_cond_e54544: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41130_cond_e54544 != 0.0
        } {
            assign41130_loop_guard += 1;
            assert!(assign41130_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) {
                s.store_sqrt(726, 726);
            }
            let (assign41130_body1_e54573,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && s.b[2020]) {
        let assign41130_body1_e54571: f64 = (s.v[719] + 1.0);
        (assign41130_body1_e54571,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41130_body1_e54573;
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) && (!s.b[2020])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1911, 726);
            s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1911, 780);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2019]) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2019])) {
            s.store_sub(336, 109, 1851);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1891, 209, -1.0, 338);
        }

        s.b[2030] = (s.v[1836] > s.v[965]);
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2030]) {
            s.copy_ad(981, 1834);
        }

        s.b[2031] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
            s.store_offset(781, 87, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign41320_e54843,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41320_e54843;

        let (assign41330_e54857,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41330_e54857;

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign41360_e54899,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41360_e54899;

        let mut assign41370_loop_guard: usize = 0;
        while {
            let assign41370_cond_e54914: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && (s.v[719] < s.v[1914])) { 1.0 } else { 0.0 };
            assign41370_cond_e54914 != 0.0
        } {
            assign41370_loop_guard += 1;
            assert!(assign41370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign41370_body2_e54962,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
        let assign41370_body2_e54960: f64 = (s.v[719] + 1.0);
        (assign41370_body2_e54960,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41370_body2_e54962;
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2032] = ((((s.v[1914] == 1.0) || (s.v[1914] == 2.0)) || (s.v[1914] == 4.0)) || (s.v[1914] == 8.0));
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        s.b[2033] = (s.v[1914] == 1.0);
        s.v[2033] = if s.b[2033] { 1.0 } else { 0.0 };

        let (assign41420_e55028,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && s.b[2033]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41420_e55028;

        s.b[2034] = (s.v[1914] == 2.0);
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

        let (assign41440_e55052,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (!s.b[2033])) && s.b[2034]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41440_e55052;

        s.b[2035] = (s.v[1914] == 4.0);
        s.v[2035] = if s.b[2035] { 1.0 } else { 0.0 };

        let (assign41460_e55079,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (!s.b[2033])) && (!s.b[2034])) && s.b[2035]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41460_e55079;

        s.b[2036] = (s.v[1914] == 8.0);
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

        let (assign41480_e55109,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (!s.b[2033])) && (!s.b[2034])) && (!s.b[2035])) && s.b[2036]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41480_e55109;

        let (assign41490_e55125,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41490_e55125;

        let mut assign41500_loop_guard: usize = 0;
        while {
            let assign41500_cond_e55142: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41500_cond_e55142 != 0.0
        } {
            assign41500_loop_guard += 1;
            assert!(assign41500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) {
                s.store_sqrt(726, 726);
            }
            let (assign41500_body1_e55177,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && s.b[2032]) {
        let assign41500_body1_e55175: f64 = (s.v[719] + 1.0);
        (assign41500_body1_e55175,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41500_body1_e55177;
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) && (!s.b[2032])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1914), 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_offset(983, 780, (-0.1));
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2031]) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2031])) {
            s.copy_ad(983, 87);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
            s.store_add_scaled_inputs3_offset_indices(1916, 791, 1.0, 85, (-1.0), 1912, 1.0, (-(s.v[462] - p.p392)));
            s.store_sub(1915, 791, 1916);
        }

        s.b[2037] = ((s.v[1915] > (-s.v[1913])) && (s.v[1913] >= 0.0));
        s.v[2037] = if s.b[2037] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
            s.store_add(781, 1915, 1913);
            s.store_square(722, 781);
            s.store_square(723, 1913);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign41670_e55456,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41670_e55456;

        let (assign41680_e55470,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41680_e55470;

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign41710_e55512,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41710_e55512;

        let mut assign41720_loop_guard: usize = 0;
        while {
            let assign41720_cond_e55527: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && (s.v[719] < s.v[1914])) { 1.0 } else { 0.0 };
            assign41720_cond_e55527 != 0.0
        } {
            assign41720_loop_guard += 1;
            assert!(assign41720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign41720_body2_e55575,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
        let assign41720_body2_e55573: f64 = (s.v[719] + 1.0);
        (assign41720_body2_e55573,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41720_body2_e55575;
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2038] = ((((s.v[1914] == 1.0) || (s.v[1914] == 2.0)) || (s.v[1914] == 4.0)) || (s.v[1914] == 8.0));
        s.v[2038] = if s.b[2038] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2039] = (s.v[1914] == 1.0);
        s.v[2039] = if s.b[2039] { 1.0 } else { 0.0 };

        let (assign41770_e55641,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && s.b[2039]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41770_e55641;

        s.b[2040] = (s.v[1914] == 2.0);
        s.v[2040] = if s.b[2040] { 1.0 } else { 0.0 };

        let (assign41790_e55665,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (!s.b[2039])) && s.b[2040]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41790_e55665;

        s.b[2041] = (s.v[1914] == 4.0);
        s.v[2041] = if s.b[2041] { 1.0 } else { 0.0 };

        let (assign41810_e55692,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (!s.b[2039])) && (!s.b[2040])) && s.b[2041]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41810_e55692;

        s.b[2042] = (s.v[1914] == 8.0);
        s.v[2042] = if s.b[2042] { 1.0 } else { 0.0 };

        let (assign41830_e55722,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (!s.b[2039])) && (!s.b[2040])) && (!s.b[2041])) && s.b[2042]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41830_e55722;

        let (assign41840_e55738,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41840_e55738;

        let mut assign41850_loop_guard: usize = 0;
        while {
            let assign41850_cond_e55755: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41850_cond_e55755 != 0.0
        } {
            assign41850_loop_guard += 1;
            assert!(assign41850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) {
                s.store_sqrt(726, 726);
            }
            let (assign41850_body1_e55790,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && s.b[2038]) {
        let assign41850_body1_e55788: f64 = (s.v[719] + 1.0);
        (assign41850_body1_e55788,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41850_body1_e55790;
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) && (!s.b[2038])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1914), 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1913, 726);
            s.store_div_scaled_product3_indices(334, 1913, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(1915, 1913, -1.0, 780, 1.0);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2037]) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2037])) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2037])) {
            s.store_scalar(334, 1.0);
        }

        let (assign41940_e55946,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign41940_e55946;

        let (assign41950_e55958,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign41950_e55958;

        let mut assign41960_loop_guard: usize = 0;
        while {
            let assign41960_cond_e55971: f64 = if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign41960_cond_e55971 != 0.0
        } {
            assign41960_loop_guard += 1;
            assert!(assign41960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2043] = (s.v[983] >= 0.0);
            s.v[2043] = if s.b[2043] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2043]) {
                s.store_mul_scaled_sqrt_ad_rhs(2028, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2029, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2028), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2043])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1887)));
                s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1887);
                s.store_mul_sqrt_ad_rhs(2028, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2028, 1.0);
                s.store_mul_add_ad_rhs(2029, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            let (assign41960_body10_e56187,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] != 0.0)) {
        let assign41960_body10_e56185: f64 = (150.0 + 1.0);
        (assign41960_body10_e56185,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign41960_body10_e56187;
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(1870, 2028, 1.0, 185, A::sub(s.ad_value(1915), s.ad_value(983)), 1.0);
                s.store_sub(1871, 2029, 185);
                s.store_div_scaled_inputs_indices(1882, 1870, -1.0, 1871, 1.0);
            }
            s.b[2044] = (((s.v[1882]) as f64).abs() < (1e-10 * 100.0));
            s.v[2044] = if s.b[2044] { 1.0 } else { 0.0 };
            let (assign41960_body15_e56267,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) && s.b[2044]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign41960_body15_e56267;
            s.b[2045] = (s.v[1882] > 0.1);
            s.v[2045] = if s.b[2045] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) && (!s.b[2044])) && s.b[2045]) {
                s.store_scalar(1882, 0.1);
            }
            s.b[2046] = (s.v[1882] < (-0.1));
            s.v[2046] = if s.b[2046] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) && (!s.b[2044])) && (!s.b[2045])) && s.b[2046]) {
                s.store_scalar(1882, (-0.1));
            }
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 1882);
            }
            let (assign41960_body21_e56349,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
        let assign41960_body21_e56347: f64 = (s.v[97] + 1.0);
        (assign41960_body21_e56347,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign41960_body21_e56349;
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2026, 1905, 1834, (0.5 * 9662367879.197212), 0.0, 1834);
            s.store_scaled_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2026)), p.p394);
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(2027, 335, 2026);
            s.store_mul(332, 2027, 983);
            s.store_exp_mul_scaled_lhs_indices(334, 2027, -1.0, 2026);
        }

        s.b[2048] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2048] = if s.b[2048] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2048]) {
            s.store_mul_exp_lhs(335, 332, 334);
            s.store_sub(336, 335, 334);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2048])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2049] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2049] = if s.b[2049] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2049]) {
            s.store_div_ln_offset_lhs(2025, 336, 1.0, 2027);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2049])) {
            s.store_div(2025, 336, 2027);
        }

        s.b[2050] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2025])) / s.v[1905]) <= 0.0);
        s.v[2050] = if s.b[2050] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && s.b[2050]) {
            s.store_scalar(981, 0.0);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) && (!s.b[2050])) {
            s.store_sqrt_ad(981, A::div_scaled_inputs2(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2025), (-(2.0 * 1.034943e-10)), s.ad_value(1905), 1.0));
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2030])) {
            if (s.v[981] > s.v[1834]) {
                s.copy_ad(981, 1834);
            } else {
            }
        }

        s.b[2051] = (s.v[981] < s.v[1834]);
        s.v[2051] = if s.b[2051] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2051]) {
            s.store_sub(990, 1834, 981);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2051])) {
            s.store_scalar(990, 0.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_neg_add(1894, 1889, 1890);
        }

        s.b[2052] = (s.v[94] < 0.0);
        s.v[2052] = if s.b[2052] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2052]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(1854, 1853);
            s.store_scalar(248, 0.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2052])) {
            s.store_mul3_affine_lhs(248, 154, 1894, 1.0 / (2.0), 0.0, 94);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2052])) {
            if (s.v[248] < 0.0) {
                s.store_scalar(248, 0.0);
            } else {
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_neg(238, 1891);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 980, 334, 1.0);
            s.store_mul(339, 336, 238);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_ad(341, s.ad_value(251), A::offset(s.ad_value(624), (-1.0)));
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
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

        s.b[2053] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2053] = if s.b[2053] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2053]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2054] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2054] = if s.b[2054] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2053])) && s.b[2054]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2053])) && (!s.b[2054])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[2055] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2055] = if s.b[2055] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2055]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2056] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2056] = if s.b[2056] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2055])) && s.b[2056]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

    }

    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2055])) && (!s.b[2056])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2055])) && (!s.b[2056])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(1886, 255);
            s.copy_ad(989, 349);
        }

        s.b[2057] = (s.v[349] > 1e-6);
        s.v[2057] = if s.b[2057] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {
            s.store_scaled_add(344, 1887, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(1907), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1907, 1.0);
        }

        s.b[2058] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2058] = if s.b[2058] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign42810_e57485,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign42810_e57485;

        let (assign42820_e57498,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42820_e57498;

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2059] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2059] = if s.b[2059] { 1.0 } else { 0.0 };

        s.b[2060] = (2.0 == 1.0);
        s.v[2060] = if s.b[2060] { 1.0 } else { 0.0 };

        let (assign42930_e57647,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && s.b[2060]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42930_e57647;

        s.b[2061] = (2.0 == 2.0);
        s.v[2061] = if s.b[2061] { 1.0 } else { 0.0 };

        let (assign42950_e57670,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (!s.b[2060])) && s.b[2061]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42950_e57670;

        s.b[2062] = (2.0 == 4.0);
        s.v[2062] = if s.b[2062] { 1.0 } else { 0.0 };

        let (assign42970_e57696,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (!s.b[2060])) && (!s.b[2061])) && s.b[2062]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42970_e57696;

        s.b[2063] = (2.0 == 8.0);
        s.v[2063] = if s.b[2063] { 1.0 } else { 0.0 };

        let (assign42990_e57725,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (!s.b[2060])) && (!s.b[2061])) && (!s.b[2062])) && s.b[2063]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42990_e57725;

        let (assign43000_e57740,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43000_e57740;

        let mut assign43010_loop_guard: usize = 0;
        while {
            let assign43010_cond_e57756: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43010_cond_e57756 != 0.0
        } {
            assign43010_loop_guard += 1;
            assert!(assign43010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) {
                s.store_sqrt(726, 726);
            }
            let (assign43010_body1_e57789,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && s.b[2059]) {
        let assign43010_body1_e57787: f64 = (s.v[719] + 1.0);
        (assign43010_body1_e57787,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign43010_body1_e57789;
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) && (!s.b[2059])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2058]) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && (!s.b[2058])) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && (!s.b[2058])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1906), 1.0, s.ad_value(337)));
        }

        s.b[2064] = ((s.v[344] < (s.v[972] + s.v[1910])) && (s.v[1910] >= 0.0));
        s.v[2064] = if s.b[2064] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) {
            s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1910, 1.0, 344, -1.0);
            s.store_square(722, 781);
            s.store_square(723, 1910);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign43180_e58049,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43180_e58049;

        let (assign43190_e58062,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43190_e58062;

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2065] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2065] = if s.b[2065] { 1.0 } else { 0.0 };

        s.b[2066] = (2.0 == 1.0);
        s.v[2066] = if s.b[2066] { 1.0 } else { 0.0 };

        let (assign43300_e58211,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && s.b[2066]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43300_e58211;

        s.b[2067] = (2.0 == 2.0);
        s.v[2067] = if s.b[2067] { 1.0 } else { 0.0 };

        let (assign43320_e58234,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && (!s.b[2066])) && s.b[2067]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43320_e58234;

        s.b[2068] = (2.0 == 4.0);
        s.v[2068] = if s.b[2068] { 1.0 } else { 0.0 };

        let (assign43340_e58260,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && (!s.b[2066])) && (!s.b[2067])) && s.b[2068]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43340_e58260;

        s.b[2069] = (2.0 == 8.0);
        s.v[2069] = if s.b[2069] { 1.0 } else { 0.0 };

        let (assign43360_e58289,) = {
    if ((((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && (!s.b[2066])) && (!s.b[2067])) && (!s.b[2068])) && s.b[2069]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43360_e58289;

        let (assign43370_e58304,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43370_e58304;

        let mut assign43380_loop_guard: usize = 0;
        while {
            let assign43380_cond_e58320: f64 = if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43380_cond_e58320 != 0.0
        } {
            assign43380_loop_guard += 1;
            assert!(assign43380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) {
                s.store_sqrt(726, 726);
            }
            let (assign43380_body1_e58353,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && s.b[2065]) {
        let assign43380_body1_e58351: f64 = (s.v[719] + 1.0);
        (assign43380_body1_e58351,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign43380_body1_e58353;
        }

        if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) && (!s.b[2065])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1910, 726);
            s.store_div_scaled_product3_indices(334, 1910, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(344, 972, 1.0, 1910, 1.0, 780, -1.0);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && s.b[2064]) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && (!s.b[2064])) {
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) && (!s.b[2064])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {
            s.store_div(335, 989, 344);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2057]) {
            s.store_mul(340, 338, 337);
            s.store_div(989, 989, 340);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_sub(335, 791, 1887);
        }

        s.b[2070] = ((s.v[335] < 1.0) && (1.0 >= 0.0));
        s.v[2070] = if s.b[2070] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) {
            s.store_sub_from_scalar(781, 1.0, 335);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign43600_e58679,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43600_e58679;

    }

    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign43610_e58690,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43610_e58690;

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2071] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2071] = if s.b[2071] { 1.0 } else { 0.0 };

        s.b[2072] = (2.0 == 1.0);
        s.v[2072] = if s.b[2072] { 1.0 } else { 0.0 };

        let (assign43720_e58821,) = {
    if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) && s.b[2072]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43720_e58821;

        s.b[2073] = (2.0 == 2.0);
        s.v[2073] = if s.b[2073] { 1.0 } else { 0.0 };

        let (assign43740_e58842,) = {
    if (((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) && (!s.b[2072])) && s.b[2073]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43740_e58842;

        s.b[2074] = (2.0 == 4.0);
        s.v[2074] = if s.b[2074] { 1.0 } else { 0.0 };

        let (assign43760_e58866,) = {
    if ((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) && (!s.b[2072])) && (!s.b[2073])) && s.b[2074]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43760_e58866;

        s.b[2075] = (2.0 == 8.0);
        s.v[2075] = if s.b[2075] { 1.0 } else { 0.0 };

        let (assign43780_e58893,) = {
    if (((((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) && (!s.b[2072])) && (!s.b[2073])) && (!s.b[2074])) && s.b[2075]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43780_e58893;

        let (assign43790_e58906,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43790_e58906;

        let mut assign43800_loop_guard: usize = 0;
        while {
            let assign43800_cond_e58920: f64 = if ((((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43800_cond_e58920 != 0.0
        } {
            assign43800_loop_guard += 1;
            assert!(assign43800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) {
                s.store_sqrt(726, 726);
            }
            let (assign43800_body1_e58949,) = {
    if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && s.b[2071]) {
        let assign43800_body1_e58947: f64 = (s.v[719] + 1.0);
        (assign43800_body1_e58947,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign43800_body1_e58949;
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) && (!s.b[2071])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(335, 1.0, 780);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2070]) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2070])) {
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2070])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_div(251, 335, 965);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p353 - 1.0));
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_mul(342, 339, 251);
            s.store_offset(336, 966, 1e-25);
            s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1885, 989, 170);
            s.store_powf(781, 989, 2.0);
            s.store_scalar(782, ((0.1) as f64).powf(2.0));
            s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div(335, 335, 170);
            s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_div(985, 254, 338);
            s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));
            s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1885);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add(135, 986, 987);
            s.copy_ad(790, 349);
        }

        s.b[2076] = (p.p283 != 0.0);
        s.v[2076] = if s.b[2076] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2076]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_mixed_ia(336, 783, (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1853), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2077] = (s.v[336] < 0.0);
        s.v[2077] = if s.b[2077] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2076]) && s.b[2077]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2076]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1439, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 1853, 1.0, 340, 1.0, 1438, -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1439), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2076])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2078] = (p.p287 != 0.0);
        s.v[2078] = if s.b[2078] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2078]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1439);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2078])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2079] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2079] = if s.b[2079] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2079]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[2080] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2080] = if s.b[2080] { 1.0 } else { 0.0 };

        s.b[2081] = (p.p296 > 0.0);
        s.v[2081] = if s.b[2081] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && s.b[2081]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && s.b[2081]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && s.b[2081]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && s.b[2081]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && s.b[2081]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && (!s.b[2081])) {
            s.copy_ad(341, 647);
        }

        s.b[2082] = (s.v[793] >= 0.0);
        s.v[2082] = if s.b[2082] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && s.b[2082]) {
            s.copy_ad(369, 793);
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && (!s.b[2082])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2083] = (s.v[369] < (20.0 * 1e-12));
        s.v[2083] = if s.b[2083] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && s.b[2083]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) && (!s.b[2083])) {
            s.store_powf_offset_input(335, 369, 1e-12, p.p297);
        }

    }

    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2080]) {
            s.store_powf_offset_input(343, 369, 1e-12, p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && (!s.b[2080])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1443] && (s.b[1445] && (!s.b[1444]))) {
            s.store_add_scaled_inputs4_indices(131, 1864, (-0.5), 1865, (-0.5), 1867, (-0.5), 1869, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1892), 1.0, s.ad_value(1893), 1.0, s.ad_value(1895), 1.0, s.ad_value(1896), 1.0), s.ad_value(1866)), 1868, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1892, 1893, (-0.5));
            s.store_neg(238, 1892);
            s.copy_ad(255, 1886);
        }

        s.b[2084] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2084] = if s.b[2084] { 1.0 } else { 0.0 };

        let (assign44890_e60540,) = {
    if ((s.b[1443] && (s.b[1445] && (!s.b[1444]))) && s.b[2084]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign44890_e60540;

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.copy_ad(2091, 960);
            s.store_scale(2133, 964, 1.6021918e-19);
            s.store_scale(2114, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_div_from_scalar(2136, (2.0 * 1.034943e-10), 2133);
            s.store_div(2130, 964, 622);
            s.store_div_from_scalar_offset_input(2129, 1.0, 2130, 1.0);
            s.store_div_ad_rhs(2134, 2114, A::square(s.ad_value(185)));
            s.store_div_from_scalar(2135, 2.0, 2134);
            s.store_scalar(2143, 2.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p.p343 * p.p340)) }));
        }

        s.b[2172] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) {
            s.store_sub_from_scalar(781, (2.0 + 0.1), 508);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign45060_e60775,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45060_e60775;

        let (assign45070_e60788,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45070_e60788;

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2173] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        s.b[2174] = (2.0 == 1.0);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        let (assign45180_e60937,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) && s.b[2174]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45180_e60937;

        s.b[2175] = (2.0 == 2.0);
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        let (assign45200_e60960,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) && (!s.b[2174])) && s.b[2175]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45200_e60960;

        s.b[2176] = (2.0 == 4.0);
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        let (assign45220_e60986,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) && (!s.b[2174])) && (!s.b[2175])) && s.b[2176]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45220_e60986;

        s.b[2177] = (2.0 == 8.0);
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        let (assign45240_e61015,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) && (!s.b[2174])) && (!s.b[2175])) && (!s.b[2176])) && s.b[2177]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45240_e61015;

        let (assign45250_e61030,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45250_e61030;

        let mut assign45260_loop_guard: usize = 0;
        while {
            let assign45260_cond_e61046: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45260_cond_e61046 != 0.0
        } {
            assign45260_loop_guard += 1;
            assert!(assign45260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) {
                s.store_sqrt(726, 726);
            }
            let (assign45260_body1_e61079,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && s.b[2173]) {
        let assign45260_body1_e61077: f64 = (s.v[719] + 1.0);
        (assign45260_body1_e61077,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign45260_body1_e61079;
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) && (!s.b[2173])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(508, (2.0 + 0.1), 780);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2172]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2172])) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2172])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_scalar(2089, 0.0);
            s.store_scalar(2090, 0.0);
            s.store_scalar(2098, 0.0);
            s.store_scalar(2099, 0.0);
            s.store_scalar(2171, 0.0);
            s.store_scalar(2146, 0.0);
            s.copy_ad(2117, 1435);
            s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, (-1.0), (-s.v[160]));
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2096, 781, (-0.5), 782, (-0.5), 0.3);
            s.store_add_scaled_inputs3_offset_indices(781, 2096, 1.0, 2117, -1.0, 2091, 1.0, (-0.01));
            s.store_scaled_sub(782, 2117, 2091, (4.0 * 0.01));
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2096, 2117, 1.0, 2091, (-1.0), 781, 0.5, 782, 0.5);
            s.copy_ad(2089, 2096);
            s.store_scalar(2087, 0.0);
            s.copy_ad(2092, 2087);
            s.store_mul_sub_rhs(2094, 2129, 1438, 2091);
            s.store_mul_neg_rhs(2150, 2129, 2091);
        }

        s.b[2178] = (((-s.v[2094]) < 0.001) && (0.001 >= 0.0));
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2094)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign45660_e61670,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45660_e61670;

        let (assign45670_e61683,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45670_e61683;

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2179] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        s.b[2180] = (2.0 == 1.0);
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        let (assign45780_e61832,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) && s.b[2180]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45780_e61832;

        s.b[2181] = (2.0 == 2.0);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign45800_e61855,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) && (!s.b[2180])) && s.b[2181]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45800_e61855;

        s.b[2182] = (2.0 == 4.0);
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        let (assign45820_e61881,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) && (!s.b[2180])) && (!s.b[2181])) && s.b[2182]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45820_e61881;

        s.b[2183] = (2.0 == 8.0);
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        let (assign45840_e61910,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) && (!s.b[2180])) && (!s.b[2181])) && (!s.b[2182])) && s.b[2183]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45840_e61910;

        let (assign45850_e61925,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45850_e61925;

        let mut assign45860_loop_guard: usize = 0;
        while {
            let assign45860_cond_e61941: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45860_cond_e61941 != 0.0
        } {
            assign45860_loop_guard += 1;
            assert!(assign45860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) {
                s.store_sqrt(726, 726);
            }
            let (assign45860_body1_e61974,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && s.b[2179]) {
        let assign45860_body1_e61972: f64 = (s.v[719] + 1.0);
        (assign45860_body1_e61972,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign45860_body1_e61974;
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) && (!s.b[2179])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2178]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2178])) {
            s.store_neg(335, 2094);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_sqrt_mul(2085, 2136, 335);
        }

        s.b[2184] = (((-s.v[2150]) < 0.001) && (0.001 >= 0.0));
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2150)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign46020_e62222,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46020_e62222;

        let (assign46030_e62235,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46030_e62235;

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2185] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        s.b[2186] = (2.0 == 1.0);
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        let (assign46140_e62384,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) && s.b[2186]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46140_e62384;

        s.b[2187] = (2.0 == 2.0);
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        let (assign46160_e62407,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) && (!s.b[2186])) && s.b[2187]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46160_e62407;

        s.b[2188] = (2.0 == 4.0);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        let (assign46180_e62433,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) && (!s.b[2186])) && (!s.b[2187])) && s.b[2188]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46180_e62433;

        s.b[2189] = (2.0 == 8.0);
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        let (assign46200_e62462,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) && (!s.b[2186])) && (!s.b[2187])) && (!s.b[2188])) && s.b[2189]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46200_e62462;

        let (assign46210_e62477,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46210_e62477;

        let mut assign46220_loop_guard: usize = 0;
        while {
            let assign46220_cond_e62493: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46220_cond_e62493 != 0.0
        } {
            assign46220_loop_guard += 1;
            assert!(assign46220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) {
                s.store_sqrt(726, 726);
            }
            let (assign46220_body1_e62526,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && s.b[2185]) {
        let assign46220_body1_e62524: f64 = (s.v[719] + 1.0);
        (assign46220_body1_e62524,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign46220_body1_e62526;
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) && (!s.b[2185])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2184]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2184])) {
            s.store_neg(335, 2150);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_sqrt_mul(2151, 2136, 335);
        }

        s.b[2190] = (p.p345 != 0.0);
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2190]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2190]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2190]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2190]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2190]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.copy_ad(2131, 965);
            s.store_sub(2148, 965, 2085);
            s.store_sub(2149, 965, 2151);
        }

        s.b[2191] = ((s.v[2148] < (p.p344 + (p.p344 * 0.1))) && ((p.p344 * 0.1) >= 0.0));
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) {
            s.store_sub_from_scalar(781, (p.p344 + (p.p344 * 0.1)), 2148);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign46560_e63086,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46560_e63086;

        let (assign46570_e63099,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46570_e63099;

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2192] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        s.b[2193] = (1.0 == 1.0);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        let (assign46660_e63218,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) && s.b[2193]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46660_e63218;

        s.b[2194] = (1.0 == 2.0);
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        let (assign46680_e63241,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) && (!s.b[2193])) && s.b[2194]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46680_e63241;

        s.b[2195] = (1.0 == 4.0);
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        let (assign46700_e63267,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) && (!s.b[2193])) && (!s.b[2194])) && s.b[2195]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46700_e63267;

        s.b[2196] = (1.0 == 8.0);
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        let (assign46720_e63296,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) && (!s.b[2193])) && (!s.b[2194])) && (!s.b[2195])) && s.b[2196]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46720_e63296;

        let (assign46730_e63311,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46730_e63311;

    }

    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign46740_loop_guard: usize = 0;
        while {
            let assign46740_cond_e63327: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46740_cond_e63327 != 0.0
        } {
            assign46740_loop_guard += 1;
            assert!(assign46740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) {
                s.store_sqrt(726, 726);
            }
            let (assign46740_body1_e63360,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && s.b[2192]) {
        let assign46740_body1_e63358: f64 = (s.v[719] + 1.0);
        (assign46740_body1_e63358,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign46740_body1_e63360;
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) && (!s.b[2192])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2148, (p.p344 + (p.p344 * 0.1)), 780);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2191]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2191])) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2191])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2197] = ((s.v[2149] < (p.p344 * 0.1)) && ((p.p344 * 0.1) >= 0.0));
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) {
            s.store_sub_from_scalar(781, (p.p344 * 0.1), 2149);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign46890_e63607,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46890_e63607;

        let (assign46900_e63620,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46900_e63620;

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2198] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        s.b[2199] = (1.0 == 1.0);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        let (assign46990_e63739,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) && s.b[2199]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46990_e63739;

        s.b[2200] = (1.0 == 2.0);
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        let (assign47010_e63762,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) && (!s.b[2199])) && s.b[2200]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47010_e63762;

        s.b[2201] = (1.0 == 4.0);
        s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };

        let (assign47030_e63788,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) && (!s.b[2199])) && (!s.b[2200])) && s.b[2201]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47030_e63788;

        s.b[2202] = (1.0 == 8.0);
        s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };

        let (assign47050_e63817,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) && (!s.b[2199])) && (!s.b[2200])) && (!s.b[2201])) && s.b[2202]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47050_e63817;

        let (assign47060_e63832,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47060_e63832;

        let mut assign47070_loop_guard: usize = 0;
        while {
            let assign47070_cond_e63848: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47070_cond_e63848 != 0.0
        } {
            assign47070_loop_guard += 1;
            assert!(assign47070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) {
                s.store_sqrt(726, 726);
            }
            let (assign47070_body1_e63881,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && s.b[2198]) {
        let assign47070_body1_e63879: f64 = (s.v[719] + 1.0);
        (assign47070_body1_e63879,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47070_body1_e63881;
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) && (!s.b[2198])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2149, (p.p344 * 0.1), 780);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2197]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2197])) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2197])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_offset_scaled_div(2152, 2148, 2149, (p.p394 - p.p395), p.p395);
        }

        let (assign47170_e64053,) = {
    if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign47170_e64053;

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_mul(2140, 2129, 2130);
        }

        let (assign47190_e64077,) = {
    if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign47190_e64077;

    }

    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign47200_loop_guard: usize = 0;
        while {
            let assign47200_cond_e64089: f64 = if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47200_cond_e64089 != 0.0
        } {
            assign47200_loop_guard += 1;
            assert!(assign47200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
                s.store_mul_sub_ad_rhs(2094, 2129, A::add_scaled_product(s.ad_value(2117), 1.0, s.ad_value(2130), s.ad_value(2092), 1.0), s.ad_value(2091));
                s.store_sub(335, 2092, 2094);
            }
            s.b[2203] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign47200_body8_e64216,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47200_body8_e64216;
            let (assign47200_body9_e64229,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body9_e64229;
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2204] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };
            s.b[2205] = (2.0 == 1.0);
            s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };
            let (assign47200_body20_e64378,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) && s.b[2205]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body20_e64378;
            s.b[2206] = (2.0 == 2.0);
            s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };
            let (assign47200_body22_e64401,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) && (!s.b[2205])) && s.b[2206]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body22_e64401;
            s.b[2207] = (2.0 == 4.0);
            s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };
            let (assign47200_body24_e64427,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) && (!s.b[2205])) && (!s.b[2206])) && s.b[2207]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body24_e64427;
            s.b[2208] = (2.0 == 8.0);
            s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };
            let (assign47200_body26_e64456,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) && (!s.b[2205])) && (!s.b[2206])) && (!s.b[2207])) && s.b[2208]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body26_e64456;
            let (assign47200_body27_e64471,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47200_body27_e64471;
            let mut assign47200_body28_loop_guard: usize = 0;
            while {
                let assign47200_body28_cond_e64487: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47200_body28_cond_e64487 != 0.0
            } {
                assign47200_body28_loop_guard += 1;
                assert!(assign47200_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47200_body28_body1_e64520,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && s.b[2204]) {
        let assign47200_body28_body1_e64518: f64 = (s.v[719] + 1.0);
        (assign47200_body28_body1_e64518,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47200_body28_body1_e64520;
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) && (!s.b[2204])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2203]) {
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2203])) {
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2203])) {
                s.store_scalar(336, 1.0);
            }
            if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
                s.store_sqrt_mul(2085, 2136, 335);
            }
            s.b[2209] = ((s.v[2085] > (s.v[2131] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) {
                s.store_offset_sub(781, 2085, 2131, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign47200_body44_e64765,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47200_body44_e64765;
            let (assign47200_body45_e64778,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body45_e64778;
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2210] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };
            s.b[2211] = (2.0 == 1.0);
            s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };
            let (assign47200_body56_e64927,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) && s.b[2211]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body56_e64927;
            s.b[2212] = (2.0 == 2.0);
            s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };
            let (assign47200_body58_e64950,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) && (!s.b[2211])) && s.b[2212]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body58_e64950;
            s.b[2213] = (2.0 == 4.0);
            s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };
            let (assign47200_body60_e64976,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) && (!s.b[2211])) && (!s.b[2212])) && s.b[2213]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body60_e64976;
            s.b[2214] = (2.0 == 8.0);
            s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };
            let (assign47200_body62_e65005,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) && (!s.b[2211])) && (!s.b[2212])) && (!s.b[2213])) && s.b[2214]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body62_e65005;
            let (assign47200_body63_e65020,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47200_body63_e65020;
            let mut assign47200_body64_loop_guard: usize = 0;
            while {
                let assign47200_body64_cond_e65036: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47200_body64_cond_e65036 != 0.0
            } {
                assign47200_body64_loop_guard += 1;
                assert!(assign47200_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47200_body64_body1_e65069,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && s.b[2210]) {
        let assign47200_body64_body1_e65067: f64 = (s.v[719] + 1.0);
        (assign47200_body64_body1_e65067,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47200_body64_body1_e65069;
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) && (!s.b[2210])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_offset_lhs(2085, 2131, (-1e-12), 780);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2209]) {
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2209])) {
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2209])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
                s.store_mul(337, 336, 337);
                s.store_add_div_rhs_mixed_ai(2137, 2089, A::add_scaled_square_product(s.ad_value(2131), 1.0, s.ad_value(2085), A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2131), 2.0), 1.0), 2136);
                s.store_scalar(2138, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2139, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2131), s.ad_value(2085)), s.ad_value(337), (-1.0)), 1.0, 2140);
            }
            s.b[2215] = ((s.v[2137] > (s.v[2087] - p.p406)) && (p.p406 >= 0.0));
            s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) {
                s.store_offset_sub(781, 2137, 2087, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign47200_body83_e65370,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47200_body83_e65370;
            let (assign47200_body84_e65383,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body84_e65383;
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) {
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
            s.b[2216] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };
            s.b[2217] = (4.0 == 1.0);
            s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };
            let (assign47200_body99_e65592,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) && s.b[2217]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body99_e65592;
            s.b[2218] = (4.0 == 2.0);
            s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };
            let (assign47200_body101_e65615,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) && (!s.b[2217])) && s.b[2218]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body101_e65615;
            s.b[2219] = (4.0 == 4.0);
            s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };
            let (assign47200_body103_e65641,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) && (!s.b[2217])) && (!s.b[2218])) && s.b[2219]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body103_e65641;
            s.b[2220] = (4.0 == 8.0);
            s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };
            let (assign47200_body105_e65670,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) && (!s.b[2217])) && (!s.b[2218])) && (!s.b[2219])) && s.b[2220]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47200_body105_e65670;
            let (assign47200_body106_e65685,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47200_body106_e65685;
            let mut assign47200_body107_loop_guard: usize = 0;
            while {
                let assign47200_body107_cond_e65701: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47200_body107_cond_e65701 != 0.0
            } {
                assign47200_body107_loop_guard += 1;
                assert!(assign47200_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47200_body107_body1_e65734,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && s.b[2216]) {
        let assign47200_body107_body1_e65732: f64 = (s.v[719] + 1.0);
        (assign47200_body107_body1_e65732,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47200_body107_body1_e65734;
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) && (!s.b[2216])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_offset_lhs(2137, 2087, (-p.p406), 780);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2215]) {
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2215])) {
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2215])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
                s.store_mul(2138, 2138, 334);
                s.store_mul(2139, 2139, 334);
                s.store_mul_sub_rhs(339, 154, 2089, 2092);
                s.store_exp(340, 339);
                s.store_sub_offset_lhs(344, 340, (-1.0), 339);
            }
            s.b[2221] = (s.v[339] >= 1e-7);
            s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };
            let (assign47200_body122_e65955,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2221]) {
        let assign47200_body122_e65953: f64 = (-1.0);
        (assign47200_body122_e65953,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign47200_body122_e65955;
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2221]) {
                s.store_mul_scaled_sqrt_rhs(2098, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2098, 1.0);
                s.store_mul_offset_rhs(2125, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2127, 345, 1.0, 340);
            }
            s.b[2222] = (s.v[339] < (-1e-7));
            s.v[2222] = if s.b[2222] { 1.0 } else { 0.0 };
            let (assign47200_body128_e66048,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2221])) && s.b[2222]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign47200_body128_e66048;
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2221])) && s.b[2222]) {
                s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2089), 1.0, s.ad_value(2117), p.p398));
                s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2092), 1.0, s.ad_value(2117), p.p398));
                s.store_mul_sqrt_ad_rhs(2098, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2098, 1.0);
                s.store_mul_add_ad_rhs(2125, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2127, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2223] = (s.v[339] > 0.0);
            s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2221])) && (!s.b[2222])) && s.b[2223]) {
                s.store_offset_scaled(2163, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2164, 2163);
                s.store_mul_ad_affine_product_lhs(2098, s.ad_value(209), A::sqrt(s.ad_value(2163)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2125, 209, s.ad_value(154), A::add(s.ad_value(2164), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2164), 1.0)), -1.0, 0.0);
                s.store_neg(2127, 2125);
            }
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2221])) && (!s.b[2222])) && (!s.b[2223])) {
                s.store_offset_scaled(2163, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2164, 2163);
                s.store_mul_ad_affine_product_lhs(2098, s.ad_value(209), A::sqrt(s.ad_value(2163)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2125, 209, s.ad_value(154), A::add(s.ad_value(2164), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2164), 1.0)), -1.0, 0.0);
                s.store_neg(2127, 2125);
            }
            let (assign47200_body146_e66464,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] != 0.0)) {
        let assign47200_body146_e66462: f64 = (150.0 + 1.0);
        (assign47200_body146_e66462,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47200_body146_e66464;
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2100, 2098, 1.0, 185, A::sub(s.ad_value(85), s.ad_value(2089)), 1.0);
                s.store_sub(2101, 2125, 185);
                s.copy_ad(2102, 2127);
                s.store_sub(2103, 2092, 2137);
                s.store_neg(2104, 2138);
                s.store_sub_from_scalar(2105, 1.0, 2139);
                s.store_add_scaled_products_indices(2106, 2101, 2105, 1.0, 2102, 2104, (-1.0));
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] == 0.0)) {
                if (s.v[2106] > 0.0) {
                    s.store_div_from_scalar_offset_input(2107, 1.0, 2106, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2107, 1.0, 2106, (-1e-25));
                }
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] == 0.0)) {
                s.copy_ad(2108, 2105);
                s.store_neg(2109, 2102);
                s.store_neg(2110, 2104);
                s.copy_ad(2111, 2101);
                s.store_mul_add_scaled_products_indices_rhs(2112, 2107, 2108, 2100, -1.0, 2109, 2103, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(2113, 2107, 2110, 2100, -1.0, 2111, 2103, -1.0);
                s.store_abs(335, 2112);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2113]) as f64).abs()) {
                    s.store_abs(335, 2113);
                } else {
                }
            }
            s.b[2224] = (s.v[335] > 0.1);
            s.v[2224] = if s.b[2224] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] == 0.0)) && s.b[2224]) {
                s.store_mul_div_from_scalar_rhs(2112, 2112, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2113, 2113, 0.1, 335);
            }
            s.b[2225] = (s.v[335] < 1e-10);
            s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };
            let (assign47200_body167_e66811,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] == 0.0)) && s.b[2225]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign47200_body167_e66811;
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (s.v[79] == 0.0)) {
                s.store_add(2089, 2089, 2112);
                s.store_add(2092, 2092, 2113);
            }
            let (assign47200_body170_e66856,) = {
    if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
        let assign47200_body170_e66854: f64 = (s.v[97] + 1.0);
        (assign47200_body170_e66854,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47200_body170_e66856;
        }

    }

    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_mul_sub_rhs(339, 154, 2089, 2092);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[339] > 0.0) {
                s.store_mul_scaled_sqrt_rhs(2122, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2122, 209, 344);
            }
        }

        s.b[2227] = (1.0 == 1.0);
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = (((s.v[2089] - s.v[2087]) < p.p403) && (p.p403 >= 0.0));
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2089), s.ad_value(2087)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign47330_e67040,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47330_e67040;

        let (assign47340_e67055,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47340_e67055;

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) {
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

        s.b[2229] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        s.b[2230] = (6.0 == 1.0);
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        let (assign47530_e67358,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) && s.b[2230]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47530_e67358;

        s.b[2231] = (6.0 == 2.0);
        s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };

        let (assign47550_e67383,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) && (!s.b[2230])) && s.b[2231]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47550_e67383;

        s.b[2232] = (6.0 == 4.0);
        s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };

        let (assign47570_e67411,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) && (!s.b[2230])) && (!s.b[2231])) && s.b[2232]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47570_e67411;

        s.b[2233] = (6.0 == 8.0);
        s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };

        let (assign47590_e67442,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) && (!s.b[2230])) && (!s.b[2231])) && (!s.b[2232])) && s.b[2233]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47590_e67442;

        let (assign47600_e67459,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47600_e67459;

        let mut assign47610_loop_guard: usize = 0;
        while {
            let assign47610_cond_e67477: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47610_cond_e67477 != 0.0
        } {
            assign47610_loop_guard += 1;
            assert!(assign47610_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) {
                s.store_sqrt(726, 726);
            }
            let (assign47610_body1_e67514,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && s.b[2229]) {
        let assign47610_body1_e67512: f64 = (s.v[719] + 1.0);
        (assign47610_body1_e67512,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47610_body1_e67514;
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) && (!s.b[2229])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && s.b[2228]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) && (!s.b[2228])) {
            s.store_sub(336, 2089, 2087);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2227]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2118, 209, -1.0, 338);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2227])) {
            s.copy_ad(2118, 2122);
        }

        s.b[2234] = (1.0 == 1.0);
        s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
            s.copy_ad(2159, 85);
            s.store_offset_mul(338, 2135, 2159, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
            s.store_offset_add_ad(2160, s.ad_value(2159), A::mul_sub_from_scalar_rhs(s.ad_value(2134), 1.0, s.ad_value(337)), p.p397);
            s.copy_ad(2156, 2160);
        }

        let (assign47790_e67825,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign47790_e67825;

        let (assign47800_e67838,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign47800_e67838;

        let mut assign47810_loop_guard: usize = 0;
        while {
            let assign47810_cond_e67852: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47810_cond_e67852 != 0.0
        } {
            assign47810_loop_guard += 1;
            assert!(assign47810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
                s.store_mul_neg_lhs(335, 154, 2156);
                s.store_exp(336, 335);
                s.store_sqrt_ad(338, A::div_scaled_inputs(s.ad_value(2114), 2.0, s.ad_value(154), 1.0));
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2157, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2235] = (s.v[335] > 0.0);
            s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && s.b[2235]) {
                s.store_neg(2157, 2157);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2157, 1.0);
                s.store_mul_sub_from_scalar_rhs(2158, 345, 1.0, 336);
            }
            let (assign47810_body9_e68010,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && (s.v[79] != 0.0)) {
        let assign47810_body9_e68008: f64 = (150.0 + 1.0);
        (assign47810_body9_e68008,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47810_body9_e68010;
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && (s.v[79] == 0.0)) {
                s.store_add_scaled_offset_product_rhs_mixed_iia(2100, 2157, 1.0, 185, A::sub(s.ad_value(2159), s.ad_value(2156)), p.p397, -1.0);
                s.store_add(2101, 185, 2158);
                s.store_div_scaled_inputs_indices(2112, 2100, -1.0, 2101, 1.0);
            }
            s.b[2236] = (((s.v[2112]) as f64).abs() < 1e-10);
            s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };
            let (assign47810_body14_e68094,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && (s.v[79] == 0.0)) && s.b[2236]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign47810_body14_e68094;
            s.b[2237] = (s.v[2112] > 0.1);
            s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && (s.v[79] == 0.0)) && (!s.b[2236])) && s.b[2237]) {
                s.store_scalar(2112, 0.1);
            }
            s.b[2238] = (s.v[2112] < (-0.1));
            s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && (s.v[79] == 0.0)) && (!s.b[2236])) && (!s.b[2237])) && s.b[2238]) {
                s.store_scalar(2112, (-0.1));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && (s.v[79] == 0.0)) {
                s.store_add(2156, 2156, 2112);
            }
            let (assign47810_body20_e68180,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
        let assign47810_body20_e68178: f64 = (s.v[97] + 1.0);
        (assign47810_body20_e68178,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47810_body20_e68180;
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) {
            s.copy_ad(2161, 2156);
            s.store_sqrt_square_offset(782, 2161, ((4.0 * p.p404) * p.p404));
            s.store_offset_scaled_div(334, 2161, 782, 0.5, 0.5);
            s.store_scaled_add(2162, 2161, 782, 0.5);
        }

        s.b[2239] = (s.v[2162] < 0.0);
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2234]) && s.b[2239]) {
            s.store_scalar(2162, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) {
            s.store_offset_mul(338, 2135, 85, 1.0);
            s.store_offset(339, 2135, 1.0);
        }

        s.b[2240] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign47970_e68431,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47970_e68431;

        let (assign47980_e68447,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47980_e68447;

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2241] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        s.b[2242] = (2.0 == 1.0);
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign48090_e68623,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) && s.b[2242]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48090_e68623;

        s.b[2243] = (2.0 == 2.0);
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        let (assign48110_e68649,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) && (!s.b[2242])) && s.b[2243]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48110_e68649;

        s.b[2244] = (2.0 == 4.0);
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        let (assign48130_e68678,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) && (!s.b[2242])) && (!s.b[2243])) && s.b[2244]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48130_e68678;

        s.b[2245] = (2.0 == 8.0);
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        let (assign48150_e68710,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) && (!s.b[2242])) && (!s.b[2243])) && (!s.b[2244])) && s.b[2245]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48150_e68710;

        let (assign48160_e68728,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48160_e68728;

        let mut assign48170_loop_guard: usize = 0;
        while {
            let assign48170_cond_e68747: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48170_cond_e68747 != 0.0
        } {
            assign48170_loop_guard += 1;
            assert!(assign48170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) {
                s.store_sqrt(726, 726);
            }
            let (assign48170_body1_e68786,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && s.b[2241]) {
        let assign48170_body1_e68784: f64 = (s.v[719] + 1.0);
        (assign48170_body1_e68784,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48170_body1_e68786;
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) && (!s.b[2241])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2240]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && (!s.b[2240])) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && (!s.b[2240])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(2134), 1.0, s.ad_value(337)));
        }

        s.b[2246] = ((s.v[344] < p.p404) && (p.p404 >= 0.0));
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) {
            s.store_sub_from_scalar(781, p.p404, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p404 * p.p404));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign48340_e69094,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48340_e69094;

        let (assign48350_e69110,) = {
    if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48350_e69110;

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2247] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        s.b[2248] = (2.0 == 1.0);
        s.v[2248] = if s.b[2248] { 1.0 } else { 0.0 };

        let (assign48460_e69286,) = {
    if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) && s.b[2248]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48460_e69286;

        s.b[2249] = (2.0 == 2.0);
        s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };

        let (assign48480_e69312,) = {
    if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) && (!s.b[2248])) && s.b[2249]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48480_e69312;

        s.b[2250] = (2.0 == 4.0);
        s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };

        let (assign48500_e69341,) = {
    if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) && (!s.b[2248])) && (!s.b[2249])) && s.b[2250]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48500_e69341;

        s.b[2251] = (2.0 == 8.0);
        s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };

        let (assign48520_e69373,) = {
    if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) && (!s.b[2248])) && (!s.b[2249])) && (!s.b[2250])) && s.b[2251]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48520_e69373;

        let (assign48530_e69391,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48530_e69391;

        let mut assign48540_loop_guard: usize = 0;
        while {
            let assign48540_cond_e69410: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48540_cond_e69410 != 0.0
        } {
            assign48540_loop_guard += 1;
            assert!(assign48540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) {
                s.store_sqrt(726, 726);
            }
            let (assign48540_body1_e69449,) = {
    if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && s.b[2247]) {
        let assign48540_body1_e69447: f64 = (s.v[719] + 1.0);
        (assign48540_body1_e69447,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48540_body1_e69449;
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) && (!s.b[2247])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p404);
            s.store_div_scaled_product_indices(334, 725, 726, p.p404, 770, 1.0);
            s.store_sub_from_scalar(2162, p.p404, 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && s.b[2246]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2234])) && (!s.b[2246])) {
            s.copy_ad(2162, 344);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.copy_ad(349, 790);
            s.store_div(335, 790, 2162);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), s.ad_value(658));
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)));
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_div(348, 790, 338);
            s.copy_ad(790, 348);
        }

        s.b[2252] = (s.v[790] < 0.0);
        s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2252]) {
            s.copy_ad(2090, 2089);
            s.copy_ad(2095, 2094);
            s.copy_ad(2093, 2092);
            s.copy_ad(2123, 2122);
            s.copy_ad(2119, 2118);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            s.copy_ad(2088, 790);
            s.store_add_scaled_inputs3_offset_indices(781, 2089, 1.0, 2088, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 2089, 2088, (4.0 * 0.01));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2097, 2089, 1.0, 2088, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 2097, 1.0, 2117, -1.0, 2091, 1.0, (-0.01));
            s.store_scaled_sub(782, 2117, 2091, (4.0 * 0.01));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2097, 2117, 1.0, 2091, (-1.0), 781, 0.5, 782, 0.5);
            s.copy_ad(2093, 2088);
            s.copy_ad(2090, 2097);
        }

        let (assign48910_e70074,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign48910_e70074;

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            s.store_mul(2141, 2129, 2130);
        }

        let (assign48930_e70104,) = {
    if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.v[98] = assign48930_e70104;

    }
}
