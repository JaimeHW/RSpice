#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_94(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3341] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3341] = if s.b[3341] { 1.0 } else { 0.0 };

        s.b[3342] = (2.0 == 1.0);
        s.v[3342] = if s.b[3342] { 1.0 } else { 0.0 };

        if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && s.b[3342]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3343] = (2.0 == 2.0);
        s.v[3343] = if s.b[3343] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && s.b[3343]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3344] = (2.0 == 4.0);
        s.v[3344] = if s.b[3344] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && s.b[3344]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3345] = (2.0 == 8.0);
        s.v[3345] = if s.b[3345] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && (!s.b[3344])) && s.b[3345]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign103230_loop_guard: usize = 0;
        while {
            let assign103230_cond_e155130: f64 = if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign103230_cond_e155130 != 0.0
        } {
            assign103230_loop_guard += 1;
            assert!(assign103230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && (!s.b[3341])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-25);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1e-25, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(613, 1e-25, 780);
        }

        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {
        }

        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3332] && (!s.b[3333])) {
            s.store_div_from_scalar(5, 1.0, 613);
            s.store_div(5, 5, 164);
            s.store_add(5, 5, 648);
        }

        s.b[3347] = (s.v[5] < p.p444);
        s.v[3347] = if s.b[3347] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3333])) && s.b[3347]) {
            s.store_scalar(5, p.p444);
        }

        if (s.b[3332] && (!s.b[3333])) {
            s.store_scale(716, 5, 1.0 / (s.v[365]));
        }

        s.b[3352] = (s.v[75] == 0.0);
        s.v[3352] = if s.b[3352] { 1.0 } else { 0.0 };

        if (s.b[3332] && (!s.b[3352])) {
            s.copy_ad(3348, 729);
            s.copy_ad(3349, 728);
        }

        s.b[3353] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.v[3353] = if s.b[3353] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {
            s.store_ad_value(335, {
                if (s.v[676] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(676), p.p415)
                }
            });
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {
            s.store_div_from_scalar(787, s.v[567], 335);
            s.store_ad_value(334, A::sub_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), 1.0, A::sub_from_scalar(1.0, s.ad_value(676)), p.p417));
            s.store_div_from_scalar(788, s.v[568], 334);
            s.store_add_ad_rhs(956, 956, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p438));
        }

        s.b[3355] = (s.v[956] < 0.1);
        s.v[3355] = if s.b[3355] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3353]) && s.b[3355]) {
            s.store_scalar(956, 0.1);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3353])) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_scalar(164, (s.v[630] * p.p7));
            s.store_scalar(785, (p.p67 + p.p68));
            s.store_offset(789, 451, 1e-12);
            s.store_scalar(408, s.v[459]);
            s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(3349), p.p410, A::scale(s.ad_value(3349), p.p411)), 1.0);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(654, 335, 782, 0.5);
        }

        s.b[3356] = (s.v[654] < 0.0);
        s.v[3356] = if s.b[3356] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3356]) {
            s.store_scalar(654, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_mul3_lhs(593, 787, 653, 654);
            s.store_offset_ad(3351, A::mul3(s.ad_value(788), s.ad_value(786), s.ad_value(652)), 1e-25);
            s.copy_ad(594, 453);
            s.store_scalar(595, p.p421);
            s.store_scale(335, 593, 10000.0);
            s.store_scale(336, 3351, 100.0);
        }

        s.b[3359] = (s.v[799] < 0.0);
        s.v[3359] = if s.b[3359] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {
            s.store_scale(781, 799, ((-0.5) * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_ad(782, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_offset_mul_ad(783, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_ad_value(336, A::div_scaled_inputs(s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0));
        }

        s.b[3360] = (s.v[108] < 1e-12);
        s.v[3360] = if s.b[3360] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3359]) && s.b[3360]) {
            s.store_scalar(108, 1e-12);
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {
            s.store_sub_scaled_inputs(598, 799, 1.0, 108, 2.0);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {
            s.store_scale(781, 799, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_ad(782, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_offset_mul_ad(783, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_ad_value(336, A::div_scaled_inputs(s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0));
        }

        s.b[3361] = (s.v[108] < 1e-12);
        s.v[3361] = if s.b[3361] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) && s.b[3361]) {
            s.store_scalar(108, 1e-12);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {
            s.store_add_scaled_inputs(598, 799, 1.0, 108, 2.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_div(591, 598, 785);
            s.store_mul(592, 593, 591);
        }

        s.b[3362] = (s.v[799] >= 0.0);
        s.v[3362] = if s.b[3362] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3362]) {
            s.store_div(335, 592, 3351);
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3362])) {
            s.store_scaled_div(335, 592, 3351, -1.0);
        }

        s.b[3363] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3363] = if s.b[3363] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3363]) {
            s.store_scalar(337, 1.0);
        }

        s.b[3364] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3364] = if s.b[3364] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && s.b[3364]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && (!s.b[3364])) {
            s.store_pow_ad(337, s.ad_value(335), A::offset(s.ad_value(956), (-1.0)));
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[3365] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3365] = if s.b[3365] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3365]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[3366] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[3366] = if s.b[3366] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && s.b[3366]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {
            s.store_ad_value(340, {
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)))
                }
            });
        }

        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_mul(3350, 593, 339);
            s.store_offset(338, 335, 1.0);
            s.store_div_from_scalar(339, 1.0, 338);
            s.store_offset_ad(338, A::div_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(595), 1.0, s.ad_value(339)), s.ad_value(598), 1.0, A::offset(s.ad_value(785), (-p.p423)), 1.0), 1.0);
            s.store_offset(781, 338, (-0.001));
            s.store_scalar(782, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(339, 781, 782, 0.5);
            s.store_mul(717, 408, 339);
            s.store_scale(718, 698, (6.241449993689894e18 * p.p430));
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(717), 1.0, s.ad_value(718), (-1.0), s.ad_value(717), (-0.001)));
            s.store_scaled_mul(782, 717, 717, (4.0 * 0.001));
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(718, A::add_scaled_inputs3(s.ad_value(717), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
            s.store_sub(597, 717, 718);
        }

        s.b[3367] = ((p.p441 > 0.0) && (p.p440 > 1.0));
        s.v[3367] = if s.b[3367] { 1.0 } else { 0.0 };

        s.b[3368] = ((s.v[597] > ((s.v[408] * p.p440) - (s.v[408] * p.p441))) && ((s.v[408] * p.p441) >= 0.0));
        s.v[3368] = if s.b[3368] { 1.0 } else { 0.0 };

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(597), 1.0, s.ad_value(408), (-p.p440), s.ad_value(408), p.p441));
            s.store_square(722, 781);
            s.store_scaled_mul(723, 408, 408, (p.p441 * p.p441));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_95(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign104390_loop_guard: usize = 0;
        while {
            let assign104390_cond_e156585: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (s.v[719] < p.p442)) { 1.0 } else { 0.0 };
            assign104390_cond_e156585 != 0.0
        } {
            assign104390_loop_guard += 1;
            assert!(assign104390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3369] = ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0));
        s.v[3369] = if s.b[3369] { 1.0 } else { 0.0 };

        s.b[3370] = (p.p442 == 1.0);
        s.v[3370] = if s.b[3370] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && s.b[3370]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3371] = (p.p442 == 2.0);
        s.v[3371] = if s.b[3371] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && s.b[3371]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3372] = (p.p442 == 4.0);
        s.v[3372] = if s.b[3372] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && s.b[3372]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3373] = (p.p442 == 8.0);
        s.v[3373] = if s.b[3373] { 1.0 } else { 0.0 };

        if ((((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && (!s.b[3372])) && s.b[3373]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign104520_loop_guard: usize = 0;
        while {
            let assign104520_cond_e156780: f64 = if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign104520_cond_e156780 != 0.0
        } {
            assign104520_loop_guard += 1;
            assert!(assign104520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (!s.b[3369])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * p.p442)))
                }
            });
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 408, p.p441, 0.0, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(408), s.ad_value(725), s.ad_value(726), p.p441, s.ad_value(770), 1.0));
            s.store_ad_value(336, A::add_scaled_inputs3(s.ad_value(408), p.p440, s.ad_value(408), (-p.p441), s.ad_value(780), 1.0));
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && (!s.b[3368])) {
            s.copy_ad(336, 597);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3367]) {
            s.copy_ad(597, 336);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_neg(334, 697);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[3374] = (s.v[334] < 0.0);
        s.v[3374] = if s.b[3374] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3374]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));
            s.store_sqrt_mul(599, 650, 334);
            s.store_offset_sub(336, 3348, 3349, p.p137);
            s.store_sqrt_square_offset(782, 336, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3375] = (s.v[336] < 0.0);
        s.v[3375] = if s.b[3375] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3375]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));
            s.store_sqrt_mul(600, 651, 336);
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(789), 1.0, s.ad_value(600), (-1.0), s.ad_value(789), (-0.01)));
            s.store_scaled_mul(782, 789, 789, (4.0 * 0.01));
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(602, A::add_scaled_inputs3(s.ad_value(789), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
            s.store_scalar(601, (p.p419 + 1e-25));
            s.store_mul_sub_from_scalar_ad_rhs(596, 649, 1.0, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789)))));
            s.store_sqrt_ad(782, A::add_scaled_square_product(s.ad_value(596), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), ((1.0 / (100.0) * 4.0) * 1.0 / (100.0))));
            s.store_offset_scaled_div(343, 596, 782, 0.5, 0.5);
            s.store_scaled_add(596, 596, 782, 0.5);
        }

        s.b[3376] = (s.v[596] < 0.0);
        s.v[3376] = if s.b[3376] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3376]) {
            s.store_scalar(596, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_div_from_scalar_offset_input(335, 1.6021918e-19, 785, p.p422);
            s.store_mul_ad_lhs(739, A::mul3(s.ad_value(335), s.ad_value(596), s.ad_value(3350)), 597);
        }

        s.b[3377] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));
        s.v[3377] = if s.b[3377] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
            s.store_sub_from_scalar(781, 1e-25, 739);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-25 * 1e-25));
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

        s.b[3378] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3378] = if s.b[3378] { 1.0 } else { 0.0 };

        s.b[3379] = (2.0 == 1.0);
        s.v[3379] = if s.b[3379] { 1.0 } else { 0.0 };

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && s.b[3379]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3380] = (2.0 == 2.0);
        s.v[3380] = if s.b[3380] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && s.b[3380]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3381] = (2.0 == 4.0);
        s.v[3381] = if s.b[3381] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && s.b[3381]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3382] = (2.0 == 8.0);
        s.v[3382] = if s.b[3382] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && (!s.b[3381])) && s.b[3382]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && (!s.b[3378])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-25);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1e-25, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(739, 1e-25, 780);
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3377])) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3377])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_div_from_scalar(4, 1.0, 739);
            s.store_div(4, 4, 164);
        }

        s.b[3383] = ((s.v[4] > (1000000.0 - 1000.0)) && (1000.0 >= 0.0));
        s.v[3383] = if s.b[3383] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
            s.store_offset(781, 4, (((-1000000.0)) + (1000.0)));
            s.store_square(722, 781);
            s.store_scalar(723, (1000.0 * 1000.0));
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

        s.b[3384] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[3384] = if s.b[3384] { 1.0 } else { 0.0 };

        s.b[3385] = (2.0 == 1.0);
        s.v[3385] = if s.b[3385] { 1.0 } else { 0.0 };

        if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && s.b[3385]) {
            s.store_scalar(720, 1.0);
        }

        s.b[3386] = (2.0 == 2.0);
        s.v[3386] = if s.b[3386] { 1.0 } else { 0.0 };

        if (((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && s.b[3386]) {
            s.store_scalar(720, 2.0);
        }

        s.b[3387] = (2.0 == 4.0);
        s.v[3387] = if s.b[3387] { 1.0 } else { 0.0 };

        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && s.b[3387]) {
            s.store_scalar(720, 3.0);
        }

        s.b[3388] = (2.0 == 8.0);
        s.v[3388] = if s.b[3388] { 1.0 } else { 0.0 };

        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && (!s.b[3387])) && s.b[3388]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_96(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && (!s.b[3384])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1000.0);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1000.0, s.ad_value(770), 1.0));
            s.store_offset(4, 780, (1000000.0 - 1000.0));
        }

        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3383])) {
        }

        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3383])) {
            s.store_scalar(334, 1.0);
        }

        s.b[3389] = ((p.p54 == 1.0) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.v[3389] = if s.b[3389] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3389]) {
            s.store_sub_from_scalar(385, p.p334, 384);
            s.store_scaled_div(4, 4, 385, s.v[165]);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_add(4, 4, 644);
        }

        s.b[3391] = (s.v[4] < p.p444);
        s.v[3391] = if s.b[3391] { 1.0 } else { 0.0 };

        if ((s.b[3332] && (!s.b[3352])) && s.b[3391]) {
            s.store_scalar(4, p.p444);
        }

        if (s.b[3332] && (!s.b[3352])) {
            s.store_scale(715, 4, 1.0 / (s.v[365]));
        }

        s.b[3392] = (s.v[4] < p.p444);
        s.v[3392] = if s.b[3392] { 1.0 } else { 0.0 };

        if ((!s.b[3332]) && s.b[3392]) {
            s.store_scalar(4, p.p444);
        }

        s.b[3393] = (s.v[5] < p.p444);
        s.v[3393] = if s.b[3393] { 1.0 } else { 0.0 };

        if ((!s.b[3332]) && s.b[3393]) {
            s.store_scalar(5, p.p444);
        }

        s.b[3394] = (s.v[370] > 0.0);
        s.v[3394] = if s.b[3394] { 1.0 } else { 0.0 };

        if ((!s.b[3332]) && s.b[3394]) {
            s.store_scale(715, 4, 1.0 / (s.v[365]));
            s.store_scale(716, 5, 1.0 / (s.v[365]));
        }

        if ((!s.b[3332]) && (!s.b[3394])) {
            s.store_scale(715, 5, 1.0 / (s.v[365]));
            s.store_scale(716, 4, 1.0 / (s.v[365]));
        }

        s.copy_ad(4, 715);

        s.copy_ad(5, 716);

        s.b[3395] = (s.v[949] > 0.0);
        s.v[3395] = if s.b[3395] { 1.0 } else { 0.0 };

        if s.b[3395] {
            s.copy_ad(134, 0);
            s.copy_ad(19, 701);
            s.copy_ad(18, 700);
            s.copy_ad(741, 702);
            s.store_neg_ad(20, A::add_scaled_inputs3(s.ad_value(700), 1.0, s.ad_value(701), 1.0, s.ad_value(702), 1.0));
            s.copy_ad(280, 709);
            s.copy_ad(281, 710);
            s.copy_ad(400, 699);
        }

        if (s.b[3395] && (s.v[81] != 0.0)) {
            s.copy_ad(247, 708);
        }

        if (!s.b[3395]) {
            s.store_neg(134, 0);
            s.copy_ad(19, 702);
            s.copy_ad(18, 700);
            s.copy_ad(741, 701);
            s.store_neg_ad(20, A::add_scaled_inputs3(s.ad_value(700), 1.0, s.ad_value(701), 1.0, s.ad_value(702), 1.0));
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
            s.store_scalar(400, 0.0);
        }

        if ((!s.b[3395]) && (s.v[81] != 0.0)) {
            s.store_sub_from_scalar(247, 1.0, 708);
        }

        s.store_add(18, 18, 811);

        s.store_add(19, 19, 810);

        s.store_add(741, 741, 812);

        s.store_neg_ad(20, A::add_scaled_inputs3(s.ad_value(18), 1.0, s.ad_value(19), 1.0, s.ad_value(741), 1.0));

        s.copy_ad(299, 703);

        s.copy_ad(301, 704);

        s.copy_ad(742, 706);

        s.copy_ad(743, 705);

        s.store_neg_ad(744, A::add_scaled_inputs3(s.ad_value(705), 1.0, s.ad_value(706), 1.0, s.ad_value(707), 1.0));

        s.b[3396] = (p.p53 > 0.0);
        s.v[3396] = if s.b[3396] { 1.0 } else { 0.0 };

        s.b[3397] = (s.v[766] > 0.0001);
        s.v[3397] = if s.b[3397] { 1.0 } else { 0.0 };

        if (s.b[3396] && s.b[3397]) {
            s.store_div_from_scalar(740, 1.0, 766);
        }

        if (s.b[3396] && (!s.b[3397])) {
            s.store_scalar(740, (1.0 / 0.0001));
        }

        s.b[3398] = ((s.v[729] * (s.v[733] - s.v[729])) >= 0.0);
        s.v[3398] = if s.b[3398] { 1.0 } else { 0.0 };

        s.b[3399] = (s.v[529] == 1.0);
        s.v[3399] = if s.b[3399] { 1.0 } else { 0.0 };

        if ((s.b[3396] && s.b[3398]) && s.b[3399]) {
            s.copy_ad(745, 733);
        }

        if ((s.b[3396] && s.b[3398]) && (!s.b[3399])) {
            s.store_ad_value(745, A::add_scaled_product(s.ad_value(729), 1.0, s.ad_value(683), A::sub(s.ad_value(733), s.ad_value(729)), 1.0));
        }

        if (s.b[3396] && (!s.b[3398])) {
            s.copy_ad(745, 729);
        }

        if s.b[3396] {
            s.store_mul(746, 134, 745);
        }

        s.b[3400] = (p.p53 == 1.0);
        s.v[3400] = if s.b[3400] { 1.0 } else { 0.0 };

        if (s.b[3396] && s.b[3400]) {
            s.store_scale(335, 740, p.p433);
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(335), 1.0, s.ad_value(746), (-1.0), s.ad_value(740), (-p.p337)));
            s.store_scaled_mul(782, 335, 740, (4.0 * p.p337));
        }

        if (s.b[3396] && s.b[3400]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[3396] && s.b[3400]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(336, A::add_scaled_inputs3(s.ad_value(335), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
            s.copy_ad(746, 336);
        }

        if (!s.b[3396]) {
            s.store_scalar(740, 0.0);
            s.store_scalar(746, 0.0);
        }

        if (s.v[81] != 0.0) {
            s.store_mul(751, 747, 247);
            s.store_sub_scaled_inputs(753, 747, -1.0, 748, 1.0);
            s.store_mul_sub_from_scalar_rhs(752, 747, 1.0, 247);
        }

        if (s.v[81] == 0.0) {
            s.store_scalar(751, 0.0);
            s.store_scalar(753, 0.0);
            s.store_scalar(752, 0.0);
        }

        s.store_scaled_mul(0, 949, 134, p.p87);

        s.store_scalar(22, A::ddx_projection(&s.ad_value(18), Some(5), None));

        s.store_scale(22, 22, p.p87);

        s.store_scalar(23, A::ddx_projection(&s.ad_value(18), Some(7), None));

        s.store_scale(23, 23, p.p87);

        s.b[3403] = (s.v[949] == 1.0);
        s.v[3403] = if s.b[3403] { 1.0 } else { 0.0 };

        if s.b[3403] {
            s.copy_ad(757, 23);
        }

        if (!s.b[3403]) {
            s.copy_ad(757, 22);
        }

        s.b[3405] = (p.p48 > 0.0);
        s.v[3405] = if s.b[3405] { 1.0 } else { 0.0 };

        s.b[3409] = (p.p53 > 0.0);
        s.v[3409] = if s.b[3409] { 1.0 } else { 0.0 };

        if (!s.b[3409]) {
            s.store_scalar(767, 0.0);
        }

        if (p.p28 != 0.0) {
            s.store_scalar(800, 1.0);
            s.store_scalar(801, 1.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17, eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_d_b9, eq0_e1018_d_b10, eq0_e1018_d_b11,) = {
    if s.b[3305] {
        let eq0_e1015: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[924]);
        let eq0_e1015_d_n0: f64 = (s.dn[924][0] * ddt_scale);
        let eq0_e1015_d_n1: f64 = (s.dn[924][1] * ddt_scale);
        let eq0_e1015_d_n2: f64 = (s.dn[924][2] * ddt_scale);
        let eq0_e1015_d_n3: f64 = (s.dn[924][3] * ddt_scale);
        let eq0_e1015_d_n4: f64 = (s.dn[924][4] * ddt_scale);
        let eq0_e1015_d_n5: f64 = (s.dn[924][5] * ddt_scale);
        let eq0_e1015_d_n6: f64 = (s.dn[924][6] * ddt_scale);
        let eq0_e1015_d_n7: f64 = (s.dn[924][7] * ddt_scale);
        let eq0_e1015_d_n8: f64 = (s.dn[924][8] * ddt_scale);
        let eq0_e1015_d_n9: f64 = (s.dn[924][9] * ddt_scale);
        let eq0_e1015_d_n10: f64 = (s.dn[924][10] * ddt_scale);
        let eq0_e1015_d_n11: f64 = (s.dn[924][11] * ddt_scale);
        let eq0_e1015_d_n12: f64 = (s.dn[924][12] * ddt_scale);
        let eq0_e1015_d_n13: f64 = (s.dn[924][13] * ddt_scale);
        let eq0_e1015_d_n14: f64 = (s.dn[924][14] * ddt_scale);
        let eq0_e1015_d_n15: f64 = (s.dn[924][15] * ddt_scale);
        let eq0_e1015_d_n16: f64 = (s.dn[924][16] * ddt_scale);
        let eq0_e1015_d_n17: f64 = (s.dn[924][17] * ddt_scale);
        let eq0_e1015_d_b0: f64 = (s.db[924][0] * ddt_scale);
        let eq0_e1015_d_b1: f64 = (s.db[924][1] * ddt_scale);
        let eq0_e1015_d_b2: f64 = (s.db[924][2] * ddt_scale);
        let eq0_e1015_d_b3: f64 = (s.db[924][3] * ddt_scale);
        let eq0_e1015_d_b4: f64 = (s.db[924][4] * ddt_scale);
        let eq0_e1015_d_b5: f64 = (s.db[924][5] * ddt_scale);
        let eq0_e1015_d_b6: f64 = (s.db[924][6] * ddt_scale);
        let eq0_e1015_d_b7: f64 = (s.db[924][7] * ddt_scale);
        let eq0_e1015_d_b8: f64 = (s.db[924][8] * ddt_scale);
        let eq0_e1015_d_b9: f64 = (s.db[924][9] * ddt_scale);
        let eq0_e1015_d_b10: f64 = (s.db[924][10] * ddt_scale);
        let eq0_e1015_d_b11: f64 = (s.db[924][11] * ddt_scale);
        let eq0_e1016: f64 = (s.v[926] + eq0_e1015);
        let eq0_e1016_d_n0: f64 = (s.dn[926][0] + eq0_e1015_d_n0);
        let eq0_e1016_d_n1: f64 = (s.dn[926][1] + eq0_e1015_d_n1);
        let eq0_e1016_d_n2: f64 = (s.dn[926][2] + eq0_e1015_d_n2);
        let eq0_e1016_d_n3: f64 = (s.dn[926][3] + eq0_e1015_d_n3);
        let eq0_e1016_d_n4: f64 = (s.dn[926][4] + eq0_e1015_d_n4);
        let eq0_e1016_d_n5: f64 = (s.dn[926][5] + eq0_e1015_d_n5);
        let eq0_e1016_d_n6: f64 = (s.dn[926][6] + eq0_e1015_d_n6);
        let eq0_e1016_d_n7: f64 = (s.dn[926][7] + eq0_e1015_d_n7);
        let eq0_e1016_d_n8: f64 = (s.dn[926][8] + eq0_e1015_d_n8);
        let eq0_e1016_d_n9: f64 = (s.dn[926][9] + eq0_e1015_d_n9);
        let eq0_e1016_d_n10: f64 = (s.dn[926][10] + eq0_e1015_d_n10);
        let eq0_e1016_d_n11: f64 = (s.dn[926][11] + eq0_e1015_d_n11);
        let eq0_e1016_d_n12: f64 = (s.dn[926][12] + eq0_e1015_d_n12);
        let eq0_e1016_d_n13: f64 = (s.dn[926][13] + eq0_e1015_d_n13);
        let eq0_e1016_d_n14: f64 = (s.dn[926][14] + eq0_e1015_d_n14);
        let eq0_e1016_d_n15: f64 = (s.dn[926][15] + eq0_e1015_d_n15);
        let eq0_e1016_d_n16: f64 = (s.dn[926][16] + eq0_e1015_d_n16);
        let eq0_e1016_d_n17: f64 = (s.dn[926][17] + eq0_e1015_d_n17);
        let eq0_e1016_d_b0: f64 = (s.db[926][0] + eq0_e1015_d_b0);
        let eq0_e1016_d_b1: f64 = (s.db[926][1] + eq0_e1015_d_b1);
        let eq0_e1016_d_b2: f64 = (s.db[926][2] + eq0_e1015_d_b2);
        let eq0_e1016_d_b3: f64 = (s.db[926][3] + eq0_e1015_d_b3);
        let eq0_e1016_d_b4: f64 = (s.db[926][4] + eq0_e1015_d_b4);
        let eq0_e1016_d_b5: f64 = (s.db[926][5] + eq0_e1015_d_b5);
        let eq0_e1016_d_b6: f64 = (s.db[926][6] + eq0_e1015_d_b6);
        let eq0_e1016_d_b7: f64 = (s.db[926][7] + eq0_e1015_d_b7);
        let eq0_e1016_d_b8: f64 = (s.db[926][8] + eq0_e1015_d_b8);
        let eq0_e1016_d_b9: f64 = (s.db[926][9] + eq0_e1015_d_b9);
        let eq0_e1016_d_b10: f64 = (s.db[926][10] + eq0_e1015_d_b10);
        let eq0_e1016_d_b11: f64 = (s.db[926][11] + eq0_e1015_d_b11);
        (eq0_e1016, eq0_e1016_d_n0, eq0_e1016_d_n1, eq0_e1016_d_n2, eq0_e1016_d_n3, eq0_e1016_d_n4, eq0_e1016_d_n5, eq0_e1016_d_n6, eq0_e1016_d_n7, eq0_e1016_d_n8, eq0_e1016_d_n9, eq0_e1016_d_n10, eq0_e1016_d_n11, eq0_e1016_d_n12, eq0_e1016_d_n13, eq0_e1016_d_n14, eq0_e1016_d_n15, eq0_e1016_d_n16, eq0_e1016_d_n17, eq0_e1016_d_b0, eq0_e1016_d_b1, eq0_e1016_d_b2, eq0_e1016_d_b3, eq0_e1016_d_b4, eq0_e1016_d_b5, eq0_e1016_d_b6, eq0_e1016_d_b7, eq0_e1016_d_b8, eq0_e1016_d_b9, eq0_e1016_d_b10, eq0_e1016_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1018;
        let eq0_node_derivatives: [f64; 18] = [eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17];
        let eq0_branch_derivatives: [f64; 12] = [eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_d_b9, eq0_e1018_d_b10, eq0_e1018_d_b11];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17, eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_d_b9, eq1_e1025_d_b10, eq1_e1025_d_b11,) = {
    if s.b[3305] {
        let eq1_e1022: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[925]);
        let eq1_e1022_d_n0: f64 = (s.dn[925][0] * ddt_scale);
        let eq1_e1022_d_n1: f64 = (s.dn[925][1] * ddt_scale);
        let eq1_e1022_d_n2: f64 = (s.dn[925][2] * ddt_scale);
        let eq1_e1022_d_n3: f64 = (s.dn[925][3] * ddt_scale);
        let eq1_e1022_d_n4: f64 = (s.dn[925][4] * ddt_scale);
        let eq1_e1022_d_n5: f64 = (s.dn[925][5] * ddt_scale);
        let eq1_e1022_d_n6: f64 = (s.dn[925][6] * ddt_scale);
        let eq1_e1022_d_n7: f64 = (s.dn[925][7] * ddt_scale);
        let eq1_e1022_d_n8: f64 = (s.dn[925][8] * ddt_scale);
        let eq1_e1022_d_n9: f64 = (s.dn[925][9] * ddt_scale);
        let eq1_e1022_d_n10: f64 = (s.dn[925][10] * ddt_scale);
        let eq1_e1022_d_n11: f64 = (s.dn[925][11] * ddt_scale);
        let eq1_e1022_d_n12: f64 = (s.dn[925][12] * ddt_scale);
        let eq1_e1022_d_n13: f64 = (s.dn[925][13] * ddt_scale);
        let eq1_e1022_d_n14: f64 = (s.dn[925][14] * ddt_scale);
        let eq1_e1022_d_n15: f64 = (s.dn[925][15] * ddt_scale);
        let eq1_e1022_d_n16: f64 = (s.dn[925][16] * ddt_scale);
        let eq1_e1022_d_n17: f64 = (s.dn[925][17] * ddt_scale);
        let eq1_e1022_d_b0: f64 = (s.db[925][0] * ddt_scale);
        let eq1_e1022_d_b1: f64 = (s.db[925][1] * ddt_scale);
        let eq1_e1022_d_b2: f64 = (s.db[925][2] * ddt_scale);
        let eq1_e1022_d_b3: f64 = (s.db[925][3] * ddt_scale);
        let eq1_e1022_d_b4: f64 = (s.db[925][4] * ddt_scale);
        let eq1_e1022_d_b5: f64 = (s.db[925][5] * ddt_scale);
        let eq1_e1022_d_b6: f64 = (s.db[925][6] * ddt_scale);
        let eq1_e1022_d_b7: f64 = (s.db[925][7] * ddt_scale);
        let eq1_e1022_d_b8: f64 = (s.db[925][8] * ddt_scale);
        let eq1_e1022_d_b9: f64 = (s.db[925][9] * ddt_scale);
        let eq1_e1022_d_b10: f64 = (s.db[925][10] * ddt_scale);
        let eq1_e1022_d_b11: f64 = (s.db[925][11] * ddt_scale);
        let eq1_e1023: f64 = (s.v[927] + eq1_e1022);
        let eq1_e1023_d_n0: f64 = (s.dn[927][0] + eq1_e1022_d_n0);
        let eq1_e1023_d_n1: f64 = (s.dn[927][1] + eq1_e1022_d_n1);
        let eq1_e1023_d_n2: f64 = (s.dn[927][2] + eq1_e1022_d_n2);
        let eq1_e1023_d_n3: f64 = (s.dn[927][3] + eq1_e1022_d_n3);
        let eq1_e1023_d_n4: f64 = (s.dn[927][4] + eq1_e1022_d_n4);
        let eq1_e1023_d_n5: f64 = (s.dn[927][5] + eq1_e1022_d_n5);
        let eq1_e1023_d_n6: f64 = (s.dn[927][6] + eq1_e1022_d_n6);
        let eq1_e1023_d_n7: f64 = (s.dn[927][7] + eq1_e1022_d_n7);
        let eq1_e1023_d_n8: f64 = (s.dn[927][8] + eq1_e1022_d_n8);
        let eq1_e1023_d_n9: f64 = (s.dn[927][9] + eq1_e1022_d_n9);
        let eq1_e1023_d_n10: f64 = (s.dn[927][10] + eq1_e1022_d_n10);
        let eq1_e1023_d_n11: f64 = (s.dn[927][11] + eq1_e1022_d_n11);
        let eq1_e1023_d_n12: f64 = (s.dn[927][12] + eq1_e1022_d_n12);
        let eq1_e1023_d_n13: f64 = (s.dn[927][13] + eq1_e1022_d_n13);
        let eq1_e1023_d_n14: f64 = (s.dn[927][14] + eq1_e1022_d_n14);
        let eq1_e1023_d_n15: f64 = (s.dn[927][15] + eq1_e1022_d_n15);
        let eq1_e1023_d_n16: f64 = (s.dn[927][16] + eq1_e1022_d_n16);
        let eq1_e1023_d_n17: f64 = (s.dn[927][17] + eq1_e1022_d_n17);
        let eq1_e1023_d_b0: f64 = (s.db[927][0] + eq1_e1022_d_b0);
        let eq1_e1023_d_b1: f64 = (s.db[927][1] + eq1_e1022_d_b1);
        let eq1_e1023_d_b2: f64 = (s.db[927][2] + eq1_e1022_d_b2);
        let eq1_e1023_d_b3: f64 = (s.db[927][3] + eq1_e1022_d_b3);
        let eq1_e1023_d_b4: f64 = (s.db[927][4] + eq1_e1022_d_b4);
        let eq1_e1023_d_b5: f64 = (s.db[927][5] + eq1_e1022_d_b5);
        let eq1_e1023_d_b6: f64 = (s.db[927][6] + eq1_e1022_d_b6);
        let eq1_e1023_d_b7: f64 = (s.db[927][7] + eq1_e1022_d_b7);
        let eq1_e1023_d_b8: f64 = (s.db[927][8] + eq1_e1022_d_b8);
        let eq1_e1023_d_b9: f64 = (s.db[927][9] + eq1_e1022_d_b9);
        let eq1_e1023_d_b10: f64 = (s.db[927][10] + eq1_e1022_d_b10);
        let eq1_e1023_d_b11: f64 = (s.db[927][11] + eq1_e1022_d_b11);
        (eq1_e1023, eq1_e1023_d_n0, eq1_e1023_d_n1, eq1_e1023_d_n2, eq1_e1023_d_n3, eq1_e1023_d_n4, eq1_e1023_d_n5, eq1_e1023_d_n6, eq1_e1023_d_n7, eq1_e1023_d_n8, eq1_e1023_d_n9, eq1_e1023_d_n10, eq1_e1023_d_n11, eq1_e1023_d_n12, eq1_e1023_d_n13, eq1_e1023_d_n14, eq1_e1023_d_n15, eq1_e1023_d_n16, eq1_e1023_d_n17, eq1_e1023_d_b0, eq1_e1023_d_b1, eq1_e1023_d_b2, eq1_e1023_d_b3, eq1_e1023_d_b4, eq1_e1023_d_b5, eq1_e1023_d_b6, eq1_e1023_d_b7, eq1_e1023_d_b8, eq1_e1023_d_b9, eq1_e1023_d_b10, eq1_e1023_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1025;
        let eq1_node_derivatives: [f64; 18] = [eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17];
        let eq1_branch_derivatives: [f64; 12] = [eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_d_b9, eq1_e1025_d_b10, eq1_e1025_d_b11];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1030,) = {
    if (!s.b[3305]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1030;
        stamper.stamp_potential_const_local(
            0,
            eq2_value,
        );
        let (eq3_e1035,) = {
    if (!s.b[3305]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1035;
        stamper.stamp_potential_const_local(
            1,
            eq3_value,
        );
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17, eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_d_b9, eq4_e1042_d_b10, eq4_e1042_d_b11,) = {
    if s.b[3306] {
        let eq4_e1039: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[931]);
        let eq4_e1039_d_n0: f64 = (s.dn[931][0] * ddt_scale);
        let eq4_e1039_d_n1: f64 = (s.dn[931][1] * ddt_scale);
        let eq4_e1039_d_n2: f64 = (s.dn[931][2] * ddt_scale);
        let eq4_e1039_d_n3: f64 = (s.dn[931][3] * ddt_scale);
        let eq4_e1039_d_n4: f64 = (s.dn[931][4] * ddt_scale);
        let eq4_e1039_d_n5: f64 = (s.dn[931][5] * ddt_scale);
        let eq4_e1039_d_n6: f64 = (s.dn[931][6] * ddt_scale);
        let eq4_e1039_d_n7: f64 = (s.dn[931][7] * ddt_scale);
        let eq4_e1039_d_n8: f64 = (s.dn[931][8] * ddt_scale);
        let eq4_e1039_d_n9: f64 = (s.dn[931][9] * ddt_scale);
        let eq4_e1039_d_n10: f64 = (s.dn[931][10] * ddt_scale);
        let eq4_e1039_d_n11: f64 = (s.dn[931][11] * ddt_scale);
        let eq4_e1039_d_n12: f64 = (s.dn[931][12] * ddt_scale);
        let eq4_e1039_d_n13: f64 = (s.dn[931][13] * ddt_scale);
        let eq4_e1039_d_n14: f64 = (s.dn[931][14] * ddt_scale);
        let eq4_e1039_d_n15: f64 = (s.dn[931][15] * ddt_scale);
        let eq4_e1039_d_n16: f64 = (s.dn[931][16] * ddt_scale);
        let eq4_e1039_d_n17: f64 = (s.dn[931][17] * ddt_scale);
        let eq4_e1039_d_b0: f64 = (s.db[931][0] * ddt_scale);
        let eq4_e1039_d_b1: f64 = (s.db[931][1] * ddt_scale);
        let eq4_e1039_d_b2: f64 = (s.db[931][2] * ddt_scale);
        let eq4_e1039_d_b3: f64 = (s.db[931][3] * ddt_scale);
        let eq4_e1039_d_b4: f64 = (s.db[931][4] * ddt_scale);
        let eq4_e1039_d_b5: f64 = (s.db[931][5] * ddt_scale);
        let eq4_e1039_d_b6: f64 = (s.db[931][6] * ddt_scale);
        let eq4_e1039_d_b7: f64 = (s.db[931][7] * ddt_scale);
        let eq4_e1039_d_b8: f64 = (s.db[931][8] * ddt_scale);
        let eq4_e1039_d_b9: f64 = (s.db[931][9] * ddt_scale);
        let eq4_e1039_d_b10: f64 = (s.db[931][10] * ddt_scale);
        let eq4_e1039_d_b11: f64 = (s.db[931][11] * ddt_scale);
        let eq4_e1040: f64 = (s.v[932] + eq4_e1039);
        let eq4_e1040_d_n0: f64 = (s.dn[932][0] + eq4_e1039_d_n0);
        let eq4_e1040_d_n1: f64 = (s.dn[932][1] + eq4_e1039_d_n1);
        let eq4_e1040_d_n2: f64 = (s.dn[932][2] + eq4_e1039_d_n2);
        let eq4_e1040_d_n3: f64 = (s.dn[932][3] + eq4_e1039_d_n3);
        let eq4_e1040_d_n4: f64 = (s.dn[932][4] + eq4_e1039_d_n4);
        let eq4_e1040_d_n5: f64 = (s.dn[932][5] + eq4_e1039_d_n5);
        let eq4_e1040_d_n6: f64 = (s.dn[932][6] + eq4_e1039_d_n6);
        let eq4_e1040_d_n7: f64 = (s.dn[932][7] + eq4_e1039_d_n7);
        let eq4_e1040_d_n8: f64 = (s.dn[932][8] + eq4_e1039_d_n8);
        let eq4_e1040_d_n9: f64 = (s.dn[932][9] + eq4_e1039_d_n9);
        let eq4_e1040_d_n10: f64 = (s.dn[932][10] + eq4_e1039_d_n10);
        let eq4_e1040_d_n11: f64 = (s.dn[932][11] + eq4_e1039_d_n11);
        let eq4_e1040_d_n12: f64 = (s.dn[932][12] + eq4_e1039_d_n12);
        let eq4_e1040_d_n13: f64 = (s.dn[932][13] + eq4_e1039_d_n13);
        let eq4_e1040_d_n14: f64 = (s.dn[932][14] + eq4_e1039_d_n14);
        let eq4_e1040_d_n15: f64 = (s.dn[932][15] + eq4_e1039_d_n15);
        let eq4_e1040_d_n16: f64 = (s.dn[932][16] + eq4_e1039_d_n16);
        let eq4_e1040_d_n17: f64 = (s.dn[932][17] + eq4_e1039_d_n17);
        let eq4_e1040_d_b0: f64 = (s.db[932][0] + eq4_e1039_d_b0);
        let eq4_e1040_d_b1: f64 = (s.db[932][1] + eq4_e1039_d_b1);
        let eq4_e1040_d_b2: f64 = (s.db[932][2] + eq4_e1039_d_b2);
        let eq4_e1040_d_b3: f64 = (s.db[932][3] + eq4_e1039_d_b3);
        let eq4_e1040_d_b4: f64 = (s.db[932][4] + eq4_e1039_d_b4);
        let eq4_e1040_d_b5: f64 = (s.db[932][5] + eq4_e1039_d_b5);
        let eq4_e1040_d_b6: f64 = (s.db[932][6] + eq4_e1039_d_b6);
        let eq4_e1040_d_b7: f64 = (s.db[932][7] + eq4_e1039_d_b7);
        let eq4_e1040_d_b8: f64 = (s.db[932][8] + eq4_e1039_d_b8);
        let eq4_e1040_d_b9: f64 = (s.db[932][9] + eq4_e1039_d_b9);
        let eq4_e1040_d_b10: f64 = (s.db[932][10] + eq4_e1039_d_b10);
        let eq4_e1040_d_b11: f64 = (s.db[932][11] + eq4_e1039_d_b11);
        (eq4_e1040, eq4_e1040_d_n0, eq4_e1040_d_n1, eq4_e1040_d_n2, eq4_e1040_d_n3, eq4_e1040_d_n4, eq4_e1040_d_n5, eq4_e1040_d_n6, eq4_e1040_d_n7, eq4_e1040_d_n8, eq4_e1040_d_n9, eq4_e1040_d_n10, eq4_e1040_d_n11, eq4_e1040_d_n12, eq4_e1040_d_n13, eq4_e1040_d_n14, eq4_e1040_d_n15, eq4_e1040_d_n16, eq4_e1040_d_n17, eq4_e1040_d_b0, eq4_e1040_d_b1, eq4_e1040_d_b2, eq4_e1040_d_b3, eq4_e1040_d_b4, eq4_e1040_d_b5, eq4_e1040_d_b6, eq4_e1040_d_b7, eq4_e1040_d_b8, eq4_e1040_d_b9, eq4_e1040_d_b10, eq4_e1040_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1042;
        let eq4_node_derivatives: [f64; 18] = [eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17];
        let eq4_branch_derivatives: [f64; 12] = [eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_d_b9, eq4_e1042_d_b10, eq4_e1042_d_b11];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1047,) = {
    if (!s.b[3306]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1047;
        stamper.stamp_potential_const_local(
            2,
            eq5_value,
        );
        let eq6_e1051: f64 = (s.v[134] + s.v[400]);
        let eq6_e1051_d_n0: f64 = (s.dn[134][0] + s.dn[400][0]);
        let eq6_e1051_d_n1: f64 = (s.dn[134][1] + s.dn[400][1]);
        let eq6_e1051_d_n2: f64 = (s.dn[134][2] + s.dn[400][2]);
        let eq6_e1051_d_n3: f64 = (s.dn[134][3] + s.dn[400][3]);
        let eq6_e1051_d_n4: f64 = (s.dn[134][4] + s.dn[400][4]);
        let eq6_e1051_d_n5: f64 = (s.dn[134][5] + s.dn[400][5]);
        let eq6_e1051_d_n6: f64 = (s.dn[134][6] + s.dn[400][6]);
        let eq6_e1051_d_n7: f64 = (s.dn[134][7] + s.dn[400][7]);
        let eq6_e1051_d_n8: f64 = (s.dn[134][8] + s.dn[400][8]);
        let eq6_e1051_d_n9: f64 = (s.dn[134][9] + s.dn[400][9]);
        let eq6_e1051_d_n10: f64 = (s.dn[134][10] + s.dn[400][10]);
        let eq6_e1051_d_n11: f64 = (s.dn[134][11] + s.dn[400][11]);
        let eq6_e1051_d_n12: f64 = (s.dn[134][12] + s.dn[400][12]);
        let eq6_e1051_d_n13: f64 = (s.dn[134][13] + s.dn[400][13]);
        let eq6_e1051_d_n14: f64 = (s.dn[134][14] + s.dn[400][14]);
        let eq6_e1051_d_n15: f64 = (s.dn[134][15] + s.dn[400][15]);
        let eq6_e1051_d_n16: f64 = (s.dn[134][16] + s.dn[400][16]);
        let eq6_e1051_d_n17: f64 = (s.dn[134][17] + s.dn[400][17]);
        let eq6_e1051_d_b0: f64 = (s.db[134][0] + s.db[400][0]);
        let eq6_e1051_d_b1: f64 = (s.db[134][1] + s.db[400][1]);
        let eq6_e1051_d_b2: f64 = (s.db[134][2] + s.db[400][2]);
        let eq6_e1051_d_b3: f64 = (s.db[134][3] + s.db[400][3]);
        let eq6_e1051_d_b4: f64 = (s.db[134][4] + s.db[400][4]);
        let eq6_e1051_d_b5: f64 = (s.db[134][5] + s.db[400][5]);
        let eq6_e1051_d_b6: f64 = (s.db[134][6] + s.db[400][6]);
        let eq6_e1051_d_b7: f64 = (s.db[134][7] + s.db[400][7]);
        let eq6_e1051_d_b8: f64 = (s.db[134][8] + s.db[400][8]);
        let eq6_e1051_d_b9: f64 = (s.db[134][9] + s.db[400][9]);
        let eq6_e1051_d_b10: f64 = (s.db[134][10] + s.db[400][10]);
        let eq6_e1051_d_b11: f64 = (s.db[134][11] + s.db[400][11]);
        let eq6_e1053: f64 = (eq6_e1051 - s.v[738]);
        let eq6_e1053_d_n0: f64 = (eq6_e1051_d_n0 - s.dn[738][0]);
        let eq6_e1053_d_n1: f64 = (eq6_e1051_d_n1 - s.dn[738][1]);
        let eq6_e1053_d_n2: f64 = (eq6_e1051_d_n2 - s.dn[738][2]);
        let eq6_e1053_d_n3: f64 = (eq6_e1051_d_n3 - s.dn[738][3]);
        let eq6_e1053_d_n4: f64 = (eq6_e1051_d_n4 - s.dn[738][4]);
        let eq6_e1053_d_n5: f64 = (eq6_e1051_d_n5 - s.dn[738][5]);
        let eq6_e1053_d_n6: f64 = (eq6_e1051_d_n6 - s.dn[738][6]);
        let eq6_e1053_d_n7: f64 = (eq6_e1051_d_n7 - s.dn[738][7]);
        let eq6_e1053_d_n8: f64 = (eq6_e1051_d_n8 - s.dn[738][8]);
        let eq6_e1053_d_n9: f64 = (eq6_e1051_d_n9 - s.dn[738][9]);
        let eq6_e1053_d_n10: f64 = (eq6_e1051_d_n10 - s.dn[738][10]);
        let eq6_e1053_d_n11: f64 = (eq6_e1051_d_n11 - s.dn[738][11]);
        let eq6_e1053_d_n12: f64 = (eq6_e1051_d_n12 - s.dn[738][12]);
        let eq6_e1053_d_n13: f64 = (eq6_e1051_d_n13 - s.dn[738][13]);
        let eq6_e1053_d_n14: f64 = (eq6_e1051_d_n14 - s.dn[738][14]);
        let eq6_e1053_d_n15: f64 = (eq6_e1051_d_n15 - s.dn[738][15]);
        let eq6_e1053_d_n16: f64 = (eq6_e1051_d_n16 - s.dn[738][16]);
        let eq6_e1053_d_n17: f64 = (eq6_e1051_d_n17 - s.dn[738][17]);
        let eq6_e1053_d_b0: f64 = (eq6_e1051_d_b0 - s.db[738][0]);
        let eq6_e1053_d_b1: f64 = (eq6_e1051_d_b1 - s.db[738][1]);
        let eq6_e1053_d_b2: f64 = (eq6_e1051_d_b2 - s.db[738][2]);
        let eq6_e1053_d_b3: f64 = (eq6_e1051_d_b3 - s.db[738][3]);
        let eq6_e1053_d_b4: f64 = (eq6_e1051_d_b4 - s.db[738][4]);
        let eq6_e1053_d_b5: f64 = (eq6_e1051_d_b5 - s.db[738][5]);
        let eq6_e1053_d_b6: f64 = (eq6_e1051_d_b6 - s.db[738][6]);
        let eq6_e1053_d_b7: f64 = (eq6_e1051_d_b7 - s.db[738][7]);
        let eq6_e1053_d_b8: f64 = (eq6_e1051_d_b8 - s.db[738][8]);
        let eq6_e1053_d_b9: f64 = (eq6_e1051_d_b9 - s.db[738][9]);
        let eq6_e1053_d_b10: f64 = (eq6_e1051_d_b10 - s.db[738][10]);
        let eq6_e1053_d_b11: f64 = (eq6_e1051_d_b11 - s.db[738][11]);
        let eq6_e1054: f64 = (p.p87 * eq6_e1053);
        let eq6_e1054_d_n0: f64 = (p.p87 * eq6_e1053_d_n0);
        let eq6_e1054_d_n1: f64 = (p.p87 * eq6_e1053_d_n1);
        let eq6_e1054_d_n2: f64 = (p.p87 * eq6_e1053_d_n2);
        let eq6_e1054_d_n3: f64 = (p.p87 * eq6_e1053_d_n3);
        let eq6_e1054_d_n4: f64 = (p.p87 * eq6_e1053_d_n4);
        let eq6_e1054_d_n5: f64 = (p.p87 * eq6_e1053_d_n5);
        let eq6_e1054_d_n6: f64 = (p.p87 * eq6_e1053_d_n6);
        let eq6_e1054_d_n7: f64 = (p.p87 * eq6_e1053_d_n7);
        let eq6_e1054_d_n8: f64 = (p.p87 * eq6_e1053_d_n8);
        let eq6_e1054_d_n9: f64 = (p.p87 * eq6_e1053_d_n9);
        let eq6_e1054_d_n10: f64 = (p.p87 * eq6_e1053_d_n10);
        let eq6_e1054_d_n11: f64 = (p.p87 * eq6_e1053_d_n11);
        let eq6_e1054_d_n12: f64 = (p.p87 * eq6_e1053_d_n12);
        let eq6_e1054_d_n13: f64 = (p.p87 * eq6_e1053_d_n13);
        let eq6_e1054_d_n14: f64 = (p.p87 * eq6_e1053_d_n14);
        let eq6_e1054_d_n15: f64 = (p.p87 * eq6_e1053_d_n15);
        let eq6_e1054_d_n16: f64 = (p.p87 * eq6_e1053_d_n16);
        let eq6_e1054_d_n17: f64 = (p.p87 * eq6_e1053_d_n17);
        let eq6_e1054_d_b0: f64 = (p.p87 * eq6_e1053_d_b0);
        let eq6_e1054_d_b1: f64 = (p.p87 * eq6_e1053_d_b1);
        let eq6_e1054_d_b2: f64 = (p.p87 * eq6_e1053_d_b2);
        let eq6_e1054_d_b3: f64 = (p.p87 * eq6_e1053_d_b3);
        let eq6_e1054_d_b4: f64 = (p.p87 * eq6_e1053_d_b4);
        let eq6_e1054_d_b5: f64 = (p.p87 * eq6_e1053_d_b5);
        let eq6_e1054_d_b6: f64 = (p.p87 * eq6_e1053_d_b6);
        let eq6_e1054_d_b7: f64 = (p.p87 * eq6_e1053_d_b7);
        let eq6_e1054_d_b8: f64 = (p.p87 * eq6_e1053_d_b8);
        let eq6_e1054_d_b9: f64 = (p.p87 * eq6_e1053_d_b9);
        let eq6_e1054_d_b10: f64 = (p.p87 * eq6_e1053_d_b10);
        let eq6_e1054_d_b11: f64 = (p.p87 * eq6_e1053_d_b11);
        let eq6_value: f64 = eq6_e1054;
        let eq6_node_derivatives: [f64; 18] = [eq6_e1054_d_n0, eq6_e1054_d_n1, eq6_e1054_d_n2, eq6_e1054_d_n3, eq6_e1054_d_n4, eq6_e1054_d_n5, eq6_e1054_d_n6, eq6_e1054_d_n7, eq6_e1054_d_n8, eq6_e1054_d_n9, eq6_e1054_d_n10, eq6_e1054_d_n11, eq6_e1054_d_n12, eq6_e1054_d_n13, eq6_e1054_d_n14, eq6_e1054_d_n15, eq6_e1054_d_n16, eq6_e1054_d_n17];
        let eq6_branch_derivatives: [f64; 12] = [eq6_e1054_d_b0, eq6_e1054_d_b1, eq6_e1054_d_b2, eq6_e1054_d_b3, eq6_e1054_d_b4, eq6_e1054_d_b5, eq6_e1054_d_b6, eq6_e1054_d_b7, eq6_e1054_d_b8, eq6_e1054_d_b9, eq6_e1054_d_b10, eq6_e1054_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e1058: f64 = (s.v[424] - s.v[425]);
        let eq7_e1058_d_n0: f64 = (s.dn[424][0] - s.dn[425][0]);
        let eq7_e1058_d_n1: f64 = (s.dn[424][1] - s.dn[425][1]);
        let eq7_e1058_d_n2: f64 = (s.dn[424][2] - s.dn[425][2]);
        let eq7_e1058_d_n3: f64 = (s.dn[424][3] - s.dn[425][3]);
        let eq7_e1058_d_n4: f64 = (s.dn[424][4] - s.dn[425][4]);
        let eq7_e1058_d_n5: f64 = (s.dn[424][5] - s.dn[425][5]);
        let eq7_e1058_d_n6: f64 = (s.dn[424][6] - s.dn[425][6]);
        let eq7_e1058_d_n7: f64 = (s.dn[424][7] - s.dn[425][7]);
        let eq7_e1058_d_n8: f64 = (s.dn[424][8] - s.dn[425][8]);
        let eq7_e1058_d_n9: f64 = (s.dn[424][9] - s.dn[425][9]);
        let eq7_e1058_d_n10: f64 = (s.dn[424][10] - s.dn[425][10]);
        let eq7_e1058_d_n11: f64 = (s.dn[424][11] - s.dn[425][11]);
        let eq7_e1058_d_n12: f64 = (s.dn[424][12] - s.dn[425][12]);
        let eq7_e1058_d_n13: f64 = (s.dn[424][13] - s.dn[425][13]);
        let eq7_e1058_d_n14: f64 = (s.dn[424][14] - s.dn[425][14]);
        let eq7_e1058_d_n15: f64 = (s.dn[424][15] - s.dn[425][15]);
        let eq7_e1058_d_n16: f64 = (s.dn[424][16] - s.dn[425][16]);
        let eq7_e1058_d_n17: f64 = (s.dn[424][17] - s.dn[425][17]);
        let eq7_e1058_d_b0: f64 = (s.db[424][0] - s.db[425][0]);
        let eq7_e1058_d_b1: f64 = (s.db[424][1] - s.db[425][1]);
        let eq7_e1058_d_b2: f64 = (s.db[424][2] - s.db[425][2]);
        let eq7_e1058_d_b3: f64 = (s.db[424][3] - s.db[425][3]);
        let eq7_e1058_d_b4: f64 = (s.db[424][4] - s.db[425][4]);
        let eq7_e1058_d_b5: f64 = (s.db[424][5] - s.db[425][5]);
        let eq7_e1058_d_b6: f64 = (s.db[424][6] - s.db[425][6]);
        let eq7_e1058_d_b7: f64 = (s.db[424][7] - s.db[425][7]);
        let eq7_e1058_d_b8: f64 = (s.db[424][8] - s.db[425][8]);
        let eq7_e1058_d_b9: f64 = (s.db[424][9] - s.db[425][9]);
        let eq7_e1058_d_b10: f64 = (s.db[424][10] - s.db[425][10]);
        let eq7_e1058_d_b11: f64 = (s.db[424][11] - s.db[425][11]);
        let eq7_e1059: f64 = (p.p87 * eq7_e1058);
        let eq7_e1059_d_n0: f64 = (p.p87 * eq7_e1058_d_n0);
        let eq7_e1059_d_n1: f64 = (p.p87 * eq7_e1058_d_n1);
        let eq7_e1059_d_n2: f64 = (p.p87 * eq7_e1058_d_n2);
        let eq7_e1059_d_n3: f64 = (p.p87 * eq7_e1058_d_n3);
        let eq7_e1059_d_n4: f64 = (p.p87 * eq7_e1058_d_n4);
        let eq7_e1059_d_n5: f64 = (p.p87 * eq7_e1058_d_n5);
        let eq7_e1059_d_n6: f64 = (p.p87 * eq7_e1058_d_n6);
        let eq7_e1059_d_n7: f64 = (p.p87 * eq7_e1058_d_n7);
        let eq7_e1059_d_n8: f64 = (p.p87 * eq7_e1058_d_n8);
        let eq7_e1059_d_n9: f64 = (p.p87 * eq7_e1058_d_n9);
        let eq7_e1059_d_n10: f64 = (p.p87 * eq7_e1058_d_n10);
        let eq7_e1059_d_n11: f64 = (p.p87 * eq7_e1058_d_n11);
        let eq7_e1059_d_n12: f64 = (p.p87 * eq7_e1058_d_n12);
        let eq7_e1059_d_n13: f64 = (p.p87 * eq7_e1058_d_n13);
        let eq7_e1059_d_n14: f64 = (p.p87 * eq7_e1058_d_n14);
        let eq7_e1059_d_n15: f64 = (p.p87 * eq7_e1058_d_n15);
        let eq7_e1059_d_n16: f64 = (p.p87 * eq7_e1058_d_n16);
        let eq7_e1059_d_n17: f64 = (p.p87 * eq7_e1058_d_n17);
        let eq7_e1059_d_b0: f64 = (p.p87 * eq7_e1058_d_b0);
        let eq7_e1059_d_b1: f64 = (p.p87 * eq7_e1058_d_b1);
        let eq7_e1059_d_b2: f64 = (p.p87 * eq7_e1058_d_b2);
        let eq7_e1059_d_b3: f64 = (p.p87 * eq7_e1058_d_b3);
        let eq7_e1059_d_b4: f64 = (p.p87 * eq7_e1058_d_b4);
        let eq7_e1059_d_b5: f64 = (p.p87 * eq7_e1058_d_b5);
        let eq7_e1059_d_b6: f64 = (p.p87 * eq7_e1058_d_b6);
        let eq7_e1059_d_b7: f64 = (p.p87 * eq7_e1058_d_b7);
        let eq7_e1059_d_b8: f64 = (p.p87 * eq7_e1058_d_b8);
        let eq7_e1059_d_b9: f64 = (p.p87 * eq7_e1058_d_b9);
        let eq7_e1059_d_b10: f64 = (p.p87 * eq7_e1058_d_b10);
        let eq7_e1059_d_b11: f64 = (p.p87 * eq7_e1058_d_b11);
        let eq7_value: f64 = eq7_e1059;
        let eq7_node_derivatives: [f64; 18] = [eq7_e1059_d_n0, eq7_e1059_d_n1, eq7_e1059_d_n2, eq7_e1059_d_n3, eq7_e1059_d_n4, eq7_e1059_d_n5, eq7_e1059_d_n6, eq7_e1059_d_n7, eq7_e1059_d_n8, eq7_e1059_d_n9, eq7_e1059_d_n10, eq7_e1059_d_n11, eq7_e1059_d_n12, eq7_e1059_d_n13, eq7_e1059_d_n14, eq7_e1059_d_n15, eq7_e1059_d_n16, eq7_e1059_d_n17];
        let eq7_branch_derivatives: [f64; 12] = [eq7_e1059_d_b0, eq7_e1059_d_b1, eq7_e1059_d_b2, eq7_e1059_d_b3, eq7_e1059_d_b4, eq7_e1059_d_b5, eq7_e1059_d_b6, eq7_e1059_d_b7, eq7_e1059_d_b8, eq7_e1059_d_b9, eq7_e1059_d_b10, eq7_e1059_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq8_e1063: f64 = (s.v[203] + s.v[280]);
        let eq8_e1063_d_n0: f64 = (s.dn[203][0] + s.dn[280][0]);
        let eq8_e1063_d_n1: f64 = (s.dn[203][1] + s.dn[280][1]);
        let eq8_e1063_d_n2: f64 = (s.dn[203][2] + s.dn[280][2]);
        let eq8_e1063_d_n3: f64 = (s.dn[203][3] + s.dn[280][3]);
        let eq8_e1063_d_n4: f64 = (s.dn[203][4] + s.dn[280][4]);
        let eq8_e1063_d_n5: f64 = (s.dn[203][5] + s.dn[280][5]);
        let eq8_e1063_d_n6: f64 = (s.dn[203][6] + s.dn[280][6]);
        let eq8_e1063_d_n7: f64 = (s.dn[203][7] + s.dn[280][7]);
        let eq8_e1063_d_n8: f64 = (s.dn[203][8] + s.dn[280][8]);
        let eq8_e1063_d_n9: f64 = (s.dn[203][9] + s.dn[280][9]);
        let eq8_e1063_d_n10: f64 = (s.dn[203][10] + s.dn[280][10]);
        let eq8_e1063_d_n11: f64 = (s.dn[203][11] + s.dn[280][11]);
        let eq8_e1063_d_n12: f64 = (s.dn[203][12] + s.dn[280][12]);
        let eq8_e1063_d_n13: f64 = (s.dn[203][13] + s.dn[280][13]);
        let eq8_e1063_d_n14: f64 = (s.dn[203][14] + s.dn[280][14]);
        let eq8_e1063_d_n15: f64 = (s.dn[203][15] + s.dn[280][15]);
        let eq8_e1063_d_n16: f64 = (s.dn[203][16] + s.dn[280][16]);
        let eq8_e1063_d_n17: f64 = (s.dn[203][17] + s.dn[280][17]);
        let eq8_e1063_d_b0: f64 = (s.db[203][0] + s.db[280][0]);
        let eq8_e1063_d_b1: f64 = (s.db[203][1] + s.db[280][1]);
        let eq8_e1063_d_b2: f64 = (s.db[203][2] + s.db[280][2]);
        let eq8_e1063_d_b3: f64 = (s.db[203][3] + s.db[280][3]);
        let eq8_e1063_d_b4: f64 = (s.db[203][4] + s.db[280][4]);
        let eq8_e1063_d_b5: f64 = (s.db[203][5] + s.db[280][5]);
        let eq8_e1063_d_b6: f64 = (s.db[203][6] + s.db[280][6]);
        let eq8_e1063_d_b7: f64 = (s.db[203][7] + s.db[280][7]);
        let eq8_e1063_d_b8: f64 = (s.db[203][8] + s.db[280][8]);
        let eq8_e1063_d_b9: f64 = (s.db[203][9] + s.db[280][9]);
        let eq8_e1063_d_b10: f64 = (s.db[203][10] + s.db[280][10]);
        let eq8_e1063_d_b11: f64 = (s.db[203][11] + s.db[280][11]);
        let eq8_e1065: f64 = (eq8_e1063 + s.v[431]);
        let eq8_e1065_d_n0: f64 = (eq8_e1063_d_n0 + s.dn[431][0]);
        let eq8_e1065_d_n1: f64 = (eq8_e1063_d_n1 + s.dn[431][1]);
        let eq8_e1065_d_n2: f64 = (eq8_e1063_d_n2 + s.dn[431][2]);
        let eq8_e1065_d_n3: f64 = (eq8_e1063_d_n3 + s.dn[431][3]);
        let eq8_e1065_d_n4: f64 = (eq8_e1063_d_n4 + s.dn[431][4]);
        let eq8_e1065_d_n5: f64 = (eq8_e1063_d_n5 + s.dn[431][5]);
        let eq8_e1065_d_n6: f64 = (eq8_e1063_d_n6 + s.dn[431][6]);
        let eq8_e1065_d_n7: f64 = (eq8_e1063_d_n7 + s.dn[431][7]);
        let eq8_e1065_d_n8: f64 = (eq8_e1063_d_n8 + s.dn[431][8]);
        let eq8_e1065_d_n9: f64 = (eq8_e1063_d_n9 + s.dn[431][9]);
        let eq8_e1065_d_n10: f64 = (eq8_e1063_d_n10 + s.dn[431][10]);
        let eq8_e1065_d_n11: f64 = (eq8_e1063_d_n11 + s.dn[431][11]);
        let eq8_e1065_d_n12: f64 = (eq8_e1063_d_n12 + s.dn[431][12]);
        let eq8_e1065_d_n13: f64 = (eq8_e1063_d_n13 + s.dn[431][13]);
        let eq8_e1065_d_n14: f64 = (eq8_e1063_d_n14 + s.dn[431][14]);
        let eq8_e1065_d_n15: f64 = (eq8_e1063_d_n15 + s.dn[431][15]);
        let eq8_e1065_d_n16: f64 = (eq8_e1063_d_n16 + s.dn[431][16]);
        let eq8_e1065_d_n17: f64 = (eq8_e1063_d_n17 + s.dn[431][17]);
        let eq8_e1065_d_b0: f64 = (eq8_e1063_d_b0 + s.db[431][0]);
        let eq8_e1065_d_b1: f64 = (eq8_e1063_d_b1 + s.db[431][1]);
        let eq8_e1065_d_b2: f64 = (eq8_e1063_d_b2 + s.db[431][2]);
        let eq8_e1065_d_b3: f64 = (eq8_e1063_d_b3 + s.db[431][3]);
        let eq8_e1065_d_b4: f64 = (eq8_e1063_d_b4 + s.db[431][4]);
        let eq8_e1065_d_b5: f64 = (eq8_e1063_d_b5 + s.db[431][5]);
        let eq8_e1065_d_b6: f64 = (eq8_e1063_d_b6 + s.db[431][6]);
        let eq8_e1065_d_b7: f64 = (eq8_e1063_d_b7 + s.db[431][7]);
        let eq8_e1065_d_b8: f64 = (eq8_e1063_d_b8 + s.db[431][8]);
        let eq8_e1065_d_b9: f64 = (eq8_e1063_d_b9 + s.db[431][9]);
        let eq8_e1065_d_b10: f64 = (eq8_e1063_d_b10 + s.db[431][10]);
        let eq8_e1065_d_b11: f64 = (eq8_e1063_d_b11 + s.db[431][11]);
        let eq8_e1066: f64 = (p.p87 * eq8_e1065);
        let eq8_e1066_d_n0: f64 = (p.p87 * eq8_e1065_d_n0);
        let eq8_e1066_d_n1: f64 = (p.p87 * eq8_e1065_d_n1);
        let eq8_e1066_d_n2: f64 = (p.p87 * eq8_e1065_d_n2);
        let eq8_e1066_d_n3: f64 = (p.p87 * eq8_e1065_d_n3);
        let eq8_e1066_d_n4: f64 = (p.p87 * eq8_e1065_d_n4);
        let eq8_e1066_d_n5: f64 = (p.p87 * eq8_e1065_d_n5);
        let eq8_e1066_d_n6: f64 = (p.p87 * eq8_e1065_d_n6);
        let eq8_e1066_d_n7: f64 = (p.p87 * eq8_e1065_d_n7);
        let eq8_e1066_d_n8: f64 = (p.p87 * eq8_e1065_d_n8);
        let eq8_e1066_d_n9: f64 = (p.p87 * eq8_e1065_d_n9);
        let eq8_e1066_d_n10: f64 = (p.p87 * eq8_e1065_d_n10);
        let eq8_e1066_d_n11: f64 = (p.p87 * eq8_e1065_d_n11);
        let eq8_e1066_d_n12: f64 = (p.p87 * eq8_e1065_d_n12);
        let eq8_e1066_d_n13: f64 = (p.p87 * eq8_e1065_d_n13);
        let eq8_e1066_d_n14: f64 = (p.p87 * eq8_e1065_d_n14);
        let eq8_e1066_d_n15: f64 = (p.p87 * eq8_e1065_d_n15);
        let eq8_e1066_d_n16: f64 = (p.p87 * eq8_e1065_d_n16);
        let eq8_e1066_d_n17: f64 = (p.p87 * eq8_e1065_d_n17);
        let eq8_e1066_d_b0: f64 = (p.p87 * eq8_e1065_d_b0);
        let eq8_e1066_d_b1: f64 = (p.p87 * eq8_e1065_d_b1);
        let eq8_e1066_d_b2: f64 = (p.p87 * eq8_e1065_d_b2);
        let eq8_e1066_d_b3: f64 = (p.p87 * eq8_e1065_d_b3);
        let eq8_e1066_d_b4: f64 = (p.p87 * eq8_e1065_d_b4);
        let eq8_e1066_d_b5: f64 = (p.p87 * eq8_e1065_d_b5);
        let eq8_e1066_d_b6: f64 = (p.p87 * eq8_e1065_d_b6);
        let eq8_e1066_d_b7: f64 = (p.p87 * eq8_e1065_d_b7);
        let eq8_e1066_d_b8: f64 = (p.p87 * eq8_e1065_d_b8);
        let eq8_e1066_d_b9: f64 = (p.p87 * eq8_e1065_d_b9);
        let eq8_e1066_d_b10: f64 = (p.p87 * eq8_e1065_d_b10);
        let eq8_e1066_d_b11: f64 = (p.p87 * eq8_e1065_d_b11);
        let eq8_value: f64 = eq8_e1066;
        let eq8_node_derivatives: [f64; 18] = [eq8_e1066_d_n0, eq8_e1066_d_n1, eq8_e1066_d_n2, eq8_e1066_d_n3, eq8_e1066_d_n4, eq8_e1066_d_n5, eq8_e1066_d_n6, eq8_e1066_d_n7, eq8_e1066_d_n8, eq8_e1066_d_n9, eq8_e1066_d_n10, eq8_e1066_d_n11, eq8_e1066_d_n12, eq8_e1066_d_n13, eq8_e1066_d_n14, eq8_e1066_d_n15, eq8_e1066_d_n16, eq8_e1066_d_n17];
        let eq8_branch_derivatives: [f64; 12] = [eq8_e1066_d_b0, eq8_e1066_d_b1, eq8_e1066_d_b2, eq8_e1066_d_b3, eq8_e1066_d_b4, eq8_e1066_d_b5, eq8_e1066_d_b6, eq8_e1066_d_b7, eq8_e1066_d_b8, eq8_e1066_d_b9, eq8_e1066_d_b10, eq8_e1066_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1070: f64 = (s.v[204] + s.v[736]);
        let eq9_e1070_d_n0: f64 = (s.dn[204][0] + s.dn[736][0]);
        let eq9_e1070_d_n1: f64 = (s.dn[204][1] + s.dn[736][1]);
        let eq9_e1070_d_n2: f64 = (s.dn[204][2] + s.dn[736][2]);
        let eq9_e1070_d_n3: f64 = (s.dn[204][3] + s.dn[736][3]);
        let eq9_e1070_d_n4: f64 = (s.dn[204][4] + s.dn[736][4]);
        let eq9_e1070_d_n5: f64 = (s.dn[204][5] + s.dn[736][5]);
        let eq9_e1070_d_n6: f64 = (s.dn[204][6] + s.dn[736][6]);
        let eq9_e1070_d_n7: f64 = (s.dn[204][7] + s.dn[736][7]);
        let eq9_e1070_d_n8: f64 = (s.dn[204][8] + s.dn[736][8]);
        let eq9_e1070_d_n9: f64 = (s.dn[204][9] + s.dn[736][9]);
        let eq9_e1070_d_n10: f64 = (s.dn[204][10] + s.dn[736][10]);
        let eq9_e1070_d_n11: f64 = (s.dn[204][11] + s.dn[736][11]);
        let eq9_e1070_d_n12: f64 = (s.dn[204][12] + s.dn[736][12]);
        let eq9_e1070_d_n13: f64 = (s.dn[204][13] + s.dn[736][13]);
        let eq9_e1070_d_n14: f64 = (s.dn[204][14] + s.dn[736][14]);
        let eq9_e1070_d_n15: f64 = (s.dn[204][15] + s.dn[736][15]);
        let eq9_e1070_d_n16: f64 = (s.dn[204][16] + s.dn[736][16]);
        let eq9_e1070_d_n17: f64 = (s.dn[204][17] + s.dn[736][17]);
        let eq9_e1070_d_b0: f64 = (s.db[204][0] + s.db[736][0]);
        let eq9_e1070_d_b1: f64 = (s.db[204][1] + s.db[736][1]);
        let eq9_e1070_d_b2: f64 = (s.db[204][2] + s.db[736][2]);
        let eq9_e1070_d_b3: f64 = (s.db[204][3] + s.db[736][3]);
        let eq9_e1070_d_b4: f64 = (s.db[204][4] + s.db[736][4]);
        let eq9_e1070_d_b5: f64 = (s.db[204][5] + s.db[736][5]);
        let eq9_e1070_d_b6: f64 = (s.db[204][6] + s.db[736][6]);
        let eq9_e1070_d_b7: f64 = (s.db[204][7] + s.db[736][7]);
        let eq9_e1070_d_b8: f64 = (s.db[204][8] + s.db[736][8]);
        let eq9_e1070_d_b9: f64 = (s.db[204][9] + s.db[736][9]);
        let eq9_e1070_d_b10: f64 = (s.db[204][10] + s.db[736][10]);
        let eq9_e1070_d_b11: f64 = (s.db[204][11] + s.db[736][11]);
        let eq9_e1072: f64 = (eq9_e1070 + s.v[432]);
        let eq9_e1072_d_n0: f64 = (eq9_e1070_d_n0 + s.dn[432][0]);
        let eq9_e1072_d_n1: f64 = (eq9_e1070_d_n1 + s.dn[432][1]);
        let eq9_e1072_d_n2: f64 = (eq9_e1070_d_n2 + s.dn[432][2]);
        let eq9_e1072_d_n3: f64 = (eq9_e1070_d_n3 + s.dn[432][3]);
        let eq9_e1072_d_n4: f64 = (eq9_e1070_d_n4 + s.dn[432][4]);
        let eq9_e1072_d_n5: f64 = (eq9_e1070_d_n5 + s.dn[432][5]);
        let eq9_e1072_d_n6: f64 = (eq9_e1070_d_n6 + s.dn[432][6]);
        let eq9_e1072_d_n7: f64 = (eq9_e1070_d_n7 + s.dn[432][7]);
        let eq9_e1072_d_n8: f64 = (eq9_e1070_d_n8 + s.dn[432][8]);
        let eq9_e1072_d_n9: f64 = (eq9_e1070_d_n9 + s.dn[432][9]);
        let eq9_e1072_d_n10: f64 = (eq9_e1070_d_n10 + s.dn[432][10]);
        let eq9_e1072_d_n11: f64 = (eq9_e1070_d_n11 + s.dn[432][11]);
        let eq9_e1072_d_n12: f64 = (eq9_e1070_d_n12 + s.dn[432][12]);
        let eq9_e1072_d_n13: f64 = (eq9_e1070_d_n13 + s.dn[432][13]);
        let eq9_e1072_d_n14: f64 = (eq9_e1070_d_n14 + s.dn[432][14]);
        let eq9_e1072_d_n15: f64 = (eq9_e1070_d_n15 + s.dn[432][15]);
        let eq9_e1072_d_n16: f64 = (eq9_e1070_d_n16 + s.dn[432][16]);
        let eq9_e1072_d_n17: f64 = (eq9_e1070_d_n17 + s.dn[432][17]);
        let eq9_e1072_d_b0: f64 = (eq9_e1070_d_b0 + s.db[432][0]);
        let eq9_e1072_d_b1: f64 = (eq9_e1070_d_b1 + s.db[432][1]);
        let eq9_e1072_d_b2: f64 = (eq9_e1070_d_b2 + s.db[432][2]);
        let eq9_e1072_d_b3: f64 = (eq9_e1070_d_b3 + s.db[432][3]);
        let eq9_e1072_d_b4: f64 = (eq9_e1070_d_b4 + s.db[432][4]);
        let eq9_e1072_d_b5: f64 = (eq9_e1070_d_b5 + s.db[432][5]);
        let eq9_e1072_d_b6: f64 = (eq9_e1070_d_b6 + s.db[432][6]);
        let eq9_e1072_d_b7: f64 = (eq9_e1070_d_b7 + s.db[432][7]);
        let eq9_e1072_d_b8: f64 = (eq9_e1070_d_b8 + s.db[432][8]);
        let eq9_e1072_d_b9: f64 = (eq9_e1070_d_b9 + s.db[432][9]);
        let eq9_e1072_d_b10: f64 = (eq9_e1070_d_b10 + s.db[432][10]);
        let eq9_e1072_d_b11: f64 = (eq9_e1070_d_b11 + s.db[432][11]);
        let eq9_e1073: f64 = (p.p87 * eq9_e1072);
        let eq9_e1073_d_n0: f64 = (p.p87 * eq9_e1072_d_n0);
        let eq9_e1073_d_n1: f64 = (p.p87 * eq9_e1072_d_n1);
        let eq9_e1073_d_n2: f64 = (p.p87 * eq9_e1072_d_n2);
        let eq9_e1073_d_n3: f64 = (p.p87 * eq9_e1072_d_n3);
        let eq9_e1073_d_n4: f64 = (p.p87 * eq9_e1072_d_n4);
        let eq9_e1073_d_n5: f64 = (p.p87 * eq9_e1072_d_n5);
        let eq9_e1073_d_n6: f64 = (p.p87 * eq9_e1072_d_n6);
        let eq9_e1073_d_n7: f64 = (p.p87 * eq9_e1072_d_n7);
        let eq9_e1073_d_n8: f64 = (p.p87 * eq9_e1072_d_n8);
        let eq9_e1073_d_n9: f64 = (p.p87 * eq9_e1072_d_n9);
        let eq9_e1073_d_n10: f64 = (p.p87 * eq9_e1072_d_n10);
        let eq9_e1073_d_n11: f64 = (p.p87 * eq9_e1072_d_n11);
        let eq9_e1073_d_n12: f64 = (p.p87 * eq9_e1072_d_n12);
        let eq9_e1073_d_n13: f64 = (p.p87 * eq9_e1072_d_n13);
        let eq9_e1073_d_n14: f64 = (p.p87 * eq9_e1072_d_n14);
        let eq9_e1073_d_n15: f64 = (p.p87 * eq9_e1072_d_n15);
        let eq9_e1073_d_n16: f64 = (p.p87 * eq9_e1072_d_n16);
        let eq9_e1073_d_n17: f64 = (p.p87 * eq9_e1072_d_n17);
        let eq9_e1073_d_b0: f64 = (p.p87 * eq9_e1072_d_b0);
        let eq9_e1073_d_b1: f64 = (p.p87 * eq9_e1072_d_b1);
        let eq9_e1073_d_b2: f64 = (p.p87 * eq9_e1072_d_b2);
        let eq9_e1073_d_b3: f64 = (p.p87 * eq9_e1072_d_b3);
        let eq9_e1073_d_b4: f64 = (p.p87 * eq9_e1072_d_b4);
        let eq9_e1073_d_b5: f64 = (p.p87 * eq9_e1072_d_b5);
        let eq9_e1073_d_b6: f64 = (p.p87 * eq9_e1072_d_b6);
        let eq9_e1073_d_b7: f64 = (p.p87 * eq9_e1072_d_b7);
        let eq9_e1073_d_b8: f64 = (p.p87 * eq9_e1072_d_b8);
        let eq9_e1073_d_b9: f64 = (p.p87 * eq9_e1072_d_b9);
        let eq9_e1073_d_b10: f64 = (p.p87 * eq9_e1072_d_b10);
        let eq9_e1073_d_b11: f64 = (p.p87 * eq9_e1072_d_b11);
        let eq9_value: f64 = eq9_e1073;
        let eq9_node_derivatives: [f64; 18] = [eq9_e1073_d_n0, eq9_e1073_d_n1, eq9_e1073_d_n2, eq9_e1073_d_n3, eq9_e1073_d_n4, eq9_e1073_d_n5, eq9_e1073_d_n6, eq9_e1073_d_n7, eq9_e1073_d_n8, eq9_e1073_d_n9, eq9_e1073_d_n10, eq9_e1073_d_n11, eq9_e1073_d_n12, eq9_e1073_d_n13, eq9_e1073_d_n14, eq9_e1073_d_n15, eq9_e1073_d_n16, eq9_e1073_d_n17];
        let eq9_branch_derivatives: [f64; 12] = [eq9_e1073_d_b0, eq9_e1073_d_b1, eq9_e1073_d_b2, eq9_e1073_d_b3, eq9_e1073_d_b4, eq9_e1073_d_b5, eq9_e1073_d_b6, eq9_e1073_d_b7, eq9_e1073_d_b8, eq9_e1073_d_b9, eq9_e1073_d_b10, eq9_e1073_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1076: f64 = (p.p87 * s.v[281]);
        let eq10_e1076_d_n0: f64 = (p.p87 * s.dn[281][0]);
        let eq10_e1076_d_n1: f64 = (p.p87 * s.dn[281][1]);
        let eq10_e1076_d_n2: f64 = (p.p87 * s.dn[281][2]);
        let eq10_e1076_d_n3: f64 = (p.p87 * s.dn[281][3]);
        let eq10_e1076_d_n4: f64 = (p.p87 * s.dn[281][4]);
        let eq10_e1076_d_n5: f64 = (p.p87 * s.dn[281][5]);
        let eq10_e1076_d_n6: f64 = (p.p87 * s.dn[281][6]);
        let eq10_e1076_d_n7: f64 = (p.p87 * s.dn[281][7]);
        let eq10_e1076_d_n8: f64 = (p.p87 * s.dn[281][8]);
        let eq10_e1076_d_n9: f64 = (p.p87 * s.dn[281][9]);
        let eq10_e1076_d_n10: f64 = (p.p87 * s.dn[281][10]);
        let eq10_e1076_d_n11: f64 = (p.p87 * s.dn[281][11]);
        let eq10_e1076_d_n12: f64 = (p.p87 * s.dn[281][12]);
        let eq10_e1076_d_n13: f64 = (p.p87 * s.dn[281][13]);
        let eq10_e1076_d_n14: f64 = (p.p87 * s.dn[281][14]);
        let eq10_e1076_d_n15: f64 = (p.p87 * s.dn[281][15]);
        let eq10_e1076_d_n16: f64 = (p.p87 * s.dn[281][16]);
        let eq10_e1076_d_n17: f64 = (p.p87 * s.dn[281][17]);
        let eq10_e1076_d_b0: f64 = (p.p87 * s.db[281][0]);
        let eq10_e1076_d_b1: f64 = (p.p87 * s.db[281][1]);
        let eq10_e1076_d_b2: f64 = (p.p87 * s.db[281][2]);
        let eq10_e1076_d_b3: f64 = (p.p87 * s.db[281][3]);
        let eq10_e1076_d_b4: f64 = (p.p87 * s.db[281][4]);
        let eq10_e1076_d_b5: f64 = (p.p87 * s.db[281][5]);
        let eq10_e1076_d_b6: f64 = (p.p87 * s.db[281][6]);
        let eq10_e1076_d_b7: f64 = (p.p87 * s.db[281][7]);
        let eq10_e1076_d_b8: f64 = (p.p87 * s.db[281][8]);
        let eq10_e1076_d_b9: f64 = (p.p87 * s.db[281][9]);
        let eq10_e1076_d_b10: f64 = (p.p87 * s.db[281][10]);
        let eq10_e1076_d_b11: f64 = (p.p87 * s.db[281][11]);
        let eq10_value: f64 = eq10_e1076;
        let eq10_node_derivatives: [f64; 18] = [eq10_e1076_d_n0, eq10_e1076_d_n1, eq10_e1076_d_n2, eq10_e1076_d_n3, eq10_e1076_d_n4, eq10_e1076_d_n5, eq10_e1076_d_n6, eq10_e1076_d_n7, eq10_e1076_d_n8, eq10_e1076_d_n9, eq10_e1076_d_n10, eq10_e1076_d_n11, eq10_e1076_d_n12, eq10_e1076_d_n13, eq10_e1076_d_n14, eq10_e1076_d_n15, eq10_e1076_d_n16, eq10_e1076_d_n17];
        let eq10_branch_derivatives: [f64; 12] = [eq10_e1076_d_b0, eq10_e1076_d_b1, eq10_e1076_d_b2, eq10_e1076_d_b3, eq10_e1076_d_b4, eq10_e1076_d_b5, eq10_e1076_d_b6, eq10_e1076_d_b7, eq10_e1076_d_b8, eq10_e1076_d_b9, eq10_e1076_d_b10, eq10_e1076_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1079: f64 = (p.p87 * s.v[737]);
        let eq11_e1079_d_n0: f64 = (p.p87 * s.dn[737][0]);
        let eq11_e1079_d_n1: f64 = (p.p87 * s.dn[737][1]);
        let eq11_e1079_d_n2: f64 = (p.p87 * s.dn[737][2]);
        let eq11_e1079_d_n3: f64 = (p.p87 * s.dn[737][3]);
        let eq11_e1079_d_n4: f64 = (p.p87 * s.dn[737][4]);
        let eq11_e1079_d_n5: f64 = (p.p87 * s.dn[737][5]);
        let eq11_e1079_d_n6: f64 = (p.p87 * s.dn[737][6]);
        let eq11_e1079_d_n7: f64 = (p.p87 * s.dn[737][7]);
        let eq11_e1079_d_n8: f64 = (p.p87 * s.dn[737][8]);
        let eq11_e1079_d_n9: f64 = (p.p87 * s.dn[737][9]);
        let eq11_e1079_d_n10: f64 = (p.p87 * s.dn[737][10]);
        let eq11_e1079_d_n11: f64 = (p.p87 * s.dn[737][11]);
        let eq11_e1079_d_n12: f64 = (p.p87 * s.dn[737][12]);
        let eq11_e1079_d_n13: f64 = (p.p87 * s.dn[737][13]);
        let eq11_e1079_d_n14: f64 = (p.p87 * s.dn[737][14]);
        let eq11_e1079_d_n15: f64 = (p.p87 * s.dn[737][15]);
        let eq11_e1079_d_n16: f64 = (p.p87 * s.dn[737][16]);
        let eq11_e1079_d_n17: f64 = (p.p87 * s.dn[737][17]);
        let eq11_e1079_d_b0: f64 = (p.p87 * s.db[737][0]);
        let eq11_e1079_d_b1: f64 = (p.p87 * s.db[737][1]);
        let eq11_e1079_d_b2: f64 = (p.p87 * s.db[737][2]);
        let eq11_e1079_d_b3: f64 = (p.p87 * s.db[737][3]);
        let eq11_e1079_d_b4: f64 = (p.p87 * s.db[737][4]);
        let eq11_e1079_d_b5: f64 = (p.p87 * s.db[737][5]);
        let eq11_e1079_d_b6: f64 = (p.p87 * s.db[737][6]);
        let eq11_e1079_d_b7: f64 = (p.p87 * s.db[737][7]);
        let eq11_e1079_d_b8: f64 = (p.p87 * s.db[737][8]);
        let eq11_e1079_d_b9: f64 = (p.p87 * s.db[737][9]);
        let eq11_e1079_d_b10: f64 = (p.p87 * s.db[737][10]);
        let eq11_e1079_d_b11: f64 = (p.p87 * s.db[737][11]);
        let eq11_value: f64 = eq11_e1079;
        let eq11_node_derivatives: [f64; 18] = [eq11_e1079_d_n0, eq11_e1079_d_n1, eq11_e1079_d_n2, eq11_e1079_d_n3, eq11_e1079_d_n4, eq11_e1079_d_n5, eq11_e1079_d_n6, eq11_e1079_d_n7, eq11_e1079_d_n8, eq11_e1079_d_n9, eq11_e1079_d_n10, eq11_e1079_d_n11, eq11_e1079_d_n12, eq11_e1079_d_n13, eq11_e1079_d_n14, eq11_e1079_d_n15, eq11_e1079_d_n16, eq11_e1079_d_n17];
        let eq11_branch_derivatives: [f64; 12] = [eq11_e1079_d_b0, eq11_e1079_d_b1, eq11_e1079_d_b2, eq11_e1079_d_b3, eq11_e1079_d_b4, eq11_e1079_d_b5, eq11_e1079_d_b6, eq11_e1079_d_b7, eq11_e1079_d_b8, eq11_e1079_d_b9, eq11_e1079_d_b10, eq11_e1079_d_b11];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1082: f64 = (p.p87 * s.v[862]);
        let eq12_e1082_d_n0: f64 = (p.p87 * s.dn[862][0]);
        let eq12_e1082_d_n1: f64 = (p.p87 * s.dn[862][1]);
        let eq12_e1082_d_n2: f64 = (p.p87 * s.dn[862][2]);
        let eq12_e1082_d_n3: f64 = (p.p87 * s.dn[862][3]);
        let eq12_e1082_d_n4: f64 = (p.p87 * s.dn[862][4]);
        let eq12_e1082_d_n5: f64 = (p.p87 * s.dn[862][5]);
        let eq12_e1082_d_n6: f64 = (p.p87 * s.dn[862][6]);
        let eq12_e1082_d_n7: f64 = (p.p87 * s.dn[862][7]);
        let eq12_e1082_d_n8: f64 = (p.p87 * s.dn[862][8]);
        let eq12_e1082_d_n9: f64 = (p.p87 * s.dn[862][9]);
        let eq12_e1082_d_n10: f64 = (p.p87 * s.dn[862][10]);
        let eq12_e1082_d_n11: f64 = (p.p87 * s.dn[862][11]);
        let eq12_e1082_d_n12: f64 = (p.p87 * s.dn[862][12]);
        let eq12_e1082_d_n13: f64 = (p.p87 * s.dn[862][13]);
        let eq12_e1082_d_n14: f64 = (p.p87 * s.dn[862][14]);
        let eq12_e1082_d_n15: f64 = (p.p87 * s.dn[862][15]);
        let eq12_e1082_d_n16: f64 = (p.p87 * s.dn[862][16]);
        let eq12_e1082_d_n17: f64 = (p.p87 * s.dn[862][17]);
        let eq12_e1082_d_b0: f64 = (p.p87 * s.db[862][0]);
        let eq12_e1082_d_b1: f64 = (p.p87 * s.db[862][1]);
        let eq12_e1082_d_b2: f64 = (p.p87 * s.db[862][2]);
        let eq12_e1082_d_b3: f64 = (p.p87 * s.db[862][3]);
        let eq12_e1082_d_b4: f64 = (p.p87 * s.db[862][4]);
        let eq12_e1082_d_b5: f64 = (p.p87 * s.db[862][5]);
        let eq12_e1082_d_b6: f64 = (p.p87 * s.db[862][6]);
        let eq12_e1082_d_b7: f64 = (p.p87 * s.db[862][7]);
        let eq12_e1082_d_b8: f64 = (p.p87 * s.db[862][8]);
        let eq12_e1082_d_b9: f64 = (p.p87 * s.db[862][9]);
        let eq12_e1082_d_b10: f64 = (p.p87 * s.db[862][10]);
        let eq12_e1082_d_b11: f64 = (p.p87 * s.db[862][11]);
        let eq12_value: f64 = eq12_e1082;
        let eq12_node_derivatives: [f64; 18] = [eq12_e1082_d_n0, eq12_e1082_d_n1, eq12_e1082_d_n2, eq12_e1082_d_n3, eq12_e1082_d_n4, eq12_e1082_d_n5, eq12_e1082_d_n6, eq12_e1082_d_n7, eq12_e1082_d_n8, eq12_e1082_d_n9, eq12_e1082_d_n10, eq12_e1082_d_n11, eq12_e1082_d_n12, eq12_e1082_d_n13, eq12_e1082_d_n14, eq12_e1082_d_n15, eq12_e1082_d_n16, eq12_e1082_d_n17];
        let eq12_branch_derivatives: [f64; 12] = [eq12_e1082_d_b0, eq12_e1082_d_b1, eq12_e1082_d_b2, eq12_e1082_d_b3, eq12_e1082_d_b4, eq12_e1082_d_b5, eq12_e1082_d_b6, eq12_e1082_d_b7, eq12_e1082_d_b8, eq12_e1082_d_b9, eq12_e1082_d_b10, eq12_e1082_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1085: f64 = (p.p87 * s.v[861]);
        let eq13_e1085_d_n0: f64 = (p.p87 * s.dn[861][0]);
        let eq13_e1085_d_n1: f64 = (p.p87 * s.dn[861][1]);
        let eq13_e1085_d_n2: f64 = (p.p87 * s.dn[861][2]);
        let eq13_e1085_d_n3: f64 = (p.p87 * s.dn[861][3]);
        let eq13_e1085_d_n4: f64 = (p.p87 * s.dn[861][4]);
        let eq13_e1085_d_n5: f64 = (p.p87 * s.dn[861][5]);
        let eq13_e1085_d_n6: f64 = (p.p87 * s.dn[861][6]);
        let eq13_e1085_d_n7: f64 = (p.p87 * s.dn[861][7]);
        let eq13_e1085_d_n8: f64 = (p.p87 * s.dn[861][8]);
        let eq13_e1085_d_n9: f64 = (p.p87 * s.dn[861][9]);
        let eq13_e1085_d_n10: f64 = (p.p87 * s.dn[861][10]);
        let eq13_e1085_d_n11: f64 = (p.p87 * s.dn[861][11]);
        let eq13_e1085_d_n12: f64 = (p.p87 * s.dn[861][12]);
        let eq13_e1085_d_n13: f64 = (p.p87 * s.dn[861][13]);
        let eq13_e1085_d_n14: f64 = (p.p87 * s.dn[861][14]);
        let eq13_e1085_d_n15: f64 = (p.p87 * s.dn[861][15]);
        let eq13_e1085_d_n16: f64 = (p.p87 * s.dn[861][16]);
        let eq13_e1085_d_n17: f64 = (p.p87 * s.dn[861][17]);
        let eq13_e1085_d_b0: f64 = (p.p87 * s.db[861][0]);
        let eq13_e1085_d_b1: f64 = (p.p87 * s.db[861][1]);
        let eq13_e1085_d_b2: f64 = (p.p87 * s.db[861][2]);
        let eq13_e1085_d_b3: f64 = (p.p87 * s.db[861][3]);
        let eq13_e1085_d_b4: f64 = (p.p87 * s.db[861][4]);
        let eq13_e1085_d_b5: f64 = (p.p87 * s.db[861][5]);
        let eq13_e1085_d_b6: f64 = (p.p87 * s.db[861][6]);
        let eq13_e1085_d_b7: f64 = (p.p87 * s.db[861][7]);
        let eq13_e1085_d_b8: f64 = (p.p87 * s.db[861][8]);
        let eq13_e1085_d_b9: f64 = (p.p87 * s.db[861][9]);
        let eq13_e1085_d_b10: f64 = (p.p87 * s.db[861][10]);
        let eq13_e1085_d_b11: f64 = (p.p87 * s.db[861][11]);
        let eq13_value: f64 = eq13_e1085;
        let eq13_node_derivatives: [f64; 18] = [eq13_e1085_d_n0, eq13_e1085_d_n1, eq13_e1085_d_n2, eq13_e1085_d_n3, eq13_e1085_d_n4, eq13_e1085_d_n5, eq13_e1085_d_n6, eq13_e1085_d_n7, eq13_e1085_d_n8, eq13_e1085_d_n9, eq13_e1085_d_n10, eq13_e1085_d_n11, eq13_e1085_d_n12, eq13_e1085_d_n13, eq13_e1085_d_n14, eq13_e1085_d_n15, eq13_e1085_d_n16, eq13_e1085_d_n17];
        let eq13_branch_derivatives: [f64; 12] = [eq13_e1085_d_b0, eq13_e1085_d_b1, eq13_e1085_d_b2, eq13_e1085_d_b3, eq13_e1085_d_b4, eq13_e1085_d_b5, eq13_e1085_d_b6, eq13_e1085_d_b7, eq13_e1085_d_b8, eq13_e1085_d_b9, eq13_e1085_d_b10, eq13_e1085_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1088: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[66]);
        let eq14_e1088_d_n0: f64 = (s.dn[66][0] * ddt_scale);
        let eq14_e1088_d_n1: f64 = (s.dn[66][1] * ddt_scale);
        let eq14_e1088_d_n2: f64 = (s.dn[66][2] * ddt_scale);
        let eq14_e1088_d_n3: f64 = (s.dn[66][3] * ddt_scale);
        let eq14_e1088_d_n4: f64 = (s.dn[66][4] * ddt_scale);
        let eq14_e1088_d_n5: f64 = (s.dn[66][5] * ddt_scale);
        let eq14_e1088_d_n6: f64 = (s.dn[66][6] * ddt_scale);
        let eq14_e1088_d_n7: f64 = (s.dn[66][7] * ddt_scale);
        let eq14_e1088_d_n8: f64 = (s.dn[66][8] * ddt_scale);
        let eq14_e1088_d_n9: f64 = (s.dn[66][9] * ddt_scale);
        let eq14_e1088_d_n10: f64 = (s.dn[66][10] * ddt_scale);
        let eq14_e1088_d_n11: f64 = (s.dn[66][11] * ddt_scale);
        let eq14_e1088_d_n12: f64 = (s.dn[66][12] * ddt_scale);
        let eq14_e1088_d_n13: f64 = (s.dn[66][13] * ddt_scale);
        let eq14_e1088_d_n14: f64 = (s.dn[66][14] * ddt_scale);
        let eq14_e1088_d_n15: f64 = (s.dn[66][15] * ddt_scale);
        let eq14_e1088_d_n16: f64 = (s.dn[66][16] * ddt_scale);
        let eq14_e1088_d_n17: f64 = (s.dn[66][17] * ddt_scale);
        let eq14_e1088_d_b0: f64 = (s.db[66][0] * ddt_scale);
        let eq14_e1088_d_b1: f64 = (s.db[66][1] * ddt_scale);
        let eq14_e1088_d_b2: f64 = (s.db[66][2] * ddt_scale);
        let eq14_e1088_d_b3: f64 = (s.db[66][3] * ddt_scale);
        let eq14_e1088_d_b4: f64 = (s.db[66][4] * ddt_scale);
        let eq14_e1088_d_b5: f64 = (s.db[66][5] * ddt_scale);
        let eq14_e1088_d_b6: f64 = (s.db[66][6] * ddt_scale);
        let eq14_e1088_d_b7: f64 = (s.db[66][7] * ddt_scale);
        let eq14_e1088_d_b8: f64 = (s.db[66][8] * ddt_scale);
        let eq14_e1088_d_b9: f64 = (s.db[66][9] * ddt_scale);
        let eq14_e1088_d_b10: f64 = (s.db[66][10] * ddt_scale);
        let eq14_e1088_d_b11: f64 = (s.db[66][11] * ddt_scale);
        let eq14_e1089: f64 = (p.p87 * eq14_e1088);
        let eq14_e1089_d_n0: f64 = (p.p87 * eq14_e1088_d_n0);
        let eq14_e1089_d_n1: f64 = (p.p87 * eq14_e1088_d_n1);
        let eq14_e1089_d_n2: f64 = (p.p87 * eq14_e1088_d_n2);
        let eq14_e1089_d_n3: f64 = (p.p87 * eq14_e1088_d_n3);
        let eq14_e1089_d_n4: f64 = (p.p87 * eq14_e1088_d_n4);
        let eq14_e1089_d_n5: f64 = (p.p87 * eq14_e1088_d_n5);
        let eq14_e1089_d_n6: f64 = (p.p87 * eq14_e1088_d_n6);
        let eq14_e1089_d_n7: f64 = (p.p87 * eq14_e1088_d_n7);
        let eq14_e1089_d_n8: f64 = (p.p87 * eq14_e1088_d_n8);
        let eq14_e1089_d_n9: f64 = (p.p87 * eq14_e1088_d_n9);
        let eq14_e1089_d_n10: f64 = (p.p87 * eq14_e1088_d_n10);
        let eq14_e1089_d_n11: f64 = (p.p87 * eq14_e1088_d_n11);
        let eq14_e1089_d_n12: f64 = (p.p87 * eq14_e1088_d_n12);
        let eq14_e1089_d_n13: f64 = (p.p87 * eq14_e1088_d_n13);
        let eq14_e1089_d_n14: f64 = (p.p87 * eq14_e1088_d_n14);
        let eq14_e1089_d_n15: f64 = (p.p87 * eq14_e1088_d_n15);
        let eq14_e1089_d_n16: f64 = (p.p87 * eq14_e1088_d_n16);
        let eq14_e1089_d_n17: f64 = (p.p87 * eq14_e1088_d_n17);
        let eq14_e1089_d_b0: f64 = (p.p87 * eq14_e1088_d_b0);
        let eq14_e1089_d_b1: f64 = (p.p87 * eq14_e1088_d_b1);
        let eq14_e1089_d_b2: f64 = (p.p87 * eq14_e1088_d_b2);
        let eq14_e1089_d_b3: f64 = (p.p87 * eq14_e1088_d_b3);
        let eq14_e1089_d_b4: f64 = (p.p87 * eq14_e1088_d_b4);
        let eq14_e1089_d_b5: f64 = (p.p87 * eq14_e1088_d_b5);
        let eq14_e1089_d_b6: f64 = (p.p87 * eq14_e1088_d_b6);
        let eq14_e1089_d_b7: f64 = (p.p87 * eq14_e1088_d_b7);
        let eq14_e1089_d_b8: f64 = (p.p87 * eq14_e1088_d_b8);
        let eq14_e1089_d_b9: f64 = (p.p87 * eq14_e1088_d_b9);
        let eq14_e1089_d_b10: f64 = (p.p87 * eq14_e1088_d_b10);
        let eq14_e1089_d_b11: f64 = (p.p87 * eq14_e1088_d_b11);
        let eq14_value: f64 = eq14_e1089;
        let eq14_node_derivatives: [f64; 18] = [eq14_e1089_d_n0, eq14_e1089_d_n1, eq14_e1089_d_n2, eq14_e1089_d_n3, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, eq14_e1089_d_n11, eq14_e1089_d_n12, eq14_e1089_d_n13, eq14_e1089_d_n14, eq14_e1089_d_n15, eq14_e1089_d_n16, eq14_e1089_d_n17];
        let eq14_branch_derivatives: [f64; 12] = [eq14_e1089_d_b0, eq14_e1089_d_b1, eq14_e1089_d_b2, eq14_e1089_d_b3, eq14_e1089_d_b4, eq14_e1089_d_b5, eq14_e1089_d_b6, eq14_e1089_d_b7, eq14_e1089_d_b8, eq14_e1089_d_b9, eq14_e1089_d_b10, eq14_e1089_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq15_e1092: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[65]);
        let eq15_e1092_d_n0: f64 = (s.dn[65][0] * ddt_scale);
        let eq15_e1092_d_n1: f64 = (s.dn[65][1] * ddt_scale);
        let eq15_e1092_d_n2: f64 = (s.dn[65][2] * ddt_scale);
        let eq15_e1092_d_n3: f64 = (s.dn[65][3] * ddt_scale);
        let eq15_e1092_d_n4: f64 = (s.dn[65][4] * ddt_scale);
        let eq15_e1092_d_n5: f64 = (s.dn[65][5] * ddt_scale);
        let eq15_e1092_d_n6: f64 = (s.dn[65][6] * ddt_scale);
        let eq15_e1092_d_n7: f64 = (s.dn[65][7] * ddt_scale);
        let eq15_e1092_d_n8: f64 = (s.dn[65][8] * ddt_scale);
        let eq15_e1092_d_n9: f64 = (s.dn[65][9] * ddt_scale);
        let eq15_e1092_d_n10: f64 = (s.dn[65][10] * ddt_scale);
        let eq15_e1092_d_n11: f64 = (s.dn[65][11] * ddt_scale);
        let eq15_e1092_d_n12: f64 = (s.dn[65][12] * ddt_scale);
        let eq15_e1092_d_n13: f64 = (s.dn[65][13] * ddt_scale);
        let eq15_e1092_d_n14: f64 = (s.dn[65][14] * ddt_scale);
        let eq15_e1092_d_n15: f64 = (s.dn[65][15] * ddt_scale);
        let eq15_e1092_d_n16: f64 = (s.dn[65][16] * ddt_scale);
        let eq15_e1092_d_n17: f64 = (s.dn[65][17] * ddt_scale);
        let eq15_e1092_d_b0: f64 = (s.db[65][0] * ddt_scale);
        let eq15_e1092_d_b1: f64 = (s.db[65][1] * ddt_scale);
        let eq15_e1092_d_b2: f64 = (s.db[65][2] * ddt_scale);
        let eq15_e1092_d_b3: f64 = (s.db[65][3] * ddt_scale);
        let eq15_e1092_d_b4: f64 = (s.db[65][4] * ddt_scale);
        let eq15_e1092_d_b5: f64 = (s.db[65][5] * ddt_scale);
        let eq15_e1092_d_b6: f64 = (s.db[65][6] * ddt_scale);
        let eq15_e1092_d_b7: f64 = (s.db[65][7] * ddt_scale);
        let eq15_e1092_d_b8: f64 = (s.db[65][8] * ddt_scale);
        let eq15_e1092_d_b9: f64 = (s.db[65][9] * ddt_scale);
        let eq15_e1092_d_b10: f64 = (s.db[65][10] * ddt_scale);
        let eq15_e1092_d_b11: f64 = (s.db[65][11] * ddt_scale);
        let eq15_e1093: f64 = (p.p87 * eq15_e1092);
        let eq15_e1093_d_n0: f64 = (p.p87 * eq15_e1092_d_n0);
        let eq15_e1093_d_n1: f64 = (p.p87 * eq15_e1092_d_n1);
        let eq15_e1093_d_n2: f64 = (p.p87 * eq15_e1092_d_n2);
        let eq15_e1093_d_n3: f64 = (p.p87 * eq15_e1092_d_n3);
        let eq15_e1093_d_n4: f64 = (p.p87 * eq15_e1092_d_n4);
        let eq15_e1093_d_n5: f64 = (p.p87 * eq15_e1092_d_n5);
        let eq15_e1093_d_n6: f64 = (p.p87 * eq15_e1092_d_n6);
        let eq15_e1093_d_n7: f64 = (p.p87 * eq15_e1092_d_n7);
        let eq15_e1093_d_n8: f64 = (p.p87 * eq15_e1092_d_n8);
        let eq15_e1093_d_n9: f64 = (p.p87 * eq15_e1092_d_n9);
        let eq15_e1093_d_n10: f64 = (p.p87 * eq15_e1092_d_n10);
        let eq15_e1093_d_n11: f64 = (p.p87 * eq15_e1092_d_n11);
        let eq15_e1093_d_n12: f64 = (p.p87 * eq15_e1092_d_n12);
        let eq15_e1093_d_n13: f64 = (p.p87 * eq15_e1092_d_n13);
        let eq15_e1093_d_n14: f64 = (p.p87 * eq15_e1092_d_n14);
        let eq15_e1093_d_n15: f64 = (p.p87 * eq15_e1092_d_n15);
        let eq15_e1093_d_n16: f64 = (p.p87 * eq15_e1092_d_n16);
        let eq15_e1093_d_n17: f64 = (p.p87 * eq15_e1092_d_n17);
        let eq15_e1093_d_b0: f64 = (p.p87 * eq15_e1092_d_b0);
        let eq15_e1093_d_b1: f64 = (p.p87 * eq15_e1092_d_b1);
        let eq15_e1093_d_b2: f64 = (p.p87 * eq15_e1092_d_b2);
        let eq15_e1093_d_b3: f64 = (p.p87 * eq15_e1092_d_b3);
        let eq15_e1093_d_b4: f64 = (p.p87 * eq15_e1092_d_b4);
        let eq15_e1093_d_b5: f64 = (p.p87 * eq15_e1092_d_b5);
        let eq15_e1093_d_b6: f64 = (p.p87 * eq15_e1092_d_b6);
        let eq15_e1093_d_b7: f64 = (p.p87 * eq15_e1092_d_b7);
        let eq15_e1093_d_b8: f64 = (p.p87 * eq15_e1092_d_b8);
        let eq15_e1093_d_b9: f64 = (p.p87 * eq15_e1092_d_b9);
        let eq15_e1093_d_b10: f64 = (p.p87 * eq15_e1092_d_b10);
        let eq15_e1093_d_b11: f64 = (p.p87 * eq15_e1092_d_b11);
        let eq15_value: f64 = eq15_e1093;
        let eq15_node_derivatives: [f64; 18] = [eq15_e1093_d_n0, eq15_e1093_d_n1, eq15_e1093_d_n2, eq15_e1093_d_n3, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, eq15_e1093_d_n12, eq15_e1093_d_n13, eq15_e1093_d_n14, eq15_e1093_d_n15, eq15_e1093_d_n16, eq15_e1093_d_n17];
        let eq15_branch_derivatives: [f64; 12] = [eq15_e1093_d_b0, eq15_e1093_d_b1, eq15_e1093_d_b2, eq15_e1093_d_b3, eq15_e1093_d_b4, eq15_e1093_d_b5, eq15_e1093_d_b6, eq15_e1093_d_b7, eq15_e1093_d_b8, eq15_e1093_d_b9, eq15_e1093_d_b10, eq15_e1093_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e1099, eq16_e1099_d_n0, eq16_e1099_d_n1, eq16_e1099_d_n2, eq16_e1099_d_n3, eq16_e1099_d_n4, eq16_e1099_d_n5, eq16_e1099_d_n6, eq16_e1099_d_n7, eq16_e1099_d_n8, eq16_e1099_d_n9, eq16_e1099_d_n10, eq16_e1099_d_n11, eq16_e1099_d_n12, eq16_e1099_d_n13, eq16_e1099_d_n14, eq16_e1099_d_n15, eq16_e1099_d_n16, eq16_e1099_d_n17, eq16_e1099_d_b0, eq16_e1099_d_b1, eq16_e1099_d_b2, eq16_e1099_d_b3, eq16_e1099_d_b4, eq16_e1099_d_b5, eq16_e1099_d_b6, eq16_e1099_d_b7, eq16_e1099_d_b8, eq16_e1099_d_b9, eq16_e1099_d_b10, eq16_e1099_d_b11,) = {
    if s.b[3405] {
        let eq16_e1097: f64 = (p.p87 * s.v[870]);
        let eq16_e1097_d_n0: f64 = (p.p87 * s.dn[870][0]);
        let eq16_e1097_d_n1: f64 = (p.p87 * s.dn[870][1]);
        let eq16_e1097_d_n2: f64 = (p.p87 * s.dn[870][2]);
        let eq16_e1097_d_n3: f64 = (p.p87 * s.dn[870][3]);
        let eq16_e1097_d_n4: f64 = (p.p87 * s.dn[870][4]);
        let eq16_e1097_d_n5: f64 = (p.p87 * s.dn[870][5]);
        let eq16_e1097_d_n6: f64 = (p.p87 * s.dn[870][6]);
        let eq16_e1097_d_n7: f64 = (p.p87 * s.dn[870][7]);
        let eq16_e1097_d_n8: f64 = (p.p87 * s.dn[870][8]);
        let eq16_e1097_d_n9: f64 = (p.p87 * s.dn[870][9]);
        let eq16_e1097_d_n10: f64 = (p.p87 * s.dn[870][10]);
        let eq16_e1097_d_n11: f64 = (p.p87 * s.dn[870][11]);
        let eq16_e1097_d_n12: f64 = (p.p87 * s.dn[870][12]);
        let eq16_e1097_d_n13: f64 = (p.p87 * s.dn[870][13]);
        let eq16_e1097_d_n14: f64 = (p.p87 * s.dn[870][14]);
        let eq16_e1097_d_n15: f64 = (p.p87 * s.dn[870][15]);
        let eq16_e1097_d_n16: f64 = (p.p87 * s.dn[870][16]);
        let eq16_e1097_d_n17: f64 = (p.p87 * s.dn[870][17]);
        let eq16_e1097_d_b0: f64 = (p.p87 * s.db[870][0]);
        let eq16_e1097_d_b1: f64 = (p.p87 * s.db[870][1]);
        let eq16_e1097_d_b2: f64 = (p.p87 * s.db[870][2]);
        let eq16_e1097_d_b3: f64 = (p.p87 * s.db[870][3]);
        let eq16_e1097_d_b4: f64 = (p.p87 * s.db[870][4]);
        let eq16_e1097_d_b5: f64 = (p.p87 * s.db[870][5]);
        let eq16_e1097_d_b6: f64 = (p.p87 * s.db[870][6]);
        let eq16_e1097_d_b7: f64 = (p.p87 * s.db[870][7]);
        let eq16_e1097_d_b8: f64 = (p.p87 * s.db[870][8]);
        let eq16_e1097_d_b9: f64 = (p.p87 * s.db[870][9]);
        let eq16_e1097_d_b10: f64 = (p.p87 * s.db[870][10]);
        let eq16_e1097_d_b11: f64 = (p.p87 * s.db[870][11]);
        (eq16_e1097, eq16_e1097_d_n0, eq16_e1097_d_n1, eq16_e1097_d_n2, eq16_e1097_d_n3, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, eq16_e1097_d_n12, eq16_e1097_d_n13, eq16_e1097_d_n14, eq16_e1097_d_n15, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_b0, eq16_e1097_d_b1, eq16_e1097_d_b2, eq16_e1097_d_b3, eq16_e1097_d_b4, eq16_e1097_d_b5, eq16_e1097_d_b6, eq16_e1097_d_b7, eq16_e1097_d_b8, eq16_e1097_d_b9, eq16_e1097_d_b10, eq16_e1097_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1099;
        let eq16_node_derivatives: [f64; 18] = [eq16_e1099_d_n0, eq16_e1099_d_n1, eq16_e1099_d_n2, eq16_e1099_d_n3, eq16_e1099_d_n4, eq16_e1099_d_n5, eq16_e1099_d_n6, eq16_e1099_d_n7, eq16_e1099_d_n8, eq16_e1099_d_n9, eq16_e1099_d_n10, eq16_e1099_d_n11, eq16_e1099_d_n12, eq16_e1099_d_n13, eq16_e1099_d_n14, eq16_e1099_d_n15, eq16_e1099_d_n16, eq16_e1099_d_n17];
        let eq16_branch_derivatives: [f64; 12] = [eq16_e1099_d_b0, eq16_e1099_d_b1, eq16_e1099_d_b2, eq16_e1099_d_b3, eq16_e1099_d_b4, eq16_e1099_d_b5, eq16_e1099_d_b6, eq16_e1099_d_b7, eq16_e1099_d_b8, eq16_e1099_d_b9, eq16_e1099_d_b10, eq16_e1099_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1105, eq17_e1105_d_n0, eq17_e1105_d_n1, eq17_e1105_d_n2, eq17_e1105_d_n3, eq17_e1105_d_n4, eq17_e1105_d_n5, eq17_e1105_d_n6, eq17_e1105_d_n7, eq17_e1105_d_n8, eq17_e1105_d_n9, eq17_e1105_d_n10, eq17_e1105_d_n11, eq17_e1105_d_n12, eq17_e1105_d_n13, eq17_e1105_d_n14, eq17_e1105_d_n15, eq17_e1105_d_n16, eq17_e1105_d_n17, eq17_e1105_d_b0, eq17_e1105_d_b1, eq17_e1105_d_b2, eq17_e1105_d_b3, eq17_e1105_d_b4, eq17_e1105_d_b5, eq17_e1105_d_b6, eq17_e1105_d_b7, eq17_e1105_d_b8, eq17_e1105_d_b9, eq17_e1105_d_b10, eq17_e1105_d_b11,) = {
    if s.b[3405] {
        let eq17_e1103: f64 = (p.p87 * s.v[869]);
        let eq17_e1103_d_n0: f64 = (p.p87 * s.dn[869][0]);
        let eq17_e1103_d_n1: f64 = (p.p87 * s.dn[869][1]);
        let eq17_e1103_d_n2: f64 = (p.p87 * s.dn[869][2]);
        let eq17_e1103_d_n3: f64 = (p.p87 * s.dn[869][3]);
        let eq17_e1103_d_n4: f64 = (p.p87 * s.dn[869][4]);
        let eq17_e1103_d_n5: f64 = (p.p87 * s.dn[869][5]);
        let eq17_e1103_d_n6: f64 = (p.p87 * s.dn[869][6]);
        let eq17_e1103_d_n7: f64 = (p.p87 * s.dn[869][7]);
        let eq17_e1103_d_n8: f64 = (p.p87 * s.dn[869][8]);
        let eq17_e1103_d_n9: f64 = (p.p87 * s.dn[869][9]);
        let eq17_e1103_d_n10: f64 = (p.p87 * s.dn[869][10]);
        let eq17_e1103_d_n11: f64 = (p.p87 * s.dn[869][11]);
        let eq17_e1103_d_n12: f64 = (p.p87 * s.dn[869][12]);
        let eq17_e1103_d_n13: f64 = (p.p87 * s.dn[869][13]);
        let eq17_e1103_d_n14: f64 = (p.p87 * s.dn[869][14]);
        let eq17_e1103_d_n15: f64 = (p.p87 * s.dn[869][15]);
        let eq17_e1103_d_n16: f64 = (p.p87 * s.dn[869][16]);
        let eq17_e1103_d_n17: f64 = (p.p87 * s.dn[869][17]);
        let eq17_e1103_d_b0: f64 = (p.p87 * s.db[869][0]);
        let eq17_e1103_d_b1: f64 = (p.p87 * s.db[869][1]);
        let eq17_e1103_d_b2: f64 = (p.p87 * s.db[869][2]);
        let eq17_e1103_d_b3: f64 = (p.p87 * s.db[869][3]);
        let eq17_e1103_d_b4: f64 = (p.p87 * s.db[869][4]);
        let eq17_e1103_d_b5: f64 = (p.p87 * s.db[869][5]);
        let eq17_e1103_d_b6: f64 = (p.p87 * s.db[869][6]);
        let eq17_e1103_d_b7: f64 = (p.p87 * s.db[869][7]);
        let eq17_e1103_d_b8: f64 = (p.p87 * s.db[869][8]);
        let eq17_e1103_d_b9: f64 = (p.p87 * s.db[869][9]);
        let eq17_e1103_d_b10: f64 = (p.p87 * s.db[869][10]);
        let eq17_e1103_d_b11: f64 = (p.p87 * s.db[869][11]);
        (eq17_e1103, eq17_e1103_d_n0, eq17_e1103_d_n1, eq17_e1103_d_n2, eq17_e1103_d_n3, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n11, eq17_e1103_d_n12, eq17_e1103_d_n13, eq17_e1103_d_n14, eq17_e1103_d_n15, eq17_e1103_d_n16, eq17_e1103_d_n17, eq17_e1103_d_b0, eq17_e1103_d_b1, eq17_e1103_d_b2, eq17_e1103_d_b3, eq17_e1103_d_b4, eq17_e1103_d_b5, eq17_e1103_d_b6, eq17_e1103_d_b7, eq17_e1103_d_b8, eq17_e1103_d_b9, eq17_e1103_d_b10, eq17_e1103_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1105;
        let eq17_node_derivatives: [f64; 18] = [eq17_e1105_d_n0, eq17_e1105_d_n1, eq17_e1105_d_n2, eq17_e1105_d_n3, eq17_e1105_d_n4, eq17_e1105_d_n5, eq17_e1105_d_n6, eq17_e1105_d_n7, eq17_e1105_d_n8, eq17_e1105_d_n9, eq17_e1105_d_n10, eq17_e1105_d_n11, eq17_e1105_d_n12, eq17_e1105_d_n13, eq17_e1105_d_n14, eq17_e1105_d_n15, eq17_e1105_d_n16, eq17_e1105_d_n17];
        let eq17_branch_derivatives: [f64; 12] = [eq17_e1105_d_b0, eq17_e1105_d_b1, eq17_e1105_d_b2, eq17_e1105_d_b3, eq17_e1105_d_b4, eq17_e1105_d_b5, eq17_e1105_d_b6, eq17_e1105_d_b7, eq17_e1105_d_b8, eq17_e1105_d_b9, eq17_e1105_d_b10, eq17_e1105_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17, eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11,) = {
    if s.b[3405] {
        let eq18_e1109: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[68]);
        let eq18_e1109_d_n0: f64 = (s.dn[68][0] * ddt_scale);
        let eq18_e1109_d_n1: f64 = (s.dn[68][1] * ddt_scale);
        let eq18_e1109_d_n2: f64 = (s.dn[68][2] * ddt_scale);
        let eq18_e1109_d_n3: f64 = (s.dn[68][3] * ddt_scale);
        let eq18_e1109_d_n4: f64 = (s.dn[68][4] * ddt_scale);
        let eq18_e1109_d_n5: f64 = (s.dn[68][5] * ddt_scale);
        let eq18_e1109_d_n6: f64 = (s.dn[68][6] * ddt_scale);
        let eq18_e1109_d_n7: f64 = (s.dn[68][7] * ddt_scale);
        let eq18_e1109_d_n8: f64 = (s.dn[68][8] * ddt_scale);
        let eq18_e1109_d_n9: f64 = (s.dn[68][9] * ddt_scale);
        let eq18_e1109_d_n10: f64 = (s.dn[68][10] * ddt_scale);
        let eq18_e1109_d_n11: f64 = (s.dn[68][11] * ddt_scale);
        let eq18_e1109_d_n12: f64 = (s.dn[68][12] * ddt_scale);
        let eq18_e1109_d_n13: f64 = (s.dn[68][13] * ddt_scale);
        let eq18_e1109_d_n14: f64 = (s.dn[68][14] * ddt_scale);
        let eq18_e1109_d_n15: f64 = (s.dn[68][15] * ddt_scale);
        let eq18_e1109_d_n16: f64 = (s.dn[68][16] * ddt_scale);
        let eq18_e1109_d_n17: f64 = (s.dn[68][17] * ddt_scale);
        let eq18_e1109_d_b0: f64 = (s.db[68][0] * ddt_scale);
        let eq18_e1109_d_b1: f64 = (s.db[68][1] * ddt_scale);
        let eq18_e1109_d_b2: f64 = (s.db[68][2] * ddt_scale);
        let eq18_e1109_d_b3: f64 = (s.db[68][3] * ddt_scale);
        let eq18_e1109_d_b4: f64 = (s.db[68][4] * ddt_scale);
        let eq18_e1109_d_b5: f64 = (s.db[68][5] * ddt_scale);
        let eq18_e1109_d_b6: f64 = (s.db[68][6] * ddt_scale);
        let eq18_e1109_d_b7: f64 = (s.db[68][7] * ddt_scale);
        let eq18_e1109_d_b8: f64 = (s.db[68][8] * ddt_scale);
        let eq18_e1109_d_b9: f64 = (s.db[68][9] * ddt_scale);
        let eq18_e1109_d_b10: f64 = (s.db[68][10] * ddt_scale);
        let eq18_e1109_d_b11: f64 = (s.db[68][11] * ddt_scale);
        let eq18_e1110: f64 = (p.p87 * eq18_e1109);
        let eq18_e1110_d_n0: f64 = (p.p87 * eq18_e1109_d_n0);
        let eq18_e1110_d_n1: f64 = (p.p87 * eq18_e1109_d_n1);
        let eq18_e1110_d_n2: f64 = (p.p87 * eq18_e1109_d_n2);
        let eq18_e1110_d_n3: f64 = (p.p87 * eq18_e1109_d_n3);
        let eq18_e1110_d_n4: f64 = (p.p87 * eq18_e1109_d_n4);
        let eq18_e1110_d_n5: f64 = (p.p87 * eq18_e1109_d_n5);
        let eq18_e1110_d_n6: f64 = (p.p87 * eq18_e1109_d_n6);
        let eq18_e1110_d_n7: f64 = (p.p87 * eq18_e1109_d_n7);
        let eq18_e1110_d_n8: f64 = (p.p87 * eq18_e1109_d_n8);
        let eq18_e1110_d_n9: f64 = (p.p87 * eq18_e1109_d_n9);
        let eq18_e1110_d_n10: f64 = (p.p87 * eq18_e1109_d_n10);
        let eq18_e1110_d_n11: f64 = (p.p87 * eq18_e1109_d_n11);
        let eq18_e1110_d_n12: f64 = (p.p87 * eq18_e1109_d_n12);
        let eq18_e1110_d_n13: f64 = (p.p87 * eq18_e1109_d_n13);
        let eq18_e1110_d_n14: f64 = (p.p87 * eq18_e1109_d_n14);
        let eq18_e1110_d_n15: f64 = (p.p87 * eq18_e1109_d_n15);
        let eq18_e1110_d_n16: f64 = (p.p87 * eq18_e1109_d_n16);
        let eq18_e1110_d_n17: f64 = (p.p87 * eq18_e1109_d_n17);
        let eq18_e1110_d_b0: f64 = (p.p87 * eq18_e1109_d_b0);
        let eq18_e1110_d_b1: f64 = (p.p87 * eq18_e1109_d_b1);
        let eq18_e1110_d_b2: f64 = (p.p87 * eq18_e1109_d_b2);
        let eq18_e1110_d_b3: f64 = (p.p87 * eq18_e1109_d_b3);
        let eq18_e1110_d_b4: f64 = (p.p87 * eq18_e1109_d_b4);
        let eq18_e1110_d_b5: f64 = (p.p87 * eq18_e1109_d_b5);
        let eq18_e1110_d_b6: f64 = (p.p87 * eq18_e1109_d_b6);
        let eq18_e1110_d_b7: f64 = (p.p87 * eq18_e1109_d_b7);
        let eq18_e1110_d_b8: f64 = (p.p87 * eq18_e1109_d_b8);
        let eq18_e1110_d_b9: f64 = (p.p87 * eq18_e1109_d_b9);
        let eq18_e1110_d_b10: f64 = (p.p87 * eq18_e1109_d_b10);
        let eq18_e1110_d_b11: f64 = (p.p87 * eq18_e1109_d_b11);
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_n12, eq18_e1110_d_n13, eq18_e1110_d_n14, eq18_e1110_d_n15, eq18_e1110_d_n16, eq18_e1110_d_n17, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6, eq18_e1110_d_b7, eq18_e1110_d_b8, eq18_e1110_d_b9, eq18_e1110_d_b10, eq18_e1110_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1112;
        let eq18_node_derivatives: [f64; 18] = [eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17];
        let eq18_branch_derivatives: [f64; 12] = [eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17, eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11,) = {
    if s.b[3405] {
        let eq19_e1116: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[67]);
        let eq19_e1116_d_n0: f64 = (s.dn[67][0] * ddt_scale);
        let eq19_e1116_d_n1: f64 = (s.dn[67][1] * ddt_scale);
        let eq19_e1116_d_n2: f64 = (s.dn[67][2] * ddt_scale);
        let eq19_e1116_d_n3: f64 = (s.dn[67][3] * ddt_scale);
        let eq19_e1116_d_n4: f64 = (s.dn[67][4] * ddt_scale);
        let eq19_e1116_d_n5: f64 = (s.dn[67][5] * ddt_scale);
        let eq19_e1116_d_n6: f64 = (s.dn[67][6] * ddt_scale);
        let eq19_e1116_d_n7: f64 = (s.dn[67][7] * ddt_scale);
        let eq19_e1116_d_n8: f64 = (s.dn[67][8] * ddt_scale);
        let eq19_e1116_d_n9: f64 = (s.dn[67][9] * ddt_scale);
        let eq19_e1116_d_n10: f64 = (s.dn[67][10] * ddt_scale);
        let eq19_e1116_d_n11: f64 = (s.dn[67][11] * ddt_scale);
        let eq19_e1116_d_n12: f64 = (s.dn[67][12] * ddt_scale);
        let eq19_e1116_d_n13: f64 = (s.dn[67][13] * ddt_scale);
        let eq19_e1116_d_n14: f64 = (s.dn[67][14] * ddt_scale);
        let eq19_e1116_d_n15: f64 = (s.dn[67][15] * ddt_scale);
        let eq19_e1116_d_n16: f64 = (s.dn[67][16] * ddt_scale);
        let eq19_e1116_d_n17: f64 = (s.dn[67][17] * ddt_scale);
        let eq19_e1116_d_b0: f64 = (s.db[67][0] * ddt_scale);
        let eq19_e1116_d_b1: f64 = (s.db[67][1] * ddt_scale);
        let eq19_e1116_d_b2: f64 = (s.db[67][2] * ddt_scale);
        let eq19_e1116_d_b3: f64 = (s.db[67][3] * ddt_scale);
        let eq19_e1116_d_b4: f64 = (s.db[67][4] * ddt_scale);
        let eq19_e1116_d_b5: f64 = (s.db[67][5] * ddt_scale);
        let eq19_e1116_d_b6: f64 = (s.db[67][6] * ddt_scale);
        let eq19_e1116_d_b7: f64 = (s.db[67][7] * ddt_scale);
        let eq19_e1116_d_b8: f64 = (s.db[67][8] * ddt_scale);
        let eq19_e1116_d_b9: f64 = (s.db[67][9] * ddt_scale);
        let eq19_e1116_d_b10: f64 = (s.db[67][10] * ddt_scale);
        let eq19_e1116_d_b11: f64 = (s.db[67][11] * ddt_scale);
        let eq19_e1117: f64 = (p.p87 * eq19_e1116);
        let eq19_e1117_d_n0: f64 = (p.p87 * eq19_e1116_d_n0);
        let eq19_e1117_d_n1: f64 = (p.p87 * eq19_e1116_d_n1);
        let eq19_e1117_d_n2: f64 = (p.p87 * eq19_e1116_d_n2);
        let eq19_e1117_d_n3: f64 = (p.p87 * eq19_e1116_d_n3);
        let eq19_e1117_d_n4: f64 = (p.p87 * eq19_e1116_d_n4);
        let eq19_e1117_d_n5: f64 = (p.p87 * eq19_e1116_d_n5);
        let eq19_e1117_d_n6: f64 = (p.p87 * eq19_e1116_d_n6);
        let eq19_e1117_d_n7: f64 = (p.p87 * eq19_e1116_d_n7);
        let eq19_e1117_d_n8: f64 = (p.p87 * eq19_e1116_d_n8);
        let eq19_e1117_d_n9: f64 = (p.p87 * eq19_e1116_d_n9);
        let eq19_e1117_d_n10: f64 = (p.p87 * eq19_e1116_d_n10);
        let eq19_e1117_d_n11: f64 = (p.p87 * eq19_e1116_d_n11);
        let eq19_e1117_d_n12: f64 = (p.p87 * eq19_e1116_d_n12);
        let eq19_e1117_d_n13: f64 = (p.p87 * eq19_e1116_d_n13);
        let eq19_e1117_d_n14: f64 = (p.p87 * eq19_e1116_d_n14);
        let eq19_e1117_d_n15: f64 = (p.p87 * eq19_e1116_d_n15);
        let eq19_e1117_d_n16: f64 = (p.p87 * eq19_e1116_d_n16);
        let eq19_e1117_d_n17: f64 = (p.p87 * eq19_e1116_d_n17);
        let eq19_e1117_d_b0: f64 = (p.p87 * eq19_e1116_d_b0);
        let eq19_e1117_d_b1: f64 = (p.p87 * eq19_e1116_d_b1);
        let eq19_e1117_d_b2: f64 = (p.p87 * eq19_e1116_d_b2);
        let eq19_e1117_d_b3: f64 = (p.p87 * eq19_e1116_d_b3);
        let eq19_e1117_d_b4: f64 = (p.p87 * eq19_e1116_d_b4);
        let eq19_e1117_d_b5: f64 = (p.p87 * eq19_e1116_d_b5);
        let eq19_e1117_d_b6: f64 = (p.p87 * eq19_e1116_d_b6);
        let eq19_e1117_d_b7: f64 = (p.p87 * eq19_e1116_d_b7);
        let eq19_e1117_d_b8: f64 = (p.p87 * eq19_e1116_d_b8);
        let eq19_e1117_d_b9: f64 = (p.p87 * eq19_e1116_d_b9);
        let eq19_e1117_d_b10: f64 = (p.p87 * eq19_e1116_d_b10);
        let eq19_e1117_d_b11: f64 = (p.p87 * eq19_e1116_d_b11);
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n1, eq19_e1117_d_n2, eq19_e1117_d_n3, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n11, eq19_e1117_d_n12, eq19_e1117_d_n13, eq19_e1117_d_n14, eq19_e1117_d_n15, eq19_e1117_d_n16, eq19_e1117_d_n17, eq19_e1117_d_b0, eq19_e1117_d_b1, eq19_e1117_d_b2, eq19_e1117_d_b3, eq19_e1117_d_b4, eq19_e1117_d_b5, eq19_e1117_d_b6, eq19_e1117_d_b7, eq19_e1117_d_b8, eq19_e1117_d_b9, eq19_e1117_d_b10, eq19_e1117_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1119;
        let eq19_node_derivatives: [f64; 18] = [eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17];
        let eq19_branch_derivatives: [f64; 12] = [eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1125, eq20_e1125_d_n0, eq20_e1125_d_n1, eq20_e1125_d_n2, eq20_e1125_d_n3, eq20_e1125_d_n4, eq20_e1125_d_n5, eq20_e1125_d_n6, eq20_e1125_d_n7, eq20_e1125_d_n8, eq20_e1125_d_n9, eq20_e1125_d_n10, eq20_e1125_d_n11, eq20_e1125_d_n12, eq20_e1125_d_n13, eq20_e1125_d_n14, eq20_e1125_d_n15, eq20_e1125_d_n16, eq20_e1125_d_n17, eq20_e1125_d_b0, eq20_e1125_d_b1, eq20_e1125_d_b2, eq20_e1125_d_b3, eq20_e1125_d_b4, eq20_e1125_d_b5, eq20_e1125_d_b6, eq20_e1125_d_b7, eq20_e1125_d_b8, eq20_e1125_d_b9, eq20_e1125_d_b10, eq20_e1125_d_b11,) = {
    if s.b[3406] {
        let eq20_e1123: f64 = (p.p87 * s.v[200]);
        let eq20_e1123_d_n0: f64 = (p.p87 * s.dn[200][0]);
        let eq20_e1123_d_n1: f64 = (p.p87 * s.dn[200][1]);
        let eq20_e1123_d_n2: f64 = (p.p87 * s.dn[200][2]);
        let eq20_e1123_d_n3: f64 = (p.p87 * s.dn[200][3]);
        let eq20_e1123_d_n4: f64 = (p.p87 * s.dn[200][4]);
        let eq20_e1123_d_n5: f64 = (p.p87 * s.dn[200][5]);
        let eq20_e1123_d_n6: f64 = (p.p87 * s.dn[200][6]);
        let eq20_e1123_d_n7: f64 = (p.p87 * s.dn[200][7]);
        let eq20_e1123_d_n8: f64 = (p.p87 * s.dn[200][8]);
        let eq20_e1123_d_n9: f64 = (p.p87 * s.dn[200][9]);
        let eq20_e1123_d_n10: f64 = (p.p87 * s.dn[200][10]);
        let eq20_e1123_d_n11: f64 = (p.p87 * s.dn[200][11]);
        let eq20_e1123_d_n12: f64 = (p.p87 * s.dn[200][12]);
        let eq20_e1123_d_n13: f64 = (p.p87 * s.dn[200][13]);
        let eq20_e1123_d_n14: f64 = (p.p87 * s.dn[200][14]);
        let eq20_e1123_d_n15: f64 = (p.p87 * s.dn[200][15]);
        let eq20_e1123_d_n16: f64 = (p.p87 * s.dn[200][16]);
        let eq20_e1123_d_n17: f64 = (p.p87 * s.dn[200][17]);
        let eq20_e1123_d_b0: f64 = (p.p87 * s.db[200][0]);
        let eq20_e1123_d_b1: f64 = (p.p87 * s.db[200][1]);
        let eq20_e1123_d_b2: f64 = (p.p87 * s.db[200][2]);
        let eq20_e1123_d_b3: f64 = (p.p87 * s.db[200][3]);
        let eq20_e1123_d_b4: f64 = (p.p87 * s.db[200][4]);
        let eq20_e1123_d_b5: f64 = (p.p87 * s.db[200][5]);
        let eq20_e1123_d_b6: f64 = (p.p87 * s.db[200][6]);
        let eq20_e1123_d_b7: f64 = (p.p87 * s.db[200][7]);
        let eq20_e1123_d_b8: f64 = (p.p87 * s.db[200][8]);
        let eq20_e1123_d_b9: f64 = (p.p87 * s.db[200][9]);
        let eq20_e1123_d_b10: f64 = (p.p87 * s.db[200][10]);
        let eq20_e1123_d_b11: f64 = (p.p87 * s.db[200][11]);
        (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n1, eq20_e1123_d_n2, eq20_e1123_d_n3, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n12, eq20_e1123_d_n13, eq20_e1123_d_n14, eq20_e1123_d_n15, eq20_e1123_d_n16, eq20_e1123_d_n17, eq20_e1123_d_b0, eq20_e1123_d_b1, eq20_e1123_d_b2, eq20_e1123_d_b3, eq20_e1123_d_b4, eq20_e1123_d_b5, eq20_e1123_d_b6, eq20_e1123_d_b7, eq20_e1123_d_b8, eq20_e1123_d_b9, eq20_e1123_d_b10, eq20_e1123_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        let eq20_node_derivatives: [f64; 18] = [eq20_e1125_d_n0, eq20_e1125_d_n1, eq20_e1125_d_n2, eq20_e1125_d_n3, eq20_e1125_d_n4, eq20_e1125_d_n5, eq20_e1125_d_n6, eq20_e1125_d_n7, eq20_e1125_d_n8, eq20_e1125_d_n9, eq20_e1125_d_n10, eq20_e1125_d_n11, eq20_e1125_d_n12, eq20_e1125_d_n13, eq20_e1125_d_n14, eq20_e1125_d_n15, eq20_e1125_d_n16, eq20_e1125_d_n17];
        let eq20_branch_derivatives: [f64; 12] = [eq20_e1125_d_b0, eq20_e1125_d_b1, eq20_e1125_d_b2, eq20_e1125_d_b3, eq20_e1125_d_b4, eq20_e1125_d_b5, eq20_e1125_d_b6, eq20_e1125_d_b7, eq20_e1125_d_b8, eq20_e1125_d_b9, eq20_e1125_d_b10, eq20_e1125_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1131, eq21_e1131_d_n0, eq21_e1131_d_n1, eq21_e1131_d_n2, eq21_e1131_d_n3, eq21_e1131_d_n4, eq21_e1131_d_n5, eq21_e1131_d_n6, eq21_e1131_d_n7, eq21_e1131_d_n8, eq21_e1131_d_n9, eq21_e1131_d_n10, eq21_e1131_d_n11, eq21_e1131_d_n12, eq21_e1131_d_n13, eq21_e1131_d_n14, eq21_e1131_d_n15, eq21_e1131_d_n16, eq21_e1131_d_n17, eq21_e1131_d_b0, eq21_e1131_d_b1, eq21_e1131_d_b2, eq21_e1131_d_b3, eq21_e1131_d_b4, eq21_e1131_d_b5, eq21_e1131_d_b6, eq21_e1131_d_b7, eq21_e1131_d_b8, eq21_e1131_d_b9, eq21_e1131_d_b10, eq21_e1131_d_b11,) = {
    if s.b[3406] {
        let eq21_e1129: f64 = (p.p87 * s.v[201]);
        let eq21_e1129_d_n0: f64 = (p.p87 * s.dn[201][0]);
        let eq21_e1129_d_n1: f64 = (p.p87 * s.dn[201][1]);
        let eq21_e1129_d_n2: f64 = (p.p87 * s.dn[201][2]);
        let eq21_e1129_d_n3: f64 = (p.p87 * s.dn[201][3]);
        let eq21_e1129_d_n4: f64 = (p.p87 * s.dn[201][4]);
        let eq21_e1129_d_n5: f64 = (p.p87 * s.dn[201][5]);
        let eq21_e1129_d_n6: f64 = (p.p87 * s.dn[201][6]);
        let eq21_e1129_d_n7: f64 = (p.p87 * s.dn[201][7]);
        let eq21_e1129_d_n8: f64 = (p.p87 * s.dn[201][8]);
        let eq21_e1129_d_n9: f64 = (p.p87 * s.dn[201][9]);
        let eq21_e1129_d_n10: f64 = (p.p87 * s.dn[201][10]);
        let eq21_e1129_d_n11: f64 = (p.p87 * s.dn[201][11]);
        let eq21_e1129_d_n12: f64 = (p.p87 * s.dn[201][12]);
        let eq21_e1129_d_n13: f64 = (p.p87 * s.dn[201][13]);
        let eq21_e1129_d_n14: f64 = (p.p87 * s.dn[201][14]);
        let eq21_e1129_d_n15: f64 = (p.p87 * s.dn[201][15]);
        let eq21_e1129_d_n16: f64 = (p.p87 * s.dn[201][16]);
        let eq21_e1129_d_n17: f64 = (p.p87 * s.dn[201][17]);
        let eq21_e1129_d_b0: f64 = (p.p87 * s.db[201][0]);
        let eq21_e1129_d_b1: f64 = (p.p87 * s.db[201][1]);
        let eq21_e1129_d_b2: f64 = (p.p87 * s.db[201][2]);
        let eq21_e1129_d_b3: f64 = (p.p87 * s.db[201][3]);
        let eq21_e1129_d_b4: f64 = (p.p87 * s.db[201][4]);
        let eq21_e1129_d_b5: f64 = (p.p87 * s.db[201][5]);
        let eq21_e1129_d_b6: f64 = (p.p87 * s.db[201][6]);
        let eq21_e1129_d_b7: f64 = (p.p87 * s.db[201][7]);
        let eq21_e1129_d_b8: f64 = (p.p87 * s.db[201][8]);
        let eq21_e1129_d_b9: f64 = (p.p87 * s.db[201][9]);
        let eq21_e1129_d_b10: f64 = (p.p87 * s.db[201][10]);
        let eq21_e1129_d_b11: f64 = (p.p87 * s.db[201][11]);
        (eq21_e1129, eq21_e1129_d_n0, eq21_e1129_d_n1, eq21_e1129_d_n2, eq21_e1129_d_n3, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n11, eq21_e1129_d_n12, eq21_e1129_d_n13, eq21_e1129_d_n14, eq21_e1129_d_n15, eq21_e1129_d_n16, eq21_e1129_d_n17, eq21_e1129_d_b0, eq21_e1129_d_b1, eq21_e1129_d_b2, eq21_e1129_d_b3, eq21_e1129_d_b4, eq21_e1129_d_b5, eq21_e1129_d_b6, eq21_e1129_d_b7, eq21_e1129_d_b8, eq21_e1129_d_b9, eq21_e1129_d_b10, eq21_e1129_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1131;
        let eq21_node_derivatives: [f64; 18] = [eq21_e1131_d_n0, eq21_e1131_d_n1, eq21_e1131_d_n2, eq21_e1131_d_n3, eq21_e1131_d_n4, eq21_e1131_d_n5, eq21_e1131_d_n6, eq21_e1131_d_n7, eq21_e1131_d_n8, eq21_e1131_d_n9, eq21_e1131_d_n10, eq21_e1131_d_n11, eq21_e1131_d_n12, eq21_e1131_d_n13, eq21_e1131_d_n14, eq21_e1131_d_n15, eq21_e1131_d_n16, eq21_e1131_d_n17];
        let eq21_branch_derivatives: [f64; 12] = [eq21_e1131_d_b0, eq21_e1131_d_b1, eq21_e1131_d_b2, eq21_e1131_d_b3, eq21_e1131_d_b4, eq21_e1131_d_b5, eq21_e1131_d_b6, eq21_e1131_d_b7, eq21_e1131_d_b8, eq21_e1131_d_b9, eq21_e1131_d_b10, eq21_e1131_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1137, eq22_e1137_d_n0, eq22_e1137_d_n1, eq22_e1137_d_n2, eq22_e1137_d_n3, eq22_e1137_d_n4, eq22_e1137_d_n5, eq22_e1137_d_n6, eq22_e1137_d_n7, eq22_e1137_d_n8, eq22_e1137_d_n9, eq22_e1137_d_n10, eq22_e1137_d_n11, eq22_e1137_d_n12, eq22_e1137_d_n13, eq22_e1137_d_n14, eq22_e1137_d_n15, eq22_e1137_d_n16, eq22_e1137_d_n17, eq22_e1137_d_b0, eq22_e1137_d_b1, eq22_e1137_d_b2, eq22_e1137_d_b3, eq22_e1137_d_b4, eq22_e1137_d_b5, eq22_e1137_d_b6, eq22_e1137_d_b7, eq22_e1137_d_b8, eq22_e1137_d_b9, eq22_e1137_d_b10, eq22_e1137_d_b11,) = {
    if s.b[3406] {
        let eq22_e1135: f64 = (p.p87 * s.v[202]);
        let eq22_e1135_d_n0: f64 = (p.p87 * s.dn[202][0]);
        let eq22_e1135_d_n1: f64 = (p.p87 * s.dn[202][1]);
        let eq22_e1135_d_n2: f64 = (p.p87 * s.dn[202][2]);
        let eq22_e1135_d_n3: f64 = (p.p87 * s.dn[202][3]);
        let eq22_e1135_d_n4: f64 = (p.p87 * s.dn[202][4]);
        let eq22_e1135_d_n5: f64 = (p.p87 * s.dn[202][5]);
        let eq22_e1135_d_n6: f64 = (p.p87 * s.dn[202][6]);
        let eq22_e1135_d_n7: f64 = (p.p87 * s.dn[202][7]);
        let eq22_e1135_d_n8: f64 = (p.p87 * s.dn[202][8]);
        let eq22_e1135_d_n9: f64 = (p.p87 * s.dn[202][9]);
        let eq22_e1135_d_n10: f64 = (p.p87 * s.dn[202][10]);
        let eq22_e1135_d_n11: f64 = (p.p87 * s.dn[202][11]);
        let eq22_e1135_d_n12: f64 = (p.p87 * s.dn[202][12]);
        let eq22_e1135_d_n13: f64 = (p.p87 * s.dn[202][13]);
        let eq22_e1135_d_n14: f64 = (p.p87 * s.dn[202][14]);
        let eq22_e1135_d_n15: f64 = (p.p87 * s.dn[202][15]);
        let eq22_e1135_d_n16: f64 = (p.p87 * s.dn[202][16]);
        let eq22_e1135_d_n17: f64 = (p.p87 * s.dn[202][17]);
        let eq22_e1135_d_b0: f64 = (p.p87 * s.db[202][0]);
        let eq22_e1135_d_b1: f64 = (p.p87 * s.db[202][1]);
        let eq22_e1135_d_b2: f64 = (p.p87 * s.db[202][2]);
        let eq22_e1135_d_b3: f64 = (p.p87 * s.db[202][3]);
        let eq22_e1135_d_b4: f64 = (p.p87 * s.db[202][4]);
        let eq22_e1135_d_b5: f64 = (p.p87 * s.db[202][5]);
        let eq22_e1135_d_b6: f64 = (p.p87 * s.db[202][6]);
        let eq22_e1135_d_b7: f64 = (p.p87 * s.db[202][7]);
        let eq22_e1135_d_b8: f64 = (p.p87 * s.db[202][8]);
        let eq22_e1135_d_b9: f64 = (p.p87 * s.db[202][9]);
        let eq22_e1135_d_b10: f64 = (p.p87 * s.db[202][10]);
        let eq22_e1135_d_b11: f64 = (p.p87 * s.db[202][11]);
        (eq22_e1135, eq22_e1135_d_n0, eq22_e1135_d_n1, eq22_e1135_d_n2, eq22_e1135_d_n3, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n11, eq22_e1135_d_n12, eq22_e1135_d_n13, eq22_e1135_d_n14, eq22_e1135_d_n15, eq22_e1135_d_n16, eq22_e1135_d_n17, eq22_e1135_d_b0, eq22_e1135_d_b1, eq22_e1135_d_b2, eq22_e1135_d_b3, eq22_e1135_d_b4, eq22_e1135_d_b5, eq22_e1135_d_b6, eq22_e1135_d_b7, eq22_e1135_d_b8, eq22_e1135_d_b9, eq22_e1135_d_b10, eq22_e1135_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1137;
        let eq22_node_derivatives: [f64; 18] = [eq22_e1137_d_n0, eq22_e1137_d_n1, eq22_e1137_d_n2, eq22_e1137_d_n3, eq22_e1137_d_n4, eq22_e1137_d_n5, eq22_e1137_d_n6, eq22_e1137_d_n7, eq22_e1137_d_n8, eq22_e1137_d_n9, eq22_e1137_d_n10, eq22_e1137_d_n11, eq22_e1137_d_n12, eq22_e1137_d_n13, eq22_e1137_d_n14, eq22_e1137_d_n15, eq22_e1137_d_n16, eq22_e1137_d_n17];
        let eq22_branch_derivatives: [f64; 12] = [eq22_e1137_d_b0, eq22_e1137_d_b1, eq22_e1137_d_b2, eq22_e1137_d_b3, eq22_e1137_d_b4, eq22_e1137_d_b5, eq22_e1137_d_b6, eq22_e1137_d_b7, eq22_e1137_d_b8, eq22_e1137_d_b9, eq22_e1137_d_b10, eq22_e1137_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq23_e1143, eq23_e1143_d_n0, eq23_e1143_d_n1, eq23_e1143_d_n2, eq23_e1143_d_n3, eq23_e1143_d_n4, eq23_e1143_d_n5, eq23_e1143_d_n6, eq23_e1143_d_n7, eq23_e1143_d_n8, eq23_e1143_d_n9, eq23_e1143_d_n10, eq23_e1143_d_n11, eq23_e1143_d_n12, eq23_e1143_d_n13, eq23_e1143_d_n14, eq23_e1143_d_n15, eq23_e1143_d_n16, eq23_e1143_d_n17, eq23_e1143_d_b0, eq23_e1143_d_b1, eq23_e1143_d_b2, eq23_e1143_d_b3, eq23_e1143_d_b4, eq23_e1143_d_b5, eq23_e1143_d_b6, eq23_e1143_d_b7, eq23_e1143_d_b8, eq23_e1143_d_b9, eq23_e1143_d_b10, eq23_e1143_d_b11,) = {
    if (s.v[75] != 0.0) {
        let eq23_e1141: f64 = ((nv0 - nv5) / s.v[4]);
        let eq23_e1141_d_n0: f64 = ((s.v[4] - ((nv0 - nv5) * s.dn[4][0])) / (s.v[4] * s.v[4]));
        let eq23_e1141_d_n1: f64 = (-(((nv0 - nv5) * s.dn[4][1]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n2: f64 = (-(((nv0 - nv5) * s.dn[4][2]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n3: f64 = (-(((nv0 - nv5) * s.dn[4][3]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n4: f64 = (-(((nv0 - nv5) * s.dn[4][4]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n5: f64 = (((-s.v[4]) - ((nv0 - nv5) * s.dn[4][5])) / (s.v[4] * s.v[4]));
        let eq23_e1141_d_n6: f64 = (-(((nv0 - nv5) * s.dn[4][6]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n7: f64 = (-(((nv0 - nv5) * s.dn[4][7]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n8: f64 = (-(((nv0 - nv5) * s.dn[4][8]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n9: f64 = (-(((nv0 - nv5) * s.dn[4][9]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n10: f64 = (-(((nv0 - nv5) * s.dn[4][10]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n11: f64 = (-(((nv0 - nv5) * s.dn[4][11]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n12: f64 = (-(((nv0 - nv5) * s.dn[4][12]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n13: f64 = (-(((nv0 - nv5) * s.dn[4][13]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n14: f64 = (-(((nv0 - nv5) * s.dn[4][14]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n15: f64 = (-(((nv0 - nv5) * s.dn[4][15]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n16: f64 = (-(((nv0 - nv5) * s.dn[4][16]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_n17: f64 = (-(((nv0 - nv5) * s.dn[4][17]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b0: f64 = (-(((nv0 - nv5) * s.db[4][0]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b1: f64 = (-(((nv0 - nv5) * s.db[4][1]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b2: f64 = (-(((nv0 - nv5) * s.db[4][2]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b3: f64 = (-(((nv0 - nv5) * s.db[4][3]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b4: f64 = (-(((nv0 - nv5) * s.db[4][4]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b5: f64 = (-(((nv0 - nv5) * s.db[4][5]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b6: f64 = (-(((nv0 - nv5) * s.db[4][6]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b7: f64 = (-(((nv0 - nv5) * s.db[4][7]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b8: f64 = (-(((nv0 - nv5) * s.db[4][8]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b9: f64 = (-(((nv0 - nv5) * s.db[4][9]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b10: f64 = (-(((nv0 - nv5) * s.db[4][10]) / (s.v[4] * s.v[4])));
        let eq23_e1141_d_b11: f64 = (-(((nv0 - nv5) * s.db[4][11]) / (s.v[4] * s.v[4])));
        (eq23_e1141, eq23_e1141_d_n0, eq23_e1141_d_n1, eq23_e1141_d_n2, eq23_e1141_d_n3, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n11, eq23_e1141_d_n12, eq23_e1141_d_n13, eq23_e1141_d_n14, eq23_e1141_d_n15, eq23_e1141_d_n16, eq23_e1141_d_n17, eq23_e1141_d_b0, eq23_e1141_d_b1, eq23_e1141_d_b2, eq23_e1141_d_b3, eq23_e1141_d_b4, eq23_e1141_d_b5, eq23_e1141_d_b6, eq23_e1141_d_b7, eq23_e1141_d_b8, eq23_e1141_d_b9, eq23_e1141_d_b10, eq23_e1141_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1143;
        let eq23_node_derivatives: [f64; 18] = [eq23_e1143_d_n0, eq23_e1143_d_n1, eq23_e1143_d_n2, eq23_e1143_d_n3, eq23_e1143_d_n4, eq23_e1143_d_n5, eq23_e1143_d_n6, eq23_e1143_d_n7, eq23_e1143_d_n8, eq23_e1143_d_n9, eq23_e1143_d_n10, eq23_e1143_d_n11, eq23_e1143_d_n12, eq23_e1143_d_n13, eq23_e1143_d_n14, eq23_e1143_d_n15, eq23_e1143_d_n16, eq23_e1143_d_n17];
        let eq23_branch_derivatives: [f64; 12] = [eq23_e1143_d_b0, eq23_e1143_d_b1, eq23_e1143_d_b2, eq23_e1143_d_b3, eq23_e1143_d_b4, eq23_e1143_d_b5, eq23_e1143_d_b6, eq23_e1143_d_b7, eq23_e1143_d_b8, eq23_e1143_d_b9, eq23_e1143_d_b10, eq23_e1143_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1148,) = {
    if (s.v[75] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e1148;
        stamper.stamp_potential_const_local(
            3,
            eq24_value,
        );
        let (eq25_e1154, eq25_e1154_d_n0, eq25_e1154_d_n1, eq25_e1154_d_n2, eq25_e1154_d_n3, eq25_e1154_d_n4, eq25_e1154_d_n5, eq25_e1154_d_n6, eq25_e1154_d_n7, eq25_e1154_d_n8, eq25_e1154_d_n9, eq25_e1154_d_n10, eq25_e1154_d_n11, eq25_e1154_d_n12, eq25_e1154_d_n13, eq25_e1154_d_n14, eq25_e1154_d_n15, eq25_e1154_d_n16, eq25_e1154_d_n17, eq25_e1154_d_b0, eq25_e1154_d_b1, eq25_e1154_d_b2, eq25_e1154_d_b3, eq25_e1154_d_b4, eq25_e1154_d_b5, eq25_e1154_d_b6, eq25_e1154_d_b7, eq25_e1154_d_b8, eq25_e1154_d_b9, eq25_e1154_d_b10, eq25_e1154_d_b11,) = {
    if (s.v[76] != 0.0) {
        let eq25_e1152: f64 = ((nv7 - nv2) / s.v[5]);
        let eq25_e1152_d_n0: f64 = (-(((nv7 - nv2) * s.dn[5][0]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n1: f64 = (-(((nv7 - nv2) * s.dn[5][1]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n2: f64 = (((-s.v[5]) - ((nv7 - nv2) * s.dn[5][2])) / (s.v[5] * s.v[5]));
        let eq25_e1152_d_n3: f64 = (-(((nv7 - nv2) * s.dn[5][3]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n4: f64 = (-(((nv7 - nv2) * s.dn[5][4]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n5: f64 = (-(((nv7 - nv2) * s.dn[5][5]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n6: f64 = (-(((nv7 - nv2) * s.dn[5][6]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n7: f64 = ((s.v[5] - ((nv7 - nv2) * s.dn[5][7])) / (s.v[5] * s.v[5]));
        let eq25_e1152_d_n8: f64 = (-(((nv7 - nv2) * s.dn[5][8]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n9: f64 = (-(((nv7 - nv2) * s.dn[5][9]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n10: f64 = (-(((nv7 - nv2) * s.dn[5][10]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n11: f64 = (-(((nv7 - nv2) * s.dn[5][11]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n12: f64 = (-(((nv7 - nv2) * s.dn[5][12]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n13: f64 = (-(((nv7 - nv2) * s.dn[5][13]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n14: f64 = (-(((nv7 - nv2) * s.dn[5][14]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n15: f64 = (-(((nv7 - nv2) * s.dn[5][15]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n16: f64 = (-(((nv7 - nv2) * s.dn[5][16]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_n17: f64 = (-(((nv7 - nv2) * s.dn[5][17]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b0: f64 = (-(((nv7 - nv2) * s.db[5][0]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b1: f64 = (-(((nv7 - nv2) * s.db[5][1]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b2: f64 = (-(((nv7 - nv2) * s.db[5][2]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b3: f64 = (-(((nv7 - nv2) * s.db[5][3]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b4: f64 = (-(((nv7 - nv2) * s.db[5][4]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b5: f64 = (-(((nv7 - nv2) * s.db[5][5]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b6: f64 = (-(((nv7 - nv2) * s.db[5][6]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b7: f64 = (-(((nv7 - nv2) * s.db[5][7]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b8: f64 = (-(((nv7 - nv2) * s.db[5][8]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b9: f64 = (-(((nv7 - nv2) * s.db[5][9]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b10: f64 = (-(((nv7 - nv2) * s.db[5][10]) / (s.v[5] * s.v[5])));
        let eq25_e1152_d_b11: f64 = (-(((nv7 - nv2) * s.db[5][11]) / (s.v[5] * s.v[5])));
        (eq25_e1152, eq25_e1152_d_n0, eq25_e1152_d_n1, eq25_e1152_d_n2, eq25_e1152_d_n3, eq25_e1152_d_n4, eq25_e1152_d_n5, eq25_e1152_d_n6, eq25_e1152_d_n7, eq25_e1152_d_n8, eq25_e1152_d_n9, eq25_e1152_d_n10, eq25_e1152_d_n11, eq25_e1152_d_n12, eq25_e1152_d_n13, eq25_e1152_d_n14, eq25_e1152_d_n15, eq25_e1152_d_n16, eq25_e1152_d_n17, eq25_e1152_d_b0, eq25_e1152_d_b1, eq25_e1152_d_b2, eq25_e1152_d_b3, eq25_e1152_d_b4, eq25_e1152_d_b5, eq25_e1152_d_b6, eq25_e1152_d_b7, eq25_e1152_d_b8, eq25_e1152_d_b9, eq25_e1152_d_b10, eq25_e1152_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1154;
        let eq25_node_derivatives: [f64; 18] = [eq25_e1154_d_n0, eq25_e1154_d_n1, eq25_e1154_d_n2, eq25_e1154_d_n3, eq25_e1154_d_n4, eq25_e1154_d_n5, eq25_e1154_d_n6, eq25_e1154_d_n7, eq25_e1154_d_n8, eq25_e1154_d_n9, eq25_e1154_d_n10, eq25_e1154_d_n11, eq25_e1154_d_n12, eq25_e1154_d_n13, eq25_e1154_d_n14, eq25_e1154_d_n15, eq25_e1154_d_n16, eq25_e1154_d_n17];
        let eq25_branch_derivatives: [f64; 12] = [eq25_e1154_d_b0, eq25_e1154_d_b1, eq25_e1154_d_b2, eq25_e1154_d_b3, eq25_e1154_d_b4, eq25_e1154_d_b5, eq25_e1154_d_b6, eq25_e1154_d_b7, eq25_e1154_d_b8, eq25_e1154_d_b9, eq25_e1154_d_b10, eq25_e1154_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1159,) = {
    if (s.v[76] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1159;
        stamper.stamp_potential_const_local(
            4,
            eq26_value,
        );
        let eq27_e1163: f64 = (s.v[18] + s.v[753]);
        let eq27_e1163_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq27_e1163_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq27_e1163_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq27_e1163_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq27_e1163_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq27_e1163_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq27_e1163_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq27_e1163_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq27_e1163_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq27_e1163_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq27_e1163_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq27_e1163_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq27_e1163_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq27_e1163_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq27_e1163_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq27_e1163_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq27_e1163_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq27_e1163_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq27_e1163_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq27_e1163_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq27_e1163_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq27_e1163_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq27_e1163_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq27_e1163_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq27_e1163_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq27_e1163_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq27_e1163_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq27_e1163_d_b9: f64 = (s.db[18][9] + s.db[753][9]);
        let eq27_e1163_d_b10: f64 = (s.db[18][10] + s.db[753][10]);
        let eq27_e1163_d_b11: f64 = (s.db[18][11] + s.db[753][11]);
        let eq27_e1164: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq27_e1163);
        let eq27_e1164_d_n0: f64 = (eq27_e1163_d_n0 * ddt_scale);
        let eq27_e1164_d_n1: f64 = (eq27_e1163_d_n1 * ddt_scale);
        let eq27_e1164_d_n2: f64 = (eq27_e1163_d_n2 * ddt_scale);
        let eq27_e1164_d_n3: f64 = (eq27_e1163_d_n3 * ddt_scale);
        let eq27_e1164_d_n4: f64 = (eq27_e1163_d_n4 * ddt_scale);
        let eq27_e1164_d_n5: f64 = (eq27_e1163_d_n5 * ddt_scale);
        let eq27_e1164_d_n6: f64 = (eq27_e1163_d_n6 * ddt_scale);
        let eq27_e1164_d_n7: f64 = (eq27_e1163_d_n7 * ddt_scale);
        let eq27_e1164_d_n8: f64 = (eq27_e1163_d_n8 * ddt_scale);
        let eq27_e1164_d_n9: f64 = (eq27_e1163_d_n9 * ddt_scale);
        let eq27_e1164_d_n10: f64 = (eq27_e1163_d_n10 * ddt_scale);
        let eq27_e1164_d_n11: f64 = (eq27_e1163_d_n11 * ddt_scale);
        let eq27_e1164_d_n12: f64 = (eq27_e1163_d_n12 * ddt_scale);
        let eq27_e1164_d_n13: f64 = (eq27_e1163_d_n13 * ddt_scale);
        let eq27_e1164_d_n14: f64 = (eq27_e1163_d_n14 * ddt_scale);
        let eq27_e1164_d_n15: f64 = (eq27_e1163_d_n15 * ddt_scale);
        let eq27_e1164_d_n16: f64 = (eq27_e1163_d_n16 * ddt_scale);
        let eq27_e1164_d_n17: f64 = (eq27_e1163_d_n17 * ddt_scale);
        let eq27_e1164_d_b0: f64 = (eq27_e1163_d_b0 * ddt_scale);
        let eq27_e1164_d_b1: f64 = (eq27_e1163_d_b1 * ddt_scale);
        let eq27_e1164_d_b2: f64 = (eq27_e1163_d_b2 * ddt_scale);
        let eq27_e1164_d_b3: f64 = (eq27_e1163_d_b3 * ddt_scale);
        let eq27_e1164_d_b4: f64 = (eq27_e1163_d_b4 * ddt_scale);
        let eq27_e1164_d_b5: f64 = (eq27_e1163_d_b5 * ddt_scale);
        let eq27_e1164_d_b6: f64 = (eq27_e1163_d_b6 * ddt_scale);
        let eq27_e1164_d_b7: f64 = (eq27_e1163_d_b7 * ddt_scale);
        let eq27_e1164_d_b8: f64 = (eq27_e1163_d_b8 * ddt_scale);
        let eq27_e1164_d_b9: f64 = (eq27_e1163_d_b9 * ddt_scale);
        let eq27_e1164_d_b10: f64 = (eq27_e1163_d_b10 * ddt_scale);
        let eq27_e1164_d_b11: f64 = (eq27_e1163_d_b11 * ddt_scale);
        let eq27_e1165: f64 = (p.p87 * eq27_e1164);
        let eq27_e1165_d_n0: f64 = (p.p87 * eq27_e1164_d_n0);
        let eq27_e1165_d_n1: f64 = (p.p87 * eq27_e1164_d_n1);
        let eq27_e1165_d_n2: f64 = (p.p87 * eq27_e1164_d_n2);
        let eq27_e1165_d_n3: f64 = (p.p87 * eq27_e1164_d_n3);
        let eq27_e1165_d_n4: f64 = (p.p87 * eq27_e1164_d_n4);
        let eq27_e1165_d_n5: f64 = (p.p87 * eq27_e1164_d_n5);
        let eq27_e1165_d_n6: f64 = (p.p87 * eq27_e1164_d_n6);
        let eq27_e1165_d_n7: f64 = (p.p87 * eq27_e1164_d_n7);
        let eq27_e1165_d_n8: f64 = (p.p87 * eq27_e1164_d_n8);
        let eq27_e1165_d_n9: f64 = (p.p87 * eq27_e1164_d_n9);
        let eq27_e1165_d_n10: f64 = (p.p87 * eq27_e1164_d_n10);
        let eq27_e1165_d_n11: f64 = (p.p87 * eq27_e1164_d_n11);
        let eq27_e1165_d_n12: f64 = (p.p87 * eq27_e1164_d_n12);
        let eq27_e1165_d_n13: f64 = (p.p87 * eq27_e1164_d_n13);
        let eq27_e1165_d_n14: f64 = (p.p87 * eq27_e1164_d_n14);
        let eq27_e1165_d_n15: f64 = (p.p87 * eq27_e1164_d_n15);
        let eq27_e1165_d_n16: f64 = (p.p87 * eq27_e1164_d_n16);
        let eq27_e1165_d_n17: f64 = (p.p87 * eq27_e1164_d_n17);
        let eq27_e1165_d_b0: f64 = (p.p87 * eq27_e1164_d_b0);
        let eq27_e1165_d_b1: f64 = (p.p87 * eq27_e1164_d_b1);
        let eq27_e1165_d_b2: f64 = (p.p87 * eq27_e1164_d_b2);
        let eq27_e1165_d_b3: f64 = (p.p87 * eq27_e1164_d_b3);
        let eq27_e1165_d_b4: f64 = (p.p87 * eq27_e1164_d_b4);
        let eq27_e1165_d_b5: f64 = (p.p87 * eq27_e1164_d_b5);
        let eq27_e1165_d_b6: f64 = (p.p87 * eq27_e1164_d_b6);
        let eq27_e1165_d_b7: f64 = (p.p87 * eq27_e1164_d_b7);
        let eq27_e1165_d_b8: f64 = (p.p87 * eq27_e1164_d_b8);
        let eq27_e1165_d_b9: f64 = (p.p87 * eq27_e1164_d_b9);
        let eq27_e1165_d_b10: f64 = (p.p87 * eq27_e1164_d_b10);
        let eq27_e1165_d_b11: f64 = (p.p87 * eq27_e1164_d_b11);
        let eq27_value: f64 = eq27_e1165;
        let eq27_node_derivatives: [f64; 18] = [eq27_e1165_d_n0, eq27_e1165_d_n1, eq27_e1165_d_n2, eq27_e1165_d_n3, eq27_e1165_d_n4, eq27_e1165_d_n5, eq27_e1165_d_n6, eq27_e1165_d_n7, eq27_e1165_d_n8, eq27_e1165_d_n9, eq27_e1165_d_n10, eq27_e1165_d_n11, eq27_e1165_d_n12, eq27_e1165_d_n13, eq27_e1165_d_n14, eq27_e1165_d_n15, eq27_e1165_d_n16, eq27_e1165_d_n17];
        let eq27_branch_derivatives: [f64; 12] = [eq27_e1165_d_b0, eq27_e1165_d_b1, eq27_e1165_d_b2, eq27_e1165_d_b3, eq27_e1165_d_b4, eq27_e1165_d_b5, eq27_e1165_d_b6, eq27_e1165_d_b7, eq27_e1165_d_b8, eq27_e1165_d_b9, eq27_e1165_d_b10, eq27_e1165_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let eq28_e1169: f64 = (s.v[19] + s.v[751]);
        let eq28_e1169_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq28_e1169_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq28_e1169_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq28_e1169_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq28_e1169_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq28_e1169_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq28_e1169_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq28_e1169_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq28_e1169_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq28_e1169_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq28_e1169_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq28_e1169_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq28_e1169_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq28_e1169_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq28_e1169_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq28_e1169_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq28_e1169_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq28_e1169_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq28_e1169_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq28_e1169_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq28_e1169_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq28_e1169_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq28_e1169_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq28_e1169_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq28_e1169_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq28_e1169_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq28_e1169_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq28_e1169_d_b9: f64 = (s.db[19][9] + s.db[751][9]);
        let eq28_e1169_d_b10: f64 = (s.db[19][10] + s.db[751][10]);
        let eq28_e1169_d_b11: f64 = (s.db[19][11] + s.db[751][11]);
        let eq28_e1170: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq28_e1169);
        let eq28_e1170_d_n0: f64 = (eq28_e1169_d_n0 * ddt_scale);
        let eq28_e1170_d_n1: f64 = (eq28_e1169_d_n1 * ddt_scale);
        let eq28_e1170_d_n2: f64 = (eq28_e1169_d_n2 * ddt_scale);
        let eq28_e1170_d_n3: f64 = (eq28_e1169_d_n3 * ddt_scale);
        let eq28_e1170_d_n4: f64 = (eq28_e1169_d_n4 * ddt_scale);
        let eq28_e1170_d_n5: f64 = (eq28_e1169_d_n5 * ddt_scale);
        let eq28_e1170_d_n6: f64 = (eq28_e1169_d_n6 * ddt_scale);
        let eq28_e1170_d_n7: f64 = (eq28_e1169_d_n7 * ddt_scale);
        let eq28_e1170_d_n8: f64 = (eq28_e1169_d_n8 * ddt_scale);
        let eq28_e1170_d_n9: f64 = (eq28_e1169_d_n9 * ddt_scale);
        let eq28_e1170_d_n10: f64 = (eq28_e1169_d_n10 * ddt_scale);
        let eq28_e1170_d_n11: f64 = (eq28_e1169_d_n11 * ddt_scale);
        let eq28_e1170_d_n12: f64 = (eq28_e1169_d_n12 * ddt_scale);
        let eq28_e1170_d_n13: f64 = (eq28_e1169_d_n13 * ddt_scale);
        let eq28_e1170_d_n14: f64 = (eq28_e1169_d_n14 * ddt_scale);
        let eq28_e1170_d_n15: f64 = (eq28_e1169_d_n15 * ddt_scale);
        let eq28_e1170_d_n16: f64 = (eq28_e1169_d_n16 * ddt_scale);
        let eq28_e1170_d_n17: f64 = (eq28_e1169_d_n17 * ddt_scale);
        let eq28_e1170_d_b0: f64 = (eq28_e1169_d_b0 * ddt_scale);
        let eq28_e1170_d_b1: f64 = (eq28_e1169_d_b1 * ddt_scale);
        let eq28_e1170_d_b2: f64 = (eq28_e1169_d_b2 * ddt_scale);
        let eq28_e1170_d_b3: f64 = (eq28_e1169_d_b3 * ddt_scale);
        let eq28_e1170_d_b4: f64 = (eq28_e1169_d_b4 * ddt_scale);
        let eq28_e1170_d_b5: f64 = (eq28_e1169_d_b5 * ddt_scale);
        let eq28_e1170_d_b6: f64 = (eq28_e1169_d_b6 * ddt_scale);
        let eq28_e1170_d_b7: f64 = (eq28_e1169_d_b7 * ddt_scale);
        let eq28_e1170_d_b8: f64 = (eq28_e1169_d_b8 * ddt_scale);
        let eq28_e1170_d_b9: f64 = (eq28_e1169_d_b9 * ddt_scale);
        let eq28_e1170_d_b10: f64 = (eq28_e1169_d_b10 * ddt_scale);
        let eq28_e1170_d_b11: f64 = (eq28_e1169_d_b11 * ddt_scale);
        let eq28_e1171: f64 = (p.p87 * eq28_e1170);
        let eq28_e1171_d_n0: f64 = (p.p87 * eq28_e1170_d_n0);
        let eq28_e1171_d_n1: f64 = (p.p87 * eq28_e1170_d_n1);
        let eq28_e1171_d_n2: f64 = (p.p87 * eq28_e1170_d_n2);
        let eq28_e1171_d_n3: f64 = (p.p87 * eq28_e1170_d_n3);
        let eq28_e1171_d_n4: f64 = (p.p87 * eq28_e1170_d_n4);
        let eq28_e1171_d_n5: f64 = (p.p87 * eq28_e1170_d_n5);
        let eq28_e1171_d_n6: f64 = (p.p87 * eq28_e1170_d_n6);
        let eq28_e1171_d_n7: f64 = (p.p87 * eq28_e1170_d_n7);
        let eq28_e1171_d_n8: f64 = (p.p87 * eq28_e1170_d_n8);
        let eq28_e1171_d_n9: f64 = (p.p87 * eq28_e1170_d_n9);
        let eq28_e1171_d_n10: f64 = (p.p87 * eq28_e1170_d_n10);
        let eq28_e1171_d_n11: f64 = (p.p87 * eq28_e1170_d_n11);
        let eq28_e1171_d_n12: f64 = (p.p87 * eq28_e1170_d_n12);
        let eq28_e1171_d_n13: f64 = (p.p87 * eq28_e1170_d_n13);
        let eq28_e1171_d_n14: f64 = (p.p87 * eq28_e1170_d_n14);
        let eq28_e1171_d_n15: f64 = (p.p87 * eq28_e1170_d_n15);
        let eq28_e1171_d_n16: f64 = (p.p87 * eq28_e1170_d_n16);
        let eq28_e1171_d_n17: f64 = (p.p87 * eq28_e1170_d_n17);
        let eq28_e1171_d_b0: f64 = (p.p87 * eq28_e1170_d_b0);
        let eq28_e1171_d_b1: f64 = (p.p87 * eq28_e1170_d_b1);
        let eq28_e1171_d_b2: f64 = (p.p87 * eq28_e1170_d_b2);
        let eq28_e1171_d_b3: f64 = (p.p87 * eq28_e1170_d_b3);
        let eq28_e1171_d_b4: f64 = (p.p87 * eq28_e1170_d_b4);
        let eq28_e1171_d_b5: f64 = (p.p87 * eq28_e1170_d_b5);
        let eq28_e1171_d_b6: f64 = (p.p87 * eq28_e1170_d_b6);
        let eq28_e1171_d_b7: f64 = (p.p87 * eq28_e1170_d_b7);
        let eq28_e1171_d_b8: f64 = (p.p87 * eq28_e1170_d_b8);
        let eq28_e1171_d_b9: f64 = (p.p87 * eq28_e1170_d_b9);
        let eq28_e1171_d_b10: f64 = (p.p87 * eq28_e1170_d_b10);
        let eq28_e1171_d_b11: f64 = (p.p87 * eq28_e1170_d_b11);
        let eq28_value: f64 = eq28_e1171;
        let eq28_node_derivatives: [f64; 18] = [eq28_e1171_d_n0, eq28_e1171_d_n1, eq28_e1171_d_n2, eq28_e1171_d_n3, eq28_e1171_d_n4, eq28_e1171_d_n5, eq28_e1171_d_n6, eq28_e1171_d_n7, eq28_e1171_d_n8, eq28_e1171_d_n9, eq28_e1171_d_n10, eq28_e1171_d_n11, eq28_e1171_d_n12, eq28_e1171_d_n13, eq28_e1171_d_n14, eq28_e1171_d_n15, eq28_e1171_d_n16, eq28_e1171_d_n17];
        let eq28_branch_derivatives: [f64; 12] = [eq28_e1171_d_b0, eq28_e1171_d_b1, eq28_e1171_d_b2, eq28_e1171_d_b3, eq28_e1171_d_b4, eq28_e1171_d_b5, eq28_e1171_d_b6, eq28_e1171_d_b7, eq28_e1171_d_b8, eq28_e1171_d_b9, eq28_e1171_d_b10, eq28_e1171_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq29_e1176: f64 = (s.v[753] + s.v[751]);
        let eq29_e1176_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq29_e1176_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq29_e1176_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq29_e1176_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq29_e1176_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq29_e1176_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq29_e1176_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq29_e1176_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq29_e1176_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq29_e1176_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq29_e1176_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq29_e1176_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq29_e1176_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq29_e1176_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq29_e1176_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq29_e1176_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq29_e1176_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq29_e1176_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq29_e1176_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq29_e1176_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq29_e1176_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq29_e1176_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq29_e1176_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq29_e1176_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq29_e1176_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq29_e1176_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq29_e1176_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq29_e1176_d_b9: f64 = (s.db[753][9] + s.db[751][9]);
        let eq29_e1176_d_b10: f64 = (s.db[753][10] + s.db[751][10]);
        let eq29_e1176_d_b11: f64 = (s.db[753][11] + s.db[751][11]);
        let eq29_e1178: f64 = (eq29_e1176 + s.v[752]);
        let eq29_e1178_d_n0: f64 = (eq29_e1176_d_n0 + s.dn[752][0]);
        let eq29_e1178_d_n1: f64 = (eq29_e1176_d_n1 + s.dn[752][1]);
        let eq29_e1178_d_n2: f64 = (eq29_e1176_d_n2 + s.dn[752][2]);
        let eq29_e1178_d_n3: f64 = (eq29_e1176_d_n3 + s.dn[752][3]);
        let eq29_e1178_d_n4: f64 = (eq29_e1176_d_n4 + s.dn[752][4]);
        let eq29_e1178_d_n5: f64 = (eq29_e1176_d_n5 + s.dn[752][5]);
        let eq29_e1178_d_n6: f64 = (eq29_e1176_d_n6 + s.dn[752][6]);
        let eq29_e1178_d_n7: f64 = (eq29_e1176_d_n7 + s.dn[752][7]);
        let eq29_e1178_d_n8: f64 = (eq29_e1176_d_n8 + s.dn[752][8]);
        let eq29_e1178_d_n9: f64 = (eq29_e1176_d_n9 + s.dn[752][9]);
        let eq29_e1178_d_n10: f64 = (eq29_e1176_d_n10 + s.dn[752][10]);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + s.dn[752][11]);
        let eq29_e1178_d_n12: f64 = (eq29_e1176_d_n12 + s.dn[752][12]);
        let eq29_e1178_d_n13: f64 = (eq29_e1176_d_n13 + s.dn[752][13]);
        let eq29_e1178_d_n14: f64 = (eq29_e1176_d_n14 + s.dn[752][14]);
        let eq29_e1178_d_n15: f64 = (eq29_e1176_d_n15 + s.dn[752][15]);
        let eq29_e1178_d_n16: f64 = (eq29_e1176_d_n16 + s.dn[752][16]);
        let eq29_e1178_d_n17: f64 = (eq29_e1176_d_n17 + s.dn[752][17]);
        let eq29_e1178_d_b0: f64 = (eq29_e1176_d_b0 + s.db[752][0]);
        let eq29_e1178_d_b1: f64 = (eq29_e1176_d_b1 + s.db[752][1]);
        let eq29_e1178_d_b2: f64 = (eq29_e1176_d_b2 + s.db[752][2]);
        let eq29_e1178_d_b3: f64 = (eq29_e1176_d_b3 + s.db[752][3]);
        let eq29_e1178_d_b4: f64 = (eq29_e1176_d_b4 + s.db[752][4]);
        let eq29_e1178_d_b5: f64 = (eq29_e1176_d_b5 + s.db[752][5]);
        let eq29_e1178_d_b6: f64 = (eq29_e1176_d_b6 + s.db[752][6]);
        let eq29_e1178_d_b7: f64 = (eq29_e1176_d_b7 + s.db[752][7]);
        let eq29_e1178_d_b8: f64 = (eq29_e1176_d_b8 + s.db[752][8]);
        let eq29_e1178_d_b9: f64 = (eq29_e1176_d_b9 + s.db[752][9]);
        let eq29_e1178_d_b10: f64 = (eq29_e1176_d_b10 + s.db[752][10]);
        let eq29_e1178_d_b11: f64 = (eq29_e1176_d_b11 + s.db[752][11]);
        let eq29_e1179: f64 = (s.v[20] - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (s.dn[20][0] - eq29_e1178_d_n0);
        let eq29_e1179_d_n1: f64 = (s.dn[20][1] - eq29_e1178_d_n1);
        let eq29_e1179_d_n2: f64 = (s.dn[20][2] - eq29_e1178_d_n2);
        let eq29_e1179_d_n3: f64 = (s.dn[20][3] - eq29_e1178_d_n3);
        let eq29_e1179_d_n4: f64 = (s.dn[20][4] - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (s.dn[20][5] - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (s.dn[20][6] - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (s.dn[20][7] - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (s.dn[20][8] - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (s.dn[20][9] - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (s.dn[20][10] - eq29_e1178_d_n10);
        let eq29_e1179_d_n11: f64 = (s.dn[20][11] - eq29_e1178_d_n11);
        let eq29_e1179_d_n12: f64 = (s.dn[20][12] - eq29_e1178_d_n12);
        let eq29_e1179_d_n13: f64 = (s.dn[20][13] - eq29_e1178_d_n13);
        let eq29_e1179_d_n14: f64 = (s.dn[20][14] - eq29_e1178_d_n14);
        let eq29_e1179_d_n15: f64 = (s.dn[20][15] - eq29_e1178_d_n15);
        let eq29_e1179_d_n16: f64 = (s.dn[20][16] - eq29_e1178_d_n16);
        let eq29_e1179_d_n17: f64 = (s.dn[20][17] - eq29_e1178_d_n17);
        let eq29_e1179_d_b0: f64 = (s.db[20][0] - eq29_e1178_d_b0);
        let eq29_e1179_d_b1: f64 = (s.db[20][1] - eq29_e1178_d_b1);
        let eq29_e1179_d_b2: f64 = (s.db[20][2] - eq29_e1178_d_b2);
        let eq29_e1179_d_b3: f64 = (s.db[20][3] - eq29_e1178_d_b3);
        let eq29_e1179_d_b4: f64 = (s.db[20][4] - eq29_e1178_d_b4);
        let eq29_e1179_d_b5: f64 = (s.db[20][5] - eq29_e1178_d_b5);
        let eq29_e1179_d_b6: f64 = (s.db[20][6] - eq29_e1178_d_b6);
        let eq29_e1179_d_b7: f64 = (s.db[20][7] - eq29_e1178_d_b7);
        let eq29_e1179_d_b8: f64 = (s.db[20][8] - eq29_e1178_d_b8);
        let eq29_e1179_d_b9: f64 = (s.db[20][9] - eq29_e1178_d_b9);
        let eq29_e1179_d_b10: f64 = (s.db[20][10] - eq29_e1178_d_b10);
        let eq29_e1179_d_b11: f64 = (s.db[20][11] - eq29_e1178_d_b11);
        let eq29_e1180: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq29_e1179);
        let eq29_e1180_d_n0: f64 = (eq29_e1179_d_n0 * ddt_scale);
        let eq29_e1180_d_n1: f64 = (eq29_e1179_d_n1 * ddt_scale);
        let eq29_e1180_d_n2: f64 = (eq29_e1179_d_n2 * ddt_scale);
        let eq29_e1180_d_n3: f64 = (eq29_e1179_d_n3 * ddt_scale);
        let eq29_e1180_d_n4: f64 = (eq29_e1179_d_n4 * ddt_scale);
        let eq29_e1180_d_n5: f64 = (eq29_e1179_d_n5 * ddt_scale);
        let eq29_e1180_d_n6: f64 = (eq29_e1179_d_n6 * ddt_scale);
        let eq29_e1180_d_n7: f64 = (eq29_e1179_d_n7 * ddt_scale);
        let eq29_e1180_d_n8: f64 = (eq29_e1179_d_n8 * ddt_scale);
        let eq29_e1180_d_n9: f64 = (eq29_e1179_d_n9 * ddt_scale);
        let eq29_e1180_d_n10: f64 = (eq29_e1179_d_n10 * ddt_scale);
        let eq29_e1180_d_n11: f64 = (eq29_e1179_d_n11 * ddt_scale);
        let eq29_e1180_d_n12: f64 = (eq29_e1179_d_n12 * ddt_scale);
        let eq29_e1180_d_n13: f64 = (eq29_e1179_d_n13 * ddt_scale);
        let eq29_e1180_d_n14: f64 = (eq29_e1179_d_n14 * ddt_scale);
        let eq29_e1180_d_n15: f64 = (eq29_e1179_d_n15 * ddt_scale);
        let eq29_e1180_d_n16: f64 = (eq29_e1179_d_n16 * ddt_scale);
        let eq29_e1180_d_n17: f64 = (eq29_e1179_d_n17 * ddt_scale);
        let eq29_e1180_d_b0: f64 = (eq29_e1179_d_b0 * ddt_scale);
        let eq29_e1180_d_b1: f64 = (eq29_e1179_d_b1 * ddt_scale);
        let eq29_e1180_d_b2: f64 = (eq29_e1179_d_b2 * ddt_scale);
        let eq29_e1180_d_b3: f64 = (eq29_e1179_d_b3 * ddt_scale);
        let eq29_e1180_d_b4: f64 = (eq29_e1179_d_b4 * ddt_scale);
        let eq29_e1180_d_b5: f64 = (eq29_e1179_d_b5 * ddt_scale);
        let eq29_e1180_d_b6: f64 = (eq29_e1179_d_b6 * ddt_scale);
        let eq29_e1180_d_b7: f64 = (eq29_e1179_d_b7 * ddt_scale);
        let eq29_e1180_d_b8: f64 = (eq29_e1179_d_b8 * ddt_scale);
        let eq29_e1180_d_b9: f64 = (eq29_e1179_d_b9 * ddt_scale);
        let eq29_e1180_d_b10: f64 = (eq29_e1179_d_b10 * ddt_scale);
        let eq29_e1180_d_b11: f64 = (eq29_e1179_d_b11 * ddt_scale);
        let eq29_e1181: f64 = (p.p87 * eq29_e1180);
        let eq29_e1181_d_n0: f64 = (p.p87 * eq29_e1180_d_n0);
        let eq29_e1181_d_n1: f64 = (p.p87 * eq29_e1180_d_n1);
        let eq29_e1181_d_n2: f64 = (p.p87 * eq29_e1180_d_n2);
        let eq29_e1181_d_n3: f64 = (p.p87 * eq29_e1180_d_n3);
        let eq29_e1181_d_n4: f64 = (p.p87 * eq29_e1180_d_n4);
        let eq29_e1181_d_n5: f64 = (p.p87 * eq29_e1180_d_n5);
        let eq29_e1181_d_n6: f64 = (p.p87 * eq29_e1180_d_n6);
        let eq29_e1181_d_n7: f64 = (p.p87 * eq29_e1180_d_n7);
        let eq29_e1181_d_n8: f64 = (p.p87 * eq29_e1180_d_n8);
        let eq29_e1181_d_n9: f64 = (p.p87 * eq29_e1180_d_n9);
        let eq29_e1181_d_n10: f64 = (p.p87 * eq29_e1180_d_n10);
        let eq29_e1181_d_n11: f64 = (p.p87 * eq29_e1180_d_n11);
        let eq29_e1181_d_n12: f64 = (p.p87 * eq29_e1180_d_n12);
        let eq29_e1181_d_n13: f64 = (p.p87 * eq29_e1180_d_n13);
        let eq29_e1181_d_n14: f64 = (p.p87 * eq29_e1180_d_n14);
        let eq29_e1181_d_n15: f64 = (p.p87 * eq29_e1180_d_n15);
        let eq29_e1181_d_n16: f64 = (p.p87 * eq29_e1180_d_n16);
        let eq29_e1181_d_n17: f64 = (p.p87 * eq29_e1180_d_n17);
        let eq29_e1181_d_b0: f64 = (p.p87 * eq29_e1180_d_b0);
        let eq29_e1181_d_b1: f64 = (p.p87 * eq29_e1180_d_b1);
        let eq29_e1181_d_b2: f64 = (p.p87 * eq29_e1180_d_b2);
        let eq29_e1181_d_b3: f64 = (p.p87 * eq29_e1180_d_b3);
        let eq29_e1181_d_b4: f64 = (p.p87 * eq29_e1180_d_b4);
        let eq29_e1181_d_b5: f64 = (p.p87 * eq29_e1180_d_b5);
        let eq29_e1181_d_b6: f64 = (p.p87 * eq29_e1180_d_b6);
        let eq29_e1181_d_b7: f64 = (p.p87 * eq29_e1180_d_b7);
        let eq29_e1181_d_b8: f64 = (p.p87 * eq29_e1180_d_b8);
        let eq29_e1181_d_b9: f64 = (p.p87 * eq29_e1180_d_b9);
        let eq29_e1181_d_b10: f64 = (p.p87 * eq29_e1180_d_b10);
        let eq29_e1181_d_b11: f64 = (p.p87 * eq29_e1180_d_b11);
        let eq29_value: f64 = eq29_e1181;
        let eq29_node_derivatives: [f64; 18] = [eq29_e1181_d_n0, eq29_e1181_d_n1, eq29_e1181_d_n2, eq29_e1181_d_n3, eq29_e1181_d_n4, eq29_e1181_d_n5, eq29_e1181_d_n6, eq29_e1181_d_n7, eq29_e1181_d_n8, eq29_e1181_d_n9, eq29_e1181_d_n10, eq29_e1181_d_n11, eq29_e1181_d_n12, eq29_e1181_d_n13, eq29_e1181_d_n14, eq29_e1181_d_n15, eq29_e1181_d_n16, eq29_e1181_d_n17];
        let eq29_branch_derivatives: [f64; 12] = [eq29_e1181_d_b0, eq29_e1181_d_b1, eq29_e1181_d_b2, eq29_e1181_d_b3, eq29_e1181_d_b4, eq29_e1181_d_b5, eq29_e1181_d_b6, eq29_e1181_d_b7, eq29_e1181_d_b8, eq29_e1181_d_b9, eq29_e1181_d_b10, eq29_e1181_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[743]);
        let eq30_e1184_d_n0: f64 = (s.dn[743][0] * ddt_scale);
        let eq30_e1184_d_n1: f64 = (s.dn[743][1] * ddt_scale);
        let eq30_e1184_d_n2: f64 = (s.dn[743][2] * ddt_scale);
        let eq30_e1184_d_n3: f64 = (s.dn[743][3] * ddt_scale);
        let eq30_e1184_d_n4: f64 = (s.dn[743][4] * ddt_scale);
        let eq30_e1184_d_n5: f64 = (s.dn[743][5] * ddt_scale);
        let eq30_e1184_d_n6: f64 = (s.dn[743][6] * ddt_scale);
        let eq30_e1184_d_n7: f64 = (s.dn[743][7] * ddt_scale);
        let eq30_e1184_d_n8: f64 = (s.dn[743][8] * ddt_scale);
        let eq30_e1184_d_n9: f64 = (s.dn[743][9] * ddt_scale);
        let eq30_e1184_d_n10: f64 = (s.dn[743][10] * ddt_scale);
        let eq30_e1184_d_n11: f64 = (s.dn[743][11] * ddt_scale);
        let eq30_e1184_d_n12: f64 = (s.dn[743][12] * ddt_scale);
        let eq30_e1184_d_n13: f64 = (s.dn[743][13] * ddt_scale);
        let eq30_e1184_d_n14: f64 = (s.dn[743][14] * ddt_scale);
        let eq30_e1184_d_n15: f64 = (s.dn[743][15] * ddt_scale);
        let eq30_e1184_d_n16: f64 = (s.dn[743][16] * ddt_scale);
        let eq30_e1184_d_n17: f64 = (s.dn[743][17] * ddt_scale);
        let eq30_e1184_d_b0: f64 = (s.db[743][0] * ddt_scale);
        let eq30_e1184_d_b1: f64 = (s.db[743][1] * ddt_scale);
        let eq30_e1184_d_b2: f64 = (s.db[743][2] * ddt_scale);
        let eq30_e1184_d_b3: f64 = (s.db[743][3] * ddt_scale);
        let eq30_e1184_d_b4: f64 = (s.db[743][4] * ddt_scale);
        let eq30_e1184_d_b5: f64 = (s.db[743][5] * ddt_scale);
        let eq30_e1184_d_b6: f64 = (s.db[743][6] * ddt_scale);
        let eq30_e1184_d_b7: f64 = (s.db[743][7] * ddt_scale);
        let eq30_e1184_d_b8: f64 = (s.db[743][8] * ddt_scale);
        let eq30_e1184_d_b9: f64 = (s.db[743][9] * ddt_scale);
        let eq30_e1184_d_b10: f64 = (s.db[743][10] * ddt_scale);
        let eq30_e1184_d_b11: f64 = (s.db[743][11] * ddt_scale);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1184_d_n0);
        let eq30_e1185_d_n1: f64 = (p.p87 * eq30_e1184_d_n1);
        let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1184_d_n2);
        let eq30_e1185_d_n3: f64 = (p.p87 * eq30_e1184_d_n3);
        let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1184_d_n4);
        let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1184_d_n5);
        let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1184_d_n6);
        let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1184_d_n7);
        let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1184_d_n8);
        let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1184_d_n9);
        let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1184_d_n10);
        let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1184_d_n11);
        let eq30_e1185_d_n12: f64 = (p.p87 * eq30_e1184_d_n12);
        let eq30_e1185_d_n13: f64 = (p.p87 * eq30_e1184_d_n13);
        let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1184_d_n14);
        let eq30_e1185_d_n15: f64 = (p.p87 * eq30_e1184_d_n15);
        let eq30_e1185_d_n16: f64 = (p.p87 * eq30_e1184_d_n16);
        let eq30_e1185_d_n17: f64 = (p.p87 * eq30_e1184_d_n17);
        let eq30_e1185_d_b0: f64 = (p.p87 * eq30_e1184_d_b0);
        let eq30_e1185_d_b1: f64 = (p.p87 * eq30_e1184_d_b1);
        let eq30_e1185_d_b2: f64 = (p.p87 * eq30_e1184_d_b2);
        let eq30_e1185_d_b3: f64 = (p.p87 * eq30_e1184_d_b3);
        let eq30_e1185_d_b4: f64 = (p.p87 * eq30_e1184_d_b4);
        let eq30_e1185_d_b5: f64 = (p.p87 * eq30_e1184_d_b5);
        let eq30_e1185_d_b6: f64 = (p.p87 * eq30_e1184_d_b6);
        let eq30_e1185_d_b7: f64 = (p.p87 * eq30_e1184_d_b7);
        let eq30_e1185_d_b8: f64 = (p.p87 * eq30_e1184_d_b8);
        let eq30_e1185_d_b9: f64 = (p.p87 * eq30_e1184_d_b9);
        let eq30_e1185_d_b10: f64 = (p.p87 * eq30_e1184_d_b10);
        let eq30_e1185_d_b11: f64 = (p.p87 * eq30_e1184_d_b11);
        let eq30_value: f64 = eq30_e1185;
        let eq30_node_derivatives: [f64; 18] = [eq30_e1185_d_n0, eq30_e1185_d_n1, eq30_e1185_d_n2, eq30_e1185_d_n3, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, eq30_e1185_d_n15, eq30_e1185_d_n16, eq30_e1185_d_n17];
        let eq30_branch_derivatives: [f64; 12] = [eq30_e1185_d_b0, eq30_e1185_d_b1, eq30_e1185_d_b2, eq30_e1185_d_b3, eq30_e1185_d_b4, eq30_e1185_d_b5, eq30_e1185_d_b6, eq30_e1185_d_b7, eq30_e1185_d_b8, eq30_e1185_d_b9, eq30_e1185_d_b10, eq30_e1185_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[742]);
        let eq31_e1188_d_n0: f64 = (s.dn[742][0] * ddt_scale);
        let eq31_e1188_d_n1: f64 = (s.dn[742][1] * ddt_scale);
        let eq31_e1188_d_n2: f64 = (s.dn[742][2] * ddt_scale);
        let eq31_e1188_d_n3: f64 = (s.dn[742][3] * ddt_scale);
        let eq31_e1188_d_n4: f64 = (s.dn[742][4] * ddt_scale);
        let eq31_e1188_d_n5: f64 = (s.dn[742][5] * ddt_scale);
        let eq31_e1188_d_n6: f64 = (s.dn[742][6] * ddt_scale);
        let eq31_e1188_d_n7: f64 = (s.dn[742][7] * ddt_scale);
        let eq31_e1188_d_n8: f64 = (s.dn[742][8] * ddt_scale);
        let eq31_e1188_d_n9: f64 = (s.dn[742][9] * ddt_scale);
        let eq31_e1188_d_n10: f64 = (s.dn[742][10] * ddt_scale);
        let eq31_e1188_d_n11: f64 = (s.dn[742][11] * ddt_scale);
        let eq31_e1188_d_n12: f64 = (s.dn[742][12] * ddt_scale);
        let eq31_e1188_d_n13: f64 = (s.dn[742][13] * ddt_scale);
        let eq31_e1188_d_n14: f64 = (s.dn[742][14] * ddt_scale);
        let eq31_e1188_d_n15: f64 = (s.dn[742][15] * ddt_scale);
        let eq31_e1188_d_n16: f64 = (s.dn[742][16] * ddt_scale);
        let eq31_e1188_d_n17: f64 = (s.dn[742][17] * ddt_scale);
        let eq31_e1188_d_b0: f64 = (s.db[742][0] * ddt_scale);
        let eq31_e1188_d_b1: f64 = (s.db[742][1] * ddt_scale);
        let eq31_e1188_d_b2: f64 = (s.db[742][2] * ddt_scale);
        let eq31_e1188_d_b3: f64 = (s.db[742][3] * ddt_scale);
        let eq31_e1188_d_b4: f64 = (s.db[742][4] * ddt_scale);
        let eq31_e1188_d_b5: f64 = (s.db[742][5] * ddt_scale);
        let eq31_e1188_d_b6: f64 = (s.db[742][6] * ddt_scale);
        let eq31_e1188_d_b7: f64 = (s.db[742][7] * ddt_scale);
        let eq31_e1188_d_b8: f64 = (s.db[742][8] * ddt_scale);
        let eq31_e1188_d_b9: f64 = (s.db[742][9] * ddt_scale);
        let eq31_e1188_d_b10: f64 = (s.db[742][10] * ddt_scale);
        let eq31_e1188_d_b11: f64 = (s.db[742][11] * ddt_scale);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * eq31_e1188_d_n0);
        let eq31_e1189_d_n1: f64 = (p.p87 * eq31_e1188_d_n1);
        let eq31_e1189_d_n2: f64 = (p.p87 * eq31_e1188_d_n2);
        let eq31_e1189_d_n3: f64 = (p.p87 * eq31_e1188_d_n3);
        let eq31_e1189_d_n4: f64 = (p.p87 * eq31_e1188_d_n4);
        let eq31_e1189_d_n5: f64 = (p.p87 * eq31_e1188_d_n5);
        let eq31_e1189_d_n6: f64 = (p.p87 * eq31_e1188_d_n6);
        let eq31_e1189_d_n7: f64 = (p.p87 * eq31_e1188_d_n7);
        let eq31_e1189_d_n8: f64 = (p.p87 * eq31_e1188_d_n8);
        let eq31_e1189_d_n9: f64 = (p.p87 * eq31_e1188_d_n9);
        let eq31_e1189_d_n10: f64 = (p.p87 * eq31_e1188_d_n10);
        let eq31_e1189_d_n11: f64 = (p.p87 * eq31_e1188_d_n11);
        let eq31_e1189_d_n12: f64 = (p.p87 * eq31_e1188_d_n12);
        let eq31_e1189_d_n13: f64 = (p.p87 * eq31_e1188_d_n13);
        let eq31_e1189_d_n14: f64 = (p.p87 * eq31_e1188_d_n14);
        let eq31_e1189_d_n15: f64 = (p.p87 * eq31_e1188_d_n15);
        let eq31_e1189_d_n16: f64 = (p.p87 * eq31_e1188_d_n16);
        let eq31_e1189_d_n17: f64 = (p.p87 * eq31_e1188_d_n17);
        let eq31_e1189_d_b0: f64 = (p.p87 * eq31_e1188_d_b0);
        let eq31_e1189_d_b1: f64 = (p.p87 * eq31_e1188_d_b1);
        let eq31_e1189_d_b2: f64 = (p.p87 * eq31_e1188_d_b2);
        let eq31_e1189_d_b3: f64 = (p.p87 * eq31_e1188_d_b3);
        let eq31_e1189_d_b4: f64 = (p.p87 * eq31_e1188_d_b4);
        let eq31_e1189_d_b5: f64 = (p.p87 * eq31_e1188_d_b5);
        let eq31_e1189_d_b6: f64 = (p.p87 * eq31_e1188_d_b6);
        let eq31_e1189_d_b7: f64 = (p.p87 * eq31_e1188_d_b7);
        let eq31_e1189_d_b8: f64 = (p.p87 * eq31_e1188_d_b8);
        let eq31_e1189_d_b9: f64 = (p.p87 * eq31_e1188_d_b9);
        let eq31_e1189_d_b10: f64 = (p.p87 * eq31_e1188_d_b10);
        let eq31_e1189_d_b11: f64 = (p.p87 * eq31_e1188_d_b11);
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivatives: [f64; 18] = [eq31_e1189_d_n0, eq31_e1189_d_n1, eq31_e1189_d_n2, eq31_e1189_d_n3, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n12, eq31_e1189_d_n13, eq31_e1189_d_n14, eq31_e1189_d_n15, eq31_e1189_d_n16, eq31_e1189_d_n17];
        let eq31_branch_derivatives: [f64; 12] = [eq31_e1189_d_b0, eq31_e1189_d_b1, eq31_e1189_d_b2, eq31_e1189_d_b3, eq31_e1189_d_b4, eq31_e1189_d_b5, eq31_e1189_d_b6, eq31_e1189_d_b7, eq31_e1189_d_b8, eq31_e1189_d_b9, eq31_e1189_d_b10, eq31_e1189_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[744]);
        let eq32_e1192_d_n0: f64 = (s.dn[744][0] * ddt_scale);
        let eq32_e1192_d_n1: f64 = (s.dn[744][1] * ddt_scale);
        let eq32_e1192_d_n2: f64 = (s.dn[744][2] * ddt_scale);
        let eq32_e1192_d_n3: f64 = (s.dn[744][3] * ddt_scale);
        let eq32_e1192_d_n4: f64 = (s.dn[744][4] * ddt_scale);
        let eq32_e1192_d_n5: f64 = (s.dn[744][5] * ddt_scale);
        let eq32_e1192_d_n6: f64 = (s.dn[744][6] * ddt_scale);
        let eq32_e1192_d_n7: f64 = (s.dn[744][7] * ddt_scale);
        let eq32_e1192_d_n8: f64 = (s.dn[744][8] * ddt_scale);
        let eq32_e1192_d_n9: f64 = (s.dn[744][9] * ddt_scale);
        let eq32_e1192_d_n10: f64 = (s.dn[744][10] * ddt_scale);
        let eq32_e1192_d_n11: f64 = (s.dn[744][11] * ddt_scale);
        let eq32_e1192_d_n12: f64 = (s.dn[744][12] * ddt_scale);
        let eq32_e1192_d_n13: f64 = (s.dn[744][13] * ddt_scale);
        let eq32_e1192_d_n14: f64 = (s.dn[744][14] * ddt_scale);
        let eq32_e1192_d_n15: f64 = (s.dn[744][15] * ddt_scale);
        let eq32_e1192_d_n16: f64 = (s.dn[744][16] * ddt_scale);
        let eq32_e1192_d_n17: f64 = (s.dn[744][17] * ddt_scale);
        let eq32_e1192_d_b0: f64 = (s.db[744][0] * ddt_scale);
        let eq32_e1192_d_b1: f64 = (s.db[744][1] * ddt_scale);
        let eq32_e1192_d_b2: f64 = (s.db[744][2] * ddt_scale);
        let eq32_e1192_d_b3: f64 = (s.db[744][3] * ddt_scale);
        let eq32_e1192_d_b4: f64 = (s.db[744][4] * ddt_scale);
        let eq32_e1192_d_b5: f64 = (s.db[744][5] * ddt_scale);
        let eq32_e1192_d_b6: f64 = (s.db[744][6] * ddt_scale);
        let eq32_e1192_d_b7: f64 = (s.db[744][7] * ddt_scale);
        let eq32_e1192_d_b8: f64 = (s.db[744][8] * ddt_scale);
        let eq32_e1192_d_b9: f64 = (s.db[744][9] * ddt_scale);
        let eq32_e1192_d_b10: f64 = (s.db[744][10] * ddt_scale);
        let eq32_e1192_d_b11: f64 = (s.db[744][11] * ddt_scale);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * eq32_e1192_d_n0);
        let eq32_e1193_d_n1: f64 = (p.p87 * eq32_e1192_d_n1);
        let eq32_e1193_d_n2: f64 = (p.p87 * eq32_e1192_d_n2);
        let eq32_e1193_d_n3: f64 = (p.p87 * eq32_e1192_d_n3);
        let eq32_e1193_d_n4: f64 = (p.p87 * eq32_e1192_d_n4);
        let eq32_e1193_d_n5: f64 = (p.p87 * eq32_e1192_d_n5);
        let eq32_e1193_d_n6: f64 = (p.p87 * eq32_e1192_d_n6);
        let eq32_e1193_d_n7: f64 = (p.p87 * eq32_e1192_d_n7);
        let eq32_e1193_d_n8: f64 = (p.p87 * eq32_e1192_d_n8);
        let eq32_e1193_d_n9: f64 = (p.p87 * eq32_e1192_d_n9);
        let eq32_e1193_d_n10: f64 = (p.p87 * eq32_e1192_d_n10);
        let eq32_e1193_d_n11: f64 = (p.p87 * eq32_e1192_d_n11);
        let eq32_e1193_d_n12: f64 = (p.p87 * eq32_e1192_d_n12);
        let eq32_e1193_d_n13: f64 = (p.p87 * eq32_e1192_d_n13);
        let eq32_e1193_d_n14: f64 = (p.p87 * eq32_e1192_d_n14);
        let eq32_e1193_d_n15: f64 = (p.p87 * eq32_e1192_d_n15);
        let eq32_e1193_d_n16: f64 = (p.p87 * eq32_e1192_d_n16);
        let eq32_e1193_d_n17: f64 = (p.p87 * eq32_e1192_d_n17);
        let eq32_e1193_d_b0: f64 = (p.p87 * eq32_e1192_d_b0);
        let eq32_e1193_d_b1: f64 = (p.p87 * eq32_e1192_d_b1);
        let eq32_e1193_d_b2: f64 = (p.p87 * eq32_e1192_d_b2);
        let eq32_e1193_d_b3: f64 = (p.p87 * eq32_e1192_d_b3);
        let eq32_e1193_d_b4: f64 = (p.p87 * eq32_e1192_d_b4);
        let eq32_e1193_d_b5: f64 = (p.p87 * eq32_e1192_d_b5);
        let eq32_e1193_d_b6: f64 = (p.p87 * eq32_e1192_d_b6);
        let eq32_e1193_d_b7: f64 = (p.p87 * eq32_e1192_d_b7);
        let eq32_e1193_d_b8: f64 = (p.p87 * eq32_e1192_d_b8);
        let eq32_e1193_d_b9: f64 = (p.p87 * eq32_e1192_d_b9);
        let eq32_e1193_d_b10: f64 = (p.p87 * eq32_e1192_d_b10);
        let eq32_e1193_d_b11: f64 = (p.p87 * eq32_e1192_d_b11);
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivatives: [f64; 18] = [eq32_e1193_d_n0, eq32_e1193_d_n1, eq32_e1193_d_n2, eq32_e1193_d_n3, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n12, eq32_e1193_d_n13, eq32_e1193_d_n14, eq32_e1193_d_n15, eq32_e1193_d_n16, eq32_e1193_d_n17];
        let eq32_branch_derivatives: [f64; 12] = [eq32_e1193_d_b0, eq32_e1193_d_b1, eq32_e1193_d_b2, eq32_e1193_d_b3, eq32_e1193_d_b4, eq32_e1193_d_b5, eq32_e1193_d_b6, eq32_e1193_d_b7, eq32_e1193_d_b8, eq32_e1193_d_b9, eq32_e1193_d_b10, eq32_e1193_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[299]);
        let eq33_e1197_d_n0: f64 = (s.dn[299][0] * ddt_scale);
        let eq33_e1197_d_n1: f64 = (s.dn[299][1] * ddt_scale);
        let eq33_e1197_d_n2: f64 = (s.dn[299][2] * ddt_scale);
        let eq33_e1197_d_n3: f64 = (s.dn[299][3] * ddt_scale);
        let eq33_e1197_d_n4: f64 = (s.dn[299][4] * ddt_scale);
        let eq33_e1197_d_n5: f64 = (s.dn[299][5] * ddt_scale);
        let eq33_e1197_d_n6: f64 = (s.dn[299][6] * ddt_scale);
        let eq33_e1197_d_n7: f64 = (s.dn[299][7] * ddt_scale);
        let eq33_e1197_d_n8: f64 = (s.dn[299][8] * ddt_scale);
        let eq33_e1197_d_n9: f64 = (s.dn[299][9] * ddt_scale);
        let eq33_e1197_d_n10: f64 = (s.dn[299][10] * ddt_scale);
        let eq33_e1197_d_n11: f64 = (s.dn[299][11] * ddt_scale);
        let eq33_e1197_d_n12: f64 = (s.dn[299][12] * ddt_scale);
        let eq33_e1197_d_n13: f64 = (s.dn[299][13] * ddt_scale);
        let eq33_e1197_d_n14: f64 = (s.dn[299][14] * ddt_scale);
        let eq33_e1197_d_n15: f64 = (s.dn[299][15] * ddt_scale);
        let eq33_e1197_d_n16: f64 = (s.dn[299][16] * ddt_scale);
        let eq33_e1197_d_n17: f64 = (s.dn[299][17] * ddt_scale);
        let eq33_e1197_d_b0: f64 = (s.db[299][0] * ddt_scale);
        let eq33_e1197_d_b1: f64 = (s.db[299][1] * ddt_scale);
        let eq33_e1197_d_b2: f64 = (s.db[299][2] * ddt_scale);
        let eq33_e1197_d_b3: f64 = (s.db[299][3] * ddt_scale);
        let eq33_e1197_d_b4: f64 = (s.db[299][4] * ddt_scale);
        let eq33_e1197_d_b5: f64 = (s.db[299][5] * ddt_scale);
        let eq33_e1197_d_b6: f64 = (s.db[299][6] * ddt_scale);
        let eq33_e1197_d_b7: f64 = (s.db[299][7] * ddt_scale);
        let eq33_e1197_d_b8: f64 = (s.db[299][8] * ddt_scale);
        let eq33_e1197_d_b9: f64 = (s.db[299][9] * ddt_scale);
        let eq33_e1197_d_b10: f64 = (s.db[299][10] * ddt_scale);
        let eq33_e1197_d_b11: f64 = (s.db[299][11] * ddt_scale);
        let eq33_e1198: f64 = (eq33_e1195 * eq33_e1197);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * eq33_e1197_d_n0);
        let eq33_e1198_d_n1: f64 = (eq33_e1195 * eq33_e1197_d_n1);
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * eq33_e1197_d_n2);
        let eq33_e1198_d_n3: f64 = (eq33_e1195 * eq33_e1197_d_n3);
        let eq33_e1198_d_n4: f64 = (eq33_e1195 * eq33_e1197_d_n4);
        let eq33_e1198_d_n5: f64 = (eq33_e1195 * eq33_e1197_d_n5);
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * eq33_e1197_d_n6);
        let eq33_e1198_d_n7: f64 = (eq33_e1195 * eq33_e1197_d_n7);
        let eq33_e1198_d_n8: f64 = (eq33_e1195 * eq33_e1197_d_n8);
        let eq33_e1198_d_n9: f64 = (eq33_e1195 * eq33_e1197_d_n9);
        let eq33_e1198_d_n10: f64 = (eq33_e1195 * eq33_e1197_d_n10);
        let eq33_e1198_d_n11: f64 = (eq33_e1195 * eq33_e1197_d_n11);
        let eq33_e1198_d_n12: f64 = (eq33_e1195 * eq33_e1197_d_n12);
        let eq33_e1198_d_n13: f64 = (eq33_e1195 * eq33_e1197_d_n13);
        let eq33_e1198_d_n14: f64 = (eq33_e1195 * eq33_e1197_d_n14);
        let eq33_e1198_d_n15: f64 = (eq33_e1195 * eq33_e1197_d_n15);
        let eq33_e1198_d_n16: f64 = (eq33_e1195 * eq33_e1197_d_n16);
        let eq33_e1198_d_n17: f64 = (eq33_e1195 * eq33_e1197_d_n17);
        let eq33_e1198_d_b0: f64 = (eq33_e1195 * eq33_e1197_d_b0);
        let eq33_e1198_d_b1: f64 = (eq33_e1195 * eq33_e1197_d_b1);
        let eq33_e1198_d_b2: f64 = (eq33_e1195 * eq33_e1197_d_b2);
        let eq33_e1198_d_b3: f64 = (eq33_e1195 * eq33_e1197_d_b3);
        let eq33_e1198_d_b4: f64 = (eq33_e1195 * eq33_e1197_d_b4);
        let eq33_e1198_d_b5: f64 = (eq33_e1195 * eq33_e1197_d_b5);
        let eq33_e1198_d_b6: f64 = (eq33_e1195 * eq33_e1197_d_b6);
        let eq33_e1198_d_b7: f64 = (eq33_e1195 * eq33_e1197_d_b7);
        let eq33_e1198_d_b8: f64 = (eq33_e1195 * eq33_e1197_d_b8);
        let eq33_e1198_d_b9: f64 = (eq33_e1195 * eq33_e1197_d_b9);
        let eq33_e1198_d_b10: f64 = (eq33_e1195 * eq33_e1197_d_b10);
        let eq33_e1198_d_b11: f64 = (eq33_e1195 * eq33_e1197_d_b11);
        let eq33_value: f64 = eq33_e1198;
        let eq33_node_derivatives: [f64; 18] = [eq33_e1198_d_n0, eq33_e1198_d_n1, eq33_e1198_d_n2, eq33_e1198_d_n3, eq33_e1198_d_n4, eq33_e1198_d_n5, eq33_e1198_d_n6, eq33_e1198_d_n7, eq33_e1198_d_n8, eq33_e1198_d_n9, eq33_e1198_d_n10, eq33_e1198_d_n11, eq33_e1198_d_n12, eq33_e1198_d_n13, eq33_e1198_d_n14, eq33_e1198_d_n15, eq33_e1198_d_n16, eq33_e1198_d_n17];
        let eq33_branch_derivatives: [f64; 12] = [eq33_e1198_d_b0, eq33_e1198_d_b1, eq33_e1198_d_b2, eq33_e1198_d_b3, eq33_e1198_d_b4, eq33_e1198_d_b5, eq33_e1198_d_b6, eq33_e1198_d_b7, eq33_e1198_d_b8, eq33_e1198_d_b9, eq33_e1198_d_b10, eq33_e1198_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(0),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[301]);
        let eq34_e1202_d_n0: f64 = (s.dn[301][0] * ddt_scale);
        let eq34_e1202_d_n1: f64 = (s.dn[301][1] * ddt_scale);
        let eq34_e1202_d_n2: f64 = (s.dn[301][2] * ddt_scale);
        let eq34_e1202_d_n3: f64 = (s.dn[301][3] * ddt_scale);
        let eq34_e1202_d_n4: f64 = (s.dn[301][4] * ddt_scale);
        let eq34_e1202_d_n5: f64 = (s.dn[301][5] * ddt_scale);
        let eq34_e1202_d_n6: f64 = (s.dn[301][6] * ddt_scale);
        let eq34_e1202_d_n7: f64 = (s.dn[301][7] * ddt_scale);
        let eq34_e1202_d_n8: f64 = (s.dn[301][8] * ddt_scale);
        let eq34_e1202_d_n9: f64 = (s.dn[301][9] * ddt_scale);
        let eq34_e1202_d_n10: f64 = (s.dn[301][10] * ddt_scale);
        let eq34_e1202_d_n11: f64 = (s.dn[301][11] * ddt_scale);
        let eq34_e1202_d_n12: f64 = (s.dn[301][12] * ddt_scale);
        let eq34_e1202_d_n13: f64 = (s.dn[301][13] * ddt_scale);
        let eq34_e1202_d_n14: f64 = (s.dn[301][14] * ddt_scale);
        let eq34_e1202_d_n15: f64 = (s.dn[301][15] * ddt_scale);
        let eq34_e1202_d_n16: f64 = (s.dn[301][16] * ddt_scale);
        let eq34_e1202_d_n17: f64 = (s.dn[301][17] * ddt_scale);
        let eq34_e1202_d_b0: f64 = (s.db[301][0] * ddt_scale);
        let eq34_e1202_d_b1: f64 = (s.db[301][1] * ddt_scale);
        let eq34_e1202_d_b2: f64 = (s.db[301][2] * ddt_scale);
        let eq34_e1202_d_b3: f64 = (s.db[301][3] * ddt_scale);
        let eq34_e1202_d_b4: f64 = (s.db[301][4] * ddt_scale);
        let eq34_e1202_d_b5: f64 = (s.db[301][5] * ddt_scale);
        let eq34_e1202_d_b6: f64 = (s.db[301][6] * ddt_scale);
        let eq34_e1202_d_b7: f64 = (s.db[301][7] * ddt_scale);
        let eq34_e1202_d_b8: f64 = (s.db[301][8] * ddt_scale);
        let eq34_e1202_d_b9: f64 = (s.db[301][9] * ddt_scale);
        let eq34_e1202_d_b10: f64 = (s.db[301][10] * ddt_scale);
        let eq34_e1202_d_b11: f64 = (s.db[301][11] * ddt_scale);
        let eq34_e1203: f64 = (eq34_e1200 * eq34_e1202);
        let eq34_e1203_d_n0: f64 = (eq34_e1200 * eq34_e1202_d_n0);
        let eq34_e1203_d_n1: f64 = (eq34_e1200 * eq34_e1202_d_n1);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * eq34_e1202_d_n2);
        let eq34_e1203_d_n3: f64 = (eq34_e1200 * eq34_e1202_d_n3);
        let eq34_e1203_d_n4: f64 = (eq34_e1200 * eq34_e1202_d_n4);
        let eq34_e1203_d_n5: f64 = (eq34_e1200 * eq34_e1202_d_n5);
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * eq34_e1202_d_n6);
        let eq34_e1203_d_n7: f64 = (eq34_e1200 * eq34_e1202_d_n7);
        let eq34_e1203_d_n8: f64 = (eq34_e1200 * eq34_e1202_d_n8);
        let eq34_e1203_d_n9: f64 = (eq34_e1200 * eq34_e1202_d_n9);
        let eq34_e1203_d_n10: f64 = (eq34_e1200 * eq34_e1202_d_n10);
        let eq34_e1203_d_n11: f64 = (eq34_e1200 * eq34_e1202_d_n11);
        let eq34_e1203_d_n12: f64 = (eq34_e1200 * eq34_e1202_d_n12);
        let eq34_e1203_d_n13: f64 = (eq34_e1200 * eq34_e1202_d_n13);
        let eq34_e1203_d_n14: f64 = (eq34_e1200 * eq34_e1202_d_n14);
        let eq34_e1203_d_n15: f64 = (eq34_e1200 * eq34_e1202_d_n15);
        let eq34_e1203_d_n16: f64 = (eq34_e1200 * eq34_e1202_d_n16);
        let eq34_e1203_d_n17: f64 = (eq34_e1200 * eq34_e1202_d_n17);
        let eq34_e1203_d_b0: f64 = (eq34_e1200 * eq34_e1202_d_b0);
        let eq34_e1203_d_b1: f64 = (eq34_e1200 * eq34_e1202_d_b1);
        let eq34_e1203_d_b2: f64 = (eq34_e1200 * eq34_e1202_d_b2);
        let eq34_e1203_d_b3: f64 = (eq34_e1200 * eq34_e1202_d_b3);
        let eq34_e1203_d_b4: f64 = (eq34_e1200 * eq34_e1202_d_b4);
        let eq34_e1203_d_b5: f64 = (eq34_e1200 * eq34_e1202_d_b5);
        let eq34_e1203_d_b6: f64 = (eq34_e1200 * eq34_e1202_d_b6);
        let eq34_e1203_d_b7: f64 = (eq34_e1200 * eq34_e1202_d_b7);
        let eq34_e1203_d_b8: f64 = (eq34_e1200 * eq34_e1202_d_b8);
        let eq34_e1203_d_b9: f64 = (eq34_e1200 * eq34_e1202_d_b9);
        let eq34_e1203_d_b10: f64 = (eq34_e1200 * eq34_e1202_d_b10);
        let eq34_e1203_d_b11: f64 = (eq34_e1200 * eq34_e1202_d_b11);
        let eq34_value: f64 = eq34_e1203;
        let eq34_node_derivatives: [f64; 18] = [eq34_e1203_d_n0, eq34_e1203_d_n1, eq34_e1203_d_n2, eq34_e1203_d_n3, eq34_e1203_d_n4, eq34_e1203_d_n5, eq34_e1203_d_n6, eq34_e1203_d_n7, eq34_e1203_d_n8, eq34_e1203_d_n9, eq34_e1203_d_n10, eq34_e1203_d_n11, eq34_e1203_d_n12, eq34_e1203_d_n13, eq34_e1203_d_n14, eq34_e1203_d_n15, eq34_e1203_d_n16, eq34_e1203_d_n17];
        let eq34_branch_derivatives: [f64; 12] = [eq34_e1203_d_b0, eq34_e1203_d_b1, eq34_e1203_d_b2, eq34_e1203_d_b3, eq34_e1203_d_b4, eq34_e1203_d_b5, eq34_e1203_d_b6, eq34_e1203_d_b7, eq34_e1203_d_b8, eq34_e1203_d_b9, eq34_e1203_d_b10, eq34_e1203_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq35_value),
        );
        let eq36_e1214: f64 = (nv14 - 0.0);
        let eq36_value: f64 = eq36_e1214;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq36_value),
            14,
            multiplicity * (1.0),
        );
        let eq37_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (eq37_value),
        );
        let eq38_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq38_value),
        );
        let eq39_e1229: f64 = (s.v[951] * (nv14 - 0.0));
        let eq39_e1229_d_n0: f64 = (s.dn[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_n1: f64 = (s.dn[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_n2: f64 = (s.dn[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_n3: f64 = (s.dn[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_n4: f64 = (s.dn[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_n5: f64 = (s.dn[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_n6: f64 = (s.dn[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_n7: f64 = (s.dn[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_n8: f64 = (s.dn[951][8] * (nv14 - 0.0));
        let eq39_e1229_d_n9: f64 = (s.dn[951][9] * (nv14 - 0.0));
        let eq39_e1229_d_n10: f64 = (s.dn[951][10] * (nv14 - 0.0));
        let eq39_e1229_d_n11: f64 = (s.dn[951][11] * (nv14 - 0.0));
        let eq39_e1229_d_n12: f64 = (s.dn[951][12] * (nv14 - 0.0));
        let eq39_e1229_d_n13: f64 = (s.dn[951][13] * (nv14 - 0.0));
        let eq39_e1229_d_n14: f64 = ((s.dn[951][14] * (nv14 - 0.0)) + s.v[951]);
        let eq39_e1229_d_n15: f64 = (s.dn[951][15] * (nv14 - 0.0));
        let eq39_e1229_d_n16: f64 = (s.dn[951][16] * (nv14 - 0.0));
        let eq39_e1229_d_n17: f64 = (s.dn[951][17] * (nv14 - 0.0));
        let eq39_e1229_d_b0: f64 = (s.db[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_b1: f64 = (s.db[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_b2: f64 = (s.db[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_b3: f64 = (s.db[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_b4: f64 = (s.db[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_b5: f64 = (s.db[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_b6: f64 = (s.db[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_b7: f64 = (s.db[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_b8: f64 = (s.db[951][8] * (nv14 - 0.0));
        let eq39_e1229_d_b9: f64 = (s.db[951][9] * (nv14 - 0.0));
        let eq39_e1229_d_b10: f64 = (s.db[951][10] * (nv14 - 0.0));
        let eq39_e1229_d_b11: f64 = (s.db[951][11] * (nv14 - 0.0));
        let eq39_value: f64 = eq39_e1229;
        let eq39_node_derivatives: [f64; 18] = [eq39_e1229_d_n0, eq39_e1229_d_n1, eq39_e1229_d_n2, eq39_e1229_d_n3, eq39_e1229_d_n4, eq39_e1229_d_n5, eq39_e1229_d_n6, eq39_e1229_d_n7, eq39_e1229_d_n8, eq39_e1229_d_n9, eq39_e1229_d_n10, eq39_e1229_d_n11, eq39_e1229_d_n12, eq39_e1229_d_n13, eq39_e1229_d_n14, eq39_e1229_d_n15, eq39_e1229_d_n16, eq39_e1229_d_n17];
        let eq39_branch_derivatives: [f64; 12] = [eq39_e1229_d_b0, eq39_e1229_d_b1, eq39_e1229_d_b2, eq39_e1229_d_b3, eq39_e1229_d_b4, eq39_e1229_d_b5, eq39_e1229_d_b6, eq39_e1229_d_b7, eq39_e1229_d_b8, eq39_e1229_d_b9, eq39_e1229_d_b10, eq39_e1229_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1232_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);
        let eq40_e1232_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);
        let eq40_e1232_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);
        let eq40_e1233: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq40_e1232);
        let eq40_e1233_d_n0: f64 = (eq40_e1232_d_n0 * ddt_scale);
        let eq40_e1233_d_n1: f64 = (eq40_e1232_d_n1 * ddt_scale);
        let eq40_e1233_d_n2: f64 = (eq40_e1232_d_n2 * ddt_scale);
        let eq40_e1233_d_n3: f64 = (eq40_e1232_d_n3 * ddt_scale);
        let eq40_e1233_d_n4: f64 = (eq40_e1232_d_n4 * ddt_scale);
        let eq40_e1233_d_n5: f64 = (eq40_e1232_d_n5 * ddt_scale);
        let eq40_e1233_d_n6: f64 = (eq40_e1232_d_n6 * ddt_scale);
        let eq40_e1233_d_n7: f64 = (eq40_e1232_d_n7 * ddt_scale);
        let eq40_e1233_d_n8: f64 = (eq40_e1232_d_n8 * ddt_scale);
        let eq40_e1233_d_n9: f64 = (eq40_e1232_d_n9 * ddt_scale);
        let eq40_e1233_d_n10: f64 = (eq40_e1232_d_n10 * ddt_scale);
        let eq40_e1233_d_n11: f64 = (eq40_e1232_d_n11 * ddt_scale);
        let eq40_e1233_d_n12: f64 = (eq40_e1232_d_n12 * ddt_scale);
        let eq40_e1233_d_n13: f64 = (eq40_e1232_d_n13 * ddt_scale);
        let eq40_e1233_d_n14: f64 = (eq40_e1232_d_n14 * ddt_scale);
        let eq40_e1233_d_n15: f64 = (eq40_e1232_d_n15 * ddt_scale);
        let eq40_e1233_d_n16: f64 = (eq40_e1232_d_n16 * ddt_scale);
        let eq40_e1233_d_n17: f64 = (eq40_e1232_d_n17 * ddt_scale);
        let eq40_e1233_d_b0: f64 = (eq40_e1232_d_b0 * ddt_scale);
        let eq40_e1233_d_b1: f64 = (eq40_e1232_d_b1 * ddt_scale);
        let eq40_e1233_d_b2: f64 = (eq40_e1232_d_b2 * ddt_scale);
        let eq40_e1233_d_b3: f64 = (eq40_e1232_d_b3 * ddt_scale);
        let eq40_e1233_d_b4: f64 = (eq40_e1232_d_b4 * ddt_scale);
        let eq40_e1233_d_b5: f64 = (eq40_e1232_d_b5 * ddt_scale);
        let eq40_e1233_d_b6: f64 = (eq40_e1232_d_b6 * ddt_scale);
        let eq40_e1233_d_b7: f64 = (eq40_e1232_d_b7 * ddt_scale);
        let eq40_e1233_d_b8: f64 = (eq40_e1232_d_b8 * ddt_scale);
        let eq40_e1233_d_b9: f64 = (eq40_e1232_d_b9 * ddt_scale);
        let eq40_e1233_d_b10: f64 = (eq40_e1232_d_b10 * ddt_scale);
        let eq40_e1233_d_b11: f64 = (eq40_e1232_d_b11 * ddt_scale);
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivatives: [f64; 18] = [eq40_e1233_d_n0, eq40_e1233_d_n1, eq40_e1233_d_n2, eq40_e1233_d_n3, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n12, eq40_e1233_d_n13, eq40_e1233_d_n14, eq40_e1233_d_n15, eq40_e1233_d_n16, eq40_e1233_d_n17];
        let eq40_branch_derivatives: [f64; 12] = [eq40_e1233_d_b0, eq40_e1233_d_b1, eq40_e1233_d_b2, eq40_e1233_d_b3, eq40_e1233_d_b4, eq40_e1233_d_b5, eq40_e1233_d_b6, eq40_e1233_d_b7, eq40_e1233_d_b8, eq40_e1233_d_b9, eq40_e1233_d_b10, eq40_e1233_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1236_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);
        let eq41_e1236_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);
        let eq41_e1236_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq41_e1236);
        let eq41_e1237_d_n0: f64 = (eq41_e1236_d_n0 * ddt_scale);
        let eq41_e1237_d_n1: f64 = (eq41_e1236_d_n1 * ddt_scale);
        let eq41_e1237_d_n2: f64 = (eq41_e1236_d_n2 * ddt_scale);
        let eq41_e1237_d_n3: f64 = (eq41_e1236_d_n3 * ddt_scale);
        let eq41_e1237_d_n4: f64 = (eq41_e1236_d_n4 * ddt_scale);
        let eq41_e1237_d_n5: f64 = (eq41_e1236_d_n5 * ddt_scale);
        let eq41_e1237_d_n6: f64 = (eq41_e1236_d_n6 * ddt_scale);
        let eq41_e1237_d_n7: f64 = (eq41_e1236_d_n7 * ddt_scale);
        let eq41_e1237_d_n8: f64 = (eq41_e1236_d_n8 * ddt_scale);
        let eq41_e1237_d_n9: f64 = (eq41_e1236_d_n9 * ddt_scale);
        let eq41_e1237_d_n10: f64 = (eq41_e1236_d_n10 * ddt_scale);
        let eq41_e1237_d_n11: f64 = (eq41_e1236_d_n11 * ddt_scale);
        let eq41_e1237_d_n12: f64 = (eq41_e1236_d_n12 * ddt_scale);
        let eq41_e1237_d_n13: f64 = (eq41_e1236_d_n13 * ddt_scale);
        let eq41_e1237_d_n14: f64 = (eq41_e1236_d_n14 * ddt_scale);
        let eq41_e1237_d_n15: f64 = (eq41_e1236_d_n15 * ddt_scale);
        let eq41_e1237_d_n16: f64 = (eq41_e1236_d_n16 * ddt_scale);
        let eq41_e1237_d_n17: f64 = (eq41_e1236_d_n17 * ddt_scale);
        let eq41_e1237_d_b0: f64 = (eq41_e1236_d_b0 * ddt_scale);
        let eq41_e1237_d_b1: f64 = (eq41_e1236_d_b1 * ddt_scale);
        let eq41_e1237_d_b2: f64 = (eq41_e1236_d_b2 * ddt_scale);
        let eq41_e1237_d_b3: f64 = (eq41_e1236_d_b3 * ddt_scale);
        let eq41_e1237_d_b4: f64 = (eq41_e1236_d_b4 * ddt_scale);
        let eq41_e1237_d_b5: f64 = (eq41_e1236_d_b5 * ddt_scale);
        let eq41_e1237_d_b6: f64 = (eq41_e1236_d_b6 * ddt_scale);
        let eq41_e1237_d_b7: f64 = (eq41_e1236_d_b7 * ddt_scale);
        let eq41_e1237_d_b8: f64 = (eq41_e1236_d_b8 * ddt_scale);
        let eq41_e1237_d_b9: f64 = (eq41_e1236_d_b9 * ddt_scale);
        let eq41_e1237_d_b10: f64 = (eq41_e1236_d_b10 * ddt_scale);
        let eq41_e1237_d_b11: f64 = (eq41_e1236_d_b11 * ddt_scale);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivatives: [f64; 18] = [eq41_e1237_d_n0, eq41_e1237_d_n1, eq41_e1237_d_n2, eq41_e1237_d_n3, eq41_e1237_d_n4, eq41_e1237_d_n5, eq41_e1237_d_n6, eq41_e1237_d_n7, eq41_e1237_d_n8, eq41_e1237_d_n9, eq41_e1237_d_n10, eq41_e1237_d_n11, eq41_e1237_d_n12, eq41_e1237_d_n13, eq41_e1237_d_n14, eq41_e1237_d_n15, eq41_e1237_d_n16, eq41_e1237_d_n17];
        let eq41_branch_derivatives: [f64; 12] = [eq41_e1237_d_b0, eq41_e1237_d_b1, eq41_e1237_d_b2, eq41_e1237_d_b3, eq41_e1237_d_b4, eq41_e1237_d_b5, eq41_e1237_d_b6, eq41_e1237_d_b7, eq41_e1237_d_b8, eq41_e1237_d_b9, eq41_e1237_d_b10, eq41_e1237_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq42_e1245,) = {
    if (s.v[76] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e1245;
        stamper.stamp_current_const_local(
            Some(7),
            Some(2),
            multiplicity * (eq42_value),
        );
        let (eq43_e1253,) = {
    if (s.v[75] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq43_value: f64 = eq43_e1253;
        stamper.stamp_current_const_local(
            Some(0),
            Some(5),
            multiplicity * (eq43_value),
        );
        let eq44_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (eq44_value),
        );
        let eq45_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (eq45_value),
        );
        let eq46_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(8),
            multiplicity * (eq46_value),
        );
        let (eq47_e1283, eq47_e1283_d_n0, eq47_e1283_d_n1, eq47_e1283_d_n2, eq47_e1283_d_n3, eq47_e1283_d_n4, eq47_e1283_d_n5, eq47_e1283_d_n6, eq47_e1283_d_n7, eq47_e1283_d_n8, eq47_e1283_d_n9, eq47_e1283_d_n10, eq47_e1283_d_n11, eq47_e1283_d_n12, eq47_e1283_d_n13, eq47_e1283_d_n14, eq47_e1283_d_n15, eq47_e1283_d_n16, eq47_e1283_d_n17, eq47_e1283_d_b0, eq47_e1283_d_b1, eq47_e1283_d_b2, eq47_e1283_d_b3, eq47_e1283_d_b4, eq47_e1283_d_b5, eq47_e1283_d_b6, eq47_e1283_d_b7, eq47_e1283_d_b8, eq47_e1283_d_b9, eq47_e1283_d_b10, eq47_e1283_d_b11,) = {
    if s.b[3408] {
        let eq47_e1281: f64 = (s.v[643] * (nv1 - nv6));
        let eq47_e1281_d_n0: f64 = (s.dn[643][0] * (nv1 - nv6));
        let eq47_e1281_d_n1: f64 = ((s.dn[643][1] * (nv1 - nv6)) + s.v[643]);
        let eq47_e1281_d_n2: f64 = (s.dn[643][2] * (nv1 - nv6));
        let eq47_e1281_d_n3: f64 = (s.dn[643][3] * (nv1 - nv6));
        let eq47_e1281_d_n4: f64 = (s.dn[643][4] * (nv1 - nv6));
        let eq47_e1281_d_n5: f64 = (s.dn[643][5] * (nv1 - nv6));
        let eq47_e1281_d_n6: f64 = ((s.dn[643][6] * (nv1 - nv6)) + (-s.v[643]));
        let eq47_e1281_d_n7: f64 = (s.dn[643][7] * (nv1 - nv6));
        let eq47_e1281_d_n8: f64 = (s.dn[643][8] * (nv1 - nv6));
        let eq47_e1281_d_n9: f64 = (s.dn[643][9] * (nv1 - nv6));
        let eq47_e1281_d_n10: f64 = (s.dn[643][10] * (nv1 - nv6));
        let eq47_e1281_d_n11: f64 = (s.dn[643][11] * (nv1 - nv6));
        let eq47_e1281_d_n12: f64 = (s.dn[643][12] * (nv1 - nv6));
        let eq47_e1281_d_n13: f64 = (s.dn[643][13] * (nv1 - nv6));
        let eq47_e1281_d_n14: f64 = (s.dn[643][14] * (nv1 - nv6));
        let eq47_e1281_d_n15: f64 = (s.dn[643][15] * (nv1 - nv6));
        let eq47_e1281_d_n16: f64 = (s.dn[643][16] * (nv1 - nv6));
        let eq47_e1281_d_n17: f64 = (s.dn[643][17] * (nv1 - nv6));
        let eq47_e1281_d_b0: f64 = (s.db[643][0] * (nv1 - nv6));
        let eq47_e1281_d_b1: f64 = (s.db[643][1] * (nv1 - nv6));
        let eq47_e1281_d_b2: f64 = (s.db[643][2] * (nv1 - nv6));
        let eq47_e1281_d_b3: f64 = (s.db[643][3] * (nv1 - nv6));
        let eq47_e1281_d_b4: f64 = (s.db[643][4] * (nv1 - nv6));
        let eq47_e1281_d_b5: f64 = (s.db[643][5] * (nv1 - nv6));
        let eq47_e1281_d_b6: f64 = (s.db[643][6] * (nv1 - nv6));
        let eq47_e1281_d_b7: f64 = (s.db[643][7] * (nv1 - nv6));
        let eq47_e1281_d_b8: f64 = (s.db[643][8] * (nv1 - nv6));
        let eq47_e1281_d_b9: f64 = (s.db[643][9] * (nv1 - nv6));
        let eq47_e1281_d_b10: f64 = (s.db[643][10] * (nv1 - nv6));
        let eq47_e1281_d_b11: f64 = (s.db[643][11] * (nv1 - nv6));
        (eq47_e1281, eq47_e1281_d_n0, eq47_e1281_d_n1, eq47_e1281_d_n2, eq47_e1281_d_n3, eq47_e1281_d_n4, eq47_e1281_d_n5, eq47_e1281_d_n6, eq47_e1281_d_n7, eq47_e1281_d_n8, eq47_e1281_d_n9, eq47_e1281_d_n10, eq47_e1281_d_n11, eq47_e1281_d_n12, eq47_e1281_d_n13, eq47_e1281_d_n14, eq47_e1281_d_n15, eq47_e1281_d_n16, eq47_e1281_d_n17, eq47_e1281_d_b0, eq47_e1281_d_b1, eq47_e1281_d_b2, eq47_e1281_d_b3, eq47_e1281_d_b4, eq47_e1281_d_b5, eq47_e1281_d_b6, eq47_e1281_d_b7, eq47_e1281_d_b8, eq47_e1281_d_b9, eq47_e1281_d_b10, eq47_e1281_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e1283;
        let eq47_node_derivatives: [f64; 18] = [eq47_e1283_d_n0, eq47_e1283_d_n1, eq47_e1283_d_n2, eq47_e1283_d_n3, eq47_e1283_d_n4, eq47_e1283_d_n5, eq47_e1283_d_n6, eq47_e1283_d_n7, eq47_e1283_d_n8, eq47_e1283_d_n9, eq47_e1283_d_n10, eq47_e1283_d_n11, eq47_e1283_d_n12, eq47_e1283_d_n13, eq47_e1283_d_n14, eq47_e1283_d_n15, eq47_e1283_d_n16, eq47_e1283_d_n17];
        let eq47_branch_derivatives: [f64; 12] = [eq47_e1283_d_b0, eq47_e1283_d_b1, eq47_e1283_d_b2, eq47_e1283_d_b3, eq47_e1283_d_b4, eq47_e1283_d_b5, eq47_e1283_d_b6, eq47_e1283_d_b7, eq47_e1283_d_b8, eq47_e1283_d_b9, eq47_e1283_d_b10, eq47_e1283_d_b11];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(6),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e1288,) = {
    if (!s.b[3408]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1288;
        stamper.stamp_potential_const_local(
            5,
            eq48_value,
        );
        let (eq49_e1294, eq49_e1294_d_n0, eq49_e1294_d_n1, eq49_e1294_d_n2, eq49_e1294_d_n3, eq49_e1294_d_n4, eq49_e1294_d_n5, eq49_e1294_d_n6, eq49_e1294_d_n7, eq49_e1294_d_n8, eq49_e1294_d_n9, eq49_e1294_d_n10, eq49_e1294_d_n11, eq49_e1294_d_n12, eq49_e1294_d_n13, eq49_e1294_d_n14, eq49_e1294_d_n15, eq49_e1294_d_n16, eq49_e1294_d_n17, eq49_e1294_d_b0, eq49_e1294_d_b1, eq49_e1294_d_b2, eq49_e1294_d_b3, eq49_e1294_d_b4, eq49_e1294_d_b5, eq49_e1294_d_b6, eq49_e1294_d_b7, eq49_e1294_d_b8, eq49_e1294_d_b9, eq49_e1294_d_b10, eq49_e1294_d_b11,) = {
    if (p.p52 != 0.0) {
        let eq49_e1292: f64 = (s.v[656] * (nv10 - nv8));
        let eq49_e1292_d_n0: f64 = (s.dn[656][0] * (nv10 - nv8));
        let eq49_e1292_d_n1: f64 = (s.dn[656][1] * (nv10 - nv8));
        let eq49_e1292_d_n2: f64 = (s.dn[656][2] * (nv10 - nv8));
        let eq49_e1292_d_n3: f64 = (s.dn[656][3] * (nv10 - nv8));
        let eq49_e1292_d_n4: f64 = (s.dn[656][4] * (nv10 - nv8));
        let eq49_e1292_d_n5: f64 = (s.dn[656][5] * (nv10 - nv8));
        let eq49_e1292_d_n6: f64 = (s.dn[656][6] * (nv10 - nv8));
        let eq49_e1292_d_n7: f64 = (s.dn[656][7] * (nv10 - nv8));
        let eq49_e1292_d_n8: f64 = ((s.dn[656][8] * (nv10 - nv8)) + (-s.v[656]));
        let eq49_e1292_d_n9: f64 = (s.dn[656][9] * (nv10 - nv8));
        let eq49_e1292_d_n10: f64 = ((s.dn[656][10] * (nv10 - nv8)) + s.v[656]);
        let eq49_e1292_d_n11: f64 = (s.dn[656][11] * (nv10 - nv8));
        let eq49_e1292_d_n12: f64 = (s.dn[656][12] * (nv10 - nv8));
        let eq49_e1292_d_n13: f64 = (s.dn[656][13] * (nv10 - nv8));
        let eq49_e1292_d_n14: f64 = (s.dn[656][14] * (nv10 - nv8));
        let eq49_e1292_d_n15: f64 = (s.dn[656][15] * (nv10 - nv8));
        let eq49_e1292_d_n16: f64 = (s.dn[656][16] * (nv10 - nv8));
        let eq49_e1292_d_n17: f64 = (s.dn[656][17] * (nv10 - nv8));
        let eq49_e1292_d_b0: f64 = (s.db[656][0] * (nv10 - nv8));
        let eq49_e1292_d_b1: f64 = (s.db[656][1] * (nv10 - nv8));
        let eq49_e1292_d_b2: f64 = (s.db[656][2] * (nv10 - nv8));
        let eq49_e1292_d_b3: f64 = (s.db[656][3] * (nv10 - nv8));
        let eq49_e1292_d_b4: f64 = (s.db[656][4] * (nv10 - nv8));
        let eq49_e1292_d_b5: f64 = (s.db[656][5] * (nv10 - nv8));
        let eq49_e1292_d_b6: f64 = (s.db[656][6] * (nv10 - nv8));
        let eq49_e1292_d_b7: f64 = (s.db[656][7] * (nv10 - nv8));
        let eq49_e1292_d_b8: f64 = (s.db[656][8] * (nv10 - nv8));
        let eq49_e1292_d_b9: f64 = (s.db[656][9] * (nv10 - nv8));
        let eq49_e1292_d_b10: f64 = (s.db[656][10] * (nv10 - nv8));
        let eq49_e1292_d_b11: f64 = (s.db[656][11] * (nv10 - nv8));
        (eq49_e1292, eq49_e1292_d_n0, eq49_e1292_d_n1, eq49_e1292_d_n2, eq49_e1292_d_n3, eq49_e1292_d_n4, eq49_e1292_d_n5, eq49_e1292_d_n6, eq49_e1292_d_n7, eq49_e1292_d_n8, eq49_e1292_d_n9, eq49_e1292_d_n10, eq49_e1292_d_n11, eq49_e1292_d_n12, eq49_e1292_d_n13, eq49_e1292_d_n14, eq49_e1292_d_n15, eq49_e1292_d_n16, eq49_e1292_d_n17, eq49_e1292_d_b0, eq49_e1292_d_b1, eq49_e1292_d_b2, eq49_e1292_d_b3, eq49_e1292_d_b4, eq49_e1292_d_b5, eq49_e1292_d_b6, eq49_e1292_d_b7, eq49_e1292_d_b8, eq49_e1292_d_b9, eq49_e1292_d_b10, eq49_e1292_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1294;
        let eq49_node_derivatives: [f64; 18] = [eq49_e1294_d_n0, eq49_e1294_d_n1, eq49_e1294_d_n2, eq49_e1294_d_n3, eq49_e1294_d_n4, eq49_e1294_d_n5, eq49_e1294_d_n6, eq49_e1294_d_n7, eq49_e1294_d_n8, eq49_e1294_d_n9, eq49_e1294_d_n10, eq49_e1294_d_n11, eq49_e1294_d_n12, eq49_e1294_d_n13, eq49_e1294_d_n14, eq49_e1294_d_n15, eq49_e1294_d_n16, eq49_e1294_d_n17];
        let eq49_branch_derivatives: [f64; 12] = [eq49_e1294_d_b0, eq49_e1294_d_b1, eq49_e1294_d_b2, eq49_e1294_d_b3, eq49_e1294_d_b4, eq49_e1294_d_b5, eq49_e1294_d_b6, eq49_e1294_d_b7, eq49_e1294_d_b8, eq49_e1294_d_b9, eq49_e1294_d_b10, eq49_e1294_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(8),
            multiplicity * (eq49_value),
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let (eq50_e1300, eq50_e1300_d_n0, eq50_e1300_d_n1, eq50_e1300_d_n2, eq50_e1300_d_n3, eq50_e1300_d_n4, eq50_e1300_d_n5, eq50_e1300_d_n6, eq50_e1300_d_n7, eq50_e1300_d_n8, eq50_e1300_d_n9, eq50_e1300_d_n10, eq50_e1300_d_n11, eq50_e1300_d_n12, eq50_e1300_d_n13, eq50_e1300_d_n14, eq50_e1300_d_n15, eq50_e1300_d_n16, eq50_e1300_d_n17, eq50_e1300_d_b0, eq50_e1300_d_b1, eq50_e1300_d_b2, eq50_e1300_d_b3, eq50_e1300_d_b4, eq50_e1300_d_b5, eq50_e1300_d_b6, eq50_e1300_d_b7, eq50_e1300_d_b8, eq50_e1300_d_b9, eq50_e1300_d_b10, eq50_e1300_d_b11,) = {
    if (p.p52 != 0.0) {
        let eq50_e1298: f64 = (s.v[657] * (nv9 - nv8));
        let eq50_e1298_d_n0: f64 = (s.dn[657][0] * (nv9 - nv8));
        let eq50_e1298_d_n1: f64 = (s.dn[657][1] * (nv9 - nv8));
        let eq50_e1298_d_n2: f64 = (s.dn[657][2] * (nv9 - nv8));
        let eq50_e1298_d_n3: f64 = (s.dn[657][3] * (nv9 - nv8));
        let eq50_e1298_d_n4: f64 = (s.dn[657][4] * (nv9 - nv8));
        let eq50_e1298_d_n5: f64 = (s.dn[657][5] * (nv9 - nv8));
        let eq50_e1298_d_n6: f64 = (s.dn[657][6] * (nv9 - nv8));
        let eq50_e1298_d_n7: f64 = (s.dn[657][7] * (nv9 - nv8));
        let eq50_e1298_d_n8: f64 = ((s.dn[657][8] * (nv9 - nv8)) + (-s.v[657]));
        let eq50_e1298_d_n9: f64 = ((s.dn[657][9] * (nv9 - nv8)) + s.v[657]);
        let eq50_e1298_d_n10: f64 = (s.dn[657][10] * (nv9 - nv8));
        let eq50_e1298_d_n11: f64 = (s.dn[657][11] * (nv9 - nv8));
        let eq50_e1298_d_n12: f64 = (s.dn[657][12] * (nv9 - nv8));
        let eq50_e1298_d_n13: f64 = (s.dn[657][13] * (nv9 - nv8));
        let eq50_e1298_d_n14: f64 = (s.dn[657][14] * (nv9 - nv8));
        let eq50_e1298_d_n15: f64 = (s.dn[657][15] * (nv9 - nv8));
        let eq50_e1298_d_n16: f64 = (s.dn[657][16] * (nv9 - nv8));
        let eq50_e1298_d_n17: f64 = (s.dn[657][17] * (nv9 - nv8));
        let eq50_e1298_d_b0: f64 = (s.db[657][0] * (nv9 - nv8));
        let eq50_e1298_d_b1: f64 = (s.db[657][1] * (nv9 - nv8));
        let eq50_e1298_d_b2: f64 = (s.db[657][2] * (nv9 - nv8));
        let eq50_e1298_d_b3: f64 = (s.db[657][3] * (nv9 - nv8));
        let eq50_e1298_d_b4: f64 = (s.db[657][4] * (nv9 - nv8));
        let eq50_e1298_d_b5: f64 = (s.db[657][5] * (nv9 - nv8));
        let eq50_e1298_d_b6: f64 = (s.db[657][6] * (nv9 - nv8));
        let eq50_e1298_d_b7: f64 = (s.db[657][7] * (nv9 - nv8));
        let eq50_e1298_d_b8: f64 = (s.db[657][8] * (nv9 - nv8));
        let eq50_e1298_d_b9: f64 = (s.db[657][9] * (nv9 - nv8));
        let eq50_e1298_d_b10: f64 = (s.db[657][10] * (nv9 - nv8));
        let eq50_e1298_d_b11: f64 = (s.db[657][11] * (nv9 - nv8));
        (eq50_e1298, eq50_e1298_d_n0, eq50_e1298_d_n1, eq50_e1298_d_n2, eq50_e1298_d_n3, eq50_e1298_d_n4, eq50_e1298_d_n5, eq50_e1298_d_n6, eq50_e1298_d_n7, eq50_e1298_d_n8, eq50_e1298_d_n9, eq50_e1298_d_n10, eq50_e1298_d_n11, eq50_e1298_d_n12, eq50_e1298_d_n13, eq50_e1298_d_n14, eq50_e1298_d_n15, eq50_e1298_d_n16, eq50_e1298_d_n17, eq50_e1298_d_b0, eq50_e1298_d_b1, eq50_e1298_d_b2, eq50_e1298_d_b3, eq50_e1298_d_b4, eq50_e1298_d_b5, eq50_e1298_d_b6, eq50_e1298_d_b7, eq50_e1298_d_b8, eq50_e1298_d_b9, eq50_e1298_d_b10, eq50_e1298_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1300;
        let eq50_node_derivatives: [f64; 18] = [eq50_e1300_d_n0, eq50_e1300_d_n1, eq50_e1300_d_n2, eq50_e1300_d_n3, eq50_e1300_d_n4, eq50_e1300_d_n5, eq50_e1300_d_n6, eq50_e1300_d_n7, eq50_e1300_d_n8, eq50_e1300_d_n9, eq50_e1300_d_n10, eq50_e1300_d_n11, eq50_e1300_d_n12, eq50_e1300_d_n13, eq50_e1300_d_n14, eq50_e1300_d_n15, eq50_e1300_d_n16, eq50_e1300_d_n17];
        let eq50_branch_derivatives: [f64; 12] = [eq50_e1300_d_b0, eq50_e1300_d_b1, eq50_e1300_d_b2, eq50_e1300_d_b3, eq50_e1300_d_b4, eq50_e1300_d_b5, eq50_e1300_d_b6, eq50_e1300_d_b7, eq50_e1300_d_b8, eq50_e1300_d_b9, eq50_e1300_d_b10, eq50_e1300_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1306, eq51_e1306_d_n0, eq51_e1306_d_n1, eq51_e1306_d_n2, eq51_e1306_d_n3, eq51_e1306_d_n4, eq51_e1306_d_n5, eq51_e1306_d_n6, eq51_e1306_d_n7, eq51_e1306_d_n8, eq51_e1306_d_n9, eq51_e1306_d_n10, eq51_e1306_d_n11, eq51_e1306_d_n12, eq51_e1306_d_n13, eq51_e1306_d_n14, eq51_e1306_d_n15, eq51_e1306_d_n16, eq51_e1306_d_n17, eq51_e1306_d_b0, eq51_e1306_d_b1, eq51_e1306_d_b2, eq51_e1306_d_b3, eq51_e1306_d_b4, eq51_e1306_d_b5, eq51_e1306_d_b6, eq51_e1306_d_b7, eq51_e1306_d_b8, eq51_e1306_d_b9, eq51_e1306_d_b10, eq51_e1306_d_b11,) = {
    if (p.p52 != 0.0) {
        let eq51_e1304: f64 = (s.v[655] * (nv3 - nv8));
        let eq51_e1304_d_n0: f64 = (s.dn[655][0] * (nv3 - nv8));
        let eq51_e1304_d_n1: f64 = (s.dn[655][1] * (nv3 - nv8));
        let eq51_e1304_d_n2: f64 = (s.dn[655][2] * (nv3 - nv8));
        let eq51_e1304_d_n3: f64 = ((s.dn[655][3] * (nv3 - nv8)) + s.v[655]);
        let eq51_e1304_d_n4: f64 = (s.dn[655][4] * (nv3 - nv8));
        let eq51_e1304_d_n5: f64 = (s.dn[655][5] * (nv3 - nv8));
        let eq51_e1304_d_n6: f64 = (s.dn[655][6] * (nv3 - nv8));
        let eq51_e1304_d_n7: f64 = (s.dn[655][7] * (nv3 - nv8));
        let eq51_e1304_d_n8: f64 = ((s.dn[655][8] * (nv3 - nv8)) + (-s.v[655]));
        let eq51_e1304_d_n9: f64 = (s.dn[655][9] * (nv3 - nv8));
        let eq51_e1304_d_n10: f64 = (s.dn[655][10] * (nv3 - nv8));
        let eq51_e1304_d_n11: f64 = (s.dn[655][11] * (nv3 - nv8));
        let eq51_e1304_d_n12: f64 = (s.dn[655][12] * (nv3 - nv8));
        let eq51_e1304_d_n13: f64 = (s.dn[655][13] * (nv3 - nv8));
        let eq51_e1304_d_n14: f64 = (s.dn[655][14] * (nv3 - nv8));
        let eq51_e1304_d_n15: f64 = (s.dn[655][15] * (nv3 - nv8));
        let eq51_e1304_d_n16: f64 = (s.dn[655][16] * (nv3 - nv8));
        let eq51_e1304_d_n17: f64 = (s.dn[655][17] * (nv3 - nv8));
        let eq51_e1304_d_b0: f64 = (s.db[655][0] * (nv3 - nv8));
        let eq51_e1304_d_b1: f64 = (s.db[655][1] * (nv3 - nv8));
        let eq51_e1304_d_b2: f64 = (s.db[655][2] * (nv3 - nv8));
        let eq51_e1304_d_b3: f64 = (s.db[655][3] * (nv3 - nv8));
        let eq51_e1304_d_b4: f64 = (s.db[655][4] * (nv3 - nv8));
        let eq51_e1304_d_b5: f64 = (s.db[655][5] * (nv3 - nv8));
        let eq51_e1304_d_b6: f64 = (s.db[655][6] * (nv3 - nv8));
        let eq51_e1304_d_b7: f64 = (s.db[655][7] * (nv3 - nv8));
        let eq51_e1304_d_b8: f64 = (s.db[655][8] * (nv3 - nv8));
        let eq51_e1304_d_b9: f64 = (s.db[655][9] * (nv3 - nv8));
        let eq51_e1304_d_b10: f64 = (s.db[655][10] * (nv3 - nv8));
        let eq51_e1304_d_b11: f64 = (s.db[655][11] * (nv3 - nv8));
        (eq51_e1304, eq51_e1304_d_n0, eq51_e1304_d_n1, eq51_e1304_d_n2, eq51_e1304_d_n3, eq51_e1304_d_n4, eq51_e1304_d_n5, eq51_e1304_d_n6, eq51_e1304_d_n7, eq51_e1304_d_n8, eq51_e1304_d_n9, eq51_e1304_d_n10, eq51_e1304_d_n11, eq51_e1304_d_n12, eq51_e1304_d_n13, eq51_e1304_d_n14, eq51_e1304_d_n15, eq51_e1304_d_n16, eq51_e1304_d_n17, eq51_e1304_d_b0, eq51_e1304_d_b1, eq51_e1304_d_b2, eq51_e1304_d_b3, eq51_e1304_d_b4, eq51_e1304_d_b5, eq51_e1304_d_b6, eq51_e1304_d_b7, eq51_e1304_d_b8, eq51_e1304_d_b9, eq51_e1304_d_b10, eq51_e1304_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1306;
        let eq51_node_derivatives: [f64; 18] = [eq51_e1306_d_n0, eq51_e1306_d_n1, eq51_e1306_d_n2, eq51_e1306_d_n3, eq51_e1306_d_n4, eq51_e1306_d_n5, eq51_e1306_d_n6, eq51_e1306_d_n7, eq51_e1306_d_n8, eq51_e1306_d_n9, eq51_e1306_d_n10, eq51_e1306_d_n11, eq51_e1306_d_n12, eq51_e1306_d_n13, eq51_e1306_d_n14, eq51_e1306_d_n15, eq51_e1306_d_n16, eq51_e1306_d_n17];
        let eq51_branch_derivatives: [f64; 12] = [eq51_e1306_d_b0, eq51_e1306_d_b1, eq51_e1306_d_b2, eq51_e1306_d_b3, eq51_e1306_d_b4, eq51_e1306_d_b5, eq51_e1306_d_b6, eq51_e1306_d_b7, eq51_e1306_d_b8, eq51_e1306_d_b9, eq51_e1306_d_b10, eq51_e1306_d_b11];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e1311,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e1311;
        stamper.stamp_potential_const_local(
            6,
            eq52_value,
        );
        let (eq53_e1316,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e1316;
        stamper.stamp_potential_const_local(
            7,
            eq53_value,
        );
        let (eq54_e1321,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1321;
        stamper.stamp_potential_const_local(
            8,
            eq54_value,
        );
        let (eq55_e1327, eq55_e1327_d_n0, eq55_e1327_d_n1, eq55_e1327_d_n2, eq55_e1327_d_n3, eq55_e1327_d_n4, eq55_e1327_d_n5, eq55_e1327_d_n6, eq55_e1327_d_n7, eq55_e1327_d_n8, eq55_e1327_d_n9, eq55_e1327_d_n10, eq55_e1327_d_n11, eq55_e1327_d_n12, eq55_e1327_d_n13, eq55_e1327_d_n14, eq55_e1327_d_n15, eq55_e1327_d_n16, eq55_e1327_d_n17, eq55_e1327_d_b0, eq55_e1327_d_b1, eq55_e1327_d_b2, eq55_e1327_d_b3, eq55_e1327_d_b4, eq55_e1327_d_b5, eq55_e1327_d_b6, eq55_e1327_d_b7, eq55_e1327_d_b8, eq55_e1327_d_b9, eq55_e1327_d_b10, eq55_e1327_d_b11,) = {
    if s.b[3409] {
        let eq55_e1325: f64 = ((nv4 - 0.0) * s.v[740]);
        let eq55_e1325_d_n0: f64 = ((nv4 - 0.0) * s.dn[740][0]);
        let eq55_e1325_d_n1: f64 = ((nv4 - 0.0) * s.dn[740][1]);
        let eq55_e1325_d_n2: f64 = ((nv4 - 0.0) * s.dn[740][2]);
        let eq55_e1325_d_n3: f64 = ((nv4 - 0.0) * s.dn[740][3]);
        let eq55_e1325_d_n4: f64 = (s.v[740] + ((nv4 - 0.0) * s.dn[740][4]));
        let eq55_e1325_d_n5: f64 = ((nv4 - 0.0) * s.dn[740][5]);
        let eq55_e1325_d_n6: f64 = ((nv4 - 0.0) * s.dn[740][6]);
        let eq55_e1325_d_n7: f64 = ((nv4 - 0.0) * s.dn[740][7]);
        let eq55_e1325_d_n8: f64 = ((nv4 - 0.0) * s.dn[740][8]);
        let eq55_e1325_d_n9: f64 = ((nv4 - 0.0) * s.dn[740][9]);
        let eq55_e1325_d_n10: f64 = ((nv4 - 0.0) * s.dn[740][10]);
        let eq55_e1325_d_n11: f64 = ((nv4 - 0.0) * s.dn[740][11]);
        let eq55_e1325_d_n12: f64 = ((nv4 - 0.0) * s.dn[740][12]);
        let eq55_e1325_d_n13: f64 = ((nv4 - 0.0) * s.dn[740][13]);
        let eq55_e1325_d_n14: f64 = ((nv4 - 0.0) * s.dn[740][14]);
        let eq55_e1325_d_n15: f64 = ((nv4 - 0.0) * s.dn[740][15]);
        let eq55_e1325_d_n16: f64 = ((nv4 - 0.0) * s.dn[740][16]);
        let eq55_e1325_d_n17: f64 = ((nv4 - 0.0) * s.dn[740][17]);
        let eq55_e1325_d_b0: f64 = ((nv4 - 0.0) * s.db[740][0]);
        let eq55_e1325_d_b1: f64 = ((nv4 - 0.0) * s.db[740][1]);
        let eq55_e1325_d_b2: f64 = ((nv4 - 0.0) * s.db[740][2]);
        let eq55_e1325_d_b3: f64 = ((nv4 - 0.0) * s.db[740][3]);
        let eq55_e1325_d_b4: f64 = ((nv4 - 0.0) * s.db[740][4]);
        let eq55_e1325_d_b5: f64 = ((nv4 - 0.0) * s.db[740][5]);
        let eq55_e1325_d_b6: f64 = ((nv4 - 0.0) * s.db[740][6]);
        let eq55_e1325_d_b7: f64 = ((nv4 - 0.0) * s.db[740][7]);
        let eq55_e1325_d_b8: f64 = ((nv4 - 0.0) * s.db[740][8]);
        let eq55_e1325_d_b9: f64 = ((nv4 - 0.0) * s.db[740][9]);
        let eq55_e1325_d_b10: f64 = ((nv4 - 0.0) * s.db[740][10]);
        let eq55_e1325_d_b11: f64 = ((nv4 - 0.0) * s.db[740][11]);
        (eq55_e1325, eq55_e1325_d_n0, eq55_e1325_d_n1, eq55_e1325_d_n2, eq55_e1325_d_n3, eq55_e1325_d_n4, eq55_e1325_d_n5, eq55_e1325_d_n6, eq55_e1325_d_n7, eq55_e1325_d_n8, eq55_e1325_d_n9, eq55_e1325_d_n10, eq55_e1325_d_n11, eq55_e1325_d_n12, eq55_e1325_d_n13, eq55_e1325_d_n14, eq55_e1325_d_n15, eq55_e1325_d_n16, eq55_e1325_d_n17, eq55_e1325_d_b0, eq55_e1325_d_b1, eq55_e1325_d_b2, eq55_e1325_d_b3, eq55_e1325_d_b4, eq55_e1325_d_b5, eq55_e1325_d_b6, eq55_e1325_d_b7, eq55_e1325_d_b8, eq55_e1325_d_b9, eq55_e1325_d_b10, eq55_e1325_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1327;
        let eq55_node_derivatives: [f64; 18] = [eq55_e1327_d_n0, eq55_e1327_d_n1, eq55_e1327_d_n2, eq55_e1327_d_n3, eq55_e1327_d_n4, eq55_e1327_d_n5, eq55_e1327_d_n6, eq55_e1327_d_n7, eq55_e1327_d_n8, eq55_e1327_d_n9, eq55_e1327_d_n10, eq55_e1327_d_n11, eq55_e1327_d_n12, eq55_e1327_d_n13, eq55_e1327_d_n14, eq55_e1327_d_n15, eq55_e1327_d_n16, eq55_e1327_d_n17];
        let eq55_branch_derivatives: [f64; 12] = [eq55_e1327_d_b0, eq55_e1327_d_b1, eq55_e1327_d_b2, eq55_e1327_d_b3, eq55_e1327_d_b4, eq55_e1327_d_b5, eq55_e1327_d_b6, eq55_e1327_d_b7, eq55_e1327_d_b8, eq55_e1327_d_b9, eq55_e1327_d_b10, eq55_e1327_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e1332, eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17, eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8, eq56_e1332_d_b9, eq56_e1332_d_b10, eq56_e1332_d_b11,) = {
    if s.b[3409] {
        let eq56_e1330: f64 = (-s.v[802]);
        let eq56_e1330_d_n0: f64 = (-s.dn[802][0]);
        let eq56_e1330_d_n1: f64 = (-s.dn[802][1]);
        let eq56_e1330_d_n2: f64 = (-s.dn[802][2]);
        let eq56_e1330_d_n3: f64 = (-s.dn[802][3]);
        let eq56_e1330_d_n4: f64 = (-s.dn[802][4]);
        let eq56_e1330_d_n5: f64 = (-s.dn[802][5]);
        let eq56_e1330_d_n6: f64 = (-s.dn[802][6]);
        let eq56_e1330_d_n7: f64 = (-s.dn[802][7]);
        let eq56_e1330_d_n8: f64 = (-s.dn[802][8]);
        let eq56_e1330_d_n9: f64 = (-s.dn[802][9]);
        let eq56_e1330_d_n10: f64 = (-s.dn[802][10]);
        let eq56_e1330_d_n11: f64 = (-s.dn[802][11]);
        let eq56_e1330_d_n12: f64 = (-s.dn[802][12]);
        let eq56_e1330_d_n13: f64 = (-s.dn[802][13]);
        let eq56_e1330_d_n14: f64 = (-s.dn[802][14]);
        let eq56_e1330_d_n15: f64 = (-s.dn[802][15]);
        let eq56_e1330_d_n16: f64 = (-s.dn[802][16]);
        let eq56_e1330_d_n17: f64 = (-s.dn[802][17]);
        let eq56_e1330_d_b0: f64 = (-s.db[802][0]);
        let eq56_e1330_d_b1: f64 = (-s.db[802][1]);
        let eq56_e1330_d_b2: f64 = (-s.db[802][2]);
        let eq56_e1330_d_b3: f64 = (-s.db[802][3]);
        let eq56_e1330_d_b4: f64 = (-s.db[802][4]);
        let eq56_e1330_d_b5: f64 = (-s.db[802][5]);
        let eq56_e1330_d_b6: f64 = (-s.db[802][6]);
        let eq56_e1330_d_b7: f64 = (-s.db[802][7]);
        let eq56_e1330_d_b8: f64 = (-s.db[802][8]);
        let eq56_e1330_d_b9: f64 = (-s.db[802][9]);
        let eq56_e1330_d_b10: f64 = (-s.db[802][10]);
        let eq56_e1330_d_b11: f64 = (-s.db[802][11]);
        (eq56_e1330, eq56_e1330_d_n0, eq56_e1330_d_n1, eq56_e1330_d_n2, eq56_e1330_d_n3, eq56_e1330_d_n4, eq56_e1330_d_n5, eq56_e1330_d_n6, eq56_e1330_d_n7, eq56_e1330_d_n8, eq56_e1330_d_n9, eq56_e1330_d_n10, eq56_e1330_d_n11, eq56_e1330_d_n12, eq56_e1330_d_n13, eq56_e1330_d_n14, eq56_e1330_d_n15, eq56_e1330_d_n16, eq56_e1330_d_n17, eq56_e1330_d_b0, eq56_e1330_d_b1, eq56_e1330_d_b2, eq56_e1330_d_b3, eq56_e1330_d_b4, eq56_e1330_d_b5, eq56_e1330_d_b6, eq56_e1330_d_b7, eq56_e1330_d_b8, eq56_e1330_d_b9, eq56_e1330_d_b10, eq56_e1330_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1332;
        let eq56_node_derivatives: [f64; 18] = [eq56_e1332_d_n0, eq56_e1332_d_n1, eq56_e1332_d_n2, eq56_e1332_d_n3, eq56_e1332_d_n4, eq56_e1332_d_n5, eq56_e1332_d_n6, eq56_e1332_d_n7, eq56_e1332_d_n8, eq56_e1332_d_n9, eq56_e1332_d_n10, eq56_e1332_d_n11, eq56_e1332_d_n12, eq56_e1332_d_n13, eq56_e1332_d_n14, eq56_e1332_d_n15, eq56_e1332_d_n16, eq56_e1332_d_n17];
        let eq56_branch_derivatives: [f64; 12] = [eq56_e1332_d_b0, eq56_e1332_d_b1, eq56_e1332_d_b2, eq56_e1332_d_b3, eq56_e1332_d_b4, eq56_e1332_d_b5, eq56_e1332_d_b6, eq56_e1332_d_b7, eq56_e1332_d_b8, eq56_e1332_d_b9, eq56_e1332_d_b10, eq56_e1332_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1339, eq57_e1339_d_n4,) = {
    if (!s.b[3409]) {
        let eq57_e1337: f64 = ((nv4 - 0.0) * 10000.0);
        let eq57_e1337_d_n4: f64 = 10000.0;
        (eq57_e1337, eq57_e1337_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1339;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq57_value),
            4,
            multiplicity * (eq57_e1339_d_n4),
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq58_e1342: f64 = (s.v[767] * (nv4 - 0.0));
        let eq58_e1342_d_n0: f64 = (s.dn[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_n1: f64 = (s.dn[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_n2: f64 = (s.dn[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_n3: f64 = (s.dn[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_n4: f64 = ((s.dn[767][4] * (nv4 - 0.0)) + s.v[767]);
        let eq58_e1342_d_n5: f64 = (s.dn[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_n6: f64 = (s.dn[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_n7: f64 = (s.dn[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_n8: f64 = (s.dn[767][8] * (nv4 - 0.0));
        let eq58_e1342_d_n9: f64 = (s.dn[767][9] * (nv4 - 0.0));
        let eq58_e1342_d_n10: f64 = (s.dn[767][10] * (nv4 - 0.0));
        let eq58_e1342_d_n11: f64 = (s.dn[767][11] * (nv4 - 0.0));
        let eq58_e1342_d_n12: f64 = (s.dn[767][12] * (nv4 - 0.0));
        let eq58_e1342_d_n13: f64 = (s.dn[767][13] * (nv4 - 0.0));
        let eq58_e1342_d_n14: f64 = (s.dn[767][14] * (nv4 - 0.0));
        let eq58_e1342_d_n15: f64 = (s.dn[767][15] * (nv4 - 0.0));
        let eq58_e1342_d_n16: f64 = (s.dn[767][16] * (nv4 - 0.0));
        let eq58_e1342_d_n17: f64 = (s.dn[767][17] * (nv4 - 0.0));
        let eq58_e1342_d_b0: f64 = (s.db[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_b1: f64 = (s.db[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_b2: f64 = (s.db[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_b3: f64 = (s.db[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_b4: f64 = (s.db[767][4] * (nv4 - 0.0));
        let eq58_e1342_d_b5: f64 = (s.db[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_b6: f64 = (s.db[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_b7: f64 = (s.db[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_b8: f64 = (s.db[767][8] * (nv4 - 0.0));
        let eq58_e1342_d_b9: f64 = (s.db[767][9] * (nv4 - 0.0));
        let eq58_e1342_d_b10: f64 = (s.db[767][10] * (nv4 - 0.0));
        let eq58_e1342_d_b11: f64 = (s.db[767][11] * (nv4 - 0.0));
        let eq58_e1343: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, eq58_e1342);
        let eq58_e1343_d_n0: f64 = (eq58_e1342_d_n0 * ddt_scale);
        let eq58_e1343_d_n1: f64 = (eq58_e1342_d_n1 * ddt_scale);
        let eq58_e1343_d_n2: f64 = (eq58_e1342_d_n2 * ddt_scale);
        let eq58_e1343_d_n3: f64 = (eq58_e1342_d_n3 * ddt_scale);
        let eq58_e1343_d_n4: f64 = (eq58_e1342_d_n4 * ddt_scale);
        let eq58_e1343_d_n5: f64 = (eq58_e1342_d_n5 * ddt_scale);
        let eq58_e1343_d_n6: f64 = (eq58_e1342_d_n6 * ddt_scale);
        let eq58_e1343_d_n7: f64 = (eq58_e1342_d_n7 * ddt_scale);
        let eq58_e1343_d_n8: f64 = (eq58_e1342_d_n8 * ddt_scale);
        let eq58_e1343_d_n9: f64 = (eq58_e1342_d_n9 * ddt_scale);
        let eq58_e1343_d_n10: f64 = (eq58_e1342_d_n10 * ddt_scale);
        let eq58_e1343_d_n11: f64 = (eq58_e1342_d_n11 * ddt_scale);
        let eq58_e1343_d_n12: f64 = (eq58_e1342_d_n12 * ddt_scale);
        let eq58_e1343_d_n13: f64 = (eq58_e1342_d_n13 * ddt_scale);
        let eq58_e1343_d_n14: f64 = (eq58_e1342_d_n14 * ddt_scale);
        let eq58_e1343_d_n15: f64 = (eq58_e1342_d_n15 * ddt_scale);
        let eq58_e1343_d_n16: f64 = (eq58_e1342_d_n16 * ddt_scale);
        let eq58_e1343_d_n17: f64 = (eq58_e1342_d_n17 * ddt_scale);
        let eq58_e1343_d_b0: f64 = (eq58_e1342_d_b0 * ddt_scale);
        let eq58_e1343_d_b1: f64 = (eq58_e1342_d_b1 * ddt_scale);
        let eq58_e1343_d_b2: f64 = (eq58_e1342_d_b2 * ddt_scale);
        let eq58_e1343_d_b3: f64 = (eq58_e1342_d_b3 * ddt_scale);
        let eq58_e1343_d_b4: f64 = (eq58_e1342_d_b4 * ddt_scale);
        let eq58_e1343_d_b5: f64 = (eq58_e1342_d_b5 * ddt_scale);
        let eq58_e1343_d_b6: f64 = (eq58_e1342_d_b6 * ddt_scale);
        let eq58_e1343_d_b7: f64 = (eq58_e1342_d_b7 * ddt_scale);
        let eq58_e1343_d_b8: f64 = (eq58_e1342_d_b8 * ddt_scale);
        let eq58_e1343_d_b9: f64 = (eq58_e1342_d_b9 * ddt_scale);
        let eq58_e1343_d_b10: f64 = (eq58_e1342_d_b10 * ddt_scale);
        let eq58_e1343_d_b11: f64 = (eq58_e1342_d_b11 * ddt_scale);
        let eq58_value: f64 = eq58_e1343;
        let eq58_node_derivatives: [f64; 18] = [eq58_e1343_d_n0, eq58_e1343_d_n1, eq58_e1343_d_n2, eq58_e1343_d_n3, eq58_e1343_d_n4, eq58_e1343_d_n5, eq58_e1343_d_n6, eq58_e1343_d_n7, eq58_e1343_d_n8, eq58_e1343_d_n9, eq58_e1343_d_n10, eq58_e1343_d_n11, eq58_e1343_d_n12, eq58_e1343_d_n13, eq58_e1343_d_n14, eq58_e1343_d_n15, eq58_e1343_d_n16, eq58_e1343_d_n17];
        let eq58_branch_derivatives: [f64; 12] = [eq58_e1343_d_b0, eq58_e1343_d_b1, eq58_e1343_d_b2, eq58_e1343_d_b3, eq58_e1343_d_b4, eq58_e1343_d_b5, eq58_e1343_d_b6, eq58_e1343_d_b7, eq58_e1343_d_b8, eq58_e1343_d_b9, eq58_e1343_d_b10, eq58_e1343_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1347, eq59_e1347_d_n0, eq59_e1347_d_n1, eq59_e1347_d_n2, eq59_e1347_d_n3, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n12, eq59_e1347_d_n13, eq59_e1347_d_n14, eq59_e1347_d_n15, eq59_e1347_d_n16, eq59_e1347_d_n17, eq59_e1347_d_b0, eq59_e1347_d_b1, eq59_e1347_d_b2, eq59_e1347_d_b3, eq59_e1347_d_b4, eq59_e1347_d_b5, eq59_e1347_d_b6, eq59_e1347_d_b7, eq59_e1347_d_b8, eq59_e1347_d_b9, eq59_e1347_d_b10, eq59_e1347_d_b11,) = {
    if (p.p28 != 0.0) {
        (s.v[749], s.dn[749][0], s.dn[749][1], s.dn[749][2], s.dn[749][3], s.dn[749][4], s.dn[749][5], s.dn[749][6], s.dn[749][7], s.dn[749][8], s.dn[749][9], s.dn[749][10], s.dn[749][11], s.dn[749][12], s.dn[749][13], s.dn[749][14], s.dn[749][15], s.dn[749][16], s.dn[749][17], s.db[749][0], s.db[749][1], s.db[749][2], s.db[749][3], s.db[749][4], s.db[749][5], s.db[749][6], s.db[749][7], s.db[749][8], s.db[749][9], s.db[749][10], s.db[749][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1347;
        let eq59_node_derivatives: [f64; 18] = [eq59_e1347_d_n0, eq59_e1347_d_n1, eq59_e1347_d_n2, eq59_e1347_d_n3, eq59_e1347_d_n4, eq59_e1347_d_n5, eq59_e1347_d_n6, eq59_e1347_d_n7, eq59_e1347_d_n8, eq59_e1347_d_n9, eq59_e1347_d_n10, eq59_e1347_d_n11, eq59_e1347_d_n12, eq59_e1347_d_n13, eq59_e1347_d_n14, eq59_e1347_d_n15, eq59_e1347_d_n16, eq59_e1347_d_n17];
        let eq59_branch_derivatives: [f64; 12] = [eq59_e1347_d_b0, eq59_e1347_d_b1, eq59_e1347_d_b2, eq59_e1347_d_b3, eq59_e1347_d_b4, eq59_e1347_d_b5, eq59_e1347_d_b6, eq59_e1347_d_b7, eq59_e1347_d_b8, eq59_e1347_d_b9, eq59_e1347_d_b10, eq59_e1347_d_b11];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17, eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9, eq60_e1351_d_b10, eq60_e1351_d_b11,) = {
    if (p.p28 != 0.0) {
        (s.v[750], s.dn[750][0], s.dn[750][1], s.dn[750][2], s.dn[750][3], s.dn[750][4], s.dn[750][5], s.dn[750][6], s.dn[750][7], s.dn[750][8], s.dn[750][9], s.dn[750][10], s.dn[750][11], s.dn[750][12], s.dn[750][13], s.dn[750][14], s.dn[750][15], s.dn[750][16], s.dn[750][17], s.db[750][0], s.db[750][1], s.db[750][2], s.db[750][3], s.db[750][4], s.db[750][5], s.db[750][6], s.db[750][7], s.db[750][8], s.db[750][9], s.db[750][10], s.db[750][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivatives: [f64; 18] = [eq60_e1351_d_n0, eq60_e1351_d_n1, eq60_e1351_d_n2, eq60_e1351_d_n3, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n13, eq60_e1351_d_n14, eq60_e1351_d_n15, eq60_e1351_d_n16, eq60_e1351_d_n17];
        let eq60_branch_derivatives: [f64; 12] = [eq60_e1351_d_b0, eq60_e1351_d_b1, eq60_e1351_d_b2, eq60_e1351_d_b3, eq60_e1351_d_b4, eq60_e1351_d_b5, eq60_e1351_d_b6, eq60_e1351_d_b7, eq60_e1351_d_b8, eq60_e1351_d_b9, eq60_e1351_d_b10, eq60_e1351_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1358, eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17, eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (s.v[800] * (nv11 - 0.0));
        let eq61_e1355_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);
        let eq61_e1355_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));
        let eq61_e1355_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));
        let eq61_e1355_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));
        let eq61_e1355_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));
        let eq61_e1355_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));
        let eq61_e1355_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));
        let eq61_e1355_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));
        let eq61_e1356: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, eq61_e1355);
        let eq61_e1356_d_n0: f64 = (eq61_e1355_d_n0 * ddt_scale);
        let eq61_e1356_d_n1: f64 = (eq61_e1355_d_n1 * ddt_scale);
        let eq61_e1356_d_n2: f64 = (eq61_e1355_d_n2 * ddt_scale);
        let eq61_e1356_d_n3: f64 = (eq61_e1355_d_n3 * ddt_scale);
        let eq61_e1356_d_n4: f64 = (eq61_e1355_d_n4 * ddt_scale);
        let eq61_e1356_d_n5: f64 = (eq61_e1355_d_n5 * ddt_scale);
        let eq61_e1356_d_n6: f64 = (eq61_e1355_d_n6 * ddt_scale);
        let eq61_e1356_d_n7: f64 = (eq61_e1355_d_n7 * ddt_scale);
        let eq61_e1356_d_n8: f64 = (eq61_e1355_d_n8 * ddt_scale);
        let eq61_e1356_d_n9: f64 = (eq61_e1355_d_n9 * ddt_scale);
        let eq61_e1356_d_n10: f64 = (eq61_e1355_d_n10 * ddt_scale);
        let eq61_e1356_d_n11: f64 = (eq61_e1355_d_n11 * ddt_scale);
        let eq61_e1356_d_n12: f64 = (eq61_e1355_d_n12 * ddt_scale);
        let eq61_e1356_d_n13: f64 = (eq61_e1355_d_n13 * ddt_scale);
        let eq61_e1356_d_n14: f64 = (eq61_e1355_d_n14 * ddt_scale);
        let eq61_e1356_d_n15: f64 = (eq61_e1355_d_n15 * ddt_scale);
        let eq61_e1356_d_n16: f64 = (eq61_e1355_d_n16 * ddt_scale);
        let eq61_e1356_d_n17: f64 = (eq61_e1355_d_n17 * ddt_scale);
        let eq61_e1356_d_b0: f64 = (eq61_e1355_d_b0 * ddt_scale);
        let eq61_e1356_d_b1: f64 = (eq61_e1355_d_b1 * ddt_scale);
        let eq61_e1356_d_b2: f64 = (eq61_e1355_d_b2 * ddt_scale);
        let eq61_e1356_d_b3: f64 = (eq61_e1355_d_b3 * ddt_scale);
        let eq61_e1356_d_b4: f64 = (eq61_e1355_d_b4 * ddt_scale);
        let eq61_e1356_d_b5: f64 = (eq61_e1355_d_b5 * ddt_scale);
        let eq61_e1356_d_b6: f64 = (eq61_e1355_d_b6 * ddt_scale);
        let eq61_e1356_d_b7: f64 = (eq61_e1355_d_b7 * ddt_scale);
        let eq61_e1356_d_b8: f64 = (eq61_e1355_d_b8 * ddt_scale);
        let eq61_e1356_d_b9: f64 = (eq61_e1355_d_b9 * ddt_scale);
        let eq61_e1356_d_b10: f64 = (eq61_e1355_d_b10 * ddt_scale);
        let eq61_e1356_d_b11: f64 = (eq61_e1355_d_b11 * ddt_scale);
        (eq61_e1356, eq61_e1356_d_n0, eq61_e1356_d_n1, eq61_e1356_d_n2, eq61_e1356_d_n3, eq61_e1356_d_n4, eq61_e1356_d_n5, eq61_e1356_d_n6, eq61_e1356_d_n7, eq61_e1356_d_n8, eq61_e1356_d_n9, eq61_e1356_d_n10, eq61_e1356_d_n11, eq61_e1356_d_n12, eq61_e1356_d_n13, eq61_e1356_d_n14, eq61_e1356_d_n15, eq61_e1356_d_n16, eq61_e1356_d_n17, eq61_e1356_d_b0, eq61_e1356_d_b1, eq61_e1356_d_b2, eq61_e1356_d_b3, eq61_e1356_d_b4, eq61_e1356_d_b5, eq61_e1356_d_b6, eq61_e1356_d_b7, eq61_e1356_d_b8, eq61_e1356_d_b9, eq61_e1356_d_b10, eq61_e1356_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1358;
        let eq61_node_derivatives: [f64; 18] = [eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17];
        let eq61_branch_derivatives: [f64; 12] = [eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1365, eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17, eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (s.v[801] * (nv12 - 0.0));
        let eq62_e1362_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));
        let eq62_e1362_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);
        let eq62_e1362_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));
        let eq62_e1362_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));
        let eq62_e1362_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));
        let eq62_e1362_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));
        let eq62_e1362_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));
        let eq62_e1362_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));
        let eq62_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq62_e1362);
        let eq62_e1363_d_n0: f64 = (eq62_e1362_d_n0 * ddt_scale);
        let eq62_e1363_d_n1: f64 = (eq62_e1362_d_n1 * ddt_scale);
        let eq62_e1363_d_n2: f64 = (eq62_e1362_d_n2 * ddt_scale);
        let eq62_e1363_d_n3: f64 = (eq62_e1362_d_n3 * ddt_scale);
        let eq62_e1363_d_n4: f64 = (eq62_e1362_d_n4 * ddt_scale);
        let eq62_e1363_d_n5: f64 = (eq62_e1362_d_n5 * ddt_scale);
        let eq62_e1363_d_n6: f64 = (eq62_e1362_d_n6 * ddt_scale);
        let eq62_e1363_d_n7: f64 = (eq62_e1362_d_n7 * ddt_scale);
        let eq62_e1363_d_n8: f64 = (eq62_e1362_d_n8 * ddt_scale);
        let eq62_e1363_d_n9: f64 = (eq62_e1362_d_n9 * ddt_scale);
        let eq62_e1363_d_n10: f64 = (eq62_e1362_d_n10 * ddt_scale);
        let eq62_e1363_d_n11: f64 = (eq62_e1362_d_n11 * ddt_scale);
        let eq62_e1363_d_n12: f64 = (eq62_e1362_d_n12 * ddt_scale);
        let eq62_e1363_d_n13: f64 = (eq62_e1362_d_n13 * ddt_scale);
        let eq62_e1363_d_n14: f64 = (eq62_e1362_d_n14 * ddt_scale);
        let eq62_e1363_d_n15: f64 = (eq62_e1362_d_n15 * ddt_scale);
        let eq62_e1363_d_n16: f64 = (eq62_e1362_d_n16 * ddt_scale);
        let eq62_e1363_d_n17: f64 = (eq62_e1362_d_n17 * ddt_scale);
        let eq62_e1363_d_b0: f64 = (eq62_e1362_d_b0 * ddt_scale);
        let eq62_e1363_d_b1: f64 = (eq62_e1362_d_b1 * ddt_scale);
        let eq62_e1363_d_b2: f64 = (eq62_e1362_d_b2 * ddt_scale);
        let eq62_e1363_d_b3: f64 = (eq62_e1362_d_b3 * ddt_scale);
        let eq62_e1363_d_b4: f64 = (eq62_e1362_d_b4 * ddt_scale);
        let eq62_e1363_d_b5: f64 = (eq62_e1362_d_b5 * ddt_scale);
        let eq62_e1363_d_b6: f64 = (eq62_e1362_d_b6 * ddt_scale);
        let eq62_e1363_d_b7: f64 = (eq62_e1362_d_b7 * ddt_scale);
        let eq62_e1363_d_b8: f64 = (eq62_e1362_d_b8 * ddt_scale);
        let eq62_e1363_d_b9: f64 = (eq62_e1362_d_b9 * ddt_scale);
        let eq62_e1363_d_b10: f64 = (eq62_e1362_d_b10 * ddt_scale);
        let eq62_e1363_d_b11: f64 = (eq62_e1362_d_b11 * ddt_scale);
        (eq62_e1363, eq62_e1363_d_n0, eq62_e1363_d_n1, eq62_e1363_d_n2, eq62_e1363_d_n3, eq62_e1363_d_n4, eq62_e1363_d_n5, eq62_e1363_d_n6, eq62_e1363_d_n7, eq62_e1363_d_n8, eq62_e1363_d_n9, eq62_e1363_d_n10, eq62_e1363_d_n11, eq62_e1363_d_n12, eq62_e1363_d_n13, eq62_e1363_d_n14, eq62_e1363_d_n15, eq62_e1363_d_n16, eq62_e1363_d_n17, eq62_e1363_d_b0, eq62_e1363_d_b1, eq62_e1363_d_b2, eq62_e1363_d_b3, eq62_e1363_d_b4, eq62_e1363_d_b5, eq62_e1363_d_b6, eq62_e1363_d_b7, eq62_e1363_d_b8, eq62_e1363_d_b9, eq62_e1363_d_b10, eq62_e1363_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1365;
        let eq62_node_derivatives: [f64; 18] = [eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17];
        let eq62_branch_derivatives: [f64; 12] = [eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1370,) = {
    if (p.p28 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1370;
        stamper.stamp_potential_const_local(
            9,
            eq63_value,
        );
        let (eq64_e1375,) = {
    if (p.p28 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1375;
        stamper.stamp_potential_const_local(
            10,
            eq64_value,
        );
        let (eq65_e1379, eq65_e1379_d_n0, eq65_e1379_d_n1, eq65_e1379_d_n2, eq65_e1379_d_n3, eq65_e1379_d_n4, eq65_e1379_d_n5, eq65_e1379_d_n6, eq65_e1379_d_n7, eq65_e1379_d_n8, eq65_e1379_d_n9, eq65_e1379_d_n10, eq65_e1379_d_n11, eq65_e1379_d_n12, eq65_e1379_d_n13, eq65_e1379_d_n14, eq65_e1379_d_n15, eq65_e1379_d_n16, eq65_e1379_d_n17, eq65_e1379_d_b0, eq65_e1379_d_b1, eq65_e1379_d_b2, eq65_e1379_d_b3, eq65_e1379_d_b4, eq65_e1379_d_b5, eq65_e1379_d_b6, eq65_e1379_d_b7, eq65_e1379_d_b8, eq65_e1379_d_b9, eq65_e1379_d_b10, eq65_e1379_d_b11,) = {
    if (p.p29 != 0.0) {
        (s.v[815], s.dn[815][0], s.dn[815][1], s.dn[815][2], s.dn[815][3], s.dn[815][4], s.dn[815][5], s.dn[815][6], s.dn[815][7], s.dn[815][8], s.dn[815][9], s.dn[815][10], s.dn[815][11], s.dn[815][12], s.dn[815][13], s.dn[815][14], s.dn[815][15], s.dn[815][16], s.dn[815][17], s.db[815][0], s.db[815][1], s.db[815][2], s.db[815][3], s.db[815][4], s.db[815][5], s.db[815][6], s.db[815][7], s.db[815][8], s.db[815][9], s.db[815][10], s.db[815][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1379;
        let eq65_node_derivatives: [f64; 18] = [eq65_e1379_d_n0, eq65_e1379_d_n1, eq65_e1379_d_n2, eq65_e1379_d_n3, eq65_e1379_d_n4, eq65_e1379_d_n5, eq65_e1379_d_n6, eq65_e1379_d_n7, eq65_e1379_d_n8, eq65_e1379_d_n9, eq65_e1379_d_n10, eq65_e1379_d_n11, eq65_e1379_d_n12, eq65_e1379_d_n13, eq65_e1379_d_n14, eq65_e1379_d_n15, eq65_e1379_d_n16, eq65_e1379_d_n17];
        let eq65_branch_derivatives: [f64; 12] = [eq65_e1379_d_b0, eq65_e1379_d_b1, eq65_e1379_d_b2, eq65_e1379_d_b3, eq65_e1379_d_b4, eq65_e1379_d_b5, eq65_e1379_d_b6, eq65_e1379_d_b7, eq65_e1379_d_b8, eq65_e1379_d_b9, eq65_e1379_d_b10, eq65_e1379_d_b11];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1384, eq66_e1384_d_n13,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, (nv13 - 0.0));
        (eq66_e1382, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1384;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq66_value),
            13,
            multiplicity * (eq66_e1384_d_n13),
        );
        let (eq67_e1389,) = {
    if (p.p29 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1389;
        stamper.stamp_potential_const_local(
            11,
            eq67_value,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17, eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_d_b9, eq0_e1018_d_b10, eq0_e1018_d_b11, eq0_e1018_q, eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17, eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8, eq0_e1018_q_d_b9, eq0_e1018_q_d_b10, eq0_e1018_q_d_b11,) = {
    if s.b[3305] {
        let eq0_e1015_q: f64 = s.v[924];
        let eq0_e1016: f64 = (s.v[926] + s.v[924]);
        let eq0_e1016_d_n0: f64 = (s.dn[926][0] + s.dn[924][0]);
        let eq0_e1016_d_n1: f64 = (s.dn[926][1] + s.dn[924][1]);
        let eq0_e1016_d_n2: f64 = (s.dn[926][2] + s.dn[924][2]);
        let eq0_e1016_d_n3: f64 = (s.dn[926][3] + s.dn[924][3]);
        let eq0_e1016_d_n4: f64 = (s.dn[926][4] + s.dn[924][4]);
        let eq0_e1016_d_n5: f64 = (s.dn[926][5] + s.dn[924][5]);
        let eq0_e1016_d_n6: f64 = (s.dn[926][6] + s.dn[924][6]);
        let eq0_e1016_d_n7: f64 = (s.dn[926][7] + s.dn[924][7]);
        let eq0_e1016_d_n8: f64 = (s.dn[926][8] + s.dn[924][8]);
        let eq0_e1016_d_n9: f64 = (s.dn[926][9] + s.dn[924][9]);
        let eq0_e1016_d_n10: f64 = (s.dn[926][10] + s.dn[924][10]);
        let eq0_e1016_d_n11: f64 = (s.dn[926][11] + s.dn[924][11]);
        let eq0_e1016_d_n12: f64 = (s.dn[926][12] + s.dn[924][12]);
        let eq0_e1016_d_n13: f64 = (s.dn[926][13] + s.dn[924][13]);
        let eq0_e1016_d_n14: f64 = (s.dn[926][14] + s.dn[924][14]);
        let eq0_e1016_d_n15: f64 = (s.dn[926][15] + s.dn[924][15]);
        let eq0_e1016_d_n16: f64 = (s.dn[926][16] + s.dn[924][16]);
        let eq0_e1016_d_n17: f64 = (s.dn[926][17] + s.dn[924][17]);
        let eq0_e1016_d_b0: f64 = (s.db[926][0] + s.db[924][0]);
        let eq0_e1016_d_b1: f64 = (s.db[926][1] + s.db[924][1]);
        let eq0_e1016_d_b2: f64 = (s.db[926][2] + s.db[924][2]);
        let eq0_e1016_d_b3: f64 = (s.db[926][3] + s.db[924][3]);
        let eq0_e1016_d_b4: f64 = (s.db[926][4] + s.db[924][4]);
        let eq0_e1016_d_b5: f64 = (s.db[926][5] + s.db[924][5]);
        let eq0_e1016_d_b6: f64 = (s.db[926][6] + s.db[924][6]);
        let eq0_e1016_d_b7: f64 = (s.db[926][7] + s.db[924][7]);
        let eq0_e1016_d_b8: f64 = (s.db[926][8] + s.db[924][8]);
        let eq0_e1016_d_b9: f64 = (s.db[926][9] + s.db[924][9]);
        let eq0_e1016_d_b10: f64 = (s.db[926][10] + s.db[924][10]);
        let eq0_e1016_d_b11: f64 = (s.db[926][11] + s.db[924][11]);
        let eq0_e1016_q: f64 = eq0_e1015_q;
        (eq0_e1016, eq0_e1016_d_n0, eq0_e1016_d_n1, eq0_e1016_d_n2, eq0_e1016_d_n3, eq0_e1016_d_n4, eq0_e1016_d_n5, eq0_e1016_d_n6, eq0_e1016_d_n7, eq0_e1016_d_n8, eq0_e1016_d_n9, eq0_e1016_d_n10, eq0_e1016_d_n11, eq0_e1016_d_n12, eq0_e1016_d_n13, eq0_e1016_d_n14, eq0_e1016_d_n15, eq0_e1016_d_n16, eq0_e1016_d_n17, eq0_e1016_d_b0, eq0_e1016_d_b1, eq0_e1016_d_b2, eq0_e1016_d_b3, eq0_e1016_d_b4, eq0_e1016_d_b5, eq0_e1016_d_b6, eq0_e1016_d_b7, eq0_e1016_d_b8, eq0_e1016_d_b9, eq0_e1016_d_b10, eq0_e1016_d_b11, eq0_e1016_q, s.dn[924][0], s.dn[924][1], s.dn[924][2], s.dn[924][3], s.dn[924][4], s.dn[924][5], s.dn[924][6], s.dn[924][7], s.dn[924][8], s.dn[924][9], s.dn[924][10], s.dn[924][11], s.dn[924][12], s.dn[924][13], s.dn[924][14], s.dn[924][15], s.dn[924][16], s.dn[924][17], s.db[924][0], s.db[924][1], s.db[924][2], s.db[924][3], s.db[924][4], s.db[924][5], s.db[924][6], s.db[924][7], s.db[924][8], s.db[924][9], s.db[924][10], s.db[924][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_reactive_node_derivatives: [f64; 18] = [eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17];
        let eq0_reactive_branch_derivatives: [f64; 12] = [eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8, eq0_e1018_q_d_b9, eq0_e1018_q_d_b10, eq0_e1018_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq0_reactive_node_derivatives,
            branches,
            &eq0_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17, eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_d_b9, eq1_e1025_d_b10, eq1_e1025_d_b11, eq1_e1025_q, eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17, eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8, eq1_e1025_q_d_b9, eq1_e1025_q_d_b10, eq1_e1025_q_d_b11,) = {
    if s.b[3305] {
        let eq1_e1022_q: f64 = s.v[925];
        let eq1_e1023: f64 = (s.v[927] + s.v[925]);
        let eq1_e1023_d_n0: f64 = (s.dn[927][0] + s.dn[925][0]);
        let eq1_e1023_d_n1: f64 = (s.dn[927][1] + s.dn[925][1]);
        let eq1_e1023_d_n2: f64 = (s.dn[927][2] + s.dn[925][2]);
        let eq1_e1023_d_n3: f64 = (s.dn[927][3] + s.dn[925][3]);
        let eq1_e1023_d_n4: f64 = (s.dn[927][4] + s.dn[925][4]);
        let eq1_e1023_d_n5: f64 = (s.dn[927][5] + s.dn[925][5]);
        let eq1_e1023_d_n6: f64 = (s.dn[927][6] + s.dn[925][6]);
        let eq1_e1023_d_n7: f64 = (s.dn[927][7] + s.dn[925][7]);
        let eq1_e1023_d_n8: f64 = (s.dn[927][8] + s.dn[925][8]);
        let eq1_e1023_d_n9: f64 = (s.dn[927][9] + s.dn[925][9]);
        let eq1_e1023_d_n10: f64 = (s.dn[927][10] + s.dn[925][10]);
        let eq1_e1023_d_n11: f64 = (s.dn[927][11] + s.dn[925][11]);
        let eq1_e1023_d_n12: f64 = (s.dn[927][12] + s.dn[925][12]);
        let eq1_e1023_d_n13: f64 = (s.dn[927][13] + s.dn[925][13]);
        let eq1_e1023_d_n14: f64 = (s.dn[927][14] + s.dn[925][14]);
        let eq1_e1023_d_n15: f64 = (s.dn[927][15] + s.dn[925][15]);
        let eq1_e1023_d_n16: f64 = (s.dn[927][16] + s.dn[925][16]);
        let eq1_e1023_d_n17: f64 = (s.dn[927][17] + s.dn[925][17]);
        let eq1_e1023_d_b0: f64 = (s.db[927][0] + s.db[925][0]);
        let eq1_e1023_d_b1: f64 = (s.db[927][1] + s.db[925][1]);
        let eq1_e1023_d_b2: f64 = (s.db[927][2] + s.db[925][2]);
        let eq1_e1023_d_b3: f64 = (s.db[927][3] + s.db[925][3]);
        let eq1_e1023_d_b4: f64 = (s.db[927][4] + s.db[925][4]);
        let eq1_e1023_d_b5: f64 = (s.db[927][5] + s.db[925][5]);
        let eq1_e1023_d_b6: f64 = (s.db[927][6] + s.db[925][6]);
        let eq1_e1023_d_b7: f64 = (s.db[927][7] + s.db[925][7]);
        let eq1_e1023_d_b8: f64 = (s.db[927][8] + s.db[925][8]);
        let eq1_e1023_d_b9: f64 = (s.db[927][9] + s.db[925][9]);
        let eq1_e1023_d_b10: f64 = (s.db[927][10] + s.db[925][10]);
        let eq1_e1023_d_b11: f64 = (s.db[927][11] + s.db[925][11]);
        let eq1_e1023_q: f64 = eq1_e1022_q;
        (eq1_e1023, eq1_e1023_d_n0, eq1_e1023_d_n1, eq1_e1023_d_n2, eq1_e1023_d_n3, eq1_e1023_d_n4, eq1_e1023_d_n5, eq1_e1023_d_n6, eq1_e1023_d_n7, eq1_e1023_d_n8, eq1_e1023_d_n9, eq1_e1023_d_n10, eq1_e1023_d_n11, eq1_e1023_d_n12, eq1_e1023_d_n13, eq1_e1023_d_n14, eq1_e1023_d_n15, eq1_e1023_d_n16, eq1_e1023_d_n17, eq1_e1023_d_b0, eq1_e1023_d_b1, eq1_e1023_d_b2, eq1_e1023_d_b3, eq1_e1023_d_b4, eq1_e1023_d_b5, eq1_e1023_d_b6, eq1_e1023_d_b7, eq1_e1023_d_b8, eq1_e1023_d_b9, eq1_e1023_d_b10, eq1_e1023_d_b11, eq1_e1023_q, s.dn[925][0], s.dn[925][1], s.dn[925][2], s.dn[925][3], s.dn[925][4], s.dn[925][5], s.dn[925][6], s.dn[925][7], s.dn[925][8], s.dn[925][9], s.dn[925][10], s.dn[925][11], s.dn[925][12], s.dn[925][13], s.dn[925][14], s.dn[925][15], s.dn[925][16], s.dn[925][17], s.db[925][0], s.db[925][1], s.db[925][2], s.db[925][3], s.db[925][4], s.db[925][5], s.db[925][6], s.db[925][7], s.db[925][8], s.db[925][9], s.db[925][10], s.db[925][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_reactive_node_derivatives: [f64; 18] = [eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17];
        let eq1_reactive_branch_derivatives: [f64; 12] = [eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8, eq1_e1025_q_d_b9, eq1_e1025_q_d_b10, eq1_e1025_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            None,
            nodes,
            &eq1_reactive_node_derivatives,
            branches,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17, eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_d_b9, eq4_e1042_d_b10, eq4_e1042_d_b11, eq4_e1042_q, eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17, eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8, eq4_e1042_q_d_b9, eq4_e1042_q_d_b10, eq4_e1042_q_d_b11,) = {
    if s.b[3306] {
        let eq4_e1039_q: f64 = s.v[931];
        let eq4_e1040: f64 = (s.v[932] + s.v[931]);
        let eq4_e1040_d_n0: f64 = (s.dn[932][0] + s.dn[931][0]);
        let eq4_e1040_d_n1: f64 = (s.dn[932][1] + s.dn[931][1]);
        let eq4_e1040_d_n2: f64 = (s.dn[932][2] + s.dn[931][2]);
        let eq4_e1040_d_n3: f64 = (s.dn[932][3] + s.dn[931][3]);
        let eq4_e1040_d_n4: f64 = (s.dn[932][4] + s.dn[931][4]);
        let eq4_e1040_d_n5: f64 = (s.dn[932][5] + s.dn[931][5]);
        let eq4_e1040_d_n6: f64 = (s.dn[932][6] + s.dn[931][6]);
        let eq4_e1040_d_n7: f64 = (s.dn[932][7] + s.dn[931][7]);
        let eq4_e1040_d_n8: f64 = (s.dn[932][8] + s.dn[931][8]);
        let eq4_e1040_d_n9: f64 = (s.dn[932][9] + s.dn[931][9]);
        let eq4_e1040_d_n10: f64 = (s.dn[932][10] + s.dn[931][10]);
        let eq4_e1040_d_n11: f64 = (s.dn[932][11] + s.dn[931][11]);
        let eq4_e1040_d_n12: f64 = (s.dn[932][12] + s.dn[931][12]);
        let eq4_e1040_d_n13: f64 = (s.dn[932][13] + s.dn[931][13]);
        let eq4_e1040_d_n14: f64 = (s.dn[932][14] + s.dn[931][14]);
        let eq4_e1040_d_n15: f64 = (s.dn[932][15] + s.dn[931][15]);
        let eq4_e1040_d_n16: f64 = (s.dn[932][16] + s.dn[931][16]);
        let eq4_e1040_d_n17: f64 = (s.dn[932][17] + s.dn[931][17]);
        let eq4_e1040_d_b0: f64 = (s.db[932][0] + s.db[931][0]);
        let eq4_e1040_d_b1: f64 = (s.db[932][1] + s.db[931][1]);
        let eq4_e1040_d_b2: f64 = (s.db[932][2] + s.db[931][2]);
        let eq4_e1040_d_b3: f64 = (s.db[932][3] + s.db[931][3]);
        let eq4_e1040_d_b4: f64 = (s.db[932][4] + s.db[931][4]);
        let eq4_e1040_d_b5: f64 = (s.db[932][5] + s.db[931][5]);
        let eq4_e1040_d_b6: f64 = (s.db[932][6] + s.db[931][6]);
        let eq4_e1040_d_b7: f64 = (s.db[932][7] + s.db[931][7]);
        let eq4_e1040_d_b8: f64 = (s.db[932][8] + s.db[931][8]);
        let eq4_e1040_d_b9: f64 = (s.db[932][9] + s.db[931][9]);
        let eq4_e1040_d_b10: f64 = (s.db[932][10] + s.db[931][10]);
        let eq4_e1040_d_b11: f64 = (s.db[932][11] + s.db[931][11]);
        let eq4_e1040_q: f64 = eq4_e1039_q;
        (eq4_e1040, eq4_e1040_d_n0, eq4_e1040_d_n1, eq4_e1040_d_n2, eq4_e1040_d_n3, eq4_e1040_d_n4, eq4_e1040_d_n5, eq4_e1040_d_n6, eq4_e1040_d_n7, eq4_e1040_d_n8, eq4_e1040_d_n9, eq4_e1040_d_n10, eq4_e1040_d_n11, eq4_e1040_d_n12, eq4_e1040_d_n13, eq4_e1040_d_n14, eq4_e1040_d_n15, eq4_e1040_d_n16, eq4_e1040_d_n17, eq4_e1040_d_b0, eq4_e1040_d_b1, eq4_e1040_d_b2, eq4_e1040_d_b3, eq4_e1040_d_b4, eq4_e1040_d_b5, eq4_e1040_d_b6, eq4_e1040_d_b7, eq4_e1040_d_b8, eq4_e1040_d_b9, eq4_e1040_d_b10, eq4_e1040_d_b11, eq4_e1040_q, s.dn[931][0], s.dn[931][1], s.dn[931][2], s.dn[931][3], s.dn[931][4], s.dn[931][5], s.dn[931][6], s.dn[931][7], s.dn[931][8], s.dn[931][9], s.dn[931][10], s.dn[931][11], s.dn[931][12], s.dn[931][13], s.dn[931][14], s.dn[931][15], s.dn[931][16], s.dn[931][17], s.db[931][0], s.db[931][1], s.db[931][2], s.db[931][3], s.db[931][4], s.db[931][5], s.db[931][6], s.db[931][7], s.db[931][8], s.db[931][9], s.db[931][10], s.db[931][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 18] = [eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17];
        let eq4_reactive_branch_derivatives: [f64; 12] = [eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8, eq4_e1042_q_d_b9, eq4_e1042_q_d_b10, eq4_e1042_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            None,
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e1088_q: f64 = s.v[66];
        let eq14_e1089: f64 = (p.p87 * s.v[66]);
        let eq14_e1089_d_n0: f64 = (p.p87 * s.dn[66][0]);
        let eq14_e1089_d_n1: f64 = (p.p87 * s.dn[66][1]);
        let eq14_e1089_d_n2: f64 = (p.p87 * s.dn[66][2]);
        let eq14_e1089_d_n3: f64 = (p.p87 * s.dn[66][3]);
        let eq14_e1089_d_n4: f64 = (p.p87 * s.dn[66][4]);
        let eq14_e1089_d_n5: f64 = (p.p87 * s.dn[66][5]);
        let eq14_e1089_d_n6: f64 = (p.p87 * s.dn[66][6]);
        let eq14_e1089_d_n7: f64 = (p.p87 * s.dn[66][7]);
        let eq14_e1089_d_n8: f64 = (p.p87 * s.dn[66][8]);
        let eq14_e1089_d_n9: f64 = (p.p87 * s.dn[66][9]);
        let eq14_e1089_d_n10: f64 = (p.p87 * s.dn[66][10]);
        let eq14_e1089_d_n11: f64 = (p.p87 * s.dn[66][11]);
        let eq14_e1089_d_n12: f64 = (p.p87 * s.dn[66][12]);
        let eq14_e1089_d_n13: f64 = (p.p87 * s.dn[66][13]);
        let eq14_e1089_d_n14: f64 = (p.p87 * s.dn[66][14]);
        let eq14_e1089_d_n15: f64 = (p.p87 * s.dn[66][15]);
        let eq14_e1089_d_n16: f64 = (p.p87 * s.dn[66][16]);
        let eq14_e1089_d_n17: f64 = (p.p87 * s.dn[66][17]);
        let eq14_e1089_d_b0: f64 = (p.p87 * s.db[66][0]);
        let eq14_e1089_d_b1: f64 = (p.p87 * s.db[66][1]);
        let eq14_e1089_d_b2: f64 = (p.p87 * s.db[66][2]);
        let eq14_e1089_d_b3: f64 = (p.p87 * s.db[66][3]);
        let eq14_e1089_d_b4: f64 = (p.p87 * s.db[66][4]);
        let eq14_e1089_d_b5: f64 = (p.p87 * s.db[66][5]);
        let eq14_e1089_d_b6: f64 = (p.p87 * s.db[66][6]);
        let eq14_e1089_d_b7: f64 = (p.p87 * s.db[66][7]);
        let eq14_e1089_d_b8: f64 = (p.p87 * s.db[66][8]);
        let eq14_e1089_d_b9: f64 = (p.p87 * s.db[66][9]);
        let eq14_e1089_d_b10: f64 = (p.p87 * s.db[66][10]);
        let eq14_e1089_d_b11: f64 = (p.p87 * s.db[66][11]);
        let eq14_e1089_q: f64 = (p.p87 * eq14_e1088_q);
        let eq14_e1089_q_d_n0: f64 = (p.p87 * s.dn[66][0]);
        let eq14_e1089_q_d_n1: f64 = (p.p87 * s.dn[66][1]);
        let eq14_e1089_q_d_n2: f64 = (p.p87 * s.dn[66][2]);
        let eq14_e1089_q_d_n3: f64 = (p.p87 * s.dn[66][3]);
        let eq14_e1089_q_d_n4: f64 = (p.p87 * s.dn[66][4]);
        let eq14_e1089_q_d_n5: f64 = (p.p87 * s.dn[66][5]);
        let eq14_e1089_q_d_n6: f64 = (p.p87 * s.dn[66][6]);
        let eq14_e1089_q_d_n7: f64 = (p.p87 * s.dn[66][7]);
        let eq14_e1089_q_d_n8: f64 = (p.p87 * s.dn[66][8]);
        let eq14_e1089_q_d_n9: f64 = (p.p87 * s.dn[66][9]);
        let eq14_e1089_q_d_n10: f64 = (p.p87 * s.dn[66][10]);
        let eq14_e1089_q_d_n11: f64 = (p.p87 * s.dn[66][11]);
        let eq14_e1089_q_d_n12: f64 = (p.p87 * s.dn[66][12]);
        let eq14_e1089_q_d_n13: f64 = (p.p87 * s.dn[66][13]);
        let eq14_e1089_q_d_n14: f64 = (p.p87 * s.dn[66][14]);
        let eq14_e1089_q_d_n15: f64 = (p.p87 * s.dn[66][15]);
        let eq14_e1089_q_d_n16: f64 = (p.p87 * s.dn[66][16]);
        let eq14_e1089_q_d_n17: f64 = (p.p87 * s.dn[66][17]);
        let eq14_e1089_q_d_b0: f64 = (p.p87 * s.db[66][0]);
        let eq14_e1089_q_d_b1: f64 = (p.p87 * s.db[66][1]);
        let eq14_e1089_q_d_b2: f64 = (p.p87 * s.db[66][2]);
        let eq14_e1089_q_d_b3: f64 = (p.p87 * s.db[66][3]);
        let eq14_e1089_q_d_b4: f64 = (p.p87 * s.db[66][4]);
        let eq14_e1089_q_d_b5: f64 = (p.p87 * s.db[66][5]);
        let eq14_e1089_q_d_b6: f64 = (p.p87 * s.db[66][6]);
        let eq14_e1089_q_d_b7: f64 = (p.p87 * s.db[66][7]);
        let eq14_e1089_q_d_b8: f64 = (p.p87 * s.db[66][8]);
        let eq14_e1089_q_d_b9: f64 = (p.p87 * s.db[66][9]);
        let eq14_e1089_q_d_b10: f64 = (p.p87 * s.db[66][10]);
        let eq14_e1089_q_d_b11: f64 = (p.p87 * s.db[66][11]);
        let eq14_reactive_node_derivatives: [f64; 18] = [eq14_e1089_q_d_n0, eq14_e1089_q_d_n1, eq14_e1089_q_d_n2, eq14_e1089_q_d_n3, eq14_e1089_q_d_n4, eq14_e1089_q_d_n5, eq14_e1089_q_d_n6, eq14_e1089_q_d_n7, eq14_e1089_q_d_n8, eq14_e1089_q_d_n9, eq14_e1089_q_d_n10, eq14_e1089_q_d_n11, eq14_e1089_q_d_n12, eq14_e1089_q_d_n13, eq14_e1089_q_d_n14, eq14_e1089_q_d_n15, eq14_e1089_q_d_n16, eq14_e1089_q_d_n17];
        let eq14_reactive_branch_derivatives: [f64; 12] = [eq14_e1089_q_d_b0, eq14_e1089_q_d_b1, eq14_e1089_q_d_b2, eq14_e1089_q_d_b3, eq14_e1089_q_d_b4, eq14_e1089_q_d_b5, eq14_e1089_q_d_b6, eq14_e1089_q_d_b7, eq14_e1089_q_d_b8, eq14_e1089_q_d_b9, eq14_e1089_q_d_b10, eq14_e1089_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e1092_q: f64 = s.v[65];
        let eq15_e1093: f64 = (p.p87 * s.v[65]);
        let eq15_e1093_d_n0: f64 = (p.p87 * s.dn[65][0]);
        let eq15_e1093_d_n1: f64 = (p.p87 * s.dn[65][1]);
        let eq15_e1093_d_n2: f64 = (p.p87 * s.dn[65][2]);
        let eq15_e1093_d_n3: f64 = (p.p87 * s.dn[65][3]);
        let eq15_e1093_d_n4: f64 = (p.p87 * s.dn[65][4]);
        let eq15_e1093_d_n5: f64 = (p.p87 * s.dn[65][5]);
        let eq15_e1093_d_n6: f64 = (p.p87 * s.dn[65][6]);
        let eq15_e1093_d_n7: f64 = (p.p87 * s.dn[65][7]);
        let eq15_e1093_d_n8: f64 = (p.p87 * s.dn[65][8]);
        let eq15_e1093_d_n9: f64 = (p.p87 * s.dn[65][9]);
        let eq15_e1093_d_n10: f64 = (p.p87 * s.dn[65][10]);
        let eq15_e1093_d_n11: f64 = (p.p87 * s.dn[65][11]);
        let eq15_e1093_d_n12: f64 = (p.p87 * s.dn[65][12]);
        let eq15_e1093_d_n13: f64 = (p.p87 * s.dn[65][13]);
        let eq15_e1093_d_n14: f64 = (p.p87 * s.dn[65][14]);
        let eq15_e1093_d_n15: f64 = (p.p87 * s.dn[65][15]);
        let eq15_e1093_d_n16: f64 = (p.p87 * s.dn[65][16]);
        let eq15_e1093_d_n17: f64 = (p.p87 * s.dn[65][17]);
        let eq15_e1093_d_b0: f64 = (p.p87 * s.db[65][0]);
        let eq15_e1093_d_b1: f64 = (p.p87 * s.db[65][1]);
        let eq15_e1093_d_b2: f64 = (p.p87 * s.db[65][2]);
        let eq15_e1093_d_b3: f64 = (p.p87 * s.db[65][3]);
        let eq15_e1093_d_b4: f64 = (p.p87 * s.db[65][4]);
        let eq15_e1093_d_b5: f64 = (p.p87 * s.db[65][5]);
        let eq15_e1093_d_b6: f64 = (p.p87 * s.db[65][6]);
        let eq15_e1093_d_b7: f64 = (p.p87 * s.db[65][7]);
        let eq15_e1093_d_b8: f64 = (p.p87 * s.db[65][8]);
        let eq15_e1093_d_b9: f64 = (p.p87 * s.db[65][9]);
        let eq15_e1093_d_b10: f64 = (p.p87 * s.db[65][10]);
        let eq15_e1093_d_b11: f64 = (p.p87 * s.db[65][11]);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_e1093_q_d_n0: f64 = (p.p87 * s.dn[65][0]);
        let eq15_e1093_q_d_n1: f64 = (p.p87 * s.dn[65][1]);
        let eq15_e1093_q_d_n2: f64 = (p.p87 * s.dn[65][2]);
        let eq15_e1093_q_d_n3: f64 = (p.p87 * s.dn[65][3]);
        let eq15_e1093_q_d_n4: f64 = (p.p87 * s.dn[65][4]);
        let eq15_e1093_q_d_n5: f64 = (p.p87 * s.dn[65][5]);
        let eq15_e1093_q_d_n6: f64 = (p.p87 * s.dn[65][6]);
        let eq15_e1093_q_d_n7: f64 = (p.p87 * s.dn[65][7]);
        let eq15_e1093_q_d_n8: f64 = (p.p87 * s.dn[65][8]);
        let eq15_e1093_q_d_n9: f64 = (p.p87 * s.dn[65][9]);
        let eq15_e1093_q_d_n10: f64 = (p.p87 * s.dn[65][10]);
        let eq15_e1093_q_d_n11: f64 = (p.p87 * s.dn[65][11]);
        let eq15_e1093_q_d_n12: f64 = (p.p87 * s.dn[65][12]);
        let eq15_e1093_q_d_n13: f64 = (p.p87 * s.dn[65][13]);
        let eq15_e1093_q_d_n14: f64 = (p.p87 * s.dn[65][14]);
        let eq15_e1093_q_d_n15: f64 = (p.p87 * s.dn[65][15]);
        let eq15_e1093_q_d_n16: f64 = (p.p87 * s.dn[65][16]);
        let eq15_e1093_q_d_n17: f64 = (p.p87 * s.dn[65][17]);
        let eq15_e1093_q_d_b0: f64 = (p.p87 * s.db[65][0]);
        let eq15_e1093_q_d_b1: f64 = (p.p87 * s.db[65][1]);
        let eq15_e1093_q_d_b2: f64 = (p.p87 * s.db[65][2]);
        let eq15_e1093_q_d_b3: f64 = (p.p87 * s.db[65][3]);
        let eq15_e1093_q_d_b4: f64 = (p.p87 * s.db[65][4]);
        let eq15_e1093_q_d_b5: f64 = (p.p87 * s.db[65][5]);
        let eq15_e1093_q_d_b6: f64 = (p.p87 * s.db[65][6]);
        let eq15_e1093_q_d_b7: f64 = (p.p87 * s.db[65][7]);
        let eq15_e1093_q_d_b8: f64 = (p.p87 * s.db[65][8]);
        let eq15_e1093_q_d_b9: f64 = (p.p87 * s.db[65][9]);
        let eq15_e1093_q_d_b10: f64 = (p.p87 * s.db[65][10]);
        let eq15_e1093_q_d_b11: f64 = (p.p87 * s.db[65][11]);
        let eq15_reactive_node_derivatives: [f64; 18] = [eq15_e1093_q_d_n0, eq15_e1093_q_d_n1, eq15_e1093_q_d_n2, eq15_e1093_q_d_n3, eq15_e1093_q_d_n4, eq15_e1093_q_d_n5, eq15_e1093_q_d_n6, eq15_e1093_q_d_n7, eq15_e1093_q_d_n8, eq15_e1093_q_d_n9, eq15_e1093_q_d_n10, eq15_e1093_q_d_n11, eq15_e1093_q_d_n12, eq15_e1093_q_d_n13, eq15_e1093_q_d_n14, eq15_e1093_q_d_n15, eq15_e1093_q_d_n16, eq15_e1093_q_d_n17];
        let eq15_reactive_branch_derivatives: [f64; 12] = [eq15_e1093_q_d_b0, eq15_e1093_q_d_b1, eq15_e1093_q_d_b2, eq15_e1093_q_d_b3, eq15_e1093_q_d_b4, eq15_e1093_q_d_b5, eq15_e1093_q_d_b6, eq15_e1093_q_d_b7, eq15_e1093_q_d_b8, eq15_e1093_q_d_b9, eq15_e1093_q_d_b10, eq15_e1093_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[0]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17, eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11, eq18_e1112_q, eq18_e1112_q_d_n0, eq18_e1112_q_d_n1, eq18_e1112_q_d_n2, eq18_e1112_q_d_n3, eq18_e1112_q_d_n4, eq18_e1112_q_d_n5, eq18_e1112_q_d_n6, eq18_e1112_q_d_n7, eq18_e1112_q_d_n8, eq18_e1112_q_d_n9, eq18_e1112_q_d_n10, eq18_e1112_q_d_n11, eq18_e1112_q_d_n12, eq18_e1112_q_d_n13, eq18_e1112_q_d_n14, eq18_e1112_q_d_n15, eq18_e1112_q_d_n16, eq18_e1112_q_d_n17, eq18_e1112_q_d_b0, eq18_e1112_q_d_b1, eq18_e1112_q_d_b2, eq18_e1112_q_d_b3, eq18_e1112_q_d_b4, eq18_e1112_q_d_b5, eq18_e1112_q_d_b6, eq18_e1112_q_d_b7, eq18_e1112_q_d_b8, eq18_e1112_q_d_b9, eq18_e1112_q_d_b10, eq18_e1112_q_d_b11,) = {
    if s.b[3405] {
        let eq18_e1109_q: f64 = s.v[68];
        let eq18_e1110: f64 = (p.p87 * s.v[68]);
        let eq18_e1110_d_n0: f64 = (p.p87 * s.dn[68][0]);
        let eq18_e1110_d_n1: f64 = (p.p87 * s.dn[68][1]);
        let eq18_e1110_d_n2: f64 = (p.p87 * s.dn[68][2]);
        let eq18_e1110_d_n3: f64 = (p.p87 * s.dn[68][3]);
        let eq18_e1110_d_n4: f64 = (p.p87 * s.dn[68][4]);
        let eq18_e1110_d_n5: f64 = (p.p87 * s.dn[68][5]);
        let eq18_e1110_d_n6: f64 = (p.p87 * s.dn[68][6]);
        let eq18_e1110_d_n7: f64 = (p.p87 * s.dn[68][7]);
        let eq18_e1110_d_n8: f64 = (p.p87 * s.dn[68][8]);
        let eq18_e1110_d_n9: f64 = (p.p87 * s.dn[68][9]);
        let eq18_e1110_d_n10: f64 = (p.p87 * s.dn[68][10]);
        let eq18_e1110_d_n11: f64 = (p.p87 * s.dn[68][11]);
        let eq18_e1110_d_n12: f64 = (p.p87 * s.dn[68][12]);
        let eq18_e1110_d_n13: f64 = (p.p87 * s.dn[68][13]);
        let eq18_e1110_d_n14: f64 = (p.p87 * s.dn[68][14]);
        let eq18_e1110_d_n15: f64 = (p.p87 * s.dn[68][15]);
        let eq18_e1110_d_n16: f64 = (p.p87 * s.dn[68][16]);
        let eq18_e1110_d_n17: f64 = (p.p87 * s.dn[68][17]);
        let eq18_e1110_d_b0: f64 = (p.p87 * s.db[68][0]);
        let eq18_e1110_d_b1: f64 = (p.p87 * s.db[68][1]);
        let eq18_e1110_d_b2: f64 = (p.p87 * s.db[68][2]);
        let eq18_e1110_d_b3: f64 = (p.p87 * s.db[68][3]);
        let eq18_e1110_d_b4: f64 = (p.p87 * s.db[68][4]);
        let eq18_e1110_d_b5: f64 = (p.p87 * s.db[68][5]);
        let eq18_e1110_d_b6: f64 = (p.p87 * s.db[68][6]);
        let eq18_e1110_d_b7: f64 = (p.p87 * s.db[68][7]);
        let eq18_e1110_d_b8: f64 = (p.p87 * s.db[68][8]);
        let eq18_e1110_d_b9: f64 = (p.p87 * s.db[68][9]);
        let eq18_e1110_d_b10: f64 = (p.p87 * s.db[68][10]);
        let eq18_e1110_d_b11: f64 = (p.p87 * s.db[68][11]);
        let eq18_e1110_q: f64 = (p.p87 * eq18_e1109_q);
        let eq18_e1110_q_d_n0: f64 = (p.p87 * s.dn[68][0]);
        let eq18_e1110_q_d_n1: f64 = (p.p87 * s.dn[68][1]);
        let eq18_e1110_q_d_n2: f64 = (p.p87 * s.dn[68][2]);
        let eq18_e1110_q_d_n3: f64 = (p.p87 * s.dn[68][3]);
        let eq18_e1110_q_d_n4: f64 = (p.p87 * s.dn[68][4]);
        let eq18_e1110_q_d_n5: f64 = (p.p87 * s.dn[68][5]);
        let eq18_e1110_q_d_n6: f64 = (p.p87 * s.dn[68][6]);
        let eq18_e1110_q_d_n7: f64 = (p.p87 * s.dn[68][7]);
        let eq18_e1110_q_d_n8: f64 = (p.p87 * s.dn[68][8]);
        let eq18_e1110_q_d_n9: f64 = (p.p87 * s.dn[68][9]);
        let eq18_e1110_q_d_n10: f64 = (p.p87 * s.dn[68][10]);
        let eq18_e1110_q_d_n11: f64 = (p.p87 * s.dn[68][11]);
        let eq18_e1110_q_d_n12: f64 = (p.p87 * s.dn[68][12]);
        let eq18_e1110_q_d_n13: f64 = (p.p87 * s.dn[68][13]);
        let eq18_e1110_q_d_n14: f64 = (p.p87 * s.dn[68][14]);
        let eq18_e1110_q_d_n15: f64 = (p.p87 * s.dn[68][15]);
        let eq18_e1110_q_d_n16: f64 = (p.p87 * s.dn[68][16]);
        let eq18_e1110_q_d_n17: f64 = (p.p87 * s.dn[68][17]);
        let eq18_e1110_q_d_b0: f64 = (p.p87 * s.db[68][0]);
        let eq18_e1110_q_d_b1: f64 = (p.p87 * s.db[68][1]);
        let eq18_e1110_q_d_b2: f64 = (p.p87 * s.db[68][2]);
        let eq18_e1110_q_d_b3: f64 = (p.p87 * s.db[68][3]);
        let eq18_e1110_q_d_b4: f64 = (p.p87 * s.db[68][4]);
        let eq18_e1110_q_d_b5: f64 = (p.p87 * s.db[68][5]);
        let eq18_e1110_q_d_b6: f64 = (p.p87 * s.db[68][6]);
        let eq18_e1110_q_d_b7: f64 = (p.p87 * s.db[68][7]);
        let eq18_e1110_q_d_b8: f64 = (p.p87 * s.db[68][8]);
        let eq18_e1110_q_d_b9: f64 = (p.p87 * s.db[68][9]);
        let eq18_e1110_q_d_b10: f64 = (p.p87 * s.db[68][10]);
        let eq18_e1110_q_d_b11: f64 = (p.p87 * s.db[68][11]);
        (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_n12, eq18_e1110_d_n13, eq18_e1110_d_n14, eq18_e1110_d_n15, eq18_e1110_d_n16, eq18_e1110_d_n17, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6, eq18_e1110_d_b7, eq18_e1110_d_b8, eq18_e1110_d_b9, eq18_e1110_d_b10, eq18_e1110_d_b11, eq18_e1110_q, eq18_e1110_q_d_n0, eq18_e1110_q_d_n1, eq18_e1110_q_d_n2, eq18_e1110_q_d_n3, eq18_e1110_q_d_n4, eq18_e1110_q_d_n5, eq18_e1110_q_d_n6, eq18_e1110_q_d_n7, eq18_e1110_q_d_n8, eq18_e1110_q_d_n9, eq18_e1110_q_d_n10, eq18_e1110_q_d_n11, eq18_e1110_q_d_n12, eq18_e1110_q_d_n13, eq18_e1110_q_d_n14, eq18_e1110_q_d_n15, eq18_e1110_q_d_n16, eq18_e1110_q_d_n17, eq18_e1110_q_d_b0, eq18_e1110_q_d_b1, eq18_e1110_q_d_b2, eq18_e1110_q_d_b3, eq18_e1110_q_d_b4, eq18_e1110_q_d_b5, eq18_e1110_q_d_b6, eq18_e1110_q_d_b7, eq18_e1110_q_d_b8, eq18_e1110_q_d_b9, eq18_e1110_q_d_b10, eq18_e1110_q_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 18] = [eq18_e1112_q_d_n0, eq18_e1112_q_d_n1, eq18_e1112_q_d_n2, eq18_e1112_q_d_n3, eq18_e1112_q_d_n4, eq18_e1112_q_d_n5, eq18_e1112_q_d_n6, eq18_e1112_q_d_n7, eq18_e1112_q_d_n8, eq18_e1112_q_d_n9, eq18_e1112_q_d_n10, eq18_e1112_q_d_n11, eq18_e1112_q_d_n12, eq18_e1112_q_d_n13, eq18_e1112_q_d_n14, eq18_e1112_q_d_n15, eq18_e1112_q_d_n16, eq18_e1112_q_d_n17];
        let eq18_reactive_branch_derivatives: [f64; 12] = [eq18_e1112_q_d_b0, eq18_e1112_q_d_b1, eq18_e1112_q_d_b2, eq18_e1112_q_d_b3, eq18_e1112_q_d_b4, eq18_e1112_q_d_b5, eq18_e1112_q_d_b6, eq18_e1112_q_d_b7, eq18_e1112_q_d_b8, eq18_e1112_q_d_b9, eq18_e1112_q_d_b10, eq18_e1112_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17, eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11, eq19_e1119_q, eq19_e1119_q_d_n0, eq19_e1119_q_d_n1, eq19_e1119_q_d_n2, eq19_e1119_q_d_n3, eq19_e1119_q_d_n4, eq19_e1119_q_d_n5, eq19_e1119_q_d_n6, eq19_e1119_q_d_n7, eq19_e1119_q_d_n8, eq19_e1119_q_d_n9, eq19_e1119_q_d_n10, eq19_e1119_q_d_n11, eq19_e1119_q_d_n12, eq19_e1119_q_d_n13, eq19_e1119_q_d_n14, eq19_e1119_q_d_n15, eq19_e1119_q_d_n16, eq19_e1119_q_d_n17, eq19_e1119_q_d_b0, eq19_e1119_q_d_b1, eq19_e1119_q_d_b2, eq19_e1119_q_d_b3, eq19_e1119_q_d_b4, eq19_e1119_q_d_b5, eq19_e1119_q_d_b6, eq19_e1119_q_d_b7, eq19_e1119_q_d_b8, eq19_e1119_q_d_b9, eq19_e1119_q_d_b10, eq19_e1119_q_d_b11,) = {
    if s.b[3405] {
        let eq19_e1116_q: f64 = s.v[67];
        let eq19_e1117: f64 = (p.p87 * s.v[67]);
        let eq19_e1117_d_n0: f64 = (p.p87 * s.dn[67][0]);
        let eq19_e1117_d_n1: f64 = (p.p87 * s.dn[67][1]);
        let eq19_e1117_d_n2: f64 = (p.p87 * s.dn[67][2]);
        let eq19_e1117_d_n3: f64 = (p.p87 * s.dn[67][3]);
        let eq19_e1117_d_n4: f64 = (p.p87 * s.dn[67][4]);
        let eq19_e1117_d_n5: f64 = (p.p87 * s.dn[67][5]);
        let eq19_e1117_d_n6: f64 = (p.p87 * s.dn[67][6]);
        let eq19_e1117_d_n7: f64 = (p.p87 * s.dn[67][7]);
        let eq19_e1117_d_n8: f64 = (p.p87 * s.dn[67][8]);
        let eq19_e1117_d_n9: f64 = (p.p87 * s.dn[67][9]);
        let eq19_e1117_d_n10: f64 = (p.p87 * s.dn[67][10]);
        let eq19_e1117_d_n11: f64 = (p.p87 * s.dn[67][11]);
        let eq19_e1117_d_n12: f64 = (p.p87 * s.dn[67][12]);
        let eq19_e1117_d_n13: f64 = (p.p87 * s.dn[67][13]);
        let eq19_e1117_d_n14: f64 = (p.p87 * s.dn[67][14]);
        let eq19_e1117_d_n15: f64 = (p.p87 * s.dn[67][15]);
        let eq19_e1117_d_n16: f64 = (p.p87 * s.dn[67][16]);
        let eq19_e1117_d_n17: f64 = (p.p87 * s.dn[67][17]);
        let eq19_e1117_d_b0: f64 = (p.p87 * s.db[67][0]);
        let eq19_e1117_d_b1: f64 = (p.p87 * s.db[67][1]);
        let eq19_e1117_d_b2: f64 = (p.p87 * s.db[67][2]);
        let eq19_e1117_d_b3: f64 = (p.p87 * s.db[67][3]);
        let eq19_e1117_d_b4: f64 = (p.p87 * s.db[67][4]);
        let eq19_e1117_d_b5: f64 = (p.p87 * s.db[67][5]);
        let eq19_e1117_d_b6: f64 = (p.p87 * s.db[67][6]);
        let eq19_e1117_d_b7: f64 = (p.p87 * s.db[67][7]);
        let eq19_e1117_d_b8: f64 = (p.p87 * s.db[67][8]);
        let eq19_e1117_d_b9: f64 = (p.p87 * s.db[67][9]);
        let eq19_e1117_d_b10: f64 = (p.p87 * s.db[67][10]);
        let eq19_e1117_d_b11: f64 = (p.p87 * s.db[67][11]);
        let eq19_e1117_q: f64 = (p.p87 * eq19_e1116_q);
        let eq19_e1117_q_d_n0: f64 = (p.p87 * s.dn[67][0]);
        let eq19_e1117_q_d_n1: f64 = (p.p87 * s.dn[67][1]);
        let eq19_e1117_q_d_n2: f64 = (p.p87 * s.dn[67][2]);
        let eq19_e1117_q_d_n3: f64 = (p.p87 * s.dn[67][3]);
        let eq19_e1117_q_d_n4: f64 = (p.p87 * s.dn[67][4]);
        let eq19_e1117_q_d_n5: f64 = (p.p87 * s.dn[67][5]);
        let eq19_e1117_q_d_n6: f64 = (p.p87 * s.dn[67][6]);
        let eq19_e1117_q_d_n7: f64 = (p.p87 * s.dn[67][7]);
        let eq19_e1117_q_d_n8: f64 = (p.p87 * s.dn[67][8]);
        let eq19_e1117_q_d_n9: f64 = (p.p87 * s.dn[67][9]);
        let eq19_e1117_q_d_n10: f64 = (p.p87 * s.dn[67][10]);
        let eq19_e1117_q_d_n11: f64 = (p.p87 * s.dn[67][11]);
        let eq19_e1117_q_d_n12: f64 = (p.p87 * s.dn[67][12]);
        let eq19_e1117_q_d_n13: f64 = (p.p87 * s.dn[67][13]);
        let eq19_e1117_q_d_n14: f64 = (p.p87 * s.dn[67][14]);
        let eq19_e1117_q_d_n15: f64 = (p.p87 * s.dn[67][15]);
        let eq19_e1117_q_d_n16: f64 = (p.p87 * s.dn[67][16]);
        let eq19_e1117_q_d_n17: f64 = (p.p87 * s.dn[67][17]);
        let eq19_e1117_q_d_b0: f64 = (p.p87 * s.db[67][0]);
        let eq19_e1117_q_d_b1: f64 = (p.p87 * s.db[67][1]);
        let eq19_e1117_q_d_b2: f64 = (p.p87 * s.db[67][2]);
        let eq19_e1117_q_d_b3: f64 = (p.p87 * s.db[67][3]);
        let eq19_e1117_q_d_b4: f64 = (p.p87 * s.db[67][4]);
        let eq19_e1117_q_d_b5: f64 = (p.p87 * s.db[67][5]);
        let eq19_e1117_q_d_b6: f64 = (p.p87 * s.db[67][6]);
        let eq19_e1117_q_d_b7: f64 = (p.p87 * s.db[67][7]);
        let eq19_e1117_q_d_b8: f64 = (p.p87 * s.db[67][8]);
        let eq19_e1117_q_d_b9: f64 = (p.p87 * s.db[67][9]);
        let eq19_e1117_q_d_b10: f64 = (p.p87 * s.db[67][10]);
        let eq19_e1117_q_d_b11: f64 = (p.p87 * s.db[67][11]);
        (eq19_e1117, eq19_e1117_d_n0, eq19_e1117_d_n1, eq19_e1117_d_n2, eq19_e1117_d_n3, eq19_e1117_d_n4, eq19_e1117_d_n5, eq19_e1117_d_n6, eq19_e1117_d_n7, eq19_e1117_d_n8, eq19_e1117_d_n9, eq19_e1117_d_n10, eq19_e1117_d_n11, eq19_e1117_d_n12, eq19_e1117_d_n13, eq19_e1117_d_n14, eq19_e1117_d_n15, eq19_e1117_d_n16, eq19_e1117_d_n17, eq19_e1117_d_b0, eq19_e1117_d_b1, eq19_e1117_d_b2, eq19_e1117_d_b3, eq19_e1117_d_b4, eq19_e1117_d_b5, eq19_e1117_d_b6, eq19_e1117_d_b7, eq19_e1117_d_b8, eq19_e1117_d_b9, eq19_e1117_d_b10, eq19_e1117_d_b11, eq19_e1117_q, eq19_e1117_q_d_n0, eq19_e1117_q_d_n1, eq19_e1117_q_d_n2, eq19_e1117_q_d_n3, eq19_e1117_q_d_n4, eq19_e1117_q_d_n5, eq19_e1117_q_d_n6, eq19_e1117_q_d_n7, eq19_e1117_q_d_n8, eq19_e1117_q_d_n9, eq19_e1117_q_d_n10, eq19_e1117_q_d_n11, eq19_e1117_q_d_n12, eq19_e1117_q_d_n13, eq19_e1117_q_d_n14, eq19_e1117_q_d_n15, eq19_e1117_q_d_n16, eq19_e1117_q_d_n17, eq19_e1117_q_d_b0, eq19_e1117_q_d_b1, eq19_e1117_q_d_b2, eq19_e1117_q_d_b3, eq19_e1117_q_d_b4, eq19_e1117_q_d_b5, eq19_e1117_q_d_b6, eq19_e1117_q_d_b7, eq19_e1117_q_d_b8, eq19_e1117_q_d_b9, eq19_e1117_q_d_b10, eq19_e1117_q_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 18] = [eq19_e1119_q_d_n0, eq19_e1119_q_d_n1, eq19_e1119_q_d_n2, eq19_e1119_q_d_n3, eq19_e1119_q_d_n4, eq19_e1119_q_d_n5, eq19_e1119_q_d_n6, eq19_e1119_q_d_n7, eq19_e1119_q_d_n8, eq19_e1119_q_d_n9, eq19_e1119_q_d_n10, eq19_e1119_q_d_n11, eq19_e1119_q_d_n12, eq19_e1119_q_d_n13, eq19_e1119_q_d_n14, eq19_e1119_q_d_n15, eq19_e1119_q_d_n16, eq19_e1119_q_d_n17];
        let eq19_reactive_branch_derivatives: [f64; 12] = [eq19_e1119_q_d_b0, eq19_e1119_q_d_b1, eq19_e1119_q_d_b2, eq19_e1119_q_d_b3, eq19_e1119_q_d_b4, eq19_e1119_q_d_b5, eq19_e1119_q_d_b6, eq19_e1119_q_d_b7, eq19_e1119_q_d_b8, eq19_e1119_q_d_b9, eq19_e1119_q_d_b10, eq19_e1119_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq27_e1163: f64 = (s.v[18] + s.v[753]);
        let eq27_e1163_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq27_e1163_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq27_e1163_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq27_e1163_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq27_e1163_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq27_e1163_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq27_e1163_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq27_e1163_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq27_e1163_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq27_e1163_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq27_e1163_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq27_e1163_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq27_e1163_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq27_e1163_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq27_e1163_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq27_e1163_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq27_e1163_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq27_e1163_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq27_e1163_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq27_e1163_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq27_e1163_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq27_e1163_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq27_e1163_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq27_e1163_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq27_e1163_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq27_e1163_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq27_e1163_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq27_e1163_d_b9: f64 = (s.db[18][9] + s.db[753][9]);
        let eq27_e1163_d_b10: f64 = (s.db[18][10] + s.db[753][10]);
        let eq27_e1163_d_b11: f64 = (s.db[18][11] + s.db[753][11]);
        let eq27_e1164_q: f64 = eq27_e1163;
        let eq27_e1165: f64 = (p.p87 * eq27_e1163);
        let eq27_e1165_d_n0: f64 = (p.p87 * eq27_e1163_d_n0);
        let eq27_e1165_d_n1: f64 = (p.p87 * eq27_e1163_d_n1);
        let eq27_e1165_d_n2: f64 = (p.p87 * eq27_e1163_d_n2);
        let eq27_e1165_d_n3: f64 = (p.p87 * eq27_e1163_d_n3);
        let eq27_e1165_d_n4: f64 = (p.p87 * eq27_e1163_d_n4);
        let eq27_e1165_d_n5: f64 = (p.p87 * eq27_e1163_d_n5);
        let eq27_e1165_d_n6: f64 = (p.p87 * eq27_e1163_d_n6);
        let eq27_e1165_d_n7: f64 = (p.p87 * eq27_e1163_d_n7);
        let eq27_e1165_d_n8: f64 = (p.p87 * eq27_e1163_d_n8);
        let eq27_e1165_d_n9: f64 = (p.p87 * eq27_e1163_d_n9);
        let eq27_e1165_d_n10: f64 = (p.p87 * eq27_e1163_d_n10);
        let eq27_e1165_d_n11: f64 = (p.p87 * eq27_e1163_d_n11);
        let eq27_e1165_d_n12: f64 = (p.p87 * eq27_e1163_d_n12);
        let eq27_e1165_d_n13: f64 = (p.p87 * eq27_e1163_d_n13);
        let eq27_e1165_d_n14: f64 = (p.p87 * eq27_e1163_d_n14);
        let eq27_e1165_d_n15: f64 = (p.p87 * eq27_e1163_d_n15);
        let eq27_e1165_d_n16: f64 = (p.p87 * eq27_e1163_d_n16);
        let eq27_e1165_d_n17: f64 = (p.p87 * eq27_e1163_d_n17);
        let eq27_e1165_d_b0: f64 = (p.p87 * eq27_e1163_d_b0);
        let eq27_e1165_d_b1: f64 = (p.p87 * eq27_e1163_d_b1);
        let eq27_e1165_d_b2: f64 = (p.p87 * eq27_e1163_d_b2);
        let eq27_e1165_d_b3: f64 = (p.p87 * eq27_e1163_d_b3);
        let eq27_e1165_d_b4: f64 = (p.p87 * eq27_e1163_d_b4);
        let eq27_e1165_d_b5: f64 = (p.p87 * eq27_e1163_d_b5);
        let eq27_e1165_d_b6: f64 = (p.p87 * eq27_e1163_d_b6);
        let eq27_e1165_d_b7: f64 = (p.p87 * eq27_e1163_d_b7);
        let eq27_e1165_d_b8: f64 = (p.p87 * eq27_e1163_d_b8);
        let eq27_e1165_d_b9: f64 = (p.p87 * eq27_e1163_d_b9);
        let eq27_e1165_d_b10: f64 = (p.p87 * eq27_e1163_d_b10);
        let eq27_e1165_d_b11: f64 = (p.p87 * eq27_e1163_d_b11);
        let eq27_e1165_q: f64 = (p.p87 * eq27_e1164_q);
        let eq27_e1165_q_d_n0: f64 = (p.p87 * eq27_e1163_d_n0);
        let eq27_e1165_q_d_n1: f64 = (p.p87 * eq27_e1163_d_n1);
        let eq27_e1165_q_d_n2: f64 = (p.p87 * eq27_e1163_d_n2);
        let eq27_e1165_q_d_n3: f64 = (p.p87 * eq27_e1163_d_n3);
        let eq27_e1165_q_d_n4: f64 = (p.p87 * eq27_e1163_d_n4);
        let eq27_e1165_q_d_n5: f64 = (p.p87 * eq27_e1163_d_n5);
        let eq27_e1165_q_d_n6: f64 = (p.p87 * eq27_e1163_d_n6);
        let eq27_e1165_q_d_n7: f64 = (p.p87 * eq27_e1163_d_n7);
        let eq27_e1165_q_d_n8: f64 = (p.p87 * eq27_e1163_d_n8);
        let eq27_e1165_q_d_n9: f64 = (p.p87 * eq27_e1163_d_n9);
        let eq27_e1165_q_d_n10: f64 = (p.p87 * eq27_e1163_d_n10);
        let eq27_e1165_q_d_n11: f64 = (p.p87 * eq27_e1163_d_n11);
        let eq27_e1165_q_d_n12: f64 = (p.p87 * eq27_e1163_d_n12);
        let eq27_e1165_q_d_n13: f64 = (p.p87 * eq27_e1163_d_n13);
        let eq27_e1165_q_d_n14: f64 = (p.p87 * eq27_e1163_d_n14);
        let eq27_e1165_q_d_n15: f64 = (p.p87 * eq27_e1163_d_n15);
        let eq27_e1165_q_d_n16: f64 = (p.p87 * eq27_e1163_d_n16);
        let eq27_e1165_q_d_n17: f64 = (p.p87 * eq27_e1163_d_n17);
        let eq27_e1165_q_d_b0: f64 = (p.p87 * eq27_e1163_d_b0);
        let eq27_e1165_q_d_b1: f64 = (p.p87 * eq27_e1163_d_b1);
        let eq27_e1165_q_d_b2: f64 = (p.p87 * eq27_e1163_d_b2);
        let eq27_e1165_q_d_b3: f64 = (p.p87 * eq27_e1163_d_b3);
        let eq27_e1165_q_d_b4: f64 = (p.p87 * eq27_e1163_d_b4);
        let eq27_e1165_q_d_b5: f64 = (p.p87 * eq27_e1163_d_b5);
        let eq27_e1165_q_d_b6: f64 = (p.p87 * eq27_e1163_d_b6);
        let eq27_e1165_q_d_b7: f64 = (p.p87 * eq27_e1163_d_b7);
        let eq27_e1165_q_d_b8: f64 = (p.p87 * eq27_e1163_d_b8);
        let eq27_e1165_q_d_b9: f64 = (p.p87 * eq27_e1163_d_b9);
        let eq27_e1165_q_d_b10: f64 = (p.p87 * eq27_e1163_d_b10);
        let eq27_e1165_q_d_b11: f64 = (p.p87 * eq27_e1163_d_b11);
        let eq27_reactive_node_derivatives: [f64; 18] = [eq27_e1165_q_d_n0, eq27_e1165_q_d_n1, eq27_e1165_q_d_n2, eq27_e1165_q_d_n3, eq27_e1165_q_d_n4, eq27_e1165_q_d_n5, eq27_e1165_q_d_n6, eq27_e1165_q_d_n7, eq27_e1165_q_d_n8, eq27_e1165_q_d_n9, eq27_e1165_q_d_n10, eq27_e1165_q_d_n11, eq27_e1165_q_d_n12, eq27_e1165_q_d_n13, eq27_e1165_q_d_n14, eq27_e1165_q_d_n15, eq27_e1165_q_d_n16, eq27_e1165_q_d_n17];
        let eq27_reactive_branch_derivatives: [f64; 12] = [eq27_e1165_q_d_b0, eq27_e1165_q_d_b1, eq27_e1165_q_d_b2, eq27_e1165_q_d_b3, eq27_e1165_q_d_b4, eq27_e1165_q_d_b5, eq27_e1165_q_d_b6, eq27_e1165_q_d_b7, eq27_e1165_q_d_b8, eq27_e1165_q_d_b9, eq27_e1165_q_d_b10, eq27_e1165_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e1169: f64 = (s.v[19] + s.v[751]);
        let eq28_e1169_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq28_e1169_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq28_e1169_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq28_e1169_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq28_e1169_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq28_e1169_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq28_e1169_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq28_e1169_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq28_e1169_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq28_e1169_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq28_e1169_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq28_e1169_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq28_e1169_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq28_e1169_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq28_e1169_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq28_e1169_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq28_e1169_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq28_e1169_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq28_e1169_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq28_e1169_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq28_e1169_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq28_e1169_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq28_e1169_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq28_e1169_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq28_e1169_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq28_e1169_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq28_e1169_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq28_e1169_d_b9: f64 = (s.db[19][9] + s.db[751][9]);
        let eq28_e1169_d_b10: f64 = (s.db[19][10] + s.db[751][10]);
        let eq28_e1169_d_b11: f64 = (s.db[19][11] + s.db[751][11]);
        let eq28_e1170_q: f64 = eq28_e1169;
        let eq28_e1171: f64 = (p.p87 * eq28_e1169);
        let eq28_e1171_d_n0: f64 = (p.p87 * eq28_e1169_d_n0);
        let eq28_e1171_d_n1: f64 = (p.p87 * eq28_e1169_d_n1);
        let eq28_e1171_d_n2: f64 = (p.p87 * eq28_e1169_d_n2);
        let eq28_e1171_d_n3: f64 = (p.p87 * eq28_e1169_d_n3);
        let eq28_e1171_d_n4: f64 = (p.p87 * eq28_e1169_d_n4);
        let eq28_e1171_d_n5: f64 = (p.p87 * eq28_e1169_d_n5);
        let eq28_e1171_d_n6: f64 = (p.p87 * eq28_e1169_d_n6);
        let eq28_e1171_d_n7: f64 = (p.p87 * eq28_e1169_d_n7);
        let eq28_e1171_d_n8: f64 = (p.p87 * eq28_e1169_d_n8);
        let eq28_e1171_d_n9: f64 = (p.p87 * eq28_e1169_d_n9);
        let eq28_e1171_d_n10: f64 = (p.p87 * eq28_e1169_d_n10);
        let eq28_e1171_d_n11: f64 = (p.p87 * eq28_e1169_d_n11);
        let eq28_e1171_d_n12: f64 = (p.p87 * eq28_e1169_d_n12);
        let eq28_e1171_d_n13: f64 = (p.p87 * eq28_e1169_d_n13);
        let eq28_e1171_d_n14: f64 = (p.p87 * eq28_e1169_d_n14);
        let eq28_e1171_d_n15: f64 = (p.p87 * eq28_e1169_d_n15);
        let eq28_e1171_d_n16: f64 = (p.p87 * eq28_e1169_d_n16);
        let eq28_e1171_d_n17: f64 = (p.p87 * eq28_e1169_d_n17);
        let eq28_e1171_d_b0: f64 = (p.p87 * eq28_e1169_d_b0);
        let eq28_e1171_d_b1: f64 = (p.p87 * eq28_e1169_d_b1);
        let eq28_e1171_d_b2: f64 = (p.p87 * eq28_e1169_d_b2);
        let eq28_e1171_d_b3: f64 = (p.p87 * eq28_e1169_d_b3);
        let eq28_e1171_d_b4: f64 = (p.p87 * eq28_e1169_d_b4);
        let eq28_e1171_d_b5: f64 = (p.p87 * eq28_e1169_d_b5);
        let eq28_e1171_d_b6: f64 = (p.p87 * eq28_e1169_d_b6);
        let eq28_e1171_d_b7: f64 = (p.p87 * eq28_e1169_d_b7);
        let eq28_e1171_d_b8: f64 = (p.p87 * eq28_e1169_d_b8);
        let eq28_e1171_d_b9: f64 = (p.p87 * eq28_e1169_d_b9);
        let eq28_e1171_d_b10: f64 = (p.p87 * eq28_e1169_d_b10);
        let eq28_e1171_d_b11: f64 = (p.p87 * eq28_e1169_d_b11);
        let eq28_e1171_q: f64 = (p.p87 * eq28_e1170_q);
        let eq28_e1171_q_d_n0: f64 = (p.p87 * eq28_e1169_d_n0);
        let eq28_e1171_q_d_n1: f64 = (p.p87 * eq28_e1169_d_n1);
        let eq28_e1171_q_d_n2: f64 = (p.p87 * eq28_e1169_d_n2);
        let eq28_e1171_q_d_n3: f64 = (p.p87 * eq28_e1169_d_n3);
        let eq28_e1171_q_d_n4: f64 = (p.p87 * eq28_e1169_d_n4);
        let eq28_e1171_q_d_n5: f64 = (p.p87 * eq28_e1169_d_n5);
        let eq28_e1171_q_d_n6: f64 = (p.p87 * eq28_e1169_d_n6);
        let eq28_e1171_q_d_n7: f64 = (p.p87 * eq28_e1169_d_n7);
        let eq28_e1171_q_d_n8: f64 = (p.p87 * eq28_e1169_d_n8);
        let eq28_e1171_q_d_n9: f64 = (p.p87 * eq28_e1169_d_n9);
        let eq28_e1171_q_d_n10: f64 = (p.p87 * eq28_e1169_d_n10);
        let eq28_e1171_q_d_n11: f64 = (p.p87 * eq28_e1169_d_n11);
        let eq28_e1171_q_d_n12: f64 = (p.p87 * eq28_e1169_d_n12);
        let eq28_e1171_q_d_n13: f64 = (p.p87 * eq28_e1169_d_n13);
        let eq28_e1171_q_d_n14: f64 = (p.p87 * eq28_e1169_d_n14);
        let eq28_e1171_q_d_n15: f64 = (p.p87 * eq28_e1169_d_n15);
        let eq28_e1171_q_d_n16: f64 = (p.p87 * eq28_e1169_d_n16);
        let eq28_e1171_q_d_n17: f64 = (p.p87 * eq28_e1169_d_n17);
        let eq28_e1171_q_d_b0: f64 = (p.p87 * eq28_e1169_d_b0);
        let eq28_e1171_q_d_b1: f64 = (p.p87 * eq28_e1169_d_b1);
        let eq28_e1171_q_d_b2: f64 = (p.p87 * eq28_e1169_d_b2);
        let eq28_e1171_q_d_b3: f64 = (p.p87 * eq28_e1169_d_b3);
        let eq28_e1171_q_d_b4: f64 = (p.p87 * eq28_e1169_d_b4);
        let eq28_e1171_q_d_b5: f64 = (p.p87 * eq28_e1169_d_b5);
        let eq28_e1171_q_d_b6: f64 = (p.p87 * eq28_e1169_d_b6);
        let eq28_e1171_q_d_b7: f64 = (p.p87 * eq28_e1169_d_b7);
        let eq28_e1171_q_d_b8: f64 = (p.p87 * eq28_e1169_d_b8);
        let eq28_e1171_q_d_b9: f64 = (p.p87 * eq28_e1169_d_b9);
        let eq28_e1171_q_d_b10: f64 = (p.p87 * eq28_e1169_d_b10);
        let eq28_e1171_q_d_b11: f64 = (p.p87 * eq28_e1169_d_b11);
        let eq28_reactive_node_derivatives: [f64; 18] = [eq28_e1171_q_d_n0, eq28_e1171_q_d_n1, eq28_e1171_q_d_n2, eq28_e1171_q_d_n3, eq28_e1171_q_d_n4, eq28_e1171_q_d_n5, eq28_e1171_q_d_n6, eq28_e1171_q_d_n7, eq28_e1171_q_d_n8, eq28_e1171_q_d_n9, eq28_e1171_q_d_n10, eq28_e1171_q_d_n11, eq28_e1171_q_d_n12, eq28_e1171_q_d_n13, eq28_e1171_q_d_n14, eq28_e1171_q_d_n15, eq28_e1171_q_d_n16, eq28_e1171_q_d_n17];
        let eq28_reactive_branch_derivatives: [f64; 12] = [eq28_e1171_q_d_b0, eq28_e1171_q_d_b1, eq28_e1171_q_d_b2, eq28_e1171_q_d_b3, eq28_e1171_q_d_b4, eq28_e1171_q_d_b5, eq28_e1171_q_d_b6, eq28_e1171_q_d_b7, eq28_e1171_q_d_b8, eq28_e1171_q_d_b9, eq28_e1171_q_d_b10, eq28_e1171_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e1176: f64 = (s.v[753] + s.v[751]);
        let eq29_e1176_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq29_e1176_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq29_e1176_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq29_e1176_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq29_e1176_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq29_e1176_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq29_e1176_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq29_e1176_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq29_e1176_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq29_e1176_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq29_e1176_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq29_e1176_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq29_e1176_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq29_e1176_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq29_e1176_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq29_e1176_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq29_e1176_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq29_e1176_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq29_e1176_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq29_e1176_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq29_e1176_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq29_e1176_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq29_e1176_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq29_e1176_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq29_e1176_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq29_e1176_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq29_e1176_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq29_e1176_d_b9: f64 = (s.db[753][9] + s.db[751][9]);
        let eq29_e1176_d_b10: f64 = (s.db[753][10] + s.db[751][10]);
        let eq29_e1176_d_b11: f64 = (s.db[753][11] + s.db[751][11]);
        let eq29_e1178: f64 = (eq29_e1176 + s.v[752]);
        let eq29_e1178_d_n0: f64 = (eq29_e1176_d_n0 + s.dn[752][0]);
        let eq29_e1178_d_n1: f64 = (eq29_e1176_d_n1 + s.dn[752][1]);
        let eq29_e1178_d_n2: f64 = (eq29_e1176_d_n2 + s.dn[752][2]);
        let eq29_e1178_d_n3: f64 = (eq29_e1176_d_n3 + s.dn[752][3]);
        let eq29_e1178_d_n4: f64 = (eq29_e1176_d_n4 + s.dn[752][4]);
        let eq29_e1178_d_n5: f64 = (eq29_e1176_d_n5 + s.dn[752][5]);
        let eq29_e1178_d_n6: f64 = (eq29_e1176_d_n6 + s.dn[752][6]);
        let eq29_e1178_d_n7: f64 = (eq29_e1176_d_n7 + s.dn[752][7]);
        let eq29_e1178_d_n8: f64 = (eq29_e1176_d_n8 + s.dn[752][8]);
        let eq29_e1178_d_n9: f64 = (eq29_e1176_d_n9 + s.dn[752][9]);
        let eq29_e1178_d_n10: f64 = (eq29_e1176_d_n10 + s.dn[752][10]);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + s.dn[752][11]);
        let eq29_e1178_d_n12: f64 = (eq29_e1176_d_n12 + s.dn[752][12]);
        let eq29_e1178_d_n13: f64 = (eq29_e1176_d_n13 + s.dn[752][13]);
        let eq29_e1178_d_n14: f64 = (eq29_e1176_d_n14 + s.dn[752][14]);
        let eq29_e1178_d_n15: f64 = (eq29_e1176_d_n15 + s.dn[752][15]);
        let eq29_e1178_d_n16: f64 = (eq29_e1176_d_n16 + s.dn[752][16]);
        let eq29_e1178_d_n17: f64 = (eq29_e1176_d_n17 + s.dn[752][17]);
        let eq29_e1178_d_b0: f64 = (eq29_e1176_d_b0 + s.db[752][0]);
        let eq29_e1178_d_b1: f64 = (eq29_e1176_d_b1 + s.db[752][1]);
        let eq29_e1178_d_b2: f64 = (eq29_e1176_d_b2 + s.db[752][2]);
        let eq29_e1178_d_b3: f64 = (eq29_e1176_d_b3 + s.db[752][3]);
        let eq29_e1178_d_b4: f64 = (eq29_e1176_d_b4 + s.db[752][4]);
        let eq29_e1178_d_b5: f64 = (eq29_e1176_d_b5 + s.db[752][5]);
        let eq29_e1178_d_b6: f64 = (eq29_e1176_d_b6 + s.db[752][6]);
        let eq29_e1178_d_b7: f64 = (eq29_e1176_d_b7 + s.db[752][7]);
        let eq29_e1178_d_b8: f64 = (eq29_e1176_d_b8 + s.db[752][8]);
        let eq29_e1178_d_b9: f64 = (eq29_e1176_d_b9 + s.db[752][9]);
        let eq29_e1178_d_b10: f64 = (eq29_e1176_d_b10 + s.db[752][10]);
        let eq29_e1178_d_b11: f64 = (eq29_e1176_d_b11 + s.db[752][11]);
        let eq29_e1179: f64 = (s.v[20] - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (s.dn[20][0] - eq29_e1178_d_n0);
        let eq29_e1179_d_n1: f64 = (s.dn[20][1] - eq29_e1178_d_n1);
        let eq29_e1179_d_n2: f64 = (s.dn[20][2] - eq29_e1178_d_n2);
        let eq29_e1179_d_n3: f64 = (s.dn[20][3] - eq29_e1178_d_n3);
        let eq29_e1179_d_n4: f64 = (s.dn[20][4] - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (s.dn[20][5] - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (s.dn[20][6] - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (s.dn[20][7] - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (s.dn[20][8] - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (s.dn[20][9] - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (s.dn[20][10] - eq29_e1178_d_n10);
        let eq29_e1179_d_n11: f64 = (s.dn[20][11] - eq29_e1178_d_n11);
        let eq29_e1179_d_n12: f64 = (s.dn[20][12] - eq29_e1178_d_n12);
        let eq29_e1179_d_n13: f64 = (s.dn[20][13] - eq29_e1178_d_n13);
        let eq29_e1179_d_n14: f64 = (s.dn[20][14] - eq29_e1178_d_n14);
        let eq29_e1179_d_n15: f64 = (s.dn[20][15] - eq29_e1178_d_n15);
        let eq29_e1179_d_n16: f64 = (s.dn[20][16] - eq29_e1178_d_n16);
        let eq29_e1179_d_n17: f64 = (s.dn[20][17] - eq29_e1178_d_n17);
        let eq29_e1179_d_b0: f64 = (s.db[20][0] - eq29_e1178_d_b0);
        let eq29_e1179_d_b1: f64 = (s.db[20][1] - eq29_e1178_d_b1);
        let eq29_e1179_d_b2: f64 = (s.db[20][2] - eq29_e1178_d_b2);
        let eq29_e1179_d_b3: f64 = (s.db[20][3] - eq29_e1178_d_b3);
        let eq29_e1179_d_b4: f64 = (s.db[20][4] - eq29_e1178_d_b4);
        let eq29_e1179_d_b5: f64 = (s.db[20][5] - eq29_e1178_d_b5);
        let eq29_e1179_d_b6: f64 = (s.db[20][6] - eq29_e1178_d_b6);
        let eq29_e1179_d_b7: f64 = (s.db[20][7] - eq29_e1178_d_b7);
        let eq29_e1179_d_b8: f64 = (s.db[20][8] - eq29_e1178_d_b8);
        let eq29_e1179_d_b9: f64 = (s.db[20][9] - eq29_e1178_d_b9);
        let eq29_e1179_d_b10: f64 = (s.db[20][10] - eq29_e1178_d_b10);
        let eq29_e1179_d_b11: f64 = (s.db[20][11] - eq29_e1178_d_b11);
        let eq29_e1180_q: f64 = eq29_e1179;
        let eq29_e1181: f64 = (p.p87 * eq29_e1179);
        let eq29_e1181_d_n0: f64 = (p.p87 * eq29_e1179_d_n0);
        let eq29_e1181_d_n1: f64 = (p.p87 * eq29_e1179_d_n1);
        let eq29_e1181_d_n2: f64 = (p.p87 * eq29_e1179_d_n2);
        let eq29_e1181_d_n3: f64 = (p.p87 * eq29_e1179_d_n3);
        let eq29_e1181_d_n4: f64 = (p.p87 * eq29_e1179_d_n4);
        let eq29_e1181_d_n5: f64 = (p.p87 * eq29_e1179_d_n5);
        let eq29_e1181_d_n6: f64 = (p.p87 * eq29_e1179_d_n6);
        let eq29_e1181_d_n7: f64 = (p.p87 * eq29_e1179_d_n7);
        let eq29_e1181_d_n8: f64 = (p.p87 * eq29_e1179_d_n8);
        let eq29_e1181_d_n9: f64 = (p.p87 * eq29_e1179_d_n9);
        let eq29_e1181_d_n10: f64 = (p.p87 * eq29_e1179_d_n10);
        let eq29_e1181_d_n11: f64 = (p.p87 * eq29_e1179_d_n11);
        let eq29_e1181_d_n12: f64 = (p.p87 * eq29_e1179_d_n12);
        let eq29_e1181_d_n13: f64 = (p.p87 * eq29_e1179_d_n13);
        let eq29_e1181_d_n14: f64 = (p.p87 * eq29_e1179_d_n14);
        let eq29_e1181_d_n15: f64 = (p.p87 * eq29_e1179_d_n15);
        let eq29_e1181_d_n16: f64 = (p.p87 * eq29_e1179_d_n16);
        let eq29_e1181_d_n17: f64 = (p.p87 * eq29_e1179_d_n17);
        let eq29_e1181_d_b0: f64 = (p.p87 * eq29_e1179_d_b0);
        let eq29_e1181_d_b1: f64 = (p.p87 * eq29_e1179_d_b1);
        let eq29_e1181_d_b2: f64 = (p.p87 * eq29_e1179_d_b2);
        let eq29_e1181_d_b3: f64 = (p.p87 * eq29_e1179_d_b3);
        let eq29_e1181_d_b4: f64 = (p.p87 * eq29_e1179_d_b4);
        let eq29_e1181_d_b5: f64 = (p.p87 * eq29_e1179_d_b5);
        let eq29_e1181_d_b6: f64 = (p.p87 * eq29_e1179_d_b6);
        let eq29_e1181_d_b7: f64 = (p.p87 * eq29_e1179_d_b7);
        let eq29_e1181_d_b8: f64 = (p.p87 * eq29_e1179_d_b8);
        let eq29_e1181_d_b9: f64 = (p.p87 * eq29_e1179_d_b9);
        let eq29_e1181_d_b10: f64 = (p.p87 * eq29_e1179_d_b10);
        let eq29_e1181_d_b11: f64 = (p.p87 * eq29_e1179_d_b11);
        let eq29_e1181_q: f64 = (p.p87 * eq29_e1180_q);
        let eq29_e1181_q_d_n0: f64 = (p.p87 * eq29_e1179_d_n0);
        let eq29_e1181_q_d_n1: f64 = (p.p87 * eq29_e1179_d_n1);
        let eq29_e1181_q_d_n2: f64 = (p.p87 * eq29_e1179_d_n2);
        let eq29_e1181_q_d_n3: f64 = (p.p87 * eq29_e1179_d_n3);
        let eq29_e1181_q_d_n4: f64 = (p.p87 * eq29_e1179_d_n4);
        let eq29_e1181_q_d_n5: f64 = (p.p87 * eq29_e1179_d_n5);
        let eq29_e1181_q_d_n6: f64 = (p.p87 * eq29_e1179_d_n6);
        let eq29_e1181_q_d_n7: f64 = (p.p87 * eq29_e1179_d_n7);
        let eq29_e1181_q_d_n8: f64 = (p.p87 * eq29_e1179_d_n8);
        let eq29_e1181_q_d_n9: f64 = (p.p87 * eq29_e1179_d_n9);
        let eq29_e1181_q_d_n10: f64 = (p.p87 * eq29_e1179_d_n10);
        let eq29_e1181_q_d_n11: f64 = (p.p87 * eq29_e1179_d_n11);
        let eq29_e1181_q_d_n12: f64 = (p.p87 * eq29_e1179_d_n12);
        let eq29_e1181_q_d_n13: f64 = (p.p87 * eq29_e1179_d_n13);
        let eq29_e1181_q_d_n14: f64 = (p.p87 * eq29_e1179_d_n14);
        let eq29_e1181_q_d_n15: f64 = (p.p87 * eq29_e1179_d_n15);
        let eq29_e1181_q_d_n16: f64 = (p.p87 * eq29_e1179_d_n16);
        let eq29_e1181_q_d_n17: f64 = (p.p87 * eq29_e1179_d_n17);
        let eq29_e1181_q_d_b0: f64 = (p.p87 * eq29_e1179_d_b0);
        let eq29_e1181_q_d_b1: f64 = (p.p87 * eq29_e1179_d_b1);
        let eq29_e1181_q_d_b2: f64 = (p.p87 * eq29_e1179_d_b2);
        let eq29_e1181_q_d_b3: f64 = (p.p87 * eq29_e1179_d_b3);
        let eq29_e1181_q_d_b4: f64 = (p.p87 * eq29_e1179_d_b4);
        let eq29_e1181_q_d_b5: f64 = (p.p87 * eq29_e1179_d_b5);
        let eq29_e1181_q_d_b6: f64 = (p.p87 * eq29_e1179_d_b6);
        let eq29_e1181_q_d_b7: f64 = (p.p87 * eq29_e1179_d_b7);
        let eq29_e1181_q_d_b8: f64 = (p.p87 * eq29_e1179_d_b8);
        let eq29_e1181_q_d_b9: f64 = (p.p87 * eq29_e1179_d_b9);
        let eq29_e1181_q_d_b10: f64 = (p.p87 * eq29_e1179_d_b10);
        let eq29_e1181_q_d_b11: f64 = (p.p87 * eq29_e1179_d_b11);
        let eq29_reactive_node_derivatives: [f64; 18] = [eq29_e1181_q_d_n0, eq29_e1181_q_d_n1, eq29_e1181_q_d_n2, eq29_e1181_q_d_n3, eq29_e1181_q_d_n4, eq29_e1181_q_d_n5, eq29_e1181_q_d_n6, eq29_e1181_q_d_n7, eq29_e1181_q_d_n8, eq29_e1181_q_d_n9, eq29_e1181_q_d_n10, eq29_e1181_q_d_n11, eq29_e1181_q_d_n12, eq29_e1181_q_d_n13, eq29_e1181_q_d_n14, eq29_e1181_q_d_n15, eq29_e1181_q_d_n16, eq29_e1181_q_d_n17];
        let eq29_reactive_branch_derivatives: [f64; 12] = [eq29_e1181_q_d_b0, eq29_e1181_q_d_b1, eq29_e1181_q_d_b2, eq29_e1181_q_d_b3, eq29_e1181_q_d_b4, eq29_e1181_q_d_b5, eq29_e1181_q_d_b6, eq29_e1181_q_d_b7, eq29_e1181_q_d_b8, eq29_e1181_q_d_b9, eq29_e1181_q_d_b10, eq29_e1181_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e1184_q: f64 = s.v[743];
        let eq30_e1185: f64 = (p.p87 * s.v[743]);
        let eq30_e1185_d_n0: f64 = (p.p87 * s.dn[743][0]);
        let eq30_e1185_d_n1: f64 = (p.p87 * s.dn[743][1]);
        let eq30_e1185_d_n2: f64 = (p.p87 * s.dn[743][2]);
        let eq30_e1185_d_n3: f64 = (p.p87 * s.dn[743][3]);
        let eq30_e1185_d_n4: f64 = (p.p87 * s.dn[743][4]);
        let eq30_e1185_d_n5: f64 = (p.p87 * s.dn[743][5]);
        let eq30_e1185_d_n6: f64 = (p.p87 * s.dn[743][6]);
        let eq30_e1185_d_n7: f64 = (p.p87 * s.dn[743][7]);
        let eq30_e1185_d_n8: f64 = (p.p87 * s.dn[743][8]);
        let eq30_e1185_d_n9: f64 = (p.p87 * s.dn[743][9]);
        let eq30_e1185_d_n10: f64 = (p.p87 * s.dn[743][10]);
        let eq30_e1185_d_n11: f64 = (p.p87 * s.dn[743][11]);
        let eq30_e1185_d_n12: f64 = (p.p87 * s.dn[743][12]);
        let eq30_e1185_d_n13: f64 = (p.p87 * s.dn[743][13]);
        let eq30_e1185_d_n14: f64 = (p.p87 * s.dn[743][14]);
        let eq30_e1185_d_n15: f64 = (p.p87 * s.dn[743][15]);
        let eq30_e1185_d_n16: f64 = (p.p87 * s.dn[743][16]);
        let eq30_e1185_d_n17: f64 = (p.p87 * s.dn[743][17]);
        let eq30_e1185_d_b0: f64 = (p.p87 * s.db[743][0]);
        let eq30_e1185_d_b1: f64 = (p.p87 * s.db[743][1]);
        let eq30_e1185_d_b2: f64 = (p.p87 * s.db[743][2]);
        let eq30_e1185_d_b3: f64 = (p.p87 * s.db[743][3]);
        let eq30_e1185_d_b4: f64 = (p.p87 * s.db[743][4]);
        let eq30_e1185_d_b5: f64 = (p.p87 * s.db[743][5]);
        let eq30_e1185_d_b6: f64 = (p.p87 * s.db[743][6]);
        let eq30_e1185_d_b7: f64 = (p.p87 * s.db[743][7]);
        let eq30_e1185_d_b8: f64 = (p.p87 * s.db[743][8]);
        let eq30_e1185_d_b9: f64 = (p.p87 * s.db[743][9]);
        let eq30_e1185_d_b10: f64 = (p.p87 * s.db[743][10]);
        let eq30_e1185_d_b11: f64 = (p.p87 * s.db[743][11]);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_e1185_q_d_n0: f64 = (p.p87 * s.dn[743][0]);
        let eq30_e1185_q_d_n1: f64 = (p.p87 * s.dn[743][1]);
        let eq30_e1185_q_d_n2: f64 = (p.p87 * s.dn[743][2]);
        let eq30_e1185_q_d_n3: f64 = (p.p87 * s.dn[743][3]);
        let eq30_e1185_q_d_n4: f64 = (p.p87 * s.dn[743][4]);
        let eq30_e1185_q_d_n5: f64 = (p.p87 * s.dn[743][5]);
        let eq30_e1185_q_d_n6: f64 = (p.p87 * s.dn[743][6]);
        let eq30_e1185_q_d_n7: f64 = (p.p87 * s.dn[743][7]);
        let eq30_e1185_q_d_n8: f64 = (p.p87 * s.dn[743][8]);
        let eq30_e1185_q_d_n9: f64 = (p.p87 * s.dn[743][9]);
        let eq30_e1185_q_d_n10: f64 = (p.p87 * s.dn[743][10]);
        let eq30_e1185_q_d_n11: f64 = (p.p87 * s.dn[743][11]);
        let eq30_e1185_q_d_n12: f64 = (p.p87 * s.dn[743][12]);
        let eq30_e1185_q_d_n13: f64 = (p.p87 * s.dn[743][13]);
        let eq30_e1185_q_d_n14: f64 = (p.p87 * s.dn[743][14]);
        let eq30_e1185_q_d_n15: f64 = (p.p87 * s.dn[743][15]);
        let eq30_e1185_q_d_n16: f64 = (p.p87 * s.dn[743][16]);
        let eq30_e1185_q_d_n17: f64 = (p.p87 * s.dn[743][17]);
        let eq30_e1185_q_d_b0: f64 = (p.p87 * s.db[743][0]);
        let eq30_e1185_q_d_b1: f64 = (p.p87 * s.db[743][1]);
        let eq30_e1185_q_d_b2: f64 = (p.p87 * s.db[743][2]);
        let eq30_e1185_q_d_b3: f64 = (p.p87 * s.db[743][3]);
        let eq30_e1185_q_d_b4: f64 = (p.p87 * s.db[743][4]);
        let eq30_e1185_q_d_b5: f64 = (p.p87 * s.db[743][5]);
        let eq30_e1185_q_d_b6: f64 = (p.p87 * s.db[743][6]);
        let eq30_e1185_q_d_b7: f64 = (p.p87 * s.db[743][7]);
        let eq30_e1185_q_d_b8: f64 = (p.p87 * s.db[743][8]);
        let eq30_e1185_q_d_b9: f64 = (p.p87 * s.db[743][9]);
        let eq30_e1185_q_d_b10: f64 = (p.p87 * s.db[743][10]);
        let eq30_e1185_q_d_b11: f64 = (p.p87 * s.db[743][11]);
        let eq30_reactive_node_derivatives: [f64; 18] = [eq30_e1185_q_d_n0, eq30_e1185_q_d_n1, eq30_e1185_q_d_n2, eq30_e1185_q_d_n3, eq30_e1185_q_d_n4, eq30_e1185_q_d_n5, eq30_e1185_q_d_n6, eq30_e1185_q_d_n7, eq30_e1185_q_d_n8, eq30_e1185_q_d_n9, eq30_e1185_q_d_n10, eq30_e1185_q_d_n11, eq30_e1185_q_d_n12, eq30_e1185_q_d_n13, eq30_e1185_q_d_n14, eq30_e1185_q_d_n15, eq30_e1185_q_d_n16, eq30_e1185_q_d_n17];
        let eq30_reactive_branch_derivatives: [f64; 12] = [eq30_e1185_q_d_b0, eq30_e1185_q_d_b1, eq30_e1185_q_d_b2, eq30_e1185_q_d_b3, eq30_e1185_q_d_b4, eq30_e1185_q_d_b5, eq30_e1185_q_d_b6, eq30_e1185_q_d_b7, eq30_e1185_q_d_b8, eq30_e1185_q_d_b9, eq30_e1185_q_d_b10, eq30_e1185_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq31_e1188_q: f64 = s.v[742];
        let eq31_e1189: f64 = (p.p87 * s.v[742]);
        let eq31_e1189_d_n0: f64 = (p.p87 * s.dn[742][0]);
        let eq31_e1189_d_n1: f64 = (p.p87 * s.dn[742][1]);
        let eq31_e1189_d_n2: f64 = (p.p87 * s.dn[742][2]);
        let eq31_e1189_d_n3: f64 = (p.p87 * s.dn[742][3]);
        let eq31_e1189_d_n4: f64 = (p.p87 * s.dn[742][4]);
        let eq31_e1189_d_n5: f64 = (p.p87 * s.dn[742][5]);
        let eq31_e1189_d_n6: f64 = (p.p87 * s.dn[742][6]);
        let eq31_e1189_d_n7: f64 = (p.p87 * s.dn[742][7]);
        let eq31_e1189_d_n8: f64 = (p.p87 * s.dn[742][8]);
        let eq31_e1189_d_n9: f64 = (p.p87 * s.dn[742][9]);
        let eq31_e1189_d_n10: f64 = (p.p87 * s.dn[742][10]);
        let eq31_e1189_d_n11: f64 = (p.p87 * s.dn[742][11]);
        let eq31_e1189_d_n12: f64 = (p.p87 * s.dn[742][12]);
        let eq31_e1189_d_n13: f64 = (p.p87 * s.dn[742][13]);
        let eq31_e1189_d_n14: f64 = (p.p87 * s.dn[742][14]);
        let eq31_e1189_d_n15: f64 = (p.p87 * s.dn[742][15]);
        let eq31_e1189_d_n16: f64 = (p.p87 * s.dn[742][16]);
        let eq31_e1189_d_n17: f64 = (p.p87 * s.dn[742][17]);
        let eq31_e1189_d_b0: f64 = (p.p87 * s.db[742][0]);
        let eq31_e1189_d_b1: f64 = (p.p87 * s.db[742][1]);
        let eq31_e1189_d_b2: f64 = (p.p87 * s.db[742][2]);
        let eq31_e1189_d_b3: f64 = (p.p87 * s.db[742][3]);
        let eq31_e1189_d_b4: f64 = (p.p87 * s.db[742][4]);
        let eq31_e1189_d_b5: f64 = (p.p87 * s.db[742][5]);
        let eq31_e1189_d_b6: f64 = (p.p87 * s.db[742][6]);
        let eq31_e1189_d_b7: f64 = (p.p87 * s.db[742][7]);
        let eq31_e1189_d_b8: f64 = (p.p87 * s.db[742][8]);
        let eq31_e1189_d_b9: f64 = (p.p87 * s.db[742][9]);
        let eq31_e1189_d_b10: f64 = (p.p87 * s.db[742][10]);
        let eq31_e1189_d_b11: f64 = (p.p87 * s.db[742][11]);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_e1189_q_d_n0: f64 = (p.p87 * s.dn[742][0]);
        let eq31_e1189_q_d_n1: f64 = (p.p87 * s.dn[742][1]);
        let eq31_e1189_q_d_n2: f64 = (p.p87 * s.dn[742][2]);
        let eq31_e1189_q_d_n3: f64 = (p.p87 * s.dn[742][3]);
        let eq31_e1189_q_d_n4: f64 = (p.p87 * s.dn[742][4]);
        let eq31_e1189_q_d_n5: f64 = (p.p87 * s.dn[742][5]);
        let eq31_e1189_q_d_n6: f64 = (p.p87 * s.dn[742][6]);
        let eq31_e1189_q_d_n7: f64 = (p.p87 * s.dn[742][7]);
        let eq31_e1189_q_d_n8: f64 = (p.p87 * s.dn[742][8]);
        let eq31_e1189_q_d_n9: f64 = (p.p87 * s.dn[742][9]);
        let eq31_e1189_q_d_n10: f64 = (p.p87 * s.dn[742][10]);
        let eq31_e1189_q_d_n11: f64 = (p.p87 * s.dn[742][11]);
        let eq31_e1189_q_d_n12: f64 = (p.p87 * s.dn[742][12]);
        let eq31_e1189_q_d_n13: f64 = (p.p87 * s.dn[742][13]);
        let eq31_e1189_q_d_n14: f64 = (p.p87 * s.dn[742][14]);
        let eq31_e1189_q_d_n15: f64 = (p.p87 * s.dn[742][15]);
        let eq31_e1189_q_d_n16: f64 = (p.p87 * s.dn[742][16]);
        let eq31_e1189_q_d_n17: f64 = (p.p87 * s.dn[742][17]);
        let eq31_e1189_q_d_b0: f64 = (p.p87 * s.db[742][0]);
        let eq31_e1189_q_d_b1: f64 = (p.p87 * s.db[742][1]);
        let eq31_e1189_q_d_b2: f64 = (p.p87 * s.db[742][2]);
        let eq31_e1189_q_d_b3: f64 = (p.p87 * s.db[742][3]);
        let eq31_e1189_q_d_b4: f64 = (p.p87 * s.db[742][4]);
        let eq31_e1189_q_d_b5: f64 = (p.p87 * s.db[742][5]);
        let eq31_e1189_q_d_b6: f64 = (p.p87 * s.db[742][6]);
        let eq31_e1189_q_d_b7: f64 = (p.p87 * s.db[742][7]);
        let eq31_e1189_q_d_b8: f64 = (p.p87 * s.db[742][8]);
        let eq31_e1189_q_d_b9: f64 = (p.p87 * s.db[742][9]);
        let eq31_e1189_q_d_b10: f64 = (p.p87 * s.db[742][10]);
        let eq31_e1189_q_d_b11: f64 = (p.p87 * s.db[742][11]);
        let eq31_reactive_node_derivatives: [f64; 18] = [eq31_e1189_q_d_n0, eq31_e1189_q_d_n1, eq31_e1189_q_d_n2, eq31_e1189_q_d_n3, eq31_e1189_q_d_n4, eq31_e1189_q_d_n5, eq31_e1189_q_d_n6, eq31_e1189_q_d_n7, eq31_e1189_q_d_n8, eq31_e1189_q_d_n9, eq31_e1189_q_d_n10, eq31_e1189_q_d_n11, eq31_e1189_q_d_n12, eq31_e1189_q_d_n13, eq31_e1189_q_d_n14, eq31_e1189_q_d_n15, eq31_e1189_q_d_n16, eq31_e1189_q_d_n17];
        let eq31_reactive_branch_derivatives: [f64; 12] = [eq31_e1189_q_d_b0, eq31_e1189_q_d_b1, eq31_e1189_q_d_b2, eq31_e1189_q_d_b3, eq31_e1189_q_d_b4, eq31_e1189_q_d_b5, eq31_e1189_q_d_b6, eq31_e1189_q_d_b7, eq31_e1189_q_d_b8, eq31_e1189_q_d_b9, eq31_e1189_q_d_b10, eq31_e1189_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192_q: f64 = s.v[744];
        let eq32_e1193: f64 = (p.p87 * s.v[744]);
        let eq32_e1193_d_n0: f64 = (p.p87 * s.dn[744][0]);
        let eq32_e1193_d_n1: f64 = (p.p87 * s.dn[744][1]);
        let eq32_e1193_d_n2: f64 = (p.p87 * s.dn[744][2]);
        let eq32_e1193_d_n3: f64 = (p.p87 * s.dn[744][3]);
        let eq32_e1193_d_n4: f64 = (p.p87 * s.dn[744][4]);
        let eq32_e1193_d_n5: f64 = (p.p87 * s.dn[744][5]);
        let eq32_e1193_d_n6: f64 = (p.p87 * s.dn[744][6]);
        let eq32_e1193_d_n7: f64 = (p.p87 * s.dn[744][7]);
        let eq32_e1193_d_n8: f64 = (p.p87 * s.dn[744][8]);
        let eq32_e1193_d_n9: f64 = (p.p87 * s.dn[744][9]);
        let eq32_e1193_d_n10: f64 = (p.p87 * s.dn[744][10]);
        let eq32_e1193_d_n11: f64 = (p.p87 * s.dn[744][11]);
        let eq32_e1193_d_n12: f64 = (p.p87 * s.dn[744][12]);
        let eq32_e1193_d_n13: f64 = (p.p87 * s.dn[744][13]);
        let eq32_e1193_d_n14: f64 = (p.p87 * s.dn[744][14]);
        let eq32_e1193_d_n15: f64 = (p.p87 * s.dn[744][15]);
        let eq32_e1193_d_n16: f64 = (p.p87 * s.dn[744][16]);
        let eq32_e1193_d_n17: f64 = (p.p87 * s.dn[744][17]);
        let eq32_e1193_d_b0: f64 = (p.p87 * s.db[744][0]);
        let eq32_e1193_d_b1: f64 = (p.p87 * s.db[744][1]);
        let eq32_e1193_d_b2: f64 = (p.p87 * s.db[744][2]);
        let eq32_e1193_d_b3: f64 = (p.p87 * s.db[744][3]);
        let eq32_e1193_d_b4: f64 = (p.p87 * s.db[744][4]);
        let eq32_e1193_d_b5: f64 = (p.p87 * s.db[744][5]);
        let eq32_e1193_d_b6: f64 = (p.p87 * s.db[744][6]);
        let eq32_e1193_d_b7: f64 = (p.p87 * s.db[744][7]);
        let eq32_e1193_d_b8: f64 = (p.p87 * s.db[744][8]);
        let eq32_e1193_d_b9: f64 = (p.p87 * s.db[744][9]);
        let eq32_e1193_d_b10: f64 = (p.p87 * s.db[744][10]);
        let eq32_e1193_d_b11: f64 = (p.p87 * s.db[744][11]);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_e1193_q_d_n0: f64 = (p.p87 * s.dn[744][0]);
        let eq32_e1193_q_d_n1: f64 = (p.p87 * s.dn[744][1]);
        let eq32_e1193_q_d_n2: f64 = (p.p87 * s.dn[744][2]);
        let eq32_e1193_q_d_n3: f64 = (p.p87 * s.dn[744][3]);
        let eq32_e1193_q_d_n4: f64 = (p.p87 * s.dn[744][4]);
        let eq32_e1193_q_d_n5: f64 = (p.p87 * s.dn[744][5]);
        let eq32_e1193_q_d_n6: f64 = (p.p87 * s.dn[744][6]);
        let eq32_e1193_q_d_n7: f64 = (p.p87 * s.dn[744][7]);
        let eq32_e1193_q_d_n8: f64 = (p.p87 * s.dn[744][8]);
        let eq32_e1193_q_d_n9: f64 = (p.p87 * s.dn[744][9]);
        let eq32_e1193_q_d_n10: f64 = (p.p87 * s.dn[744][10]);
        let eq32_e1193_q_d_n11: f64 = (p.p87 * s.dn[744][11]);
        let eq32_e1193_q_d_n12: f64 = (p.p87 * s.dn[744][12]);
        let eq32_e1193_q_d_n13: f64 = (p.p87 * s.dn[744][13]);
        let eq32_e1193_q_d_n14: f64 = (p.p87 * s.dn[744][14]);
        let eq32_e1193_q_d_n15: f64 = (p.p87 * s.dn[744][15]);
        let eq32_e1193_q_d_n16: f64 = (p.p87 * s.dn[744][16]);
        let eq32_e1193_q_d_n17: f64 = (p.p87 * s.dn[744][17]);
        let eq32_e1193_q_d_b0: f64 = (p.p87 * s.db[744][0]);
        let eq32_e1193_q_d_b1: f64 = (p.p87 * s.db[744][1]);
        let eq32_e1193_q_d_b2: f64 = (p.p87 * s.db[744][2]);
        let eq32_e1193_q_d_b3: f64 = (p.p87 * s.db[744][3]);
        let eq32_e1193_q_d_b4: f64 = (p.p87 * s.db[744][4]);
        let eq32_e1193_q_d_b5: f64 = (p.p87 * s.db[744][5]);
        let eq32_e1193_q_d_b6: f64 = (p.p87 * s.db[744][6]);
        let eq32_e1193_q_d_b7: f64 = (p.p87 * s.db[744][7]);
        let eq32_e1193_q_d_b8: f64 = (p.p87 * s.db[744][8]);
        let eq32_e1193_q_d_b9: f64 = (p.p87 * s.db[744][9]);
        let eq32_e1193_q_d_b10: f64 = (p.p87 * s.db[744][10]);
        let eq32_e1193_q_d_b11: f64 = (p.p87 * s.db[744][11]);
        let eq32_reactive_node_derivatives: [f64; 18] = [eq32_e1193_q_d_n0, eq32_e1193_q_d_n1, eq32_e1193_q_d_n2, eq32_e1193_q_d_n3, eq32_e1193_q_d_n4, eq32_e1193_q_d_n5, eq32_e1193_q_d_n6, eq32_e1193_q_d_n7, eq32_e1193_q_d_n8, eq32_e1193_q_d_n9, eq32_e1193_q_d_n10, eq32_e1193_q_d_n11, eq32_e1193_q_d_n12, eq32_e1193_q_d_n13, eq32_e1193_q_d_n14, eq32_e1193_q_d_n15, eq32_e1193_q_d_n16, eq32_e1193_q_d_n17];
        let eq32_reactive_branch_derivatives: [f64; 12] = [eq32_e1193_q_d_b0, eq32_e1193_q_d_b1, eq32_e1193_q_d_b2, eq32_e1193_q_d_b3, eq32_e1193_q_d_b4, eq32_e1193_q_d_b5, eq32_e1193_q_d_b6, eq32_e1193_q_d_b7, eq32_e1193_q_d_b8, eq32_e1193_q_d_b9, eq32_e1193_q_d_b10, eq32_e1193_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197_q: f64 = s.v[299];
        let eq33_e1198: f64 = (eq33_e1195 * s.v[299]);
        let eq33_e1198_d_n0: f64 = (eq33_e1195 * s.dn[299][0]);
        let eq33_e1198_d_n1: f64 = (eq33_e1195 * s.dn[299][1]);
        let eq33_e1198_d_n2: f64 = (eq33_e1195 * s.dn[299][2]);
        let eq33_e1198_d_n3: f64 = (eq33_e1195 * s.dn[299][3]);
        let eq33_e1198_d_n4: f64 = (eq33_e1195 * s.dn[299][4]);
        let eq33_e1198_d_n5: f64 = (eq33_e1195 * s.dn[299][5]);
        let eq33_e1198_d_n6: f64 = (eq33_e1195 * s.dn[299][6]);
        let eq33_e1198_d_n7: f64 = (eq33_e1195 * s.dn[299][7]);
        let eq33_e1198_d_n8: f64 = (eq33_e1195 * s.dn[299][8]);
        let eq33_e1198_d_n9: f64 = (eq33_e1195 * s.dn[299][9]);
        let eq33_e1198_d_n10: f64 = (eq33_e1195 * s.dn[299][10]);
        let eq33_e1198_d_n11: f64 = (eq33_e1195 * s.dn[299][11]);
        let eq33_e1198_d_n12: f64 = (eq33_e1195 * s.dn[299][12]);
        let eq33_e1198_d_n13: f64 = (eq33_e1195 * s.dn[299][13]);
        let eq33_e1198_d_n14: f64 = (eq33_e1195 * s.dn[299][14]);
        let eq33_e1198_d_n15: f64 = (eq33_e1195 * s.dn[299][15]);
        let eq33_e1198_d_n16: f64 = (eq33_e1195 * s.dn[299][16]);
        let eq33_e1198_d_n17: f64 = (eq33_e1195 * s.dn[299][17]);
        let eq33_e1198_d_b0: f64 = (eq33_e1195 * s.db[299][0]);
        let eq33_e1198_d_b1: f64 = (eq33_e1195 * s.db[299][1]);
        let eq33_e1198_d_b2: f64 = (eq33_e1195 * s.db[299][2]);
        let eq33_e1198_d_b3: f64 = (eq33_e1195 * s.db[299][3]);
        let eq33_e1198_d_b4: f64 = (eq33_e1195 * s.db[299][4]);
        let eq33_e1198_d_b5: f64 = (eq33_e1195 * s.db[299][5]);
        let eq33_e1198_d_b6: f64 = (eq33_e1195 * s.db[299][6]);
        let eq33_e1198_d_b7: f64 = (eq33_e1195 * s.db[299][7]);
        let eq33_e1198_d_b8: f64 = (eq33_e1195 * s.db[299][8]);
        let eq33_e1198_d_b9: f64 = (eq33_e1195 * s.db[299][9]);
        let eq33_e1198_d_b10: f64 = (eq33_e1195 * s.db[299][10]);
        let eq33_e1198_d_b11: f64 = (eq33_e1195 * s.db[299][11]);
        let eq33_e1198_q: f64 = (eq33_e1195 * eq33_e1197_q);
        let eq33_e1198_q_d_n0: f64 = (eq33_e1195 * s.dn[299][0]);
        let eq33_e1198_q_d_n1: f64 = (eq33_e1195 * s.dn[299][1]);
        let eq33_e1198_q_d_n2: f64 = (eq33_e1195 * s.dn[299][2]);
        let eq33_e1198_q_d_n3: f64 = (eq33_e1195 * s.dn[299][3]);
        let eq33_e1198_q_d_n4: f64 = (eq33_e1195 * s.dn[299][4]);
        let eq33_e1198_q_d_n5: f64 = (eq33_e1195 * s.dn[299][5]);
        let eq33_e1198_q_d_n6: f64 = (eq33_e1195 * s.dn[299][6]);
        let eq33_e1198_q_d_n7: f64 = (eq33_e1195 * s.dn[299][7]);
        let eq33_e1198_q_d_n8: f64 = (eq33_e1195 * s.dn[299][8]);
        let eq33_e1198_q_d_n9: f64 = (eq33_e1195 * s.dn[299][9]);
        let eq33_e1198_q_d_n10: f64 = (eq33_e1195 * s.dn[299][10]);
        let eq33_e1198_q_d_n11: f64 = (eq33_e1195 * s.dn[299][11]);
        let eq33_e1198_q_d_n12: f64 = (eq33_e1195 * s.dn[299][12]);
        let eq33_e1198_q_d_n13: f64 = (eq33_e1195 * s.dn[299][13]);
        let eq33_e1198_q_d_n14: f64 = (eq33_e1195 * s.dn[299][14]);
        let eq33_e1198_q_d_n15: f64 = (eq33_e1195 * s.dn[299][15]);
        let eq33_e1198_q_d_n16: f64 = (eq33_e1195 * s.dn[299][16]);
        let eq33_e1198_q_d_n17: f64 = (eq33_e1195 * s.dn[299][17]);
        let eq33_e1198_q_d_b0: f64 = (eq33_e1195 * s.db[299][0]);
        let eq33_e1198_q_d_b1: f64 = (eq33_e1195 * s.db[299][1]);
        let eq33_e1198_q_d_b2: f64 = (eq33_e1195 * s.db[299][2]);
        let eq33_e1198_q_d_b3: f64 = (eq33_e1195 * s.db[299][3]);
        let eq33_e1198_q_d_b4: f64 = (eq33_e1195 * s.db[299][4]);
        let eq33_e1198_q_d_b5: f64 = (eq33_e1195 * s.db[299][5]);
        let eq33_e1198_q_d_b6: f64 = (eq33_e1195 * s.db[299][6]);
        let eq33_e1198_q_d_b7: f64 = (eq33_e1195 * s.db[299][7]);
        let eq33_e1198_q_d_b8: f64 = (eq33_e1195 * s.db[299][8]);
        let eq33_e1198_q_d_b9: f64 = (eq33_e1195 * s.db[299][9]);
        let eq33_e1198_q_d_b10: f64 = (eq33_e1195 * s.db[299][10]);
        let eq33_e1198_q_d_b11: f64 = (eq33_e1195 * s.db[299][11]);
        let eq33_reactive_node_derivatives: [f64; 18] = [eq33_e1198_q_d_n0, eq33_e1198_q_d_n1, eq33_e1198_q_d_n2, eq33_e1198_q_d_n3, eq33_e1198_q_d_n4, eq33_e1198_q_d_n5, eq33_e1198_q_d_n6, eq33_e1198_q_d_n7, eq33_e1198_q_d_n8, eq33_e1198_q_d_n9, eq33_e1198_q_d_n10, eq33_e1198_q_d_n11, eq33_e1198_q_d_n12, eq33_e1198_q_d_n13, eq33_e1198_q_d_n14, eq33_e1198_q_d_n15, eq33_e1198_q_d_n16, eq33_e1198_q_d_n17];
        let eq33_reactive_branch_derivatives: [f64; 12] = [eq33_e1198_q_d_b0, eq33_e1198_q_d_b1, eq33_e1198_q_d_b2, eq33_e1198_q_d_b3, eq33_e1198_q_d_b4, eq33_e1198_q_d_b5, eq33_e1198_q_d_b6, eq33_e1198_q_d_b7, eq33_e1198_q_d_b8, eq33_e1198_q_d_b9, eq33_e1198_q_d_b10, eq33_e1198_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202_q: f64 = s.v[301];
        let eq34_e1203: f64 = (eq34_e1200 * s.v[301]);
        let eq34_e1203_d_n0: f64 = (eq34_e1200 * s.dn[301][0]);
        let eq34_e1203_d_n1: f64 = (eq34_e1200 * s.dn[301][1]);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * s.dn[301][2]);
        let eq34_e1203_d_n3: f64 = (eq34_e1200 * s.dn[301][3]);
        let eq34_e1203_d_n4: f64 = (eq34_e1200 * s.dn[301][4]);
        let eq34_e1203_d_n5: f64 = (eq34_e1200 * s.dn[301][5]);
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * s.dn[301][6]);
        let eq34_e1203_d_n7: f64 = (eq34_e1200 * s.dn[301][7]);
        let eq34_e1203_d_n8: f64 = (eq34_e1200 * s.dn[301][8]);
        let eq34_e1203_d_n9: f64 = (eq34_e1200 * s.dn[301][9]);
        let eq34_e1203_d_n10: f64 = (eq34_e1200 * s.dn[301][10]);
        let eq34_e1203_d_n11: f64 = (eq34_e1200 * s.dn[301][11]);
        let eq34_e1203_d_n12: f64 = (eq34_e1200 * s.dn[301][12]);
        let eq34_e1203_d_n13: f64 = (eq34_e1200 * s.dn[301][13]);
        let eq34_e1203_d_n14: f64 = (eq34_e1200 * s.dn[301][14]);
        let eq34_e1203_d_n15: f64 = (eq34_e1200 * s.dn[301][15]);
        let eq34_e1203_d_n16: f64 = (eq34_e1200 * s.dn[301][16]);
        let eq34_e1203_d_n17: f64 = (eq34_e1200 * s.dn[301][17]);
        let eq34_e1203_d_b0: f64 = (eq34_e1200 * s.db[301][0]);
        let eq34_e1203_d_b1: f64 = (eq34_e1200 * s.db[301][1]);
        let eq34_e1203_d_b2: f64 = (eq34_e1200 * s.db[301][2]);
        let eq34_e1203_d_b3: f64 = (eq34_e1200 * s.db[301][3]);
        let eq34_e1203_d_b4: f64 = (eq34_e1200 * s.db[301][4]);
        let eq34_e1203_d_b5: f64 = (eq34_e1200 * s.db[301][5]);
        let eq34_e1203_d_b6: f64 = (eq34_e1200 * s.db[301][6]);
        let eq34_e1203_d_b7: f64 = (eq34_e1200 * s.db[301][7]);
        let eq34_e1203_d_b8: f64 = (eq34_e1200 * s.db[301][8]);
        let eq34_e1203_d_b9: f64 = (eq34_e1200 * s.db[301][9]);
        let eq34_e1203_d_b10: f64 = (eq34_e1200 * s.db[301][10]);
        let eq34_e1203_d_b11: f64 = (eq34_e1200 * s.db[301][11]);
        let eq34_e1203_q: f64 = (eq34_e1200 * eq34_e1202_q);
        let eq34_e1203_q_d_n0: f64 = (eq34_e1200 * s.dn[301][0]);
        let eq34_e1203_q_d_n1: f64 = (eq34_e1200 * s.dn[301][1]);
        let eq34_e1203_q_d_n2: f64 = (eq34_e1200 * s.dn[301][2]);
        let eq34_e1203_q_d_n3: f64 = (eq34_e1200 * s.dn[301][3]);
        let eq34_e1203_q_d_n4: f64 = (eq34_e1200 * s.dn[301][4]);
        let eq34_e1203_q_d_n5: f64 = (eq34_e1200 * s.dn[301][5]);
        let eq34_e1203_q_d_n6: f64 = (eq34_e1200 * s.dn[301][6]);
        let eq34_e1203_q_d_n7: f64 = (eq34_e1200 * s.dn[301][7]);
        let eq34_e1203_q_d_n8: f64 = (eq34_e1200 * s.dn[301][8]);
        let eq34_e1203_q_d_n9: f64 = (eq34_e1200 * s.dn[301][9]);
        let eq34_e1203_q_d_n10: f64 = (eq34_e1200 * s.dn[301][10]);
        let eq34_e1203_q_d_n11: f64 = (eq34_e1200 * s.dn[301][11]);
        let eq34_e1203_q_d_n12: f64 = (eq34_e1200 * s.dn[301][12]);
        let eq34_e1203_q_d_n13: f64 = (eq34_e1200 * s.dn[301][13]);
        let eq34_e1203_q_d_n14: f64 = (eq34_e1200 * s.dn[301][14]);
        let eq34_e1203_q_d_n15: f64 = (eq34_e1200 * s.dn[301][15]);
        let eq34_e1203_q_d_n16: f64 = (eq34_e1200 * s.dn[301][16]);
        let eq34_e1203_q_d_n17: f64 = (eq34_e1200 * s.dn[301][17]);
        let eq34_e1203_q_d_b0: f64 = (eq34_e1200 * s.db[301][0]);
        let eq34_e1203_q_d_b1: f64 = (eq34_e1200 * s.db[301][1]);
        let eq34_e1203_q_d_b2: f64 = (eq34_e1200 * s.db[301][2]);
        let eq34_e1203_q_d_b3: f64 = (eq34_e1200 * s.db[301][3]);
        let eq34_e1203_q_d_b4: f64 = (eq34_e1200 * s.db[301][4]);
        let eq34_e1203_q_d_b5: f64 = (eq34_e1200 * s.db[301][5]);
        let eq34_e1203_q_d_b6: f64 = (eq34_e1200 * s.db[301][6]);
        let eq34_e1203_q_d_b7: f64 = (eq34_e1200 * s.db[301][7]);
        let eq34_e1203_q_d_b8: f64 = (eq34_e1200 * s.db[301][8]);
        let eq34_e1203_q_d_b9: f64 = (eq34_e1200 * s.db[301][9]);
        let eq34_e1203_q_d_b10: f64 = (eq34_e1200 * s.db[301][10]);
        let eq34_e1203_q_d_b11: f64 = (eq34_e1200 * s.db[301][11]);
        let eq34_reactive_node_derivatives: [f64; 18] = [eq34_e1203_q_d_n0, eq34_e1203_q_d_n1, eq34_e1203_q_d_n2, eq34_e1203_q_d_n3, eq34_e1203_q_d_n4, eq34_e1203_q_d_n5, eq34_e1203_q_d_n6, eq34_e1203_q_d_n7, eq34_e1203_q_d_n8, eq34_e1203_q_d_n9, eq34_e1203_q_d_n10, eq34_e1203_q_d_n11, eq34_e1203_q_d_n12, eq34_e1203_q_d_n13, eq34_e1203_q_d_n14, eq34_e1203_q_d_n15, eq34_e1203_q_d_n16, eq34_e1203_q_d_n17];
        let eq34_reactive_branch_derivatives: [f64; 12] = [eq34_e1203_q_d_b0, eq34_e1203_q_d_b1, eq34_e1203_q_d_b2, eq34_e1203_q_d_b3, eq34_e1203_q_d_b4, eq34_e1203_q_d_b5, eq34_e1203_q_d_b6, eq34_e1203_q_d_b7, eq34_e1203_q_d_b8, eq34_e1203_q_d_b9, eq34_e1203_q_d_b10, eq34_e1203_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1232_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);
        let eq40_e1232_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);
        let eq40_e1232_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);
        let eq40_e1233_q: f64 = eq40_e1232;
        let eq40_reactive_node_derivatives: [f64; 18] = [eq40_e1232_d_n0, eq40_e1232_d_n1, eq40_e1232_d_n2, eq40_e1232_d_n3, eq40_e1232_d_n4, eq40_e1232_d_n5, eq40_e1232_d_n6, eq40_e1232_d_n7, eq40_e1232_d_n8, eq40_e1232_d_n9, eq40_e1232_d_n10, eq40_e1232_d_n11, eq40_e1232_d_n12, eq40_e1232_d_n13, eq40_e1232_d_n14, eq40_e1232_d_n15, eq40_e1232_d_n16, eq40_e1232_d_n17];
        let eq40_reactive_branch_derivatives: [f64; 12] = [eq40_e1232_d_b0, eq40_e1232_d_b1, eq40_e1232_d_b2, eq40_e1232_d_b3, eq40_e1232_d_b4, eq40_e1232_d_b5, eq40_e1232_d_b6, eq40_e1232_d_b7, eq40_e1232_d_b8, eq40_e1232_d_b9, eq40_e1232_d_b10, eq40_e1232_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1236_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);
        let eq41_e1236_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);
        let eq41_e1236_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 18] = [eq41_e1236_d_n0, eq41_e1236_d_n1, eq41_e1236_d_n2, eq41_e1236_d_n3, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, eq41_e1236_d_n12, eq41_e1236_d_n13, eq41_e1236_d_n14, eq41_e1236_d_n15, eq41_e1236_d_n16, eq41_e1236_d_n17];
        let eq41_reactive_branch_derivatives: [f64; 12] = [eq41_e1236_d_b0, eq41_e1236_d_b1, eq41_e1236_d_b2, eq41_e1236_d_b3, eq41_e1236_d_b4, eq41_e1236_d_b5, eq41_e1236_d_b6, eq41_e1236_d_b7, eq41_e1236_d_b8, eq41_e1236_d_b9, eq41_e1236_d_b10, eq41_e1236_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq58_e1342: f64 = (s.v[767] * (nv4 - 0.0));
        let eq58_e1342_d_n0: f64 = (s.dn[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_n1: f64 = (s.dn[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_n2: f64 = (s.dn[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_n3: f64 = (s.dn[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_n4: f64 = ((s.dn[767][4] * (nv4 - 0.0)) + s.v[767]);
        let eq58_e1342_d_n5: f64 = (s.dn[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_n6: f64 = (s.dn[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_n7: f64 = (s.dn[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_n8: f64 = (s.dn[767][8] * (nv4 - 0.0));
        let eq58_e1342_d_n9: f64 = (s.dn[767][9] * (nv4 - 0.0));
        let eq58_e1342_d_n10: f64 = (s.dn[767][10] * (nv4 - 0.0));
        let eq58_e1342_d_n11: f64 = (s.dn[767][11] * (nv4 - 0.0));
        let eq58_e1342_d_n12: f64 = (s.dn[767][12] * (nv4 - 0.0));
        let eq58_e1342_d_n13: f64 = (s.dn[767][13] * (nv4 - 0.0));
        let eq58_e1342_d_n14: f64 = (s.dn[767][14] * (nv4 - 0.0));
        let eq58_e1342_d_n15: f64 = (s.dn[767][15] * (nv4 - 0.0));
        let eq58_e1342_d_n16: f64 = (s.dn[767][16] * (nv4 - 0.0));
        let eq58_e1342_d_n17: f64 = (s.dn[767][17] * (nv4 - 0.0));
        let eq58_e1342_d_b0: f64 = (s.db[767][0] * (nv4 - 0.0));
        let eq58_e1342_d_b1: f64 = (s.db[767][1] * (nv4 - 0.0));
        let eq58_e1342_d_b2: f64 = (s.db[767][2] * (nv4 - 0.0));
        let eq58_e1342_d_b3: f64 = (s.db[767][3] * (nv4 - 0.0));
        let eq58_e1342_d_b4: f64 = (s.db[767][4] * (nv4 - 0.0));
        let eq58_e1342_d_b5: f64 = (s.db[767][5] * (nv4 - 0.0));
        let eq58_e1342_d_b6: f64 = (s.db[767][6] * (nv4 - 0.0));
        let eq58_e1342_d_b7: f64 = (s.db[767][7] * (nv4 - 0.0));
        let eq58_e1342_d_b8: f64 = (s.db[767][8] * (nv4 - 0.0));
        let eq58_e1342_d_b9: f64 = (s.db[767][9] * (nv4 - 0.0));
        let eq58_e1342_d_b10: f64 = (s.db[767][10] * (nv4 - 0.0));
        let eq58_e1342_d_b11: f64 = (s.db[767][11] * (nv4 - 0.0));
        let eq58_e1343_q: f64 = eq58_e1342;
        let eq58_reactive_node_derivatives: [f64; 18] = [eq58_e1342_d_n0, eq58_e1342_d_n1, eq58_e1342_d_n2, eq58_e1342_d_n3, eq58_e1342_d_n4, eq58_e1342_d_n5, eq58_e1342_d_n6, eq58_e1342_d_n7, eq58_e1342_d_n8, eq58_e1342_d_n9, eq58_e1342_d_n10, eq58_e1342_d_n11, eq58_e1342_d_n12, eq58_e1342_d_n13, eq58_e1342_d_n14, eq58_e1342_d_n15, eq58_e1342_d_n16, eq58_e1342_d_n17];
        let eq58_reactive_branch_derivatives: [f64; 12] = [eq58_e1342_d_b0, eq58_e1342_d_b1, eq58_e1342_d_b2, eq58_e1342_d_b3, eq58_e1342_d_b4, eq58_e1342_d_b5, eq58_e1342_d_b6, eq58_e1342_d_b7, eq58_e1342_d_b8, eq58_e1342_d_b9, eq58_e1342_d_b10, eq58_e1342_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq58_reactive_node_derivatives,
            branches,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1358, eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17, eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11, eq61_e1358_q, eq61_e1358_q_d_n0, eq61_e1358_q_d_n1, eq61_e1358_q_d_n2, eq61_e1358_q_d_n3, eq61_e1358_q_d_n4, eq61_e1358_q_d_n5, eq61_e1358_q_d_n6, eq61_e1358_q_d_n7, eq61_e1358_q_d_n8, eq61_e1358_q_d_n9, eq61_e1358_q_d_n10, eq61_e1358_q_d_n11, eq61_e1358_q_d_n12, eq61_e1358_q_d_n13, eq61_e1358_q_d_n14, eq61_e1358_q_d_n15, eq61_e1358_q_d_n16, eq61_e1358_q_d_n17, eq61_e1358_q_d_b0, eq61_e1358_q_d_b1, eq61_e1358_q_d_b2, eq61_e1358_q_d_b3, eq61_e1358_q_d_b4, eq61_e1358_q_d_b5, eq61_e1358_q_d_b6, eq61_e1358_q_d_b7, eq61_e1358_q_d_b8, eq61_e1358_q_d_b9, eq61_e1358_q_d_b10, eq61_e1358_q_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (s.v[800] * (nv11 - 0.0));
        let eq61_e1355_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);
        let eq61_e1355_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));
        let eq61_e1355_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));
        let eq61_e1355_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));
        let eq61_e1355_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));
        let eq61_e1355_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));
        let eq61_e1355_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));
        let eq61_e1355_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));
        let eq61_e1356_q: f64 = eq61_e1355;
        (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9, eq61_e1355_d_b10, eq61_e1355_d_b11, eq61_e1356_q, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9, eq61_e1355_d_b10, eq61_e1355_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 18] = [eq61_e1358_q_d_n0, eq61_e1358_q_d_n1, eq61_e1358_q_d_n2, eq61_e1358_q_d_n3, eq61_e1358_q_d_n4, eq61_e1358_q_d_n5, eq61_e1358_q_d_n6, eq61_e1358_q_d_n7, eq61_e1358_q_d_n8, eq61_e1358_q_d_n9, eq61_e1358_q_d_n10, eq61_e1358_q_d_n11, eq61_e1358_q_d_n12, eq61_e1358_q_d_n13, eq61_e1358_q_d_n14, eq61_e1358_q_d_n15, eq61_e1358_q_d_n16, eq61_e1358_q_d_n17];
        let eq61_reactive_branch_derivatives: [f64; 12] = [eq61_e1358_q_d_b0, eq61_e1358_q_d_b1, eq61_e1358_q_d_b2, eq61_e1358_q_d_b3, eq61_e1358_q_d_b4, eq61_e1358_q_d_b5, eq61_e1358_q_d_b6, eq61_e1358_q_d_b7, eq61_e1358_q_d_b8, eq61_e1358_q_d_b9, eq61_e1358_q_d_b10, eq61_e1358_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            None,
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq62_e1365, eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17, eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11, eq62_e1365_q, eq62_e1365_q_d_n0, eq62_e1365_q_d_n1, eq62_e1365_q_d_n2, eq62_e1365_q_d_n3, eq62_e1365_q_d_n4, eq62_e1365_q_d_n5, eq62_e1365_q_d_n6, eq62_e1365_q_d_n7, eq62_e1365_q_d_n8, eq62_e1365_q_d_n9, eq62_e1365_q_d_n10, eq62_e1365_q_d_n11, eq62_e1365_q_d_n12, eq62_e1365_q_d_n13, eq62_e1365_q_d_n14, eq62_e1365_q_d_n15, eq62_e1365_q_d_n16, eq62_e1365_q_d_n17, eq62_e1365_q_d_b0, eq62_e1365_q_d_b1, eq62_e1365_q_d_b2, eq62_e1365_q_d_b3, eq62_e1365_q_d_b4, eq62_e1365_q_d_b5, eq62_e1365_q_d_b6, eq62_e1365_q_d_b7, eq62_e1365_q_d_b8, eq62_e1365_q_d_b9, eq62_e1365_q_d_b10, eq62_e1365_q_d_b11,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (s.v[801] * (nv12 - 0.0));
        let eq62_e1362_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));
        let eq62_e1362_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);
        let eq62_e1362_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));
        let eq62_e1362_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));
        let eq62_e1362_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));
        let eq62_e1362_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));
        let eq62_e1362_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));
        let eq62_e1362_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));
        let eq62_e1363_q: f64 = eq62_e1362;
        (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_d_b10, eq62_e1362_d_b11, eq62_e1363_q, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_d_b10, eq62_e1362_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 18] = [eq62_e1365_q_d_n0, eq62_e1365_q_d_n1, eq62_e1365_q_d_n2, eq62_e1365_q_d_n3, eq62_e1365_q_d_n4, eq62_e1365_q_d_n5, eq62_e1365_q_d_n6, eq62_e1365_q_d_n7, eq62_e1365_q_d_n8, eq62_e1365_q_d_n9, eq62_e1365_q_d_n10, eq62_e1365_q_d_n11, eq62_e1365_q_d_n12, eq62_e1365_q_d_n13, eq62_e1365_q_d_n14, eq62_e1365_q_d_n15, eq62_e1365_q_d_n16, eq62_e1365_q_d_n17];
        let eq62_reactive_branch_derivatives: [f64; 12] = [eq62_e1365_q_d_b0, eq62_e1365_q_d_b1, eq62_e1365_q_d_b2, eq62_e1365_q_d_b3, eq62_e1365_q_d_b4, eq62_e1365_q_d_b5, eq62_e1365_q_d_b6, eq62_e1365_q_d_b7, eq62_e1365_q_d_b8, eq62_e1365_q_d_b9, eq62_e1365_q_d_b10, eq62_e1365_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq62_reactive_node_derivatives,
            branches,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1384, eq66_e1384_d_n13, eq66_e1384_q, eq66_e1384_q_d_n13,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382_q: f64 = (nv13 - 0.0);
        ((nv13 - 0.0), 1.0, eq66_e1382_q, 1.0,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq66_e1384_q_d_n13),
        );
    }
}
