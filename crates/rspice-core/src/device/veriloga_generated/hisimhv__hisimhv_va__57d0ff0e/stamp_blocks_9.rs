#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            s.store_mul_sub_rhs(339, 154, 2090, 2093);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) {
            if (s.v[2090] > s.v[2093]) {
                s.store_mul_scaled_ad_rhs(2123, 209, -1.0, A::sqrt(s.ad_value(344)));
            } else {
                s.store_mul_sqrt_rhs(2123, 209, 344);
            }
        }

        s.b[2277] = (1.0 == 1.0);
        s.v[2277] = if s.b[2277] { 1.0 } else { 0.0 };

        s.b[2278] = (((s.v[2090] - s.v[2088]) < p.p403) && (p.p403 >= 0.0));
        s.v[2278] = if s.b[2278] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2090), s.ad_value(2088)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
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

        s.b[2279] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2279] = if s.b[2279] { 1.0 } else { 0.0 };

        s.b[2280] = (6.0 == 1.0);
        s.v[2280] = if s.b[2280] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && s.b[2279]) && s.b[2280]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2281] = (6.0 == 2.0);
        s.v[2281] = if s.b[2281] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && s.b[2279]) && (!s.b[2280])) && s.b[2281]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2282] = (6.0 == 4.0);
        s.v[2282] = if s.b[2282] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && s.b[2279]) && (!s.b[2280])) && (!s.b[2281])) && s.b[2282]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2283] = (6.0 == 8.0);
        s.v[2283] = if s.b[2283] { 1.0 } else { 0.0 };

        if (((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && s.b[2279]) && (!s.b[2280])) && (!s.b[2281])) && (!s.b[2282])) && s.b[2283]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && s.b[2279]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign49350_loop_guard: usize = 0;
        while {
            let assign49350_cond_e74068: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && s.b[2279]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49350_cond_e74068 != 0.0
        } {
            assign49350_loop_guard += 1;
            assert!(assign49350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && s.b[2279]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) && (!s.b[2279])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && s.b[2278]) {
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) && (!s.b[2278])) {
            s.store_sub(336, 2090, 2088);
            s.store_scalar(334, 1.0);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && s.b[2277]) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(2119, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2252])) && (!s.b[2277])) {
            s.copy_ad(2119, 2123);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.copy_ad(87, 2089);
            s.copy_ad(91, 2090);
            s.store_sub(94, 2090, 2089);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
        }

        s.b[2284] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2284] = if s.b[2284] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2285] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2285] = if s.b[2285] { 1.0 } else { 0.0 };

        s.b[2286] = (2.0 == 1.0);
        s.v[2286] = if s.b[2286] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && s.b[2285]) && s.b[2286]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2287] = (2.0 == 2.0);
        s.v[2287] = if s.b[2287] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && s.b[2285]) && (!s.b[2286])) && s.b[2287]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2288] = (2.0 == 4.0);
        s.v[2288] = if s.b[2288] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && s.b[2285]) && (!s.b[2286])) && (!s.b[2287])) && s.b[2288]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2289] = (2.0 == 8.0);
        s.v[2289] = if s.b[2289] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && s.b[2285]) && (!s.b[2286])) && (!s.b[2287])) && (!s.b[2288])) && s.b[2289]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && s.b[2285]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign49820_loop_guard: usize = 0;
        while {
            let assign49820_cond_e74923: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && s.b[2285]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49820_cond_e74923 != 0.0
        } {
            assign49820_loop_guard += 1;
            assert!(assign49820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && s.b[2285]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) && (!s.b[2285])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2284]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2284])) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2284])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_add(109, 87, 110);
        }

        s.b[2290] = (((s.v[109] - s.v[2087]) < p.p403) && (p.p403 >= 0.0));
        s.v[2290] = if s.b[2290] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(109), s.ad_value(2087)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
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

        s.b[2291] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2291] = if s.b[2291] { 1.0 } else { 0.0 };

        s.b[2292] = (6.0 == 1.0);
        s.v[2292] = if s.b[2292] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && s.b[2292]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2293] = (6.0 == 2.0);
        s.v[2293] = if s.b[2293] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (!s.b[2292])) && s.b[2293]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2294] = (6.0 == 4.0);
        s.v[2294] = if s.b[2294] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (!s.b[2292])) && (!s.b[2293])) && s.b[2294]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2295] = (6.0 == 8.0);
        s.v[2295] = if s.b[2295] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (!s.b[2292])) && (!s.b[2293])) && (!s.b[2294])) && s.b[2295]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign50260_loop_guard: usize = 0;
        while {
            let assign50260_cond_e75603: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50260_cond_e75603 != 0.0
        } {
            assign50260_loop_guard += 1;
            assert!(assign50260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && s.b[2291]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) && (!s.b[2291])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2290]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2290])) {
            s.store_sub(336, 109, 2087);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(2120, 209, -1.0, A::sqrt(s.ad_value(338)));
            s.store_sqrt_offset_ad(782, A::mul_scaled_lhs(A::add(s.ad_value(2119), s.ad_value(2118)), 1.0, A::add(s.ad_value(2119), s.ad_value(2118))), ((4.0 * (1e-12 * 1e-6)) * (1e-12 * 1e-6)));
            s.store_scaled_offset_ad(335, A::div_scaled_inputs2(s.ad_value(2119), -1.0, s.ad_value(2118), -1.0, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_add_scaled_inputs3(2121, s.ad_value(2119), (-0.5), s.ad_value(2118), (-0.5), s.ad_value(782), 0.5);
        }

        s.b[2296] = (s.v[2121] < 0.0);
        s.v[2296] = if s.b[2296] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2296]) {
            s.store_scalar(2121, 0.0);
            s.store_scalar(335, 0.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_neg(2121, 2121);
            s.store_mul3_affine_lhs(248, 154, 2121, (-1.0 / (2.0)), 0.0, 94);
            s.store_neg(238, 2120);
            s.copy_ad(170, 162);
            s.copy_ad(790, 349);
        }

        s.b[2297] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2297] = if s.b[2297] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2298] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2298] = if s.b[2298] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {
            s.store_offset_ad(781, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(87), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16)))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2299] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2299] = if s.b[2299] { 1.0 } else { 0.0 };

        s.b[2300] = (2.0 == 1.0);
        s.v[2300] = if s.b[2300] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && s.b[2300]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2301] = (2.0 == 2.0);
        s.v[2301] = if s.b[2301] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (!s.b[2300])) && s.b[2301]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2302] = (2.0 == 4.0);
        s.v[2302] = if s.b[2302] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (!s.b[2300])) && (!s.b[2301])) && s.b[2302]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2303] = (2.0 == 8.0);
        s.v[2303] = if s.b[2303] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (!s.b[2300])) && (!s.b[2301])) && (!s.b[2302])) && s.b[2303]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign50770_loop_guard: usize = 0;
        while {
            let assign50770_cond_e76439: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50770_cond_e76439 != 0.0
        } {
            assign50770_loop_guard += 1;
            assert!(assign50770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && s.b[2299]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) && (!s.b[2299])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset(168, s.ad_value(87), 1.0, s.ad_value(790), 1.0, s.ad_value(780), 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && s.b[2298]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && (!s.b[2298])) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2297]) && (!s.b[2298])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) {
            s.store_sub(342, 91, 2117);
        }

        s.b[2304] = ((s.v[342] < (0.2 + ((-s.v[2117]) + 0.8))) && (((-s.v[2117]) + 0.8) >= 0.0));
        s.v[2304] = if s.b[2304] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {
            s.store_sub_ad_lhs(781, A::offset(A::sub_from_scalar(0.8, s.ad_value(2117)), 0.2), 342);
            s.store_square(722, 781);
            s.store_ad_value(723, A::mul_sub_from_scalar_lhs(0.8, s.ad_value(2117), A::sub_from_scalar(0.8, s.ad_value(2117))));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2305] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2305] = if s.b[2305] { 1.0 } else { 0.0 };

        s.b[2306] = (1.0 == 1.0);
        s.v[2306] = if s.b[2306] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && s.b[2306]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2307] = (1.0 == 2.0);
        s.v[2307] = if s.b[2307] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (!s.b[2306])) && s.b[2307]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2308] = (1.0 == 4.0);
        s.v[2308] = if s.b[2308] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (!s.b[2306])) && (!s.b[2307])) && s.b[2308]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2309] = (1.0 == 8.0);
        s.v[2309] = if s.b[2309] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (!s.b[2306])) && (!s.b[2307])) && (!s.b[2308])) && s.b[2309]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign51110_loop_guard: usize = 0;
        while {
            let assign51110_cond_e77064: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51110_cond_e77064 != 0.0
        } {
            assign51110_loop_guard += 1;
            assert!(assign51110_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && s.b[2305]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) && (!s.b[2305])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul_ad_lhs(780, A::mul_sub_from_scalar_rhs(s.ad_value(781), 0.8, s.ad_value(2117)), 726);
            s.store_div_scaled_product_left_ad(334, A::mul_sub_from_scalar_lhs(0.8, s.ad_value(2117), s.ad_value(725)), 726, 1.0, 770, 1.0);
            s.store_sub_ad_lhs(342, A::offset(A::sub_from_scalar(0.8, s.ad_value(2117)), 0.2), 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2304]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2304])) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2304])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) {
            s.store_mul(343, 2136, 342);
            s.store_sqrt(171, 343);
            s.store_div_from_scalar(334, 1.0, 171);
            s.store_mul(335, 238, 334);
            s.store_scale(336, 335, s.v[509]);
            s.store_scale(337, 334, s.v[509]);
            s.store_add_scaled_product_indices(339, 336, 1.0, 508, 2133, 1.0);
            s.store_div_from_scalar(335, 1.0, 339);
            s.store_scale(338, 335, 1.034943e-10);
            s.store_scalar(335, (1.0 - s.v[507]));
            s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 109, s.v[507], 335, 91, 1.0);
        }

        s.b[2310] = ((s.v[168] > (((s.v[109] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2310] = if s.b[2310] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {
            s.store_offset_ad(781, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(109), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16)))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2311] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2311] = if s.b[2311] { 1.0 } else { 0.0 };

        s.b[2312] = (2.0 == 1.0);
        s.v[2312] = if s.b[2312] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && s.b[2312]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2313] = (2.0 == 2.0);
        s.v[2313] = if s.b[2313] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (!s.b[2312])) && s.b[2313]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2314] = (2.0 == 4.0);
        s.v[2314] = if s.b[2314] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (!s.b[2312])) && (!s.b[2313])) && s.b[2314]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2315] = (2.0 == 8.0);
        s.v[2315] = if s.b[2315] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (!s.b[2312])) && (!s.b[2313])) && (!s.b[2314])) && s.b[2315]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign51570_loop_guard: usize = 0;
        while {
            let assign51570_cond_e77906: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51570_cond_e77906 != 0.0
        } {
            assign51570_loop_guard += 1;
            assert!(assign51570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && s.b[2311]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) && (!s.b[2311])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset(168, s.ad_value(109), 1.0, s.ad_value(790), 1.0, s.ad_value(780), 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && s.b[2310]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2310])) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) && (!s.b[2310])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2297])) {
            s.store_sub(340, 168, 91);
            s.store_mul(337, 154, 238);
            s.store_div_from_scalar(335, 1.0, 337);
            s.store_mul(339, 248, 335);
            s.store_scale(344, 2133, 9662367879.197212);
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

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
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

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 251, p.p160);
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(251), s.ad_value(624));
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs(336, s.ad_value(257), 0.2, s.ad_value(254), 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2316] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2316] = if s.b[2316] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2316]) {
            s.copy_ad(336, 335);
        }

        s.b[2317] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2317] = if s.b[2317] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2316])) && s.b[2317]) {
            s.store_square(336, 335);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2316])) && (!s.b[2317])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p178);
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_offset(338, 336, 1.0);
        }

        s.b[2318] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2318] = if s.b[2318] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2318]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2319] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2319] = if s.b[2319] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2318])) && s.b[2319]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2318])) && (!s.b[2319])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 338, ((-1.0) / p.p178));
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(2116, 255);
            s.store_scalar(2324, 0.0);
            s.store_scalar(2155, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(2147, 0.0);
            s.store_scalar(2322, 0.0);
            s.store_add_scaled_inputs3_offset(2144, s.ad_value(1440), 1.0, s.ad_value(120), 1.0, s.ad_value(182), -1.0, (-s.v[160]));
        }

        s.b[2326] = (0.0 == 0.0);
        s.v[2326] = if s.b[2326] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2326]) {
            s.store_offset(2145, 2144, (-p.p393));
        }

        s.b[2327] = (0.0 == 1.0);
        s.v[2327] = if s.b[2327] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2326])) && s.b[2327]) {
            s.store_offset(2145, 1440, (((-s.v[160])) + ((-p.p393))));
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2326])) && (!s.b[2327])) {
            s.store_offset(2145, 85, (-p.p393));
        }

        s.b[2328] = (((s.v[2148]) as f64).abs() <= 0.0);
        s.v[2328] = if s.b[2328] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2328]) {
            s.store_scalar(2153, 0.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.copy_ad(983, 87);
            s.store_scale(2170, 2117, p.p399);
            s.store_scalar(2325, ((s.v[160] + p.p393) - 3.0));
        }

        s.b[2329] = (1.0 == 1.0);
        s.v[2329] = if s.b[2329] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2329]) {
            s.store_scale(2170, 2117, p.p399);
            s.store_offset(983, 2170, (-1.0));
            s.copy_ad(2324, 2325);
            s.copy_ad(2146, 2325);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2329])) {
            s.store_offset_scaled(2170, 2117, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2324, 2145);
            s.copy_ad(2146, 2145);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign52470_loop_guard: usize = 0;
        while {
            let assign52470_cond_e79358: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign52470_cond_e79358 != 0.0
        } {
            assign52470_loop_guard += 1;
            assert!(assign52470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2330] = (s.v[983] >= 0.0);
            s.v[2330] = if s.b[2330] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2330]) {
                s.store_mul_scaled_ad_rhs(2322, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(2125, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2322), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2330])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(983), s.ad_value(2170))));
                s.store_exp_mul(338, 154, 2170);
                s.store_mul_sqrt_ad_rhs(2322, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2322, 1.0);
                s.store_mul_add_ad_rhs(2125, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2100, 2322, 1.0, 185, A::sub(s.ad_value(2324), s.ad_value(983)), 1.0);
                s.store_sub(2101, 2125, 185);
                s.store_div_scaled_inputs(2112, s.ad_value(2100), -1.0, s.ad_value(2101), 1.0);
            }
            s.b[2331] = (((s.v[2112]) as f64).abs() < (1e-10 * 100.0));
            s.v[2331] = if s.b[2331] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && s.b[2331]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2332] = (s.v[2112] > 0.1);
            s.v[2332] = if s.b[2332] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2331])) && s.b[2332]) {
                s.store_scalar(2112, 0.1);
            }
            s.b[2333] = (s.v[2112] < (-0.1));
            s.v[2333] = if s.b[2333] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2331])) && (!s.b[2332])) && s.b[2333]) {
                s.store_scalar(2112, (-0.1));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2112);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[2335] = (1.0 == 1.0);
        s.v[2335] = if s.b[2335] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2335]) {
            s.copy_ad(2171, 983);
        }

        s.b[2336] = ((s.v[983] < (s.v[2171] + 0.2)) && (0.2 >= 0.0));
        s.v[2336] = if s.b[2336] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {
            s.store_sub_ad_lhs(781, A::offset(s.ad_value(2171), 0.2), 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2337] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2337] = if s.b[2337] { 1.0 } else { 0.0 };

        s.b[2338] = (2.0 == 1.0);
        s.v[2338] = if s.b[2338] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && s.b[2338]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2339] = (2.0 == 2.0);
        s.v[2339] = if s.b[2339] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (!s.b[2338])) && s.b[2339]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2340] = (2.0 == 4.0);
        s.v[2340] = if s.b[2340] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (!s.b[2338])) && (!s.b[2339])) && s.b[2340]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2341] = (2.0 == 8.0);
        s.v[2341] = if s.b[2341] { 1.0 } else { 0.0 };

        if (((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (!s.b[2338])) && (!s.b[2339])) && (!s.b[2340])) && s.b[2341]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign52770_loop_guard: usize = 0;
        while {
            let assign52770_cond_e80284: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign52770_cond_e80284 != 0.0
        } {
            assign52770_loop_guard += 1;
            assert!(assign52770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && s.b[2337]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) && (!s.b[2337])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_ad_lhs(983, A::offset(s.ad_value(2171), 0.2), 780);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && s.b[2336]) {
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && (!s.b[2336])) {
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2335])) && (!s.b[2336])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.copy_ad(2153, 983);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_scalar(2142, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2342] = ((s.v[2153] > (-s.v[2142])) && (s.v[2142] >= 0.0));
        s.v[2342] = if s.b[2342] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
            s.store_add(781, 2153, 2142);
            s.store_square(722, 781);
            s.store_square(723, 2142);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

        let mut assign52990_loop_guard: usize = 0;
        while {
            let assign52990_cond_e80740: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && (s.v[719] < s.v[2143])) { 1.0 } else { 0.0 };
            assign52990_cond_e80740 != 0.0
        } {
            assign52990_loop_guard += 1;
            assert!(assign52990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2343] = ((((s.v[2143] == 1.0) || (s.v[2143] == 2.0)) || (s.v[2143] == 4.0)) || (s.v[2143] == 8.0));
        s.v[2343] = if s.b[2343] { 1.0 } else { 0.0 };

        s.b[2344] = (s.v[2143] == 1.0);
        s.v[2344] = if s.b[2344] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && s.b[2344]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2345] = (s.v[2143] == 2.0);
        s.v[2345] = if s.b[2345] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (!s.b[2344])) && s.b[2345]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2346] = (s.v[2143] == 4.0);
        s.v[2346] = if s.b[2346] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (!s.b[2344])) && (!s.b[2345])) && s.b[2346]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2347] = (s.v[2143] == 8.0);
        s.v[2347] = if s.b[2347] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (!s.b[2344])) && (!s.b[2345])) && (!s.b[2346])) && s.b[2347]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign53120_loop_guard: usize = 0;
        while {
            let assign53120_cond_e80990: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53120_cond_e80990 != 0.0
        } {
            assign53120_loop_guard += 1;
            assert!(assign53120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && s.b[2343]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) && (!s.b[2343])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2143), 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2142, 726);
            s.store_div_scaled_product3_indices(334, 2142, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2142, -1.0, 780, 1.0);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2342]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2342])) {
            s.copy_ad(983, 2153);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2320, 2133, 2148, (0.5 * 9662367879.197212), 0.0, 2148);
            s.store_mul_sqrt_ad_rhs(334, 2152, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2320)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2348] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2348] = if s.b[2348] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2348]) {
            s.store_div_ad_lhs(2321, A::ln(s.ad_value(335)), 2320);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2348])) {
            s.store_ad_value(2321, A::mul3(A::square(s.ad_value(2152)), s.ad_value(154), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334)))));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_mul(332, 2321, 983);
        }

        s.b[2349] = (s.v[332] > 500.0);
        s.v[2349] = if s.b[2349] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2349]) {
            s.store_sub(2165, 983, 2320);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) {
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(2321), -1.0, s.ad_value(2320)));
        }

        s.b[2350] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2350] = if s.b[2350] { 1.0 } else { 0.0 };

        s.b[2351] = (s.v[332] >= 500.0);
        s.v[2351] = if s.b[2351] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && s.b[2351]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign53380_loop_guard: usize = 0;
        while {
            let assign53380_cond_e81500: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign53380_cond_e81500 != 0.0
        } {
            assign53380_loop_guard += 1;
            assert!(assign53380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) && (!s.b[2351])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2350]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && (!s.b[2350])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2352] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2352] = if s.b[2352] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && s.b[2352]) {
            s.store_div_ad_lhs(2165, A::ln(A::offset(s.ad_value(336), 1.0)), 2321);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2349])) && (!s.b[2352])) {
            s.store_div(2165, 336, 2321);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_sub(336, 983, 2165);
        }

        s.b[2353] = (0.0 == 0.0);
        s.v[2353] = if s.b[2353] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2353]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2147, A::sqrt(A::mul_scaled_lhs(s.ad_value(2136), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2147, 2136, 336);
            }
        }

        s.b[2354] = (s.v[336] < 0.0);
        s.v[2354] = if s.b[2354] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2353])) && s.b[2354]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2147, A::sqrt(A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2353])) && (!s.b[2354])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2147, A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_sub(990, 2148, 2147);
        }

        s.b[2355] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2355] = if s.b[2355] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2356] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2356] = if s.b[2356] { 1.0 } else { 0.0 };

        s.b[2357] = (2.0 == 1.0);
        s.v[2357] = if s.b[2357] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && s.b[2357]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2358] = (2.0 == 2.0);
        s.v[2358] = if s.b[2358] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && s.b[2358]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2359] = (2.0 == 4.0);
        s.v[2359] = if s.b[2359] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && (!s.b[2358])) && s.b[2359]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2360] = (2.0 == 8.0);
        s.v[2360] = if s.b[2360] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (!s.b[2357])) && (!s.b[2358])) && (!s.b[2359])) && s.b[2360]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign53830_loop_guard: usize = 0;
        while {
            let assign53830_cond_e82339: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53830_cond_e82339 != 0.0
        } {
            assign53830_loop_guard += 1;
            assert!(assign53830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && s.b[2356]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) && (!s.b[2356])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2355]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2355])) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2355])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2361] = (1.0 == 1.0);
        s.v[2361] = if s.b[2361] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2361]) {
            s.copy_ad(2155, 990);
        }

        s.b[2362] = (2.0 == 1.0);
        s.v[2362] = if s.b[2362] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2362]) {
            s.store_scale(2170, 2117, p.p399);
            s.store_offset(983, 2170, (-1.0));
            s.copy_ad(2324, 2325);
            s.copy_ad(2146, 2325);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2362])) {
            s.store_offset_scaled(2170, 2117, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2324, 2145);
            s.copy_ad(2146, 2145);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign54050_loop_guard: usize = 0;
        while {
            let assign54050_cond_e82743: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign54050_cond_e82743 != 0.0
        } {
            assign54050_loop_guard += 1;
            assert!(assign54050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2363] = (s.v[983] >= 0.0);
            s.v[2363] = if s.b[2363] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2363]) {
                s.store_mul_scaled_ad_rhs(2322, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(2125, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2322), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2363])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(983), s.ad_value(2170))));
                s.store_exp_mul(338, 154, 2170);
                s.store_mul_sqrt_ad_rhs(2322, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2322, 1.0);
                s.store_mul_add_ad_rhs(2125, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2100, 2322, 1.0, 185, A::sub(s.ad_value(2324), s.ad_value(983)), 1.0);
                s.store_sub(2101, 2125, 185);
                s.store_div_scaled_inputs(2112, s.ad_value(2100), -1.0, s.ad_value(2101), 1.0);
            }
            s.b[2364] = (((s.v[2112]) as f64).abs() < (1e-10 * 100.0));
            s.v[2364] = if s.b[2364] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && s.b[2364]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2365] = (s.v[2112] > 0.1);
            s.v[2365] = if s.b[2365] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2364])) && s.b[2365]) {
                s.store_scalar(2112, 0.1);
            }
            s.b[2366] = (s.v[2112] < (-0.1));
            s.v[2366] = if s.b[2366] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) && (!s.b[2364])) && (!s.b[2365])) && s.b[2366]) {
                s.store_scalar(2112, (-0.1));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2112);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[2368] = (2.0 == 1.0);
        s.v[2368] = if s.b[2368] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2368]) {
            s.copy_ad(2171, 983);
        }

        s.b[2369] = ((s.v[983] < (s.v[2171] + 0.2)) && (0.2 >= 0.0));
        s.v[2369] = if s.b[2369] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {
            s.store_sub_ad_lhs(781, A::offset(s.ad_value(2171), 0.2), 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2370] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2370] = if s.b[2370] { 1.0 } else { 0.0 };

        s.b[2371] = (2.0 == 1.0);
        s.v[2371] = if s.b[2371] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && s.b[2371]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2372] = (2.0 == 2.0);
        s.v[2372] = if s.b[2372] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && s.b[2372]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2373] = (2.0 == 4.0);
        s.v[2373] = if s.b[2373] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && (!s.b[2372])) && s.b[2373]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2374] = (2.0 == 8.0);
        s.v[2374] = if s.b[2374] { 1.0 } else { 0.0 };

        if (((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (!s.b[2371])) && (!s.b[2372])) && (!s.b[2373])) && s.b[2374]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign54350_loop_guard: usize = 0;
        while {
            let assign54350_cond_e83669: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54350_cond_e83669 != 0.0
        } {
            assign54350_loop_guard += 1;
            assert!(assign54350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && s.b[2370]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) && (!s.b[2370])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_ad_lhs(983, A::offset(s.ad_value(2171), 0.2), 780);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && s.b[2369]) {
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && (!s.b[2369])) {
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2368])) && (!s.b[2369])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.copy_ad(2153, 983);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_scalar(2142, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2375] = ((s.v[2153] > (-s.v[2142])) && (s.v[2142] >= 0.0));
        s.v[2375] = if s.b[2375] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
            s.store_add(781, 2153, 2142);
            s.store_square(722, 781);
            s.store_square(723, 2142);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign54570_loop_guard: usize = 0;
        while {
            let assign54570_cond_e84125: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && (s.v[719] < s.v[2143])) { 1.0 } else { 0.0 };
            assign54570_cond_e84125 != 0.0
        } {
            assign54570_loop_guard += 1;
            assert!(assign54570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2376] = ((((s.v[2143] == 1.0) || (s.v[2143] == 2.0)) || (s.v[2143] == 4.0)) || (s.v[2143] == 8.0));
        s.v[2376] = if s.b[2376] { 1.0 } else { 0.0 };

        s.b[2377] = (s.v[2143] == 1.0);
        s.v[2377] = if s.b[2377] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && s.b[2377]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2378] = (s.v[2143] == 2.0);
        s.v[2378] = if s.b[2378] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && s.b[2378]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2379] = (s.v[2143] == 4.0);
        s.v[2379] = if s.b[2379] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && (!s.b[2378])) && s.b[2379]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2380] = (s.v[2143] == 8.0);
        s.v[2380] = if s.b[2380] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (!s.b[2377])) && (!s.b[2378])) && (!s.b[2379])) && s.b[2380]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign54700_loop_guard: usize = 0;
        while {
            let assign54700_cond_e84375: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54700_cond_e84375 != 0.0
        } {
            assign54700_loop_guard += 1;
            assert!(assign54700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && s.b[2376]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) && (!s.b[2376])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2143), 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2142, 726);
            s.store_div_scaled_product3_indices(334, 2142, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2142, -1.0, 780, 1.0);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2375]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2375])) {
            s.copy_ad(983, 2153);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2320, 2133, 2148, (0.5 * 9662367879.197212), 0.0, 2148);
            s.store_mul_sqrt_ad_rhs(334, 2152, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2320)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2381] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2381] = if s.b[2381] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2381]) {
            s.store_div_ad_lhs(2321, A::ln(s.ad_value(335)), 2320);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2381])) {
            s.store_ad_value(2321, A::mul3(A::square(s.ad_value(2152)), s.ad_value(154), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334)))));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_mul(332, 2321, 983);
        }

        s.b[2382] = (s.v[332] > 500.0);
        s.v[2382] = if s.b[2382] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2382]) {
            s.store_sub(2165, 983, 2320);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) {
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(2321), -1.0, s.ad_value(2320)));
        }

        s.b[2383] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2383] = if s.b[2383] { 1.0 } else { 0.0 };

        s.b[2384] = (s.v[332] >= 500.0);
        s.v[2384] = if s.b[2384] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && s.b[2384]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign54960_loop_guard: usize = 0;
        while {
            let assign54960_cond_e84885: f64 = if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign54960_cond_e84885 != 0.0
        } {
            assign54960_loop_guard += 1;
            assert!(assign54960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) && (!s.b[2384])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2383]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && (!s.b[2383])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2385] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2385] = if s.b[2385] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && s.b[2385]) {
            s.store_div_ad_lhs(2165, A::ln(A::offset(s.ad_value(336), 1.0)), 2321);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2382])) && (!s.b[2385])) {
            s.store_div(2165, 336, 2321);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_sub(336, 983, 2165);
        }

        s.b[2386] = (0.0 == 0.0);
        s.v[2386] = if s.b[2386] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2386]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2147, A::sqrt(A::mul_scaled_lhs(s.ad_value(2136), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2147, 2136, 336);
            }
        }

        s.b[2387] = (s.v[336] < 0.0);
        s.v[2387] = if s.b[2387] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2386])) && s.b[2387]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2147, A::sqrt(A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2386])) && (!s.b[2387])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2147, A::mul3(s.ad_value(2136), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) {
            s.store_sub(990, 2148, 2147);
        }

        s.b[2388] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2388] = if s.b[2388] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2389] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2389] = if s.b[2389] { 1.0 } else { 0.0 };

        s.b[2390] = (2.0 == 1.0);
        s.v[2390] = if s.b[2390] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && s.b[2390]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2391] = (2.0 == 2.0);
        s.v[2391] = if s.b[2391] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && s.b[2391]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2392] = (2.0 == 4.0);
        s.v[2392] = if s.b[2392] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && (!s.b[2391])) && s.b[2392]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2393] = (2.0 == 8.0);
        s.v[2393] = if s.b[2393] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (!s.b[2390])) && (!s.b[2391])) && (!s.b[2392])) && s.b[2393]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign55410_loop_guard: usize = 0;
        while {
            let assign55410_cond_e85724: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55410_cond_e85724 != 0.0
        } {
            assign55410_loop_guard += 1;
            assert!(assign55410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && s.b[2389]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) && (!s.b[2389])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2388]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2388])) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && (!s.b[2388])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2394] = (2.0 == 1.0);
        s.v[2394] = if s.b[2394] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2328])) && s.b[2394]) {
            s.copy_ad(2155, 990);
        }

        s.b[2395] = (0.0 == 0.0);
        s.v[2395] = if s.b[2395] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) {
            s.copy_ad(989, 349);
            s.store_scaled_add(344, 2117, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(2135), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 2135, 1.0);
        }

        s.b[2396] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2396] = if s.b[2396] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2397] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2397] = if s.b[2397] { 1.0 } else { 0.0 };

        s.b[2398] = (2.0 == 1.0);
        s.v[2398] = if s.b[2398] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && s.b[2398]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2399] = (2.0 == 2.0);
        s.v[2399] = if s.b[2399] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && s.b[2399]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2400] = (2.0 == 4.0);
        s.v[2400] = if s.b[2400] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && (!s.b[2399])) && s.b[2400]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2401] = (2.0 == 8.0);
        s.v[2401] = if s.b[2401] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (!s.b[2398])) && (!s.b[2399])) && (!s.b[2400])) && s.b[2401]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign55830_loop_guard: usize = 0;
        while {
            let assign55830_cond_e86417: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55830_cond_e86417 != 0.0
        } {
            assign55830_loop_guard += 1;
            assert!(assign55830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && s.b[2397]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) && (!s.b[2397])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2396]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && (!s.b[2396])) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && (!s.b[2396])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(2134), 1.0, s.ad_value(337)));
        }

        s.b[2402] = ((s.v[344] < (s.v[972] + p.p405)) && (p.p405 >= 0.0));
        s.v[2402] = if s.b[2402] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {
            s.store_sub_ad_lhs(781, A::offset(s.ad_value(972), p.p405), 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p405 * p.p405));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2403] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2403] = if s.b[2403] { 1.0 } else { 0.0 };

        s.b[2404] = (2.0 == 1.0);
        s.v[2404] = if s.b[2404] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && s.b[2404]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2405] = (2.0 == 2.0);
        s.v[2405] = if s.b[2405] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && s.b[2405]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2406] = (2.0 == 4.0);
        s.v[2406] = if s.b[2406] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && (!s.b[2405])) && s.b[2406]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2407] = (2.0 == 8.0);
        s.v[2407] = if s.b[2407] { 1.0 } else { 0.0 };

        if ((((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (!s.b[2404])) && (!s.b[2405])) && (!s.b[2406])) && s.b[2407]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign56200_loop_guard: usize = 0;
        while {
            let assign56200_cond_e87047: f64 = if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56200_cond_e87047 != 0.0
        } {
            assign56200_loop_guard += 1;
            assert!(assign56200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && s.b[2403]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) && (!s.b[2403])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p405);
            s.store_div_scaled_product_indices(334, 725, 726, p.p405, 770, 1.0);
            s.store_sub_ad_lhs(992, A::offset(s.ad_value(972), p.p405), 780);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && s.b[2402]) {
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2395]) && (!s.b[2402])) {
            s.copy_ad(992, 344);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
            s.copy_ad(2159, 2145);
            s.store_offset_mul(338, 2135, 2159, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_neg_ad(337, A::sqrt_scaled_input(s.ad_value(338), -1.0));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
            s.store_add_ad_rhs(2160, 2159, A::mul_sub_from_scalar_rhs(s.ad_value(2134), 1.0, s.ad_value(337)));
            s.copy_ad(2156, 2160);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign56360_loop_guard: usize = 0;
        while {
            let assign56360_cond_e87368: f64 = if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign56360_cond_e87368 != 0.0
        } {
            assign56360_loop_guard += 1;
            assert!(assign56360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
                s.store_mul_neg_lhs(335, 154, 2156);
                s.store_exp(336, 335);
                s.store_sqrt_ad(338, A::div_scaled_inputs(s.ad_value(2114), 2.0, s.ad_value(154), 1.0));
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2157, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2408] = (s.v[335] > 0.0);
            s.v[2408] = if s.b[2408] { 1.0 } else { 0.0 };
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && s.b[2408]) {
                s.store_neg(2157, 2157);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2157, 1.0);
                s.store_mul_sub_from_scalar_rhs(2158, 345, 1.0, 336);
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2100, 2157, 1.0, 185, A::sub(s.ad_value(2159), s.ad_value(2156)), -1.0);
                s.store_add(2101, 185, 2158);
                s.store_div_scaled_inputs(2112, s.ad_value(2100), -1.0, s.ad_value(2101), 1.0);
            }
            s.b[2409] = (((s.v[2112]) as f64).abs() < 1e-10);
            s.v[2409] = if s.b[2409] { 1.0 } else { 0.0 };
            if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && s.b[2409]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2410] = (s.v[2112] > 0.1);
            s.v[2410] = if s.b[2410] { 1.0 } else { 0.0 };
            if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && (!s.b[2409])) && s.b[2410]) {
                s.store_scalar(2112, 0.1);
            }
            s.b[2411] = (s.v[2112] < (-0.1));
            s.v[2411] = if s.b[2411] { 1.0 } else { 0.0 };
            if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) && (!s.b[2409])) && (!s.b[2410])) && s.b[2411]) {
                s.store_scalar(2112, (-0.1));
            }
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && (s.v[79] == 0.0)) {
                s.store_add(2156, 2156, 2112);
            }
            if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) {
            s.copy_ad(2153, 2156);
            s.copy_ad(989, 349);
            s.store_sqrt_square_offset(782, 2153, ((4.0 * p.p405) * p.p405));
            s.store_offset_scaled_div(334, 2153, 782, 0.5, 0.5);
            s.store_scaled_add(992, 2153, 782, 0.5);
        }

        s.b[2412] = (s.v[992] < 0.0);
        s.v[2412] = if s.b[2412] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2395])) && s.b[2412]) {
            s.store_scalar(992, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_div(335, 989, 992);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_mul(340, 338, 337);
        }

        s.b[2413] = ((s.v[349] > (s.v[972] - (s.v[972] * 0.5))) && ((s.v[972] * 0.5) >= 0.0));
        s.v[2413] = if s.b[2413] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {
            s.store_add_scaled_inputs3(781, s.ad_value(349), 1.0, s.ad_value(972), (-1.0), s.ad_value(972), 0.5);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 972, 972, (0.5 * 0.5));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2414] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2414] = if s.b[2414] { 1.0 } else { 0.0 };

        s.b[2415] = (2.0 == 1.0);
        s.v[2415] = if s.b[2415] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && s.b[2415]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2416] = (2.0 == 2.0);
        s.v[2416] = if s.b[2416] { 1.0 } else { 0.0 };

        if (((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && s.b[2416]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2417] = (2.0 == 4.0);
        s.v[2417] = if s.b[2417] { 1.0 } else { 0.0 };

        if ((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && (!s.b[2416])) && s.b[2417]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2418] = (2.0 == 8.0);
        s.v[2418] = if s.b[2418] { 1.0 } else { 0.0 };

        if (((((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (!s.b[2415])) && (!s.b[2416])) && (!s.b[2417])) && s.b[2418]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) {
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let mut assign56760_loop_guard: usize = 0;
        while {
            let assign56760_cond_e88294: f64 = if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56760_cond_e88294 != 0.0
        } {
            assign56760_loop_guard += 1;
            assert!(assign56760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && s.b[2414]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) && (!s.b[2414])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);
            s.store_add_scaled_inputs3(2166, s.ad_value(972), 1.0, s.ad_value(972), (-0.5), s.ad_value(780), 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2413]) {
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2413])) {
            s.copy_ad(2166, 349);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_add_ad_lhs(989, A::div(s.ad_value(989), s.ad_value(340)), 2166);
            s.store_mul_square_lhs(338, 2166, 2166);
            s.store_offset(334, 338, 0.0001);
            s.store_div(2167, 338, 334);
        }

        s.b[2419] = (p.p43 == (-1.0));
        s.v[2419] = if s.b[2419] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2419]) {
            s.store_scalar(2167, 0.0);
            s.copy_ad(989, 349);
        }

        s.b[2420] = (p.p43 == 2.0);
        s.v[2420] = if s.b[2420] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) && s.b[2420]) {
            s.copy_ad(989, 349);
            s.store_scalar(2166, 0.0);
            s.store_scalar(2167, 0.0);
            s.store_sub(335, 2146, 972);
            s.store_add_scaled_inputs3_offset(992, s.ad_value(335), 0.5, A::ln(A::cosh(s.ad_value(335))), 0.5, s.ad_value(972), 1.0, (((2.0) as f64).ln() * 0.5));
        }

        s.b[2421] = (p.p43 == 3.0);
        s.v[2421] = if s.b[2421] { 1.0 } else { 0.0 };

        if ((((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) && (!s.b[2420])) && s.b[2421]) {
            s.store_add_ad_lhs(992, A::ln_one_plus_exp(A::sub(s.ad_value(2146), s.ad_value(972))), 972);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {
            s.store_div(335, 989, 992);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2419])) {
            s.store_mul(340, 338, 337);
            s.store_add_ad_lhs(989, A::div(s.ad_value(989), s.ad_value(340)), 2166);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_mul(2124, 990, 2133);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 2124, 343);
            s.store_offset_sqrt_ad(2168, A::offset(A::square(s.ad_value(989)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 2168, 688, 1.0);
            s.store_offset_mul(339, 2168, 689, 1.0);
        }

        s.b[2422] = param_given[408];
        s.v[2422] = if s.b[2422] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2422]) {
            s.store_div_scaled_value_by_product(2154, A::sub_from_scalar(p.p408, s.ad_value(2092)), 1.0, s.ad_value(965), s.ad_value(339), 100.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2422])) {
            s.store_div_scaled_inputs(2154, s.ad_value(2124), 9662367879.197212, s.ad_value(339), 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[2154] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 2154, p.p376);
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add(s.ad_value(966), A::mul3_scaled_output(s.ad_value(968), s.ad_value(338), s.ad_value(252), 1e-10)), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div_scaled_value_offset_denominator(2115, s.ad_value(989), 1.0, s.ad_value(162), p.p401, 1.0);
            s.store_powf(781, 989, 2.0);
            s.store_scalar(782, ((0.01) as f64).powf(2.0));
            s.store_sub_ad(334, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div_scaled_value_offset_denominator(2169, s.ad_value(334), 1.0, s.ad_value(162), (-p.p402), 1.0);
            s.store_div_scaled_product_indices(335, 254, 2169, 1.0, 973, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.store_div(985, 254, 338);
            s.store_mul_offset_ad_rhs(2132, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2115), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2115), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p.p400), 1.0);
            s.store_scaled_mul(335, 990, 2132, 1.6021918e-19);
            s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p.p7);
            s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2115);
            s.store_mul3_affine_lhs(988, 336, 2155, p.p363, 0.0, 2167);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add_scaled_inputs3(135, s.ad_value(986), 1.0, s.ad_value(987), 1.0, s.ad_value(988), 1.0);
            s.copy_ad(790, 349);
        }

        s.b[2423] = (p.p283 != 0.0);
        s.v[2423] = if s.b[2423] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2423]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(2089), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2424] = (s.v[336] < 0.0);
        s.v[2424] = if s.b[2424] { 1.0 } else { 0.0 };

        if (((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2423]) && s.b[2424]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2423]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1439, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3(339, s.ad_value(2089), 1.0, s.ad_value(340), 1.0, s.ad_value(1438), -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1439), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2423])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2425] = (p.p287 != 0.0);
        s.v[2425] = if s.b[2425] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2425]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1439);
        }

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && (!s.b[2425])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2426] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2426] = if s.b[2426] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2426]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        if (s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) {
            s.copy_ad(134, 135);
            s.store_add_scaled_inputs4(131, s.ad_value(2098), (-0.5), s.ad_value(2122), ((-1.0) * (-0.5)), s.ad_value(2099), (-0.5), s.ad_value(2123), (-(-0.5)));
            s.store_scaled_add(133, 2122, 2123, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 2122, 2123, (-0.5));
            s.store_neg(238, 2122);
            s.copy_ad(255, 2116);
        }

        s.b[2427] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2427] = if s.b[2427] { 1.0 } else { 0.0 };

        if ((s.b[1443] && (s.b[1446] && (!(s.b[1444] || s.b[1445])))) && s.b[2427]) {
            s.store_scalar(78, 1.0);
        }

        s.b[2428] = (s.v[791] < s.v[86]);
        s.v[2428] = if s.b[2428] { 1.0 } else { 0.0 };

        if ((!s.b[1443]) && s.b[2428]) {
            s.store_scalar(347, (-1.0));
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_sub_rhs(332, 154, 85, 1435);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2429] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2429] = if s.b[2429] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2428]) && s.b[2429]) {
            s.store_div_scaled_inputs(274, s.ad_value(277), 0.5, s.ad_value(278), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1443]) && s.b[2428]) && (!s.b[2429])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((!s.b[1443]) && s.b[2428]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if ((!s.b[1443]) && s.b[2428]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 1435, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 1435);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_ad_lhs(87, A::div(s.ad_value(335), s.ad_value(337)), 1435);
            s.copy_ad(91, 87);
            s.store_scalar(94, 0.0);
            s.store_sub(336, 85, 87);
            s.store_mul(131, 185, 336);
            s.store_scalar(133, 0.0);
            s.store_scalar(247, 0.0);
            s.store_scalar(169, 0.0);
            s.store_scalar(134, 0.0);
            s.store_scalar(127, 0.0);
            s.store_scalar(78, 1.0);
            s.store_scalar(946, 1.0);
        }

        s.b[2430] = (s.v[946] == 0.0);
        s.v[2430] = if s.b[2430] { 1.0 } else { 0.0 };

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_offset_ad(332, A::div_scaled_offset_numerator(A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1435))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_add_ad_rhs(89, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
        }

        s.b[2431] = (s.v[77] == 0.0);
        s.v[2431] = if s.b[2431] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2431]) {
            s.store_mul_sub_rhs(116, 154, 89, 1435);
        }

        s.b[2432] = (s.v[116] < 3.0);
        s.v[2432] = if s.b[2432] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {
            s.store_mul_sub_rhs(333, 154, 85, 1435);
            s.store_div_from_scalar_ad(335, 1.0, A::mul_scaled_lhs(s.ad_value(154), (1.414213562373095 / 108.0), s.ad_value(212)));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_ad_value(337, A::add_scaled_sub_value_product((-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, s.ad_value(335), s.ad_value(333), 27.0));
            s.store_ad_value(338, A::add_scaled_sub_value_product(1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, s.ad_value(335), s.ad_value(333), 27.0));
            s.store_square(338, 338);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 1435, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2433] = (s.v[791] <= s.v[118]);
        s.v[2433] = if s.b[2433] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && s.b[2433]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 85, 85);
            s.store_add_ad_rhs(337, 154, A::div_from_scalar(2.0, s.ad_value(85)));
            s.store_div_ad_lhs(90, A::ln(s.ad_value(336)), 337);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(88, s.ad_value(90), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_offset(332, 1435, (1e-12 / 2.0));
        }

        s.b[2434] = (s.v[88] < s.v[332]);
        s.v[2434] = if s.b[2434] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2434]) {
            s.copy_ad(88, 332);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.copy_ad(87, 88);
            s.copy_ad(92, 89);
            s.store_exp_mul(229, 154, 1435);
            s.store_mul(222, 210, 229);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign58550_loop_guard: usize = 0;
        while {
            let assign58550_cond_e90917: f64 = (s.v[421] + 1.0);
            let assign58550_cond_e90919: f64 = if (((!s.b[1443]) && s.b[2430]) && (s.v[97] <= assign58550_cond_e90917)) { 1.0 } else { 0.0 };
            assign58550_cond_e90919 != 0.0
        } {
            assign58550_loop_guard += 1;
            assert!(assign58550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1443]) && s.b[2430]) {
                s.store_mul_sub_rhs(116, 154, 87, 1435);
            }
            s.b[2435] = (s.v[116] < 5.0);
            s.v[2435] = if s.b[2435] { 1.0 } else { 0.0 };
            if (((!s.b[1443]) && s.b[2430]) && s.b[2435]) {
                s.store_ad_value(225, A::mul3(A::square(s.ad_value(116)), s.ad_value(116), A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(226, A::mul_offset_rhs(A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(214, 222, 225, 225);
                s.store_mul_ad_lhs(215, A::mul3_scaled_output(s.ad_value(222), s.ad_value(154), s.ad_value(225), 2.0), 226);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(224, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_inputs2(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, s.ad_value(215), 1.0, s.ad_value(216), 2.0);
            }
            s.b[2436] = (s.v[116] < 60.0);
            s.v[2436] = if s.b[2436] { 1.0 } else { 0.0 };
            if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) && s.b[2436]) {
                s.store_exp(227, 116);
                s.store_mul_offset_rhs(214, 222, 227, (-1.0));
                s.store_mul3_lhs(215, 222, 154, 227);
            }
            if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) && (!s.b[2436])) {
                s.store_exp_mul(231, 154, 87);
                s.store_mul_sub_rhs(214, 210, 231, 229);
                s.store_mul3_lhs(215, 210, 154, 231);
            }
            if (((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) {
                s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));
                s.store_div_scaled_inputs2(217, s.ad_value(154), 1.0, s.ad_value(215), 1.0, s.ad_value(216), 2.0);
            }
            if ((!s.b[1443]) && s.b[2430]) {
                s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));
                s.store_sub_from_scalar_ad(233, (-1.0), A::mul(s.ad_value(212), s.ad_value(217)));
            }
            s.b[2437] = (s.v[79] == 1.0);
            s.v[2437] = if s.b[2437] { 1.0 } else { 0.0 };
            if (((!s.b[1443]) && s.b[2430]) && s.b[2437]) {
                s.store_scalar(944, 1.0);
            }
            s.b[2438] = (s.v[944] == 0.0);
            s.v[2438] = if s.b[2438] { 1.0 } else { 0.0 };
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2439] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2439] = if s.b[2439] { 1.0 } else { 0.0 };
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2438]) && s.b[2439]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {
                s.store_add(87, 87, 236);
            }
            s.b[2440] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2440] = if s.b[2440] { 1.0 } else { 0.0 };
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2438]) && s.b[2440]) {
                s.store_scalar(79, 1.0);
            }
            if (((!s.b[1443]) && s.b[2430]) && (s.v[944] != 0.0)) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((!s.b[1443]) && s.b[2430]) {
                s.store_scalar(944, 0.0);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_offset(97, 97, (-1.0));
        }

        s.b[2442] = (s.v[116] < 5.0);
        s.v[2442] = if s.b[2442] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2442]) {
            s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2442])) {
            s.store_scalar(347, 3.0);
            s.store_scalar(78, 0.0);
            s.store_offset(99, 116, (-1.0));
            s.store_sqrt(100, 99);
            s.store_mul(101, 99, 100);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_mul(239, 209, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_offset_ad(238, A::mul3(s.ad_value(209), s.ad_value(214), s.ad_value(335)), 1e-25);
        }

        s.b[2443] = (s.v[116] < 5.0);
        s.v[2443] = if s.b[2443] { 1.0 } else { 0.0 };

        s.b[2444] = (s.v[116] < 3.0);
        s.v[2444] = if s.b[2444] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && s.b[2444]) {
            s.store_scalar(347, 1.0);
            s.store_scalar(78, 1.0);
            s.copy_ad(133, 238);
            s.copy_ad(131, 239);
            s.store_scalar(247, 0.5);
            s.store_scalar(169, 0.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && (!s.b[2444])) {
            s.store_scalar(347, 2.0);
            s.store_scalar(78, 0.0);
            s.store_scalar(335, (1.0 / (5.0 - 3.0)));
            s.store_mul_offset_rhs(332, 335, 116, (-3.0));
            s.store_ad_value(207, A::mul3(A::square(s.ad_value(332)), s.ad_value(332), A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0)));
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_mul(127, 238, 186);
            s.copy_ad(349, 790);
            s.store_div_ad_rhs(336, 636, A::square(s.ad_value(185)));
            s.store_add_scaled_inputs3(334, s.ad_value(85), 1.0, s.ad_value(155), (-1.0), s.ad_value(1438), -1.0);
            s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);
            s.store_scaled_add(343, 335, 782, 0.5);
        }

        s.b[2445] = (s.v[343] < 0.0);
        s.v[2445] = if s.b[2445] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2445]) {
            s.store_scalar(343, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2445]) {
            s.store_scalar(332, 0.0);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(336), 1.0, s.ad_value(337)));
            s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);
            s.store_scaled_add(344, 344, 782, 0.5);
        }

        s.b[2446] = (s.v[344] < 0.0);
        s.v[2446] = if s.b[2446] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2446]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), A::offset(s.ad_value(658), (-1.0)));
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
            s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1435), s.ad_value(790))));
        }

        s.b[2447] = (s.v[790] < 0.0);
        s.v[2447] = if s.b[2447] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2447]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(91, 87);
            s.store_scalar(947, 1.0);
        }

        s.b[2448] = (s.v[947] == 0.0);
        s.v[2448] = if s.b[2448] { 1.0 } else { 0.0 };

        s.b[2449] = (s.v[77] == 0.0);
        s.v[2449] = if s.b[2449] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }

        s.b[2450] = (((1.0 + 0.3) * s.v[96]) > 0.03);
        s.v[2450] = if s.b[2450] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {
            s.store_offset_ad(781, A::sub_scaled_inputs(s.ad_value(96), (1.0 + 0.3), s.ad_value(790), 1.0), (-0.03));
            s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(95, s.ad_value(96), (1.0 + 0.3), s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && (!s.b[2450])) {
            s.store_scale(95, 96, (1.0 + 0.3));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }

        s.b[2451] = (s.v[95] < 0.0);
        s.v[2451] = if s.b[2451] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2451]) {
            s.store_scalar(95, 0.0);
        }

        s.b[2452] = (s.v[95] > s.v[790]);
        s.v[2452] = if s.b[2452] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && (!s.b[2451])) && s.b[2452]) {
            s.copy_ad(95, 790);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2448]) {
            s.copy_ad(94, 95);
            s.store_add(91, 87, 94);
            s.store_scalar(79, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && (s.v[947] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_scalar(98, 1.0);
        }

        let mut assign59390_loop_guard: usize = 0;
        while {
            let assign59390_cond_e92333: f64 = (40.0 + 1.0);
            let assign59390_cond_e92335: f64 = if (((!s.b[1443]) && s.b[2430]) && (s.v[98] <= assign59390_cond_e92333)) { 1.0 } else { 0.0 };
            assign59390_cond_e92335 != 0.0
        } {
            assign59390_loop_guard += 1;
            assert!(assign59390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1443]) && s.b[2430]) {
                s.store_mul_sub_rhs(116, 154, 91, 1435);
            }
            s.b[2453] = (s.v[116] < 5.0);
            s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };
            if (((!s.b[1443]) && s.b[2430]) && s.b[2453]) {
                s.store_ad_value(225, A::mul3(A::square(s.ad_value(116)), s.ad_value(116), A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771)));
                s.store_ad_value(226, A::mul_offset_rhs(A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul(222, 210, 230);
                s.store_mul3_lhs(218, 222, 225, 225);
                s.store_mul_ad_lhs(219, A::mul3_scaled_output(s.ad_value(222), s.ad_value(154), s.ad_value(225), 2.0), 226);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_ad(224, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_square_add(220, 223, 218);
                s.store_div_scaled_inputs2(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, s.ad_value(219), 1.0, s.ad_value(220), 2.0);
            }
            if (((!s.b[1443]) && s.b[2430]) && (!s.b[2453])) {
                s.store_mul_sub_rhs(117, 154, 91, 790);
                s.store_exp(228, 117);
                s.store_mul_sub_rhs(218, 210, 228, 230);
                s.store_mul3_lhs(219, 210, 154, 228);
                s.store_offset(102, 116, (-1.0));
                s.store_sqrt_add(220, 102, 218);
                s.store_div_scaled_inputs2(221, s.ad_value(154), 1.0, s.ad_value(219), 1.0, s.ad_value(220), 2.0);
            }
            if ((!s.b[1443]) && s.b[2430]) {
                s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));
                s.store_sub_from_scalar_ad(235, (-1.0), A::mul(s.ad_value(212), s.ad_value(221)));
            }
            s.b[2454] = (s.v[79] == 1.0);
            s.v[2454] = if s.b[2454] { 1.0 } else { 0.0 };
            if (((!s.b[1443]) && s.b[2430]) && s.b[2454]) {
                s.store_scalar(945, 1.0);
            }
            s.b[2455] = (s.v[945] == 0.0);
            s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {
                s.store_div_scaled_inputs(237, s.ad_value(234), -1.0, s.ad_value(235), 1.0);
            }
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2456] = (((s.v[237]) as f64).abs() > s.v[93]);
            s.v[2456] = if s.b[2456] { 1.0 } else { 0.0 };
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2455]) && s.b[2456]) {
                s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {
                s.store_add(91, 91, 237);
            }
            s.b[2457] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));
            s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2455]) && s.b[2457]) {
                s.store_scalar(79, 1.0);
            }
            if (((!s.b[1443]) && s.b[2430]) && (s.v[945] != 0.0)) {
                s.store_scalar(98, (40.0 + 1.0));
            }
            if ((!s.b[1443]) && s.b[2430]) {
                s.store_scalar(945, 0.0);
                s.store_offset(98, 98, 1.0);
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_offset(98, 98, (-1.0));
        }

        s.b[2459] = (s.v[116] < 5.0);
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2459]) {
            s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2459])) {
            s.store_offset(102, 116, (-1.0));
            s.store_sqrt(103, 102);
            s.store_mul(104, 102, 103);
        }

        if ((!s.b[1443]) && s.b[2430]) {
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
            s.store_offset_ad(335, A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 3.0), 3.0);
            s.store_mul3_affine_lhs(267, 100, 264, 0.6666666666666667, 0.0, 335);
            s.store_offset_ad(335, A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 5.0), 10.0), 10.0), 5.0);
            s.store_mul_ad_lhs(268, A::mul3(A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), s.ad_value(101), s.ad_value(265)), 335);
            s.store_sub_ad_lhs(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);
            s.store_add_scaled_inputs4(335, s.ad_value(85), 1.0, s.ad_value(155), 1.0, s.ad_value(87), (-(2.0 * 0.5)), s.ad_value(94), (-0.5));
            s.store_sub(336, 266, 267);
            s.store_mul(337, 154, 185);
            s.store_mul(338, 154, 209);
            s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);
            s.store_mul(248, 94, 250);
        }

        s.b[2460] = (s.v[347] == 1.0);
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2460]) {
            s.store_scalar(948, 1.0);
        }

        s.b[2461] = (s.v[948] == 0.0);
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        s.b[2462] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2463] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
            s.store_offset_ad(781, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(87), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16)))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
        }

    }

    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
    ) {
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
            s.copy_ad(726, 770);
        }

        s.b[2464] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        s.b[2465] = (2.0 == 1.0);
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && s.b[2465]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2466] = (2.0 == 2.0);
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && s.b[2466]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2467] = (2.0 == 4.0);
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2467]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2468] = (2.0 == 8.0);
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        if ((((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2467])) && s.b[2468]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign60040_loop_guard: usize = 0;
        while {
            let assign60040_cond_e93589: f64 = if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60040_cond_e93589 != 0.0
        } {
            assign60040_loop_guard += 1;
            assert!(assign60040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && (!s.b[2464])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset(168, s.ad_value(87), 1.0, s.ad_value(790), 1.0, s.ad_value(780), 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) {
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) {
            s.store_scalar(334, 1.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {
            s.copy_ad(335, 684);
            s.store_sqrt_sub(342, 91, 1435);
            s.store_mul(171, 335, 342);
            s.store_div_scaled_inputs(343, s.ad_value(335), 0.5, s.ad_value(342), 1.0);
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

        s.b[2469] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {
            s.store_offset_ad(781, A::add_scaled_inputs3_offset(s.ad_value(168), 1.0, s.ad_value(87), -1.0, s.ad_value(790), -1.0, (-(-(10.0 * 2.220446049250313e-16)))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2470] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        s.b[2471] = (2.0 == 1.0);
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && s.b[2471]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2472] = (2.0 == 2.0);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && s.b[2472]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2473] = (2.0 == 4.0);
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && (!s.b[2472])) && s.b[2473]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2474] = (2.0 == 8.0);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

        if ((((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && (!s.b[2472])) && (!s.b[2473])) && s.b[2474]) {
            s.store_scalar(720, 4.0);
        }

        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign60520_loop_guard: usize = 0;
        while {
            let assign60520_cond_e94370: f64 = if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign60520_cond_e94370 != 0.0
        } {
            assign60520_loop_guard += 1;
            assert!(assign60520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && (!s.b[2470])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset(168, s.ad_value(87), 1.0, s.ad_value(790), 1.0, s.ad_value(780), 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && (!s.b[2469])) {
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && (!s.b[2469])) {
            s.store_scalar(334, 1.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {
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

        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {
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
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
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

        s.b[2475] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        s.b[2476] = (4.0 == 1.0);
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && s.b[2476]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2477] = (4.0 == 2.0);
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && s.b[2477]) {
            s.store_scalar(720, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2478] = (4.0 == 4.0);
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && (!s.b[2477])) && s.b[2478]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2479] = (4.0 == 8.0);
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && (!s.b[2477])) && (!s.b[2478])) && s.b[2479]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign61220_loop_guard: usize = 0;
        while {
            let assign61220_cond_e95343: f64 = if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61220_cond_e95343 != 0.0
        } {
            assign61220_loop_guard += 1;
            assert!(assign61220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2475])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(338, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(125, 1.0, 333);
            s.store_offset_ad(242, A::mul_offset_rhs(s.ad_value(125), s.ad_value(125), 1.0), 1.0);
        }

        s.b[2480] = (((1.0 + s.v[125]) < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {
            s.store_sub_from_scalar_ad(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), A::offset(s.ad_value(125), 1.0));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2481] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        s.b[2482] = (2.0 == 1.0);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && s.b[2482]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2483] = (2.0 == 2.0);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && s.b[2483]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2484] = (2.0 == 4.0);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && (!s.b[2483])) && s.b[2484]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2485] = (2.0 == 8.0);
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

        if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && (!s.b[2483])) && (!s.b[2484])) && s.b[2485]) {
            s.store_scalar(720, 4.0);
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign61550_loop_guard: usize = 0;
        while {
            let assign61550_cond_e95798: f64 = if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign61550_cond_e95798 != 0.0
        } {
            assign61550_loop_guard += 1;
            assert!(assign61550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && (!s.b[2481])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(243, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2480])) {
            s.store_offset(243, 125, 1.0);
            s.store_scalar(334, 1.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {
            s.store_div_scaled_product_indices(335, 127, 242, 0.6666666666666667, 243, 1.0);
            s.store_mul(133, 335, 185);
            s.store_offset(244, 125, 0.5);
            s.store_mul(245, 243, 242);
            s.store_div_scaled_inputs(246, s.ad_value(244), 0.4, s.ad_value(245), 1.0);
            s.store_sub_from_scalar(247, 0.6, 246);
        }

        s.b[2486] = (s.v[247] > 0.5);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2486]) {
            s.store_scalar(247, 0.5);
        }

        s.b[2487] = (s.v[347] == 2.0);
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {
            s.copy_ad(335, 131);
            s.store_add_scaled_product_value_ad(131, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(239)), 1.0, 207, 131, 1.0);
        }

        s.b[2488] = (s.v[131] < 0.0);
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) && s.b[2488]) {
            s.store_scalar(131, 0.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {
            s.copy_ad(335, 133);
            s.store_add_scaled_product_value_ad(133, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(238)), 1.0, 207, 133, 1.0);
        }

        s.b[2489] = (s.v[133] < 0.0);
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) && s.b[2489]) {
            s.store_scalar(133, 0.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {
            s.copy_ad(335, 247);
            s.store_add_scaled_product_value_ad(247, A::scale_offset(s.ad_value(207), (-0.5), 0.5), 1.0, 207, 247, 1.0);
            s.copy_ad(335, 169);
            s.store_mul(169, 207, 169);
        }

        if (((!s.b[1443]) && s.b[2430]) && (s.v[948] != 0.0)) {
            s.store_scalar(948, 0.0);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_sub(170, 162, 169);
        }

        s.b[2490] = (s.v[170] < 1e-9);
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2490]) {
            s.store_scalar(170, 1e-9);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_scalar(335, (s.v[625] / 100.0));
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_mul_ad(338, A::sub(s.ad_value(91), s.ad_value(87)), s.ad_value(334), 1.0);
            s.store_add_scaled_products_indices(339, 335, 131, 1.0, 336, 133, 1.0);
            s.store_div(337, 339, 338);
            s.store_mul_ad_rhs(251, 337, A::scale_offset(s.ad_value(1438), p.p166, 1.0));
        }

        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_mul(342, 339, 251);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_ad(341, s.ad_value(251), A::offset(s.ad_value(624), (-1.0)));
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
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
            s.store_div_scaled_inputs(336, s.ad_value(257), 0.2, s.ad_value(254), 1.0);
            s.store_div_scaled_inputs(337, s.ad_value(336), -1.0, s.ad_value(254), 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2491] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2491]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2492] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2491])) && s.b[2492]) {
            s.copy_ad(337, 335);
        }

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2491])) && (!s.b[2492])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[2493] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2493]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2494] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && s.b[2494]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {
            s.store_mul(339, 338, 340);
        }

        if ((!s.b[1443]) && s.b[2430]) {
            s.store_mul(253, 254, 339);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_div_scaled_inputs(335, s.ad_value(115), -1.0, s.ad_value(170), 1.0);
            s.store_mul3_lhs(135, 115, 248, 253);
        }

    }

    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2495] = (p.p283 != 0.0);
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_scale(336, 336, 0.5);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(87), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2496] = (s.v[336] < 0.0);
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2495]) && s.b[2496]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1439, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3(339, s.ad_value(87), 1.0, s.ad_value(340), 1.0, s.ad_value(1438), -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1439), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2495])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2497] = (p.p287 != 0.0);
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2497]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1439);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2497])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2498] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2498]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_mul3_lhs(45, 115, 249, 253);
            s.store_add(135, 135, 45);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2498])) {
            s.store_scalar(45, 0.0);
        }

        s.b[2499] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        s.b[2500] = (p.p296 > 0.0);
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(336, s.ad_value(338), 1.0, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(341, s.ad_value(337), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2500])) {
            s.copy_ad(341, 647);
        }

        s.b[2501] = (s.v[793] >= 0.0);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2501]) {
            s.copy_ad(369, 793);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2501])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2502] = (s.v[369] < (20.0 * 1e-12));
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2502]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2502])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2499]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2499])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        s.b[2503] = (p.p27 != 0.0);
        s.v[2503] = if s.b[2503] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_scale(335, 186, 1.034943e-10);
            s.copy_ad(336, 684);
            s.store_scalar(337, (s.v[628] - p.p139));
            s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));
            s.store_mul_ad_product_lhs(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), s.ad_value(336), 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs(340, s.ad_value(339), 0.5, s.ad_value(181), 1.0);
            s.store_mul_ad_product_lhs(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.ad_value(338), 181);
            s.store_mul_ad_lhs(342, A::mul3_scaled_output(s.ad_value(335), s.ad_value(336), s.ad_value(338), (-2.0)), 181);
            s.store_scalar(338, s.v[496]);
            s.store_scalar(340, s.v[497]);
            s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1439, 1.0);
            s.store_mul(137, 121, 335);
            s.store_sub_from_scalar_ad(335, s.v[498], A::scale(s.ad_value(790), p.p213));
            s.store_add_scaled_inputs3_offset(138, s.ad_value(1440), 1.0, s.ad_value(335), 1.0, s.ad_value(137), 1.0, (-s.v[160]));
            s.store_mul3_lhs(141, 694, 186, 186);
            s.store_scaled_mul(142, 141, 154, 0.5);
            s.store_scaled_mul(143, 142, 154, 2.0);
            s.store_scale(345, 154, 0.25);
            s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);
            s.store_offset_sub(335, 1440, 344, (-0.005));
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_sqrt_ad(336, A::add_scaled_square_product(s.ad_value(335), 1.0, s.ad_value(334), s.ad_value(344), (4.0 * 0.005)));
            s.store_sub_ad_lhs(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1438);
            s.store_offset_mul(338, 154, 337, (-1.0));
            s.store_div_from_scalar(339, 4.0, 143);
            s.store_offset_mul(335, 338, 339, 1.0);
            s.store_mul(340, 154, 339);
            s.store_mul(341, 338, 339);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2504] = (s.v[335] < 0.0);
        s.v[2504] = if s.b[2504] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2504]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(144, 335);
            s.store_mul_sub_from_scalar_rhs(334, 142, 1.0, 144);
            s.store_add(146, 138, 334);
            s.store_div_from_scalar_add_ad(334, 1.0, s.ad_value(154), A::div_scalar_offset_denominator(2.0, s.ad_value(138), 1e-25, 1.0));
            s.store_mul_ln_ad_lhs(147, A::mul(A::div_scalar_by_product(1.0, s.ad_value(140), s.ad_value(141), 1.0), A::square(s.ad_value(138))), 334);
            s.store_offset_sub(148, 147, 146, (-0.002));
            s.store_sqrt_ad(334, A::add_scaled_inputs(A::square(s.ad_value(148)), 1.0, s.ad_value(147), (4.0 * 0.002)));
            s.store_add_scaled_inputs3(149, s.ad_value(147), 1.0, s.ad_value(148), (-0.5), s.ad_value(334), (-0.5));
            s.store_mul_exp_ad_rhs(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));
            s.store_add_ad_lhs(335, A::offset(A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438))), (-1.0)), 334);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2505] = (s.v[335] < 0.0);
        s.v[2505] = if s.b[2505] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2505]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
        }

    }

    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_sqrt(150, 335);
            s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438)), (-1.0));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2506] = (s.v[335] < 0.0);
        s.v[2506] = if s.b[2506] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2506]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(151, 335);
            s.store_div_from_scalar(336, 0.5, 151);
            s.store_mul_sub_rhs(152, 139, 150, 151);
            s.store_sub(335, 146, 149);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2507] = (s.v[335] < 0.0);
        s.v[2507] = if s.b[2507] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2507]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_offset(335, 335, 1e-25);
            s.store_div(332, 790, 335);
            s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));
            s.store_square(722, 332);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
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

        s.b[2508] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2508] = if s.b[2508] { 1.0 } else { 0.0 };

        s.b[2509] = (4.0 == 1.0);
        s.v[2509] = if s.b[2509] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && s.b[2509]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2510] = (4.0 == 2.0);
        s.v[2510] = if s.b[2510] { 1.0 } else { 0.0 };

        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && s.b[2510]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2511] = (4.0 == 4.0);
        s.v[2511] = if s.b[2511] { 1.0 } else { 0.0 };

        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && s.b[2511]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2512] = (4.0 == 8.0);
        s.v[2512] = if s.b[2512] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && (!s.b[2511])) && s.b[2512]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign64120_loop_guard: usize = 0;
        while {
            let assign64120_cond_e99023: f64 = if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign64120_cond_e99023 != 0.0
        } {
            assign64120_loop_guard += 1;
            assert!(assign64120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && (!s.b[2508])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);
            s.store_scale(145, 155, ((2.0 * s.v[495]) * p.p7));
            s.copy_ad(335, 170);
            s.store_div_scaled_product_left_ad(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);
            s.store_add(134, 134, 153);
        }

        s.b[2513] = (((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[963] == 0.0));
        s.v[2513] = if s.b[2513] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {
            s.store_square(317, 127);
            s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);
            s.store_sub(319, 317, 318);
            s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);
            s.store_scaled_add(317, 317, 782, 0.5);
        }

        s.b[2514] = (s.v[317] < 0.0);
        s.v[2514] = if s.b[2514] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2514]) {
            s.store_scalar(317, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {
            s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);
            s.store_scaled_add(319, 319, 782, 0.5);
        }

        s.b[2515] = (s.v[319] < 0.0);
        s.v[2515] = if s.b[2515] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2515]) {
            s.store_scalar(319, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {
            s.store_sub(320, 317, 319);
        }

        s.b[2516] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));
        s.v[2516] = if s.b[2516] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2516]) {
            s.store_scalar(321, 0.0);
        }

        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && (!s.b[2516])) {
            s.store_scalar(321, 1.0);
        }

        if ((!s.b[1443]) && (s.v[946] != 0.0)) {
            s.store_scalar(946, 0.0);
        }

        s.b[2517] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));
        s.v[2517] = if s.b[2517] { 1.0 } else { 0.0 };

        if ((!s.b[1443]) && s.b[2517]) {
            s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);
            s.store_add_ad_lhs(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);
        }

        if ((!s.b[1443]) && (!s.b[2517])) {
            s.store_scalar(128, 0.0);
        }

        if (!s.b[1443]) {
            s.copy_ad(136, 134);
            s.store_scalar(46, 0.0);
        }

        s.b[2519] = ((p.p450 > 0.0) && (p.p454 > 0.0));
        s.v[2519] = if s.b[2519] { 1.0 } else { 0.0 };

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_scalar(2524, 1e-5);
            s.store_offset_ad(2525, A::add_scaled_inputs3_offset(s.ad_value(791), 1.0, s.ad_value(120), 1.0, s.ad_value(182), -1.0, (-s.v[160])), (-p.p455));
            s.store_offset(2526, 118, p.p455);
            s.store_sqrt_offset_ad(781, A::mul(A::sub(s.ad_value(960), s.ad_value(1435)), A::sub(s.ad_value(960), s.ad_value(1435))), ((4.0 * 0.01) * 0.01));
            s.store_add_scaled_inputs3(2536, s.ad_value(960), 0.5, s.ad_value(1435), ((-1.0) * 0.5), s.ad_value(781), 0.5);
            s.store_sqrt_ad(2520, A::div_scaled_product_offset_denominator(s.ad_value(2536), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));
            s.store_mul(2522, 2520, 162);
            s.store_div_scaled_product_denominator_ad(993, 2522, 2522, (-0.25), A::add(s.ad_value(790), s.ad_value(2522)), 1.0);
        }

        s.b[2538] = (p.p457 > 0.0);
        s.v[2538] = if s.b[2538] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2519]) && s.b[2538]) {
            s.store_scalar(2523, p.p457);
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.copy_ad(2539, 993);
            s.copy_ad(2540, 2526);
            s.store_offset_ad(332, A::div_scaled_offset_numerator(A::mul(s.ad_value(154), A::sub(s.ad_value(2525), s.ad_value(2539))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.store_add_ad_rhs(89, 2525, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
            s.store_mul_sub_rhs(116, 154, 89, 2539);
        }

        s.b[2541] = (s.v[116] < 3.0);
        s.v[2541] = if s.b[2541] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            s.store_mul_sub_rhs(333, 154, 2525, 2539);
            s.store_div_from_scalar_ad(335, 1.0, A::mul_scaled_lhs(s.ad_value(154), (1.414213562373095 / 108.0), s.ad_value(212)));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_ad_value(337, A::add_scaled_sub_value_product((-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, s.ad_value(335), s.ad_value(333), 27.0));
            s.store_ad_value(338, A::add_scaled_sub_value_product(1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, s.ad_value(335), s.ad_value(333), 27.0));
            s.store_square(338, 338);
        }

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 2539, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2542] = (s.v[791] <= s.v[2540]);
        s.v[2542] = if s.b[2542] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && s.b[2542]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 2525, 2525);
            s.store_add_ad_rhs(337, 154, A::div_from_scalar(2.0, s.ad_value(2525)));
            s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p.p456);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

    }

    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_add_scaled_inputs3(88, s.ad_value(90), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.store_offset(332, 2539, (1e-12 / 2.0));
        }

        s.b[2543] = (s.v[88] < s.v[332]);
        s.v[2543] = if s.b[2543] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2543]) {
            s.copy_ad(88, 332);
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            s.copy_ad(2523, 88);
        }

        s.b[2544] = (p.p451 == 1.0);
        s.v[2544] = if s.b[2544] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) {
            s.copy_ad(88, 2523);
            s.copy_ad(2545, 993);
            s.store_offset_ad(86, A::add_scaled_inputs3_offset(s.ad_value(120), (-1.0), s.ad_value(182), 1.0, s.ad_value(2545), 1.0, s.v[160]), p.p455);
        }

        s.b[2554] = (s.v[791] < s.v[86]);
        s.v[2554] = if s.b[2554] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            s.store_scalar(347, (-1.0));
            s.store_mul_scaled_ad_rhs(271, 155, 2.0, A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
            s.store_mul_sub_rhs(332, 154, 2525, 2545);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2555] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2555] = if s.b[2555] { 1.0 } else { 0.0 };

        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && s.b[2555]) {
            s.store_add_scaled_inputs3_offset(274, s.ad_value(278), 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, s.ad_value(339), 1.0, ((-7.0) * 1.414213562373095));
        }

        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && (!s.b[2555])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_add_ad_lhs(274, A::offset(s.ad_value(275), ((-7.0) * 1.414213562373095)), 339);
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 2545, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 2545);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_ad_lhs(2523, A::div(s.ad_value(335), s.ad_value(337)), 2545);
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
            s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2545), (-p.p456)));
            s.store_scalar(79, 0.0);
            s.copy_ad(2546, 88);
            s.store_mul3_affine_lhs(2547, 166, 2524, (0.5 * 9662367879.197212), 0.0, 2524);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2547)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(2548, A::ln(s.ad_value(335)), 2547);
            s.store_scalar(97, 1.0);
        }

        let mut assign65310_loop_guard: usize = 0;
        while {
            let assign65310_cond_e100816: f64 = (s.v[421] + 1.0);
            let assign65310_cond_e100818: f64 = if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (s.v[97] <= assign65310_cond_e100816)) { 1.0 } else { 0.0 };
            assign65310_cond_e100818 != 0.0
        } {
            assign65310_loop_guard += 1;
            assert!(assign65310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
                s.store_sub(2549, 2546, 2545);
                s.store_mul(116, 154, 2549);
                s.store_mul_sub_rhs(333, 2548, 2549, 2547);
            }
            s.b[2556] = (s.v[333] < 60.0);
            s.v[2556] = if s.b[2556] { 1.0 } else { 0.0 };
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2556]) {
                s.store_exp(335, 333);
                s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(2548), -1.0, s.ad_value(2547)));
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(2551, A::ln(A::offset(s.ad_value(336), 1.0)), 2548);
                s.store_div_scaled_value_offset_denominator(2552, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2556])) {
                s.store_sub(2551, 2549, 2547);
                s.store_scalar(2552, 1.0);
            }
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
                s.store_mul(2550, 154, 2551);
            }
            s.b[2557] = (((s.v[116]) as f64).abs() < 1e-16);
            s.v[2557] = if s.b[2557] { 1.0 } else { 0.0 };
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2557]) {
                s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2552))), 1.0 / (2.0));
                s.store_mul(223, 116, 334);
                s.store_mul(2553, 154, 334);
            }
            s.b[2558] = (s.v[116] < 0.0);
            s.v[2558] = if s.b[2558] { 1.0 } else { 0.0 };
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2557]) && s.b[2558]) {
                s.store_neg(223, 223);
                s.store_neg(2553, 2553);
            }
            s.b[2559] = (((s.v[116]) as f64).abs() < 0.005);
            s.v[2559] = if s.b[2559] { 1.0 } else { 0.0 };
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2557])) && s.b[2559]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2550, 1.0, 2550, 1.0, 2550, 1.0, 2550, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 2550, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2550), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2550), 1.0, A::scale(s.ad_value(2550), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(223, 334, 336);
                s.store_div_scaled_product_right_ad(2553, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2552), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2557])) && (!s.b[2559])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 2550);
                s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2550), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
                s.store_div_scaled_product_right_ad(2553, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2552), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            s.b[2560] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));
            s.v[2560] = if s.b[2560] { 1.0 } else { 0.0 };
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2560]) {
                s.store_scalar(347, (-1.0));
            }
            s.b[2561] = (s.v[116] < 0.0);
            s.v[2561] = if s.b[2561] { 1.0 } else { 0.0 };
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2561]) {
                s.store_neg(216, 223);
                s.store_neg(217, 2553);
            }
            s.b[2562] = (s.v[116] < 1e-7);
            s.v[2562] = if s.b[2562] { 1.0 } else { 0.0 };
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2561])) && s.b[2562]) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 2553);
            }
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2561])) && (!s.b[2562])) {
                s.store_mul_offset_rhs(117, 154, 2546, (-p.p456));
                s.store_exp(228, 117);
                s.store_mul_ad_rhs(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 210, s.ad_value(154), A::sub(s.ad_value(228), s.ad_value(230)));
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(2553), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
                s.store_add_scaled_inputs_product_indices(232, 2546, 1.0, 2525, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2563] = (s.v[79] == 1.0);
            s.v[2563] = if s.b[2563] { 1.0 } else { 0.0 };
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2563]) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2546]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2546))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2564] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2564] = if s.b[2564] { 1.0 } else { 0.0 };
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) && s.b[2564]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {
                s.store_add(2546, 2546, 236);
            }
            s.b[2565] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2565] = if s.b[2565] { 1.0 } else { 0.0 };
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) && s.b[2565]) {
                s.store_scalar(79, 1.0);
            }
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
            s.copy_ad(2523, 2546);
        }

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_mul_scaled_ad_rhs(339, 154, -1.0, A::sub(s.ad_value(2523), s.ad_value(993)));
            s.store_abs(2535, 339);
            s.store_exp(340, 339);
            s.store_sub_ad_lhs(341, A::offset(s.ad_value(340), (-1.0)), 339);
        }

        s.b[2566] = (s.v[339] > 1e-7);
        s.v[2566] = if s.b[2566] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2519]) && s.b[2566]) {
            s.store_mul_scaled_ad_rhs(2537, 209, -1.0, A::sqrt(s.ad_value(341)));
        }

        s.b[2567] = (s.v[2535] > 1e-7);
        s.v[2567] = if s.b[2567] { 1.0 } else { 0.0 };

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2566])) && s.b[2567]) {
            s.store_mul_sqrt_rhs(2537, 209, 341);
        }

        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2566])) && (!s.b[2567])) {
            s.store_mul_scaled_ad_rhs(2537, 339, (-0.7071067811865475), A::sqrt(A::offset(A::mul_scaled_lhs(s.ad_value(2535), 0.3333333333333333, A::scale_offset(s.ad_value(2535), 0.25, 1.0)), 1.0)));
        }

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_sqrt_square_offset(781, 2537, ((4.0 * 1e-6) * 1e-6));
            s.store_scaled_add(2532, 2537, 781, 0.5);
            s.store_div_scaled_inputs(2533, s.ad_value(2532), 1.0, s.ad_value(586), 1.6021918e-19);
            s.store_offset(335, 2533, (-p.p452));
            s.store_scale(2534, 2533, 0.01);
            s.store_sqrt_ad(781, A::add_scaled_square_product(s.ad_value(335), 1.0, s.ad_value(2534), s.ad_value(2534), 4.0));
            s.store_scaled_add(336, 335, 781, 0.5);
            s.store_div_scaled_product_by_product(2531, s.ad_value(336), s.ad_value(336), 1.0, s.ad_value(2533), s.ad_value(2533), 1.0);
            s.store_add_scaled_product_left_ad(994, 993, 1.0, A::sub(s.ad_value(2523), s.ad_value(993)), 2531, 1.0);
            s.store_ad_value(333, A::mul_sub_from_scalar_rhs(A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1435), 1.0))), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790)))));
            s.store_scalar(2527, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());
            s.store_mul_sqrt_rhs(2528, 2527, 155);
            s.store_mul_sub_rhs(2521, 154, 994, 993);
        }

        s.b[2568] = ((s.v[2521] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));
        s.v[2568] = if s.b[2568] { 1.0 } else { 0.0 };

        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
            s.store_sub_scaled_inputs(781, 154, 0.2, 2521, 1.0);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2569] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        s.b[2570] = (1.0 == 1.0);
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if (((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && s.b[2570]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2571] = (1.0 == 2.0);
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        if ((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && s.b[2571]) {
            s.store_scalar(720, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2572] = (1.0 == 4.0);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        if (((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && (!s.b[2571])) && s.b[2572]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2573] = (1.0 == 8.0);
        s.v[2573] = if s.b[2573] { 1.0 } else { 0.0 };

        if ((((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && (!s.b[2571])) && (!s.b[2572])) && s.b[2573]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign65790_loop_guard: usize = 0;
        while {
            let assign65790_cond_e102498: f64 = if (((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign65790_cond_e102498 != 0.0
        } {
            assign65790_loop_guard += 1;
            assert!(assign65790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && (!s.b[2569])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);
            s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);
        }

        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
        }

        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2568])) {
            s.copy_ad(335, 2521);
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[1443]) && s.b[2519]) {
            s.store_sqrt_offset_input(2529, 335, (10.0 * 2.220446049250313e-16));
            s.store_mul(2530, 2528, 2529);
            s.store_mul_scaled_ad_lhs(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2530, p.p454);
            s.store_scaled_mul(46, 995, 333, s.v[632]);
            s.store_add(134, 136, 46);
        }

        if (!s.b[1443]) {
            s.store_add(134, 136, 46);
            s.copy_ad(978, 133);
        }

        s.store_scale(335, 162, (-s.v[635]));

        s.store_mul(20, 335, 131);

        s.store_mul(132, 335, 133);

        s.store_mul(19, 132, 247);

        s.store_mul(979, 335, 978);

        s.store_scaled_sub(335, 790, 94, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p263)));

        s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);

        s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));

        s.store_div_from_scalar(110, p.p263, 782);

        s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);

        s.b[2574] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2574] = if s.b[2574] { 1.0 } else { 0.0 };

        if s.b[2574] {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2575] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        s.b[2576] = (2.0 == 1.0);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        if ((s.b[2574] && s.b[2575]) && s.b[2576]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2577] = (2.0 == 2.0);
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        if (((s.b[2574] && s.b[2575]) && (!s.b[2576])) && s.b[2577]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2578] = (2.0 == 4.0);
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if ((((s.b[2574] && s.b[2575]) && (!s.b[2576])) && (!s.b[2577])) && s.b[2578]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2579] = (2.0 == 8.0);
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        if (((((s.b[2574] && s.b[2575]) && (!s.b[2576])) && (!s.b[2577])) && (!s.b[2578])) && s.b[2579]) {
            s.store_scalar(720, 4.0);
        }

        if (s.b[2574] && s.b[2575]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign66330_loop_guard: usize = 0;
        while {
            let assign66330_cond_e103006: f64 = if ((s.b[2574] && s.b[2575]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66330_cond_e103006 != 0.0
        } {
            assign66330_loop_guard += 1;
            assert!(assign66330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[2574] && s.b[2575]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (s.b[2574] && (!s.b[2575])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if s.b[2574] {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if s.b[2574] {
        }

        if (!s.b[2574]) {
        }

        if (!s.b[2574]) {
            s.store_scalar(334, 1.0);
        }

        s.store_add(109, 87, 110);

        s.store_add_scaled_product_left_ad(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p.p435);

        s.b[2580] = (p.p23 == 0.0);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        if s.b[2580] {
            s.store_scalar(280, 0.0);
            s.store_scalar(288, 0.0);
        }

        s.b[2581] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        if ((!s.b[2580]) && s.b[2581]) {
            s.store_mul(335, 659, 85);
            s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));
            s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1438, (-1.0));
            s.store_offset_mul(340, 338, 339, 1.0);
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.b[2582] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
            s.store_sub_ad_lhs(781, A::offset(s.ad_value(341), 1e-6), 340);
            s.store_square(722, 781);
            s.store_square(723, 341);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
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

        s.b[2583] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        s.b[2584] = (4.0 == 1.0);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && s.b[2584]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2585] = (4.0 == 2.0);
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        if ((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && s.b[2585]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2586] = (4.0 == 4.0);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if (((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && (!s.b[2585])) && s.b[2586]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2587] = (4.0 == 8.0);
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if ((((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && (!s.b[2585])) && (!s.b[2586])) && s.b[2587]) {
            s.store_scalar(720, 4.0);
        }

        if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign66840_loop_guard: usize = 0;
        while {
            let assign66840_cond_e103517: f64 = if (((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66840_cond_e103517 != 0.0
        } {
            assign66840_loop_guard += 1;
            assert!(assign66840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && (!s.b[2583])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 341, 726);
            s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);
            s.store_sub_ad_lhs(340, A::offset(s.ad_value(341), 1e-6), 780);
        }

        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
        }

        if (((!s.b[2580]) && s.b[2581]) && (!s.b[2582])) {
        }

        if (((!s.b[2580]) && s.b[2581]) && (!s.b[2582])) {
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[2580]) && s.b[2581]) {
            s.store_sqrt(340, 340);
            s.store_add_ad_rhs(282, 335, A::mul_sub_from_scalar_rhs(s.ad_value(337), 1.0, s.ad_value(340)));
            s.store_div_from_scalar_offset_input(336, s.v[582], 661, s.v[582]);
            s.store_add_scaled_inputs_product_indices(283, 1439, s.v[483], 109, 1.0, 336, 282, (-1.0));
            s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.b[2588] = (s.v[283] < 0.0);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if (((!s.b[2580]) && s.b[2581]) && s.b[2588]) {
            s.store_scalar(283, 0.0);
        }

    }
}
