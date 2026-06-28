#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign48920_loop_guard: usize = 0;
        while {
            let assign48920_cond_e70112: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            assign48920_cond_e70112 != 0.0
        } {
            assign48920_loop_guard += 1;
            assert!(assign48920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul_sub_ad_rhs(2093, 2127, A::add_scaled_product(s.ad_value(2115), 1.0, s.ad_value(2128), s.ad_value(2091), 1.0), s.ad_value(2089));
                s.store_sub(335, 2091, 2093);
            }
            s.b[2251] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign48920_body8_e70263,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48920_body8_e70263;
            let (assign48920_body9_e70279,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body9_e70279;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2252] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };
            s.b[2253] = (2.0 == 1.0);
            s.v[2253] = if s.b[2253] { 1.0 } else { 0.0 };
            let (assign48920_body20_e70455,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && s.b[2253]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body20_e70455;
            s.b[2254] = (2.0 == 2.0);
            s.v[2254] = if s.b[2254] { 1.0 } else { 0.0 };
            let (assign48920_body22_e70481,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && s.b[2254]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body22_e70481;
            s.b[2255] = (2.0 == 4.0);
            s.v[2255] = if s.b[2255] { 1.0 } else { 0.0 };
            let (assign48920_body24_e70510,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && (!s.b[2254])) && s.b[2255]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body24_e70510;
            s.b[2256] = (2.0 == 8.0);
            s.v[2256] = if s.b[2256] { 1.0 } else { 0.0 };
            let (assign48920_body26_e70542,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && (!s.b[2254])) && (!s.b[2255])) && s.b[2256]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body26_e70542;
            let (assign48920_body27_e70560,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48920_body27_e70560;
            let mut assign48920_body28_loop_guard: usize = 0;
            while {
                let assign48920_body28_cond_e70579: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body28_cond_e70579 != 0.0
            } {
                assign48920_body28_loop_guard += 1;
                assert!(assign48920_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) {
                    s.store_sqrt(726, 726);
                }
                let (assign48920_body28_body1_e70618,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) {
        let assign48920_body28_body1_e70616: f64 = (s.v[719] + 1.0);
        (assign48920_body28_body1_e70616,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign48920_body28_body1_e70618;
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && (!s.b[2252])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2251])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2251])) {
                s.store_scalar(336, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_sqrt_mul(2084, 2134, 335);
            }
            s.b[2257] = ((s.v[2084] > (s.v[2129] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2257] = if s.b[2257] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
                s.store_offset_sub(781, 2084, 2129, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign48920_body44_e70908,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48920_body44_e70908;
            let (assign48920_body45_e70924,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body45_e70924;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2258] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2258] = if s.b[2258] { 1.0 } else { 0.0 };
            s.b[2259] = (2.0 == 1.0);
            s.v[2259] = if s.b[2259] { 1.0 } else { 0.0 };
            let (assign48920_body56_e71100,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && s.b[2259]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body56_e71100;
            s.b[2260] = (2.0 == 2.0);
            s.v[2260] = if s.b[2260] { 1.0 } else { 0.0 };
            let (assign48920_body58_e71126,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && s.b[2260]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body58_e71126;
            s.b[2261] = (2.0 == 4.0);
            s.v[2261] = if s.b[2261] { 1.0 } else { 0.0 };
            let (assign48920_body60_e71155,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && (!s.b[2260])) && s.b[2261]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body60_e71155;
            s.b[2262] = (2.0 == 8.0);
            s.v[2262] = if s.b[2262] { 1.0 } else { 0.0 };
            let (assign48920_body62_e71187,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && (!s.b[2260])) && (!s.b[2261])) && s.b[2262]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body62_e71187;
            let (assign48920_body63_e71205,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48920_body63_e71205;
            let mut assign48920_body64_loop_guard: usize = 0;
            while {
                let assign48920_body64_cond_e71224: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body64_cond_e71224 != 0.0
            } {
                assign48920_body64_loop_guard += 1;
                assert!(assign48920_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) {
                    s.store_sqrt(726, 726);
                }
                let (assign48920_body64_body1_e71263,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) {
        let assign48920_body64_body1_e71261: f64 = (s.v[719] + 1.0);
        (assign48920_body64_body1_e71261,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign48920_body64_body1_e71263;
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && (!s.b[2258])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_offset_lhs(2084, 2129, (-1e-12), 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2257])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2257])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul(337, 336, 337);
                s.store_add_div_rhs_mixed_ai(2135, 2088, A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2084), A::sub_scaled_inputs(s.ad_value(2084), 1.0, s.ad_value(2129), 2.0), 1.0), 2134);
                s.store_scalar(2136, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2084)), s.ad_value(337), (-1.0)), 1.0, 2139);
            }
            s.b[2263] = ((s.v[2135] > (s.v[2086] - p.p406)) && (p.p406 >= 0.0));
            s.v[2263] = if s.b[2263] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
                s.store_offset_sub(781, 2135, 2086, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign48920_body83_e71618,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48920_body83_e71618;
            let (assign48920_body84_e71634,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body84_e71634;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
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
            s.b[2264] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2264] = if s.b[2264] { 1.0 } else { 0.0 };
            s.b[2265] = (4.0 == 1.0);
            s.v[2265] = if s.b[2265] { 1.0 } else { 0.0 };
            let (assign48920_body99_e71882,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && s.b[2265]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body99_e71882;
            s.b[2266] = (4.0 == 2.0);
            s.v[2266] = if s.b[2266] { 1.0 } else { 0.0 };
            let (assign48920_body101_e71908,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && s.b[2266]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body101_e71908;
            s.b[2267] = (4.0 == 4.0);
            s.v[2267] = if s.b[2267] { 1.0 } else { 0.0 };
            let (assign48920_body103_e71937,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && (!s.b[2266])) && s.b[2267]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body103_e71937;
            s.b[2268] = (4.0 == 8.0);
            s.v[2268] = if s.b[2268] { 1.0 } else { 0.0 };
            let (assign48920_body105_e71969,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && (!s.b[2266])) && (!s.b[2267])) && s.b[2268]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign48920_body105_e71969;
            let (assign48920_body106_e71987,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48920_body106_e71987;
            let mut assign48920_body107_loop_guard: usize = 0;
            while {
                let assign48920_body107_cond_e72006: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body107_cond_e72006 != 0.0
            } {
                assign48920_body107_loop_guard += 1;
                assert!(assign48920_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) {
                    s.store_sqrt(726, 726);
                }
                let (assign48920_body107_body1_e72045,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) {
        let assign48920_body107_body1_e72043: f64 = (s.v[719] + 1.0);
        (assign48920_body107_body1_e72043,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign48920_body107_body1_e72045;
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && (!s.b[2264])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_offset_lhs(2135, 2086, (-p.p406), 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2263])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2263])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul(2136, 2136, 334);
                s.store_mul(2137, 2137, 334);
                s.store_mul_sub_rhs(339, 154, 2088, 2091);
                s.store_exp(340, 339);
                s.store_sub_offset_lhs(344, 340, (-1.0), 339);
            }
            s.b[2269] = (s.v[339] >= 1e-7);
            s.v[2269] = if s.b[2269] { 1.0 } else { 0.0 };
            let (assign48920_body122_e72308,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2269]) {
        let assign48920_body122_e72306: f64 = (-1.0);
        (assign48920_body122_e72306,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign48920_body122_e72308;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2269]) {
                s.store_mul_scaled_sqrt_rhs(2097, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2097, 1.0);
                s.store_mul_offset_rhs(2124, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2126, 345, 1.0, 340);
            }
            s.b[2270] = (s.v[339] < (-1e-7));
            s.v[2270] = if s.b[2270] { 1.0 } else { 0.0 };
            let (assign48920_body128_e72416,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && s.b[2270]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign48920_body128_e72416;
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && s.b[2270]) {
                s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2088), 1.0, s.ad_value(2115), p.p398));
                s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2091), 1.0, s.ad_value(2115), p.p398));
                s.store_mul_sqrt_ad_rhs(2097, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2097, 1.0);
                s.store_mul_add_ad_rhs(2124, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2126, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2271] = (s.v[339] > 0.0);
            s.v[2271] = if s.b[2271] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && (!s.b[2270])) && s.b[2271]) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2097, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2124, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2126, 2124);
            }
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && (!s.b[2270])) && (!s.b[2271])) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2097, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2124, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2126, 2124);
            }
            let (assign48920_body146_e72883,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] != 0.0)) {
        let assign48920_body146_e72881: f64 = (150.0 + 1.0);
        (assign48920_body146_e72881,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign48920_body146_e72883;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2098, 2097, 1.0, 185, 85, 2088, 1.0);
                s.store_sub(2099, 2124, 185);
                s.copy_ad(2100, 2126);
                s.store_sub(2101, 2091, 2135);
                s.store_neg(2102, 2136);
                s.store_sub_from_scalar(2103, 1.0, 2137);
                s.store_add_scaled_products_indices(2104, 2099, 2103, 1.0, 2100, 2102, (-1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                if (s.v[2104] > 0.0) {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, (-1e-25));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.copy_ad(2106, 2103);
                s.store_neg(2107, 2100);
                s.store_neg(2108, 2102);
                s.copy_ad(2109, 2099);
                s.store_mul_add_scaled_products_indices_rhs(2110, 2105, 2106, 2098, -1.0, 2107, 2101, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(2111, 2105, 2108, 2098, -1.0, 2109, 2101, -1.0);
                s.store_abs(335, 2110);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2111]) as f64).abs()) {
                    s.store_abs(335, 2111);
                } else {
                }
            }
            s.b[2272] = (s.v[335] > 0.1);
            s.v[2272] = if s.b[2272] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) && s.b[2272]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2273] = (s.v[335] < 1e-10);
            s.v[2273] = if s.b[2273] { 1.0 } else { 0.0 };
            let (assign48920_body167_e73287,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) && s.b[2273]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign48920_body167_e73287;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_add(2088, 2088, 2110);
                s.store_add(2091, 2091, 2111);
            }
            let (assign48920_body170_e73341,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
        let assign48920_body170_e73339: f64 = (s.v[98] + 1.0);
        (assign48920_body170_e73339,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign48920_body170_e73341;
        }

    }

    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_mul_sub_rhs(339, 154, 2088, 2091);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            if (s.v[2088] > s.v[2091]) {
                s.store_mul_scaled_sqrt_rhs(2121, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2121, 209, 344);
            }
        }

        s.b[2275] = (1.0 == 1.0);
        s.v[2275] = if s.b[2275] { 1.0 } else { 0.0 };

        s.b[2276] = (((s.v[2088] - s.v[2086]) < p.p403) && (p.p403 >= 0.0));
        s.v[2276] = if s.b[2276] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2088), s.ad_value(2086)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign49050_e73555,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49050_e73555;

        let (assign49060_e73573,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49060_e73573;

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
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

        s.b[2277] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2277] = if s.b[2277] { 1.0 } else { 0.0 };

        s.b[2278] = (6.0 == 1.0);
        s.v[2278] = if s.b[2278] { 1.0 } else { 0.0 };

        let (assign49250_e73927,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && s.b[2278]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49250_e73927;

        s.b[2279] = (6.0 == 2.0);
        s.v[2279] = if s.b[2279] { 1.0 } else { 0.0 };

        let (assign49270_e73955,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && s.b[2279]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49270_e73955;

        s.b[2280] = (6.0 == 4.0);
        s.v[2280] = if s.b[2280] { 1.0 } else { 0.0 };

        let (assign49290_e73986,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && (!s.b[2279])) && s.b[2280]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49290_e73986;

        s.b[2281] = (6.0 == 8.0);
        s.v[2281] = if s.b[2281] { 1.0 } else { 0.0 };

        let (assign49310_e74020,) = {
    if (((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && (!s.b[2279])) && (!s.b[2280])) && s.b[2281]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49310_e74020;

        let (assign49320_e74040,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49320_e74040;

        let mut assign49330_loop_guard: usize = 0;
        while {
            let assign49330_cond_e74061: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49330_cond_e74061 != 0.0
        } {
            assign49330_loop_guard += 1;
            assert!(assign49330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) {
                s.store_sqrt(726, 726);
            }
            let (assign49330_body1_e74104,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) {
        let assign49330_body1_e74102: f64 = (s.v[719] + 1.0);
        (assign49330_body1_e74102,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign49330_body1_e74104;
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && (!s.b[2277])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && (!s.b[2276])) {
            s.store_sub(336, 2088, 2086);
            s.store_scalar(334, 1.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2117, 209, -1.0, 338);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2275])) {
            s.copy_ad(2117, 2121);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(87, 2087);
            s.copy_ad(91, 2088);
            s.store_sub(94, 2088, 2087);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[2282] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2282] = if s.b[2282] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign49600_e74645,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49600_e74645;

        let (assign49610_e74658,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49610_e74658;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2283] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2283] = if s.b[2283] { 1.0 } else { 0.0 };

        s.b[2284] = (2.0 == 1.0);
        s.v[2284] = if s.b[2284] { 1.0 } else { 0.0 };

        let (assign49720_e74807,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && s.b[2284]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49720_e74807;

        s.b[2285] = (2.0 == 2.0);
        s.v[2285] = if s.b[2285] { 1.0 } else { 0.0 };

        let (assign49740_e74830,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && s.b[2285]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49740_e74830;

        s.b[2286] = (2.0 == 4.0);
        s.v[2286] = if s.b[2286] { 1.0 } else { 0.0 };

        let (assign49760_e74856,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && (!s.b[2285])) && s.b[2286]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49760_e74856;

        s.b[2287] = (2.0 == 8.0);
        s.v[2287] = if s.b[2287] { 1.0 } else { 0.0 };

        let (assign49780_e74885,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && (!s.b[2285])) && (!s.b[2286])) && s.b[2287]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49780_e74885;

        let (assign49790_e74900,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49790_e74900;

        let mut assign49800_loop_guard: usize = 0;
        while {
            let assign49800_cond_e74916: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49800_cond_e74916 != 0.0
        } {
            assign49800_loop_guard += 1;
            assert!(assign49800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) {
                s.store_sqrt(726, 726);
            }
            let (assign49800_body1_e74949,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) {
        let assign49800_body1_e74947: f64 = (s.v[719] + 1.0);
        (assign49800_body1_e74947,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign49800_body1_e74949;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && (!s.b[2283])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2282])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2282])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_add(109, 87, 110);
        }

        s.b[2288] = (((s.v[109] - s.v[2085]) < p.p403) && (p.p403 >= 0.0));
        s.v[2288] = if s.b[2288] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(109), s.ad_value(2085)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
            s.store_scalar(725, 1.0);
        }

        let (assign49960_e75205,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign49960_e75205;

        let (assign49970_e75218,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign49970_e75218;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
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

        s.b[2289] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2289] = if s.b[2289] { 1.0 } else { 0.0 };

        s.b[2290] = (6.0 == 1.0);
        s.v[2290] = if s.b[2290] { 1.0 } else { 0.0 };

        let (assign50160_e75487,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && s.b[2290]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50160_e75487;

        s.b[2291] = (6.0 == 2.0);
        s.v[2291] = if s.b[2291] { 1.0 } else { 0.0 };

        let (assign50180_e75510,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && s.b[2291]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50180_e75510;

        s.b[2292] = (6.0 == 4.0);
        s.v[2292] = if s.b[2292] { 1.0 } else { 0.0 };

        let (assign50200_e75536,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && (!s.b[2291])) && s.b[2292]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50200_e75536;

        s.b[2293] = (6.0 == 8.0);
        s.v[2293] = if s.b[2293] { 1.0 } else { 0.0 };

        let (assign50220_e75565,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && (!s.b[2291])) && (!s.b[2292])) && s.b[2293]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50220_e75565;

        let (assign50230_e75580,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50230_e75580;

        let mut assign50240_loop_guard: usize = 0;
        while {
            let assign50240_cond_e75596: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50240_cond_e75596 != 0.0
        } {
            assign50240_loop_guard += 1;
            assert!(assign50240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) {
                s.store_sqrt(726, 726);
            }
            let (assign50240_body1_e75629,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) {
        let assign50240_body1_e75627: f64 = (s.v[719] + 1.0);
        (assign50240_body1_e75627,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign50240_body1_e75629;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && (!s.b[2289])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2288])) {
            s.store_sub(336, 109, 2085);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2118, 209, -1.0, 338);
            s.store_sqrt_offset_ad(782, A::mul_scaled_lhs(A::add(s.ad_value(2117), s.ad_value(2116)), 1.0, A::add(s.ad_value(2117), s.ad_value(2116))), ((4.0 * (1e-12 * 1e-6)) * (1e-12 * 1e-6)));
            s.store_scaled_offset_ad(335, A::div_scaled_inputs2(s.ad_value(2117), -1.0, s.ad_value(2116), -1.0, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_add_scaled_inputs3_indices(2119, 2117, (-0.5), 2116, (-0.5), 782, 0.5);
        }

        s.b[2294] = (s.v[2119] < 0.0);
        s.v[2294] = if s.b[2294] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2294]) {
            s.store_scalar(2119, 0.0);
            s.store_scalar(335, 0.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_neg(2119, 2119);
            s.store_mul3_affine_lhs(248, 154, 2119, (-1.0 / (2.0)), 0.0, 94);
            s.store_neg(238, 2118);
            s.copy_ad(170, 162);
            s.copy_ad(790, 349);
        }

        s.b[2295] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2295] = if s.b[2295] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2296] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2296] = if s.b[2296] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign50550_e76131,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50550_e76131;

        let (assign50560_e76146,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50560_e76146;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2297] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2297] = if s.b[2297] { 1.0 } else { 0.0 };

        s.b[2298] = (2.0 == 1.0);
        s.v[2298] = if s.b[2298] { 1.0 } else { 0.0 };

        let (assign50670_e76313,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && s.b[2298]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50670_e76313;

        s.b[2299] = (2.0 == 2.0);
        s.v[2299] = if s.b[2299] { 1.0 } else { 0.0 };

        let (assign50690_e76338,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && s.b[2299]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50690_e76338;

        s.b[2300] = (2.0 == 4.0);
        s.v[2300] = if s.b[2300] { 1.0 } else { 0.0 };

        let (assign50710_e76366,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && (!s.b[2299])) && s.b[2300]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50710_e76366;

        s.b[2301] = (2.0 == 8.0);
        s.v[2301] = if s.b[2301] { 1.0 } else { 0.0 };

        let (assign50730_e76397,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && (!s.b[2299])) && (!s.b[2300])) && s.b[2301]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50730_e76397;

        let (assign50740_e76414,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50740_e76414;

        let mut assign50750_loop_guard: usize = 0;
        while {
            let assign50750_cond_e76432: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50750_cond_e76432 != 0.0
        } {
            assign50750_loop_guard += 1;
            assert!(assign50750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) {
                s.store_sqrt(726, 726);
            }
            let (assign50750_body1_e76469,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) {
        let assign50750_body1_e76467: f64 = (s.v[719] + 1.0);
        (assign50750_body1_e76467,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign50750_body1_e76469;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && (!s.b[2297])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && (!s.b[2296])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && (!s.b[2296])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) {
            s.store_sub(342, 91, 2115);
        }

        s.b[2302] = ((s.v[342] < (0.2 + ((-s.v[2115]) + 0.8))) && (((-s.v[2115]) + 0.8) >= 0.0));
        s.v[2302] = if s.b[2302] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
            s.store_sub_offset_ad_lhs(781, A::sub_from_scalar(0.8, s.ad_value(2115)), 0.2, 342);
            s.store_square(722, 781);
            s.store_square_ad(723, A::sub_from_scalar(0.8, s.ad_value(2115)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign50910_e76777,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign50910_e76777;

        let (assign50920_e76793,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign50920_e76793;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2303] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2303] = if s.b[2303] { 1.0 } else { 0.0 };

        s.b[2304] = (1.0 == 1.0);
        s.v[2304] = if s.b[2304] { 1.0 } else { 0.0 };

        let (assign51010_e76933,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && s.b[2304]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51010_e76933;

        s.b[2305] = (1.0 == 2.0);
        s.v[2305] = if s.b[2305] { 1.0 } else { 0.0 };

        let (assign51030_e76959,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && s.b[2305]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51030_e76959;

        s.b[2306] = (1.0 == 4.0);
        s.v[2306] = if s.b[2306] { 1.0 } else { 0.0 };

        let (assign51050_e76988,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && (!s.b[2305])) && s.b[2306]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51050_e76988;

        s.b[2307] = (1.0 == 8.0);
        s.v[2307] = if s.b[2307] { 1.0 } else { 0.0 };

        let (assign51070_e77020,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && (!s.b[2305])) && (!s.b[2306])) && s.b[2307]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51070_e77020;

        let (assign51080_e77038,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign51080_e77038;

        let mut assign51090_loop_guard: usize = 0;
        while {
            let assign51090_cond_e77057: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51090_cond_e77057 != 0.0
        } {
            assign51090_loop_guard += 1;
            assert!(assign51090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) {
                s.store_sqrt(726, 726);
            }
            let (assign51090_body1_e77096,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) {
        let assign51090_body1_e77094: f64 = (s.v[719] + 1.0);
        (assign51090_body1_e77094,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign51090_body1_e77096;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && (!s.b[2303])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul_ad_lhs(780, A::mul_sub_from_scalar_rhs(s.ad_value(781), 0.8, s.ad_value(2115)), 726);
            s.store_div_scaled_product_left_ad(334, A::mul_sub_from_scalar_lhs(0.8, s.ad_value(2115), s.ad_value(725)), 726, 1.0, 770, 1.0);
            s.store_sub_offset_ad_lhs(342, A::sub_from_scalar(0.8, s.ad_value(2115)), 0.2, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2302])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2302])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) {
            s.store_mul(343, 2134, 342);
            s.store_sqrt(171, 343);
            s.store_div_from_scalar(334, 1.0, 171);
            s.store_mul(335, 238, 334);
            s.store_scale(336, 335, s.v[509]);
            s.store_scale(337, 334, s.v[509]);
            s.store_add_scaled_product_indices(339, 336, 1.0, 508, 2131, 1.0);
            s.store_div_from_scalar(335, 1.0, 339);
            s.store_scale(338, 335, 1.034943e-10);
            s.store_scalar(335, (1.0 - s.v[507]));
            s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 109, s.v[507], 335, 91, 1.0);
        }

        s.b[2308] = ((s.v[168] > (((s.v[109] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2308] = if s.b[2308] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 109, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign51350_e77583,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign51350_e77583;

        let (assign51360_e77599,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51360_e77599;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2309] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2309] = if s.b[2309] { 1.0 } else { 0.0 };

        s.b[2310] = (2.0 == 1.0);
        s.v[2310] = if s.b[2310] { 1.0 } else { 0.0 };

        let (assign51470_e77775,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && s.b[2310]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51470_e77775;

        s.b[2311] = (2.0 == 2.0);
        s.v[2311] = if s.b[2311] { 1.0 } else { 0.0 };

        let (assign51490_e77801,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && s.b[2311]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51490_e77801;

        s.b[2312] = (2.0 == 4.0);
        s.v[2312] = if s.b[2312] { 1.0 } else { 0.0 };

        let (assign51510_e77830,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && (!s.b[2311])) && s.b[2312]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51510_e77830;

        s.b[2313] = (2.0 == 8.0);
        s.v[2313] = if s.b[2313] { 1.0 } else { 0.0 };

        let (assign51530_e77862,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && (!s.b[2311])) && (!s.b[2312])) && s.b[2313]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign51530_e77862;

        let (assign51540_e77880,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign51540_e77880;

        let mut assign51550_loop_guard: usize = 0;
        while {
            let assign51550_cond_e77899: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51550_cond_e77899 != 0.0
        } {
            assign51550_loop_guard += 1;
            assert!(assign51550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) {
                s.store_sqrt(726, 726);
            }
            let (assign51550_body1_e77938,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) {
        let assign51550_body1_e77936: f64 = (s.v[719] + 1.0);
        (assign51550_body1_e77936,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign51550_body1_e77938;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && (!s.b[2309])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 109, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2308])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2308])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) {
            s.store_sub(340, 168, 91);
            s.store_mul(337, 154, 238);
            s.store_div_from_scalar(335, 1.0, 337);
            s.store_mul(339, 248, 335);
            s.store_scale(344, 2131, 9662367879.197212);
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

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
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

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 251, p.p160);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(251), s.ad_value(624));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
        }

    }

    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2314] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2314] = if s.b[2314] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2314]) {
            s.copy_ad(336, 335);
        }

        s.b[2315] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2315] = if s.b[2315] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2314])) && s.b[2315]) {
            s.store_square(336, 335);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2314])) && (!s.b[2315])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p178);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset(338, 336, 1.0);
        }

        s.b[2316] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2316] = if s.b[2316] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2316]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2317] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2317] = if s.b[2317] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2316])) && s.b[2317]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2316])) && (!s.b[2317])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 338, ((-1.0) / p.p178));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(2114, 255);
            s.store_scalar(2322, 0.0);
            s.store_scalar(2153, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(2145, 0.0);
            s.store_scalar(2320, 0.0);
            s.store_add_scaled_inputs3_offset_indices(2142, 1438, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));
        }

        s.b[2324] = (0.0 == 0.0);
        s.v[2324] = if s.b[2324] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2324]) {
            s.store_offset(2143, 2142, (-p.p393));
        }

        s.b[2325] = (0.0 == 1.0);
        s.v[2325] = if s.b[2325] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2324])) && s.b[2325]) {
            s.store_offset(2143, 1438, (((-s.v[160])) + ((-p.p393))));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2324])) && (!s.b[2325])) {
            s.store_offset(2143, 85, (-p.p393));
        }

        s.b[2326] = (((s.v[2146]) as f64).abs() <= 0.0);
        s.v[2326] = if s.b[2326] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2326]) {
            s.store_scalar(2151, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.copy_ad(983, 87);
            s.store_scale(2168, 2115, p.p399);
            s.store_scalar(2323, ((s.v[160] + p.p393) - 3.0));
        }

        s.b[2327] = (1.0 == 1.0);
        s.v[2327] = if s.b[2327] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2327]) {
            s.store_scale(2168, 2115, p.p399);
            s.store_offset(983, 2168, (-1.0));
            s.copy_ad(2322, 2323);
            s.copy_ad(2144, 2323);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2327])) {
            s.store_offset_scaled(2168, 2115, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2322, 2143);
            s.copy_ad(2144, 2143);
        }

        let (assign52430_e79322,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign52430_e79322;

        let (assign52440_e79336,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign52440_e79336;

        let mut assign52450_loop_guard: usize = 0;
        while {
            let assign52450_cond_e79351: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign52450_cond_e79351 != 0.0
        } {
            assign52450_loop_guard += 1;
            assert!(assign52450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2328] = (s.v[983] >= 0.0);
            s.v[2328] = if s.b[2328] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2328]) {
                s.store_mul_scaled_sqrt_ad_rhs(2320, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2123, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2320), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2328])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2168)));
                s.store_exp_mul(338, 154, 2168);
                s.store_mul_sqrt_ad_rhs(2320, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2320, 1.0);
                s.store_mul_add_ad_rhs(2123, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            let (assign52450_body10_e79585,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] != 0.0)) {
        let assign52450_body10_e79583: f64 = (150.0 + 1.0);
        (assign52450_body10_e79583,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign52450_body10_e79585;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2098, 2320, 1.0, 185, 2322, 983, 1.0);
                s.store_sub(2099, 2123, 185);
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2329] = (((s.v[2110]) as f64).abs() < (1e-10 * 100.0));
            s.v[2329] = if s.b[2329] { 1.0 } else { 0.0 };
            let (assign52450_body15_e79673,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && s.b[2329]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign52450_body15_e79673;
            s.b[2330] = (s.v[2110] > 0.1);
            s.v[2330] = if s.b[2330] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2329])) && s.b[2330]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2331] = (s.v[2110] < (-0.1));
            s.v[2331] = if s.b[2331] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2329])) && (!s.b[2330])) && s.b[2331]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2110);
            }
            let (assign52450_body21_e79763,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
        let assign52450_body21_e79761: f64 = (s.v[97] + 1.0);
        (assign52450_body21_e79761,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign52450_body21_e79763;
        }

        s.b[2333] = (1.0 == 1.0);
        s.v[2333] = if s.b[2333] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2333]) {
            s.copy_ad(2169, 983);
        }

        s.b[2334] = ((s.v[983] < (s.v[2169] + 0.2)) && (0.2 >= 0.0));
        s.v[2334] = if s.b[2334] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
            s.store_sub_offset_lhs(781, 2169, 0.2, 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign52550_e79916,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52550_e79916;

        let (assign52560_e79935,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52560_e79935;

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2335] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2335] = if s.b[2335] { 1.0 } else { 0.0 };

        s.b[2336] = (2.0 == 1.0);
        s.v[2336] = if s.b[2336] { 1.0 } else { 0.0 };

        let (assign52670_e80138,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && s.b[2336]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52670_e80138;

        s.b[2337] = (2.0 == 2.0);
        s.v[2337] = if s.b[2337] { 1.0 } else { 0.0 };

        let (assign52690_e80167,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && s.b[2337]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52690_e80167;

        s.b[2338] = (2.0 == 4.0);
        s.v[2338] = if s.b[2338] { 1.0 } else { 0.0 };

        let (assign52710_e80199,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && (!s.b[2337])) && s.b[2338]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52710_e80199;

        s.b[2339] = (2.0 == 8.0);
        s.v[2339] = if s.b[2339] { 1.0 } else { 0.0 };

        let (assign52730_e80234,) = {
    if (((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && (!s.b[2337])) && (!s.b[2338])) && s.b[2339]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52730_e80234;

        let (assign52740_e80255,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52740_e80255;

        let mut assign52750_loop_guard: usize = 0;
        while {
            let assign52750_cond_e80277: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign52750_cond_e80277 != 0.0
        } {
            assign52750_loop_guard += 1;
            assert!(assign52750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) {
                s.store_sqrt(726, 726);
            }
            let (assign52750_body1_e80322,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) {
        let assign52750_body1_e80320: f64 = (s.v[719] + 1.0);
        (assign52750_body1_e80320,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign52750_body1_e80322;
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && (!s.b[2335])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_offset_lhs(983, 2169, 0.2, 780);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && (!s.b[2334])) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && (!s.b[2334])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.copy_ad(2151, 983);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_scalar(2140, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2340] = ((s.v[2151] > (-s.v[2140])) && (s.v[2140] >= 0.0));
        s.v[2340] = if s.b[2340] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_add(781, 2151, 2140);
            s.store_square(722, 781);
        }

    }

    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
    ) {
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_square(723, 2140);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign52920_e80652,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52920_e80652;

        let (assign52930_e80668,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign52930_e80668;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign52960_e80716,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign52960_e80716;

        let mut assign52970_loop_guard: usize = 0;
        while {
            let assign52970_cond_e80733: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && (s.v[719] < s.v[2141])) { 1.0 } else { 0.0 };
            assign52970_cond_e80733 != 0.0
        } {
            assign52970_loop_guard += 1;
            assert!(assign52970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign52970_body2_e80787,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
        let assign52970_body2_e80785: f64 = (s.v[719] + 1.0);
        (assign52970_body2_e80785,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign52970_body2_e80787;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2341] = ((((s.v[2141] == 1.0) || (s.v[2141] == 2.0)) || (s.v[2141] == 4.0)) || (s.v[2141] == 8.0));
        s.v[2341] = if s.b[2341] { 1.0 } else { 0.0 };

        s.b[2342] = (s.v[2141] == 1.0);
        s.v[2342] = if s.b[2342] { 1.0 } else { 0.0 };

        let (assign53020_e80859,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && s.b[2342]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53020_e80859;

        s.b[2343] = (s.v[2141] == 2.0);
        s.v[2343] = if s.b[2343] { 1.0 } else { 0.0 };

        let (assign53040_e80885,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && s.b[2343]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53040_e80885;

        s.b[2344] = (s.v[2141] == 4.0);
        s.v[2344] = if s.b[2344] { 1.0 } else { 0.0 };

        let (assign53060_e80914,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && (!s.b[2343])) && s.b[2344]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53060_e80914;

        s.b[2345] = (s.v[2141] == 8.0);
        s.v[2345] = if s.b[2345] { 1.0 } else { 0.0 };

        let (assign53080_e80946,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && (!s.b[2343])) && (!s.b[2344])) && s.b[2345]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53080_e80946;

        let (assign53090_e80964,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign53090_e80964;

        let mut assign53100_loop_guard: usize = 0;
        while {
            let assign53100_cond_e80983: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53100_cond_e80983 != 0.0
        } {
            assign53100_loop_guard += 1;
            assert!(assign53100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) {
                s.store_sqrt(726, 726);
            }
            let (assign53100_body1_e81022,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) {
        let assign53100_body1_e81020: f64 = (s.v[719] + 1.0);
        (assign53100_body1_e81020,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign53100_body1_e81022;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && (!s.b[2341])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2141), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2140, 726);
            s.store_div_scaled_product3_indices(334, 2140, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2140, -1.0, 780, 1.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2340])) {
            s.copy_ad(983, 2151);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2318, 2131, 2146, (0.5 * 9662367879.197212), 0.0, 2146);
            s.store_mul_sqrt_ad_rhs(334, 2150, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2318)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2346] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2346] = if s.b[2346] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2346]) {
            s.store_div_ln_lhs(2319, 335, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2346])) {
            s.store_mul3_ad_middle(2319, A::square(s.ad_value(2150)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_mul(332, 2319, 983);
        }

        s.b[2347] = (s.v[332] > 500.0);
        s.v[2347] = if s.b[2347] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2347]) {
            s.store_sub(2163, 983, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2319, -1.0, 2318);
        }

        s.b[2348] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2348] = if s.b[2348] { 1.0 } else { 0.0 };

        s.b[2349] = (s.v[332] >= 500.0);
        s.v[2349] = if s.b[2349] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && s.b[2349]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign53360_loop_guard: usize = 0;
        while {
            let assign53360_cond_e81493: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign53360_cond_e81493 != 0.0
        } {
            assign53360_loop_guard += 1;
            assert!(assign53360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && (!s.b[2348])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2350] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2350] = if s.b[2350] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2350]) {
            s.store_div_ln_offset_lhs(2163, 336, 1.0, 2319);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && (!s.b[2350])) {
            s.store_div(2163, 336, 2319);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(336, 983, 2163);
        }

        s.b[2351] = (0.0 == 0.0);
        s.v[2351] = if s.b[2351] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2351]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2145, A::sqrt(A::mul_scaled_lhs(s.ad_value(2134), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2145, 2134, 336);
            }
        }

        s.b[2352] = (s.v[336] < 0.0);
        s.v[2352] = if s.b[2352] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2351])) && s.b[2352]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2145, A::sqrt(A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2351])) && (!s.b[2352])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2145, A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(990, 2146, 2145);
        }

        s.b[2353] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2353] = if s.b[2353] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign53610_e82016,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign53610_e82016;

        let (assign53620_e82032,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53620_e82032;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2354] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2354] = if s.b[2354] { 1.0 } else { 0.0 };

        s.b[2355] = (2.0 == 1.0);
        s.v[2355] = if s.b[2355] { 1.0 } else { 0.0 };

        let (assign53730_e82208,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && s.b[2355]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53730_e82208;

        s.b[2356] = (2.0 == 2.0);
        s.v[2356] = if s.b[2356] { 1.0 } else { 0.0 };

        let (assign53750_e82234,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && s.b[2356]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53750_e82234;

        s.b[2357] = (2.0 == 4.0);
        s.v[2357] = if s.b[2357] { 1.0 } else { 0.0 };

        let (assign53770_e82263,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && (!s.b[2356])) && s.b[2357]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53770_e82263;

        s.b[2358] = (2.0 == 8.0);
        s.v[2358] = if s.b[2358] { 1.0 } else { 0.0 };

        let (assign53790_e82295,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && (!s.b[2356])) && (!s.b[2357])) && s.b[2358]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign53790_e82295;

        let (assign53800_e82313,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign53800_e82313;

        let mut assign53810_loop_guard: usize = 0;
        while {
            let assign53810_cond_e82332: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53810_cond_e82332 != 0.0
        } {
            assign53810_loop_guard += 1;
            assert!(assign53810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) {
                s.store_sqrt(726, 726);
            }
            let (assign53810_body1_e82371,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) {
        let assign53810_body1_e82369: f64 = (s.v[719] + 1.0);
        (assign53810_body1_e82369,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign53810_body1_e82371;
        }

    }

    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && (!s.b[2354])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2353])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2353])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2359] = (1.0 == 1.0);
        s.v[2359] = if s.b[2359] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2359]) {
            s.copy_ad(2153, 990);
        }

        s.b[2360] = (2.0 == 1.0);
        s.v[2360] = if s.b[2360] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2360]) {
            s.store_scale(2168, 2115, p.p399);
            s.store_offset(983, 2168, (-1.0));
            s.copy_ad(2322, 2323);
            s.copy_ad(2144, 2323);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2360])) {
            s.store_offset_scaled(2168, 2115, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2322, 2143);
            s.copy_ad(2144, 2143);
        }

        let (assign54010_e82707,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign54010_e82707;

        let (assign54020_e82721,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign54020_e82721;

        let mut assign54030_loop_guard: usize = 0;
        while {
            let assign54030_cond_e82736: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign54030_cond_e82736 != 0.0
        } {
            assign54030_loop_guard += 1;
            assert!(assign54030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2361] = (s.v[983] >= 0.0);
            s.v[2361] = if s.b[2361] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2361]) {
                s.store_mul_scaled_sqrt_ad_rhs(2320, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2123, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2320), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2361])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2168)));
                s.store_exp_mul(338, 154, 2168);
                s.store_mul_sqrt_ad_rhs(2320, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2320, 1.0);
                s.store_mul_add_ad_rhs(2123, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            let (assign54030_body10_e82970,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] != 0.0)) {
        let assign54030_body10_e82968: f64 = (150.0 + 1.0);
        (assign54030_body10_e82968,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign54030_body10_e82970;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2098, 2320, 1.0, 185, 2322, 983, 1.0);
                s.store_sub(2099, 2123, 185);
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2362] = (((s.v[2110]) as f64).abs() < (1e-10 * 100.0));
            s.v[2362] = if s.b[2362] { 1.0 } else { 0.0 };
            let (assign54030_body15_e83058,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && s.b[2362]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign54030_body15_e83058;
            s.b[2363] = (s.v[2110] > 0.1);
            s.v[2363] = if s.b[2363] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2362])) && s.b[2363]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2364] = (s.v[2110] < (-0.1));
            s.v[2364] = if s.b[2364] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2362])) && (!s.b[2363])) && s.b[2364]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2110);
            }
            let (assign54030_body21_e83148,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
        let assign54030_body21_e83146: f64 = (s.v[97] + 1.0);
        (assign54030_body21_e83146,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign54030_body21_e83148;
        }

        s.b[2366] = (2.0 == 1.0);
        s.v[2366] = if s.b[2366] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2366]) {
            s.copy_ad(2169, 983);
        }

        s.b[2367] = ((s.v[983] < (s.v[2169] + 0.2)) && (0.2 >= 0.0));
        s.v[2367] = if s.b[2367] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
            s.store_sub_offset_lhs(781, 2169, 0.2, 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign54130_e83301,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54130_e83301;

        let (assign54140_e83320,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54140_e83320;

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2368] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2368] = if s.b[2368] { 1.0 } else { 0.0 };

        s.b[2369] = (2.0 == 1.0);
        s.v[2369] = if s.b[2369] { 1.0 } else { 0.0 };

        let (assign54250_e83523,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && s.b[2369]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54250_e83523;

        s.b[2370] = (2.0 == 2.0);
        s.v[2370] = if s.b[2370] { 1.0 } else { 0.0 };

        let (assign54270_e83552,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && s.b[2370]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54270_e83552;

        s.b[2371] = (2.0 == 4.0);
        s.v[2371] = if s.b[2371] { 1.0 } else { 0.0 };

        let (assign54290_e83584,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && (!s.b[2370])) && s.b[2371]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54290_e83584;

        s.b[2372] = (2.0 == 8.0);
        s.v[2372] = if s.b[2372] { 1.0 } else { 0.0 };

        let (assign54310_e83619,) = {
    if (((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && (!s.b[2370])) && (!s.b[2371])) && s.b[2372]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54310_e83619;

        let (assign54320_e83640,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54320_e83640;

        let mut assign54330_loop_guard: usize = 0;
        while {
            let assign54330_cond_e83662: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54330_cond_e83662 != 0.0
        } {
            assign54330_loop_guard += 1;
            assert!(assign54330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) {
                s.store_sqrt(726, 726);
            }
            let (assign54330_body1_e83707,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) {
        let assign54330_body1_e83705: f64 = (s.v[719] + 1.0);
        (assign54330_body1_e83705,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign54330_body1_e83707;
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && (!s.b[2368])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_offset_lhs(983, 2169, 0.2, 780);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && (!s.b[2367])) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && (!s.b[2367])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.copy_ad(2151, 983);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_scalar(2140, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2373] = ((s.v[2151] > (-s.v[2140])) && (s.v[2140] >= 0.0));
        s.v[2373] = if s.b[2373] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
            s.store_add(781, 2151, 2140);
            s.store_square(722, 781);
            s.store_square(723, 2140);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign54500_e84037,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54500_e84037;

        let (assign54510_e84053,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54510_e84053;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign54540_e84101,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54540_e84101;

        let mut assign54550_loop_guard: usize = 0;
        while {
            let assign54550_cond_e84118: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && (s.v[719] < s.v[2141])) { 1.0 } else { 0.0 };
            assign54550_cond_e84118 != 0.0
        } {
            assign54550_loop_guard += 1;
            assert!(assign54550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
            }
            let (assign54550_body2_e84172,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
        let assign54550_body2_e84170: f64 = (s.v[719] + 1.0);
        (assign54550_body2_e84170,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign54550_body2_e84172;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2374] = ((((s.v[2141] == 1.0) || (s.v[2141] == 2.0)) || (s.v[2141] == 4.0)) || (s.v[2141] == 8.0));
        s.v[2374] = if s.b[2374] { 1.0 } else { 0.0 };

        s.b[2375] = (s.v[2141] == 1.0);
        s.v[2375] = if s.b[2375] { 1.0 } else { 0.0 };

        let (assign54600_e84244,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && s.b[2375]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54600_e84244;

        s.b[2376] = (s.v[2141] == 2.0);
        s.v[2376] = if s.b[2376] { 1.0 } else { 0.0 };

        let (assign54620_e84270,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && s.b[2376]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54620_e84270;

        s.b[2377] = (s.v[2141] == 4.0);
        s.v[2377] = if s.b[2377] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign54640_e84299,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && (!s.b[2376])) && s.b[2377]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54640_e84299;

        s.b[2378] = (s.v[2141] == 8.0);
        s.v[2378] = if s.b[2378] { 1.0 } else { 0.0 };

        let (assign54660_e84331,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && (!s.b[2376])) && (!s.b[2377])) && s.b[2378]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign54660_e84331;

        let (assign54670_e84349,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign54670_e84349;

        let mut assign54680_loop_guard: usize = 0;
        while {
            let assign54680_cond_e84368: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54680_cond_e84368 != 0.0
        } {
            assign54680_loop_guard += 1;
            assert!(assign54680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) {
                s.store_sqrt(726, 726);
            }
            let (assign54680_body1_e84407,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) {
        let assign54680_body1_e84405: f64 = (s.v[719] + 1.0);
        (assign54680_body1_e84405,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign54680_body1_e84407;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && (!s.b[2374])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2141), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2140, 726);
            s.store_div_scaled_product3_indices(334, 2140, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2140, -1.0, 780, 1.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2373])) {
            s.copy_ad(983, 2151);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2318, 2131, 2146, (0.5 * 9662367879.197212), 0.0, 2146);
            s.store_mul_sqrt_ad_rhs(334, 2150, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2318)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2379] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2379] = if s.b[2379] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2379]) {
            s.store_div_ln_lhs(2319, 335, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2379])) {
            s.store_mul3_ad_middle(2319, A::square(s.ad_value(2150)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_mul(332, 2319, 983);
        }

        s.b[2380] = (s.v[332] > 500.0);
        s.v[2380] = if s.b[2380] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2380]) {
            s.store_sub(2163, 983, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2319, -1.0, 2318);
        }

        s.b[2381] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2381] = if s.b[2381] { 1.0 } else { 0.0 };

        s.b[2382] = (s.v[332] >= 500.0);
        s.v[2382] = if s.b[2382] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && s.b[2382]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign54940_loop_guard: usize = 0;
        while {
            let assign54940_cond_e84878: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign54940_cond_e84878 != 0.0
        } {
            assign54940_loop_guard += 1;
            assert!(assign54940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && (!s.b[2381])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2383] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2383] = if s.b[2383] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2383]) {
            s.store_div_ln_offset_lhs(2163, 336, 1.0, 2319);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && (!s.b[2383])) {
            s.store_div(2163, 336, 2319);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(336, 983, 2163);
        }

        s.b[2384] = (0.0 == 0.0);
        s.v[2384] = if s.b[2384] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2384]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2145, A::sqrt(A::mul_scaled_lhs(s.ad_value(2134), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2145, 2134, 336);
            }
        }

        s.b[2385] = (s.v[336] < 0.0);
        s.v[2385] = if s.b[2385] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2384])) && s.b[2385]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2145, A::sqrt(A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2384])) && (!s.b[2385])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2145, A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(990, 2146, 2145);
        }

        s.b[2386] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2386] = if s.b[2386] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign55190_e85401,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55190_e85401;

        let (assign55200_e85417,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55200_e85417;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2387] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2387] = if s.b[2387] { 1.0 } else { 0.0 };

        s.b[2388] = (2.0 == 1.0);
        s.v[2388] = if s.b[2388] { 1.0 } else { 0.0 };

        let (assign55310_e85593,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && s.b[2388]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55310_e85593;

        s.b[2389] = (2.0 == 2.0);
        s.v[2389] = if s.b[2389] { 1.0 } else { 0.0 };

        let (assign55330_e85619,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && s.b[2389]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55330_e85619;

        s.b[2390] = (2.0 == 4.0);
        s.v[2390] = if s.b[2390] { 1.0 } else { 0.0 };

        let (assign55350_e85648,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && (!s.b[2389])) && s.b[2390]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55350_e85648;

        s.b[2391] = (2.0 == 8.0);
        s.v[2391] = if s.b[2391] { 1.0 } else { 0.0 };

        let (assign55370_e85680,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && (!s.b[2389])) && (!s.b[2390])) && s.b[2391]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55370_e85680;

        let (assign55380_e85698,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55380_e85698;

        let mut assign55390_loop_guard: usize = 0;
        while {
            let assign55390_cond_e85717: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55390_cond_e85717 != 0.0
        } {
            assign55390_loop_guard += 1;
            assert!(assign55390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) {
                s.store_sqrt(726, 726);
            }
            let (assign55390_body1_e85756,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) {
        let assign55390_body1_e85754: f64 = (s.v[719] + 1.0);
        (assign55390_body1_e85754,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign55390_body1_e85756;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && (!s.b[2387])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2386])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2386])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2392] = (2.0 == 1.0);
        s.v[2392] = if s.b[2392] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2392]) {
            s.copy_ad(2153, 990);
        }

        s.b[2393] = (0.0 == 0.0);
        s.v[2393] = if s.b[2393] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) {
            s.copy_ad(989, 349);
            s.store_scaled_add(344, 2115, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(2133), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 2133, 1.0);
        }

        s.b[2394] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2394] = if s.b[2394] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign55610_e86109,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55610_e86109;

        let (assign55620_e86124,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55620_e86124;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2395] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2395] = if s.b[2395] { 1.0 } else { 0.0 };

        s.b[2396] = (2.0 == 1.0);
        s.v[2396] = if s.b[2396] { 1.0 } else { 0.0 };

        let (assign55730_e86291,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && s.b[2396]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55730_e86291;

        s.b[2397] = (2.0 == 2.0);
        s.v[2397] = if s.b[2397] { 1.0 } else { 0.0 };

        let (assign55750_e86316,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && s.b[2397]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55750_e86316;

        s.b[2398] = (2.0 == 4.0);
        s.v[2398] = if s.b[2398] { 1.0 } else { 0.0 };

        let (assign55770_e86344,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && (!s.b[2397])) && s.b[2398]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55770_e86344;

        s.b[2399] = (2.0 == 8.0);
        s.v[2399] = if s.b[2399] { 1.0 } else { 0.0 };

        let (assign55790_e86375,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && (!s.b[2397])) && (!s.b[2398])) && s.b[2399]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55790_e86375;

        let (assign55800_e86392,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55800_e86392;

        let mut assign55810_loop_guard: usize = 0;
        while {
            let assign55810_cond_e86410: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55810_cond_e86410 != 0.0
        } {
            assign55810_loop_guard += 1;
            assert!(assign55810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {
                s.store_sqrt(726, 726);
            }
            let (assign55810_body1_e86447,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {
        let assign55810_body1_e86445: f64 = (s.v[719] + 1.0);
        (assign55810_body1_e86445,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign55810_body1_e86447;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && (!s.b[2395])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2394])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2394])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)));
        }

        s.b[2400] = ((s.v[344] < (s.v[972] + p.p405)) && (p.p405 >= 0.0));
        s.v[2400] = if s.b[2400] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
            s.store_sub_offset_lhs(781, 972, p.p405, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p405 * p.p405));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign55980_e86739,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign55980_e86739;

        let (assign55990_e86754,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign55990_e86754;

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2401] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2401] = if s.b[2401] { 1.0 } else { 0.0 };

        s.b[2402] = (2.0 == 1.0);
        s.v[2402] = if s.b[2402] { 1.0 } else { 0.0 };

        let (assign56100_e86921,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && s.b[2402]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56100_e86921;

        s.b[2403] = (2.0 == 2.0);
        s.v[2403] = if s.b[2403] { 1.0 } else { 0.0 };

        let (assign56120_e86946,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && s.b[2403]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56120_e86946;

        s.b[2404] = (2.0 == 4.0);
        s.v[2404] = if s.b[2404] { 1.0 } else { 0.0 };

        let (assign56140_e86974,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && (!s.b[2403])) && s.b[2404]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56140_e86974;

        s.b[2405] = (2.0 == 8.0);
        s.v[2405] = if s.b[2405] { 1.0 } else { 0.0 };

        let (assign56160_e87005,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && (!s.b[2403])) && (!s.b[2404])) && s.b[2405]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56160_e87005;

        let (assign56170_e87022,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign56170_e87022;

        let mut assign56180_loop_guard: usize = 0;
        while {
            let assign56180_cond_e87040: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56180_cond_e87040 != 0.0
        } {
            assign56180_loop_guard += 1;
            assert!(assign56180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {
                s.store_sqrt(726, 726);
            }
            let (assign56180_body1_e87077,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {
        let assign56180_body1_e87075: f64 = (s.v[719] + 1.0);
        (assign56180_body1_e87075,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign56180_body1_e87077;
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && (!s.b[2401])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p405);
            s.store_div_scaled_product_indices(334, 725, 726, p.p405, 770, 1.0);
            s.store_sub_offset_lhs(992, 972, p.p405, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2400])) {
            s.copy_ad(992, 344);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            s.copy_ad(2157, 2143);
            s.store_offset_mul(338, 2133, 2157, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            s.store_add_ad_rhs(2158, 2157, A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)));
            s.copy_ad(2154, 2158);
        }

        let (assign56320_e87332,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign56320_e87332;

        let (assign56330_e87346,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign56330_e87346;

        let mut assign56340_loop_guard: usize = 0;
        while {
            let assign56340_cond_e87361: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign56340_cond_e87361 != 0.0
        } {
            assign56340_loop_guard += 1;
            assert!(assign56340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
                s.store_mul_neg_lhs(335, 154, 2154);
                s.store_exp(336, 335);
                s.store_sqrt_div_scaled_inputs(338, 2112, 2.0, 154, 1.0);
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2155, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2406] = (s.v[335] > 0.0);
            s.v[2406] = if s.b[2406] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && s.b[2406]) {
                s.store_neg(2155, 2155);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2155, 1.0);
                s.store_mul_sub_from_scalar_rhs(2156, 345, 1.0, 336);
            }
            let (assign56340_body9_e87528,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] != 0.0)) {
        let assign56340_body9_e87526: f64 = (150.0 + 1.0);
        (assign56340_body9_e87526,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign56340_body9_e87528;
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2098, 2155, 1.0, 185, 2157, 2154, -1.0);
                s.store_add(2099, 185, 2156);
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2407] = (((s.v[2110]) as f64).abs() < 1e-10);
            s.v[2407] = if s.b[2407] { 1.0 } else { 0.0 };
            let (assign56340_body14_e87614,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && s.b[2407]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign56340_body14_e87614;
            s.b[2408] = (s.v[2110] > 0.1);
            s.v[2408] = if s.b[2408] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && (!s.b[2407])) && s.b[2408]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2409] = (s.v[2110] < (-0.1));
            s.v[2409] = if s.b[2409] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && (!s.b[2407])) && (!s.b[2408])) && s.b[2409]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) {
                s.store_add(2154, 2154, 2110);
            }
            let (assign56340_body20_e87704,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
        let assign56340_body20_e87702: f64 = (s.v[97] + 1.0);
        (assign56340_body20_e87702,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign56340_body20_e87704;
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            s.copy_ad(2151, 2154);
            s.copy_ad(989, 349);
            s.store_sqrt_square_offset(782, 2151, ((4.0 * p.p405) * p.p405));
            s.store_offset_scaled_div(334, 2151, 782, 0.5, 0.5);
            s.store_scaled_add(992, 2151, 782, 0.5);
        }

        s.b[2410] = (s.v[992] < 0.0);
        s.v[2410] = if s.b[2410] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && s.b[2410]) {
            s.store_scalar(992, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div(335, 989, 992);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
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
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul(340, 338, 337);
        }

        s.b[2411] = ((s.v[349] > (s.v[972] - (s.v[972] * 0.5))) && ((s.v[972] * 0.5) >= 0.0));
        s.v[2411] = if s.b[2411] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
            s.store_add_scaled_inputs3_indices(781, 349, 1.0, 972, (-1.0), 972, 0.5);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 972, 972, (0.5 * 0.5));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign56540_e88016,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign56540_e88016;

        let (assign56550_e88029,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56550_e88029;

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2412] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2412] = if s.b[2412] { 1.0 } else { 0.0 };

        s.b[2413] = (2.0 == 1.0);
        s.v[2413] = if s.b[2413] { 1.0 } else { 0.0 };

        let (assign56660_e88178,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && s.b[2413]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56660_e88178;

        s.b[2414] = (2.0 == 2.0);
        s.v[2414] = if s.b[2414] { 1.0 } else { 0.0 };

        let (assign56680_e88201,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && s.b[2414]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56680_e88201;

        s.b[2415] = (2.0 == 4.0);
        s.v[2415] = if s.b[2415] { 1.0 } else { 0.0 };

        let (assign56700_e88227,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && s.b[2415]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56700_e88227;

        s.b[2416] = (2.0 == 8.0);
        s.v[2416] = if s.b[2416] { 1.0 } else { 0.0 };

        let (assign56720_e88256,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && (!s.b[2415])) && s.b[2416]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign56720_e88256;

        let (assign56730_e88271,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign56730_e88271;

        let mut assign56740_loop_guard: usize = 0;
        while {
            let assign56740_cond_e88287: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56740_cond_e88287 != 0.0
        } {
            assign56740_loop_guard += 1;
            assert!(assign56740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
                s.store_sqrt(726, 726);
            }
            let (assign56740_body1_e88320,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
        let assign56740_body1_e88318: f64 = (s.v[719] + 1.0);
        (assign56740_body1_e88318,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign56740_body1_e88320;
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && (!s.b[2412])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);
            s.store_add_scaled_inputs3_indices(2164, 972, 1.0, 972, (-0.5), 780, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2411])) {
            s.copy_ad(2164, 349);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_add_div_lhs_indices(989, 989, 340, 2164);
            s.store_mul_square_lhs(338, 2164, 2164);
            s.store_offset(334, 338, 0.0001);
            s.store_div(2165, 338, 334);
        }

        s.b[2417] = (p.p43 == (-1.0));
        s.v[2417] = if s.b[2417] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2417]) {
            s.store_scalar(2165, 0.0);
            s.copy_ad(989, 349);
        }

        s.b[2418] = (p.p43 == 2.0);
        s.v[2418] = if s.b[2418] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) && s.b[2418]) {
            s.copy_ad(989, 349);
            s.store_scalar(2164, 0.0);
            s.store_scalar(2165, 0.0);
            s.store_sub(335, 2144, 972);
            s.store_add_scaled_inputs3_offset_mixed_iai(992, 335, 0.5, A::ln(A::cosh(s.ad_value(335))), 0.5, 972, 1.0, (((2.0) as f64).ln() * 0.5));
        }

        s.b[2419] = (p.p43 == 3.0);
        s.v[2419] = if s.b[2419] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) && (!s.b[2418])) && s.b[2419]) {
            s.store_add_ad_lhs(992, A::ln_one_plus_exp(A::sub(s.ad_value(2144), s.ad_value(972))), 972);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            s.store_div(335, 989, 992);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            s.store_mul(340, 338, 337);
            s.store_add_div_lhs_indices(989, 989, 340, 2164);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul(2122, 990, 2131);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 2122, 343);
            s.store_offset_sqrt_ad(2166, A::offset(A::square(s.ad_value(989)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 2166, 688, 1.0);
            s.store_offset_mul(339, 2166, 689, 1.0);
        }

        s.b[2420] = param_given[408];
        s.v[2420] = if s.b[2420] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2420]) {
            s.store_div_scaled_value_by_product(2152, A::sub_from_scalar(p.p408, s.ad_value(2090)), 1.0, s.ad_value(965), s.ad_value(339), 100.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2420])) {
            s.store_div_scaled_inputs_indices(2152, 2122, 9662367879.197212, 339, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[2152] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 2152, p.p376);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add(s.ad_value(966), A::mul3_scaled_output(s.ad_value(968), s.ad_value(338), s.ad_value(252), 1e-10)), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div_scaled_value_offset_denominator(2113, s.ad_value(989), 1.0, s.ad_value(162), p.p401, 1.0);
            s.store_square(781, 989);
            s.store_scalar(782, ((0.01) as f64).powf(2.0));
            s.store_sub_ad(334, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div_scaled_value_offset_denominator(2167, s.ad_value(334), 1.0, s.ad_value(162), (-p.p402), 1.0);
            s.store_div_scaled_product_indices(335, 254, 2167, 1.0, 973, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div(985, 254, 338);
            s.store_mul_offset_ad_rhs(2130, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2113), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2113), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p.p400), 1.0);
            s.store_scaled_mul(335, 990, 2130, 1.6021918e-19);
            s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p.p7);
            s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2113);
            s.store_mul3_affine_lhs(988, 336, 2153, p.p363, 0.0, 2165);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add_scaled_inputs3_indices(135, 986, 1.0, 987, 1.0, 988, 1.0);
            s.copy_ad(790, 349);
        }

        s.b[2421] = (p.p283 != 0.0);
        s.v[2421] = if s.b[2421] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(2087), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2422] = (s.v[336] < 0.0);
        s.v[2422] = if s.b[2422] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) && s.b[2422]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
        }

    }

    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1437, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 2087, 1.0, 340, 1.0, 1436, -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1437), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2421])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2423] = (p.p287 != 0.0);
        s.v[2423] = if s.b[2423] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2423]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1437);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2423])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2424] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2424] = if s.b[2424] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2424]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(134, 135);
            s.store_add_scaled_inputs4_indices(131, 2096, (-0.5), 2120, ((-1.0) * (-0.5)), 2097, (-0.5), 2121, (-(-0.5)));
            s.store_scaled_add(133, 2120, 2121, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 2120, 2121, (-0.5));
            s.store_neg(238, 2120);
            s.copy_ad(255, 2114);
        }

        s.b[2425] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2425] = if s.b[2425] { 1.0 } else { 0.0 };

        let (assign57780_e89986,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2425]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign57780_e89986;

        s.b[2426] = (s.v[791] < s.v[86]);
        s.v[2426] = if s.b[2426] { 1.0 } else { 0.0 };

        let (assign57800_e89997,) = {
    if ((!s.b[1441]) && s.b[2426]) {
        let assign57800_e89995: f64 = (-1.0);
        (assign57800_e89995,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign57800_e89997;

        if ((!s.b[1441]) && s.b[2426]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 85, 1433);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2427] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2427] = if s.b[2427] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2426]) && s.b[2427]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((!s.b[1441]) && s.b[2426]) && (!s.b[2427])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((!s.b[1441]) && s.b[2426]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if ((!s.b[1441]) && s.b[2426]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 1433, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 1433);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_div_lhs_indices(87, 335, 337, 1433);
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

        let (assign58130_e90335,) = {
    if ((!s.b[1441]) && s.b[2426]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58130_e90335;

        let (assign58140_e90342,) = {
    if ((!s.b[1441]) && s.b[2426]) {
        (1.0,)
    } else {
        (s.v[946],)
    }
};
        s.v[946] = assign58140_e90342;

        s.b[2428] = (s.v[946] == 0.0);
        s.v[2428] = if s.b[2428] { 1.0 } else { 0.0 };

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1433))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_add_ad_rhs(89, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
        }

        s.b[2429] = (s.v[77] == 0.0);
        s.v[2429] = if s.b[2429] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2429]) {
            s.store_mul_sub_rhs(116, 154, 89, 1433);
        }

        s.b[2430] = (s.v[116] < 3.0);
        s.v[2430] = if s.b[2430] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {
            s.store_mul_sub_rhs(333, 154, 85, 1433);
            s.store_div_from_scalar_scaled_mul(335, 1.0, 154, 212, (1.414213562373095 / 108.0));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 1433, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2431] = (s.v[791] <= s.v[118]);
        s.v[2431] = if s.b[2431] { 1.0 } else { 0.0 };

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && s.b[2431]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 85, 85);
            s.store_add_ad_rhs(337, 154, A::div_from_scalar(2.0, s.ad_value(85)));
            s.store_div_ln_lhs(90, 336, 337);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(332, 1433, (1e-12 / 2.0));
        }

        s.b[2432] = (s.v[88] < s.v[332]);
        s.v[2432] = if s.b[2432] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2432]) {
            s.copy_ad(88, 332);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.copy_ad(87, 88);
            s.copy_ad(92, 89);
            s.store_exp_mul(229, 154, 1433);
            s.store_mul(222, 210, 229);
        }

        let (assign58510_e90895,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign58510_e90895;

        let (assign58520_e90902,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign58520_e90902;

    }

    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
    ) {
        let mut assign58530_loop_guard: usize = 0;
        while {
            let assign58530_cond_e90910: f64 = (s.v[421] + 1.0);
            let assign58530_cond_e90912: f64 = if (((!s.b[1441]) && s.b[2428]) && (s.v[97] <= assign58530_cond_e90910)) { 1.0 } else { 0.0 };
            assign58530_cond_e90912 != 0.0
        } {
            assign58530_loop_guard += 1;
            assert!(assign58530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_mul_sub_rhs(116, 154, 87, 1433);
            }
            s.b[2433] = (s.v[116] < 5.0);
            s.v[2433] = if s.b[2433] { 1.0 } else { 0.0 };
            if (((!s.b[1441]) && s.b[2428]) && s.b[2433]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(214, 222, 225, 225);
                s.store_mul_product3_rhs(215, 226, s.ad_value(222), s.ad_value(154), s.ad_value(225), 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_inputs2_mixed_aii(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 215, 1.0, 216, 2.0);
            }
            s.b[2434] = (s.v[116] < 60.0);
            s.v[2434] = if s.b[2434] { 1.0 } else { 0.0 };
            if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) && s.b[2434]) {
                s.store_exp(227, 116);
                s.store_mul_offset_rhs(214, 222, 227, (-1.0));
                s.store_mul3_lhs(215, 222, 154, 227);
            }
            if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) && (!s.b[2434])) {
                s.store_exp_mul(231, 154, 87);
                s.store_mul_sub_rhs(214, 210, 231, 229);
                s.store_mul3_lhs(215, 210, 154, 231);
            }
            if (((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) {
                s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));
                s.store_div_scaled_inputs2_indices(217, 154, 1.0, 215, 1.0, 216, 2.0);
            }
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));
                s.store_sub_from_scalar_scaled_mul(233, (-1.0), 212, 217, 1.0);
            }
            s.b[2435] = (s.v[79] == 1.0);
            s.v[2435] = if s.b[2435] { 1.0 } else { 0.0 };
            let (assign58530_body23_e91282,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2435]) {
        (1.0,)
    } else {
        (s.v[944],)
    }
};
            s.v[944] = assign58530_body23_e91282;
            s.b[2436] = (s.v[944] == 0.0);
            s.v[2436] = if s.b[2436] { 1.0 } else { 0.0 };
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2437] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2437] = if s.b[2437] { 1.0 } else { 0.0 };
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2436]) && s.b[2437]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {
                s.store_add(87, 87, 236);
            }
            s.b[2438] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2438] = if s.b[2438] { 1.0 } else { 0.0 };
            let (assign58530_body31_e91373,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2436]) && s.b[2438]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign58530_body31_e91373;
            let (assign58530_body32_e91384,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[944] != 0.0)) {
        let assign58530_body32_e91382: f64 = (s.v[421] + 1.0);
        (assign58530_body32_e91382,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign58530_body32_e91384;
            let (assign58530_body33_e91391,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (0.0,)
    } else {
        (s.v[944],)
    }
};
            s.v[944] = assign58530_body33_e91391;
            let (assign58530_body34_e91400,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let assign58530_body34_e91398: f64 = (s.v[97] + 1.0);
        (assign58530_body34_e91398,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign58530_body34_e91400;
        }

        let (assign58540_e91409,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let assign58540_e91407: f64 = (s.v[97] - 1.0);
        (assign58540_e91407,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign58540_e91409;

        s.b[2440] = (s.v[116] < 5.0);
        s.v[2440] = if s.b[2440] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2440]) {
            s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        let (assign58600_e91470,) = {
    if (((!s.b[1441]) && s.b[2428]) && (!s.b[2440])) {
        (3.0,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign58600_e91470;

        let (assign58610_e91480,) = {
    if (((!s.b[1441]) && s.b[2428]) && (!s.b[2440])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58610_e91480;

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2440])) {
            s.store_offset(99, 116, (-1.0));
            s.store_sqrt(100, 99);
            s.store_mul(101, 99, 100);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(239, 209, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_offset_product3(238, s.ad_value(209), s.ad_value(214), s.ad_value(335), 1.0, 1e-25);
        }

        s.b[2441] = (s.v[116] < 5.0);
        s.v[2441] = if s.b[2441] { 1.0 } else { 0.0 };

        s.b[2442] = (s.v[116] < 3.0);
        s.v[2442] = if s.b[2442] { 1.0 } else { 0.0 };

        let (assign58700_e91565,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && s.b[2442]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign58700_e91565;

        let (assign58710_e91576,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && s.b[2442]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58710_e91576;

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && s.b[2442]) {
            s.copy_ad(133, 238);
            s.copy_ad(131, 239);
            s.store_scalar(247, 0.5);
            s.store_scalar(169, 0.0);
        }

        let (assign58760_e91632,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && (!s.b[2442])) {
        (2.0,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign58760_e91632;

        let (assign58770_e91644,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && (!s.b[2442])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign58770_e91644;

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && (!s.b[2442])) {
            s.store_scalar(335, (1.0 / (5.0 - 3.0)));
            s.store_mul_offset_rhs(332, 335, 116, (-3.0));
            s.store_mul3_ad_middle(207, A::square(s.ad_value(332)), 332, A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0));
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(127, 238, 186);
            s.copy_ad(349, 790);
            s.store_div_square_rhs(336, 636, 185);
            s.store_add_scaled_inputs3_indices(334, 85, 1.0, 155, (-1.0), 1436, -1.0);
            s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);
            s.store_scaled_add(343, 335, 782, 0.5);
        }

        s.b[2443] = (s.v[343] < 0.0);
        s.v[2443] = if s.b[2443] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2443]) {
            s.store_scalar(343, 0.0);
            s.store_scalar(332, 0.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(336), 1.0, s.ad_value(337)));
            s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);
            s.store_scaled_add(344, 344, 782, 0.5);
        }

        s.b[2444] = (s.v[344] < 0.0);
        s.v[2444] = if s.b[2444] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2444]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
            s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1433), s.ad_value(790))));
        }

        s.b[2445] = (s.v[790] < 0.0);
        s.v[2445] = if s.b[2445] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2445]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(91, 87);
        }

        let (assign59140_e92045,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2445]) {
        (1.0,)
    } else {
        (s.v[947],)
    }
};
        s.v[947] = assign59140_e92045;

        s.b[2446] = (s.v[947] == 0.0);
        s.v[2446] = if s.b[2446] { 1.0 } else { 0.0 };

        s.b[2447] = (s.v[77] == 0.0);
        s.v[2447] = if s.b[2447] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }

        s.b[2448] = (((1.0 + 0.3) * s.v[96]) > 0.03);
        s.v[2448] = if s.b[2448] { 1.0 } else { 0.0 };

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {
            s.store_offset_sub_scaled_inputs(781, s.ad_value(96), (1.0 + 0.3), s.ad_value(790), 1.0, (-0.03));
            s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(95, 96, (1.0 + 0.3), 781, (-0.5), 782, (-0.5));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && (!s.b[2448])) {
            s.store_scale(95, 96, (1.0 + 0.3));
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }

        s.b[2449] = (s.v[95] < 0.0);
        s.v[2449] = if s.b[2449] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2449]) {
            s.store_scalar(95, 0.0);
        }

        s.b[2450] = (s.v[95] > s.v[790]);
        s.v[2450] = if s.b[2450] { 1.0 } else { 0.0 };

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && (!s.b[2449])) && s.b[2450]) {
            s.copy_ad(95, 790);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2446]) {
            s.copy_ad(94, 95);
            s.store_add(91, 87, 94);
        }

    }

    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
    ) {
        let (assign59340_e92302,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2446]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign59340_e92302;

        let (assign59350_e92311,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[947] != 0.0)) {
        (0.0,)
    } else {
        (s.v[947],)
    }
};
        s.v[947] = assign59350_e92311;

        let (assign59360_e92318,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.v[98] = assign59360_e92318;

        let mut assign59370_loop_guard: usize = 0;
        while {
            let assign59370_cond_e92326: f64 = (40.0 + 1.0);
            let assign59370_cond_e92328: f64 = if (((!s.b[1441]) && s.b[2428]) && (s.v[98] <= assign59370_cond_e92326)) { 1.0 } else { 0.0 };
            assign59370_cond_e92328 != 0.0
        } {
            assign59370_loop_guard += 1;
            assert!(assign59370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_mul_sub_rhs(116, 154, 91, 1433);
            }
            s.b[2451] = (s.v[116] < 5.0);
            s.v[2451] = if s.b[2451] { 1.0 } else { 0.0 };
            if (((!s.b[1441]) && s.b[2428]) && s.b[2451]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul(222, 210, 230);
                s.store_mul3_lhs(218, 222, 225, 225);
                s.store_mul_product3_rhs(219, 226, s.ad_value(222), s.ad_value(154), s.ad_value(225), 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(220, 223, 218);
                s.store_div_scaled_inputs2_mixed_aii(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 219, 1.0, 220, 2.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && (!s.b[2451])) {
                s.store_mul_sub_rhs(117, 154, 91, 790);
                s.store_exp(228, 117);
                s.store_mul_sub_rhs(218, 210, 228, 230);
                s.store_mul3_lhs(219, 210, 154, 228);
                s.store_offset(102, 116, (-1.0));
                s.store_sqrt_add(220, 102, 218);
                s.store_div_scaled_inputs2_indices(221, 154, 1.0, 219, 1.0, 220, 2.0);
            }
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));
                s.store_sub_from_scalar_scaled_mul(235, (-1.0), 212, 221, 1.0);
            }
            s.b[2452] = (s.v[79] == 1.0);
            s.v[2452] = if s.b[2452] { 1.0 } else { 0.0 };
            let (assign59370_body22_e92674,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2452]) {
        (1.0,)
    } else {
        (s.v[945],)
    }
};
            s.v[945] = assign59370_body22_e92674;
            s.b[2453] = (s.v[945] == 0.0);
            s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {
                s.store_div_scaled_inputs_indices(237, 234, -1.0, 235, 1.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2454] = (((s.v[237]) as f64).abs() > s.v[93]);
            s.v[2454] = if s.b[2454] { 1.0 } else { 0.0 };
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2453]) && s.b[2454]) {
                s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {
                s.store_add(91, 91, 237);
            }
            s.b[2455] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));
            s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };
            let (assign59370_body30_e92765,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2453]) && s.b[2455]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign59370_body30_e92765;
            let (assign59370_body31_e92776,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[945] != 0.0)) {
        let assign59370_body31_e92774: f64 = (40.0 + 1.0);
        (assign59370_body31_e92774,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign59370_body31_e92776;
            let (assign59370_body32_e92783,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        (0.0,)
    } else {
        (s.v[945],)
    }
};
            s.v[945] = assign59370_body32_e92783;
            let (assign59370_body33_e92792,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let assign59370_body33_e92790: f64 = (s.v[98] + 1.0);
        (assign59370_body33_e92790,)
    } else {
        (s.v[98],)
    }
};
            s.v[98] = assign59370_body33_e92792;
        }

        let (assign59380_e92801,) = {
    if ((!s.b[1441]) && s.b[2428]) {
        let assign59380_e92799: f64 = (s.v[98] - 1.0);
        (assign59380_e92799,)
    } else {
        (s.v[98],)
    }
};
        s.v[98] = assign59380_e92801;

        s.b[2457] = (s.v[116] < 5.0);
        s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2457]) {
            s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2457])) {
            s.store_offset(102, 116, (-1.0));
            s.store_sqrt(103, 102);
            s.store_mul(104, 102, 103);
        }

        if ((!s.b[1441]) && s.b[2428]) {
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
            s.store_mul_product3_rhs(268, 335, A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), s.ad_value(101), s.ad_value(265), 1.0);
            s.store_sub_ad_lhs(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);
            s.store_add_scaled_inputs4_indices(335, 85, 1.0, 155, 1.0, 87, (-(2.0 * 0.5)), 94, (-0.5));
            s.store_sub(336, 266, 267);
            s.store_mul(337, 154, 185);
            s.store_mul(338, 154, 209);
            s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);
            s.store_mul(248, 94, 250);
        }

        s.b[2458] = (s.v[347] == 1.0);
        s.v[2458] = if s.b[2458] { 1.0 } else { 0.0 };

        let (assign59710_e93158,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2458]) {
        (1.0,)
    } else {
        (s.v[948],)
    }
};
        s.v[948] = assign59710_e93158;

        s.b[2459] = (s.v[948] == 0.0);
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        s.b[2460] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2461] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign59820_e93311,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign59820_e93311;

        let (assign59830_e93324,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59830_e93324;

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2462] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        s.b[2463] = (2.0 == 1.0);
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        let (assign59940_e93473,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59940_e93473;

        s.b[2464] = (2.0 == 2.0);
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        let (assign59960_e93496,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) && s.b[2464]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59960_e93496;

        s.b[2465] = (2.0 == 4.0);
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        let (assign59980_e93522,) = {
    if (((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) && (!s.b[2464])) && s.b[2465]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign59980_e93522;

        s.b[2466] = (2.0 == 8.0);
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        let (assign60000_e93551,) = {
    if ((((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) && (!s.b[2464])) && (!s.b[2465])) && s.b[2466]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60000_e93551;

        let (assign60010_e93566,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign60010_e93566;

        let mut assign60020_loop_guard: usize = 0;
        while {
            let assign60020_cond_e93582: f64 = if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60020_cond_e93582 != 0.0
        } {
            assign60020_loop_guard += 1;
            assert!(assign60020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) {
                s.store_sqrt(726, 726);
            }
            let (assign60020_body1_e93615,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && s.b[2462]) {
        let assign60020_body1_e93613: f64 = (s.v[719] + 1.0);
        (assign60020_body1_e93613,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign60020_body1_e93615;
        }

        if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) && (!s.b[2462])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) {
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && (!s.b[2461])) {
            s.store_scalar(334, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
    ) {
        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) {
            s.copy_ad(335, 684);
            s.store_sqrt_sub(342, 91, 1433);
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

        s.b[2467] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign60300_e94077,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign60300_e94077;

        let (assign60310_e94091,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60310_e94091;

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2468] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        s.b[2469] = (2.0 == 1.0);
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        let (assign60420_e94249,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && s.b[2469]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60420_e94249;

        s.b[2470] = (2.0 == 2.0);
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        let (assign60440_e94273,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (!s.b[2469])) && s.b[2470]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60440_e94273;

        s.b[2471] = (2.0 == 4.0);
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        let (assign60460_e94300,) = {
    if (((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (!s.b[2469])) && (!s.b[2470])) && s.b[2471]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60460_e94300;

        s.b[2472] = (2.0 == 8.0);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        let (assign60480_e94330,) = {
    if ((((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (!s.b[2469])) && (!s.b[2470])) && (!s.b[2471])) && s.b[2472]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60480_e94330;

        let (assign60490_e94346,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign60490_e94346;

        let mut assign60500_loop_guard: usize = 0;
        while {
            let assign60500_cond_e94363: f64 = if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60500_cond_e94363 != 0.0
        } {
            assign60500_loop_guard += 1;
            assert!(assign60500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) {
                s.store_sqrt(726, 726);
            }
            let (assign60500_body1_e94398,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && s.b[2468]) {
        let assign60500_body1_e94396: f64 = (s.v[719] + 1.0);
        (assign60500_body1_e94396,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign60500_body1_e94398;
        }

        if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) && (!s.b[2468])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && s.b[2467]) {
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && (!s.b[2467])) {
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) && (!s.b[2467])) {
            s.store_scalar(334, 1.0);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2460])) {
            s.store_sub(340, 168, 91);
            s.store_mul(337, 154, 238);
            s.store_div_from_scalar(335, 1.0, 337);
            s.store_mul_ad_product_lhs(339, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), s.ad_value(250), 335);
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

        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
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

        let (assign60960_e95081,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign60960_e95081;

        let (assign60970_e95090,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign60970_e95090;

        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
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

        s.b[2473] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        s.b[2474] = (4.0 == 1.0);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

        let (assign61120_e95247,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && s.b[2474]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61120_e95247;

        s.b[2475] = (4.0 == 2.0);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        let (assign61140_e95266,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (!s.b[2474])) && s.b[2475]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61140_e95266;

        s.b[2476] = (4.0 == 4.0);
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        let (assign61160_e95288,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (!s.b[2474])) && (!s.b[2475])) && s.b[2476]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61160_e95288;

        s.b[2477] = (4.0 == 8.0);
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        let (assign61180_e95313,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (!s.b[2474])) && (!s.b[2475])) && (!s.b[2476])) && s.b[2477]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61180_e95313;

    }

    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign61190_e95324,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign61190_e95324;

        let mut assign61200_loop_guard: usize = 0;
        while {
            let assign61200_cond_e95336: f64 = if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61200_cond_e95336 != 0.0
        } {
            assign61200_loop_guard += 1;
            assert!(assign61200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) {
                s.store_sqrt(726, 726);
            }
            let (assign61200_body1_e95361,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2473]) {
        let assign61200_body1_e95359: f64 = (s.v[719] + 1.0);
        (assign61200_body1_e95359,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign61200_body1_e95361;
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2473])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(338, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(125, 1.0, 333);
            s.store_offset_mul_offset_rhs(242, 125, 125, 1.0, 1.0);
        }

        s.b[2478] = (((1.0 + s.v[125]) < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
            s.store_sub_from_scalar_ad(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), A::offset(s.ad_value(125), 1.0));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign61330_e95550,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign61330_e95550;

        let (assign61340_e95561,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61340_e95561;

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2479] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        s.b[2480] = (2.0 == 1.0);
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

        let (assign61450_e95692,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && s.b[2480]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61450_e95692;

        s.b[2481] = (2.0 == 2.0);
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        let (assign61470_e95713,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (!s.b[2480])) && s.b[2481]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61470_e95713;

        s.b[2482] = (2.0 == 4.0);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        let (assign61490_e95737,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (!s.b[2480])) && (!s.b[2481])) && s.b[2482]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61490_e95737;

        s.b[2483] = (2.0 == 8.0);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        let (assign61510_e95764,) = {
    if (((((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (!s.b[2480])) && (!s.b[2481])) && (!s.b[2482])) && s.b[2483]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign61510_e95764;

        let (assign61520_e95777,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign61520_e95777;

        let mut assign61530_loop_guard: usize = 0;
        while {
            let assign61530_cond_e95791: f64 = if ((((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61530_cond_e95791 != 0.0
        } {
            assign61530_loop_guard += 1;
            assert!(assign61530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) {
                s.store_sqrt(726, 726);
            }
            let (assign61530_body1_e95820,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && s.b[2479]) {
        let assign61530_body1_e95818: f64 = (s.v[719] + 1.0);
        (assign61530_body1_e95818,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign61530_body1_e95820;
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) && (!s.b[2479])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(243, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2478]) {
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && (!s.b[2478])) {
            s.store_offset(243, 125, 1.0);
            s.store_scalar(334, 1.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2459]) {
            s.store_div_scaled_product_indices(335, 127, 242, 0.6666666666666667, 243, 1.0);
            s.store_mul(133, 335, 185);
            s.store_offset(244, 125, 0.5);
            s.store_mul(245, 243, 242);
            s.store_div_scaled_inputs_indices(246, 244, 0.4, 245, 1.0);
            s.store_sub_from_scalar(247, 0.6, 246);
        }

        s.b[2484] = (s.v[247] > 0.5);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2484]) {
            s.store_scalar(247, 0.5);
        }

        s.b[2485] = (s.v[347] == 2.0);
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) {
            s.copy_ad(335, 131);
            s.store_add_scaled_product_value_ad(131, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(239)), 1.0, 207, 131, 1.0);
        }

        s.b[2486] = (s.v[131] < 0.0);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) && s.b[2486]) {
            s.store_scalar(131, 0.0);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) {
            s.copy_ad(335, 133);
            s.store_add_scaled_product_value_ad(133, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(238)), 1.0, 207, 133, 1.0);
        }

        s.b[2487] = (s.v[133] < 0.0);
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) && s.b[2487]) {
            s.store_scalar(133, 0.0);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2485]) {
            s.copy_ad(335, 247);
            s.store_add_scaled_product_value_ad(247, A::scale_offset(s.ad_value(207), (-0.5), 0.5), 1.0, 207, 247, 1.0);
            s.copy_ad(335, 169);
            s.store_mul(169, 207, 169);
        }

        let (assign61830_e96194,) = {
    if (((!s.b[1441]) && s.b[2428]) && (s.v[948] != 0.0)) {
        (0.0,)
    } else {
        (s.v[948],)
    }
};
        s.v[948] = assign61830_e96194;

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_sub(170, 162, 169);
        }

        s.b[2488] = (s.v[170] < 1e-9);
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2488]) {
            s.store_scalar(170, 1e-9);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_scalar(335, (s.v[625] / 100.0));
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_mul_ad(338, A::sub(s.ad_value(91), s.ad_value(87)), s.ad_value(334), 1.0);
            s.store_add_scaled_products_indices(339, 335, 131, 1.0, 336, 133, 1.0);
            s.store_div(337, 339, 338);
            s.store_mul_scale_offset_rhs(251, 337, 1436, p.p166, 1.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(342, 339, 251);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 133, 343);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), s.v[474])), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_square(337, 335);
            s.store_mul_neg_lhs(338, 154, 337);
            s.store_mul(339, 338, 170);
            s.store_mul_offset_rhs(340, 338, 238, 1e-25);
            s.store_mul_ad_product_lhs(333, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), s.ad_value(250), 335);
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
            s.store_div_scaled_inputs_indices(337, 336, -1.0, 254, 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2489] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2489]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2490] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2489])) && s.b[2490]) {
            s.copy_ad(337, 335);
        }

        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2489])) && (!s.b[2490])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2491] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2491]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2492] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2491])) && s.b[2492]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2491])) && (!s.b[2492])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2491])) && (!s.b[2492])) {
            s.store_mul(339, 338, 340);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(253, 254, 339);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_div_scaled_inputs_indices(335, 115, -1.0, 170, 1.0);
            s.store_mul3_lhs(135, 115, 248, 253);
        }

        s.b[2493] = (p.p283 != 0.0);
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2493]) {
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

        s.b[2494] = (s.v[336] < 0.0);
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2493]) && s.b[2494]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2493]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1437, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 87, 1.0, 340, 1.0, 1436, -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1437), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2493])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2495] = (p.p287 != 0.0);
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2495]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1437);
        }

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2495])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2496] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2496]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_mul3_lhs(45, 115, 249, 253);
            s.store_add(135, 135, 45);
        }

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2496])) {
            s.store_scalar(45, 0.0);
        }

        s.b[2497] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        s.b[2498] = (p.p296 > 0.0);
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2498]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && (!s.b[2498])) {
            s.copy_ad(341, 647);
        }

        s.b[2499] = (s.v[793] >= 0.0);
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2499]) {
            s.copy_ad(369, 793);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && (!s.b[2499])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2500] = (s.v[369] < (20.0 * 1e-12));
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && s.b[2500]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2497]) && (!s.b[2500])) {
            s.store_powf_offset_input(335, 369, 1e-12, p.p297);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2497]) {
            s.store_powf_offset_input(343, 369, 1e-12, p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2497])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        s.b[2501] = (p.p27 != 0.0);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_scale(335, 186, 1.034943e-10);
            s.copy_ad(336, 684);
            s.store_scalar(337, (s.v[628] - p.p139));
            s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));
            s.store_mul_ad_product_lhs(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), s.ad_value(336), 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);
            s.store_mul_ad_product_lhs(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.ad_value(338), 181);
            s.store_mul_product3_rhs(342, 181, s.ad_value(335), s.ad_value(336), s.ad_value(338), (-2.0));
            s.store_scalar(338, s.v[496]);
            s.store_scalar(340, s.v[497]);
            s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1437, 1.0);
            s.store_mul(137, 121, 335);
            s.store_sub_from_scalar_scaled_input(335, s.v[498], 790, p.p213);
            s.store_add_scaled_inputs3_offset_indices(138, 1438, 1.0, 335, 1.0, 137, 1.0, (-s.v[160]));
            s.store_mul3_lhs(141, 694, 186, 186);
            s.store_scaled_mul(142, 141, 154, 0.5);
            s.store_scaled_mul(143, 142, 154, 2.0);
            s.store_scale(345, 154, 0.25);
            s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);
            s.store_offset_sub(335, 1438, 344, (-0.005));
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_sqrt_add_scaled_square_product(336, 335, 1.0, 334, 344, (4.0 * 0.005));
            s.store_sub_ad_lhs(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1436);
            s.store_offset_mul(338, 154, 337, (-1.0));
            s.store_div_from_scalar(339, 4.0, 143);
            s.store_offset_mul(335, 338, 339, 1.0);
            s.store_mul(340, 154, 339);
            s.store_mul(341, 338, 339);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2502] = (s.v[335] < 0.0);
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2502]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
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
