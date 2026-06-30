#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[51] = param_given[10];
        s.store_scalar(51, if s.b[51] { 1.0 } else { 0.0 });

        if s.b[51] {
            s.store_scalar(13, p.p10);
        }

        if (!s.b[51]) {
            s.store_scalar(13, 1.0);
        }

        s.b[52] = param_given[11];
        s.store_scalar(52, if s.b[52] { 1.0 } else { 0.0 });

        if s.b[52] {
            s.store_scalar(14, (1.0 - (0.01 * p.p11)));
        }

        if (!s.b[52]) {
            s.store_scalar(14, 1.0);
        }

        s.store_scaled_mul(18, 14, 13, 1000000.0);

        s.store_scalar(11, (273.15 + p.p16));

        s.store_scalar(28, ((ctx_temp + p.p5) - 273.15));

        s.b[56] = ((p.p3 != 0.0) && (p.p4 != 0.0));
        s.store_scalar(56, if s.b[56] { 1.0 } else { 0.0 });

        if s.b[56] {
            s.store_scalar(17, p.p23);
        }

        s.b[57] = ((p.p3 != 0.0) || (p.p4 != 0.0));
        s.store_scalar(57, if s.b[57] { 1.0 } else { 0.0 });

        if ((!s.b[56]) && s.b[57]) {
            s.store_scalar(17, (p.p23 * 0.5));
        }

        if ((!s.b[56]) && (!s.b[57])) {
            s.store_scalar(17, 0.0);
        }

        s.b[58] = ((param_given[1] && param_given[2]) && (!param_given[0]));
        s.store_scalar(58, if s.b[58] { 1.0 } else { 0.0 });

        s.b[59] = ((p.p2 == 0.0) || (p.p1 == 0.0));
        s.store_scalar(59, if s.b[59] { 1.0 } else { 0.0 });

        if (s.b[58] && s.b[59]) {
            s.store_scalar(19, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
            s.store_scalar(5, 0.0);
        }

        let (assign270_e231,) = {
    if (s.b[58] && s.b[59]) {
        (1e99,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign270_e231);

        if (s.b[58] && (!s.b[59])) {
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
        }

        s.b[61] = (s.v[3] > 0.0);
        s.store_scalar(61, if s.b[61] { 1.0 } else { 0.0 });

        if ((s.b[58] && (!s.b[59])) && s.b[61]) {
            s.store_scale(4, 3, (p.p17 / p.p2));
            s.store_offset(20, 4, (-p.p22));
            s.store_scalar(5, p.p2);
        }

        let (assign360_e302,) = {
    if ((s.b[58] && (!s.b[59])) && s.b[61]) {
        let assign360_e300: f64 = (1.0 / s.v[5]);
        (assign360_e300,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign360_e302);

        if ((s.b[58] && (!s.b[59])) && (!s.b[61])) {
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
            s.store_scalar(5, 0.0);
        }

        let (assign400_e346,) = {
    if ((s.b[58] && (!s.b[59])) && (!s.b[61])) {
        (1e99,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign400_e346);

        s.b[63] = (param_given[2] && (!param_given[1]));
        s.store_scalar(63, if s.b[63] { 1.0 } else { 0.0 });

        s.b[64] = (p.p2 == 0.0);
        s.store_scalar(64, if s.b[64] { 1.0 } else { 0.0 });

        if (((!s.b[58]) && s.b[63]) && s.b[64]) {
            s.store_scalar(19, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
            s.store_scalar(5, 0.0);
        }

        let (assign480_e413,) = {
    if (((!s.b[58]) && s.b[63]) && s.b[64]) {
        (1e99,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign480_e413);

        s.b[65] = (p.p0 == 0.0);
        s.store_scalar(65, if s.b[65] { 1.0 } else { 0.0 });

        if ((((!s.b[58]) && s.b[63]) && (!s.b[64])) && s.b[65]) {
            s.store_scalar(20, 0.0);
            s.store_scalar(4, 0.0);
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
            s.store_scalar(5, 1e99);
        }

        let (assign550_e492,) = {
    if ((((!s.b[58]) && s.b[63]) && (!s.b[64])) && s.b[65]) {
        (0.0,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign550_e492);

        if ((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) {
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
        }

        s.b[67] = (s.v[4] > 0.0);
        s.store_scalar(67, if s.b[67] { 1.0 } else { 0.0 });

        if (((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) && s.b[67]) {
            s.store_scale(3, 4, (p.p2 / p.p17));
            s.store_sub(19, 3, 17);
            s.store_scalar(5, p.p2);
        }

        let (assign640_e599,) = {
    if (((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) && s.b[67]) {
        let assign640_e597: f64 = (1.0 / s.v[5]);
        (assign640_e597,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign640_e599);

        if (((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) && (!s.b[67])) {
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
            s.store_scalar(5, 1e99);
        }

        let (assign680_e667,) = {
    if (((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) && (!s.b[67])) {
        (0.0,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign680_e667);

        s.b[69] = (p.p0 == 0.0);
        s.store_scalar(69, if s.b[69] { 1.0 } else { 0.0 });

        if (((!s.b[58]) && (!s.b[63])) && s.b[69]) {
            s.store_scalar(20, 0.0);
            s.store_scalar(4, 0.0);
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
            s.store_scalar(5, 1e99);
        }

        let (assign750_e734,) = {
    if (((!s.b[58]) && (!s.b[63])) && s.b[69]) {
        (0.0,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign750_e734);

        s.b[70] = (p.p1 == 0.0);
        s.store_scalar(70, if s.b[70] { 1.0 } else { 0.0 });

        if ((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && s.b[70]) {
            s.store_scalar(19, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
            s.store_scalar(5, 0.0);
        }

        let (assign820_e819,) = {
    if ((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && s.b[70]) {
        (1e99,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign820_e819);

        if ((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) {
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
        }

        s.b[72] = (s.v[4] > 0.0);
        s.store_scalar(72, if s.b[72] { 1.0 } else { 0.0 });

        s.b[74] = (s.v[3] > 0.0);
        s.store_scalar(74, if s.b[74] { 1.0 } else { 0.0 });

        if ((((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) && s.b[72]) && s.b[74]) {
            s.store_scaled_div(5, 3, 4, p.p17);
        }

        let (assign920_e937,) = {
    if ((((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) && s.b[72]) && s.b[74]) {
        let assign920_e935: f64 = (1.0 / s.v[5]);
        (assign920_e935,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign920_e937);

        if ((((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) && s.b[72]) && (!s.b[74])) {
            s.store_scalar(5, 0.0);
        }

        let (assign940_e975,) = {
    if ((((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) && s.b[72]) && (!s.b[74])) {
        (1e99,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign940_e975);

        if (((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) && (!s.b[72])) {
            s.store_scalar(5, 1e99);
        }

        let (assign960_e1009,) = {
    if (((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) && (!s.b[72])) {
        (0.0,)
    } else {
        (s.v[22],)
    }
};
        s.store_scalar(22, assign960_e1009);

        if (p.p25 != 0.0) {
            s.store_offset(21, 3, p.p24);
        }

        if (p.p25 == 0.0) {
            s.store_offset(21, 19, p.p24);
        }

        s.store_scalar(37, p.p37);

        s.store_scalar(38, p.p38);

        s.b[80] = (s.v[3] > 0.0);
        s.store_scalar(80, if s.b[80] { 1.0 } else { 0.0 });

        s.b[81] = ((p.p3 != 0.0) && (p.p4 != 0.0));
        s.store_scalar(81, if s.b[81] { 1.0 } else { 0.0 });

        if (s.b[80] && s.b[81]) {
            s.store_offset_div_from_scalar_ad(37, p.p39, s.ad_value(3), s.v[37]);
            s.store_offset_div_from_scalar_ad(38, p.p40, s.ad_value(3), s.v[38]);
        }

        s.b[82] = ((p.p3 != 0.0) || (p.p4 != 0.0));
        s.store_scalar(82, if s.b[82] { 1.0 } else { 0.0 });

        if ((s.b[80] && (!s.b[81])) && s.b[82]) {
            s.store_add_div_from_scalar_rhs(37, 37, (0.5 * p.p39), 3);
            s.store_add_div_from_scalar_rhs(38, 38, (0.5 * p.p40), 3);
        }

        s.b[83] = (s.v[4] > 0.0);
        s.store_scalar(83, if s.b[83] { 1.0 } else { 0.0 });

        if s.b[83] {
            s.store_add_div_from_scalar_rhs(37, 37, p.p41, 4);
            s.store_add_div_from_scalar_rhs(38, 38, p.p42, 4);
        }

        s.b[85] = ((p.p3 != 0.0) && (p.p4 != 0.0));
        s.store_scalar(85, if s.b[85] { 1.0 } else { 0.0 });

        if s.b[85] {
            s.store_scaled_add(46, 19, 20, 2.0);
        }

        s.b[86] = ((p.p3 != 0.0) || (p.p4 != 0.0));
        s.store_scalar(86, if s.b[86] { 1.0 } else { 0.0 });

        if ((!s.b[85]) && s.b[86]) {
            s.store_add_scaled_inputs(46, 19, 2.0, 20, 1.0);
        }

        if ((!s.b[85]) && (!s.b[86])) {
            s.store_scale(46, 19, 2.0);
        }

        s.store_mul(47, 19, 20);

        s.store_add_scaled_ad_lhs(41, A::scale_offset(s.ad_value(46), p.p45, p.p44), 47, p.p46);

        s.store_add_scaled_ad_lhs(9, A::scale_offset(s.ad_value(46), p.p48, p.p47), 47, p.p49);

        s.store_voltage(42, ctx, nodes, Some(2), None);

        s.store_offset_scaled(28, 42, p.p7, s.v[28]);

        s.b[88] = (s.v[28] < (p.p35 + 1.0));
        s.store_scalar(88, if s.b[88] { 1.0 } else { 0.0 });

        if s.b[88] {
            s.store_offset_exp_ad(28, A::offset(s.ad_value(28), (((-p.p35)) + ((-1.0)))), p.p35);
        }

        s.b[89] = (s.v[28] > (p.p36 - 1.0));
        s.store_scalar(89, if s.b[89] { 1.0 } else { 0.0 });

        if ((!s.b[88]) && s.b[89]) {
            s.store_sub_from_scalar_ad(28, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(28)), (-1.0))));
        }

        if ((!s.b[88]) && (!s.b[89])) {
        }

        s.store_offset(12, 28, 273.15);

        s.store_offset(15, 12, (-s.v[11]));

        s.store_offset_mul_ad(16, s.ad_value(15), A::add_scaled_product(s.ad_value(37), 1.0, s.ad_value(15), s.ad_value(38), 1.0), 1.0);

        s.b[90] = (s.v[16] < (0.01 + 0.1));
        s.store_scalar(90, if s.b[90] { 1.0 } else { 0.0 });

        if s.b[90] {
            s.store_offset_scaled_ad(16, A::exp(A::scale_offset(s.ad_value(16), 10.0, (((((-0.01)) * (10.0))) + ((-1.0))))), 0.1, 0.01);
        }

        if (!s.b[90]) {
        }

        s.store_mul(23, 5, 16);

        s.store_voltage(33, ctx, nodes, Some(0), Some(1));

        s.b[92] = ((s.v[5] > 0.0) && ((p.p29 > 0.0) || (p.p27 > 0.0)));
        s.store_scalar(92, if s.b[92] { 1.0 } else { 0.0 });

        if s.b[92] {
            s.store_div(34, 33, 21);
            s.store_scale(35, 34, p.p28);
            s.store_sqrt_square_offset(26, 35, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[92] {
            s.store_scaled_abs(36, 34, p.p26);
            s.store_powf_ad(27, A::offset(A::mul(A::square(s.ad_value(36)), s.ad_value(36)), 1.0), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(32, A::scale_offset(s.ad_value(26), p.p29, ((1.0 - p.p29) - p.p27)), 27, p.p27);
        }

        if (!s.b[92]) {
            s.store_scalar(32, 1.0);
        }

        s.store_mul(6, 23, 32);

        s.copy_ad(0, 33);

        s.store_div(1, 0, 6);

        s.store_mul_neg_lhs(43, 0, 1);

        s.store_mul(44, 42, 41);

        s.store_mul(45, 42, 9);

        s.b[99] = ((s.v[5] > 0.0) && (s.v[22] > 0.0));
        s.store_scalar(99, if s.b[99] { 1.0 } else { 0.0 });

        if s.b[99] {
            s.store_mul(6, 23, 32);
        }

        if (!s.b[99]) {
            s.copy_ad(6, 5);
        }

    }
}
