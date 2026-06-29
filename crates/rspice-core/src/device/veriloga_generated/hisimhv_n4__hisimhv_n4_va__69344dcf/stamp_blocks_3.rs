#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign48900_loop_guard: usize = 0;
        while {
            let assign48900_cond_e70099: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            assign48900_cond_e70099 != 0.0
        } {
            assign48900_loop_guard += 1;
            assert!(assign48900_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_mul_sub_ad_rhs(2091, 2125, A::add_scaled_product(s.ad_value(2113), 1.0, s.ad_value(2126), s.ad_value(2089), 1.0), s.ad_value(2087));
                s.store_sub(335, 2089, 2091);
            }
            s.b[2249] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign48900_body8_e70250,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48900_body8_e70250;
            let (assign48900_body9_e70266,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body9_e70266;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2250] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };
            s.b[2251] = (2.0 == 1.0);
            s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };
            let (assign48900_body20_e70442,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && s.b[2251]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body20_e70442;
            s.b[2252] = (2.0 == 2.0);
            s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };
            let (assign48900_body22_e70468,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && s.b[2252]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body22_e70468;
            s.b[2253] = (2.0 == 4.0);
            s.v[2253] = if s.b[2253] { 1.0 } else { 0.0 };
            let (assign48900_body24_e70497,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && s.b[2253]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body24_e70497;
            s.b[2254] = (2.0 == 8.0);
            s.v[2254] = if s.b[2254] { 1.0 } else { 0.0 };
            let (assign48900_body26_e70529,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && (!s.b[2253])) && s.b[2254]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body26_e70529;
            let (assign48900_body27_e70547,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48900_body27_e70547;
            let mut assign48900_body28_loop_guard: usize = 0;
            while {
                let assign48900_body28_cond_e70566: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48900_body28_cond_e70566 != 0.0
            } {
                assign48900_body28_loop_guard += 1;
                assert!(assign48900_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
                    s.store_sqrt(726, 726);
                }
                let (assign48900_body28_body1_e70605,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
        let assign48900_body28_body1_e70603: f64 = (s.v[719] + 1.0);
        (assign48900_body28_body1_e70603,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign48900_body28_body1_e70605;
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && (!s.b[2250])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {
                s.store_scalar(336, 1.0);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_sqrt_mul(2082, 2132, 335);
            }
            s.b[2255] = ((s.v[2082] > (s.v[2127] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2255] = if s.b[2255] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
                s.store_offset_sub(781, 2082, 2127, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign48900_body44_e70895,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48900_body44_e70895;
            let (assign48900_body45_e70911,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body45_e70911;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2256] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2256] = if s.b[2256] { 1.0 } else { 0.0 };
            s.b[2257] = (2.0 == 1.0);
            s.v[2257] = if s.b[2257] { 1.0 } else { 0.0 };
            let (assign48900_body56_e71087,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && s.b[2257]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body56_e71087;
            s.b[2258] = (2.0 == 2.0);
            s.v[2258] = if s.b[2258] { 1.0 } else { 0.0 };
            let (assign48900_body58_e71113,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body58_e71113;
            s.b[2259] = (2.0 == 4.0);
            s.v[2259] = if s.b[2259] { 1.0 } else { 0.0 };
            let (assign48900_body60_e71142,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && s.b[2259]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body60_e71142;
            s.b[2260] = (2.0 == 8.0);
            s.v[2260] = if s.b[2260] { 1.0 } else { 0.0 };
            let (assign48900_body62_e71174,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && (!s.b[2259])) && s.b[2260]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body62_e71174;
            let (assign48900_body63_e71192,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48900_body63_e71192;
            let mut assign48900_body64_loop_guard: usize = 0;
            while {
                let assign48900_body64_cond_e71211: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48900_body64_cond_e71211 != 0.0
            } {
                assign48900_body64_loop_guard += 1;
                assert!(assign48900_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
                    s.store_sqrt(726, 726);
                }
                let (assign48900_body64_body1_e71250,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
        let assign48900_body64_body1_e71248: f64 = (s.v[719] + 1.0);
        (assign48900_body64_body1_e71248,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign48900_body64_body1_e71250;
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && (!s.b[2256])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_offset_lhs(2082, 2127, (-1e-12), 780);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_mul(337, 336, 337);
                s.store_add_div_rhs_mixed_ai(2133, 2086, A::add_scaled_square_product(s.ad_value(2127), 1.0, s.ad_value(2082), A::sub_scaled_inputs(s.ad_value(2082), 1.0, s.ad_value(2127), 2.0), 1.0), 2132);
                s.store_scalar(2134, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2135, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2127), s.ad_value(2082)), s.ad_value(337), (-1.0)), 1.0, 2137);
            }
            s.b[2261] = ((s.v[2133] > (s.v[2084] - p.p406)) && (p.p406 >= 0.0));
            s.v[2261] = if s.b[2261] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
                s.store_offset_sub(781, 2133, 2084, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign48900_body83_e71605,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48900_body83_e71605;
            let (assign48900_body84_e71621,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body84_e71621;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
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
            s.b[2262] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2262] = if s.b[2262] { 1.0 } else { 0.0 };
            s.b[2263] = (4.0 == 1.0);
            s.v[2263] = if s.b[2263] { 1.0 } else { 0.0 };
            let (assign48900_body99_e71869,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && s.b[2263]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body99_e71869;
            s.b[2264] = (4.0 == 2.0);
            s.v[2264] = if s.b[2264] { 1.0 } else { 0.0 };
            let (assign48900_body101_e71895,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && s.b[2264]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body101_e71895;
            s.b[2265] = (4.0 == 4.0);
            s.v[2265] = if s.b[2265] { 1.0 } else { 0.0 };
            let (assign48900_body103_e71924,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && s.b[2265]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body103_e71924;
            s.b[2266] = (4.0 == 8.0);
            s.v[2266] = if s.b[2266] { 1.0 } else { 0.0 };
            let (assign48900_body105_e71956,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && (!s.b[2265])) && s.b[2266]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48900_body105_e71956;
            let (assign48900_body106_e71974,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48900_body106_e71974;
            let mut assign48900_body107_loop_guard: usize = 0;
            while {
                let assign48900_body107_cond_e71993: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48900_body107_cond_e71993 != 0.0
            } {
                assign48900_body107_loop_guard += 1;
                assert!(assign48900_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
                    s.store_sqrt(726, 726);
                }
                let (assign48900_body107_body1_e72032,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
        let assign48900_body107_body1_e72030: f64 = (s.v[719] + 1.0);
        (assign48900_body107_body1_e72030,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign48900_body107_body1_e72032;
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && (!s.b[2262])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_offset_lhs(2133, 2084, (-p.p406), 780);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_mul(2134, 2134, 334);
                s.store_mul(2135, 2135, 334);
                s.store_mul_sub_rhs(339, 154, 2086, 2089);
                s.store_exp(340, 339);
                s.store_sub_offset_lhs(344, 340, (-1.0), 339);
            }
            s.b[2267] = (s.v[339] >= 1e-7);
            s.v[2267] = if s.b[2267] { 1.0 } else { 0.0 };
            let (assign48900_body122_e72295,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {
        let assign48900_body122_e72293: f64 = (-1.0);
        (assign48900_body122_e72293,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign48900_body122_e72295;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {
                s.store_mul_scaled_sqrt_rhs(2095, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);
                s.store_mul_offset_rhs(2122, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2124, 345, 1.0, 340);
            }
            s.b[2268] = (s.v[339] < (-1e-7));
            s.v[2268] = if s.b[2268] { 1.0 } else { 0.0 };
            let (assign48900_body128_e72403,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign48900_body128_e72403;
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {
                s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2086), 1.0, s.ad_value(2113), p.p398));
                s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2089), 1.0, s.ad_value(2113), p.p398));
                s.store_mul_sqrt_ad_rhs(2095, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);
                s.store_mul_add_ad_rhs(2122, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2124, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2269] = (s.v[339] > 0.0);
            s.v[2269] = if s.b[2269] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && s.b[2269]) {
                s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2160, 2159);
                s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);
                s.store_neg(2124, 2122);
            }
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && (!s.b[2269])) {
                s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2160, 2159);
                s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);
                s.store_neg(2124, 2122);
            }
            let (assign48900_body146_e72870,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] != 0.0)) {
        let assign48900_body146_e72868: f64 = (150.0 + 1.0);
        (assign48900_body146_e72868,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign48900_body146_e72870;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2096, 2095, 1.0, 185, 85, 2086, 1.0);
                s.store_sub(2097, 2122, 185);
                s.copy_ad(2098, 2124);
                s.store_sub(2099, 2089, 2133);
                s.store_neg(2100, 2134);
                s.store_sub_from_scalar(2101, 1.0, 2135);
                s.store_add_scaled_products_indices(2102, 2097, 2101, 1.0, 2098, 2100, (-1.0));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                if (s.v[2102] > 0.0) {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, (-1e-25));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.copy_ad(2104, 2101);
                s.store_neg(2105, 2098);
                s.store_neg(2106, 2100);
                s.copy_ad(2107, 2097);
                s.store_mul_add_scaled_products_indices_rhs(2108, 2103, 2104, 2096, -1.0, 2105, 2099, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(2109, 2103, 2106, 2096, -1.0, 2107, 2099, -1.0);
                s.store_abs(335, 2108);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2109]) as f64).abs()) {
                    s.store_abs(335, 2109);
                } else {
                }
            }
            s.b[2270] = (s.v[335] > 0.1);
            s.v[2270] = if s.b[2270] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2270]) {
                s.store_mul_div_from_scalar_rhs(2108, 2108, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2109, 2109, 0.1, 335);
            }
            s.b[2271] = (s.v[335] < 1e-10);
            s.v[2271] = if s.b[2271] { 1.0 } else { 0.0 };
            let (assign48900_body167_e73274,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2271]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign48900_body167_e73274;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.store_add(2086, 2086, 2108);
                s.store_add(2089, 2089, 2109);
            }
            let (assign48900_body170_e73328,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        let assign48900_body170_e73326: f64 = (s.v[98] + 1.0);
        (assign48900_body170_e73326,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign48900_body170_e73328;
        }

    }

    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_mul_sub_rhs(339, 154, 2086, 2089);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[2086] > s.v[2089]) {
                s.store_mul_scaled_sqrt_rhs(2119, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2119, 209, 344);
            }
        }

        s.b[2273] = (1.0 == 1.0);
        s.v[2273] = if s.b[2273] { 1.0 } else { 0.0 };

        s.b[2274] = (((s.v[2086] - s.v[2084]) < p.p403) && (p.p403 >= 0.0));
        s.v[2274] = if s.b[2274] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2086), s.ad_value(2084)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign49030_e73542,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49030_e73542;

        let (assign49040_e73560,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49040_e73560;

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
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

        s.b[2275] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2275] = if s.b[2275] { 1.0 } else { 0.0 };

        s.b[2276] = (6.0 == 1.0);
        s.v[2276] = if s.b[2276] { 1.0 } else { 0.0 };

        let (assign49230_e73914,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && s.b[2276]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49230_e73914;

        s.b[2277] = (6.0 == 2.0);
        s.v[2277] = if s.b[2277] { 1.0 } else { 0.0 };

        let (assign49250_e73942,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && s.b[2277]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49250_e73942;

        s.b[2278] = (6.0 == 4.0);
        s.v[2278] = if s.b[2278] { 1.0 } else { 0.0 };

        let (assign49270_e73973,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && s.b[2278]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49270_e73973;

        s.b[2279] = (6.0 == 8.0);
        s.v[2279] = if s.b[2279] { 1.0 } else { 0.0 };

        let (assign49290_e74007,) = {
    if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && (!s.b[2278])) && s.b[2279]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49290_e74007;

        let (assign49300_e74027,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49300_e74027;

        let mut assign49310_loop_guard: usize = 0;
        while {
            let assign49310_cond_e74048: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49310_cond_e74048 != 0.0
        } {
            assign49310_loop_guard += 1;
            assert!(assign49310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {
                s.store_sqrt(726, 726);
            }
            let (assign49310_body1_e74091,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {
        let assign49310_body1_e74089: f64 = (s.v[719] + 1.0);
        (assign49310_body1_e74089,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign49310_body1_e74091;
        }

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && (!s.b[2275])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && (!s.b[2274])) {
            s.store_sub(336, 2086, 2084);
            s.store_scalar(334, 1.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2115, 209, -1.0, 338);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2273])) {
            s.copy_ad(2115, 2119);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.copy_ad(87, 2085);
            s.copy_ad(91, 2086);
            s.store_sub(94, 2086, 2085);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[2280] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2280] = if s.b[2280] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign49580_e74632,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49580_e74632;

        let (assign49590_e74645,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49590_e74645;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2281] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2281] = if s.b[2281] { 1.0 } else { 0.0 };

        s.b[2282] = (2.0 == 1.0);
        s.v[2282] = if s.b[2282] { 1.0 } else { 0.0 };

        let (assign49700_e74794,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && s.b[2282]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49700_e74794;

        s.b[2283] = (2.0 == 2.0);
        s.v[2283] = if s.b[2283] { 1.0 } else { 0.0 };

        let (assign49720_e74817,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && s.b[2283]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49720_e74817;

        s.b[2284] = (2.0 == 4.0);
        s.v[2284] = if s.b[2284] { 1.0 } else { 0.0 };

        let (assign49740_e74843,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && s.b[2284]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49740_e74843;

        s.b[2285] = (2.0 == 8.0);
        s.v[2285] = if s.b[2285] { 1.0 } else { 0.0 };

        let (assign49760_e74872,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && (!s.b[2284])) && s.b[2285]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49760_e74872;

        let (assign49770_e74887,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49770_e74887;

        let mut assign49780_loop_guard: usize = 0;
        while {
            let assign49780_cond_e74903: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49780_cond_e74903 != 0.0
        } {
            assign49780_loop_guard += 1;
            assert!(assign49780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {
                s.store_sqrt(726, 726);
            }
            let (assign49780_body1_e74936,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {
        let assign49780_body1_e74934: f64 = (s.v[719] + 1.0);
        (assign49780_body1_e74934,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign49780_body1_e74936;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && (!s.b[2281])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2280])) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2280])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_add(109, 87, 110);
        }

        s.b[2286] = (((s.v[109] - s.v[2083]) < p.p403) && (p.p403 >= 0.0));
        s.v[2286] = if s.b[2286] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(109), s.ad_value(2083)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
            s.store_scalar(725, 1.0);
        }

        let (assign49940_e75192,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49940_e75192;

        let (assign49950_e75205,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49950_e75205;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
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

        s.b[2287] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2287] = if s.b[2287] { 1.0 } else { 0.0 };

        s.b[2288] = (6.0 == 1.0);
        s.v[2288] = if s.b[2288] { 1.0 } else { 0.0 };

        let (assign50140_e75474,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && s.b[2288]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50140_e75474;

        s.b[2289] = (6.0 == 2.0);
        s.v[2289] = if s.b[2289] { 1.0 } else { 0.0 };

        let (assign50160_e75497,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && s.b[2289]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50160_e75497;

        s.b[2290] = (6.0 == 4.0);
        s.v[2290] = if s.b[2290] { 1.0 } else { 0.0 };

        let (assign50180_e75523,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && s.b[2290]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50180_e75523;

        s.b[2291] = (6.0 == 8.0);
        s.v[2291] = if s.b[2291] { 1.0 } else { 0.0 };

        let (assign50200_e75552,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && (!s.b[2290])) && s.b[2291]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50200_e75552;

        let (assign50210_e75567,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50210_e75567;

        let mut assign50220_loop_guard: usize = 0;
        while {
            let assign50220_cond_e75583: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50220_cond_e75583 != 0.0
        } {
            assign50220_loop_guard += 1;
            assert!(assign50220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {
                s.store_sqrt(726, 726);
            }
            let (assign50220_body1_e75616,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {
        let assign50220_body1_e75614: f64 = (s.v[719] + 1.0);
        (assign50220_body1_e75614,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign50220_body1_e75616;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && (!s.b[2287])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2286])) {
            s.store_sub(336, 109, 2083);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2116, 209, -1.0, 338);
            s.store_sqrt_offset_ad(782, A::mul_scaled_lhs(A::add(s.ad_value(2115), s.ad_value(2114)), 1.0, A::add(s.ad_value(2115), s.ad_value(2114))), ((4.0 * (1e-12 * 1e-6)) * (1e-12 * 1e-6)));
            s.store_scaled_offset_ad(335, A::div_scaled_inputs2(s.ad_value(2115), -1.0, s.ad_value(2114), -1.0, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_add_scaled_inputs3_indices(2117, 2115, (-0.5), 2114, (-0.5), 782, 0.5);
        }

        s.b[2292] = (s.v[2117] < 0.0);
        s.v[2292] = if s.b[2292] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2292]) {
            s.store_scalar(2117, 0.0);
            s.store_scalar(335, 0.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_neg(2117, 2117);
            s.store_mul3_affine_lhs(248, 154, 2117, (-1.0 / (2.0)), 0.0, 94);
            s.store_neg(238, 2116);
            s.copy_ad(170, 162);
            s.copy_ad(790, 349);
        }

        s.b[2293] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2293] = if s.b[2293] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2294] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2294] = if s.b[2294] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign50530_e76118,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50530_e76118;

        let (assign50540_e76133,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50540_e76133;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2295] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2295] = if s.b[2295] { 1.0 } else { 0.0 };

        s.b[2296] = (2.0 == 1.0);
        s.v[2296] = if s.b[2296] { 1.0 } else { 0.0 };

        let (assign50650_e76300,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && s.b[2296]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50650_e76300;

        s.b[2297] = (2.0 == 2.0);
        s.v[2297] = if s.b[2297] { 1.0 } else { 0.0 };

        let (assign50670_e76325,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && s.b[2297]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50670_e76325;

        s.b[2298] = (2.0 == 4.0);
        s.v[2298] = if s.b[2298] { 1.0 } else { 0.0 };

        let (assign50690_e76353,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50690_e76353;

        s.b[2299] = (2.0 == 8.0);
        s.v[2299] = if s.b[2299] { 1.0 } else { 0.0 };

        let (assign50710_e76384,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) && s.b[2299]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50710_e76384;

        let (assign50720_e76401,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50720_e76401;

        let mut assign50730_loop_guard: usize = 0;
        while {
            let assign50730_cond_e76419: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50730_cond_e76419 != 0.0
        } {
            assign50730_loop_guard += 1;
            assert!(assign50730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {
                s.store_sqrt(726, 726);
            }
            let (assign50730_body1_e76456,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && s.b[2295]) {
        let assign50730_body1_e76454: f64 = (s.v[719] + 1.0);
        (assign50730_body1_e76454,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign50730_body1_e76456;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) && (!s.b[2295])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && (!s.b[2294])) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && (!s.b[2294])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {
            s.store_sub(342, 91, 2113);
        }

        s.b[2300] = ((s.v[342] < (0.2 + ((-s.v[2113]) + 0.8))) && (((-s.v[2113]) + 0.8) >= 0.0));
        s.v[2300] = if s.b[2300] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
            s.store_sub_offset_ad_lhs(781, A::sub_from_scalar(0.8, s.ad_value(2113)), 0.2, 342);
            s.store_square(722, 781);
            s.store_square_ad(723, A::sub_from_scalar(0.8, s.ad_value(2113)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign50890_e76764,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50890_e76764;

        let (assign50900_e76780,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50900_e76780;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2301] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2301] = if s.b[2301] { 1.0 } else { 0.0 };

        s.b[2302] = (1.0 == 1.0);
        s.v[2302] = if s.b[2302] { 1.0 } else { 0.0 };

        let (assign50990_e76920,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && s.b[2302]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50990_e76920;

        s.b[2303] = (1.0 == 2.0);
        s.v[2303] = if s.b[2303] { 1.0 } else { 0.0 };

        let (assign51010_e76946,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && s.b[2303]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51010_e76946;

        s.b[2304] = (1.0 == 4.0);
        s.v[2304] = if s.b[2304] { 1.0 } else { 0.0 };

        let (assign51030_e76975,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && (!s.b[2303])) && s.b[2304]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51030_e76975;

        s.b[2305] = (1.0 == 8.0);
        s.v[2305] = if s.b[2305] { 1.0 } else { 0.0 };

        let (assign51050_e77007,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (!s.b[2302])) && (!s.b[2303])) && (!s.b[2304])) && s.b[2305]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51050_e77007;

        let (assign51060_e77025,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign51060_e77025;

        let mut assign51070_loop_guard: usize = 0;
        while {
            let assign51070_cond_e77044: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51070_cond_e77044 != 0.0
        } {
            assign51070_loop_guard += 1;
            assert!(assign51070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {
                s.store_sqrt(726, 726);
            }
            let (assign51070_body1_e77083,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && s.b[2301]) {
        let assign51070_body1_e77081: f64 = (s.v[719] + 1.0);
        (assign51070_body1_e77081,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign51070_body1_e77083;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) && (!s.b[2301])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul_ad_lhs(780, A::mul_sub_from_scalar_rhs(s.ad_value(781), 0.8, s.ad_value(2113)), 726);
            s.store_div_scaled_product_left_ad(334, A::mul_sub_from_scalar_lhs(0.8, s.ad_value(2113), s.ad_value(725)), 726, 1.0, 770, 1.0);
            s.store_sub_offset_ad_lhs(342, A::sub_from_scalar(0.8, s.ad_value(2113)), 0.2, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2300]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2300])) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2300])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {
            s.store_mul(343, 2132, 342);
            s.store_sqrt(171, 343);
            s.store_div_from_scalar(334, 1.0, 171);
            s.store_mul(335, 238, 334);
            s.store_scale(336, 335, s.v[509]);
            s.store_scale(337, 334, s.v[509]);
            s.store_add_scaled_product_indices(339, 336, 1.0, 508, 2129, 1.0);
            s.store_div_from_scalar(335, 1.0, 339);
            s.store_scale(338, 335, 1.034943e-10);
            s.store_scalar(335, (1.0 - s.v[507]));
            s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 109, s.v[507], 335, 91, 1.0);
        }

        s.b[2306] = ((s.v[168] > (((s.v[109] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2306] = if s.b[2306] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 109, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign51330_e77570,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign51330_e77570;

        let (assign51340_e77586,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51340_e77586;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2307] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2307] = if s.b[2307] { 1.0 } else { 0.0 };

        s.b[2308] = (2.0 == 1.0);
        s.v[2308] = if s.b[2308] { 1.0 } else { 0.0 };

        let (assign51450_e77762,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && s.b[2308]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51450_e77762;

        s.b[2309] = (2.0 == 2.0);
        s.v[2309] = if s.b[2309] { 1.0 } else { 0.0 };

        let (assign51470_e77788,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && s.b[2309]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51470_e77788;

        s.b[2310] = (2.0 == 4.0);
        s.v[2310] = if s.b[2310] { 1.0 } else { 0.0 };

        let (assign51490_e77817,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && (!s.b[2309])) && s.b[2310]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51490_e77817;

        s.b[2311] = (2.0 == 8.0);
        s.v[2311] = if s.b[2311] { 1.0 } else { 0.0 };

        let (assign51510_e77849,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (!s.b[2308])) && (!s.b[2309])) && (!s.b[2310])) && s.b[2311]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51510_e77849;

        let (assign51520_e77867,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign51520_e77867;

        let mut assign51530_loop_guard: usize = 0;
        while {
            let assign51530_cond_e77886: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51530_cond_e77886 != 0.0
        } {
            assign51530_loop_guard += 1;
            assert!(assign51530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {
                s.store_sqrt(726, 726);
            }
            let (assign51530_body1_e77925,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && s.b[2307]) {
        let assign51530_body1_e77923: f64 = (s.v[719] + 1.0);
        (assign51530_body1_e77923,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign51530_body1_e77925;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) && (!s.b[2307])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 109, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && s.b[2306]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2306])) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) && (!s.b[2306])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2293])) {
            s.store_sub(340, 168, 91);
            s.store_mul(337, 154, 238);
            s.store_div_from_scalar(335, 1.0, 337);
            s.store_mul(339, 248, 335);
            s.store_scale(344, 2129, 9662367879.197212);
            s.store_scalar(335, 100000.0);
            s.store_div_from_scalar(336, 1.0, 162);
            s.store_mul_ad_lhs(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);
            s.store_mul(341, 345, 338);
            s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));
            s.store_mul3_lhs(342, 345, 338, 338);
            s.store_sqrt_square_add(343, 341, 342);
            s.store_scaled_sub(169, 343, 341, 0.5);
            s.copy_ad(335, 169);
            s.store_mul(169, 208, 335);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_scale(169, 169, s.v[619]);
            s.store_sub(170, 170, 169);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 980, 334, 1.0);
            s.store_mul(339, 336, 238);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 251, p.p160);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(251), s.ad_value(624));
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);
        }

    }

    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2312] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2312] = if s.b[2312] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2312]) {
            s.copy_ad(336, 335);
        }

        s.b[2313] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2313] = if s.b[2313] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2312])) && s.b[2313]) {
            s.store_square(336, 335);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2312])) && (!s.b[2313])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p178);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset(338, 336, 1.0);
        }

        s.b[2314] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2314] = if s.b[2314] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2314]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2315] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2315] = if s.b[2315] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2314])) && s.b[2315]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2314])) && (!s.b[2315])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 338, ((-1.0) / p.p178));
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(2112, 255);
            s.store_scalar(2320, 0.0);
            s.store_scalar(2151, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(2143, 0.0);
            s.store_scalar(2318, 0.0);
            s.store_add_scaled_inputs3_offset_indices(2140, 1436, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));
        }

        s.b[2322] = (0.0 == 0.0);
        s.v[2322] = if s.b[2322] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2322]) {
            s.store_offset(2141, 2140, (-p.p393));
        }

        s.b[2323] = (0.0 == 1.0);
        s.v[2323] = if s.b[2323] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2322])) && s.b[2323]) {
            s.store_offset(2141, 1436, (((-s.v[160])) + ((-p.p393))));
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2322])) && (!s.b[2323])) {
            s.store_offset(2141, 85, (-p.p393));
        }

        s.b[2324] = (((s.v[2144]) as f64).abs() <= 0.0);
        s.v[2324] = if s.b[2324] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2324]) {
            s.store_scalar(2149, 0.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.copy_ad(983, 87);
            s.store_scale(2166, 2113, p.p399);
            s.store_scalar(2321, ((s.v[160] + p.p393) - 3.0));
        }

        s.b[2325] = (1.0 == 1.0);
        s.v[2325] = if s.b[2325] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2325]) {
            s.store_scale(2166, 2113, p.p399);
            s.store_offset(983, 2166, (-1.0));
            s.copy_ad(2320, 2321);
            s.copy_ad(2142, 2321);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2325])) {
            s.store_offset_scaled(2166, 2113, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2320, 2141);
            s.copy_ad(2142, 2141);
        }

        let (assign52410_e79309,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign52410_e79309;

        let (assign52420_e79323,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign52420_e79323;

        let mut assign52430_loop_guard: usize = 0;
        while {
            let assign52430_cond_e79338: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign52430_cond_e79338 != 0.0
        } {
            assign52430_loop_guard += 1;
            assert!(assign52430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2326] = (s.v[983] >= 0.0);
            s.v[2326] = if s.b[2326] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2326]) {
                s.store_mul_scaled_sqrt_ad_rhs(2318, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(2121, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 2318, 1.0);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2326])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2166)));
                s.store_exp_mul(338, 154, 2166);
                s.store_mul_sqrt_ad_rhs(2318, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2318, 1.0);
                s.store_mul_add_ad_rhs(2121, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            let (assign52430_body10_e79572,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] != 0.0)) {
        let assign52430_body10_e79570: f64 = (150.0 + 1.0);
        (assign52430_body10_e79570,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign52430_body10_e79572;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2096, 2318, 1.0, 185, 2320, 983, 1.0);
                s.store_sub(2097, 2121, 185);
                s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);
            }
            s.b[2327] = (((s.v[2108]) as f64).abs() < (1e-10 * 100.0));
            s.v[2327] = if s.b[2327] { 1.0 } else { 0.0 };
            let (assign52430_body15_e79660,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && s.b[2327]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign52430_body15_e79660;
            s.b[2328] = (s.v[2108] > 0.1);
            s.v[2328] = if s.b[2328] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2327])) && s.b[2328]) {
                s.store_scalar(2108, 0.1);
            }
            s.b[2329] = (s.v[2108] < (-0.1));
            s.v[2329] = if s.b[2329] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2327])) && (!s.b[2328])) && s.b[2329]) {
                s.store_scalar(2108, (-0.1));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2108);
            }
            let (assign52430_body21_e79750,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        let assign52430_body21_e79748: f64 = (s.v[97] + 1.0);
        (assign52430_body21_e79748,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign52430_body21_e79750;
        }

        s.b[2331] = (1.0 == 1.0);
        s.v[2331] = if s.b[2331] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2331]) {
            s.copy_ad(2167, 983);
        }

        s.b[2332] = ((s.v[983] < (s.v[2167] + 0.2)) && (0.2 >= 0.0));
        s.v[2332] = if s.b[2332] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
            s.store_sub_offset_lhs(781, 2167, 0.2, 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign52530_e79903,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52530_e79903;

        let (assign52540_e79922,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52540_e79922;

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2333] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2333] = if s.b[2333] { 1.0 } else { 0.0 };

        s.b[2334] = (2.0 == 1.0);
        s.v[2334] = if s.b[2334] { 1.0 } else { 0.0 };

        let (assign52650_e80125,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && s.b[2334]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52650_e80125;

        s.b[2335] = (2.0 == 2.0);
        s.v[2335] = if s.b[2335] { 1.0 } else { 0.0 };

        let (assign52670_e80154,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && s.b[2335]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52670_e80154;

        s.b[2336] = (2.0 == 4.0);
        s.v[2336] = if s.b[2336] { 1.0 } else { 0.0 };

        let (assign52690_e80186,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && (!s.b[2335])) && s.b[2336]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52690_e80186;

        s.b[2337] = (2.0 == 8.0);
        s.v[2337] = if s.b[2337] { 1.0 } else { 0.0 };

        let (assign52710_e80221,) = {
    if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (!s.b[2334])) && (!s.b[2335])) && (!s.b[2336])) && s.b[2337]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52710_e80221;

        let (assign52720_e80242,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52720_e80242;

        let mut assign52730_loop_guard: usize = 0;
        while {
            let assign52730_cond_e80264: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign52730_cond_e80264 != 0.0
        } {
            assign52730_loop_guard += 1;
            assert!(assign52730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {
                s.store_sqrt(726, 726);
            }
            let (assign52730_body1_e80309,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && s.b[2333]) {
        let assign52730_body1_e80307: f64 = (s.v[719] + 1.0);
        (assign52730_body1_e80307,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign52730_body1_e80309;
        }

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) && (!s.b[2333])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_offset_lhs(983, 2167, 0.2, 780);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && s.b[2332]) {
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && (!s.b[2332])) {
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2331])) && (!s.b[2332])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.copy_ad(2149, 983);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_scalar(2138, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2338] = ((s.v[2149] > (-s.v[2138])) && (s.v[2138] >= 0.0));
        s.v[2338] = if s.b[2338] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
            s.store_add(781, 2149, 2138);
            s.store_square(722, 781);
        }

    }

    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
            s.store_square(723, 2138);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign52900_e80639,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52900_e80639;

        let (assign52910_e80655,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52910_e80655;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign52940_e80703,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52940_e80703;

        let mut assign52950_loop_guard: usize = 0;
        while {
            let assign52950_cond_e80720: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && (s.v[719] < s.v[2139])) { 1.0 } else { 0.0 };
            assign52950_cond_e80720 != 0.0
        } {
            assign52950_loop_guard += 1;
            assert!(assign52950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign52950_body2_e80774,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        let assign52950_body2_e80772: f64 = (s.v[719] + 1.0);
        (assign52950_body2_e80772,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign52950_body2_e80774;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2339] = ((((s.v[2139] == 1.0) || (s.v[2139] == 2.0)) || (s.v[2139] == 4.0)) || (s.v[2139] == 8.0));
        s.v[2339] = if s.b[2339] { 1.0 } else { 0.0 };

        s.b[2340] = (s.v[2139] == 1.0);
        s.v[2340] = if s.b[2340] { 1.0 } else { 0.0 };

        let (assign53000_e80846,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && s.b[2340]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53000_e80846;

        s.b[2341] = (s.v[2139] == 2.0);
        s.v[2341] = if s.b[2341] { 1.0 } else { 0.0 };

        let (assign53020_e80872,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && s.b[2341]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53020_e80872;

        s.b[2342] = (s.v[2139] == 4.0);
        s.v[2342] = if s.b[2342] { 1.0 } else { 0.0 };

        let (assign53040_e80901,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && (!s.b[2341])) && s.b[2342]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53040_e80901;

        s.b[2343] = (s.v[2139] == 8.0);
        s.v[2343] = if s.b[2343] { 1.0 } else { 0.0 };

        let (assign53060_e80933,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (!s.b[2340])) && (!s.b[2341])) && (!s.b[2342])) && s.b[2343]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53060_e80933;

        let (assign53070_e80951,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign53070_e80951;

        let mut assign53080_loop_guard: usize = 0;
        while {
            let assign53080_cond_e80970: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53080_cond_e80970 != 0.0
        } {
            assign53080_loop_guard += 1;
            assert!(assign53080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {
                s.store_sqrt(726, 726);
            }
            let (assign53080_body1_e81009,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && s.b[2339]) {
        let assign53080_body1_e81007: f64 = (s.v[719] + 1.0);
        (assign53080_body1_e81007,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign53080_body1_e81009;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) && (!s.b[2339])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2139), 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2138, 726);
            s.store_div_scaled_product3_indices(334, 2138, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2138, -1.0, 780, 1.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2338]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2338])) {
            s.copy_ad(983, 2149);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2316, 2129, 2144, (0.5 * 9662367879.197212), 0.0, 2144);
            s.store_mul_sqrt_ad_rhs(334, 2148, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2316)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2344] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2344] = if s.b[2344] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2344]) {
            s.store_div_ln_lhs(2317, 335, 2316);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2344])) {
            s.store_mul3_ad_middle(2317, A::square(s.ad_value(2148)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_mul(332, 2317, 983);
        }

        s.b[2345] = (s.v[332] > 500.0);
        s.v[2345] = if s.b[2345] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2345]) {
            s.store_sub(2161, 983, 2316);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2317, -1.0, 2316);
        }

        s.b[2346] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2346] = if s.b[2346] { 1.0 } else { 0.0 };

        s.b[2347] = (s.v[332] >= 500.0);
        s.v[2347] = if s.b[2347] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && s.b[2347]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign53340_loop_guard: usize = 0;
        while {
            let assign53340_cond_e81480: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign53340_cond_e81480 != 0.0
        } {
            assign53340_loop_guard += 1;
            assert!(assign53340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) && (!s.b[2347])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2346]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && (!s.b[2346])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2348] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2348] = if s.b[2348] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && s.b[2348]) {
            s.store_div_ln_offset_lhs(2161, 336, 1.0, 2317);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2345])) && (!s.b[2348])) {
            s.store_div(2161, 336, 2317);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_sub(336, 983, 2161);
        }

        s.b[2349] = (0.0 == 0.0);
        s.v[2349] = if s.b[2349] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2349]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2143, A::sqrt(A::mul_scaled_lhs(s.ad_value(2132), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2143, 2132, 336);
            }
        }

        s.b[2350] = (s.v[336] < 0.0);
        s.v[2350] = if s.b[2350] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2349])) && s.b[2350]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2143, A::sqrt(A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2349])) && (!s.b[2350])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2143, A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_sub(990, 2144, 2143);
        }

        s.b[2351] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2351] = if s.b[2351] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign53590_e82003,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign53590_e82003;

        let (assign53600_e82019,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53600_e82019;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2352] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2352] = if s.b[2352] { 1.0 } else { 0.0 };

        s.b[2353] = (2.0 == 1.0);
        s.v[2353] = if s.b[2353] { 1.0 } else { 0.0 };

        let (assign53710_e82195,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && s.b[2353]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53710_e82195;

        s.b[2354] = (2.0 == 2.0);
        s.v[2354] = if s.b[2354] { 1.0 } else { 0.0 };

        let (assign53730_e82221,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && s.b[2354]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53730_e82221;

        s.b[2355] = (2.0 == 4.0);
        s.v[2355] = if s.b[2355] { 1.0 } else { 0.0 };

        let (assign53750_e82250,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && (!s.b[2354])) && s.b[2355]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53750_e82250;

        s.b[2356] = (2.0 == 8.0);
        s.v[2356] = if s.b[2356] { 1.0 } else { 0.0 };

        let (assign53770_e82282,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (!s.b[2353])) && (!s.b[2354])) && (!s.b[2355])) && s.b[2356]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53770_e82282;

        let (assign53780_e82300,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign53780_e82300;

        let mut assign53790_loop_guard: usize = 0;
        while {
            let assign53790_cond_e82319: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53790_cond_e82319 != 0.0
        } {
            assign53790_loop_guard += 1;
            assert!(assign53790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {
                s.store_sqrt(726, 726);
            }
            let (assign53790_body1_e82358,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && s.b[2352]) {
        let assign53790_body1_e82356: f64 = (s.v[719] + 1.0);
        (assign53790_body1_e82356,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign53790_body1_e82358;
        }

    }

    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) && (!s.b[2352])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2351]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2351])) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2351])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2357] = (1.0 == 1.0);
        s.v[2357] = if s.b[2357] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2357]) {
            s.copy_ad(2151, 990);
        }

        s.b[2358] = (2.0 == 1.0);
        s.v[2358] = if s.b[2358] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2358]) {
            s.store_scale(2166, 2113, p.p399);
            s.store_offset(983, 2166, (-1.0));
            s.copy_ad(2320, 2321);
            s.copy_ad(2142, 2321);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2358])) {
            s.store_offset_scaled(2166, 2113, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2320, 2141);
            s.copy_ad(2142, 2141);
        }

        let (assign53990_e82694,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign53990_e82694;

        let (assign54000_e82708,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign54000_e82708;

        let mut assign54010_loop_guard: usize = 0;
        while {
            let assign54010_cond_e82723: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign54010_cond_e82723 != 0.0
        } {
            assign54010_loop_guard += 1;
            assert!(assign54010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2359] = (s.v[983] >= 0.0);
            s.v[2359] = if s.b[2359] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2359]) {
                s.store_mul_scaled_sqrt_ad_rhs(2318, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(2121, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 2318, 1.0);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2359])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2166)));
                s.store_exp_mul(338, 154, 2166);
                s.store_mul_sqrt_ad_rhs(2318, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2318, 1.0);
                s.store_mul_add_ad_rhs(2121, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            let (assign54010_body10_e82957,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] != 0.0)) {
        let assign54010_body10_e82955: f64 = (150.0 + 1.0);
        (assign54010_body10_e82955,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign54010_body10_e82957;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2096, 2318, 1.0, 185, 2320, 983, 1.0);
                s.store_sub(2097, 2121, 185);
                s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);
            }
            s.b[2360] = (((s.v[2108]) as f64).abs() < (1e-10 * 100.0));
            s.v[2360] = if s.b[2360] { 1.0 } else { 0.0 };
            let (assign54010_body15_e83045,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && s.b[2360]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign54010_body15_e83045;
            s.b[2361] = (s.v[2108] > 0.1);
            s.v[2361] = if s.b[2361] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2360])) && s.b[2361]) {
                s.store_scalar(2108, 0.1);
            }
            s.b[2362] = (s.v[2108] < (-0.1));
            s.v[2362] = if s.b[2362] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) && (!s.b[2360])) && (!s.b[2361])) && s.b[2362]) {
                s.store_scalar(2108, (-0.1));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2108);
            }
            let (assign54010_body21_e83135,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
        let assign54010_body21_e83133: f64 = (s.v[97] + 1.0);
        (assign54010_body21_e83133,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign54010_body21_e83135;
        }

        s.b[2364] = (2.0 == 1.0);
        s.v[2364] = if s.b[2364] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2364]) {
            s.copy_ad(2167, 983);
        }

        s.b[2365] = ((s.v[983] < (s.v[2167] + 0.2)) && (0.2 >= 0.0));
        s.v[2365] = if s.b[2365] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
            s.store_sub_offset_lhs(781, 2167, 0.2, 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign54110_e83288,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54110_e83288;

        let (assign54120_e83307,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54120_e83307;

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2366] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2366] = if s.b[2366] { 1.0 } else { 0.0 };

        s.b[2367] = (2.0 == 1.0);
        s.v[2367] = if s.b[2367] { 1.0 } else { 0.0 };

        let (assign54230_e83510,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && s.b[2367]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54230_e83510;

        s.b[2368] = (2.0 == 2.0);
        s.v[2368] = if s.b[2368] { 1.0 } else { 0.0 };

        let (assign54250_e83539,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && s.b[2368]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54250_e83539;

        s.b[2369] = (2.0 == 4.0);
        s.v[2369] = if s.b[2369] { 1.0 } else { 0.0 };

        let (assign54270_e83571,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && (!s.b[2368])) && s.b[2369]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54270_e83571;

        s.b[2370] = (2.0 == 8.0);
        s.v[2370] = if s.b[2370] { 1.0 } else { 0.0 };

        let (assign54290_e83606,) = {
    if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (!s.b[2367])) && (!s.b[2368])) && (!s.b[2369])) && s.b[2370]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54290_e83606;

        let (assign54300_e83627,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54300_e83627;

        let mut assign54310_loop_guard: usize = 0;
        while {
            let assign54310_cond_e83649: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54310_cond_e83649 != 0.0
        } {
            assign54310_loop_guard += 1;
            assert!(assign54310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {
                s.store_sqrt(726, 726);
            }
            let (assign54310_body1_e83694,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && s.b[2366]) {
        let assign54310_body1_e83692: f64 = (s.v[719] + 1.0);
        (assign54310_body1_e83692,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign54310_body1_e83694;
        }

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) && (!s.b[2366])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_offset_lhs(983, 2167, 0.2, 780);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && s.b[2365]) {
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && (!s.b[2365])) {
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2364])) && (!s.b[2365])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.copy_ad(2149, 983);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_scalar(2138, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2371] = ((s.v[2149] > (-s.v[2138])) && (s.v[2138] >= 0.0));
        s.v[2371] = if s.b[2371] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
            s.store_add(781, 2149, 2138);
            s.store_square(722, 781);
            s.store_square(723, 2138);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign54480_e84024,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54480_e84024;

        let (assign54490_e84040,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54490_e84040;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign54520_e84088,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54520_e84088;

        let mut assign54530_loop_guard: usize = 0;
        while {
            let assign54530_cond_e84105: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && (s.v[719] < s.v[2139])) { 1.0 } else { 0.0 };
            assign54530_cond_e84105 != 0.0
        } {
            assign54530_loop_guard += 1;
            assert!(assign54530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign54530_body2_e84159,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        let assign54530_body2_e84157: f64 = (s.v[719] + 1.0);
        (assign54530_body2_e84157,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign54530_body2_e84159;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2372] = ((((s.v[2139] == 1.0) || (s.v[2139] == 2.0)) || (s.v[2139] == 4.0)) || (s.v[2139] == 8.0));
        s.v[2372] = if s.b[2372] { 1.0 } else { 0.0 };

        s.b[2373] = (s.v[2139] == 1.0);
        s.v[2373] = if s.b[2373] { 1.0 } else { 0.0 };

        let (assign54580_e84231,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && s.b[2373]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54580_e84231;

        s.b[2374] = (s.v[2139] == 2.0);
        s.v[2374] = if s.b[2374] { 1.0 } else { 0.0 };

        let (assign54600_e84257,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && s.b[2374]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54600_e84257;

        s.b[2375] = (s.v[2139] == 4.0);
        s.v[2375] = if s.b[2375] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign54620_e84286,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && (!s.b[2374])) && s.b[2375]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54620_e84286;

        s.b[2376] = (s.v[2139] == 8.0);
        s.v[2376] = if s.b[2376] { 1.0 } else { 0.0 };

        let (assign54640_e84318,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (!s.b[2373])) && (!s.b[2374])) && (!s.b[2375])) && s.b[2376]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54640_e84318;

        let (assign54650_e84336,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54650_e84336;

        let mut assign54660_loop_guard: usize = 0;
        while {
            let assign54660_cond_e84355: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54660_cond_e84355 != 0.0
        } {
            assign54660_loop_guard += 1;
            assert!(assign54660_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {
                s.store_sqrt(726, 726);
            }
            let (assign54660_body1_e84394,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && s.b[2372]) {
        let assign54660_body1_e84392: f64 = (s.v[719] + 1.0);
        (assign54660_body1_e84392,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign54660_body1_e84394;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) && (!s.b[2372])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2139), 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2138, 726);
            s.store_div_scaled_product3_indices(334, 2138, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2138, -1.0, 780, 1.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2371]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2371])) {
            s.copy_ad(983, 2149);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2316, 2129, 2144, (0.5 * 9662367879.197212), 0.0, 2144);
            s.store_mul_sqrt_ad_rhs(334, 2148, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2316)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2377] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2377] = if s.b[2377] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2377]) {
            s.store_div_ln_lhs(2317, 335, 2316);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2377])) {
            s.store_mul3_ad_middle(2317, A::square(s.ad_value(2148)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_mul(332, 2317, 983);
        }

        s.b[2378] = (s.v[332] > 500.0);
        s.v[2378] = if s.b[2378] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2378]) {
            s.store_sub(2161, 983, 2316);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2317, -1.0, 2316);
        }

        s.b[2379] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2379] = if s.b[2379] { 1.0 } else { 0.0 };

        s.b[2380] = (s.v[332] >= 500.0);
        s.v[2380] = if s.b[2380] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && s.b[2380]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign54920_loop_guard: usize = 0;
        while {
            let assign54920_cond_e84865: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign54920_cond_e84865 != 0.0
        } {
            assign54920_loop_guard += 1;
            assert!(assign54920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) && (!s.b[2380])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2379]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && (!s.b[2379])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2381] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2381] = if s.b[2381] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && s.b[2381]) {
            s.store_div_ln_offset_lhs(2161, 336, 1.0, 2317);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2378])) && (!s.b[2381])) {
            s.store_div(2161, 336, 2317);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_sub(336, 983, 2161);
        }

        s.b[2382] = (0.0 == 0.0);
        s.v[2382] = if s.b[2382] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2382]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2143, A::sqrt(A::mul_scaled_lhs(s.ad_value(2132), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2143, 2132, 336);
            }
        }

        s.b[2383] = (s.v[336] < 0.0);
        s.v[2383] = if s.b[2383] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2382])) && s.b[2383]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2143, A::sqrt(A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2382])) && (!s.b[2383])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2143, A::mul3(s.ad_value(2132), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) {
            s.store_sub(990, 2144, 2143);
        }

        s.b[2384] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2384] = if s.b[2384] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign55170_e85388,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55170_e85388;

        let (assign55180_e85404,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55180_e85404;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2385] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2385] = if s.b[2385] { 1.0 } else { 0.0 };

        s.b[2386] = (2.0 == 1.0);
        s.v[2386] = if s.b[2386] { 1.0 } else { 0.0 };

        let (assign55290_e85580,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && s.b[2386]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55290_e85580;

        s.b[2387] = (2.0 == 2.0);
        s.v[2387] = if s.b[2387] { 1.0 } else { 0.0 };

        let (assign55310_e85606,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && s.b[2387]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55310_e85606;

        s.b[2388] = (2.0 == 4.0);
        s.v[2388] = if s.b[2388] { 1.0 } else { 0.0 };

        let (assign55330_e85635,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && (!s.b[2387])) && s.b[2388]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55330_e85635;

        s.b[2389] = (2.0 == 8.0);
        s.v[2389] = if s.b[2389] { 1.0 } else { 0.0 };

        let (assign55350_e85667,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (!s.b[2386])) && (!s.b[2387])) && (!s.b[2388])) && s.b[2389]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55350_e85667;

        let (assign55360_e85685,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55360_e85685;

        let mut assign55370_loop_guard: usize = 0;
        while {
            let assign55370_cond_e85704: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55370_cond_e85704 != 0.0
        } {
            assign55370_loop_guard += 1;
            assert!(assign55370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {
                s.store_sqrt(726, 726);
            }
            let (assign55370_body1_e85743,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && s.b[2385]) {
        let assign55370_body1_e85741: f64 = (s.v[719] + 1.0);
        (assign55370_body1_e85741,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign55370_body1_e85743;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) && (!s.b[2385])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2384]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2384])) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && (!s.b[2384])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2390] = (2.0 == 1.0);
        s.v[2390] = if s.b[2390] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2324])) && s.b[2390]) {
            s.copy_ad(2151, 990);
        }

        s.b[2391] = (0.0 == 0.0);
        s.v[2391] = if s.b[2391] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) {
            s.copy_ad(989, 349);
            s.store_scaled_add(344, 2113, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(2131), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 2131, 1.0);
        }

        s.b[2392] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2392] = if s.b[2392] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign55590_e86096,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55590_e86096;

        let (assign55600_e86111,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55600_e86111;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2393] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2393] = if s.b[2393] { 1.0 } else { 0.0 };

        s.b[2394] = (2.0 == 1.0);
        s.v[2394] = if s.b[2394] { 1.0 } else { 0.0 };

        let (assign55710_e86278,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && s.b[2394]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55710_e86278;

        s.b[2395] = (2.0 == 2.0);
        s.v[2395] = if s.b[2395] { 1.0 } else { 0.0 };

        let (assign55730_e86303,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (!s.b[2394])) && s.b[2395]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55730_e86303;

        s.b[2396] = (2.0 == 4.0);
        s.v[2396] = if s.b[2396] { 1.0 } else { 0.0 };

        let (assign55750_e86331,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (!s.b[2394])) && (!s.b[2395])) && s.b[2396]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55750_e86331;

        s.b[2397] = (2.0 == 8.0);
        s.v[2397] = if s.b[2397] { 1.0 } else { 0.0 };

        let (assign55770_e86362,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (!s.b[2394])) && (!s.b[2395])) && (!s.b[2396])) && s.b[2397]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55770_e86362;

        let (assign55780_e86379,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55780_e86379;

        let mut assign55790_loop_guard: usize = 0;
        while {
            let assign55790_cond_e86397: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55790_cond_e86397 != 0.0
        } {
            assign55790_loop_guard += 1;
            assert!(assign55790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) {
                s.store_sqrt(726, 726);
            }
            let (assign55790_body1_e86434,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && s.b[2393]) {
        let assign55790_body1_e86432: f64 = (s.v[719] + 1.0);
        (assign55790_body1_e86432,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign55790_body1_e86434;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) && (!s.b[2393])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2392]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && (!s.b[2392])) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && (!s.b[2392])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) {
            s.store_sqrt(337, 338);
            s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2130, 1.0, 337);
        }

        s.b[2398] = ((s.v[344] < (s.v[972] + p.p405)) && (p.p405 >= 0.0));
        s.v[2398] = if s.b[2398] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {
            s.store_sub_offset_lhs(781, 972, p.p405, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p405 * p.p405));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign55960_e86726,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55960_e86726;

        let (assign55970_e86741,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55970_e86741;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2399] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2399] = if s.b[2399] { 1.0 } else { 0.0 };

        s.b[2400] = (2.0 == 1.0);
        s.v[2400] = if s.b[2400] { 1.0 } else { 0.0 };

        let (assign56080_e86908,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && s.b[2400]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56080_e86908;

        s.b[2401] = (2.0 == 2.0);
        s.v[2401] = if s.b[2401] { 1.0 } else { 0.0 };

        let (assign56100_e86933,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (!s.b[2400])) && s.b[2401]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56100_e86933;

        s.b[2402] = (2.0 == 4.0);
        s.v[2402] = if s.b[2402] { 1.0 } else { 0.0 };

        let (assign56120_e86961,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (!s.b[2400])) && (!s.b[2401])) && s.b[2402]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56120_e86961;

        s.b[2403] = (2.0 == 8.0);
        s.v[2403] = if s.b[2403] { 1.0 } else { 0.0 };

        let (assign56140_e86992,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (!s.b[2400])) && (!s.b[2401])) && (!s.b[2402])) && s.b[2403]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56140_e86992;

        let (assign56150_e87009,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign56150_e87009;

        let mut assign56160_loop_guard: usize = 0;
        while {
            let assign56160_cond_e87027: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56160_cond_e87027 != 0.0
        } {
            assign56160_loop_guard += 1;
            assert!(assign56160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) {
                s.store_sqrt(726, 726);
            }
            let (assign56160_body1_e87064,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && s.b[2399]) {
        let assign56160_body1_e87062: f64 = (s.v[719] + 1.0);
        (assign56160_body1_e87062,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign56160_body1_e87064;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) && (!s.b[2399])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p405);
            s.store_div_scaled_product_indices(334, 725, 726, p.p405, 770, 1.0);
            s.store_sub_offset_lhs(992, 972, p.p405, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && s.b[2398]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2391]) && (!s.b[2398])) {
            s.copy_ad(992, 344);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
            s.copy_ad(2155, 2141);
            s.store_offset_mul(338, 2131, 2155, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
            s.store_add_mul_sub_from_scalar_rhs_indices(2156, 2155, 2130, 1.0, 337);
            s.copy_ad(2152, 2156);
        }

        let (assign56300_e87319,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign56300_e87319;

        let (assign56310_e87333,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign56310_e87333;

        let mut assign56320_loop_guard: usize = 0;
        while {
            let assign56320_cond_e87348: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign56320_cond_e87348 != 0.0
        } {
            assign56320_loop_guard += 1;
            assert!(assign56320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
                s.store_mul_neg_lhs(335, 154, 2152);
                s.store_exp(336, 335);
                s.store_sqrt_div_scaled_inputs(338, 2110, 2.0, 154, 1.0);
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2153, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2404] = (s.v[335] > 0.0);
            s.v[2404] = if s.b[2404] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && s.b[2404]) {
                s.store_neg(2153, 2153);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2153, 1.0);
                s.store_mul_sub_from_scalar_rhs(2154, 345, 1.0, 336);
            }
            let (assign56320_body9_e87515,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && (s.v[79] != 0.0)) {
        let assign56320_body9_e87513: f64 = (150.0 + 1.0);
        (assign56320_body9_e87513,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign56320_body9_e87515;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2096, 2153, 1.0, 185, 2155, 2152, -1.0);
                s.store_add(2097, 185, 2154);
                s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);
            }
            s.b[2405] = (((s.v[2108]) as f64).abs() < 1e-10);
            s.v[2405] = if s.b[2405] { 1.0 } else { 0.0 };
            let (assign56320_body14_e87601,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && (s.v[79] == 0.0)) && s.b[2405]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign56320_body14_e87601;
            s.b[2406] = (s.v[2108] > 0.1);
            s.v[2406] = if s.b[2406] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && (s.v[79] == 0.0)) && (!s.b[2405])) && s.b[2406]) {
                s.store_scalar(2108, 0.1);
            }
            s.b[2407] = (s.v[2108] < (-0.1));
            s.v[2407] = if s.b[2407] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && (s.v[79] == 0.0)) && (!s.b[2405])) && (!s.b[2406])) && s.b[2407]) {
                s.store_scalar(2108, (-0.1));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && (s.v[79] == 0.0)) {
                s.store_add(2152, 2152, 2108);
            }
            let (assign56320_body20_e87691,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
        let assign56320_body20_e87689: f64 = (s.v[97] + 1.0);
        (assign56320_body20_e87689,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign56320_body20_e87691;
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) {
            s.copy_ad(2149, 2152);
            s.copy_ad(989, 349);
            s.store_sqrt_square_offset(782, 2149, ((4.0 * p.p405) * p.p405));
            s.store_offset_scaled_div(334, 2149, 782, 0.5, 0.5);
            s.store_scaled_add(992, 2149, 782, 0.5);
        }

        s.b[2408] = (s.v[992] < 0.0);
        s.v[2408] = if s.b[2408] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2391])) && s.b[2408]) {
            s.store_scalar(992, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_div(335, 989, 992);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

    }

    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_mul(340, 338, 337);
        }

        s.b[2409] = ((s.v[349] > (s.v[972] - (s.v[972] * 0.5))) && ((s.v[972] * 0.5) >= 0.0));
        s.v[2409] = if s.b[2409] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
            s.store_add_scaled_inputs3_indices(781, 349, 1.0, 972, (-1.0), 972, 0.5);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 972, 972, (0.5 * 0.5));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign56520_e88003,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign56520_e88003;

        let (assign56530_e88016,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56530_e88016;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2410] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2410] = if s.b[2410] { 1.0 } else { 0.0 };

        s.b[2411] = (2.0 == 1.0);
        s.v[2411] = if s.b[2411] { 1.0 } else { 0.0 };

        let (assign56640_e88165,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) && s.b[2411]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56640_e88165;

        s.b[2412] = (2.0 == 2.0);
        s.v[2412] = if s.b[2412] { 1.0 } else { 0.0 };

        let (assign56660_e88188,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) && (!s.b[2411])) && s.b[2412]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56660_e88188;

        s.b[2413] = (2.0 == 4.0);
        s.v[2413] = if s.b[2413] { 1.0 } else { 0.0 };

        let (assign56680_e88214,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) && (!s.b[2411])) && (!s.b[2412])) && s.b[2413]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56680_e88214;

        s.b[2414] = (2.0 == 8.0);
        s.v[2414] = if s.b[2414] { 1.0 } else { 0.0 };

        let (assign56700_e88243,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) && (!s.b[2411])) && (!s.b[2412])) && (!s.b[2413])) && s.b[2414]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56700_e88243;

        let (assign56710_e88258,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign56710_e88258;

        let mut assign56720_loop_guard: usize = 0;
        while {
            let assign56720_cond_e88274: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56720_cond_e88274 != 0.0
        } {
            assign56720_loop_guard += 1;
            assert!(assign56720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) {
                s.store_sqrt(726, 726);
            }
            let (assign56720_body1_e88307,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && s.b[2410]) {
        let assign56720_body1_e88305: f64 = (s.v[719] + 1.0);
        (assign56720_body1_e88305,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign56720_body1_e88307;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) && (!s.b[2410])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);
            s.store_add_scaled_inputs3_indices(2162, 972, 1.0, 972, (-0.5), 780, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2409]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2409])) {
            s.copy_ad(2162, 349);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_add_div_lhs_indices(989, 989, 340, 2162);
            s.store_mul_square_lhs(338, 2162, 2162);
            s.store_offset(334, 338, 0.0001);
            s.store_div(2163, 338, 334);
        }

        s.b[2415] = (p.p43 == (-1.0));
        s.v[2415] = if s.b[2415] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2415]) {
            s.store_scalar(2163, 0.0);
            s.copy_ad(989, 349);
        }

        s.b[2416] = (p.p43 == 2.0);
        s.v[2416] = if s.b[2416] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) && s.b[2416]) {
            s.copy_ad(989, 349);
            s.store_scalar(2162, 0.0);
            s.store_scalar(2163, 0.0);
            s.store_sub(335, 2142, 972);
            s.store_add_scaled_inputs3_offset_mixed_iai(992, 335, 0.5, A::ln(A::cosh(s.ad_value(335))), 0.5, 972, 1.0, (((2.0) as f64).ln() * 0.5));
        }

        s.b[2417] = (p.p43 == 3.0);
        s.v[2417] = if s.b[2417] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) && (!s.b[2416])) && s.b[2417]) {
            s.store_add_ad_lhs(992, A::ln_one_plus_exp(A::sub(s.ad_value(2142), s.ad_value(972))), 972);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            s.store_div(335, 989, 992);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2415])) {
            s.store_mul(340, 338, 337);
            s.store_add_div_lhs_indices(989, 989, 340, 2162);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_mul(2120, 990, 2129);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 2120, 343);
            s.store_offset_sqrt_ad(2164, A::offset(A::square(s.ad_value(989)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 2164, 688, 1.0);
            s.store_offset_mul(339, 2164, 689, 1.0);
        }

        s.b[2418] = param_given[408];
        s.v[2418] = if s.b[2418] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2418]) {
            s.store_div_scaled_value_by_product(2150, A::sub_from_scalar(p.p408, s.ad_value(2088)), 1.0, s.ad_value(965), s.ad_value(339), 100.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2418])) {
            s.store_div_scaled_inputs_indices(2150, 2120, 9662367879.197212, 339, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[2150] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 2150, p.p376);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add(s.ad_value(966), A::mul3_scaled_output(s.ad_value(968), s.ad_value(338), s.ad_value(252), 1e-10)), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div_scaled_value_offset_denominator(2111, s.ad_value(989), 1.0, s.ad_value(162), p.p401, 1.0);
            s.store_square(781, 989);
            s.store_scalar(782, ((0.01) as f64).powf(2.0));
            s.store_sub_ad(334, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div_scaled_value_offset_denominator(2165, s.ad_value(334), 1.0, s.ad_value(162), (-p.p402), 1.0);
            s.store_div_scaled_product_indices(335, 254, 2165, 1.0, 973, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_div(985, 254, 338);
            s.store_mul_offset_ad_rhs(2128, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2111), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2111), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p.p400), 1.0);
            s.store_scaled_mul(335, 990, 2128, 1.6021918e-19);
            s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p.p7);
            s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2111);
            s.store_mul3_affine_lhs(988, 336, 2151, p.p363, 0.0, 2163);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add_scaled_inputs3_indices(135, 986, 1.0, 987, 1.0, 988, 1.0);
            s.copy_ad(790, 349);
        }

        s.b[2419] = (p.p283 != 0.0);
        s.v[2419] = if s.b[2419] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2419]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(2085), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2420] = (s.v[336] < 0.0);
        s.v[2420] = if s.b[2420] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2419]) && s.b[2420]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2419]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
        }

    }

    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2419]) {
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1435, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 2085, 1.0, 340, 1.0, 1434, -1.0);
            s.store_add_product3_rhs_indices(338, 338, 1435, 334, 339, 1.0);
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2419])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2421] = (p.p287 != 0.0);
        s.v[2421] = if s.b[2421] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2421]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1435);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2421])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2422] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2422] = if s.b[2422] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2422]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.copy_ad(134, 135);
            s.store_add_scaled_inputs4_indices(131, 2094, (-0.5), 2118, ((-1.0) * (-0.5)), 2095, (-0.5), 2119, (-(-0.5)));
            s.store_scaled_add(133, 2118, 2119, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 2118, 2119, (-0.5));
            s.store_neg(238, 2118);
            s.copy_ad(255, 2112);
        }

        s.b[2423] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2423] = if s.b[2423] { 1.0 } else { 0.0 };

        let (assign57760_e89973,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2423]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign57760_e89973;

        s.b[2424] = (s.v[791] < s.v[86]);
        s.v[2424] = if s.b[2424] { 1.0 } else { 0.0 };

        let (assign57780_e89984,) = {
    if ((!s.b[1439]) && s.b[2424]) {
        let assign57780_e89982: f64 = (-1.0);
        (assign57780_e89982,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign57780_e89984;

        if ((!s.b[1439]) && s.b[2424]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 85, 1431);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2425] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2425] = if s.b[2425] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2424]) && s.b[2425]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((!s.b[1439]) && s.b[2424]) && (!s.b[2425])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((!s.b[1439]) && s.b[2424]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if ((!s.b[1439]) && s.b[2424]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 1431, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 1431);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_div_lhs_indices(87, 335, 337, 1431);
            s.copy_ad(91, 87);
            s.store_scalar(94, 0.0);
            s.store_sub(336, 85, 87);
            s.store_mul(131, 185, 336);
            s.store_scalar(133, 0.0);
            s.store_scalar(247, 0.0);
            s.store_scalar(169, 0.0);
            s.store_scalar(134, 0.0);
            s.store_scalar(127, 0.0);
        }

        let (assign58110_e90322,) = {
    if ((!s.b[1439]) && s.b[2424]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58110_e90322;

        let (assign58120_e90329,) = {
    if ((!s.b[1439]) && s.b[2424]) {
        (1.0,)
    } else {
        (s.v[946],)
    }
};
        s.v[946] = assign58120_e90329;

        s.b[2426] = (s.v[946] == 0.0);
        s.v[2426] = if s.b[2426] { 1.0 } else { 0.0 };

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1431))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_add_product3_rhs_mixed_iia(89, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);
        }

        s.b[2427] = (s.v[77] == 0.0);
        s.v[2427] = if s.b[2427] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2427]) {
            s.store_mul_sub_rhs(116, 154, 89, 1431);
        }

        s.b[2428] = (s.v[116] < 3.0);
        s.v[2428] = if s.b[2428] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && s.b[2428]) {
            s.store_mul_sub_rhs(333, 154, 85, 1431);
            s.store_div_from_scalar_scaled_mul(335, 1.0, 154, 212, (1.414213562373095 / 108.0));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && s.b[2428]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && s.b[2428]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 1431, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2429] = (s.v[791] <= s.v[118]);
        s.v[2429] = if s.b[2429] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && s.b[2429]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && (!s.b[2429])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 85, 85);
            s.store_add_div_from_scalar_rhs(337, 154, 2.0, 85);
            s.store_div_ln_lhs(90, 336, 337);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && (!s.b[2429])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2427]) && (!s.b[2428])) && (!s.b[2429])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(332, 1431, (1e-12 / 2.0));
        }

        s.b[2430] = (s.v[88] < s.v[332]);
        s.v[2430] = if s.b[2430] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2430]) {
            s.copy_ad(88, 332);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.copy_ad(87, 88);
            s.copy_ad(92, 89);
            s.store_exp_mul(229, 154, 1431);
            s.store_mul(222, 210, 229);
        }

        let (assign58490_e90882,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign58490_e90882;

        let (assign58500_e90889,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign58500_e90889;

    }

    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
    ) {
        let mut assign58510_loop_guard: usize = 0;
        while {
            let assign58510_cond_e90897: f64 = (s.v[421] + 1.0);
            let assign58510_cond_e90899: f64 = if (((!s.b[1439]) && s.b[2426]) && (s.v[97] <= assign58510_cond_e90897)) { 1.0 } else { 0.0 };
            assign58510_cond_e90899 != 0.0
        } {
            assign58510_loop_guard += 1;
            assert!(assign58510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_mul_sub_rhs(116, 154, 87, 1431);
            }
            s.b[2431] = (s.v[116] < 5.0);
            s.v[2431] = if s.b[2431] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2431]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(214, 222, 225, 225);
                s.store_mul_product3_indices(215, 226, 222, 154, 225, 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_inputs2_mixed_aii(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 215, 1.0, 216, 2.0);
            }
            s.b[2432] = (s.v[116] < 60.0);
            s.v[2432] = if s.b[2432] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2431])) && s.b[2432]) {
                s.store_exp(227, 116);
                s.store_mul_offset_rhs(214, 222, 227, (-1.0));
                s.store_mul3_lhs(215, 222, 154, 227);
            }
            if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2431])) && (!s.b[2432])) {
                s.store_exp_mul(231, 154, 87);
                s.store_mul_sub_rhs(214, 210, 231, 229);
                s.store_mul3_lhs(215, 210, 154, 231);
            }
            if (((!s.b[1439]) && s.b[2426]) && (!s.b[2431])) {
                s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));
                s.store_div_scaled_inputs2_indices(217, 154, 1.0, 215, 1.0, 216, 2.0);
            }
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));
                s.store_sub_from_scalar_scaled_mul(233, (-1.0), 212, 217, 1.0);
            }
            s.b[2433] = (s.v[79] == 1.0);
            s.v[2433] = if s.b[2433] { 1.0 } else { 0.0 };
            let (assign58510_body23_e91269,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2433]) {
        (1.0,)
    } else {
        (s.v[944],)
    }
};
            s.v[944] = assign58510_body23_e91269;
            s.b[2434] = (s.v[944] == 0.0);
            s.v[2434] = if s.b[2434] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2434]) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2434]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2435] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2435] = if s.b[2435] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2434]) && s.b[2435]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2434]) {
                s.store_add(87, 87, 236);
            }
            s.b[2436] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2436] = if s.b[2436] { 1.0 } else { 0.0 };
            let (assign58510_body31_e91360,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2434]) && s.b[2436]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign58510_body31_e91360;
            let (assign58510_body32_e91371,) = {
    if (((!s.b[1439]) && s.b[2426]) && (s.v[944] != 0.0)) {
        let assign58510_body32_e91369: f64 = (s.v[421] + 1.0);
        (assign58510_body32_e91369,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign58510_body32_e91371;
            let (assign58510_body33_e91378,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        (0.0,)
    } else {
        (s.v[944],)
    }
};
            s.v[944] = assign58510_body33_e91378;
            let (assign58510_body34_e91387,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        let assign58510_body34_e91385: f64 = (s.v[97] + 1.0);
        (assign58510_body34_e91385,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign58510_body34_e91387;
        }

        let (assign58520_e91396,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        let assign58520_e91394: f64 = (s.v[97] - 1.0);
        (assign58520_e91394,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign58520_e91396;

        s.b[2438] = (s.v[116] < 5.0);
        s.v[2438] = if s.b[2438] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2438]) {
            s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        let (assign58580_e91457,) = {
    if (((!s.b[1439]) && s.b[2426]) && (!s.b[2438])) {
        (3.0,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign58580_e91457;

        let (assign58590_e91467,) = {
    if (((!s.b[1439]) && s.b[2426]) && (!s.b[2438])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58590_e91467;

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2438])) {
            s.store_offset(99, 116, (-1.0));
            s.store_sqrt(100, 99);
            s.store_mul(101, 99, 100);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(239, 209, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_offset_product3(238, s.ad_value(209), s.ad_value(214), s.ad_value(335), 1.0, 1e-25);
        }

        s.b[2439] = (s.v[116] < 5.0);
        s.v[2439] = if s.b[2439] { 1.0 } else { 0.0 };

        s.b[2440] = (s.v[116] < 3.0);
        s.v[2440] = if s.b[2440] { 1.0 } else { 0.0 };

        let (assign58680_e91552,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && s.b[2440]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign58680_e91552;

        let (assign58690_e91563,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && s.b[2440]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58690_e91563;

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && s.b[2440]) {
            s.copy_ad(133, 238);
            s.copy_ad(131, 239);
            s.store_scalar(247, 0.5);
            s.store_scalar(169, 0.0);
        }

        let (assign58740_e91619,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && (!s.b[2440])) {
        (2.0,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign58740_e91619;

        let (assign58750_e91631,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && (!s.b[2440])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58750_e91631;

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2439]) && (!s.b[2440])) {
            s.store_scalar(335, (1.0 / (5.0 - 3.0)));
            s.store_mul_offset_rhs(332, 335, 116, (-3.0));
            s.store_mul3_ad_middle(207, A::square(s.ad_value(332)), 332, A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0));
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(127, 238, 186);
            s.copy_ad(349, 790);
            s.store_div_square_rhs(336, 636, 185);
            s.store_add_scaled_inputs3_indices(334, 85, 1.0, 155, (-1.0), 1434, -1.0);
            s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);
            s.store_scaled_add(343, 335, 782, 0.5);
        }

        s.b[2441] = (s.v[343] < 0.0);
        s.v[2441] = if s.b[2441] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2441]) {
            s.store_scalar(343, 0.0);
            s.store_scalar(332, 0.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 336, 1.0, 337);
            s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);
            s.store_scaled_add(344, 344, 782, 0.5);
        }

        s.b[2442] = (s.v[344] < 0.0);
        s.v[2442] = if s.b[2442] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2442]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
            s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1431), s.ad_value(790))));
        }

        s.b[2443] = (s.v[790] < 0.0);
        s.v[2443] = if s.b[2443] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2443]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(91, 87);
        }

        let (assign59120_e92032,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2443]) {
        (1.0,)
    } else {
        (s.v[947],)
    }
};
        s.v[947] = assign59120_e92032;

        s.b[2444] = (s.v[947] == 0.0);
        s.v[2444] = if s.b[2444] { 1.0 } else { 0.0 };

        s.b[2445] = (s.v[77] == 0.0);
        s.v[2445] = if s.b[2445] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }

        s.b[2446] = (((1.0 + 0.3) * s.v[96]) > 0.03);
        s.v[2446] = if s.b[2446] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && s.b[2446]) {
            s.store_offset_sub_scaled_inputs_indices(781, 96, (1.0 + 0.3), 790, 1.0, (-0.03));
            s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && s.b[2446]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && s.b[2446]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(95, 96, (1.0 + 0.3), 781, (-0.5), 782, (-0.5));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) && (!s.b[2446])) {
            s.store_scale(95, 96, (1.0 + 0.3));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2445]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }

        s.b[2447] = (s.v[95] < 0.0);
        s.v[2447] = if s.b[2447] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && s.b[2447]) {
            s.store_scalar(95, 0.0);
        }

        s.b[2448] = (s.v[95] > s.v[790]);
        s.v[2448] = if s.b[2448] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2444]) && (!s.b[2447])) && s.b[2448]) {
            s.copy_ad(95, 790);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2444]) {
            s.copy_ad(94, 95);
            s.store_add(91, 87, 94);
        }

    }

    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
    ) {
        let (assign59320_e92289,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2444]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign59320_e92289;

        let (assign59330_e92298,) = {
    if (((!s.b[1439]) && s.b[2426]) && (s.v[947] != 0.0)) {
        (0.0,)
    } else {
        (s.v[947],)
    }
};
        s.v[947] = assign59330_e92298;

        let (assign59340_e92305,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.v[98] = assign59340_e92305;

        let mut assign59350_loop_guard: usize = 0;
        while {
            let assign59350_cond_e92313: f64 = (40.0 + 1.0);
            let assign59350_cond_e92315: f64 = if (((!s.b[1439]) && s.b[2426]) && (s.v[98] <= assign59350_cond_e92313)) { 1.0 } else { 0.0 };
            assign59350_cond_e92315 != 0.0
        } {
            assign59350_loop_guard += 1;
            assert!(assign59350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_mul_sub_rhs(116, 154, 91, 1431);
            }
            s.b[2449] = (s.v[116] < 5.0);
            s.v[2449] = if s.b[2449] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2449]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul(222, 210, 230);
                s.store_mul3_lhs(218, 222, 225, 225);
                s.store_mul_product3_indices(219, 226, 222, 154, 225, 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(220, 223, 218);
                s.store_div_scaled_inputs2_mixed_aii(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 219, 1.0, 220, 2.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && (!s.b[2449])) {
                s.store_mul_sub_rhs(117, 154, 91, 790);
                s.store_exp(228, 117);
                s.store_mul_sub_rhs(218, 210, 228, 230);
                s.store_mul3_lhs(219, 210, 154, 228);
                s.store_offset(102, 116, (-1.0));
                s.store_sqrt_add(220, 102, 218);
                s.store_div_scaled_inputs2_indices(221, 154, 1.0, 219, 1.0, 220, 2.0);
            }
            if ((!s.b[1439]) && s.b[2426]) {
                s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));
                s.store_sub_from_scalar_scaled_mul(235, (-1.0), 212, 221, 1.0);
            }
            s.b[2450] = (s.v[79] == 1.0);
            s.v[2450] = if s.b[2450] { 1.0 } else { 0.0 };
            let (assign59350_body22_e92661,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2450]) {
        (1.0,)
    } else {
        (s.v[945],)
    }
};
            s.v[945] = assign59350_body22_e92661;
            s.b[2451] = (s.v[945] == 0.0);
            s.v[2451] = if s.b[2451] { 1.0 } else { 0.0 };
            if (((!s.b[1439]) && s.b[2426]) && s.b[2451]) {
                s.store_div_scaled_inputs_indices(237, 234, -1.0, 235, 1.0);
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2451]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2452] = (((s.v[237]) as f64).abs() > s.v[93]);
            s.v[2452] = if s.b[2452] { 1.0 } else { 0.0 };
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2451]) && s.b[2452]) {
                s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1439]) && s.b[2426]) && s.b[2451]) {
                s.store_add(91, 91, 237);
            }
            s.b[2453] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));
            s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };
            let (assign59350_body30_e92752,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2451]) && s.b[2453]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign59350_body30_e92752;
            let (assign59350_body31_e92763,) = {
    if (((!s.b[1439]) && s.b[2426]) && (s.v[945] != 0.0)) {
        let assign59350_body31_e92761: f64 = (40.0 + 1.0);
        (assign59350_body31_e92761,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign59350_body31_e92763;
            let (assign59350_body32_e92770,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        (0.0,)
    } else {
        (s.v[945],)
    }
};
            s.v[945] = assign59350_body32_e92770;
            let (assign59350_body33_e92779,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        let assign59350_body33_e92777: f64 = (s.v[98] + 1.0);
        (assign59350_body33_e92777,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign59350_body33_e92779;
        }

        let (assign59360_e92788,) = {
    if ((!s.b[1439]) && s.b[2426]) {
        let assign59360_e92786: f64 = (s.v[98] - 1.0);
        (assign59360_e92786,)
    } else {
        (s.v[98],)
    }
};
        s.v[98] = assign59360_e92788;

        s.b[2455] = (s.v[116] < 5.0);
        s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2455]) {
            s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2455])) {
            s.store_offset(102, 116, (-1.0));
            s.store_sqrt(103, 102);
            s.store_mul(104, 102, 103);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_sub(94, 91, 87);
            s.copy_ad(790, 349);
            s.store_div(335, 154, 99);
            s.store_mul(258, 335, 94);
            s.store_offset(259, 258, 1.0);
            s.store_sqrt(260, 259);
            s.store_mul(261, 260, 259);
            s.store_mul(262, 261, 259);
            s.store_div_from_scalar_offset_input(263, 1.0, 260, 1.0);
            s.store_div_from_scalar_offset_input(264, 1.0, 261, 1.0);
            s.store_div_from_scalar_offset_input(265, 1.0, 262, 1.0);
            s.store_div(266, 263, 100);
            s.store_offset_mul_offset_rhs(335, 258, 258, 3.0, 3.0);
            s.store_mul3_affine_lhs(267, 100, 264, 0.6666666666666667, 0.0, 335);
            s.store_offset_mul_offset_rhs_ad_rhs(335, 258, A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 5.0), 10.0), 10.0, 5.0);
            s.store_mul_product3_mixed_iaii(268, 335, A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), 101, 265, 1.0);
            s.store_sub_ad_lhs(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);
            s.store_add_scaled_inputs4_indices(335, 85, 1.0, 155, 1.0, 87, (-(2.0 * 0.5)), 94, (-0.5));
            s.store_sub(336, 266, 267);
            s.store_mul(337, 154, 185);
            s.store_mul(338, 154, 209);
            s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);
            s.store_mul(248, 94, 250);
        }

        s.b[2456] = (s.v[347] == 1.0);
        s.v[2456] = if s.b[2456] { 1.0 } else { 0.0 };

        let (assign59690_e93145,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2456]) {
        (1.0,)
    } else {
        (s.v[948],)
    }
};
        s.v[948] = assign59690_e93145;

        s.b[2457] = (s.v[948] == 0.0);
        s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };

        s.b[2458] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2458] = if s.b[2458] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2459] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign59800_e93298,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign59800_e93298;

        let (assign59810_e93311,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59810_e93311;

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2460] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        s.b[2461] = (2.0 == 1.0);
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        let (assign59920_e93460,) = {
    if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59920_e93460;

        s.b[2462] = (2.0 == 2.0);
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        let (assign59940_e93483,) = {
    if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) && s.b[2462]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59940_e93483;

        s.b[2463] = (2.0 == 4.0);
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        let (assign59960_e93509,) = {
    if (((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) && (!s.b[2462])) && s.b[2463]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59960_e93509;

        s.b[2464] = (2.0 == 8.0);
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        let (assign59980_e93538,) = {
    if ((((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) && (!s.b[2462])) && (!s.b[2463])) && s.b[2464]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59980_e93538;

        let (assign59990_e93553,) = {
    if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign59990_e93553;

        let mut assign60000_loop_guard: usize = 0;
        while {
            let assign60000_cond_e93569: f64 = if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60000_cond_e93569 != 0.0
        } {
            assign60000_loop_guard += 1;
            assert!(assign60000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) {
                s.store_sqrt(726, 726);
            }
            let (assign60000_body1_e93602,) = {
    if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && s.b[2460]) {
        let assign60000_body1_e93600: f64 = (s.v[719] + 1.0);
        (assign60000_body1_e93600,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign60000_body1_e93602;
        }

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) && (!s.b[2460])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && s.b[2459]) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && (!s.b[2459])) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2458]) && (!s.b[2459])) {
            s.store_scalar(334, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
    ) {
        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) {
            s.copy_ad(335, 684);
            s.store_sqrt_sub(342, 91, 1431);
            s.store_mul(171, 335, 342);
            s.store_div_scaled_inputs_indices(343, 335, 0.5, 342, 1.0);
            s.store_div_from_scalar(334, 1.0, 171);
            s.store_mul(335, 238, 334);
            s.store_scale(336, 335, s.v[509]);
            s.store_scale(337, 334, s.v[509]);
            s.store_add_scaled_product_indices(339, 336, 1.0, 508, 166, 1.0);
            s.store_div_from_scalar(335, 1.0, 339);
            s.store_scale(338, 335, 1.034943e-10);
            s.store_scalar(335, (1.0 - s.v[507]));
            s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 87, s.v[507], 335, 91, 1.0);
        }

        s.b[2465] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign60280_e94064,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign60280_e94064;

        let (assign60290_e94078,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60290_e94078;

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2466] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        s.b[2467] = (2.0 == 1.0);
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        let (assign60400_e94236,) = {
    if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && s.b[2467]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60400_e94236;

        s.b[2468] = (2.0 == 2.0);
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        let (assign60420_e94260,) = {
    if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (!s.b[2467])) && s.b[2468]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60420_e94260;

        s.b[2469] = (2.0 == 4.0);
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        let (assign60440_e94287,) = {
    if (((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (!s.b[2467])) && (!s.b[2468])) && s.b[2469]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60440_e94287;

        s.b[2470] = (2.0 == 8.0);
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        let (assign60460_e94317,) = {
    if ((((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (!s.b[2467])) && (!s.b[2468])) && (!s.b[2469])) && s.b[2470]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60460_e94317;

        let (assign60470_e94333,) = {
    if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign60470_e94333;

        let mut assign60480_loop_guard: usize = 0;
        while {
            let assign60480_cond_e94350: f64 = if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60480_cond_e94350 != 0.0
        } {
            assign60480_loop_guard += 1;
            assert!(assign60480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) {
                s.store_sqrt(726, 726);
            }
            let (assign60480_body1_e94385,) = {
    if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && s.b[2466]) {
        let assign60480_body1_e94383: f64 = (s.v[719] + 1.0);
        (assign60480_body1_e94383,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign60480_body1_e94385;
        }

        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) && (!s.b[2466])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && s.b[2465]) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2465])) {
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2465])) {
            s.store_scalar(334, 1.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2458])) {
            s.store_sub(340, 168, 91);
            s.store_mul(337, 154, 238);
            s.store_div_from_scalar(335, 1.0, 337);
            s.store_mul_ad_product_lhs_mixed_ai(339, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);
            s.store_mul(336, 339, 154);
            s.store_scale(344, 166, 9662367879.197212);
            s.store_scalar(335, 100000.0);
            s.store_div_from_scalar(336, 1.0, 162);
            s.store_mul_ad_lhs(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);
            s.store_mul(337, 336, 338);
            s.store_mul(341, 345, 338);
            s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));
            s.store_mul3_affine_lhs(335, 344, 338, 8.0, 0.0, 338);
            s.store_scaled_mul(336, 345, 338, 2.0);
            s.store_mul3_lhs(342, 345, 338, 338);
            s.store_sqrt_square_add(343, 341, 342);
            s.store_scaled_sub(169, 343, 341, 0.5);
            s.copy_ad(335, 169);
            s.store_mul(169, 208, 335);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
            s.store_scale(169, 169, s.v[619]);
            s.store_add(335, 85, 155);
            s.store_add_scaled_product_indices(336, 269, (-1.0), 335, 267, 1.0);
            s.store_mul_ad_rhs(240, 209, A::add_scaled_products(s.ad_value(209), A::add_scaled_sub_value_product(1.5, A::offset(s.ad_value(99), 1.0), 1.0, s.ad_value(154), s.ad_value(94), (-0.5)), 1.0, s.ad_value(185), s.ad_value(336), 1.0));
            s.copy_ad(335, 154);
            s.store_div_scaled_product_indices(131, 335, 240, 1.0, 250, 1.0);
            s.store_scale(335, 212, 2.0);
            s.store_mul_sub_rhs(241, 335, 267, 100);
            s.store_scaled_sub(336, 267, 100, 2.0);
            s.store_add(126, 94, 241);
            s.store_div_from_scalar(335, 1.0, 127);
            s.store_mul(336, 126, 335);
            s.store_sub_from_scalar(337, 1.0, 336);
            s.store_sub_from_scalar(332, 1.0, 337);
            s.store_square(722, 332);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign60940_e95068,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign60940_e95068;

        let (assign60950_e95077,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60950_e95077;

        if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
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

        s.b[2471] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        s.b[2472] = (4.0 == 1.0);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        let (assign61100_e95234,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && s.b[2472]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61100_e95234;

        s.b[2473] = (4.0 == 2.0);
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        let (assign61120_e95253,) = {
    if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (!s.b[2472])) && s.b[2473]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61120_e95253;

        s.b[2474] = (4.0 == 4.0);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

        let (assign61140_e95275,) = {
    if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) && s.b[2474]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61140_e95275;

        s.b[2475] = (4.0 == 8.0);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        let (assign61160_e95300,) = {
    if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) && (!s.b[2474])) && s.b[2475]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61160_e95300;

    }

    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign61170_e95311,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign61170_e95311;

        let mut assign61180_loop_guard: usize = 0;
        while {
            let assign61180_cond_e95323: f64 = if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61180_cond_e95323 != 0.0
        } {
            assign61180_loop_guard += 1;
            assert!(assign61180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) {
                s.store_sqrt(726, 726);
            }
            let (assign61180_body1_e95348,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2471]) {
        let assign61180_body1_e95346: f64 = (s.v[719] + 1.0);
        (assign61180_body1_e95346,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign61180_body1_e95348;
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2471])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(338, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(125, 1.0, 333);
            s.store_offset_mul_offset_rhs(242, 125, 125, 1.0, 1.0);
        }

        s.b[2476] = (((1.0 + s.v[125]) < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
            s.store_sub_from_scalar_ad(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), A::offset(s.ad_value(125), 1.0));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign61310_e95537,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign61310_e95537;

        let (assign61320_e95548,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61320_e95548;

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2477] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        s.b[2478] = (2.0 == 1.0);
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        let (assign61430_e95679,) = {
    if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && s.b[2478]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61430_e95679;

        s.b[2479] = (2.0 == 2.0);
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        let (assign61450_e95700,) = {
    if (((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (!s.b[2478])) && s.b[2479]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61450_e95700;

        s.b[2480] = (2.0 == 4.0);
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

        let (assign61470_e95724,) = {
    if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (!s.b[2478])) && (!s.b[2479])) && s.b[2480]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61470_e95724;

        s.b[2481] = (2.0 == 8.0);
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        let (assign61490_e95751,) = {
    if (((((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (!s.b[2478])) && (!s.b[2479])) && (!s.b[2480])) && s.b[2481]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61490_e95751;

        let (assign61500_e95764,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign61500_e95764;

        let mut assign61510_loop_guard: usize = 0;
        while {
            let assign61510_cond_e95778: f64 = if ((((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61510_cond_e95778 != 0.0
        } {
            assign61510_loop_guard += 1;
            assert!(assign61510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) {
                s.store_sqrt(726, 726);
            }
            let (assign61510_body1_e95807,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && s.b[2477]) {
        let assign61510_body1_e95805: f64 = (s.v[719] + 1.0);
        (assign61510_body1_e95805,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign61510_body1_e95807;
        }

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) && (!s.b[2477])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(243, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2476]) {
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && (!s.b[2476])) {
            s.store_offset(243, 125, 1.0);
            s.store_scalar(334, 1.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2457]) {
            s.store_div_scaled_product_indices(335, 127, 242, 0.6666666666666667, 243, 1.0);
            s.store_mul(133, 335, 185);
            s.store_offset(244, 125, 0.5);
            s.store_mul(245, 243, 242);
            s.store_div_scaled_inputs_indices(246, 244, 0.4, 245, 1.0);
            s.store_sub_from_scalar(247, 0.6, 246);
        }

        s.b[2482] = (s.v[247] > 0.5);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2482]) {
            s.store_scalar(247, 0.5);
        }

        s.b[2483] = (s.v[347] == 2.0);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) {
            s.copy_ad(335, 131);
            s.store_add_scaled_product_value_ad(131, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(239)), 1.0, 207, 131, 1.0);
        }

        s.b[2484] = (s.v[131] < 0.0);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) && s.b[2484]) {
            s.store_scalar(131, 0.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) {
            s.copy_ad(335, 133);
            s.store_add_scaled_product_value_ad(133, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(238)), 1.0, 207, 133, 1.0);
        }

        s.b[2485] = (s.v[133] < 0.0);
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) && s.b[2485]) {
            s.store_scalar(133, 0.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2457]) && s.b[2483]) {
            s.copy_ad(335, 247);
            s.store_add_scaled_product_value_ad(247, A::scale_offset(s.ad_value(207), (-0.5), 0.5), 1.0, 207, 247, 1.0);
            s.copy_ad(335, 169);
            s.store_mul(169, 207, 169);
        }

        let (assign61810_e96181,) = {
    if (((!s.b[1439]) && s.b[2426]) && (s.v[948] != 0.0)) {
        (0.0,)
    } else {
        (s.v[948],)
    }
};
        s.v[948] = assign61810_e96181;

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_sub(170, 162, 169);
        }

        s.b[2486] = (s.v[170] < 1e-9);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2486]) {
            s.store_scalar(170, 1e-9);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_scalar(335, (s.v[625] / 100.0));
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_mul_ad(338, A::sub(s.ad_value(91), s.ad_value(87)), s.ad_value(334), 1.0);
            s.store_add_scaled_products_indices(339, 335, 131, 1.0, 336, 133, 1.0);
            s.store_div(337, 339, 338);
            s.store_mul_scale_offset_rhs(251, 337, 1434, p.p166, 1.0);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(342, 339, 251);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), s.v[474])), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_square(337, 335);
            s.store_mul_neg_lhs(338, 154, 337);
            s.store_mul(339, 338, 170);
            s.store_mul_offset_rhs(340, 338, 238, 1e-25);
            s.store_mul_ad_product_lhs_mixed_ai(333, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
            s.store_div_scaled_inputs_indices(337, 336, -1.0, 254, 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2487] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2487]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2488] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2487])) && s.b[2488]) {
            s.copy_ad(337, 335);
        }

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2487])) && (!s.b[2488])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2489] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2489]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2490] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2489])) && s.b[2490]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2489])) && (!s.b[2490])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && (!s.b[2489])) && (!s.b[2490])) {
            s.store_mul(339, 338, 340);
        }

        if ((!s.b[1439]) && s.b[2426]) {
            s.store_mul(253, 254, 339);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_div_scaled_inputs_indices(335, 115, -1.0, 170, 1.0);
            s.store_mul3_lhs(135, 115, 248, 253);
        }

        s.b[2491] = (p.p283 != 0.0);
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2491]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
            s.store_scale(336, 336, 0.5);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(87), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2492] = (s.v[336] < 0.0);
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2491]) && s.b[2492]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2491]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1435, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 87, 1.0, 340, 1.0, 1434, -1.0);
            s.store_add_product3_rhs_indices(338, 338, 1435, 334, 339, 1.0);
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2491])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2493] = (p.p287 != 0.0);
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2493]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1435);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2493])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2494] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2494]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_mul3_lhs(45, 115, 249, 253);
            s.store_add(135, 135, 45);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2494])) {
            s.store_scalar(45, 0.0);
        }

        s.b[2495] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        s.b[2496] = (p.p296 > 0.0);
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2496]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && (!s.b[2496])) {
            s.copy_ad(341, 647);
        }

        s.b[2497] = (s.v[793] >= 0.0);
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2497]) {
            s.copy_ad(369, 793);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && (!s.b[2497])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2498] = (s.v[369] < (20.0 * 1e-12));
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && s.b[2498]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2495]) && (!s.b[2498])) {
            s.store_powf_offset_input(335, 369, 1e-12, p.p297);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2495]) {
            s.store_powf_offset_input(343, 369, 1e-12, p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if (((!s.b[1439]) && s.b[2426]) && (!s.b[2495])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        s.b[2499] = (p.p27 != 0.0);
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_scale(335, 186, 1.034943e-10);
            s.copy_ad(336, 684);
            s.store_scalar(337, (s.v[628] - p.p139));
            s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));
            s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);
            s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));
            s.store_scalar(338, s.v[496]);
            s.store_scalar(340, s.v[497]);
            s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1435, 1.0);
            s.store_mul(137, 121, 335);
            s.store_sub_from_scalar_scaled_input(335, s.v[498], 790, p.p213);
            s.store_add_scaled_inputs3_offset_indices(138, 1436, 1.0, 335, 1.0, 137, 1.0, (-s.v[160]));
            s.store_mul3_lhs(141, 694, 186, 186);
            s.store_scaled_mul(142, 141, 154, 0.5);
            s.store_scaled_mul(143, 142, 154, 2.0);
            s.store_scale(345, 154, 0.25);
            s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);
            s.store_offset_sub(335, 1436, 344, (-0.005));
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_sqrt_add_scaled_square_product(336, 335, 1.0, 334, 344, (4.0 * 0.005));
            s.store_sub_ad_lhs(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1434);
            s.store_offset_mul(338, 154, 337, (-1.0));
            s.store_div_from_scalar(339, 4.0, 143);
            s.store_offset_mul(335, 338, 339, 1.0);
            s.store_mul(340, 154, 339);
            s.store_mul(341, 338, 339);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2500] = (s.v[335] < 0.0);
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2500]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(144, 335);
            s.store_mul_sub_from_scalar_rhs(334, 142, 1.0, 144);
            s.store_add(146, 138, 334);
            s.store_div_from_scalar_add_ad(334, 1.0, s.ad_value(154), A::div_scalar_offset_denominator(2.0, s.ad_value(138), 1e-25, 1.0));
            s.store_mul_ln_ad_lhs(147, A::mul(A::div_scalar_by_product(1.0, s.ad_value(140), s.ad_value(141), 1.0), A::square(s.ad_value(138))), 334);
            s.store_offset_sub(148, 147, 146, (-0.002));
        }

    }
}
